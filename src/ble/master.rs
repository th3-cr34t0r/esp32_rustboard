use crate::ble::BleStatus;
use crate::config::enums::{HidKeys, HidModifiers, KeyType};
use crate::config::{config::*, layers::*};
use crate::debounce::{Debounce, KEY_PRESSED, KEY_RELEASED};
use crate::delay::*;
use crate::matrix::{store_key, Key};

use super::{BleKeyboardMaster, KeyReport, HID_REPORT_DISCRIPTOR, KEYBOARD_ID, MEDIA_KEYS_ID};
use bstr::{ByteSlice, ByteVec};
use embassy_futures::select::select;
use esp32_nimble::{enums::*, uuid128, BLEAdvertisementData, BLEDevice, BLEHIDDevice, BLEScan};
use esp_idf_sys::{
    esp_ble_power_type_t_ESP_BLE_PWR_TYPE_ADV, esp_ble_power_type_t_ESP_BLE_PWR_TYPE_DEFAULT,
    esp_ble_power_type_t_ESP_BLE_PWR_TYPE_SCAN,
};
use heapless::{FnvIndexMap, Vec};
use spin::Mutex as spinMutex;
use zerocopy::IntoBytes;

impl BleKeyboardMaster {
    pub async fn new() -> Self {
        let device = BLEDevice::take();

        device
            .security()
            .set_auth(AuthReq::all())
            .set_io_cap(SecurityIOCap::NoInputNoOutput)
            .resolve_rpa();

        let server = device.get_server();

        let mut hid = BLEHIDDevice::new(server);

        let input_keyboard = hid.input_report(KEYBOARD_ID);
        let output_keyboard = hid.output_report(KEYBOARD_ID);
        let input_media_keys = hid.input_report(MEDIA_KEYS_ID);

        hid.manufacturer("Espressif");
        hid.pnp(0x02, 0x05ac, 0x820a, 0x0210);
        hid.hid_info(0x00, 0x01);

        hid.report_map(HID_REPORT_DISCRIPTOR);

        hid.set_battery_level(100);

        let ble_advertising = device.get_advertising();

        ble_advertising
            .lock()
            .scan_response(false)
            .set_data(
                BLEAdvertisementData::new()
                    .name("RUSTBOARD")
                    .appearance(0x03C1)
                    .add_service_uuid(hid.hid_service().lock().uuid()),
            )
            .unwrap();

        ble_advertising.lock().start().unwrap();

        // connecting to the slave device
        let mut ble_slave_scan = BLEScan::new();

        let slave_device = ble_slave_scan
            .active_scan(true)
            .interval(100)
            .window(99)
            .start(&device, 10000, |device, data| {
                if let Some(name) = data.name() {
                    if name.contains_str("rustboard-slave") {
                        return Some(*device);
                    }
                }
                None
            })
            .await
            .ok();

        let slave_device = slave_device.unwrap_or(None);

        let mut client = device.new_client();

        client.on_connect(|client| {
            client.update_conn_params(5, 10, 0, 60).unwrap();
        });

        client.connect(&slave_device.unwrap().addr()).await.unwrap();

        let client_service = client
            .get_service(uuid128!("fafafafa-fafa-fafa-fafa-fafafafafafa"))
            .await
            .unwrap();

        let uuid = uuid128!(BLE_SLAVE_UUID);

        let slave_characteristic = client_service.get_characteristic(uuid).await.unwrap();

        Self {
            server,
            slave_characteristic,
            input_keyboard,
            output_keyboard,
            input_media_keys,
            key_report: KeyReport {
                modifiers: 0,
                reserved: 0,
                keys: [0; 6],
            },
        }
    }

    pub fn connected(&self) -> bool {
        self.server.connected_count() > 0
    }

    pub fn send_report(&mut self) {
        self.input_keyboard
            .lock()
            .set_value(&self.key_report.as_bytes())
            .notify();
        esp_idf_svc::hal::delay::Ets::delay_ms(1);
    }

    pub fn set_ble_power_save(&mut self) {
        /* set power save */
        unsafe {
            esp_idf_sys::esp_ble_tx_power_set(
                esp_ble_power_type_t_ESP_BLE_PWR_TYPE_DEFAULT,
                ESP_POWER_LEVEL.convert(),
            );
            esp_idf_sys::esp_ble_tx_power_set(
                esp_ble_power_type_t_ESP_BLE_PWR_TYPE_ADV,
                ESP_POWER_LEVEL.convert(),
            );
            esp_idf_sys::esp_ble_tx_power_set(
                esp_ble_power_type_t_ESP_BLE_PWR_TYPE_SCAN,
                ESP_POWER_LEVEL.convert(),
            );
        }
    }
}

// This is the function that recieves information from the client
// about the keys pressed on the other half of the keyboard
async fn ble_client_recieve(
    ble_keyboard: &spinMutex<BleKeyboardMaster>,
    keys_pressed: &spinMutex<FnvIndexMap<Key, Debounce, PRESSED_KEYS_INDEXMAP_SIZE>>,
) {
    let mut restored_key_count: Key = Key::new(0, 0);

    loop {
        if let Some(ble_keyboard) = ble_keyboard.try_lock() {
            while let Some(key) = ble_keyboard
                .slave_characteristic
                .read_value()
                .await
                .unwrap()
                .pop_byte()
            {
                if key != 0 {
                    restored_key_count.col = key & 0xFF;
                    restored_key_count.row = (key >> BIT_MASK) & 0xFF;

                    store_key(&keys_pressed, &restored_key_count);

                    #[cfg(feature = "debug")]
                    log::info!("Recieved value from slave: {:?}", restored_key_count);
                }
            }
        }
        delay_ms(1).await;
    }
}

fn add_keys(ble_keyboard: &mut BleKeyboardMaster, valid_key: &HidKeys, layer_state: &mut Layer) {
    /* get the key type */
    match KeyType::check_type(valid_key) {
        KeyType::Macro => {
            let macro_valid_keys = HidKeys::get_macro_sequence(valid_key);
            for valid_key in macro_valid_keys.iter() {
                add_keys(ble_keyboard, valid_key, layer_state);
            }
        }
        KeyType::Layer => {
            /* check and set the layer */
            *layer_state = Layer::Upper;

            /* release all keys */
            ble_keyboard
                .key_report
                .keys
                .iter_mut()
                .for_each(|value| *value = 0);

            /* release modifiers */
            ble_keyboard.key_report.modifiers = 0;
        }
        KeyType::Modifier => {
            ble_keyboard.key_report.modifiers |= HidModifiers::get_modifier(valid_key);
        }
        KeyType::Key => {
            /* check if the key count is less than 6 */
            if !ble_keyboard.key_report.keys.contains(&(*valid_key as u8)) {
                /* find the first key slot in the array that is
                 * free */
                match ble_keyboard
                    .key_report
                    .keys
                    .iter()
                    .position(|&value| value == 0)
                {
                    Some(index) => {
                        /* add the new key to that position */
                        ble_keyboard.key_report.keys[index] = *valid_key as u8
                    }
                    None => { /* there is no free key slot available */ }
                }
            }
        }
    }
}

fn remove_keys(ble_keyboard: &mut BleKeyboardMaster, valid_key: &HidKeys, layer_state: &mut Layer) {
    /* get the key type */
    match KeyType::check_type(valid_key) {
        KeyType::Macro => {
            let macro_valid_keys = HidKeys::get_macro_sequence(valid_key);
            for valid_key in macro_valid_keys.iter() {
                remove_keys(ble_keyboard, valid_key, layer_state);
            }
        }
        KeyType::Layer => {
            /* check and set the layer */
            *layer_state = Layer::Base;

            /* release all keys */
            ble_keyboard
                .key_report
                .keys
                .iter_mut()
                .for_each(|value| *value = 0);

            /* release modifiers */
            ble_keyboard.key_report.modifiers = 0;
        }
        KeyType::Modifier => {
            /* remove the modifier */
            ble_keyboard.key_report.modifiers &= !HidModifiers::get_modifier(valid_key);
        }
        KeyType::Key => {
            /* find the key slot of the released key */
            match ble_keyboard
                .key_report
                .keys
                .iter()
                .position(|&value| value == *valid_key as u8)
            {
                Some(index) => {
                    /* remove the key from the key slot */
                    ble_keyboard.key_report.keys[index] = 0
                }
                None => { /* do nothing */ }
            }
        }
    }
}

pub async fn ble_send_keys(
    ble_keyboard: &spinMutex<BleKeyboardMaster>,
    keys_pressed: &spinMutex<FnvIndexMap<Key, Debounce, PRESSED_KEYS_INDEXMAP_SIZE>>,
    ble_status: &spinMutex<BleStatus>,
) -> ! {
    /* initialize layers */
    let mut layers = Layers::new();

    /* load the specified layout */
    layers.load_layout();

    /* layer state */
    let mut layer_state = Layer::Base;

    /* vec to store the keys needed to be removed */
    let mut pressed_keys_to_remove: Vec<Key, 6> = Vec::new();

    /* flag to set the power mode of the esp */
    let mut power_save_flag: bool = true;

    let mut ble_status_prev: BleStatus = BleStatus::NotConnected;

    /* Run the main loop */
    loop {
        if let Some(mut ble_keyboard) = ble_keyboard.try_lock() {
            if ble_keyboard.connected() {
                /* check and store the ble status, then release the lock */
                match ble_status_prev {
                    BleStatus::NotConnected => {
                        ble_status_prev = BleStatus::Connected;

                        if let Some(mut ble_status) = ble_status.try_lock() {
                            *ble_status = BleStatus::Connected;
                        }
                    }
                    BleStatus::Connected => {}
                }

                /* check if power save has been set */
                if power_save_flag {
                    /* set ble power to lowest possible */
                    ble_keyboard.set_ble_power_save();
                    /* set flag to false */
                    power_save_flag = false;
                }

                /* try to lock the hashmap */
                if let Some(mut keys_pressed) = keys_pressed.try_lock() {
                    /* check if there are pressed keys */
                    if !keys_pressed.is_empty() {
                        /* iter trough the pressed keys */
                        for (key, debounce) in keys_pressed.iter_mut() {
                            /*check the key debounce state */
                            match debounce.key_state {
                                KEY_PRESSED => {
                                    /* get the pressed key */
                                    if let Some(valid_key) =
                                        layers.get(&key.row, &key.col, &layer_state)
                                    {
                                        add_keys(&mut ble_keyboard, valid_key, &mut layer_state);
                                    }
                                }
                                /* check if the key is calculated for debounce */
                                KEY_RELEASED => {
                                    /* get the mapped key from the hashmap */
                                    if let Some(valid_key) =
                                        layers.get(&key.row, &key.col, &layer_state)
                                    {
                                        remove_keys(&mut ble_keyboard, valid_key, &mut layer_state);
                                    }
                                    /* if key has been debounced, add it to be removed */
                                    pressed_keys_to_remove
                                        .push(*key)
                                        .expect("Error adding a key to be removed!");
                                }

                                _ => { /* do nothing */ }
                            }
                        }

                        #[cfg(feature = "debug")]
                        /* debug log */
                        log::info!(
                            "ble_keyboard.key_report.keys: {:?}",
                            ble_keyboard.key_report.keys
                        );

                        /* sent the new report */
                        ble_keyboard.send_report();

                        /* remove the sent keys and empty the vec */
                        while let Some(key) = pressed_keys_to_remove.pop() {
                            keys_pressed.remove(&key).unwrap();
                        }
                    }
                }
            } else {
                #[cfg(feature = "debug")]
                /* debug log */
                log::info!("Keyboard not connected!");

                /* check and store the ble status */
                match ble_status_prev {
                    BleStatus::NotConnected => {}
                    BleStatus::Connected => {
                        ble_status_prev = BleStatus::NotConnected;

                        /* lock the mutex and set the new value */
                        *ble_status.lock() = BleStatus::NotConnected;
                    }
                }

                /* check the power save flag */
                if !power_save_flag {
                    /* if false, set to true */
                    power_save_flag = true;
                }

                /* sleep for 100ms */
                delay_ms(100).await;
            }
        }
        /* there must be a delay so the WDT in not triggered */
        delay_ms(5).await;
    }
}

/// This is the main master ble function that runs the client recieve
/// and the server for sending the keys to the connected device (PC, Phone etc.)
pub async fn ble_rx_tx(
    keys_pressed: &spinMutex<FnvIndexMap<Key, Debounce, PRESSED_KEYS_INDEXMAP_SIZE>>,
    ble_status: &spinMutex<BleStatus>,
) {
    /* construct ble master */
    let ble_keyboard: spinMutex<BleKeyboardMaster> = spinMutex::new(BleKeyboardMaster::new().await);

    select(
        ble_client_recieve(&ble_keyboard, keys_pressed),
        ble_send_keys(&ble_keyboard, keys_pressed, ble_status),
    )
    .await;
}

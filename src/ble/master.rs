extern crate alloc;

use alloc::sync::Arc;
use embassy_time::Instant;

use crate::ble::BleStatus;
use crate::config::enums::{HidKeys, HidModifiers, KeyType};
use crate::config::layout::Layout;
use crate::config::user_config::*;
use crate::debounce::{KeyInfo, KeyState};
use crate::delay::*;
use crate::matrix::{KeyPos, StoredKeys};

use super::{BleKeyboardMaster, KeyReport, HID_REPORT_DISCRIPTOR, KEYBOARD_ID, MEDIA_KEYS_ID};
use esp32_nimble::{
    enums::*, utilities::mutex::Mutex, uuid128, BLEAdvertisementData, BLEDevice, BLEHIDDevice,
    NimbleProperties,
};
use esp_idf_sys::{
    esp_ble_power_type_t_ESP_BLE_PWR_TYPE_ADV, esp_ble_power_type_t_ESP_BLE_PWR_TYPE_DEFAULT,
    esp_ble_power_type_t_ESP_BLE_PWR_TYPE_SCAN,
};
use heapless::Vec;
use zerocopy::IntoBytes;

impl BleKeyboardMaster {
    pub async fn new() -> Self {
        let device = BLEDevice::take();

        // creating server
        device
            .security()
            .set_auth(AuthReq::Bond)
            .set_io_cap(SecurityIOCap::NoInputNoOutput)
            .resolve_rpa();

        let server = device.get_server();
        let ble_advertising = device.get_advertising();

        server.on_connect(|server, desc| {
            log::info!("Client connected: {:?}", desc);

            if server.connected_count() < (esp_idf_svc::sys::CONFIG_BT_NIMBLE_MAX_CONNECTIONS as _)
            {
                log::info!("Multi-connect support: start advertising!");
                ble_advertising.lock().start().unwrap();
            }
        });

        // create ble client characteristic
        let service = server.create_service(uuid128!("fafafafa-fafa-fafa-fafa-fafafafafafa"));

        let slave_characteristic = service.lock().create_characteristic(
            BLE_SLAVE_UUID,
            NimbleProperties::READ | NimbleProperties::WRITE | NimbleProperties::WRITE_NO_RSP,
        );

        let mut hid = BLEHIDDevice::new(server);

        let input_keyboard = hid.input_report(KEYBOARD_ID);
        let output_keyboard = hid.output_report(KEYBOARD_ID);
        let input_media_keys = hid.input_report(MEDIA_KEYS_ID);

        hid.manufacturer("Espressif");
        hid.pnp(0x02, 0x05ac, 0x820a, 0x0210);
        hid.hid_info(0x00, 0x01);

        hid.report_map(HID_REPORT_DISCRIPTOR);

        hid.set_battery_level(100);

        ble_advertising
            .lock()
            .scan_response(false)
            .set_data(
                BLEAdvertisementData::new()
                    .name("Rustboard")
                    .appearance(0x03C1)
                    .add_service_uuid(hid.hid_service().lock().uuid())
                    .add_service_uuid(slave_characteristic.lock().uuid()),
            )
            .unwrap();

        ble_advertising.lock().start().unwrap();

        // on esp32-c3, advertising stops when a device is boded.
        ble_advertising.lock().on_complete(|_| {
            ble_advertising.lock().start().unwrap();
            log::info!("bonded_addresses: {:?}", device.bonded_addresses());
        });

        Self {
            server,
            slave_characteristic,
            input_keyboard,
            output_keyboard,
            input_media_keys,
            current_key_report: KeyReport::default(),
            previous_key_report: KeyReport::default(),
        }
    }

    pub fn connected(&self) -> bool {
        self.server.connected_count() > 1
    }

    pub async fn send_report(&mut self) {
        self.input_keyboard
            .lock()
            .set_value(self.current_key_report.as_bytes())
            .notify();
    }

    pub fn set_ble_power_save(&mut self) {
        // set power save
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

    fn is_key_report_changed(&mut self) -> bool {
        if self.previous_key_report != self.current_key_report {
            self.previous_key_report = self.current_key_report;
            true
        } else {
            false
        }
    }
}

fn add_keys(ble_keyboard: &mut BleKeyboardMaster, valid_key: &HidKeys, layer: &mut usize) {
    // get the key type
    match KeyType::check_type(valid_key) {
        KeyType::Macro => {
            let macro_valid_keys = HidKeys::get_macro_sequence(valid_key);
            for valid_key in macro_valid_keys.iter() {
                add_keys(ble_keyboard, valid_key, layer);
            }
        }
        KeyType::Layer => {
            // check and set the layer
            *layer = Layout::get_layer(valid_key);

            // release all keys
            ble_keyboard.current_key_report.keys.fill(0);

            // release modifiers
            ble_keyboard.current_key_report.modifiers = 0;
        }
        KeyType::Modifier => {
            ble_keyboard.current_key_report.modifiers |= HidModifiers::get_modifier(valid_key);
        }
        KeyType::Key => {
            // check if the key count is less than 6
            if !ble_keyboard
                .current_key_report
                .keys
                .contains(&(*valid_key as u8))
            {
                // find the first key slot in the array that is free
                if let Some(index) = ble_keyboard
                    .current_key_report
                    .keys
                    .iter()
                    .position(|&value| value == 0)
                {
                    // add the new key to that position
                    ble_keyboard.current_key_report.keys[index] = *valid_key as u8
                }
            }
        }
    }
}

fn remove_keys(ble_keyboard: &mut BleKeyboardMaster, valid_key: &HidKeys, layer: &mut usize) {
    // get the key type
    match KeyType::check_type(valid_key) {
        KeyType::Macro => {
            let macro_valid_keys = HidKeys::get_macro_sequence(valid_key);
            for valid_key in macro_valid_keys.iter() {
                remove_keys(ble_keyboard, valid_key, layer);
            }
        }
        KeyType::Layer => {
            // set base layer
            *layer = 0;

            // release all keys
            ble_keyboard.current_key_report.keys.fill(0);

            // release modifiers
            ble_keyboard.current_key_report.modifiers = 0;
        }
        KeyType::Modifier => {
            // remove the modifier
            ble_keyboard.current_key_report.modifiers &= !HidModifiers::get_modifier(valid_key);
        }
        KeyType::Key => {
            // find the key slot of the released key
            if let Some(index) = ble_keyboard
                .current_key_report
                .keys
                .iter()
                .position(|&value| value == *valid_key as u8)
            {
                // remove the key from the key slot
                ble_keyboard.current_key_report.keys[index] = 0
            }
        }
    }
}

fn process_slave_key_report(
    pressed_keys: &Arc<Mutex<StoredKeys>>,
    slave_key_report: &Arc<Mutex<[u8; 6]>>,
) {
    let mut recovered_key: KeyPos = KeyPos::new(255, 255);

    slave_key_report.lock().iter().for_each(|element| {
        if *element != 0 {
            recovered_key.row = *element >> BIT_SHIFT;
            recovered_key.col = *element & 0x0F;

            pressed_keys
                .lock()
                .index_map
                .insert(
                    recovered_key,
                    KeyInfo {
                        pressed_time: Instant::now(),
                        state: KeyState::Pressed,
                    },
                )
                .expect("Not enough space to store incoming slave data.");
        }
    });
}

pub async fn ble_tx(
    pressed_keys: &Arc<Mutex<StoredKeys>>,
    ble_status: &Arc<Mutex<BleStatus>>,
) -> ! {
    // init ble
    let mut ble_keyboard: BleKeyboardMaster = BleKeyboardMaster::new().await;

    // initialize layers
    let layout = Layout::init();

    // layer state
    let mut layer: usize = 0;

    // vec to store the keys needed to be removed
    let mut pressed_keys_to_remove: Vec<KeyPos, 6> = Vec::new();

    // set ble power to lowest possible
    // ble_keyboard.set_ble_power_save();

    let slave_key_report: Arc<Mutex<[u8; 6]>> = Arc::new(Mutex::new([0; 6]));

    // on_write callback
    ble_keyboard.slave_characteristic.lock().on_write({
        let slave_key_report = Arc::clone(&slave_key_report);
        move |args| {
            let mut slave_key_report_locked = slave_key_report.lock();
            let mut index: usize = 0;
            args.recv_data().iter().for_each(|byte_data| {
                slave_key_report_locked[index] = *byte_data;

                index += 1;
            });

            // debug log
            #[cfg(feature = "debug")]
            log::info!("Received from slave: {:?}", *slave_key_report_locked);
        }
    });

    // Run the main loop
    loop {
        if ble_keyboard.connected() {
            // check and store the ble status, then release the lock
            if let Some(mut ble_status) = ble_status.try_lock() {
                *ble_status = BleStatus::Connected;
            }

            // process slave key report
            process_slave_key_report(pressed_keys, &slave_key_report);

            // try to lock the hashmap
            if let Some(mut pressed_keys) = pressed_keys.try_lock() {
                // check if there are pressed keys
                if !pressed_keys.index_map.is_empty() {
                    // iter trough the pressed keys
                    for (key_pos, key_info) in pressed_keys.index_map.iter_mut() {
                        // check the key debounce state
                        match key_info.state {
                            KeyState::Pressed => {
                                // get the pressed key
                                if let Some(valid_key) = layout.keymap[layer].get(&key_pos) {
                                    add_keys(&mut ble_keyboard, valid_key, &mut layer);
                                }
                            }
                            // check if the key is calculated for debounce
                            KeyState::Released => {
                                // get the mapped key from the hashmap
                                if let Some(valid_key) = layout.keymap[layer].get(&key_pos) {
                                    remove_keys(&mut ble_keyboard, valid_key, &mut layer);
                                }

                                // if key has been debounced, add it to be removed
                                pressed_keys_to_remove
                                    .push(*key_pos)
                                    .expect("Error adding a key to be removed!");
                            }
                        }
                    }

                    // log
                    #[cfg(feature = "debug")]
                    log::info!(
                        "ble_keyboard.key_report.keys: {:?}",
                        ble_keyboard.current_key_report.keys
                    );

                    // sent the new report only if it differes from the previous
                    if ble_keyboard.is_key_report_changed() {
                        ble_keyboard.send_report().await;
                    }

                    // remove the sent keys and empty the vec
                    while let Some(key) = pressed_keys_to_remove.pop() {
                        pressed_keys.index_map.remove(&key).unwrap();
                    }
                }
            }

            // there must be a delay so the WDT in not triggered
            delay_ms(1).await;
        } else {
            // debug log
            #[cfg(feature = "debug")]
            log::info!("Keyboard not connected!");

            // check and store the ble status

            // check and store the ble status
            if let Some(mut ble_status) = ble_status.try_lock() {
                *ble_status = BleStatus::NotConnected;
            }

            // sleep for 100ms
            delay_ms(100).await;
        }
    }
}

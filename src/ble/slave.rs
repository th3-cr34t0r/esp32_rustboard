use crate::debounce::{KEY_PRESSED, KEY_RELEASED};
use crate::delay::delay_ms;
use crate::matrix::Key;
use crate::{config::config::*, debounce::Debounce};

use super::{BleKeyboardSlave, BleStatus};
use esp32_nimble::{enums::*, uuid128, BLEAdvertisementData, BLEDevice, NimbleProperties};
use esp_idf_sys::{
    esp_ble_power_type_t_ESP_BLE_PWR_TYPE_ADV, esp_ble_power_type_t_ESP_BLE_PWR_TYPE_DEFAULT,
    esp_ble_power_type_t_ESP_BLE_PWR_TYPE_SCAN,
};
use zerocopy::IntoBytes;

use heapless::{FnvIndexMap, Vec};
use spin::Mutex as spinMutex;

impl BleKeyboardSlave {
    pub fn new() -> Self {
        let device = BLEDevice::take();

        device
            .security()
            .set_auth(AuthReq::all())
            .set_io_cap(SecurityIOCap::NoInputNoOutput)
            .resolve_rpa();

        let server = device.get_server();

        let service = server.create_service(uuid128!("fafafafa-fafa-fafa-fafa-fafafafafafa"));

        let characteristic = service
            .lock()
            .create_characteristic(BLE_SLAVE_UUID, NimbleProperties::READ);

        let ble_advertising = device.get_advertising();

        ble_advertising
            .lock()
            .scan_response(false)
            .set_data(BLEAdvertisementData::new().add_service_uuid(BLE_SLAVE_UUID))
            .unwrap();

        ble_advertising.lock().start().unwrap();

        #[cfg(feature = "debug")]
        server.ble_gatts_show_local();

        Self {
            server,
            characteristic,
            keys: [0; 6],
        }
    }

    pub fn connected(&self) -> bool {
        self.server.connected_count() > 0
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

    pub async fn send_report(&mut self) {
        self.characteristic
            .lock()
            .set_value(&self.keys.as_bytes())
            .notify();

        delay_ms(1).await;
    }
}
fn add_keys(ble_keyboard_slave: &mut BleKeyboardSlave, key: &Key) {
    // combine the row and the col to a single byte before sending
    //
    // row: 0 - 3; col: 0 - 11
    //
    // Example:
    // row(2):    0000 0010 << 4bits
    // col(11):   0000 1011
    //
    // combined = 0010 1011
    //
    let combined_key = (key.row << BIT_MASK) | key.col;

    /* check if the key count is less than 6 */
    if !ble_keyboard_slave.keys.contains(&combined_key) {
        /* find the first key slot in the array that is
         * free */
        match ble_keyboard_slave.keys.iter().position(|&value| value == 0) {
            Some(index) => {
                /* add the new key to that position */
                ble_keyboard_slave.keys[index] = combined_key;
            }
            None => { /* there is no free key slot available */ }
        }
    }
}

fn remove_keys(ble_keyboard_slave: &mut BleKeyboardSlave, key: &Key) {
    // combine the row and the col to a single byte before sending
    let combined_key = (key.row << BIT_MASK) | key.col;

    /* find the key slot of the released key */
    match ble_keyboard_slave
        .keys
        .iter()
        .position(|&value| value == combined_key)
    {
        Some(index) => {
            /* remove the key from the key slot */
            ble_keyboard_slave.keys[index] = 0
        }
        None => { /* do nothing */ }
    }
}
pub async fn ble_tx(
    keys_pressed: &spinMutex<FnvIndexMap<Key, Debounce, PRESSED_KEYS_INDEXMAP_SIZE>>,
    ble_status: &spinMutex<BleStatus>,
) {
    /* construct ble slave */
    let mut ble_keyboard_slave: BleKeyboardSlave = BleKeyboardSlave::new();

    /* vec to store the keys needed to be removed */
    let mut pressed_keys_to_remove: Vec<Key, 6> = Vec::new();

    /* flag to set the power mode of the esp */
    let mut power_save_flag: bool = true;

    let mut ble_status_prev: BleStatus = BleStatus::NotConnected;

    /* Run the main loop */
    loop {
        if ble_keyboard_slave.connected() {
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
                ble_keyboard_slave.set_ble_power_save();
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
                                add_keys(&mut ble_keyboard_slave, &key);
                            }
                            /* check if the key is calculated for debounce */
                            KEY_RELEASED => {
                                remove_keys(&mut ble_keyboard_slave, key);
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
                    log::info!("ble_keyboard_slave.keys: {:?}", ble_keyboard_slave.keys);

                    /* sent the new report */
                    ble_keyboard_slave.send_report().await;

                    /* remove the sent keys and empty the vec */
                    while let Some(key) = pressed_keys_to_remove.pop() {
                        keys_pressed.remove(&key).unwrap();
                    }
                }
            }
            /* there must be a delay so the WDT in not triggered */
            delay_ms(1).await;
        } else {
            /* debug log */
            #[cfg(feature = "debug")]
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
}

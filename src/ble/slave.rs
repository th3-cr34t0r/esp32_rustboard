use crate::config::user_config::*;
use crate::debounce::KeyState;
use crate::delay::delay_ms;
use crate::matrix::{Key, StoredKeys};

extern crate alloc;
use super::{BleKeyboardSlave, BleStatus, DebounceCounter};
use alloc::sync::Arc;
use esp32_nimble::{enums::*, utilities::mutex::Mutex, uuid128, BLEAddress, BLEDevice};
use esp_idf_sys::{
    esp_ble_power_type_t_ESP_BLE_PWR_TYPE_ADV, esp_ble_power_type_t_ESP_BLE_PWR_TYPE_DEFAULT,
    esp_ble_power_type_t_ESP_BLE_PWR_TYPE_SCAN,
};
use zerocopy::IntoByteSlice;

impl BleKeyboardSlave {
    pub async fn new() -> Self {
        let device = BLEDevice::take();

        device
            .security()
            .set_auth(AuthReq::Bond)
            .set_io_cap(SecurityIOCap::NoInputNoOutput)
            .resolve_rpa();

        let mut client = device.new_client();

        client
            .connect(
                &BLEAddress::from_str("EC:DA:3B:BD:D7:B6", esp32_nimble::BLEAddressType::Public)
                    .unwrap(),
            )
            .await
            .expect("Unable to connect to server device!");

        client.on_connect(|client| {
            client.update_conn_params(1, 10, 0, 200).unwrap();
        });

        Self {
            client,
            sqc: 0,
            keys: [0; 6],
            key_report: [0; 7],
        }
    }

    async fn send_report(&mut self) {
        let remote_characteristic = self
            .client
            .get_service(uuid128!("fafafafa-fafa-fafa-fafa-fafafafafafa"))
            .await
            .unwrap()
            .get_characteristic(BLE_SLAVE_UUID)
            .await
            .unwrap();

        //increment the sqc counter
        self.sqc += 1;

        //set the sqc info as first byte
        self.key_report = [
            self.sqc.clone(),
            self.keys[0],
            self.keys[1],
            self.keys[2],
            self.keys[3],
            self.keys[4],
            self.keys[5],
        ];

        #[cfg(feature = "debug")]
        log::info!("key_report: {:?}", self.key_report);

        remote_characteristic
            .write_value(self.key_report.into_byte_slice(), false)
            .await
            .expect("Unable to set the new data!");
    }

    pub fn set_ble_power_save(&mut self) {
        //set power save
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
    let combined_key = (key.row << BIT_SHIFT) | key.col;

    //check if the key count is less than 6
    if !ble_keyboard_slave.keys.contains(&combined_key) {
        // find the first key slot in the array that is free
        if let Some(index) = ble_keyboard_slave.keys.iter().position(|&value| value == 0) {
            //add the new key to that position
            ble_keyboard_slave.keys[index] = combined_key;
        }
    }
}

pub async fn ble_tx(pressed_keys: &Arc<Mutex<StoredKeys>>, ble_status: &Arc<Mutex<BleStatus>>) {
    // construct ble slave
    let mut ble_keyboard_slave: BleKeyboardSlave = BleKeyboardSlave::new().await;

    // set ble power to lowest possible
    // ble_keyboard_slave.set_ble_power_save();

    // initate debounce
    let mut key_report_debounce: DebounceCounter = DebounceCounter::new(KEY_REPORT_INTERVAL);

    // Run the main loop
    loop {
        if ble_keyboard_slave.client.connected() {
            // check and store the ble status, then release the lock
            if let Some(mut ble_status) = ble_status.try_lock() {
                *ble_status = BleStatus::Connected;
            }

            // try to lock the hashmap
            if let Some(mut pressed_keys) = pressed_keys.try_lock() {
                // check if there are pressed keys
                if !pressed_keys.index_map.is_empty() {
                    // iter trough the pressed keys
                    for (key, debounce) in pressed_keys.index_map.iter_mut() {
                        // if key state is keyPressed, add it to the key report
                        if let KeyState::KeyPressed = debounce.key_state {
                            add_keys(&mut ble_keyboard_slave, key);
                        }
                    }

                    // only sent the key report if the key report interval has passed ann there is a key pressed
                    if key_report_debounce.is_debounced() {
                        // debug log
                        #[cfg(feature = "debug")]
                        log::info!("ble_keyboard_slave.keys: {:?}", ble_keyboard_slave.keys);

                        // sent the new report
                        ble_keyboard_slave.send_report().await;

                        let mut recovered_key: Key = Key::new(255, 255);

                        // iter trough the combined keys
                        ble_keyboard_slave.keys.iter_mut().for_each(|combined_key| {
                            if *combined_key != 0 {
                                recovered_key.row = *combined_key >> BIT_SHIFT;
                                recovered_key.col = *combined_key & 0x0F;

                                // remove the sent keys and empty the vec
                                pressed_keys.index_map.remove(&recovered_key).unwrap();
                            }
                        });
                    }
                    // reset key_report
                    ble_keyboard_slave.keys.fill(0);
                }
            }
            // there must be a delay so the WDT in not triggered
            delay_ms(1).await;
        } else {
            // debug log
            #[cfg(feature = "debug")]
            log::info!("Keyboard not connected!");

            // check and store the ble status
            if let Some(mut ble_status) = ble_status.try_lock() {
                *ble_status = BleStatus::NotConnected;
            }

            // sleep for 100ms
            delay_ms(100).await;
        }
    }
}

use crate::debounce::KeyState;
use crate::delay::delay_ms;
use crate::matrix::Key;
use crate::{config::user_config::*, debounce::Debounce};

use super::{BleKeyboardSlave, BleStatus};
use embassy_time::Instant;
use esp32_nimble::{enums::*, utilities::mutex::Mutex, uuid128, BLEAddress, BLEDevice};
use esp_idf_sys::{
    esp_ble_power_type_t_ESP_BLE_PWR_TYPE_ADV, esp_ble_power_type_t_ESP_BLE_PWR_TYPE_DEFAULT,
    esp_ble_power_type_t_ESP_BLE_PWR_TYPE_SCAN,
};
use zerocopy::IntoByteSlice;

use heapless::FnvIndexMap;

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
            keys: [0; 6],
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

        remote_characteristic
            .write_value(self.keys.into_byte_slice(), false)
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

pub async fn ble_tx(
    keys_pressed: &Mutex<FnvIndexMap<Key, Debounce, PRESSED_KEYS_INDEXMAP_SIZE>>,
    ble_status: &Mutex<BleStatus>,
) {
    //construct ble slave
    let mut ble_keyboard_slave: BleKeyboardSlave = BleKeyboardSlave::new().await;

    //set ble power to lowest possible
    // ble_keyboard_slave.set_ble_power_save();

    //key report delay elapsed
    let mut last_sent_key_report = Instant::now();

    //variable for storing info about a key pressed event
    let mut has_key_been_pressed: bool;

    //Run the main loop
    loop {
        if ble_keyboard_slave.client.connected() {
            //check and store the ble status, then release the lock
            if let Some(mut ble_status) = ble_status.try_lock() {
                *ble_status = BleStatus::Connected;
            }

            //try to lock the hashmap
            if let Some(mut keys_pressed) = keys_pressed.try_lock() {
                // check if there are pressed keys
                if !keys_pressed.is_empty() {
                    // iter trough the pressed keys
                    for (key, debounce) in keys_pressed.iter_mut() {
                        //check the key debounce state
                        match debounce.key_state {
                            //if key state is keyPressed, add it to the key report
                            KeyState::KeyPressed => {
                                add_keys(&mut ble_keyboard_slave, key);
                            }
                            _ => {
                                //do nothing
                            }
                        }
                    }

                    //check if we have a key pressed
                    has_key_been_pressed =
                        ble_keyboard_slave.keys.iter().any(|&element| element != 0);

                    //only sent the key report if the key report interval has passed ann there is a key pressed
                    if has_key_been_pressed
                        && Instant::now() >= last_sent_key_report + KEY_REPORT_INTERVAL
                    {
                        //debug log
                        #[cfg(feature = "debug")]
                        log::info!("ble_keyboard_slave.keys: {:?}", ble_keyboard_slave.keys);

                        //sent the new report
                        ble_keyboard_slave.send_report().await;

                        let mut recovered_key: Key = Key::new(255, 255);

                        //itter trough the combined keys
                        ble_keyboard_slave.keys.iter_mut().for_each(|combined_key| {
                            if *combined_key != 0 {
                                recovered_key.row = *combined_key >> BIT_SHIFT;
                                recovered_key.col = *combined_key & 0x0F;

                                //remove the sent keys and empty the vec
                                keys_pressed.remove(&recovered_key).unwrap();
                            }
                        });

                        //reset key_report
                        ble_keyboard_slave.keys.fill(0);

                        //store the time the key report has been sent
                        last_sent_key_report = Instant::now();
                    }
                }
            }
            //there must be a delay so the WDT in not triggered
            delay_ms(1).await;
        } else {
            //debug log
            #[cfg(feature = "debug")]
            log::info!("Keyboard not connected!");

            //check and store the ble status
            if let Some(mut ble_status) = ble_status.try_lock() {
                *ble_status = BleStatus::NotConnected;
            }

            //sleep for 100ms
            delay_ms(100).await;
        }
    }
}

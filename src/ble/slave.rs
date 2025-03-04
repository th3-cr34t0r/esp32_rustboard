use crate::debounce::{KEY_PRESSED, KEY_RELEASED};
use crate::delay::delay_ms;
use crate::matrix::Key;
use crate::{config::config::*, debounce::Debounce};

use super::{BleKeyboardSlave, BleStatus};
use esp32_nimble::{enums::*, uuid128, BLEAddress, BLEDevice};
use esp_idf_sys::{
    esp_ble_power_type_t_ESP_BLE_PWR_TYPE_ADV, esp_ble_power_type_t_ESP_BLE_PWR_TYPE_DEFAULT,
    esp_ble_power_type_t_ESP_BLE_PWR_TYPE_SCAN,
};
use zerocopy::{IntoByteSlice, IntoBytes};

use heapless::{FnvIndexMap, Vec};
use spin::Mutex as spinMutex;

impl BleKeyboardSlave {
    pub async fn new() -> Self {
        let device = BLEDevice::take();

        device
            .security()
            .set_auth(AuthReq::all())
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
            client.update_conn_params(5, 10, 0, 200).unwrap();
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
            .write_value(self.keys.as_bytes(), false)
            .await
            .expect("Unable to set the new data!");
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
    let mut ble_keyboard_slave: BleKeyboardSlave = BleKeyboardSlave::new().await;

    /* vec to store the keys needed to be removed */
    let mut pressed_keys_to_remove: Vec<Key, 6> = Vec::new();

    /* flag to set the power mode of the esp */
    let mut power_save_flag: bool = true;

    /* Run the main loop */
    loop {
        if ble_keyboard_slave.client.connected() {
            /* check and store the ble status, then release the lock */

            if let Some(mut ble_status) = ble_status.try_lock() {
                *ble_status = BleStatus::Connected;
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

                    /* debug log */
                    #[cfg(feature = "debug")]
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
            if let Some(mut ble_status) = ble_status.try_lock() {
                *ble_status = BleStatus::NotConnected;
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

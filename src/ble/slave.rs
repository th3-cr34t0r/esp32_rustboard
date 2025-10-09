use crate::ble::KeyboardKeyReport;
use crate::config::user_config::slave::ESP_POWER_LEVEL;
use crate::config::user_config::*;
use crate::delay::delay_ms;
use crate::key_provision::key_provision;
use crate::matrix::{KeyPos, RegisteredMatrixKeys};

extern crate alloc;
use super::{BleKeyboardSlave, BleStatus};
use alloc::sync::Arc;
use esp32_nimble::{enums::*, utilities::mutex::Mutex, uuid128, BLEAddress, BLEDevice};
use esp_idf_sys::{
    esp_ble_power_type_t_ESP_BLE_PWR_TYPE_ADV, esp_ble_power_type_t_ESP_BLE_PWR_TYPE_DEFAULT,
    esp_ble_power_type_t_ESP_BLE_PWR_TYPE_SCAN,
};
use heapless::Vec;
use zerocopy::IntoByteSlice;

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
                &BLEAddress::from_str(MASTER_BLE_MAC, esp32_nimble::BLEAddressType::Public)
                    .unwrap(),
            )
            .await
            .expect("Unable to connect to server device!");

        client.on_connect(|client| {
            client.update_conn_params(5, 20, 0, 200).unwrap();
        });

        Self {
            client,
            current_pressed_keys: [0; 6],
            previous_pressed_keys: [0; 6],
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
            .write_value(self.current_pressed_keys.into_byte_slice(), false)
            .await
            .expect("Unable to write new data to the ble_characteristic!");
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
    fn are_pressed_keys_changed(&mut self) -> bool {
        if self.previous_pressed_keys != self.current_pressed_keys {
            self.previous_pressed_keys = self.current_pressed_keys;
            true
        } else {
            false
        }
    }
}

pub async fn ble_tx(
    pressed_keys: &Arc<Mutex<RegisteredMatrixKeys>>,
    ble_status: &Arc<Mutex<BleStatus>>,
) -> ! {
    // construct ble slave
    let mut ble_keyboard_slave: BleKeyboardSlave = BleKeyboardSlave::new().await;

    // set ble power to lowest possible
    // ble_keyboard_slave.set_ble_power_save();

    let mut keyboard_key_report: KeyboardKeyReport = KeyboardKeyReport::default();

    // vec to store the keys needed to be removed
    let mut pressed_keys_to_remove: Vec<KeyPos, 12> = Vec::new();

    // Run the main loop
    loop {
        if ble_keyboard_slave.client.connected() {
            // check and store the ble status, then release the lock
            if let Some(mut ble_status) = ble_status.try_lock() {
                *ble_status = BleStatus::Connected;
            }

            // call the key provisioning function
            key_provision(
                pressed_keys,
                &mut keyboard_key_report,
                &mut pressed_keys_to_remove,
            )
            .await;

            ble_keyboard_slave.current_pressed_keys = keyboard_key_report.keys;

            // sent the new pressed keys only if they differ from the previous
            if ble_keyboard_slave.are_pressed_keys_changed() {
                // debug log
                #[cfg(feature = "debug")]
                log::info!(
                    "ble_keyboard_slave.keys: {:?}",
                    ble_keyboard_slave.current_pressed_keys
                );
                // sent the new report
                ble_keyboard_slave.send_report().await;
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

use crate::config::config::*;
use crate::matrix::Key;

use super::BleKeyboardSlave;
use esp32_nimble::{
    enums::*, utilities::BleUuid, uuid128, BLEAdvertisementData, BLEDevice, NimbleProperties,
};
use esp_idf_sys::{
    esp_ble_power_type_t_ESP_BLE_PWR_TYPE_ADV, esp_ble_power_type_t_ESP_BLE_PWR_TYPE_DEFAULT,
    esp_ble_power_type_t_ESP_BLE_PWR_TYPE_SCAN,
};

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

        let characteristic = service.lock().create_characteristic(
            uuid128!(BLE_SLAVE_UUID),
            NimbleProperties::READ | NimbleProperties::NOTIFY,
        );

        characteristic.lock().set_value(b"Init value");

        let ble_advertising = device.get_advertising();

        ble_advertising
            .lock()
            .scan_response(false)
            .set_data(
                BLEAdvertisementData::new()
                    .name("rustboard-slave")
                    .add_service_uuid(
                        BleUuid::from_uuid128_string(BLE_SLAVE_UUID)
                            .ok()
                            .expect("Invalid SLAVE UUID"),
                    ),
            )
            .unwrap();

        ble_advertising.lock().start().unwrap();

        #[cfg(feature = "debug")]
        server.ble_gatts_show_local();

        Self {
            server,
            characteristic,
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

    pub async fn ble_tx(&mut self, key: &Key) {
        if self.connected() {
            self.characteristic
                .lock()
                .set_value(format!("{} {}", key.row, key.col).as_bytes())
                .notify();
        }
    }
}

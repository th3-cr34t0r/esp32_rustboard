use crate::ble::BleStatus;
use crate::config::enums::{HidKeys, HidModifiers, KeyType};
use crate::config::{config::*, layers::*};
use crate::debounce::{Debounce, KEY_PRESSED, KEY_RELEASED};
use crate::delay::*;
use crate::matrix::Key;

use super::{BleKeyboardSlave, KeyReport, HID_REPORT_DISCRIPTOR, KEYBOARD_ID, MEDIA_KEYS_ID};
use embassy_futures::select::select;
use esp32_nimble::utilities::BleUuid;
use esp32_nimble::{enums::*, BLEAddress, BLEAdvertisementData, BLEDevice, BLEHIDDevice};
use esp_idf_sys::{
    esp_ble_power_type_t_ESP_BLE_PWR_TYPE_ADV, esp_ble_power_type_t_ESP_BLE_PWR_TYPE_DEFAULT,
    esp_ble_power_type_t_ESP_BLE_PWR_TYPE_SCAN,
};
use heapless::{FnvIndexMap, Vec};
use spin::Mutex as spinMutex;
use zerocopy::IntoBytes;

impl BleKeyboardSlave {
    pub fn new() -> Self {
        let device = BLEDevice::take();
        device
            .security()
            .set_auth(AuthReq::all())
            .set_io_cap(SecurityIOCap::NoInputNoOutput)
            .resolve_rpa();

        let server = device.get_server();

        let ble_advertising = device.get_advertising();

        ble_advertising
            .lock()
            .scan_response(false)
            .set_data(
                BLEAdvertisementData::new()
                    .name("rustboard-slave")
                    .appearance(0x03C1)
                    .add_service_uuid(
                        BleUuid::from_uuid128_string(BLE_SLAVE_UUID)
                            .ok()
                            .expect("Invalid SLAVE UUID"),
                    ),
            )
            .unwrap();

        ble_advertising.lock().start().unwrap();

        Self { server }
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
}

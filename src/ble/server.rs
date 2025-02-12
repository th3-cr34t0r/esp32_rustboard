use crate::ble::ble_send_keys::ble_send_keys;
use crate::ble::BleStatus;
use crate::config::enums::{HidKeys, HidModifiers, KeyType};
use crate::config::{config::*, layers::*};
use crate::debounce::{Debounce, KEY_PRESSED, KEY_RELEASED};
use crate::delay::*;
use crate::matrix::Key;

use super::{BleKeyboard, KeyReport, HID_REPORT_DISCRIPTOR, KEYBOARD_ID, MEDIA_KEYS_ID};
use embassy_futures::select::select;
use esp32_nimble::{enums::*, BLEAddress, BLEAdvertisementData, BLEDevice, BLEHIDDevice};
use esp_idf_sys::{
    esp_ble_power_type_t_ESP_BLE_PWR_TYPE_ADV, esp_ble_power_type_t_ESP_BLE_PWR_TYPE_DEFAULT,
    esp_ble_power_type_t_ESP_BLE_PWR_TYPE_SCAN,
};
use heapless::{FnvIndexMap, Vec};
use spin::Mutex as spinMutex;
use zerocopy::IntoBytes;

impl BleKeyboard {
    pub fn new() -> Self {
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
                    // .appearance(0x03C1)
                    .add_service_uuid(hid.hid_service().lock().uuid()),
            )
            .unwrap();
        ble_advertising.lock().start().unwrap();

        //Client initialization
        let client = device.new_client();

        Self {
            server,
            client,
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
    ble_keyboard: spinMutex<BleKeyboard>,
    keys_pressed: &spinMutex<FnvIndexMap<Key, Debounce, PRESSED_KEYS_INDEXMAP_SIZE>>,
) {
    loop {
        if let Some(ble_keyboard) = ble_keyboard.try_lock() {
            // Try to connect to the right keyboard half
            match ble_keyboard
                .client
                .connect(
                    // client mac address
                    &BLEAddress::from_str(
                        "ec:da:3b:bd:d6:d4",
                        esp32_nimble::BLEAddressType::Public,
                    )
                    .unwrap(),
                )
                .await
            {
                Ok(res) => {
                    log::info!("Successfilly connected! - {:?}", res)
                }
                Err(err) => {
                    log::info!("{err}")
                }
            }

            // Rest of the logic for handling the input.
            // The server shall recieve coordinates from the client,
            // representing the keys pressed form the layout selected.
            //
            // The coordinates shall be placed in the keys_pressed hashmap.

            // Here needs to be the fetch logic.
            delay_ms(1).await;
        }
    }
}

pub async fn ble_send_recieve(
    keys_pressed: &spinMutex<FnvIndexMap<Key, Debounce, PRESSED_KEYS_INDEXMAP_SIZE>>,
    ble_status: &mut BleStatus,
) {
    /* construct ble */
    let ble_keyboard: spinMutex<BleKeyboard> = Mutex::new(BleKeyboard::new());

    select(
        ble_client_recieved(),
        ble_send_keys(&ble_keyboard, keys_pressed, ble_status),
    );
}

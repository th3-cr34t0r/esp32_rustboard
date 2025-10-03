extern crate alloc;

use alloc::sync::Arc;

use crate::ble::BleStatus;
use crate::config::layout::Layout;
use crate::config::user_config::master::ESP_POWER_LEVEL;
use crate::config::user_config::{BLE_SLAVE_UUID, KB_NAME};
use crate::delay::*;
use crate::key_provision::key_provision;
use crate::matrix::{KeyPos, RegisteredMatrixKeys};

use super::{
    BleKeyboardMaster, KeyboardKeyReport, MouseKeyReport, HID_REPORT_DISCRIPTOR, KEYBOARD_ID,
    MEDIA_KEYS_ID, MOUSE_ID,
};

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
    async fn new() -> Self {
        let device = BLEDevice::take();

        // creating server
        device
            .security()
            .set_auth(AuthReq::all())
            .set_io_cap(SecurityIOCap::NoInputNoOutput)
            .resolve_rpa();

        let server = device.get_server();
        let ble_advertising = device.get_advertising();

        // ------------------ SLAVE CHARACTERISTIC INIT ----------------------
        server.on_connect(|server, desc| {
            log::info!("Client connected: {:?}", desc);

            if server.connected_count() < (esp_idf_svc::sys::CONFIG_BT_NIMBLE_MAX_CONNECTIONS as _)
            {
                log::info!("Multi-connect support: start advertising!");
                ble_advertising.lock().start().unwrap();
            }
        });

        let service = server.create_service(uuid128!("fafafafa-fafa-fafa-fafa-fafafafafafa"));

        let input_slave = service.lock().create_characteristic(
            BLE_SLAVE_UUID,
            NimbleProperties::READ | NimbleProperties::WRITE | NimbleProperties::WRITE_NO_RSP,
        );

        // ------------------ HID DEVICES INIT ----------------------
        let mut hid = BLEHIDDevice::new(server);

        let input_keyboard = hid.input_report(KEYBOARD_ID);
        let output_keyboard = hid.output_report(KEYBOARD_ID);
        let input_media_keys = hid.input_report(MEDIA_KEYS_ID);
        let input_mouse = hid.input_report(MOUSE_ID);

        hid.manufacturer("Espressif");
        hid.pnp(0x02, 0x05ac, 0x820a, 0x0210);
        hid.hid_info(0x00, 0x01);
        hid.report_map(HID_REPORT_DISCRIPTOR);
        hid.set_battery_level(100);

        // -------------- BLE START ADVERTIZING ------------------
        ble_advertising
            .lock()
            .scan_response(false)
            .set_data(
                BLEAdvertisementData::new()
                    .name(KB_NAME)
                    .appearance(0x03C0)
                    .add_service_uuid(hid.hid_service().lock().uuid()),
            )
            .unwrap();

        ble_advertising.lock().start().unwrap();

        // on esp32-c3, advertising stops when a device is bonded.
        ble_advertising.lock().on_complete(|_| {
            ble_advertising.lock().start().unwrap();
            log::info!("bonded_addresses: {:?}", device.bonded_addresses());
        });

        Self {
            server,
            input_slave,
            input_keyboard,
            output_keyboard,
            input_media_keys,
            input_mouse,
            current_keyboard_report: KeyboardKeyReport::default(),
            previous_keyboard_report: KeyboardKeyReport::default(),
            current_mouse_report: MouseKeyReport::default(),
            previous_mouse_report: MouseKeyReport::default(),
        }
    }

    /// Get connected status
    fn connected(&self) -> bool {
        self.server.connected_count() > 1
    }

    /// Send keyboard report
    async fn send_keyboard_report(&mut self) {
        // log
        #[cfg(feature = "debug")]
        log::info!(
            "ble_keyboard.current_keyboard_report.keys: {:?}",
            self.current_keyboard_report.keys
        );
        self.input_keyboard
            .lock()
            .set_value(self.current_keyboard_report.as_bytes())
            .notify();
    }

    /// Send mouse report
    async fn send_mouse_report(&mut self) {
        // debug log
        #[cfg(feature = "debug")]
        log::info!(
            "ble_keyboard.current_mouse_report: {:?}",
            self.current_mouse_report.construct()
        );

        self.input_mouse
            .lock()
            .set_value(self.current_mouse_report.construct().as_bytes())
            .notify();
    }

    /// Set BLE Power-save mode
    fn set_ble_power_save(&mut self) {
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

    /// Check if keyboard report changed
    fn is_keyboard_report_changed(&mut self) -> bool {
        if self.previous_keyboard_report != self.current_keyboard_report {
            self.previous_keyboard_report = self.current_keyboard_report;
            true
        } else {
            false
        }
    }

    /// Check if mouse report changed
    fn is_mouse_report_changed(&mut self) -> bool {
        if self.previous_mouse_report != self.current_mouse_report {
            self.previous_mouse_report = self.current_mouse_report;
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

    let mut keyboard_key_report: KeyboardKeyReport = KeyboardKeyReport::default();
    let mut mouse_key_report: MouseKeyReport = MouseKeyReport::default();

    #[cfg(feature = "split")]
    let slave_key_report: Arc<Mutex<[u8; 6]>> = Arc::new(Mutex::new([0; 6]));

    #[cfg(feature = "split")]
    // on_write callback
    ble_keyboard.input_slave.lock().on_write({
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

            // process the keys
            key_provision(
                &pressed_keys,
                #[cfg(feature = "split")]
                &slave_key_report,
                &layout,
                &mut layer,
                &mut keyboard_key_report,
                &mut mouse_key_report,
                &mut pressed_keys_to_remove,
            )
            .await;

            ble_keyboard.current_keyboard_report = keyboard_key_report;
            ble_keyboard.current_mouse_report = mouse_key_report;

            // sent the new keyboard report only if it differes from the previous
            if ble_keyboard.is_keyboard_report_changed() {
                ble_keyboard.send_keyboard_report().await;
            }

            // in case the cursor is being moved
            if ble_keyboard
                .current_mouse_report
                .is_cursor_position_changed()
                || ble_keyboard.is_mouse_report_changed()
            {
                ble_keyboard.send_mouse_report().await;
            }

            // there must be a delay so the WDT in not triggered
            delay_ms(5).await;
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

extern crate alloc;

use alloc::sync::Arc;

use crate::ble::BleStatus;
use crate::config::enums::{HidKeys, HidModifiers, KeyType};
use crate::config::layout::Layout;
use crate::config::user_config::*;
use crate::debounce::KeyState;
use crate::delay::*;
use crate::matrix::{KeyPos, StoredKeys};
use crate::mouse::*;

use super::{
    BleKeyboardMaster, KeyReport, HID_REPORT_DISCRIPTOR_KEYBOARD, KEYBOARD_ID, MEDIA_KEYS_ID,
    MOUSE_ID,
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

        // ------------------ SLAVE CHARACTERISTIC INIT ----------------------
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

        hid.report_map(HID_REPORT_DISCRIPTOR_KEYBOARD);

        hid.set_battery_level(100);

        // -------------- BLE START ADVERTIZING ------------------
        ble_advertising
            .lock()
            .scan_response(false)
            .set_data(
                BLEAdvertisementData::new()
                    .name("Rustboard")
                    .appearance(0x03C1)
                    .add_service_uuid(input_slave.lock().uuid())
                    .add_service_uuid(hid.hid_service().lock().uuid()), // .add_service_uuid(hid_mouse.hid_service().lock().uuid()),
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
            input_slave,
            input_keyboard,
            output_keyboard,
            input_media_keys,
            input_mouse,
            current_keyboard_report: KeyReport::default(),
            previous_keyboard_report: KeyReport::default(),
            current_mouse_report: MouseReport::default(),
            previous_mouse_report: MouseReport::default(),
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
            ble_keyboard.current_keyboard_report.keys.fill(0);

            // release modifiers
            ble_keyboard.current_keyboard_report.modifiers = 0;
        }
        KeyType::Modifier => {
            ble_keyboard.current_keyboard_report.modifiers |= HidModifiers::get_modifier(valid_key);
        }
        KeyType::Mouse => {
            // set the mouse command to the mouse ble characteristic
            ble_keyboard.current_mouse_report.set_command(valid_key);
        }
        KeyType::Key => {
            // check if the key count is less than 6
            if !ble_keyboard
                .current_keyboard_report
                .keys
                .contains(&(*valid_key as u8))
            {
                // find the first key slot in the array that is free
                if let Some(index) = ble_keyboard
                    .current_keyboard_report
                    .keys
                    .iter()
                    .position(|&value| value == 0)
                {
                    // add the new key to that position
                    ble_keyboard.current_keyboard_report.keys[index] = *valid_key as u8
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
            ble_keyboard.current_keyboard_report.keys.fill(0);

            // release modifiers
            ble_keyboard.current_keyboard_report.modifiers = 0;
        }
        KeyType::Modifier => {
            // remove the modifier
            ble_keyboard.current_keyboard_report.modifiers &=
                !HidModifiers::get_modifier(valid_key);
        }
        KeyType::Mouse => {
            // remove the mouse command from the mouse ble characteristic
            ble_keyboard.current_mouse_report.reset_report(valid_key);
        }
        KeyType::Key => {
            // find the key slot of the released key
            if let Some(index) = ble_keyboard
                .current_keyboard_report
                .keys
                .iter()
                .position(|&value| value == *valid_key as u8)
            {
                // remove the key from the key slot
                ble_keyboard.current_keyboard_report.keys[index] = 0
            }
        }
    }
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

            // try to lock the hashmap
            if let Some(mut pressed_keys) = pressed_keys.try_lock() {
                // process slave key report
                pressed_keys.store_keys_slave(&slave_key_report);

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

                    // sent the new keyboard report only if it differes from the previous
                    if ble_keyboard.is_keyboard_report_changed() {
                        ble_keyboard.send_keyboard_report().await;
                    }

                    // in case the cursor is being moved
                    if ble_keyboard
                        .current_mouse_report
                        .is_cursor_position_changed()
                    {
                        ble_keyboard.send_mouse_report().await;
                    }
                    // in case a key has been pressed
                    else if ble_keyboard.is_mouse_report_changed() {
                        ble_keyboard.send_mouse_report().await;
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
            if let Some(mut ble_status) = ble_status.try_lock() {
                *ble_status = BleStatus::NotConnected;
            }

            // sleep for 100ms
            delay_ms(100).await;
        }
    }
}

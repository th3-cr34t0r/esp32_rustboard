// originally: https://github.com/T-vK/ESP32-BLE-Keyboard
#![allow(dead_code)]
extern crate alloc;
use alloc::sync::Arc;

use crate::config::enums::{HidKeys, HidModifiers, KeyType};
use crate::config::{config::*, layers::*};
use crate::debounce::{Debounce, KEY_PRESSED, KEY_RELEASED};
use crate::delay::*;
use crate::matrix::Key;

use embassy_futures::block_on;
use esp32_nimble::{
    enums::*, hid::*, utilities::mutex::Mutex, BLEAdvertisementData, BLECharacteristic, BLEDevice,
    BLEHIDDevice, BLEServer,
};
use esp_idf_sys::{
    esp_ble_power_type_t_ESP_BLE_PWR_TYPE_ADV, esp_ble_power_type_t_ESP_BLE_PWR_TYPE_DEFAULT,
    esp_ble_power_type_t_ESP_BLE_PWR_TYPE_SCAN,
};
use heapless::{FnvIndexMap, Vec};
use spin::Mutex as spinMutex;
use zerocopy::{Immutable, IntoBytes};

const KEYBOARD_ID: u8 = 0x01;
const MEDIA_KEYS_ID: u8 = 0x02;

const HID_REPORT_DISCRIPTOR: &[u8] = hid!(
    (USAGE_PAGE, 0x01), // USAGE_PAGE (Generic Desktop Ctrls)
    (USAGE, 0x06),      // USAGE (Keyboard)
    (COLLECTION, 0x01), // COLLECTION (Application)
    // ------------------------------------------------- Keyboard
    (REPORT_ID, KEYBOARD_ID), //   REPORT_ID (1)
    (USAGE_PAGE, 0x07),       //   USAGE_PAGE (Kbrd/Keypad)
    (USAGE_MINIMUM, 0xE0),    //   USAGE_MINIMUM (0xE0)
    (USAGE_MAXIMUM, 0xE7),    //   USAGE_MAXIMUM (0xE7)
    (LOGICAL_MINIMUM, 0x00),  //   LOGICAL_MINIMUM (0)
    (LOGICAL_MAXIMUM, 0x01),  //   Logical Maximum (1)
    (REPORT_SIZE, 0x01),      //   REPORT_SIZE (1)
    (REPORT_COUNT, 0x08),     //   REPORT_COUNT (8)
    (HIDINPUT, 0x02), //   INPUT (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    (REPORT_COUNT, 0x01), //   REPORT_COUNT (1) ; 1 byte (Reserved)
    (REPORT_SIZE, 0x08), //   REPORT_SIZE (8)
    (HIDINPUT, 0x01), //   INPUT (Const,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    (REPORT_COUNT, 0x05), //   REPORT_COUNT (5) ; 5 bits (Num lock, Caps lock, Scroll lock, Compose, Kana)
    (REPORT_SIZE, 0x01),  //   REPORT_SIZE (1)
    (USAGE_PAGE, 0x08),   //   USAGE_PAGE (LEDs)
    (USAGE_MINIMUM, 0x01), //   USAGE_MINIMUM (0x01) ; Num Lock
    (USAGE_MAXIMUM, 0x05), //   USAGE_MAXIMUM (0x05) ; Kana
    (HIDOUTPUT, 0x02), //   OUTPUT (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    (REPORT_COUNT, 0x01), //   REPORT_COUNT (1) ; 3 bits (Padding)
    (REPORT_SIZE, 0x03), //   REPORT_SIZE (3)
    (HIDOUTPUT, 0x01), //   OUTPUT (Const,Array,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    (REPORT_COUNT, 0x06), //   REPORT_COUNT (6) ; 6 bytes (Keys)
    (REPORT_SIZE, 0x08), //   REPORT_SIZE(8)
    (LOGICAL_MINIMUM, 0x00), //   LOGICAL_MINIMUM(0)
    (LOGICAL_MAXIMUM, 0x65), //   LOGICAL_MAXIMUM(0x65) ; 101 keys
    (USAGE_PAGE, 0x07), //   USAGE_PAGE (Kbrd/Keypad)
    (USAGE_MINIMUM, 0x00), //   USAGE_MINIMUM (0)
    (USAGE_MAXIMUM, 0x65), //   USAGE_MAXIMUM (0x65)
    (HIDINPUT, 0x00),  //   INPUT (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    (END_COLLECTION),  // END_COLLECTION
                       // // ------------------------------------------------- Media Keys
                       // (USAGE_PAGE, 0x0C),         // USAGE_PAGE (Consumer)
                       // (USAGE, 0x01),              // USAGE (Consumer Control)
                       // (COLLECTION, 0x01),         // COLLECTION (Application)
                       // (REPORT_ID, MEDIA_KEYS_ID), //   REPORT_ID (3)
                       // (USAGE_PAGE, 0x0C),         //   USAGE_PAGE (Consumer)
                       // (LOGICAL_MINIMUM, 0x00),    //   LOGICAL_MINIMUM (0)
                       // (LOGICAL_MAXIMUM, 0x01),    //   LOGICAL_MAXIMUM (1)
                       // (REPORT_SIZE, 0x01),        //   REPORT_SIZE (1)
                       // (REPORT_COUNT, 0x10),       //   REPORT_COUNT (16)
                       // (USAGE, 0xB5),              //   USAGE (Scan Next Track)     ; bit 0: 1
                       // (USAGE, 0xB6),              //   USAGE (Scan Previous Track) ; bit 1: 2
                       // (USAGE, 0xB7),              //   USAGE (Stop)                ; bit 2: 4
                       // (USAGE, 0xCD),              //   USAGE (Play/Pause)          ; bit 3: 8
                       // (USAGE, 0xE2),              //   USAGE (Mute)                ; bit 4: 16
                       // (USAGE, 0xE9),              //   USAGE (Volume Increment)    ; bit 5: 32
                       // (USAGE, 0xEA),              //   USAGE (Volume Decrement)    ; bit 6: 64
                       // (USAGE, 0x23, 0x02),        //   Usage (WWW Home)            ; bit 7: 128
                       // (USAGE, 0x94, 0x01),        //   Usage (My Computer) ; bit 0: 1
                       // (USAGE, 0x92, 0x01),        //   Usage (Calculator)  ; bit 1: 2
                       // (USAGE, 0x2A, 0x02),        //   Usage (WWW fav)     ; bit 2: 4
                       // (USAGE, 0x21, 0x02),        //   Usage (WWW search)  ; bit 3: 8
                       // (USAGE, 0x26, 0x02),        //   Usage (WWW stop)    ; bit 4: 16
                       // (USAGE, 0x24, 0x02),        //   Usage (WWW back)    ; bit 5: 32
                       // (USAGE, 0x83, 0x01),        //   Usage (Media sel)   ; bit 6: 64
                       // (USAGE, 0x8A, 0x01),        //   Usage (Mail)        ; bit 7: 128
                       // (HIDINPUT, 0x02), // INPUT (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
                       // (END_COLLECTION), // END_COLLECTION
);

#[derive(IntoBytes, Immutable)]
#[repr(packed, C)]
struct KeyReport {
    modifiers: u8,
    reserved: u8,
    keys: [u8; 6],
}

pub struct BleKeyboard {
    server: &'static mut BLEServer,
    input_keyboard: Arc<Mutex<BLECharacteristic>>,
    output_keyboard: Arc<Mutex<BLECharacteristic>>,
    input_media_keys: Arc<Mutex<BLECharacteristic>>,
    key_report: KeyReport,
}

#[derive(Clone, Copy, Debug)]
pub enum BleStatus {
    Connected,
    NotConnected,
}

impl BleKeyboard {
    pub fn new() -> Self {
        let device = BLEDevice::take();
        device
            .security()
            .set_auth(AuthReq::all())
            .set_io_cap(SecurityIOCap::KeyboardOnly)
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

        #[cfg(feature = "left-side")]
        let name = "RUSTBOARD_LEFT";

        #[cfg(feature = "right-side")]
        let name = "RUSTBOARD_RIGHT";

        let ble_advertising = device.get_advertising();
        ble_advertising
            .lock()
            .scan_response(false)
            .set_data(
                BLEAdvertisementData::new()
                    .name(name)
                    .appearance(0x03C1)
                    .add_service_uuid(hid.hid_service().lock().uuid()),
            )
            .unwrap();
        ble_advertising.lock().start().unwrap();

        Self {
            server,
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

    fn send_report(&mut self) {
        self.input_keyboard
            .lock()
            .set_value(&self.key_report.as_bytes()) // .set_from(&self.key_report)
            .notify();
        esp_idf_svc::hal::delay::Ets::delay_ms(1);
    }

    fn set_ble_power_save(&mut self) {
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

fn add_keys(ble_keyboard: &mut BleKeyboard, valid_key: &HidKeys, layer_state: &mut Layer) {
    /* get the key type */
    match KeyType::check_type(valid_key) {
        KeyType::Macro => {
            let macro_valid_keys = HidKeys::get_macro_sequence(valid_key);
            for valid_key in macro_valid_keys.iter() {
                add_keys(ble_keyboard, valid_key, layer_state);
            }
        }
        KeyType::Layer => {
            /* check and set the layer */
            *layer_state = Layer::Upper;

            /* release all keys */
            ble_keyboard
                .key_report
                .keys
                .iter_mut()
                .for_each(|value| *value = 0);

            /* release modifiers */
            ble_keyboard.key_report.modifiers = 0;
        }
        KeyType::Modifier => {
            ble_keyboard.key_report.modifiers |= HidModifiers::get_modifier(valid_key);
        }
        KeyType::Key => {
            /* check if the key count is less than 6 */
            if !ble_keyboard.key_report.keys.contains(&(*valid_key as u8)) {
                /* find the first key slot in the array that is
                 * free */
                match ble_keyboard
                    .key_report
                    .keys
                    .iter()
                    .position(|&value| value == 0)
                {
                    Some(index) => {
                        /* add the new key to that position */
                        ble_keyboard.key_report.keys[index] = *valid_key as u8
                    }
                    None => { /* there is no free key slot available */ }
                }
            }
        }
    }
}

fn remove_keys(ble_keyboard: &mut BleKeyboard, valid_key: &HidKeys, layer_state: &mut Layer) {
    /* get the key type */
    match KeyType::check_type(valid_key) {
        KeyType::Macro => {
            let macro_valid_keys = HidKeys::get_macro_sequence(valid_key);
            for valid_key in macro_valid_keys.iter() {
                remove_keys(ble_keyboard, valid_key, layer_state);
            }
        }
        KeyType::Layer => {
            /* check and set the layer */
            *layer_state = Layer::Base;

            /* release all keys */
            ble_keyboard
                .key_report
                .keys
                .iter_mut()
                .for_each(|value| *value = 0);

            /* release modifiers */
            ble_keyboard.key_report.modifiers = 0;
        }
        KeyType::Modifier => {
            /* remove the modifier */
            ble_keyboard.key_report.modifiers &= !HidModifiers::get_modifier(valid_key);
        }
        KeyType::Key => {
            /* find the key slot of the released key */
            match ble_keyboard
                .key_report
                .keys
                .iter()
                .position(|&value| value == *valid_key as u8)
            {
                Some(index) => {
                    /* remove the key from the key slot */
                    ble_keyboard.key_report.keys[index] = 0
                }
                None => { /* do nothing */ }
            }
        }
    }
}

pub async fn ble_send_keys(
    keys_pressed: &spinMutex<FnvIndexMap<Key, Debounce, PRESSED_KEYS_INDEXMAP_SIZE>>,
    ble_status: &spinMutex<BleStatus>,
) -> ! {
    /* construct ble */
    let mut ble_keyboard = BleKeyboard::new();

    /* initialize layers */
    let mut layers = Layers::new();

    /* load the specified layout */
    layers.load_layout();

    /* layer state */
    let mut layer_state = Layer::Base;

    /* vec to store the keys needed to be removed */
    let mut pressed_keys_to_remove: Vec<Key, 6> = Vec::new();

    /* flag to set the power mode of the esp */
    let mut power_save_flag: bool = true;

    let mut ble_status_prev: BleStatus = BleStatus::NotConnected;

    /* Run the main loop */
    loop {
        if ble_keyboard.connected() {
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
                ble_keyboard.set_ble_power_save();
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
                                /* get the pressed key */
                                if let Some(valid_key) =
                                    layers.get(&key.row, &key.col, &layer_state)
                                {
                                    add_keys(&mut ble_keyboard, valid_key, &mut layer_state);
                                }
                            }
                            /* check if the key is calculated for debounce */
                            KEY_RELEASED => {
                                /* get the mapped key from the hashmap */
                                if let Some(valid_key) =
                                    layers.get(&key.row, &key.col, &layer_state)
                                {
                                    remove_keys(&mut ble_keyboard, valid_key, &mut layer_state);
                                }
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
                    log::info!(
                        "ble_keyboard.key_report.keys: {:?}",
                        ble_keyboard.key_report.keys
                    );

                    /* sent the new report */
                    ble_keyboard.send_report();

                    /* remove the sent keys and empty the vec */
                    while let Some(key) = pressed_keys_to_remove.pop() {
                        keys_pressed.remove(&key).unwrap();
                    }
                }
            }
            /* there must be a delay so the WDT in not triggered */
            delay_ms(5).await;
        } else {
            #[cfg(feature = "debug")]
            /* debug log */
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

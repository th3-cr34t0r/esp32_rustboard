// originally: https://github.com/T-vK/ESP32-BLE-Keyboard
#![allow(dead_code)]
extern crate alloc;

use alloc::sync::Arc;
use esp32_nimble::BLEDevice;
use esp32_nimble::{hid::*, utilities::mutex::Mutex, BLECharacteristic, BLEServer};
use zerocopy::{Immutable, IntoBytes};

#[cfg(feature = "master")]
pub mod master;

#[cfg(feature = "slave")]
pub mod slave;

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

pub struct BleKeyboardMaster {
    device: &'static mut BLEDevice,
    server: &'static mut BLEServer,
    input_keyboard: Arc<Mutex<BLECharacteristic>>,
    output_keyboard: Arc<Mutex<BLECharacteristic>>,
    input_media_keys: Arc<Mutex<BLECharacteristic>>,
    key_report: KeyReport,
}
pub struct BleKeyboardSlave {
    server: &'static mut BLEServer,
    characteristic: Arc<Mutex<BLECharacteristic>>,
    keys: [u8; 6],
}
#[derive(Clone, Copy, Debug)]
pub enum BleStatus {
    Connected,
    NotConnected,
}

// originally: https://github.com/T-vK/ESP32-BLE-Keyboard
#![allow(dead_code)]

extern crate alloc;
use alloc::sync::Arc;

use embassy_time::{Duration, Instant};
use esp32_nimble::BLEClient;
use esp32_nimble::{hid::*, utilities::mutex::Mutex, BLECharacteristic, BLEServer};
use zerocopy::{Immutable, IntoBytes};

use crate::mouse::*;

#[cfg(feature = "master")]
pub mod master;

#[cfg(feature = "slave")]
pub mod slave;

const KEYBOARD_ID: u8 = 0x01;
const MEDIA_KEYS_ID: u8 = 0x02;
const MOUSE_ID: u8 = 0x03;

const HID_REPORT_DISCRIPTOR_KEYBOARD: &[u8] = hid!(
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
    // ------------------------------------------------- Media Keys
    // (USAGE_PAGE, 0x0C), // USAGE_PAGE (Consumer)
    // (USAGE, 0x01),      // USAGE (Consumer Control)
    // (COLLECTION, 0x01), // COLLECTION (Application)
    // // ---------------------------------------------------
    // (REPORT_ID, MEDIA_KEYS_ID), //   REPORT_ID (2)
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
    // ------------------------------------------------------------ Mouse
    (USAGE_PAGE, 0x01), // USAGE_PAGE (Generic Desktop)
    (USAGE, 0x02),      // USAGE (Mouse)
    (COLLECTION, 0x01), // COLLECTION (Application)
    (USAGE, 0x01),      //   USAGE (Pointer)
    (COLLECTION, 0x00), //   COLLECTION (Physical)
    // ------------------------------------------------- Buttons (Left, Right, Middle, Back, Forward)
    (REPORT_ID, MOUSE_ID),   // REPORT_ID (3)
    (USAGE_PAGE, 0x09),      //     USAGE_PAGE (Button)
    (USAGE_MINIMUM, 0x01),   //     USAGE_MINIMUM (Button 1)
    (USAGE_MAXIMUM, 0x05),   //     USAGE_MAXIMUM (Button 5)
    (LOGICAL_MINIMUM, 0x00), //     LOGICAL_MINIMUM (0)
    (LOGICAL_MAXIMUM, 0x01), //     LOGICAL_MAXIMUM (1)
    (REPORT_SIZE, 0x01),     //     REPORT_SIZE (1)
    (REPORT_COUNT, 0x05),    //     REPORT_COUNT (5)
    (HIDINPUT, 0x02),        //     INPUT (Data, Variable, Absolute) ;5 button bits
    // ------------------------------------------------- Padding
    (REPORT_SIZE, 0x03),  //     REPORT_SIZE (3)
    (REPORT_COUNT, 0x01), //     REPORT_COUNT (1)
    (HIDINPUT, 0x03),     //     INPUT (Constant, Variable, Absolute) ;3 bit padding
    // ------------------------------------------------- X/Y position, Wheel
    (USAGE_PAGE, 0x01),      //     USAGE_PAGE (Generic Desktop)
    (USAGE, 0x30),           //     USAGE (X)
    (USAGE, 0x31),           //     USAGE (Y)
    (USAGE, 0x38, 0x02),     //     USAGE (Wheel)
    (LOGICAL_MINIMUM, 0x81), //     LOGICAL_MINIMUM (-127)
    (LOGICAL_MAXIMUM, 0x7f), //     LOGICAL_MAXIMUM (127)
    (REPORT_SIZE, 0x08),     //     REPORT_SIZE (8)
    (REPORT_COUNT, 0x03),    //     REPORT_COUNT (3)
    (HIDINPUT, 0x06),        //     INPUT (Data, Variable, Relative) ;3 bytes (X,Y,Wheel)
    // ------------------------------------------------- Horizontal wheel
    (USAGE_PAGE, 0x0c),      //     USAGE PAGE (Consumer Devices)
    (USAGE, 0x38),           //     USAGE (AC Pan)
    (LOGICAL_MINIMUM, 0x81), //     LOGICAL_MINIMUM (-127)
    (LOGICAL_MAXIMUM, 0x7f), //     LOGICAL_MAXIMUM (127)
    (REPORT_SIZE, 0x08),     //     REPORT_SIZE (8)
    (REPORT_COUNT, 0x01),    //     REPORT_COUNT (1)
    (HIDINPUT, 0x06),        //     INPUT (Data, Var, Rel)
    (END_COLLECTION),        //   END_COLLECTION
    (END_COLLECTION),        //   END_COLLECTION
);

#[derive(Default, PartialEq, Clone, Copy, IntoBytes, Immutable)]
#[repr(packed, C)]
struct KeyReport {
    modifiers: u8,
    reserved: u8,
    keys: [u8; 6],
}

pub struct BleKeyboardMaster {
    server: &'static mut BLEServer,
    input_slave: Arc<Mutex<BLECharacteristic>>,
    input_keyboard: Arc<Mutex<BLECharacteristic>>,
    output_keyboard: Arc<Mutex<BLECharacteristic>>,
    input_media_keys: Arc<Mutex<BLECharacteristic>>,
    input_mouse: Arc<Mutex<BLECharacteristic>>,
    current_keyboard_report: KeyReport,
    previous_keyboard_report: KeyReport,
    current_mouse_report: MouseReport,
    previous_mouse_report: MouseReport,
}

pub struct BleKeyboardSlave {
    client: BLEClient,
    current_pressed_keys: [u8; 6],
    previous_pressed_keys: [u8; 6],
}

#[derive(Clone, Copy, Debug)]
pub enum BleStatus {
    Connected,
    NotConnected,
}

pub struct DebounceCounter {
    previous_instant: Instant,
    debounce: Duration,
}

impl DebounceCounter {
    pub fn new(debounce: Duration) -> Self {
        DebounceCounter {
            previous_instant: Instant::now(),
            debounce,
        }
    }

    pub fn is_debounced(&mut self) -> bool {
        if self.previous_instant.elapsed() >= self.debounce {
            // previous is now the current
            self.previous_instant = Instant::now();
            true
        } else {
            false
        }
    }
    pub fn reset_debounce(&mut self, debounce_duraiton: Duration) {
        self.previous_instant = Instant::now() + debounce_duraiton;
    }
}

impl Default for DebounceCounter {
    fn default() -> Self {
        DebounceCounter {
            previous_instant: Instant::now(),
            debounce: Duration::from_millis(1),
        }
    }
}

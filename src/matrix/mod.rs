use crate::ble::Debounce;
use crate::config::enums::{Kc, KeyType};
use crate::config::layout::{provide_kb_matrix, Layout};
use crate::config::user_config::*;
use crate::delay::*;
use core::pin::pin;

#[cfg(feature = "master")]
use crate::config::user_config::master::COL_OFFSET;

#[cfg(feature = "slave")]
use crate::config::user_config::slave::COL_OFFSET;

use embassy_time::{Duration, Instant};
use esp_idf_svc::hal::gpio::*;

use esp32_nimble::utilities::mutex::Mutex;
use esp_idf_sys::{
    self as _, esp_bt_controller_disable, gpio_int_type_t_GPIO_INTR_HIGH_LEVEL,
    gpio_num_t_GPIO_NUM_10, gpio_num_t_GPIO_NUM_20, gpio_num_t_GPIO_NUM_6, gpio_num_t_GPIO_NUM_7,
};
use heapless::Vec;

pub use crate::ble::BleStatus;

extern crate alloc;
use alloc::sync::Arc;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct KeyPos {
    pub row: u8,
    pub col: u8,
    pub layer: usize,
}

impl KeyPos {
    pub fn new(row: u8, col: u8, layer: usize) -> KeyPos {
        KeyPos { row, col, layer }
    }
}

impl Default for KeyPos {
    fn default() -> Self {
        KeyPos {
            row: 255,
            col: 255,
            layer: 255,
        }
    }
}

pub struct PinMatrix<'a> {
    pub rows: [PinDriver<'a, AnyIOPin, Output>; ROWS],
    pub cols: [PinDriver<'a, AnyIOPin, Input>; COLS],
    pub registered_local_keys_array: [KeyPos; 6],
}

impl PinMatrix<'_> {
    pub fn new() -> PinMatrix<'static> {
        let mut pin_matrix = provide_kb_matrix();

        // set input ports to proper pull and interrupt type
        for col in pin_matrix.cols.iter_mut() {
            col.set_pull(Pull::Down).ok();
            col.set_interrupt_type(InterruptType::AnyEdge).ok();
        }

        pin_matrix
    }

    /// Enables interrupt on pins for wakeup
    fn set_col_enable_sleep_interrupts(&mut self) {
        for col in self.cols.iter_mut() {
            col.enable_interrupt().ok();
        }
    }

    /// Only used for setting gpios to listen for interrup, so the processor is woken
    fn set_light_sleep_gpio_wakeup_enable(&mut self) {
        unsafe {
            /* set gpios that can wake up the chip */
            esp_idf_sys::gpio_wakeup_enable(
                gpio_num_t_GPIO_NUM_20,
                gpio_int_type_t_GPIO_INTR_HIGH_LEVEL,
            );
            esp_idf_sys::gpio_wakeup_enable(
                gpio_num_t_GPIO_NUM_10,
                gpio_int_type_t_GPIO_INTR_HIGH_LEVEL,
            );
            esp_idf_sys::gpio_wakeup_enable(
                gpio_num_t_GPIO_NUM_7,
                gpio_int_type_t_GPIO_INTR_HIGH_LEVEL,
            );
            esp_idf_sys::gpio_wakeup_enable(
                gpio_num_t_GPIO_NUM_6,
                gpio_int_type_t_GPIO_INTR_HIGH_LEVEL,
            );
        }
    }

    /// Enter light sleep mode
    /// This function sets the home row to high,
    /// and sets the configured gpio to listen for interrupt (key press) in order to wake up the processor
    fn enter_light_sleep_mode(&mut self) {
        // enable interrupts
        self.set_col_enable_sleep_interrupts();

        // set gpio wakeup enable interrup
        self.set_light_sleep_gpio_wakeup_enable();

        // set the home row to high
        self.rows[1].set_high().unwrap();

        // enter sleep mode
        unsafe {
            // disable bt before entering sleep
            esp_bt_controller_disable();

            esp_idf_sys::esp_sleep_enable_gpio_switch(false);

            esp_idf_sys::esp_sleep_enable_gpio_wakeup();

            #[cfg(feature = "debug")]
            log::info!("Entering sleep...");

            // enter sleep
            esp_idf_sys::esp_light_sleep_start();

            #[cfg(feature = "debug")]
            log::info!("Woke up...");

            esp_idf_sys::esp_restart();
            // esp_bt_controller_enable(esp_bt_mode_t_ESP_BT_MODE_BLE);
        }
    }

    #[cfg(feature = "async-scan")]
    /// This is the standard scan mode
    /// Each row is set to high, then each col is checked if it is high or not
    async fn async_scan(
        &mut self,
        pressed_keys: &Arc<Mutex<RegisteredMatrixKeys>>,
        layer: &Arc<Mutex<usize>>,
    ) {
        // initialize counts

        use crate::config::user_config::ASYNC_ROW_WAIT;
        use embassy_futures::select::{select, select_slice, Either};
        use heapless::Vec;

        // check rows and cols
        for (row_count, row) in self.rows.iter_mut().enumerate() {
            // set row to high
            row.set_high().unwrap();

            // delay so pin can propagate
            delay_us(1).await;

            // new scope so cols are accessable as mut
            {
                let mut futures: Vec<_, COLS> = self
                    .cols
                    .iter_mut()
                    .map(|col| col.wait_for_high())
                    .collect();

                match select(
                    select_slice(pin!(futures.as_mut_slice())),
                    delay_ms(ASYNC_ROW_WAIT),
                )
                .await
                {
                    Either::First(_) => {
                        // key is pressed, check all cols
                    }
                    Either::Second(()) => {
                        // set row to low
                        row.set_low().unwrap();
                        // time is up, continue with the next row
                        continue;
                    }
                }
            }

            // check col pins
            for (col_count, col) in self.cols.iter().enumerate() {
                if col.is_high() {
                    // store the key in the buffer
                    if let Some(index) = self
                        .registered_local_keys_array
                        .iter()
                        .position(|&element| element == KeyPos::default())
                    {
                        self.registered_local_keys_array[index] = KeyPos {
                            row: row_count as u8,
                            col: col_count as u8 + COL_OFFSET,
                            layer: *layer.lock(),
                        };
                    }
                }
            }

            // set row to low
            row.set_low().unwrap();
        }

        // store the local pressed keys in the shared pressed keys hashmap
        if let Some(mut pressed_keys) = pressed_keys.try_lock() {
            pressed_keys.store_keys_local(&mut self.registered_local_keys_array);
        }
    }

    #[cfg(not(feature = "async-scan"))]
    /// This is the standard scan mode
    /// Each row is set to high, then each col is checked if it is high or not
    async fn standard_scan(
        &mut self,
        registered_matrix_keys: &Arc<Mutex<RegisteredMatrixKeys>>,
        layer: &Arc<Mutex<usize>>,
    ) {
        // initialize counts
        let mut count: KeyPos = KeyPos::new(0, COL_OFFSET, 255);

        // check rows and cols
        for row in self.rows.iter_mut() {
            // set row to high
            row.set_high().unwrap();

            // delay so pin can propagate
            delay_us(100).await;

            // check if a col is high
            for col in self.cols.iter() {
                // check if a col is set to high (key pressed)
                if col.is_high() {
                    // store the key in the buffer
                    if let Some(index) = self
                        .registered_local_keys_array
                        .iter()
                        .position(|&element| element == KeyPos::default())
                    {
                        self.registered_local_keys_array[index] = KeyPos {
                            row: count.row,
                            col: count.row,
                            layer: *layer.lock(),
                        };
                    }
                }
                // increment col
                count.col += 1;
            }

            // set row to low
            row.set_low().unwrap();

            // increment row
            count.row += 1;

            // reset col count
            count.col = COL_OFFSET;
        }

        // reset row count
        count.row = 0;

        // store the local pressed keys in the shared pressed keys hashmap
        if let Some(mut registered_matrix_keys) = registered_matrix_keys.try_lock() {
            registered_matrix_keys.store_keys_local(&mut self.registered_local_keys_array);
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum KeyState {
    Released,
    Pressed,
}

#[derive(Debug, Clone, Copy)]
pub struct KeyInfo {
    pub pressed_time: Instant,
    pub state: KeyState,
}

impl Default for KeyInfo {
    fn default() -> Self {
        Self {
            pressed_time: Instant::now(),
            state: KeyState::Released,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Key {
    pub keycode: Kc,
    pub position: KeyPos,
    pub info: KeyInfo,
}

#[derive(Debug)]
pub struct RegisteredMatrixKeys {
    pub keys: Vec<Key, REGISTERED_KEYS_ARRAY_SIZE>,
    pub sleep_condition: Debounce,
}

impl RegisteredMatrixKeys {
    pub fn new(sleep_timeout: Duration) -> Self {
        Self {
            keys: Vec::new(),
            sleep_condition: Debounce::new(sleep_timeout),
        }
    }
    /// Transform from Matrix to Hid keys
    pub fn transform_matrix_to_hid(&mut self, layout: &Layout) {
        // get the keycode for every keytype, except for combo keytype
        self.keys.iter_mut().for_each(|key| {
            match KeyType::check_type(&key.keycode) {
                KeyType::Combo => { //skip this type
                }
                _ => {
                    key.keycode = layout.keymap[key.position.layer][key.position.row as usize]
                        [key.position.col as usize];
                }
            }
        });
    }

    /// The main function for stornig the registered key in to the shared pressed keys hashmap
    pub fn store_keys_local(&mut self, registered_local_matrix_keys: &mut [KeyPos; 6]) {
        // store local registered keys, to globally available vec
        registered_local_matrix_keys.iter_mut().for_each(|element| {
            if *element != KeyPos::default() {
                // if the key is available in the vec, update it
                if let Some(index) = self
                    .keys
                    .iter_mut()
                    .position(|registered_key| registered_key.position == *element)
                {
                    self.keys[index].info = KeyInfo {
                        pressed_time: Instant::now(),
                        state: KeyState::Pressed,
                    };
                }
                // else add it
                else {
                    self.keys
                        .push(Key {
                            keycode: Kc::Undf,
                            position: *element,
                            info: KeyInfo {
                                pressed_time: Instant::now(),
                                state: KeyState::Pressed,
                            },
                        })
                        .expect("Registered matrix key Vec allocation full.");
                }

                *element = KeyPos::default();

                // reset sleep debounce
                self.sleep_condition.reset(ENTER_SLEEP_DEBOUNCE);
            }
        });
    }

    /// Store the received slave key report in the local pressed keys hashmap
    pub fn store_keys_slave(
        &mut self,
        slave_key_report: &Arc<Mutex<[u8; 6]>>,
        layer: &Arc<Mutex<usize>>,
    ) {
        // iter trough the received key report
        slave_key_report.lock().iter().for_each(|element| {
            // we don't want to store 0s
            if *element != 0 {
                let slave_element_position = KeyPos {
                    row: *element >> BIT_SHIFT,
                    col: *element & 0x0F,
                    layer: *layer.lock(),
                };

                // if the key is available in the vec, update it
                if let Some(index) = self
                    .keys
                    .iter_mut()
                    .position(|key| key.position == slave_element_position)
                {
                    self.keys[index].info = KeyInfo {
                        pressed_time: Instant::now(),
                        state: KeyState::Pressed,
                    };
                }
                // else add it
                else {
                    self.keys
                        .push(Key {
                            keycode: Kc::Undf,
                            position: slave_element_position,
                            info: KeyInfo {
                                pressed_time: Instant::now(),
                                state: KeyState::Pressed,
                            },
                        })
                        .expect("Registered matrix key Vec allocation full.");
                }

                // reset sleep debounce
                self.sleep_condition.reset(ENTER_SLEEP_DEBOUNCE);
            }
        });
    }
    /// Method for processing of combo keys - if feature enabled
    pub fn process_combos(&mut self, layout: &Layout) {
        for combo_dummy_keycode in layout.combos.iter() {
            let (_combo_vec, combo_keys) = Kc::get_combo(combo_dummy_keycode);

            let current_keys = self
                .keys
                .iter()
                .map(|key| key.keycode)
                .collect::<Vec<Kc, 12>>();

            // check if the key combination matches
            if combo_keys.iter().all(|combo_key| {
                self.keys
                    .iter()
                    .any(|key| key.keycode == *combo_key && key.info.state == KeyState::Pressed)
            }) {
                let mut pressed_time = Instant::now();
                let mut to_remove: Vec<usize, 12> = Vec::new();

                // find the keycodes and add them to be removed from the originally pressed keys
                for (index, keycode) in current_keys.iter().enumerate() {
                    if combo_keys.contains(keycode) {
                        pressed_time = self.keys[index].info.pressed_time;
                        to_remove.push(index).unwrap();
                    }
                }
                // sort them
                to_remove.sort();

                // remove reversed
                for index in to_remove.iter().rev() {
                    let _ = self.keys.remove(*index);
                }

                // add new combo key to be processed
                self.keys
                    .push(Key {
                        keycode: *combo_dummy_keycode,
                        position: KeyPos::default(),
                        info: KeyInfo {
                            pressed_time,
                            state: KeyState::Pressed,
                        },
                    })
                    .unwrap();
            } else if current_keys.contains(&combo_dummy_keycode) {
                for (index, keycode) in current_keys.iter().enumerate() {
                    if keycode != combo_dummy_keycode {
                        self.keys.remove(index);
                    }
                }
            }
        }
    }
}

/// The main matrix scan function
pub async fn scan_grid(
    registered_matrix_keys: &Arc<Mutex<RegisteredMatrixKeys>>,
    layer: &Arc<Mutex<usize>>,
    ble_status: &Arc<Mutex<BleStatus>>,
) -> ! {
    // construct the matrix
    let mut matrix = PinMatrix::new();

    // local ble status variable
    let mut ble_status_local: BleStatus = BleStatus::NotConnected;

    // ble status debounce variable
    let mut ble_status_debounce: Debounce = Debounce::new(BLE_STATUS_DEBOUNCE);

    loop {
        // // check if sleep conditions are met
        if let Some(mut pressed_keys) = registered_matrix_keys.try_lock() {
            if pressed_keys.sleep_condition.elapsed() {
                matrix.enter_light_sleep_mode();
            }
        }
        // check and store the ble status, then release the lock
        if ble_status_debounce.elapsed() {
            if let Some(ble_status) = ble_status.try_lock() {
                ble_status_local = *ble_status;
            }
        }

        // if a connection is established, run the key matrix
        match ble_status_local {
            BleStatus::Connected => {
                #[cfg(feature = "async-scan")]
                matrix.async_scan(registered_matrix_keys, layer).await;

                #[cfg(not(feature = "async-scan"))]
                matrix.standard_scan(registered_matrix_keys, layer).await;
            }
            BleStatus::NotConnected => {
                // sleep for 100ms
                delay_ms(100).await;
            }
        }
    }
}

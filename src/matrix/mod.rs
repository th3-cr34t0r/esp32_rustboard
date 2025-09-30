use crate::ble::Debounce;
use crate::config::layout::provide_kb_matrix;
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
use heapless::FnvIndexMap;

pub use crate::ble::BleStatus;
pub use crate::debounce::{KeyInfo, KeyState};

extern crate alloc;
use alloc::sync::Arc;

#[derive(PartialOrd, Ord, Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub struct KeyPos {
    pub row: u8,
    pub col: u8,
}

impl KeyPos {
    pub fn new(row: u8, col: u8) -> KeyPos {
        KeyPos { row, col }
    }
}
pub struct PinMatrix<'a> {
    pub rows: [PinDriver<'a, AnyIOPin, Output>; ROWS],
    pub cols: [PinDriver<'a, AnyIOPin, Input>; COLS],
    pub pressed_keys_array: [KeyPos; 6],
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
    async fn async_scan(&mut self, pressed_keys: &Arc<Mutex<StoredKeys>>) {
        // initialize counts

        use crate::config::user_config::ASYNC_ROW_WAIT;
        use embassy_futures::select::{select, select_slice, Either};
        use heapless::Vec;

        let mut count: KeyPos = KeyPos::new(0, COL_OFFSET);
        let mut is_pressed: bool = false;

        // check rows and cols
        for row in self.rows.iter_mut() {
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
                    delay_us(ASYNC_ROW_WAIT),
                )
                .await
                {
                    Either::First((Ok(_), _)) => {
                        // set flag in case a col pin is interupted
                        is_pressed = true;
                    }
                    Either::First((Err(_), _)) => {}
                    Either::Second(()) => {
                        // time is up, continue with the next row
                    }
                }
            }

            // check flag
            if is_pressed {
                // check col pins
                for col in self.cols.iter() {
                    if col.is_high() {
                        // store the pressed key
                        if let Some(index) = self
                            .pressed_keys_array
                            .iter()
                            .position(|&element| element == KeyPos::new(255, 255))
                        {
                            self.pressed_keys_array[index] = count;
                        }
                    }
                    // increment col
                    count.col += 1;
                }
                // reset flag
                is_pressed = false;
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
        if let Some(mut pressed_keys) = pressed_keys.try_lock() {
            pressed_keys.store_keys_local(&mut self.pressed_keys_array);
        }
    }

    #[cfg(not(feature = "async-scan"))]
    /// This is the standard scan mode
    /// Each row is set to high, then each col is checked if it is high or not
    async fn standard_scan(&mut self, pressed_keys: &Arc<Mutex<StoredKeys>>) {
        // initialize counts
        let mut count: KeyPos = KeyPos::new(0, COL_OFFSET);

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
                        .pressed_keys_array
                        .iter()
                        .position(|&element| element == KeyPos::new(255, 255))
                    {
                        self.pressed_keys_array[index] = count;
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
        if let Some(mut pressed_keys) = pressed_keys.try_lock() {
            pressed_keys.store_keys_local(&mut self.pressed_keys_array);
        }
    }
}

pub struct StoredKeys {
    pub index_map: FnvIndexMap<KeyPos, KeyInfo, PRESSED_KEYS_INDEXMAP_SIZE>,
    pub debounce: Debounce,
}

impl StoredKeys {
    pub fn new(debounce: Duration) -> Self {
        Self {
            index_map: FnvIndexMap::new(),
            debounce: Debounce::new(debounce),
        }
    }
    /// The main function for stornig the registered key in to the shared pressed keys hashmap
    pub fn store_keys_local(&mut self, pressed_keys_array: &mut [KeyPos; 6]) {
        // Inserts a key-value pair into the map.
        // If an equivalent key already exists in the map: the key remains and retains in its place in the order, its corresponding value is updated with value and the older value is returned inside Some(_).
        // If no equivalent key existed in the map: the new key-value pair is inserted, last in order, and None is returned.
        pressed_keys_array.iter_mut().for_each(|element| {
            if *element != KeyPos::new(255, 255) {
                self.index_map
                    .insert(
                        KeyPos {
                            row: element.row,
                            col: element.col,
                        },
                        KeyInfo {
                            pressed_time: Instant::now(),
                            state: KeyState::Pressed,
                        },
                    )
                    .expect("Not enough space to store the pressed keys.");

                *element = KeyPos::new(255, 255);

                // reset sleep debounce
                self.debounce.reset_debounce(ENTER_SLEEP_DEBOUNCE);
            }
        });
    }

    /// Store the received slave key report in the local pressed keys hashmap
    pub fn store_keys_slave(&mut self, slave_key_report: &Arc<Mutex<[u8; 6]>>) {
        // iter trough the received key report
        slave_key_report.lock().iter().for_each(|element| {
            // we don't want to store 0s
            if *element != 0 {
                // add the key_pos and the key_info to the hashmap
                self.index_map
                    .insert(
                        KeyPos {
                            row: *element >> BIT_SHIFT,
                            col: *element & 0x0F,
                        },
                        KeyInfo {
                            pressed_time: Instant::now(),
                            state: KeyState::Pressed,
                        },
                    )
                    .expect("Not enough space to store the slave pressed keys.");

                // reset sleep debounce
                self.debounce.reset_debounce(ENTER_SLEEP_DEBOUNCE);
            }
        });
    }
}

/// The main matrix scan function
pub async fn scan_grid(
    pressed_keys: &Arc<Mutex<StoredKeys>>,
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
        if let Some(mut pressed_keys) = pressed_keys.try_lock() {
            if pressed_keys.debounce.is_debounced() {
                matrix.enter_light_sleep_mode();
            }
        }
        // check and store the ble status, then release the lock
        if ble_status_debounce.is_debounced() {
            if let Some(ble_status) = ble_status.try_lock() {
                ble_status_local = *ble_status;
            }
        }

        // if a connection is established, run the key matrix
        match ble_status_local {
            BleStatus::Connected => {
                #[cfg(feature = "async-scan")]
                matrix.async_scan(pressed_keys).await;

                #[cfg(not(feature = "async-scan"))]
                matrix.standard_scan(pressed_keys).await;
            }
            BleStatus::NotConnected => {
                // sleep for 100ms
                delay_ms(100).await;
            }
        }
    }
}

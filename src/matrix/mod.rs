use crate::ble::DebounceCounter;
use crate::config::user_config::*;
use crate::delay::*;

use embassy_time::Instant;
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals;

use esp32_nimble::utilities::mutex::Mutex;
use esp_idf_sys::{
    self as _, esp_bt_controller_disable, gpio_int_type_t_GPIO_INTR_HIGH_LEVEL,
    gpio_num_t_GPIO_NUM_10, gpio_num_t_GPIO_NUM_20, gpio_num_t_GPIO_NUM_6, gpio_num_t_GPIO_NUM_7,
};

pub use crate::ble::BleStatus;
pub use crate::debounce::{Debounce, KeyState};
pub use heapless::FnvIndexMap;

extern crate alloc;
use alloc::sync::Arc;

#[derive(PartialOrd, Ord, Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub struct Key {
    pub row: u8,
    pub col: u8,
}

impl Key {
    pub fn new(row: u8, col: u8) -> Key {
        Key { row, col }
    }
}
pub struct PinMatrix<'a> {
    pub rows: [PinDriver<'a, AnyIOPin, Output>; ROWS],
    pub cols: [PinDriver<'a, AnyIOPin, Input>; COLS],
    pub pressed_keys_array: [Key; 6],
    pub enter_sleep_debounce: DebounceCounter,
}

impl PinMatrix<'_> {
    pub fn new() -> PinMatrix<'static> {
        let peripherals = Peripherals::take().expect("Not able to init peripherals.");

        let rows = [
            PinDriver::output(peripherals.pins.gpio0.downgrade())
                .expect("Not able to set port as output."),
            PinDriver::output(peripherals.pins.gpio1.downgrade())
                .expect("Not able to set port as output."),
            PinDriver::output(peripherals.pins.gpio2.downgrade())
                .expect("Not able to set port as output."),
            PinDriver::output(peripherals.pins.gpio3.downgrade())
                .expect("Not able to set port as output."),
        ];

        let mut cols = [
            PinDriver::input(peripherals.pins.gpio21.downgrade())
                .expect("Not able to set port as input."),
            PinDriver::input(peripherals.pins.gpio20.downgrade())
                .expect("Not able to set port as input."),
            PinDriver::input(peripherals.pins.gpio10.downgrade())
                .expect("Not able to set port as input."),
            PinDriver::input(peripherals.pins.gpio7.downgrade())
                .expect("Not able to set port as input."),
            PinDriver::input(peripherals.pins.gpio6.downgrade())
                .expect("Not able to set port as input."),
            PinDriver::input(peripherals.pins.gpio5.downgrade())
                .expect("Not able to set port as input."),
        ];

        /* set input ports to proper pull and interrupt type */

        for col in cols.iter_mut() {
            col.set_pull(Pull::Down).ok();
            col.set_interrupt_type(InterruptType::AnyEdge).ok();
        }

        PinMatrix {
            rows,
            cols,
            pressed_keys_array: [Key::new(255, 255); 6],
            enter_sleep_debounce: DebounceCounter::new(SLEEP_DELAY_NOT_CONNECTED),
        }
    }

    /// This function checks if the conditions for entering sleep mode are met
    fn sleep_mode_if_conditions_met(&mut self) {
        /* in case sleep is due */
        if self.enter_sleep_debounce.is_debounced() {
            self.enter_light_sleep_mode();
        }
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
        /* enable interrupts */
        self.set_col_enable_sleep_interrupts();

        /* set gpio wakeup enable interrup */
        self.set_light_sleep_gpio_wakeup_enable();

        /* set the home row to high */
        self.rows[1].set_high().unwrap();

        /* enter sleep mode */
        unsafe {
            /* disable bt before entering sleep */
            esp_bt_controller_disable();

            esp_idf_sys::esp_sleep_enable_gpio_switch(false);

            esp_idf_sys::esp_sleep_enable_gpio_wakeup();

            #[cfg(feature = "debug")]
            log::info!("Entering sleep...");

            /* enter sleep */
            esp_idf_sys::esp_light_sleep_start();

            #[cfg(feature = "debug")]
            log::info!("Woke up...");

            esp_idf_sys::esp_restart();
            // esp_bt_controller_enable(esp_bt_mode_t_ESP_BT_MODE_BLE);
        }
    }

    /// This is the standard scan mode
    /// Each row is set to high, then each col is checked if it is high or not
    async fn standard_scan(&mut self, pressed_keys: &Arc<Mutex<StoredKeys>>) {
        /* initialize counts */
        let mut count: Key = Key::new(0, COL_INIT);

        /* check rows and cols */
        for row in self.rows.iter_mut() {
            /* set row to high */
            row.set_high().unwrap();

            /* delay so pin can propagate */
            delay_us(100).await;

            /* check if a col is high */
            for col in self.cols.iter() {
                /* check if a col is set to high (key pressed) */
                if col.is_high() {
                    /* store the key in the buffer */
                    match self
                        .pressed_keys_array
                        .iter()
                        .position(|&element| element == Key::new(255, 255))
                    {
                        Some(index) => {
                            self.pressed_keys_array[index] = count;
                        }
                        None => {
                            // do nothing
                        }
                    }
                    // reset the sleep delay on key press
                    self.enter_sleep_debounce.reset_debounce(SLEEP_DELAY);
                }
                /* increment col */
                count.col += 1;
            }
            /* set row to low */
            row.set_low().unwrap();

            /* increment row */
            count.row += 1;

            /* reset col count */
            count.col = COL_INIT;
        }

        /* reset row count */
        count.row = 0;

        if let Some(mut pressed_keys) = pressed_keys.try_lock() {
            pressed_keys.store_key(&mut self.pressed_keys_array);
        }
    }
}

#[derive(Default)]
pub struct StoredKeys {
    pub index_map: FnvIndexMap<Key, Debounce, PRESSED_KEYS_INDEXMAP_SIZE>,
}

impl StoredKeys {
    /// The main function for stornig the registered key in to the shared pressed keys hashmap
    pub fn store_key(&mut self, pressed_keys_array: &mut [Key; 6]) {
        // Inserts a key-value pair into the map.
        // If an equivalent key already exists in the map: the key remains and retains in its place in the order, its corresponding value is updated with value and the older value is returned inside Some(_).
        // If no equivalent key existed in the map: the new key-value pair is inserted, last in order, and None is returned.
        pressed_keys_array.iter_mut().for_each(|element| {
            if *element != Key::new(255, 255) {
                self.index_map
                    .insert(
                        Key {
                            row: element.row,
                            col: element.col,
                        },
                        Debounce {
                            key_pressed_time: Instant::now(),
                            key_state: KeyState::KeyPressed,
                        },
                    )
                    .unwrap();

                *element = Key::new(255, 255);
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
    let mut ble_status_debounce: DebounceCounter = DebounceCounter::new(BLE_STATUS_DEBOUNCE_DELAY);

    loop {
        // check if sleep conditions are met
        matrix.sleep_mode_if_conditions_met();

        // check and store the ble status, then release the lock
        if ble_status_debounce.is_debounced() {
            if let Some(ble_status) = ble_status.try_lock() {
                ble_status_local = *ble_status;
            }
        }

        // if a connection is established, run the key matrix
        match ble_status_local {
            BleStatus::Connected => {
                matrix.standard_scan(pressed_keys).await;
            }
            BleStatus::NotConnected => {
                // sleep for 100ms
                delay_ms(100).await;
            }
        }
    }
}

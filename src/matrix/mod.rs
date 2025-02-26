extern crate alloc;
use alloc::sync::Arc;

use crate::ble::BleStatus;
use crate::debounce::KEY_PRESSED;
use crate::delay::*;
use crate::{config::config::*, debounce::Debounce};
use embassy_time::Instant;
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals;

use esp_idf_sys::{
    self as _, esp_bt_controller_disable, gpio_int_type_t_GPIO_INTR_HIGH_LEVEL,
    gpio_num_t_GPIO_NUM_10, gpio_num_t_GPIO_NUM_20, gpio_num_t_GPIO_NUM_6, gpio_num_t_GPIO_NUM_7,
};

use heapless::FnvIndexMap;
use spin::Mutex;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub struct Key {
    pub row: u8,
    pub col: u8,
}

impl Key {
    fn new(row: u8, col: u8) -> Key {
        Key { row, col }
    }
}
pub struct PinMatrix<'a> {
    pub rows: [PinDriver<'a, AnyIOPin, Output>; ROWS],
    pub cols: [PinDriver<'a, AnyIOPin, Input>; COLS],
    pub enter_sleep_delay: Instant,
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
            enter_sleep_delay: Instant::now() + SLEEP_DELAY_NOT_CONNECTED,
        }
    }

    /// This function checks if the conditions for entering sleep mode are met
    fn sleep_mode_if_conditions_met(&mut self) {
        /* in case sleep is due */
        if Instant::now() >= self.enter_sleep_delay {
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
    async fn scan(
        &mut self,
        keys_pressed: &Arc<Mutex<FnvIndexMap<Key, Debounce, PRESSED_KEYS_INDEXMAP_SIZE>>>,
    ) {
        // initialize buffer
        let mut stored_keys_buffer: [Key; 6] = [Key::new(255, 255); 6];

        /* initialize counts */
        let mut count = Key::new(0, 0);

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
                    /* store the key */

                    match stored_keys_buffer
                        .iter()
                        .position(|&value| value == Key::new(255, 255))
                    {
                        Some(index) => {
                            stored_keys_buffer[index] = count;
                        }
                        None => {}
                    }

                    self.enter_sleep_delay = Instant::now() + SLEEP_DELAY;
                }
                /* increment col */
                count.col += 1;
            }
            /* set row to low */
            row.set_low().unwrap();

            /* increment row */
            count.row += 1;

            /* reset col count */
            count.col = 0;
        }

        /* reset row count */
        count.row = 0;

        // store the keys in the hashmap
        store_keys(&keys_pressed, &mut stored_keys_buffer);
    }
}

/// The main function for stornig the registered key in to the shared pressed keys hashmap
fn store_keys(
    keys_pressed: &Arc<Mutex<FnvIndexMap<Key, Debounce, PRESSED_KEYS_INDEXMAP_SIZE>>>,
    keys_pressed_buffer: &mut [Key; 6],
) {
    /* lock the hashmap */
    if let Some(mut keys_pressed) = keys_pressed.try_lock() {
        /* Inserts a key-value pair into the map.
         * If an equivalent key already exists in the map: the key remains and retains in its place in the order, its corresponding value is updated with value and the older value is returned inside Some(_).
         * If no equivalent key existed in the map: the new key-value pair is inserted, last in order, and None is returned.
         */
        keys_pressed_buffer.iter_mut().for_each(|element| {
            if *element != Key::new(255, 255) {
                keys_pressed
                    .insert(
                        Key {
                            row: element.row,
                            col: element.col,
                        },
                        Debounce {
                            key_pressed_time: Instant::now(),
                            key_state: KEY_PRESSED,
                        },
                    )
                    .expect("Error setting new key in the hashmap");

                *element = Key::new(255, 255);
            }
        });

        #[cfg(feature = "debug")]
        log::info!("Pressed keys stored! {:?}", keys_pressed);
    }
}

/// The main matrix scan function
pub async fn scan_grid(
    keys_pressed: &Arc<Mutex<FnvIndexMap<Key, Debounce, PRESSED_KEYS_INDEXMAP_SIZE>>>,
    ble_status: &Mutex<BleStatus>,
) -> ! {
    /* construct the matrix */
    let mut matrix = PinMatrix::new();

    /* local ble status variable */
    let mut ble_status_local: BleStatus = BleStatus::NotConnected;
    let mut ble_status_last_timestamp: Instant = Instant::now();

    let mut current_timestamp: Instant;

    loop {
        /* check if sleep conditions are met */
        matrix.sleep_mode_if_conditions_met();

        /* check and store the ble status, then release the lock */
        current_timestamp = Instant::now();
        if current_timestamp >= ble_status_last_timestamp + BLE_STATUS_DEBOUNCE_DELAY {
            if let Some(ble_status) = ble_status.try_lock() {
                ble_status_local = *ble_status;
                ble_status_last_timestamp = current_timestamp;

                #[cfg(feature = "debug")]
                log::info!("Entered ble status check");
            }
        }

        /* if a connection is established, run the key matrix */
        match ble_status_local {
            BleStatus::Connected => {
                // scan the matrix
                matrix.scan(keys_pressed).await;
            }
            BleStatus::NotConnected => {
                /* wait till there is a connection */
                /* sleep for 100ms */
                delay_ms(100).await;
            }
        }
    }
}

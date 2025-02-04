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

#[cfg(feature = "async-scan")]
use embassy_futures::select::{select, Either};

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub struct Key {
    pub row: i8,
    pub col: i8,
}

impl Key {
    fn new(row: i8, col: i8) -> Key {
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

            /* restart the cpu, so we have faster ble connection after sleep */
            esp_idf_sys::esp_restart();
        }
    }

    /// This is the standard scan mode
    /// Each row is set to high, then each col is checked if it is high or not
    #[cfg(not(feature = "async-scan"))]
    async fn standard_scan(
        &mut self,
        keys_pressed: &Mutex<FnvIndexMap<Key, Debounce, PRESSED_KEYS_INDEXMAP_SIZE>>,
    ) {
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
                    match store_key(keys_pressed, &count) {
                        Some(()) => {
                            self.enter_sleep_delay = Instant::now() + SLEEP_DELAY;
                        }
                        None => { /* do nothing */ }
                    }
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
    }

    /// This is an experimental async way of detecting if a key is pressed
    /// Currently, does not work properly
    #[cfg(feature = "async-scan")]
    async fn async_scan(
        &mut self,
        keys_pressed: &Mutex<FnvIndexMap<Key, Debounce, PRESSED_KEYS_INDEXMAP_SIZE>>,
    ) {
        let mut count = Key::new(0, 0);

        self.set_col_enable_interrupts();

        for row in self.rows.iter_mut() {
            row.set_high().ok();

            delay_us(1).await;

            for col in self.cols.iter_mut() {
                match select(col.wait_for_high(), delay_us(50)).await {
                    Either::First(Ok(_)) => match store_key(keys_pressed, &count) {
                        Some(()) => {
                            self.enter_sleep_delay = Instant::now() + SLEEP_DELAY;
                        }
                        None => { /* do nothing */ }
                    },
                    Either::First(Err(_)) => { /* do nothing */ }
                    Either::Second(()) => { /* in case the delay is up, continue to the next col */
                    }
                }

                /* intrement col count */
                count.col += 1;
            }

            /* reset col count */
            count.col = 0;

            /* intrement row count */
            count.row += 1;
        }
    }
}

/// The main function for stornig the registered key in to the shared pressed keys hashmap
fn store_key(
    keys_pressed: &Mutex<FnvIndexMap<Key, Debounce, PRESSED_KEYS_INDEXMAP_SIZE>>,
    key: &Key,
) -> Option<()> {
    /* lock the hashmap */
    if let Some(mut keys_pressed) = keys_pressed.try_lock() {
        /* Inserts a key-value pair into the map.
         * If an equivalent key already exists in the map: the key remains and retains in its place in the order, its corresponding value is updated with value and the older value is returned inside Some(_).
         * If no equivalent key existed in the map: the new key-value pair is inserted, last in order, and None is returned.
         */
        keys_pressed
            .insert(
                Key {
                    row: key.row,
                    col: key.col,
                },
                Debounce {
                    key_pressed_time: Instant::now(),
                    key_state: KEY_PRESSED,
                },
            )
            .expect("Error setting new key in the hashmap");

        #[cfg(feature = "debug")]
        log::info!("Pressed keys stored! X:{}, Y:{}", key.row, key.col);

        /* return true to reset the sleep delay */
        Some(())
    } else {
        /* else return false */
        None
    }
}

/// The main matrix scan function
pub async fn scan_grid(
    keys_pressed: &Mutex<FnvIndexMap<Key, Debounce, PRESSED_KEYS_INDEXMAP_SIZE>>,
    ble_status: &Mutex<BleStatus>,
) -> ! {
    /* construct the matrix */
    let mut matrix = PinMatrix::new();

    /* local ble status variable */
    let mut ble_status_local: BleStatus = BleStatus::Connected;

    loop {
        /* check if sleep conditions are met */
        matrix.sleep_mode_if_conditions_met();

        /* check and store the ble status, then release the lock */
        // if let Some(ble_status) = ble_status.try_lock() {
        //     ble_status_local = *ble_status;
        // }

        /* if a connection is established, run the key matrix */
        match ble_status_local {
            BleStatus::Connected => {
                #[cfg(not(feature = "async-scan"))]
                matrix.standard_scan(keys_pressed).await;

                #[cfg(feature = "async-scan")]
                matrix.async_scan(keys_pressed).await;
            }
            BleStatus::NotConnected => {
                /* wait till there is a connection */
                /* sleep for 100ms */
                delay_ms(100).await;

                matrix.enter_sleep_delay = Instant::now() + SLEEP_DELAY_NOT_CONNECTED;
            }
        }
    }
}

/*
to build: cargo build --release
to flash: espflash flash ./target/riscv32imc-esp-espidf/release/esp32_rustboard --monitor
*/

use anyhow;
use esp32_rustboard::*;
use esp_idf_hal::task::block_on;

extern crate alloc;
use crate::config::config::*;
use crate::debounce::*;
use crate::matrix::Key;
use alloc::sync::Arc;
use ble::BleStatus;
use heapless::FnvIndexMap;
use matrix::scan_grid;
use spin::Mutex;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    /* Bind the log crate to the ESP Logging facilities */
    esp_idf_svc::log::EspLogger::initialize_default();

    /* initialize keys pressed hashmap */
    let keys_pressed: Arc<Mutex<FnvIndexMap<Key, Debounce, PRESSED_KEYS_INDEXMAP_SIZE>>> =
        Arc::new(Mutex::new(FnvIndexMap::new()));

    /* ble connection information shared variable */
    let ble_status: Arc<Mutex<BleStatus>> = Arc::new(Mutex::new(BleStatus::Connected));

    #[cfg(feature = "master")]
    {
        use crate::ble::master::{ble_rx, ble_tx};
        use ble::BleKeyboardMaster;
        use embassy_futures::select::select4;

        /* run the tasks concurrently */
        block_on(async {
            let ble_keyboard: Arc<Mutex<BleKeyboardMaster>> =
                Arc::new(Mutex::new(BleKeyboardMaster::new().await));

            select4(
                scan_grid(&keys_pressed, &ble_status),
                calculate_debounce(&keys_pressed),
                ble_rx(&ble_keyboard, &keys_pressed),
                ble_tx(&ble_keyboard, &ble_status, &keys_pressed),
            )
            .await;
        });
    }

    #[cfg(feature = "slave")]
    {
        use crate::ble::slave::ble_tx;
        use embassy_futures::select::select3;

        /* run the tasks concurrently */
        block_on(async {
            select3(
                scan_grid(&keys_pressed, &ble_status),
                calculate_debounce(&keys_pressed),
                ble_tx(&keys_pressed, &ble_status),
            )
            .await;
        });
    }

    Ok(())
}

/*
to build: cargo build --release
to flash: espflash flash ./target/riscv32imc-esp-espidf/release/esp32_rustboard --monitor
*/
extern crate alloc;
use alloc::sync::Arc;

use ble::BleStatus;
use embassy_futures::select::select3;
use esp32_rustboard::*;
use esp_idf_hal::task::block_on;
use heapless::FnvIndexMap;
use spin::Mutex;

use crate::ble::ble_send_keys;
use crate::config::user_config::*;
use crate::debounce::*;
use crate::matrix::{scan_grid, Key};

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    /* Bind the log crate to the ESP Logging facilities */
    esp_idf_svc::log::EspLogger::initialize_default();

    /* initialize keys pressed hashmap */
    let keys_pressed: Arc<Mutex<FnvIndexMap<Key, Debounce, PRESSED_KEYS_INDEXMAP_SIZE>>> =
        Arc::new(Mutex::new(FnvIndexMap::new()));

    /* ble connection information shared variable */
    let ble_status: Arc<Mutex<BleStatus>> = Arc::new(Mutex::new(BleStatus::NotConnected));

    /* run the tasks concurrently */
    block_on(async {
        select3(
            ble_send_keys(&keys_pressed, &ble_status),
            scan_grid(&keys_pressed, &ble_status),
            calculate_debounce(&keys_pressed),
        )
        .await;
    });

    Ok(())
}

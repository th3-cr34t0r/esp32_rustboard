/*
to build: cargo build --release
to flash: espflash flash ./target/riscv32imc-esp-espidf/release/esp32_rustboard --monitor
*/

use anyhow;
use esp32_rustboard::*;
use esp_idf_hal::task::block_on;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    /* Bind the log crate to the ESP Logging facilities */
    esp_idf_svc::log::EspLogger::initialize_default();

    #[cfg(feature = "master")]
    {
        use crate::ble::master::ble_rx_tx;
        use crate::config::config::*;
        use crate::debounce::*;
        use crate::matrix::Key;
        use ble::BleStatus;
        use embassy_futures::select::select3;
        use heapless::FnvIndexMap;
        use matrix::scan_grid_master;
        use spin::Mutex;

        /* initialize keys pressed hashmap */
        let keys_pressed: Mutex<FnvIndexMap<Key, Debounce, PRESSED_KEYS_INDEXMAP_SIZE>> =
            Mutex::new(FnvIndexMap::new());

        /* ble connection information shared variable */
        let ble_status: Mutex<BleStatus> = Mutex::new(BleStatus::NotConnected);

        /* run the tasks concurrently */
        block_on(async {
            select3(
                ble_rx_tx(&keys_pressed, &ble_status),
                scan_grid_master(&keys_pressed, &ble_status),
                calculate_debounce(&keys_pressed),
            )
            .await;
        });
    }

    #[cfg(feature = "slave")]
    {
        use matrix::scan_grid_slave;

        /* run the tasks concurrently */
        block_on(async {
            scan_grid_slave().await;
        });
    }

    Ok(())
}

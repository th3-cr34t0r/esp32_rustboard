// to build: cargo build --release
// to flash: espflash flash ./target/riscv32imc-esp-espidf/release/esp32_rustboard --monitor
extern crate alloc;
use alloc::sync::Arc;

use esp32_rustboard::config::user_config::ENTER_SLEEP_DEBOUNCE;
use esp32_rustboard::matrix::StoredKeys;
use esp32_rustboard::*;
use esp_idf_hal::task::block_on;

use crate::debounce::*;
use ble::BleStatus;
use embassy_futures::select::select3;
use esp32_nimble::utilities::mutex::Mutex;
use matrix::scan_grid;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // initialize keys pressed hashmap
    let pressed_keys: Arc<Mutex<StoredKeys>> =
        Arc::new(Mutex::new(StoredKeys::new(ENTER_SLEEP_DEBOUNCE)));

    // ble connection information shared variable
    let ble_status: Arc<Mutex<BleStatus>> = Arc::new(Mutex::new(BleStatus::Connected));

    #[cfg(feature = "master")]
    {
        use crate::ble::master::ble_tx;

        // run the tasks concurrently
        block_on(async {
            select3(
                scan_grid(&pressed_keys, &ble_status),
                calculate_debounce(&pressed_keys),
                ble_tx(&pressed_keys, &ble_status),
            )
            .await;
        });
    }

    #[cfg(feature = "slave")]
    {
        use crate::ble::slave::ble_tx;

        // run the tasks concurrently
        block_on(async {
            select3(
                scan_grid(&pressed_keys, &ble_status),
                calculate_debounce(&pressed_keys),
                ble_tx(&pressed_keys, &ble_status),
            )
            .await;
        });
    }

    Ok(())
}

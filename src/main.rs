// to build: cargo build --release
// to flash: espflash flash ./target/riscv32imc-esp-espidf/release/esp32_rustboard --monitor
extern crate alloc;
use alloc::sync::Arc;

use embassy_futures::select::select3;
use esp32_nimble::utilities::mutex::Mutex;
use esp32_rustboard::ble::BleStatus;
use esp32_rustboard::config::user_config::ENTER_SLEEP_DEBOUNCE;
use esp32_rustboard::debounce::calculate_debounce;
use esp32_rustboard::matrix::{scan_grid, StoredMatrixKeys};
use esp_idf_hal::task::block_on;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // initialize keys pressed hashmap
    let pressed_keys: Arc<Mutex<StoredMatrixKeys>> =
        Arc::new(Mutex::new(StoredMatrixKeys::new(ENTER_SLEEP_DEBOUNCE)));

    // ble connection information shared variable
    let ble_status: Arc<Mutex<BleStatus>> = Arc::new(Mutex::new(BleStatus::Connected));

    block_on(async {
        select3(
            scan_grid(&pressed_keys, &ble_status),
            calculate_debounce(&pressed_keys),
            #[cfg(not(feature = "slave"))]
            esp32_rustboard::ble::master::ble_tx(&pressed_keys, &ble_status),
            #[cfg(feature = "split")]
            #[cfg(feature = "slave")]
            esp32_rustboard::ble::slave::ble_tx(&pressed_keys, &ble_status),
        )
        .await;
    });

    Ok(())
}

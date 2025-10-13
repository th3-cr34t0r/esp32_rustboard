// to build: cargo build --release
// to flash: espflash flash ./target/riscv32imc-esp-espidf/release/esp32_rustboard --monitor
extern crate alloc;
use alloc::sync::Arc;

use embassy_futures::select::select3;
use esp32_nimble::utilities::mutex::Mutex;
use esp32_rustboard::ble::BleStatus;
use esp32_rustboard::config::user_config::ENTER_SLEEP_DEBOUNCE;
use esp32_rustboard::debounce::calculate_debounce;
use esp32_rustboard::matrix::{scan_grid, RegisteredMatrixKeys};
use esp_idf_hal::task::block_on;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // initialize keys pressed hashmap
    let registered_matrix_keys: Arc<Mutex<RegisteredMatrixKeys>> =
        Arc::new(Mutex::new(RegisteredMatrixKeys::new(ENTER_SLEEP_DEBOUNCE)));

    // layer state
    let layer: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));

    // ble connection information shared variable
    let ble_status: Arc<Mutex<BleStatus>> = Arc::new(Mutex::new(BleStatus::Connected));

    block_on(async {
        select3(
            scan_grid(&registered_matrix_keys, &layer, &ble_status),
            calculate_debounce(&registered_matrix_keys),
            #[cfg(feature = "master")]
            esp32_rustboard::ble::master::ble_tx(&registered_matrix_keys, &layer, &ble_status),
            #[cfg(all(feature = "split", feature = "slave"))]
            esp32_rustboard::ble::slave::ble_tx(&registered_matrix_keys, &ble_status),
        )
        .await;
    });

    Ok(())
}

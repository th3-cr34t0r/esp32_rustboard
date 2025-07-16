use embassy_time::Duration;
use esp32_nimble::{utilities::BleUuid, uuid128};

//USER CONFIGURABLE PARAMETERS

// Rows per half
pub const ROWS: usize = 4;
// Cols per half
pub const COLS: usize = 6;
pub const LAYERS: usize = 3;

// Cursor parameters
pub const CURSOR_PARAM_FAST: u8 = 6;
pub const CURSOR_PARAM_NORMAL: u8 = 2;
pub const CURSOR_PARAM_SLOW: u8 = 0;

// Debounce related params
pub const BLE_STATUS_DEBOUNCE: Duration = Duration::from_millis(500); //0.5 sec
pub const ENTER_SLEEP_DEBOUNCE: Duration = Duration::from_millis(600000); //10 minutes

// Indexmap sizes
pub const PRESSED_KEYS_INDEXMAP_SIZE: usize = 32;
pub const LAYER_INDEXMAP_SIZE: usize = 64;

pub const BIT_SHIFT: u8 = 4;
pub const BLE_SLAVE_UUID: BleUuid = uuid128!("06984d74-0fdb-491e-9c4c-c25603a9bc34");

#[cfg(feature = "master")]
pub mod master {
    use crate::EspPowerLevel;
    use embassy_time::Duration;

    pub const COL_OFFSET: u8 = 0;
    pub const KEY_DEBOUNCE: Duration = Duration::from_millis(20);
    pub const ESP_POWER_LEVEL: EspPowerLevel = EspPowerLevel::Negative0;
}

#[cfg(feature = "slave")]
pub mod slave {
    use crate::EspPowerLevel;
    use embassy_time::Duration;

    use super::COLS;

    pub const COL_OFFSET: u8 = COLS as u8;
    pub const KEY_DEBOUNCE: Duration = Duration::from_millis(20);
    pub const ESP_POWER_LEVEL: EspPowerLevel = EspPowerLevel::Negative0;
    pub const MASTER_BLE_MAC: &str = "E4:B0:63:22:EB:EA";
}

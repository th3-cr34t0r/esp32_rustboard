use embassy_time::Duration;
use esp32_nimble::utilities::BleUuid;
use esp32_nimble::uuid128;

use esp_idf_sys::{
    esp_power_level_t_ESP_PWR_LVL_N0, esp_power_level_t_ESP_PWR_LVL_N12,
    esp_power_level_t_ESP_PWR_LVL_N15, esp_power_level_t_ESP_PWR_LVL_N18,
    esp_power_level_t_ESP_PWR_LVL_N21, esp_power_level_t_ESP_PWR_LVL_N24,
    esp_power_level_t_ESP_PWR_LVL_N3, esp_power_level_t_ESP_PWR_LVL_N6,
    esp_power_level_t_ESP_PWR_LVL_N9, esp_power_level_t_ESP_PWR_LVL_P12,
    esp_power_level_t_ESP_PWR_LVL_P15, esp_power_level_t_ESP_PWR_LVL_P18,
    esp_power_level_t_ESP_PWR_LVL_P21, esp_power_level_t_ESP_PWR_LVL_P3,
    esp_power_level_t_ESP_PWR_LVL_P6, esp_power_level_t_ESP_PWR_LVL_P9,
};

/* USER CONFIGURABLE PARAMETERS */
pub const ROWS: usize = 4;
pub const COLS: usize = 6;

#[cfg(feature = "master")]
pub const COL_INIT: u8 = 0;
#[cfg(feature = "master")]
pub const DEBOUNCE_DELAY: Duration = Duration::from_millis(50);

#[cfg(feature = "slave")]
pub const COL_INIT: u8 = 6;
#[cfg(feature = "slave")]
pub const DEBOUNCE_DELAY: Duration = Duration::from_millis(5);

pub const BLE_STATUS_DEBOUNCE_DELAY: Duration = Duration::from_millis(500); /* 0.5 sec */
pub const SLEEP_DELAY: Duration = Duration::from_millis(300000); /* 5 minutes */
pub const SLEEP_DELAY_NOT_CONNECTED: Duration = Duration::from_millis(60000); /* 1 minute */
pub const PRESSED_KEYS_INDEXMAP_SIZE: usize = 16;
pub const LAYER_INDEXMAP_SIZE: usize = 64;
pub const ESP_POWER_LEVEL: EspPowerLevel = EspPowerLevel::Negative0;
pub const BLE_SLAVE_UUID: BleUuid = uuid128!("06984d74-0fdb-491e-9c4c-c25603a9bc34");
pub const BIT_SHIFT: u8 = 4;

pub enum EspPowerLevel {
    Negative24,
    Negative21,
    Negative18,
    Negative15,
    Negative12,
    Negative9,
    Negative6,
    Negative3,
    Negative0,
    Positive3,
    Positive6,
    Positive9,
    Positive12,
    Positive15,
    Positive18,
    Positive21,
}

impl EspPowerLevel {
    pub fn convert(self) -> u32 {
        match self {
            EspPowerLevel::Negative24 => esp_power_level_t_ESP_PWR_LVL_N24,
            EspPowerLevel::Negative21 => esp_power_level_t_ESP_PWR_LVL_N21,
            EspPowerLevel::Negative18 => esp_power_level_t_ESP_PWR_LVL_N18,
            EspPowerLevel::Negative15 => esp_power_level_t_ESP_PWR_LVL_N15,
            EspPowerLevel::Negative12 => esp_power_level_t_ESP_PWR_LVL_N12,
            EspPowerLevel::Negative9 => esp_power_level_t_ESP_PWR_LVL_N9,
            EspPowerLevel::Negative6 => esp_power_level_t_ESP_PWR_LVL_N6,
            EspPowerLevel::Negative3 => esp_power_level_t_ESP_PWR_LVL_N3,
            EspPowerLevel::Negative0 => esp_power_level_t_ESP_PWR_LVL_N0,
            EspPowerLevel::Positive3 => esp_power_level_t_ESP_PWR_LVL_P3,
            EspPowerLevel::Positive6 => esp_power_level_t_ESP_PWR_LVL_P6,
            EspPowerLevel::Positive9 => esp_power_level_t_ESP_PWR_LVL_P9,
            EspPowerLevel::Positive12 => esp_power_level_t_ESP_PWR_LVL_P12,
            EspPowerLevel::Positive15 => esp_power_level_t_ESP_PWR_LVL_P15,
            EspPowerLevel::Positive18 => esp_power_level_t_ESP_PWR_LVL_P18,
            EspPowerLevel::Positive21 => esp_power_level_t_ESP_PWR_LVL_P21,
        }
    }
}

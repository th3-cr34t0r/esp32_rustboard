pub mod ble;
pub mod config;
pub mod debounce;
pub mod matrix;

pub mod delay {
    use embassy_time::{Duration, Timer};

    pub async fn delay_ms(delay: u64) {
        let duration = Duration::from_millis(delay);
        Timer::after(duration).await;
    }

    pub async fn delay_us(delay: u64) {
        let duration = Duration::from_micros(delay);
        Timer::after(duration).await;
    }
}

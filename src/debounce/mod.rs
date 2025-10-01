use crate::{delay::delay_ms, matrix::StoredMatrixKeys};
use embassy_time::Instant;

#[cfg(feature = "master")]
use crate::config::user_config::master::KEY_DEBOUNCE;

#[cfg(feature = "slave")]
use crate::config::user_config::slave::KEY_DEBOUNCE;

extern crate alloc;
use alloc::sync::Arc;
use esp32_nimble::utilities::mutex::Mutex;

#[derive(Debug)]
pub enum KeyState {
    Pressed,
    Released,
}

#[derive(Debug)]
pub struct KeyInfo {
    pub pressed_time: Instant,
    pub state: KeyState,
}

pub async fn calculate_debounce(pressed_keys: &Arc<Mutex<StoredMatrixKeys>>) -> ! {
    loop {
        // try to get a lock on keys_pressed
        if let Some(mut pressed_keys) = pressed_keys.try_lock() {
            // itter throught the pressed keys
            pressed_keys
                .keys_vec
                .iter_mut()
                .for_each(|(_key_pos, key_info)| {
                    // check if the key has passed the debounce delay or has been released
                    if Instant::now() >= key_info.pressed_time + KEY_DEBOUNCE {
                        // set the key_state to RELEASED
                        key_info.state = KeyState::Released;
                    }
                });
        }
        // there must be a delay so WDT is not triggered
        delay_ms(1).await;
    }
}

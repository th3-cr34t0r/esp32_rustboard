use crate::{
    delay::delay_ms,
    matrix::{KeyState, RegisteredMatrixKeys},
};
use embassy_time::Instant;

#[cfg(feature = "master")]
use crate::config::user_config::master::KEY_DEBOUNCE;

#[cfg(feature = "slave")]
use crate::config::user_config::slave::KEY_DEBOUNCE;

extern crate alloc;
use alloc::sync::Arc;
use esp32_nimble::utilities::mutex::Mutex;

pub async fn calculate_debounce(registered_matrix_keys: &Arc<Mutex<RegisteredMatrixKeys>>) -> ! {
    loop {
        // try to get a lock on keys_pressed
        if let Some(mut registered_matrix_keys_locked) = registered_matrix_keys.try_lock() {
            // iter throught the pressed keys
            registered_matrix_keys_locked
                .keys
                .iter_mut()
                .for_each(|key| {
                    // check if the key has passed the debounce delay or has been released
                    if Instant::now() >= key.info.pressed_time + KEY_DEBOUNCE {
                        // set the key_state to RELEASED
                        key.info.state = KeyState::Released;
                    }
                });
        }
        // there must be a delay so WDT is not triggered
        delay_ms(1).await;
    }
}

use crate::{
    config::user_config::{DEBOUNCE_DELAY, PRESSED_KEYS_INDEXMAP_SIZE},
    delay::delay_ms,
    matrix::Key,
};
use embassy_time::Instant;
use heapless::FnvIndexMap;

use esp32_nimble::utilities::mutex::Mutex;

#[derive(Debug)]
pub enum KeyState {
    KeyPressed,
    KeyReleased,
}

#[derive(Debug)]
pub struct Debounce {
    pub key_pressed_time: Instant,
    pub key_state: KeyState,
}

pub async fn calculate_debounce(
    keys_pressed: &Mutex<FnvIndexMap<Key, Debounce, PRESSED_KEYS_INDEXMAP_SIZE>>,
) -> ! {
    loop {
        /* try to get a lock on keys_pressed */
        if let Some(mut keys_pressed) = keys_pressed.try_lock() {
            /* itter throught the pressed keys */
            keys_pressed.iter_mut().for_each(|(_key, debounce)| {
                /* check if the key has passed the debounce delay or has been released */
                if Instant::now() >= debounce.key_pressed_time + DEBOUNCE_DELAY {
                    /* set the key_state to RELEASED */
                    debounce.key_state = KeyState::KeyReleased;
                }
            });
        }
        /* there must be a delay so WDT is not triggered */
        delay_ms(1).await;
    }
}

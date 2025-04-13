use crate::{config::user_config::DEBOUNCE_DELAY, delay::delay_ms, matrix::StoredKeys};
use embassy_time::{Duration, Instant};

extern crate alloc;
use alloc::sync::Arc;
use esp32_nimble::utilities::mutex::Mutex;

#[derive(Debug, PartialEq, Eq)]
pub enum KeyState {
    KeyStateUndefined,
    KeyPressed,
    KeyReleased,
    ModifierPressed,
    ModifierReleased,
}

#[derive(Debug)]
pub struct Debounce {
    pub initial_press: Instant,
    pub last_press: Instant,
    pub state: KeyState,
}

#[cfg(feature = "master")]
pub async fn process_key_state(pressed_keys: &Arc<Mutex<StoredKeys>>) -> ! {
    loop {
        // try to get a lock on keys_pressed
        if let Some(mut pressed_keys) = pressed_keys.try_lock() {
            // itter throught the pressed keys
            pressed_keys
                .index_map
                .iter_mut()
                .for_each(|(_key, debounce)| {
                    // check if the key has passed the debounce delay or has been released
                    match debounce.state {
                        KeyState::KeyStateUndefined => {
                            if debounce.initial_press.elapsed() >= Duration::from_millis(200)
                                && debounce.last_press.elapsed() <= Duration::from_millis(2)
                            {
                                debounce.state = KeyState::ModifierPressed;
                            } else if debounce.last_press.elapsed() >= Duration::from_millis(2) {
                                debounce.state = KeyState::KeyPressed;
                            }
                        }
                        _ => {
                            if debounce.last_press.elapsed() >= DEBOUNCE_DELAY {
                                match debounce.state {
                                    KeyState::KeyPressed => debounce.state = KeyState::KeyReleased,
                                    KeyState::ModifierPressed => {
                                        debounce.state = KeyState::ModifierReleased
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }

                    // log
                    #[cfg(feature = "debug")]
                    log::info!(
                        "\nkey: {:?},\ninitial_press: {:?},\nlast_press: {:?},\nstate: {:?}\n",
                        _key,
                        debounce.initial_press.elapsed().as_millis(),
                        debounce.last_press.elapsed().as_millis(),
                        debounce.state
                    );

                    // if debounce.initial_press.elapsed() < DEBOUNCE_DELAY
                    //     && debounce.last_press.elapsed() >= Duration::from_millis(3)
                    // {
                    //     debounce.state = KeyState::KeyPressed;
                    // } else if (debounce.initial_press.elapsed() >= Duration::from_millis(500))
                    //     && (debounce.last_press.elapsed() <= Duration::from_millis(3))
                    // {
                    //     debounce.state = KeyState::ModifierPressed;
                    // } else if debounce.last_press.elapsed() >= DEBOUNCE_DELAY {
                    //     match debounce.state {
                    //         KeyState::KeyPressed => debounce.state = KeyState::KeyReleased,
                    //         KeyState::ModifierPressed => {
                    //             debounce.state = KeyState::ModifierReleased
                    //         }
                    //         _ => {}
                    //     }
                    // }
                });
        }
        // there must be a delay so WDT is not triggered
        delay_ms(1).await;
    }
}

#[cfg(feature = "slave")]
pub async fn calculate_debounce(pressed_keys: &Arc<Mutex<StoredKeys>>) -> ! {
    loop {
        // try to get a lock on keys_pressed
        if let Some(mut pressed_keys) = pressed_keys.try_lock() {
            // itter throught the pressed keys
            pressed_keys
                .index_map
                .iter_mut()
                .for_each(|(_key, debounce)| {
                    // check if the key has passed the debounce delay or has been released
                    if debounce.state == KeyState::KeyStateUndefined {
                        debounce.state = KeyState::KeyPressed;
                    } else if debounce.last_press.elapsed() >= DEBOUNCE_DELAY {
                        // set the key_state to RELEASED
                        debounce.state = KeyState::KeyReleased;
                    }
                });
        }
        // there must be a delay so WDT is not triggered
        delay_ms(1).await;
    }
}

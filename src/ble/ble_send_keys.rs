use crate::ble::BleStatus;
use crate::config::{config::*, layers::*};
use crate::delay::*;
use spin::Mutex as spinMutex;

use super::BleKeyboard;
use heapless::{FnvIndexMap, Vec};

use crate::debounce::{Debounce, KEY_PRESSED, KEY_RELEASED};
use crate::{
    config::{
        enums::{HidKeys, HidModifiers, KeyType},
        layers::Layer,
    },
    matrix::Key,
};

pub fn send_keys(ble_keyboard: &mut BleKeyboard, valid_key: &HidKeys, layer_state: &mut Layer) {
    /* get the key type */
    match KeyType::check_type(valid_key) {
        KeyType::Macro => {
            let macro_valid_keys = HidKeys::get_macro_sequence(valid_key);
            for valid_key in macro_valid_keys.iter() {
                send_keys(ble_keyboard, valid_key, layer_state);
            }
        }
        KeyType::Layer => {
            /* check and set the layer */
            *layer_state = Layer::Upper;

            /* release all keys */
            ble_keyboard
                .key_report
                .keys
                .iter_mut()
                .for_each(|value| *value = 0);

            /* release modifiers */
            ble_keyboard.key_report.modifiers = 0;
        }
        KeyType::Modifier => {
            ble_keyboard.key_report.modifiers |= HidModifiers::get_modifier(valid_key);
        }
        KeyType::Key => {
            /* check if the key count is less than 6 */
            if !ble_keyboard.key_report.keys.contains(&(*valid_key as u8)) {
                /* find the first key slot in the array that is
                 * free */
                match ble_keyboard
                    .key_report
                    .keys
                    .iter()
                    .position(|&value| value == 0)
                {
                    Some(index) => {
                        /* add the new key to that position */
                        ble_keyboard.key_report.keys[index] = *valid_key as u8
                    }
                    None => { /* there is no free key slot available */ }
                }
            }
        }
    }
}

fn remove_keys(ble_keyboard: &mut BleKeyboard, valid_key: &HidKeys, layer_state: &mut Layer) {
    /* get the key type */
    match KeyType::check_type(valid_key) {
        KeyType::Macro => {
            let macro_valid_keys = HidKeys::get_macro_sequence(valid_key);
            for valid_key in macro_valid_keys.iter() {
                remove_keys(ble_keyboard, valid_key, layer_state);
            }
        }
        KeyType::Layer => {
            /* check and set the layer */
            *layer_state = Layer::Base;

            /* release all keys */
            ble_keyboard
                .key_report
                .keys
                .iter_mut()
                .for_each(|value| *value = 0);

            /* release modifiers */
            ble_keyboard.key_report.modifiers = 0;
        }
        KeyType::Modifier => {
            /* remove the modifier */
            ble_keyboard.key_report.modifiers &= !HidModifiers::get_modifier(valid_key);
        }
        KeyType::Key => {
            /* find the key slot of the released key */
            match ble_keyboard
                .key_report
                .keys
                .iter()
                .position(|&value| value == *valid_key as u8)
            {
                Some(index) => {
                    /* remove the key from the key slot */
                    ble_keyboard.key_report.keys[index] = 0
                }
                None => { /* do nothing */ }
            }
        }
    }
}

pub async fn ble_send_keys(
    keys_pressed: &spinMutex<FnvIndexMap<Key, Debounce, PRESSED_KEYS_INDEXMAP_SIZE>>,
    ble_status: &spinMutex<BleStatus>,
) -> ! {
    /* initialize layers */
    let mut layers = Layers::new();

    /* load the specified layout */
    layers.load_layout();

    /* layer state */
    let mut layer_state = Layer::Base;

    /* vec to store the keys needed to be removed */
    let mut pressed_keys_to_remove: Vec<Key, 6> = Vec::new();

    /* flag to set the power mode of the esp */
    let mut power_save_flag: bool = true;

    let mut ble_status_prev: BleStatus = BleStatus::NotConnected;

    /* Run the main loop */
    loop {
        if ble_keyboard.connected() {
            /* check and store the ble status, then release the lock */
            match ble_status_prev {
                BleStatus::NotConnected => {
                    ble_status_prev = BleStatus::Connected;

                    if let Some(mut ble_status) = ble_status.try_lock() {
                        *ble_status = BleStatus::Connected;
                    }
                }
                BleStatus::Connected => {}
            }

            /* check if power save has been set */
            if power_save_flag {
                /* set ble power to lowest possible */
                ble_keyboard.set_ble_power_save();
                /* set flag to false */
                power_save_flag = false;
            }

            /* try to lock the hashmap */
            if let Some(mut keys_pressed) = keys_pressed.try_lock() {
                /* check if there are pressed keys */
                if !keys_pressed.is_empty() {
                    /* iter trough the pressed keys */
                    for (key, debounce) in keys_pressed.iter_mut() {
                        /*check the key debounce state */
                        match debounce.key_state {
                            KEY_PRESSED => {
                                /* get the pressed key */
                                if let Some(valid_key) =
                                    layers.get(&key.row, &key.col, &layer_state)
                                {
                                    send_keys(&mut ble_keyboard, valid_key, &mut layer_state);
                                }
                            }
                            /* check if the key is calculated for debounce */
                            KEY_RELEASED => {
                                /* get the mapped key from the hashmap */
                                if let Some(valid_key) =
                                    layers.get(&key.row, &key.col, &layer_state)
                                {
                                    remove_keys(&mut ble_keyboard, valid_key, &mut layer_state);
                                }
                                /* if key has been debounced, add it to be removed */
                                pressed_keys_to_remove
                                    .push(*key)
                                    .expect("Error adding a key to be removed!");
                            }

                            _ => { /* do nothing */ }
                        }
                    }

                    #[cfg(feature = "debug")]
                    /* debug log */
                    log::info!(
                        "ble_keyboard.key_report.keys: {:?}",
                        ble_keyboard.key_report.keys
                    );

                    /* sent the new report */
                    ble_keyboard.send_report();

                    /* remove the sent keys and empty the vec */
                    while let Some(key) = pressed_keys_to_remove.pop() {
                        keys_pressed.remove(&key).unwrap();
                    }
                }
            }
            /* there must be a delay so the WDT in not triggered */
            delay_ms(5).await;
        } else {
            #[cfg(feature = "debug")]
            /* debug log */
            log::info!("Keyboard not connected!");

            /* check and store the ble status */
            match ble_status_prev {
                BleStatus::NotConnected => {}
                BleStatus::Connected => {
                    ble_status_prev = BleStatus::NotConnected;

                    /* lock the mutex and set the new value */
                    *ble_status.lock() = BleStatus::NotConnected;
                }
            }

            /* check the power save flag */
            if !power_save_flag {
                /* if false, set to true */
                power_save_flag = true;
            }

            /* sleep for 100ms */
            delay_ms(100).await;
        }
    }
}

extern crate alloc;
use alloc::sync::Arc;
use esp32_nimble::utilities::mutex::Mutex;
use heapless::Vec;

use crate::{
    ble::KeyboardKeyReport,
    config::{
        enums::{HidKeys, HidModifiers, KeyType},
        layout::Layout,
        user_config::BIT_SHIFT,
    },
    matrix::{KeyPos, KeyState, StoredMatrixKeys},
    mouse::MouseKeyReport,
};

fn add_keys_master(
    keyboard_key_report: &mut KeyboardKeyReport,
    mouse_key_report: &mut MouseKeyReport,
    valid_key: &HidKeys,
    layer: &mut usize,
) {
    // get the key type
    match KeyType::check_type(valid_key) {
        KeyType::Macro => {
            let macro_valid_keys = HidKeys::get_macro_sequence(valid_key);
            for valid_key in macro_valid_keys.iter() {
                add_keys_master(keyboard_key_report, mouse_key_report, valid_key, layer);
            }
        }
        KeyType::Layer => {
            // check and set the layer
            *layer = Layout::get_layer(valid_key);

            // release all keys
            keyboard_key_report.keys.fill(0);

            // release modifiers
            keyboard_key_report.modifiers = 0;
        }
        KeyType::Modifier => {
            keyboard_key_report.modifiers |= HidModifiers::get_modifier(valid_key);
        }
        KeyType::Mouse => {
            // set the mouse command to the mouse ble characteristic
            mouse_key_report.set_command(valid_key);
        }
        KeyType::Key => {
            // check if the key count is less than 6
            if !keyboard_key_report.keys.contains(&(*valid_key as u8)) {
                // find the first key slot in the array that is free
                if let Some(index) = keyboard_key_report
                    .keys
                    .iter()
                    .position(|&value| value == 0)
                {
                    // add the new key to that position
                    keyboard_key_report.keys[index] = *valid_key as u8
                }
            }
        }
    }
}

fn remove_keys_master(
    keyboard_key_report: &mut KeyboardKeyReport,
    mouse_key_report: &mut MouseKeyReport,
    valid_key: &HidKeys,
    layer: &mut usize,
) {
    // get the key type
    match KeyType::check_type(valid_key) {
        KeyType::Macro => {
            let macro_valid_keys = HidKeys::get_macro_sequence(valid_key);
            for valid_key in macro_valid_keys.iter() {
                remove_keys_master(keyboard_key_report, mouse_key_report, valid_key, layer);
            }
        }
        KeyType::Layer => {
            // set base layer
            *layer -= 1;

            // release all keys
            keyboard_key_report.keys.fill(0);
            mouse_key_report.reset_report();

            // release modifiers
            keyboard_key_report.modifiers = 0;
        }
        KeyType::Modifier => {
            // remove the modifier
            keyboard_key_report.modifiers &= !HidModifiers::get_modifier(valid_key);
        }
        KeyType::Mouse => {
            // remove the mouse command from the mouse ble characteristic
            mouse_key_report.reset_keypress(valid_key);
        }
        KeyType::Key => {
            // find the key slot of the released key
            if let Some(index) = keyboard_key_report
                .keys
                .iter()
                .position(|&value| value == *valid_key as u8)
            {
                // remove the key from the key slot
                keyboard_key_report.keys[index] = 0
            }
        }
    }
}

/// Function that transforms and adds the pressed key on the slave device
/// to the key report which is being sent to the master device for processing
fn add_keys_slave(key_report: &mut KeyboardKeyReport, key: &KeyPos) {
    // combine the row and the col to a single byte before sending
    //
    // row: 0 - 3; col: 0 - 11
    //
    // Example:
    // row(2):    0000 0010 << 4bits
    // col(11):   0000 1011
    //
    // combined = 0010 1011
    //
    let combined_key = (key.row << BIT_SHIFT) | key.col;

    //check if the key count is less than 6
    if !key_report.keys.contains(&combined_key) {
        // find the first key slot in the array that is free
        if let Some(index) = key_report.keys.iter().position(|&value| value == 0) {
            //add the new key to that position
            key_report.keys[index] = combined_key;
        }
    }
}
/// Function that removes the pressed key from the key report
fn remove_keys_slave(keyboard_key_report: &mut KeyboardKeyReport, key: &KeyPos) {
    if let Some(index) = keyboard_key_report
        .keys
        .iter()
        .position(|&element| element == (key.row << BIT_SHIFT) | key.col)
    {
        keyboard_key_report.keys[index] = 0;
    }
}
/// Function that processes the pressed keys
/// Crosschecks the key position with the layout
/// Pnrovides the pressed key from the layout
#[warn(unused_variables)]
pub async fn key_provision(
    pressed_keys: &Arc<Mutex<StoredMatrixKeys>>,
    #[cfg(feature = "split")]
    #[cfg(feature = "master")]
    slave_key_report: &Arc<Mutex<[u8; 6]>>,
    #[cfg(feature = "master")] layout: &Layout,
    #[cfg(feature = "master")] mut layer: &mut usize,
    keyboard_key_report: &mut KeyboardKeyReport,
    #[cfg(feature = "master")] mut mouse_key_report: &mut MouseKeyReport,
    pressed_keys_to_remove: &mut Vec<KeyPos, 6>,
) {
    // try to lock the hashmap
    if let Some(mut pressed_keys) = pressed_keys.try_lock() {
        #[cfg(feature = "split")]
        #[cfg(feature = "master")]
        // process slave key report
        pressed_keys.store_keys_slave(&slave_key_report);

        // check if there are pressed keys
        if !pressed_keys.keys_vec.is_empty() {
            // iter trough the pressed keys
            for (key_pos, key_info) in pressed_keys.keys_vec.iter_mut() {
                // check the key debounce state
                match key_info.state {
                    KeyState::Pressed => {
                        #[cfg(feature = "master")]
                        // get the pressed key from the layout
                        if let Some(valid_key) = layout.keymap[*layer].get(key_pos) {
                            add_keys_master(
                                keyboard_key_report,
                                &mut mouse_key_report,
                                valid_key,
                                &mut layer,
                            );
                        }
                        #[cfg(feature = "slave")]
                        add_keys_slave(keyboard_key_report, &key_pos);
                    }
                    // check if the key is calculated for debounce
                    KeyState::Released => {
                        #[cfg(feature = "master")]
                        // get the mapped key from the hashmap
                        if let Some(valid_key) = layout.keymap[*layer].get(key_pos) {
                            remove_keys_master(
                                keyboard_key_report,
                                &mut *mouse_key_report,
                                valid_key,
                                &mut layer,
                            );
                        }
                        #[cfg(feature = "slave")]
                        remove_keys_slave(keyboard_key_report, &key_pos);

                        // if key has been debounced, add it to be removed
                        pressed_keys_to_remove
                            .push(*key_pos)
                            .expect("Error adding a key to be removed!");
                    }
                }
            }

            // remove the sent keys and empty the vec
            while let Some(key) = pressed_keys_to_remove.pop() {
                pressed_keys.keys_vec.remove(&key).unwrap();
            }
        }
    }
}

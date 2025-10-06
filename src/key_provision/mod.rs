extern crate alloc;
use alloc::sync::Arc;
use esp32_nimble::utilities::mutex::Mutex;
use heapless::Vec;

use crate::{
    ble::KeyboardKeyReport,
    config::{
        enums::{HidModifiers, Kc, KeyType},
        layout::Layout,
        user_config::BIT_SHIFT,
    },
    matrix::{KeyPos, KeyState, RegisteredMatrixKeys},
    mouse::MouseKeyReport,
};

fn add_keys_master(
    keyboard_key_report: &mut KeyboardKeyReport,
    mouse_key_report: &mut MouseKeyReport,
    hid_key: &Kc,
    layer: &Arc<Mutex<usize>>,
) {
    // get the key type
    match KeyType::check_type(hid_key) {
        KeyType::Macro => {
            let macro_valid_keys = Kc::get_macro_sequence(hid_key);
            for valid_key in macro_valid_keys.iter() {
                add_keys_master(keyboard_key_report, mouse_key_report, valid_key, layer);
            }
        }
        KeyType::Layer => {
            // check and set the layer
            *layer.lock() = Layout::get_layer(hid_key);

            // // release all keys
            // keyboard_key_report.keys.fill(0);

            // // release modifiers
            // keyboard_key_report.modifiers = 0;
        }
        KeyType::Modifier => {
            keyboard_key_report.modifiers |= HidModifiers::get_modifier(hid_key);
        }
        KeyType::Mouse => {
            // set the mouse command to the mouse ble characteristic
            mouse_key_report.set_command(hid_key);
        }
        KeyType::Key => {
            // check if the key count is less than 6
            if !keyboard_key_report.keys.contains(&(*hid_key as u8)) {
                // find the first key slot in the array that is free
                if let Some(index) = keyboard_key_report
                    .keys
                    .iter()
                    .position(|&value| value == 0)
                {
                    // add the new key to that position
                    keyboard_key_report.keys[index] = *hid_key as u8
                }
            }
        }
    }
}

fn remove_keys_master(
    keyboard_key_report: &mut KeyboardKeyReport,
    mouse_key_report: &mut MouseKeyReport,
    hid_key: &Kc,
    layer: &Arc<Mutex<usize>>,
) {
    // get the key type
    match KeyType::check_type(hid_key) {
        KeyType::Macro => {
            let macro_valid_keys = Kc::get_macro_sequence(hid_key);
            for valid_key in macro_valid_keys.iter() {
                remove_keys_master(keyboard_key_report, mouse_key_report, valid_key, layer);
            }
        }
        KeyType::Layer => {
            // set previous layer
            *layer.lock() -= 1;

            // // release all keys
            // keyboard_key_report.keys.fill(0);
            // mouse_key_report.reset_report();

            // // release modifiers
            // keyboard_key_report.modifiers = 0;
        }
        KeyType::Modifier => {
            // remove the modifier
            keyboard_key_report.modifiers &= !HidModifiers::get_modifier(hid_key);
        }
        KeyType::Mouse => {
            // remove the mouse command from the mouse ble characteristic
            mouse_key_report.reset_keypress(hid_key);
        }
        KeyType::Key => {
            // find the key index of the released key
            if let Some(index) = keyboard_key_report
                .keys
                .iter()
                .position(|&value| value == *hid_key as u8)
            {
                // remove the key from the key slot
                keyboard_key_report.keys[index] = 0
            }
        }
    }
}

/// Function that transforms and adds the pressed key on the slave device
/// to the key report which is being sent to the master device for processing
fn add_keys_slave(key_report: &mut KeyboardKeyReport, key_pos: &KeyPos) {
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
    let combined_key = (key_pos.row << BIT_SHIFT) | key_pos.col;

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
fn remove_keys_slave(keyboard_key_report: &mut KeyboardKeyReport, key_pos: &KeyPos) {
    if let Some(index) = keyboard_key_report
        .keys
        .iter()
        .position(|&element| element == (key_pos.row << BIT_SHIFT) | key_pos.col)
    {
        keyboard_key_report.keys[index] = 0;
    }
}
/// Function that processes the pressed keys
/// Crosschecks the key position with the layout
/// Pnrovides the pressed key from the layout
#[warn(unused_variables)]
pub async fn key_provision(
    registered_matrix_keys: &Arc<Mutex<RegisteredMatrixKeys>>,
    #[cfg(feature = "split")]
    #[cfg(feature = "master")]
    slave_key_report: &Arc<Mutex<[u8; 6]>>,
    #[cfg(feature = "master")] layout: &Layout,
    #[cfg(feature = "master")] mut layer: &Arc<Mutex<usize>>,
    keyboard_key_report: &mut KeyboardKeyReport,
    #[cfg(feature = "master")] mut mouse_key_report: &mut MouseKeyReport,
    registered_keys_to_remove: &mut Vec<(KeyPos, usize), 12>,
) {
    // try to lock the hashmap
    if let Some(mut registered_matrix_keys) = registered_matrix_keys.try_lock() {
        #[cfg(feature = "split")]
        #[cfg(feature = "master")]
        // process slave key report
        registered_matrix_keys.store_keys_slave(&slave_key_report, &layer);

        #[cfg(feature = "master")]
        #[cfg(feature = "combo")]
        registered_matrix_keys.process_combos(&layout);

        // check if there are pressed keys
        if !registered_matrix_keys.keys.is_empty() {
            // iter trough the pressed keys
            for key in registered_matrix_keys.keys.iter_mut() {
                // check the key debounce state
                match key.info.state {
                    KeyState::Pressed => {
                        #[cfg(feature = "master")]
                        {
                            // get the pressed key from the layout
                            let hid_key = layout.keymap[key.info.layer][key.position.row as usize]
                                [key.position.col as usize];
                            add_keys_master(
                                keyboard_key_report,
                                &mut mouse_key_report,
                                &hid_key,
                                &layer,
                            );
                        }
                        #[cfg(feature = "slave")]
                        add_keys_slave(keyboard_key_report, &key.position);
                    }
                    // check if the key is calculated for debounce
                    KeyState::Released => {
                        #[cfg(feature = "master")]
                        {
                            // get the mapped key from the layout
                            let hid_key = layout.keymap[key.info.layer][key.position.row as usize]
                                [key.position.col as usize];
                            remove_keys_master(
                                keyboard_key_report,
                                &mut *mouse_key_report,
                                &hid_key,
                                &mut layer,
                            );
                        }
                        #[cfg(feature = "slave")]
                        remove_keys_slave(keyboard_key_report, &key.position);

                        // if key has been debounced, add it to be removed
                        registered_keys_to_remove
                            .push((key.position, key.info.layer))
                            .expect("Error adding a key to be removed!");
                    }
                }
            }
            // process combos
            #[cfg(feature = "master")]
            // process_combos(&mut combo_vec, keyboard_key_report);

            // remove the sent keys and empty the vec
            while let Some((key_to_remove_pos, key_to_remove_layer)) =
                registered_keys_to_remove.pop()
            {
                if let Some(index) = registered_matrix_keys.keys.iter().position(|element| {
                    (element.position == key_to_remove_pos)
                        && (element.info.layer == key_to_remove_layer)
                }) {
                    registered_matrix_keys.keys.remove(index);
                }
            }
        }
    }
}
pub fn process_combos(combo_vec: &mut Vec<Kc, 12>, keyboard_key_report: &mut KeyboardKeyReport) {
    let mut combo_key: u8 = 0;
    while let Some(hid_key) = combo_vec.pop() {
        combo_key |= hid_key as u8;
    }

    let combo = Kc::ModCo as u8 | Kc::D as u8;

    match combo_key {
        combo => {
            if let Some(index) = keyboard_key_report
                .keys
                .iter_mut()
                .position(|element| *element == Kc::D as u8)
            {
                keyboard_key_report.keys[index] = Kc::Bksp as u8;
            }
        }
        _ => {}
    }
}

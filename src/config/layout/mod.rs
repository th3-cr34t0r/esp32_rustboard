pub mod dvorak;
pub mod qwerty;

use crate::{
    config::{enums::*, user_config::*},
    matrix::KeyPos,
};
use heapless::FnvIndexMap;

#[derive(Default)]
pub struct Layout {
    pub keymap: [FnvIndexMap<KeyPos, HidKeys, LAYER_INDEXMAP_SIZE>; LAYERS],
}

impl Layout {
    /// initializes the Layers struct with the compiled layout
    pub fn init() -> Layout {
        #[cfg(feature = "dvorak")]
        return dvorak::layout();

        #[cfg(not(feature = "dvorak"))]
        return qwerty::layout();
    }

    /// get the layer number
    pub fn get_layer(layer: &HidKeys) -> usize {
        match layer {
            HidKeys::Layer1 => 1,
            HidKeys::Layer2 => 2,
            HidKeys::Layer3 => 3,
            HidKeys::Layer4 => 4,
            HidKeys::Layer5 => 5,
            _ => 0,
        }
    }
}

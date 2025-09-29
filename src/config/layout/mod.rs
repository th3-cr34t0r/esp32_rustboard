#[cfg(feature = "colemakdh")]
pub mod colemakdh;

#[cfg(feature = "dvorak")]
pub mod dvorak;

#[cfg(feature = "dvorak-5x3")]
pub mod dvorak_5x3;

#[cfg(feature = "dvorak-coral")]
pub mod dvorak_coral;

#[cfg(feature = "dvorak-rosewood")]
pub mod dvorak_rosewood;

#[cfg(feature = "qwerty")]
pub mod qwerty;

use std::collections::HashMap;

use crate::{
    config::{enums::*, user_config::*},
    matrix::KeyPos,
};

#[derive(Default)]
pub struct Layout {
    pub keymap: [HashMap<KeyPos, HidKeys>; LAYERS],
}

impl Layout {
    /// initializes the Layers struct with the compiled layout
    pub fn init() -> Layout {
        #[cfg(feature = "qwerty")]
        return qwerty::layout();

        #[cfg(feature = "dvorak")]
        return dvorak::layout();

        #[cfg(feature = "dvorak-coral")]
        return dvorak_coral::layout();

        #[cfg(feature = "dvorak-rosewood")]
        return dvorak_rosewood::layout();

        #[cfg(feature = "dvorak-5x3")]
        return dvorak_5x3::layout();

        #[cfg(feature = "colemakdh")]
        return colemakdh::layout();
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

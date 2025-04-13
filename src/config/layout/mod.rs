pub mod dvorak;
pub mod qwerty;
use crate::config::{enums::*, user_config::*};
use heapless::FnvIndexMap;

pub enum Layer {
    Base,
    Upper,
    Lower,
}

#[derive(Debug)]
pub struct KeyCode {
    pub hid_key: HidKeys,
    pub hid_modifier: HidKeys,
}

impl KeyCode {
    pub fn new(hid_key: HidKeys, hid_modifier: HidKeys) -> Self {
        Self {
            hid_key,
            hid_modifier,
        }
    }
}

impl Layer {
    pub fn get_layer(layer: &HidKeys) -> Layer {
        match layer {
            HidKeys::UpperLayer => Layer::Upper,
            HidKeys::LowerLayer => Layer::Lower,
            _ => Layer::Base,
        }
    }
}

#[derive(Default)]
pub struct Layers {
    pub base: FnvIndexMap<(u8, u8), KeyCode, LAYER_INDEXMAP_SIZE>,
    pub upper: FnvIndexMap<(u8, u8), KeyCode, LAYER_INDEXMAP_SIZE>,
    pub lower: FnvIndexMap<(u8, u8), KeyCode, LAYER_INDEXMAP_SIZE>,
}

impl Layers {
    /// initializes the Layers struct with the compiled layout
    pub fn init() -> Layers {
        let init: Layers;
        #[cfg(feature = "dvorak")]
        {
            init = dvorak::layout();
        }

        #[cfg(feature = "qwerty")]
        {
            init = qwerty::layout();
        }
        init
    }

    /// Returns the key command mapped to the row x col
    pub fn get(&mut self, row: &u8, col: &u8, layer_state: &Layer) -> Option<&KeyCode> {
        // provide the key depending on the layer
        match layer_state {
            Layer::Base => self.base.get(&(*row as u8, *col as u8)),
            Layer::Upper => self.upper.get(&(*row as u8, *col as u8)),
            Layer::Lower => self.lower.get(&(*row as u8, *col as u8)),
        }
    }
}

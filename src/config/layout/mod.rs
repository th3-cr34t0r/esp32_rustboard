pub mod dvorak;
pub mod qwerty;
use crate::{
    config::{enums::*, user_config::*},
    matrix::KeyPos,
};
use heapless::FnvIndexMap;

pub enum Layer {
    Base,
    Upper,
    Lower,
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
    pub base: FnvIndexMap<KeyPos, HidKeys, LAYER_INDEXMAP_SIZE>,
    pub upper: FnvIndexMap<KeyPos, HidKeys, LAYER_INDEXMAP_SIZE>,
    pub lower: FnvIndexMap<KeyPos, HidKeys, LAYER_INDEXMAP_SIZE>,
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
    pub fn get(&mut self, row: &u8, col: &u8, layer_state: &Layer) -> Option<&HidKeys> {
        // provide the key depending on the layer
        match layer_state {
            Layer::Base => self.base.get(&KeyPos {
                row: *row,
                col: *col,
            }),
            Layer::Upper => self.upper.get(&KeyPos {
                row: *row,
                col: *col,
            }),
            Layer::Lower => self.lower.get(&KeyPos {
                row: *row,
                col: *col,
            }),
        }
    }
}

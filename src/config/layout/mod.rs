#[cfg(feature = "qwerty")]
pub mod qwerty;

#[cfg(feature = "dvorak")]
pub mod dvorak;

#[cfg(feature = "dvorak-5x3")]
pub mod dvorak_5x3;

#[cfg(feature = "colemakdh")]
pub mod colemakdh;

use crate::{
    config::{enums::*, user_config::*},
    matrix::PinMatrix,
};

#[derive(Default)]
pub struct Layout {
    pub keymap: [[[Kc; COLS * 2]; ROWS]; LAYERS],
}

impl Layout {
    /// initializes the Layers struct with the compiled layout
    pub fn init() -> Layout {
        #[cfg(feature = "qwerty")]
        return qwerty::layout();

        #[cfg(feature = "dvorak")]
        return dvorak::layout();

        #[cfg(feature = "dvorak-5x3")]
        return dvorak_5x3::layout();

        #[cfg(feature = "colemakdh")]
        return colemakdh::layout();
    }

    /// get the layer number
    pub fn get_layer(layer: &Kc) -> usize {
        match layer {
            Kc::L1 => 1,
            Kc::L2 => 2,
            Kc::L3 => 3,
            Kc::L4 => 4,
            Kc::L5 => 5,
            _ => 0,
        }
    }
}

pub fn provide_kb_matrix() -> PinMatrix<'static> {
    let pin_matrix;

    // Dvorak Layouts Start
    #[cfg(feature = "qwerty")]
    {
        use crate::config::layout::qwerty;
        pin_matrix = qwerty::provide_pin_matrix();
    }

    // Dvorak Layouts Start
    #[cfg(feature = "dvorak")]
    {
        use crate::config::layout::dvorak;
        pin_matrix = dvorak::provide_pin_matrix();
    }

    #[cfg(feature = "dvorak-5x3")]
    {
        use crate::config::layout::dvorak_5x3;
        pin_matrix = dvorak_5x3::provide_pin_matrix();
    }

    // Colemak Layouts Start
    #[cfg(feature = "colemakdh")]
    {
        use crate::config::layout::colemakdh;
        pin_matrix = provide_pin_matrix();
    }

    pin_matrix
}

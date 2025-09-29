//*********************************************************************************************
// LAYER 0:
//
//X \ Y|  0  |  1  |  2  |  3  |  4  |           X \ Y|  5  |  6  |  7  |  8  |  9  |
//   0 |__'__|__,__|__.__|__p__|__y__|              0 |__f__|__g__|__c__|__r__|__l__|
//   1 |__a__|__o__|__e__|__u__|__i__|              1 |__d__|__h__|__t__|__n__|__s__|
//   2 |_CTL_|__q__|__j__|__k__|__x__|              2 |__b__|__m__|__w__|__v__|__z__|
//   3             |_SUP_|SPACE|SHIFT|              3 |_TAB_|ENTER|LYR_1|
//
//*********************************************************************************************
// LAYER 1:
//
//X \ Y|  0  |  1  |  2  |  3  |  4  |           X \ Y|  5  |  6  |  7  |  8  |  9  |
//   0 |_ESC_|__7__|__8__|__9__|_PScr|              0 |__-__|__(__|__)__|__=__|_____|
//   1 |_BSP_|__4__|__5__|__6__|_DEL_|              1 |__/__|_left|_down|__up_|right|
//   2 |__0__|__1__|__2__|__3__|_ALT_|              2 |__\__|__[__|__]__|__`__|__;__|
//   3             |_SUP_|SPACE|SHIFT|              3 |_TAB_|ENTER|LYR_1|
//
//*********************************************************************************************

use crate::{
    config::{enums::*, layout::*},
    matrix::PinMatrix,
};
use esp_idf_hal::{
    gpio::{IOPin, PinDriver},
    prelude::Peripherals,
};

pub fn provide_pin_matrix() -> PinMatrix<'static> {
    let peripherals = Peripherals::take().expect("Not able to init peripherals.");

    let rows = [
        PinDriver::output(peripherals.pins.gpio12.downgrade())
            .expect("Not able to set port as output."),
        PinDriver::output(peripherals.pins.gpio18.downgrade())
            .expect("Not able to set port as output."),
        PinDriver::output(peripherals.pins.gpio19.downgrade())
            .expect("Not able to set port as output."),
        PinDriver::output(peripherals.pins.gpio20.downgrade())
            .expect("Not able to set port as output."),
    ];

    let cols = [
        PinDriver::input(peripherals.pins.gpio4.downgrade())
            .expect("Not able to set port as input."),
        PinDriver::input(peripherals.pins.gpio5.downgrade())
            .expect("Not able to set port as input."),
        PinDriver::input(peripherals.pins.gpio7.downgrade())
            .expect("Not able to set port as input."),
        PinDriver::input(peripherals.pins.gpio6.downgrade())
            .expect("Not able to set port as input."),
        PinDriver::input(peripherals.pins.gpio10.downgrade())
            .expect("Not able to set port as input."),
    ];

    let pressed_keys_array = [KeyPos::new(255, 255); 6];

    PinMatrix {
        rows,
        cols,
        pressed_keys_array,
    }
}

pub fn layout() -> Layout {
    let mut layout = Layout::default();

    // LAYER 0 LAYOUT
    let layer_keymap = [
        (0, 0, HidKeys::Quote),
        (0, 1, HidKeys::Comma),
        (0, 2, HidKeys::Period),
        (0, 3, HidKeys::P),
        (0, 4, HidKeys::Y),
        (0, 5, HidKeys::F),
        (0, 6, HidKeys::G),
        (0, 7, HidKeys::C),
        (0, 8, HidKeys::R),
        (0, 9, HidKeys::L),
        (1, 0, HidKeys::A),
        (1, 1, HidKeys::O),
        (1, 2, HidKeys::E),
        (1, 3, HidKeys::U),
        (1, 4, HidKeys::I),
        (1, 5, HidKeys::D),
        (1, 6, HidKeys::H),
        (1, 7, HidKeys::T),
        (1, 8, HidKeys::N),
        (1, 9, HidKeys::S),
        (2, 0, HidKeys::ModifierControl),
        (2, 1, HidKeys::Q),
        (2, 2, HidKeys::J),
        (2, 3, HidKeys::K),
        (2, 4, HidKeys::X),
        (2, 5, HidKeys::B),
        (2, 6, HidKeys::M),
        (2, 7, HidKeys::W),
        (2, 8, HidKeys::V),
        (2, 9, HidKeys::Z),
        (3, 2, HidKeys::ModifierSuper),
        (3, 3, HidKeys::Space),
        (3, 4, HidKeys::ModifierShift),
        (3, 5, HidKeys::Tab),
        (3, 6, HidKeys::Enter),
        (3, 7, HidKeys::Layer1),
    ];

    for (row, col, key) in layer_keymap {
        if let Some(_value) = layout.keymap[0].insert(KeyPos { row, col }, key) {
            #[cfg(feature = "debug")]
            log::info!("Value already present: {:?}", _value);
        };
    }

    // LAYER 1 LAYOUT
    let layer_keymap = [
        (0, 0, HidKeys::Escape),
        (0, 1, HidKeys::Num7),
        (0, 2, HidKeys::Num8),
        (0, 3, HidKeys::Num9),
        (0, 4, HidKeys::Pscreen),
        (0, 5, HidKeys::Minus),
        (0, 6, HidKeys::MacroLeftParenthesis),
        (0, 7, HidKeys::MacroRightParenthesis),
        (0, 8, HidKeys::Equal),
        (0, 9, HidKeys::Undefined),
        (1, 0, HidKeys::BackSpace),
        (1, 1, HidKeys::Num4),
        (1, 2, HidKeys::Num5),
        (1, 3, HidKeys::Num6),
        (1, 4, HidKeys::Delete),
        (1, 5, HidKeys::ForwardSlash),
        (1, 6, HidKeys::ArrowLeft),
        (1, 7, HidKeys::ArrowDown),
        (1, 8, HidKeys::ArrowUp),
        (1, 9, HidKeys::ArrowRight),
        (2, 0, HidKeys::Num0),
        (2, 1, HidKeys::Num1),
        (2, 2, HidKeys::Num2),
        (2, 3, HidKeys::Num3),
        (2, 4, HidKeys::ModifierAlt),
        (2, 5, HidKeys::BackSlash),
        (2, 6, HidKeys::LeftBracket),
        (2, 7, HidKeys::RightBracket),
        (2, 8, HidKeys::Grave),
        (2, 9, HidKeys::SemiColon),
        (3, 2, HidKeys::ModifierSuper),
        (3, 3, HidKeys::Space),
        (3, 4, HidKeys::ModifierShift),
        (3, 5, HidKeys::Tab),
        (3, 6, HidKeys::Enter),
        (3, 7, HidKeys::Layer1),
    ];

    for (row, col, key) in layer_keymap {
        if let Some(_value) = layout.keymap[1].insert(KeyPos { row, col }, key) {
            #[cfg(feature = "debug")]
            log::info!("Value already present: {:?}", _value);
        };
    }

    // return layout
    layout
}

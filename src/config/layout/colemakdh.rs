
//*********************************************************************************************
// LAYER 0:
//
//X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |           X \ Y|  6  |  7  |  8  |  9  |  10 |  11 |
//   0 |_ESC_|__Q__|__W__|__F__|__P__|__B__|              0 |__J__|__L__|__U__|__Y__|__;__|__/__|
//   1 |_BSP_|__A__|__R__|__S__|__T__|__G__|              1 |__M__|__N__|__E__|__I__|__O__|__-__|
//   2 |_CTL_|__Z__|__X__|__C__|__D__|__V__|              2 |__K__|__H__|__,__|__.__|__'__|__=__|
//   3                   |_SUP_|SPACE|SHIFT|              3 |_TAB_|ENTER|LYR_1|
//
//*********************************************************************************************
// LAYER 1:
//
//X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |           X \ Y|  6  |  7  |  8  |  9  |  10 |  11 |
//   0 |_ESC_|_____|__7__|__8__|__9__|_PScr|              0 |_____|__(__|__)__|_____|_____|_____|
//   1 |_BSP_|_ALT_|__4__|__5__|__6__|_DEL_|              1 |_____|_left|_down|__up_|right|_____|
//   2 |_CTL_|__0__|__1__|__2__|__3__|S_LCK|              2 |__\__|__[__|__]__|__`__|_____|_____|
//   3                   |_SUP_|SPACE|SHIFT|              3 |_TAB_|ENTER|LYR_1|
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
        PinDriver::input(peripherals.pins.gpio3.downgrade())
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
        (0, 0, HidKeys::Escape),
        (0, 1, HidKeys::Q),
        (0, 2, HidKeys::W),
        (0, 3, HidKeys::F),
        (0, 4, HidKeys::P),
        (0, 5, HidKeys::B),
        (0, 6, HidKeys::J),
        (0, 7, HidKeys::L),
        (0, 8, HidKeys::U),
        (0, 9, HidKeys::Y),
        (0, 10, HidKeys::SemiColon),
        (0, 11, HidKeys::ForwardSlash),
        (1, 0, HidKeys::BackSpace),
        (1, 1, HidKeys::A),
        (1, 2, HidKeys::R),
        (1, 3, HidKeys::S),
        (1, 4, HidKeys::T),
        (1, 5, HidKeys::G),
        (1, 6, HidKeys::M),
        (1, 7, HidKeys::N),
        (1, 8, HidKeys::E),
        (1, 9, HidKeys::I),
        (1, 10, HidKeys::O),
        (1, 11, HidKeys::Minus),
        (2, 0, HidKeys::ModifierControl),
        (2, 1, HidKeys::Z),
        (2, 2, HidKeys::X),
        (2, 3, HidKeys::C),
        (2, 4, HidKeys::D),
        (2, 5, HidKeys::V),
        (2, 6, HidKeys::K),
        (2, 7, HidKeys::H),
        (2, 8, HidKeys::Comma),
        (2, 9, HidKeys::Period),
        (2, 10, HidKeys::Quote),
        (2, 11, HidKeys::Equal),
        (3, 3, HidKeys::ModifierSuper),
        (3, 4, HidKeys::Space),
        (3, 5, HidKeys::ModifierShift),
        (3, 6, HidKeys::Tab),
        (3, 7, HidKeys::Enter),
        (3, 8, HidKeys::Layer1),
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
        (0, 2, HidKeys::Num7),
        (0, 3, HidKeys::Num8),
        (0, 4, HidKeys::Num9),
        (0, 5, HidKeys::Pscreen),
        (0, 7, HidKeys::MacroLeftParenthesis),
        (0, 8, HidKeys::MacroRightParenthesis),
        (1, 0, HidKeys::BackSpace),
        (1, 1, HidKeys::ModifierAlt),
        (1, 2, HidKeys::Num4),
        (1, 3, HidKeys::Num5),
        (1, 4, HidKeys::Num6),
        (1, 5, HidKeys::Delete),
        (1, 7, HidKeys::ArrowLeft),
        (1, 8, HidKeys::ArrowDown),
        (1, 9, HidKeys::ArrowUp),
        (1, 10, HidKeys::ArrowRight),
        (2, 0, HidKeys::ModifierControl),
        (2, 1, HidKeys::Num0),
        (2, 2, HidKeys::Num1),
        (2, 3, HidKeys::Num2),
        (2, 4, HidKeys::Num3),
        (2, 5, HidKeys::MacroSuperLock),
        (2, 6, HidKeys::BackSlash),
        (2, 7, HidKeys::LeftBracket),
        (2, 8, HidKeys::RightBracket),
        (2, 9, HidKeys::Grave),
        (3, 3, HidKeys::ModifierSuper),
        (3, 4, HidKeys::Space),
        (3, 5, HidKeys::ModifierShift),
        (3, 6, HidKeys::Tab),
        (3, 7, HidKeys::Enter),
        (3, 8, HidKeys::Layer1),
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

//
//*********************************************************************************************
// LAYER 0:
//
//X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |           X \ Y|  6  |  7  |  8  |  9  |  10 |  11 |
//   0 |_ESC_|__'__|__,__|__.__|__p__|__y__|              0 |__f__|__g__|__c__|__r__|__l__|__/__|
//   1 |_BSP_|__a__|__o__|__e__|__u__|__i__|              1 |__d__|__h__|__t__|__n__|__s__|__-__|
//   2 |_CTL_|__;__|__q__|__j__|__k__|__x__|              2 |__b__|__m__|__w__|__v__|__z__|__=__|
//   3                   |_ALT_|SPACE|SHIFT|              3 |_TAB_|ENTER|LYR_1|
//
//*********************************************************************************************
// LAYER 1:
//
//X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |           X \ Y|  6  |  7  |  8  |  9  |  10 |  11 |
//   0 |_ESC_|_SUP_|__7__|__8__|__9__|_PScr|              0 |_____|__(__|__)__|_____|_____|_____|
//   1 |_BSP_|LYR_2|__4__|__5__|__6__|_DEL_|              1 |_____|_left|_down|__up_|right|_____|
//   2 |_CTL_|__0__|__1__|__2__|__3__|S_LCK|              2 |__\__|__[__|__]__|__`__|_____|_____|
//   3                   |_ALT_|SPACE|SHIFT|              3 |_TAB_|ENTER|LYR_1|
//
//*********************************************************************************************
// LAYER 2:
//
//X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |           X \ Y|  6  |  7  |  8  |  9  |  10 |  11 |
//   0 |_____|_____|_____|_____|_____|_____|              0 |_____|_____|_____|_____|_____|_____|
//   1 |_____|LYR_2|_____|M_lcl|M_rcl|_____|              1 |_____|M_lft|M_dwn|M_up_|M_rgt|_____|
//   2 |_____|_____|_____|_____|_____|_____|              2 |_____|_____|_____|_____|_____|_____|
//   3                   |_____|CSLOW|CFAST|              3 |_____|_____|_____|
// //*********************************************************************************************
//
use crate::{
    config::{enums::*, layout::*},
    matrix::PinMatrix,
};
use esp_idf_hal::{
    gpio::{IOPin, PinDriver},
    prelude::Peripherals,
};

pub static KB_NAME: &str = "Rustboard";
pub const MASTER_BLE_MAC: &str = "EC:DA:3B:BD:D7:B6";

pub fn provide_pin_layout() -> PinMatrix<'static> {
    let peripherals = Peripherals::take().expect("Not able to init peripherals.");

    let rows = [
        PinDriver::output(peripherals.pins.gpio0.downgrade())
            .expect("Not able to set port as output."),
        PinDriver::output(peripherals.pins.gpio1.downgrade())
            .expect("Not able to set port as output."),
        PinDriver::output(peripherals.pins.gpio2.downgrade())
            .expect("Not able to set port as output."),
        PinDriver::output(peripherals.pins.gpio3.downgrade())
            .expect("Not able to set port as output."),
    ];

    let cols = [
        PinDriver::input(peripherals.pins.gpio21.downgrade())
            .expect("Not able to set port as input."),
        PinDriver::input(peripherals.pins.gpio20.downgrade())
            .expect("Not able to set port as input."),
        PinDriver::input(peripherals.pins.gpio10.downgrade())
            .expect("Not able to set port as input."),
        PinDriver::input(peripherals.pins.gpio7.downgrade())
            .expect("Not able to set port as input."),
        PinDriver::input(peripherals.pins.gpio6.downgrade())
            .expect("Not able to set port as input."),
        PinDriver::input(peripherals.pins.gpio5.downgrade())
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
        (0, 1, HidKeys::Quote),
        (0, 2, HidKeys::Comma),
        (0, 3, HidKeys::Period),
        (0, 4, HidKeys::P),
        (0, 5, HidKeys::Y),
        (0, 6, HidKeys::F),
        (0, 7, HidKeys::G),
        (0, 8, HidKeys::C),
        (0, 9, HidKeys::R),
        (0, 10, HidKeys::L),
        (0, 11, HidKeys::ForwardSlash),
        (1, 0, HidKeys::BackSpace),
        (1, 1, HidKeys::A),
        (1, 2, HidKeys::O),
        (1, 3, HidKeys::E),
        (1, 4, HidKeys::U),
        (1, 5, HidKeys::I),
        (1, 6, HidKeys::D),
        (1, 7, HidKeys::H),
        (1, 8, HidKeys::T),
        (1, 9, HidKeys::N),
        (1, 10, HidKeys::S),
        (1, 11, HidKeys::Minus),
        (2, 0, HidKeys::ModifierControl),
        (2, 1, HidKeys::SemiColon),
        (2, 2, HidKeys::Q),
        (2, 3, HidKeys::J),
        (2, 4, HidKeys::K),
        (2, 5, HidKeys::X),
        (2, 6, HidKeys::B),
        (2, 7, HidKeys::M),
        (2, 8, HidKeys::W),
        (2, 9, HidKeys::V),
        (2, 10, HidKeys::Z),
        (2, 11, HidKeys::Equal),
        (3, 3, HidKeys::ModifierAlt),
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
        (0, 1, HidKeys::ModifierSuper),
        (0, 2, HidKeys::Num7),
        (0, 3, HidKeys::Num8),
        (0, 4, HidKeys::Num9),
        (0, 5, HidKeys::Pscreen),
        (0, 7, HidKeys::MacroLeftParenthesis),
        (0, 8, HidKeys::MacroRightParenthesis),
        (1, 0, HidKeys::BackSpace),
        (1, 1, HidKeys::Layer2),
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
        (3, 3, HidKeys::ModifierAlt),
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

    // LAYER 2 LAYOUT
    let layer_keymap = [
        (0, 8, HidKeys::MouseScrollUp),
        (1, 1, HidKeys::Layer2),
        (1, 3, HidKeys::MouseLeftClick),
        (1, 4, HidKeys::MouseRightClick),
        (1, 6, HidKeys::MouseScrollLeft),
        (1, 7, HidKeys::MouseGoLeft),
        (1, 8, HidKeys::MouseGoDown),
        (1, 9, HidKeys::MouseGoUp),
        (1, 10, HidKeys::MouseGoRight),
        (1, 11, HidKeys::MouseScrollRight),
        (2, 8, HidKeys::MouseScrollDown),
        (3, 4, HidKeys::MouseCursorSlow),
        (3, 5, HidKeys::MouseCursorFast),
    ];

    for (row, col, key) in layer_keymap {
        if let Some(_value) = layout.keymap[2].insert(KeyPos { row, col }, key) {
            #[cfg(feature = "debug")]
            log::info!("Value already present: {:?}", _value);
        };
    }

    // return layout
    layout
}

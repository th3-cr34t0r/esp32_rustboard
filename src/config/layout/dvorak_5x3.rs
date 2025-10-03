//*********************************************************************************************
// LAYER 0:
//
//X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |           X \ Y|  6  |  7  |  8  |  9  |  10 |  11 |
//   0 |_____|__'__|__,__|__.__|__p__|__y__|              0 |__f__|__g__|__c__|__r__|__l__|_____|
//   1 |_____|__a__|__o__|__e__|__u__|__i__|              1 |__d__|__h__|__t__|__n__|__s__|_____|
//   2 |_____|_CTL_|__q__|__j__|__k__|__x__|              2 |__b__|__m__|__w__|__v__|__z__|_____|
//   3                   |_SUP_|SPACE|SHIFT|              3 |_TAB_|ENTER|LYR_1|
//
//*********************************************************************************************
// LAYER 1:
//
//X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |           X \ Y|  6  |  7  |  8  |  9  |  10 |  11 |
//   0 |_____|_ESC_|__7__|__8__|__9__|_PScr|              0 |_SLCK|__(__|__)__|__=__|__/__|_____|
//   1 |_____|_BSP_|__4__|__5__|__6__|_DEL_|              1 |__-__|_left|_down|__up_|right|_____|
//   2 |_____|__0__|__1__|__2__|__3__|_ALT_|              2 |__\__|__[__|__]__|__`__|__;__|_____|
//   3                   |_SUP_|SPACE|SHIFT|              3 |_TAB_|ENTER|LYR_1|
//
//*********************************************************************************************

use crate::{
    config::{enums::*, layout::*},
    matrix::{KeyPos, PinMatrix},
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
#[rustfmt::skip]
pub fn layout() -> Layout {
    let mut layout = Layout::default();

    layout.keymap = [
        [
            /* LAYER 0 */
            /***********/
            /*  ROW 0  */ [Kc::Undf, Kc::Qte,   Kc::Com,  Kc::Per,   Kc::P,    Kc::Y,     Kc::F,   Kc::G,    Kc::C,  Kc::R,    Kc::L,    Kc::Undf],
            /*  ROW 1  */ [Kc::Undf, Kc::A,     Kc::O,    Kc::E,     Kc::U,    Kc::I,     Kc::D,   Kc::H,    Kc::T,  Kc::N,    Kc::S,    Kc::Undf],
            /*  ROW 2  */ [Kc::Undf, Kc::ModCo, Kc::Q,    Kc::J,     Kc::K,    Kc::X,     Kc::B,   Kc::M,    Kc::W,  Kc::V,    Kc::Z,    Kc::Undf],
            /*  ROW 3  */ [Kc::Undf, Kc::Undf,  Kc::Undf, Kc::ModSu, Kc::Spac, Kc::ModSh, Kc::Tab, Kc::Entr, Kc::L1, Kc::Undf, Kc::Undf, Kc::Undf],
        ],
        [
            /* LAYER 1 */
            /***********/
            /*  ROW 0  */ [Kc::Undf, Kc::Esc,   Kc::N7,   Kc::N8,    Kc::N9,   Kc::Pscr,  Kc::MaSL, Kc::MaLP, Kc::MaRP, Kc::Eq,   Kc::Fsl,  Kc::Undf],
            /*  ROW 1  */ [Kc::Undf, Kc::Bksp,  Kc::N4,   Kc::N5,    Kc::N6,   Kc::Del,   Kc::Mns,  Kc::ArL,  Kc::ArD,  Kc::ArU,  Kc::ArR,  Kc::Undf],
            /*  ROW 2  */ [Kc::Undf, Kc::N0,    Kc::N1,   Kc::N2,    Kc::N3,   Kc::ModAl,  Kc::Bksl, Kc::Lbrk, Kc::Rbrk, Kc::Grav, Kc::Scn,  Kc::Undf],
            /*  ROW 3  */ [Kc::Undf, Kc::Undf,  Kc::Undf, Kc::ModSu, Kc::Spac, Kc::ModSh, Kc::Tab,  Kc::Entr, Kc::L1,   Kc::Undf, Kc::Undf, Kc::Undf],
        ],
    ];

    layout
}

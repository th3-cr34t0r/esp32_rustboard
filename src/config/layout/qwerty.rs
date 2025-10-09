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

    let registered_local_keys_array = [KeyPos::default(); 6];

    PinMatrix {
        rows,
        cols,
        registered_local_keys_array,
    }
}

//*********************************************************************************************
// LAYER 0:
//
//X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |           X \ Y|  6  |  7  |  8  |  9  |  10 |  11 |
//   0 |_ESC_|__q__|__w__|__e__|__r__|__t__|              0 |__y__|__u__|__i__|__o__|__p__|__[__|
//   1 |_BSP_|__a__|__s__|__d__|__f__|__g__|              1 |__h__|__j__|__k__|__l__|__;__|__'__|
//   2 |_CTL_|__z__|__x__|__c__|__v__|__b__|              2 |__n__|__m__|__,__|__.__|__/__|__]__|
//   3                   |_LYR_|_SPC_|_SFT_|              3 |_ALT_|_ENT_|_LYR_|
//
//*****************************************************************************
// LAYER 1:
//
//X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |           X \ Y|  6  |  7  |  8  |  9  |  10 |  11 |
//   0 |_ESC_|_SUP_|__7__|__8__|__9__|_PScr|              0 |__!__|__@__|__#__|__$__|__%__|__^__|
//   1 |_BSP_|__.__|__4__|__5__|__6__|_DEL_|              1 |__&__|_left|_down|__up_|_rght|__*__|
//   2 |_CTL_|__0__|__1__|__2__|__3__|S_LCK|              2 |__\__|__[__|__]__|__(__|__)__|__`__|
//   3                   |_ALT_|_SPC_|_SFT_|              3 |_TAB_|_ENT_|_LYR_|
//
//*********************************************************************************************
#[rustfmt::skip]
pub fn layout() -> Layout {
    let mut layout = Layout::default();

    layout.keymap = [
        [
            /* LAYER 0 */  /*    COL 0          COL 1        COL 2          COL 3         COL 4         COL 5                   COL 6         COL 7         COL 8        COL 9         COL 10        COL 11   */
            /*                 +--------------+-------------+-------------+--------------+------------+--------------+        +------------+--------------+------------+-------------+-------------+------------+*/
            /*   ROW 0  */  [/*|*/Kc::Esc,  /*|*/Kc::Q,   /*|*/Kc::W,   /*|*/Kc::E,    /*|*/Kc::R,  /*|*/Kc::T,    /*|        |*/Kc::Y,  /*|*/Kc::U,    /*|*/Kc::I,  /*|*/Kc::O,   /*|*/Kc::P,   /*|*/Kc::Lbrk,/*|*/],
            /*                 +--------------+-------------+-------------+--------------+------------+--------------+        +------------+--------------+------------+-------------+-------------+------------+*/
            /*   ROW 1  */  [/*|*/Kc::Bksp, /*|*/Kc::A,   /*|*/Kc::S,   /*|*/Kc::D,    /*|*/Kc::F,  /*|*/Kc::G,    /*|        |*/Kc::H,  /*|*/Kc::J,    /*|*/Kc::K,  /*|*/Kc::L,   /*|*/Kc::Scn, /*|*/Kc::Qte /*|*/],
            /*                 +--------------+-------------+-------------+--------------+------------+--------------+        +------------+--------------+------------+-------------+-------------+------------+*/
            /*   ROW 2  */  [/*|*/Kc::ModCo,/*|*/Kc::Z,   /*|*/Kc::X,   /*|*/Kc::C,    /*|*/Kc::V,  /*|*/Kc::B,    /*|        |*/Kc::N,  /*|*/Kc::M,    /*|*/Kc::Com,/*|*/Kc::Per, /*|*/Kc::Fsl, /*|*/Kc::Rbrk/*|*/],
            /*                 +--------------+-------------+-------------+--------------+------------+--------------+        +------------+--------------+------------+-------------+-------------+------------+*/
            /*   ROW 3  */  [/*|*/Kc::Undf, /*|*/Kc::Undf,/*|*/Kc::Undf,/*|*/Kc::ModSu,/*|*/Kc::Spac,/*|*/Kc::ModSh,/*|        |*/Kc::Tab,/*|*/Kc::Entr, /*|*/Kc::L1, /*|*/Kc::Undf,/*|*/Kc::Undf,/*|*/Kc::Undf/*|*/],
            /*                 +--------------+-------------+-------------+--------------+------------+--------------+        +------------+--------------+------------+-------------+-------------+------------+*/
        ],
        [
            /*  LAYER 1 */  /*     COL 0          COL 1        COL 2          COL 3         COL 4         COL 5                   COL 6         COL 7         COL 8        COL 9         COL 10        COL 11   */
            /*                 +--------------+----------------------------+--------------+------------+---------------+        +-------------+--------------+-------------+-------------+--------------+------------+*/
            /*   ROW 0  */  [/*|*/Kc::Esc,  /*|*/Kc::Undf, /*|*/Kc::N7,  /*|*/Kc::N8,   /*|*/Kc::N9, /*|*/Kc::Pscr,  /*|        |*/Kc::Undf,/*|*/Kc::MaLP, /*|*/Kc::MaRP,/*|*/Kc::Undf,/*|*/Kc::Undf, /*|*/Kc::Undf/*|*/],
            /*                 +--------------+----------------------------+--------------+------------+---------------+        +-------------+--------------+-------------+-------------+--------------+------------+*/
            /*   ROW 1  */  [/*|*/Kc::Bksp, /*|*/Kc::ModAl,/*|*/Kc::N4,  /*|*/Kc::N5,   /*|*/Kc::N6, /*|*/Kc::Del,   /*|        |*/Kc::Undf,/*|*/Kc::ArL,  /*|*/Kc::ArD, /*|*/Kc::ArU, /*|*/Kc::ArR,  /*|*/Kc::Undf/*|*/],
            /*                 +--------------+--------------+-------------+--------------+------------+---------------+        +-------------+--------------+-------------+-------------+--------------+------------+*/
            /*   ROW 2  */  [/*|*/Kc::ModCo,/*|*/Kc::N0,   /*|*/Kc::N1,  /*|*/Kc::N2,   /*|*/Kc::N3, /*|*/Kc::MaSL, /*|        |*/Kc::Bksl,/*|*/Kc::Lbrk, /*|*/Kc::Rbrk,/*|*/Kc::Grav, /*|*/Kc::Undf, /*|*/Kc::Undf/*|*/],
            /*                 +--------------+--------------+-------------+--------------+------------+---------------+        +-------------+--------------+-------------+-------------+--------------+------------+*/
            /*   ROW 3  */  [/*|*/Kc::Undf, /*|*/Kc::Undf, /*|*/Kc::Undf,/*|*/Kc::ModSu,/*|*/Kc::Spac,/*|*/Kc::ModSh,/*|        |*/Kc::Tab, /*|*/Kc::Entr ,/*|*/Kc::L1,  /*|*/Kc::Undf,/*|*/Kc::Undf, /*|*/Kc::Undf/*|*/],
            /*                 +--------------+--------------+-------------+--------------+------------+---------------+        +-------------+--------------+-------------+-------------+--------------+------------+*/
        ],
    ];

    // return layout
    layout
}

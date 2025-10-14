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
//   1 |_BSP_|_____|__4__|__5__|__6__|_DEL_|              1 |_____|_left|_down|__up_|right|_____|
//   2 |_CTL_|__0__|__1__|__2__|__3__|S_LCK|              2 |__\__|__[__|__]__|__`__|_____|_____|
//   3                   |_ALT_|SPACE|SHIFT|              3 |_TAB_|ENTER|LYR_1|
//
//*********************************************************************************************
#[rustfmt::skip]
pub fn layout() -> Layout {
    Layout { keymap:
        [
            [
                /* LAYER 0 */  /*    COL 0          COL 1           COL 2          COL 3         COL 4         COL 5                   COL 6         COL 7         COL 8        COL 9         COL 10        COL 11  */
                /*               +-------------+--------------+--------------+--------------+-------------+--------------+        +------------+--------------+------------+-------------+-------------+------------+*/
                /*  ROW 0  */ [/*|*/Kc::Esc, /*|*/Kc::Qte,  /*|*/ Kc::Com, /*|*/Kc::Per,  /*|*/Kc::P,   /*|*/Kc::Y,    /*|        |*/Kc::F,  /*|*/ Kc::G,   /*|*/ Kc::C, /*|*/Kc::R,   /*|*/Kc::L,   /*|*/Kc::Fsl/*|*/],
                /*               +-------------+--------------+--------------+--------------+-------------+--------------+        +------------+--------------+------------+-------------+-------------+------------+*/
                /*  ROW 1  */ [/*|*/Kc::Bksp,/*|*/Kc::A,    /*|*/ Kc::O,   /*|*/Kc::E,    /*|*/Kc::U,   /*|*/Kc::I,    /*|        |*/Kc::D,  /*|*/ Kc::H,   /*|*/ Kc::T, /*|*/Kc::N,   /*|*/Kc::S,   /*|*/Kc::Mns/*|*/],
                /*               +-------------+--------------+--------------+--------------+-------------+--------------+        +------------+--------------+------------+-------------+-------------+------------+*/
                /*  ROW 2  */ [/*|*/Kc::ModCo,/*|*/Kc::Scn,/*|*/ Kc::Q,    /*|*/Kc::J,    /*|*/Kc::K,   /*|*/Kc::X,    /*|        |*/Kc::B,  /*|*/ Kc::M,   /*|*/ Kc::W, /*|*/Kc::V,   /*|*/Kc::Z,   /*|*/Kc::Eq/*|*/],
                /*               +-------------+--------------+--------------+--------------+-------------+--------------+        +------------+--------------+------------+-------------+-------------+------------+*/
                /*  ROW 3  */ [/*|*/Kc::Undf,/*|*/Kc::Undf, /*|*/ Kc::Undf,/*|*/Kc::ModSu,/*|*/Kc::Spac,/*|*/Kc::ModSh,/*|        |*/Kc::Tab,/*|*/ Kc::Entr,/*|*/ Kc::L1,/*|*/Kc::Undf,/*|*/Kc::Undf,/*|*/Kc::Undf/*|*/],
                /*               +-------------+--------------+--------------+--------------+-------------+--------------+        +------------+--------------+------------+-------------+-------------+------------+*/

            ],
            [
                /* LAYER 1 */   /* COL 0          COL 1          COL 2          COL 3          COL 4         COL 5                     COL 6         COL 7         COL 8         COL 9         COL 10        COL 11      */
                /*               +--------------+---------------+--------------+--------------+-------------+--------------+        +-------------+--------------+-------------+-------------+-------------+------------+*/
                /*  ROW 0  */ [/*|*/Kc::Esc,  /*|*/Kc::ModSu, /*|*/Kc::N7,   /*|*/Kc::N8,   /*|*/Kc::N9,  /*|*/Kc::Pscr, /*|        |*/Kc::Undf,/*|*/Kc::MaLP, /*|*/Kc::MaRP,/*|*/Kc::Undf,/*|*/Kc::Undf,/*|*/Kc::Undf/*|*/],
                /*               +-------------+---------------+--------------+--------------+-------------+---------------+        +-------------+--------------+-------------+-------------+-------------+------------+*/
                /*  ROW 1  */ [/*|*/Kc::Bksp, /*|*/Kc::Undf,  /*|*/Kc::N4,   /*|*/Kc::N5,   /*|*/Kc::N6,  /*|*/Kc::Del,  /*|        |*/Kc::Undf, /*|*/Kc::ArL,  /*|*/Kc::ArD, /*|*/Kc::ArU, /*|*/Kc::ArR,/*|*/Kc::Undf/*|*/],
                /*               +--------------+---------------+--------------+--------------+-------------+--------------+        +-------------+--------------+-------------+-------------+-------------+------------+*/
                /*  ROW 2  */ [/*|*/Kc::ModCo,/*|*/Kc::N0,    /*|*/Kc::N1,   /*|*/Kc::N2,   /*|*/Kc::N3,  /*|*/Kc::MaSL, /*|        |*/Kc::Bksl,/*|*/Kc::Lbrk, /*|*/Kc::Rbrk,/*|*/Kc::Grav,/*|*/Kc::Undf,/*|*/Kc::Undf/*|*/],
                /*               +--------------+---------------+--------------+--------------+-------------+--------------+        +-------------+--------------+-------------+-------------+-------------+------------+*/
                /*  ROW 3  */ [/*|*/Kc::Undf, /*|*/Kc::Undf,  /*|*/Kc::Undf, /*|*/Kc::ModSu,/*|*/Kc::Spac,/*|*/Kc::ModSh,/*|        |*/Kc::Tab, /*|*/Kc::Entr, /*|*/Kc::L1,  /*|*/Kc::Undf,/*|*/Kc::Undf,/*|*/Kc::Undf/*|*/],
                /*               +--------------+---------------+--------------+--------------+-------------+--------------+        +-------------+--------------+-------------+-------------+-------------+------------+*/
            ],
        ],

        combos: [Kc::Undf]
    } 
}

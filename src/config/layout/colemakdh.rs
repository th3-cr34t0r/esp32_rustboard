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

#[rustfmt::skip]
pub fn layout() -> Layout {
    let mut layout = Layout::default();


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

layout.keymap = [
    [
        /** LAYER 0 **/  /** COL 0          COL 1           COL 2          COL 3         COL 4         COL 5                   COL 6         COL 7         COL 8        COL 9         COL 10        COL 11  **/
        /*                 +-------------+-------------+-------------+--------------+------------+--------------+        +------------+--------------+--------------+-------------+-------------+------------+*/
        /**  ROW 0  **/ [/*|*/Kc::Esc, /*|*/Kc::Q,   /*|*/Kc::W,   /*|*/Kc::F,    /*|*/Kc::P,  /*|*/Kc::B,    /*|        |*/Kc::J,  /*|*/Kc::L,    /*|*/Kc::U,    /*|*/Kc::Y,   /*|*/Kc::Smc, /*|*/Kc::Sls /*|*/],
        /*                 +-------------+-------------+-------------+--------------+------------+--------------+        +------------+--------------+--------------+-------------+-------------+------------+*/
        /**  ROW 1  **/ [/*|*/Kc::Bksp,/*|*/Kc::A,   /*|*/Kc::R,   /*|*/Kc::S,    /*|*/Kc::T,  /*|*/Kc::G,    /*|        |*/Kc::M,  /*|*/Kc::N,    /*|*/Kc::E,    /*|*/Kc::I,   /*|*/Kc::O,   /*|*/Kc::Mns /*|*/],
        /*                 +-------------+-------------+-------------+--------------+------------+--------------+        +------------+--------------+--------------+-------------+-------------+------------+*/
        /**  ROW 2  **/ [/*|*/Kc::Ctl, /*|*/Kc::Z,   /*|*/Kc::X,   /*|*/Kc::C,    /*|*/Kc::D,  /*|*/Kc::V,    /*|        |*/Kc::K,  /*|*/Kc::H,    /*|*/Kc::Comma,/*|*/Kc::Dot, /*|*/Kc::Apo, /*|*/Kc::Eq  /*|*/],
        /*                 +-------------+-------------+-------------+--------------+------------+--------------+        +------------+--------------+--------------+-------------+-------------+------------+*/
        /**  ROW 3  **/ [/*|*/Kc::Undf,/*|*/Kc::Undf,/*|*/Kc::Undf,/*|*/Kc::ModSu,/*|*/Kc::Spc,/*|*/Kc::ModSh,/*|        |*/Kc::Tab,/*|*/Kc::Enter,/*|*/Kc::L1,   /*|*/Kc::Undf,/*|*/Kc::Undf,/*|*/Kc::Undf/*|*/],
        /*                 +-------------+-------------+-------------+--------------+------------+--------------+        +------------+--------------+--------------+-------------+-------------+------------+*/
    ],
    [
        /** LAYER 1 **/  /** COL 0          COL 1          COL 2          COL 3          COL 4         COL 5                     COL 6         COL 7         COL 8         COL 9         COL 10        COL 11  **/
        /*                 +--------------+----------------------------+--------------+------------+---------------+        +-------------+--------------+-------------+-------------+--------------+------------+*/
        /**  ROW 0  **/ [/*|*/Kc::Esc,  /*|*/Kc::Undf, /*|*/Kc::N7,  /*|*/Kc::N8,   /*|*/Kc::N9, /*|*/Kc::Pscr,  /*|        |*/Kc::Undf,/*|*/Kc::LPar, /*|*/Kc::RPar,/*|*/Kc::Undf,/*|*/Kc::Undf, /*|*/Kc::Undf/*|*/],
        /*                 +--------------+----------------------------+--------------+------------+---------------+        +-------------+--------------+-------------+-------------+--------------+------------+*/
        /**  ROW 1  **/ [/*|*/Kc::Bksp, /*|*/Kc::Alt,  /*|*/Kc::N4,  /*|*/Kc::N5,   /*|*/Kc::N6, /*|*/Kc::Del,   /*|        |*/Kc::Undf,/*|*/Kc::ArL,  /*|*/Kc::ArD, /*|*/Kc::ArU, /*|*/Kc::ArR,  /*|*/Kc::Undf/*|*/],
        /*                 +--------------+--------------+-------------+--------------+------------+---------------+        +-------------+--------------+-------------+-------------+--------------+------------+*/
        /**  ROW 2  **/ [/*|*/Kc::ModCo,/*|*/Kc::N0,   /*|*/Kc::N1,  /*|*/Kc::N2,   /*|*/Kc::N3, /*|*/Kc::ScLock,/*|        |*/Kc::Bksl,/*|*/Kc::LBrk, /*|*/Kc::RBrk,/*|*/Kc::Grv, /*|*/Kc::Undf, /*|*/Kc::Undf/*|*/],
        /*                 +--------------+--------------+-------------+--------------+------------+---------------+        +-------------+--------------+-------------+-------------+--------------+------------+*/
        /**  ROW 3  **/ [/*|*/Kc::Undf, /*|*/Kc::Undf, /*|*/Kc::Undf,/*|*/Kc::ModSu,/*|*/Kc::Spc,/*|*/Kc::ModSh, /*|        |*/Kc::Tab, /*|*/Kc::Enter,/*|*/Kc::L1,  /*|*/Kc::Undf,/*|*/Kc::Undf, /*|*/Kc::Undf/*|*/],
        /*                 +--------------+--------------+-------------+--------------+------------+---------------+        +-------------+--------------+-------------+-------------+--------------+------------+*/
    ],
];

    // return layout
    layout
}

//
//*********************************************************************************************
//BASE LAYER:
//
//X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |           X \ Y|  6  |  7  |  8  |  9  |  10 |  11 |
//   0 |_ESC_|__'__|__,__|__.__|__p__|__y__|              0 |__f__|__g__|__c__|__r__|__l__|__/__|
//   1 |_BSP_|__a__|__o__|__e__|__u__|__i__|              1 |__d__|__h__|__t__|__n__|__s__|__-__|
//   2 |_CTL_|__;__|__q__|__j__|__k__|__x__|              2 |__b__|__m__|__w__|__v__|__z__|__=__|
//   3                   |_ALT_|_SPC_|_SFT_|              3 |_TAB_|_ENT_|_UPR_|
//
//*********************************************************************************************
//UPPER LAYER:
//
//X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |           X \ Y|  6  |  7  |  8  |  9  |  10 |  11 |
//   0 |_ESC_|_SUP_|__7__|__8__|__9__|_PScr|              0 |__!__|__@__|__#__|__$__|__%__|__^__|
//   1 |_BSP_|__.__|__4__|__5__|__6__|_DEL_|              1 |__&__|_left|_down|__up_|_rght|__*__|
//   2 |_CTL_|__0__|__1__|__2__|__3__|_PST_|              2 |__\__|__[__|__]__|__(__|__)__|__`__|
//   3                   |_ALT_|_SPC_|_SFT_|              3 |_TAB_|_ENT_|_UPR_|
//
//*********************************************************************************************
//LOWER LAYER:
//
//X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |           X \ Y|  6  |  7  |  8  |  9  |  10 |  11 |
//   0 |_ESC_|_WLCK|_____|_____|_____|_____|              0 |_____|_____|_____|_____|_____|_____|
//   1 |_BSP_|_ALT_|_____|_____|_____|_____|              1 |_____|_left|_down|__up_|_rght|_____|
//   2 |_CTL_|_____|_____|_____|_____|_____|              2 |_____|_____|_____|_____|_____|_____|
//   3                   |_ALT_|_SPC_|_SFT_|              3 |_TAB_|_ENT_|_UPR_|
//
//*********************************************************************************************
//
use crate::config::{enums::*, layout::*};

pub fn layout() -> Layers {
    let mut layout = Layers::default();

    {
        // BASE LAYER LAYOUT
        let base_layer_keys = [
            (0, 0, HidKeys::Escape, HidKeys::Undefined),
            (0, 1, HidKeys::Quote, HidKeys::Undefined),
            (0, 2, HidKeys::Comma, HidKeys::Undefined),
            (0, 3, HidKeys::Period, HidKeys::Undefined),
            (0, 4, HidKeys::P, HidKeys::Undefined),
            (0, 5, HidKeys::Y, HidKeys::Undefined),
            (0, 6, HidKeys::F, HidKeys::Undefined),
            (0, 7, HidKeys::G, HidKeys::Undefined),
            (0, 8, HidKeys::C, HidKeys::Undefined),
            (0, 9, HidKeys::R, HidKeys::Undefined),
            (0, 10, HidKeys::L, HidKeys::Undefined),
            (0, 11, HidKeys::Slash, HidKeys::Undefined),
            (1, 0, HidKeys::BackSpace, HidKeys::Undefined),
            (1, 1, HidKeys::A, HidKeys::ModifierSuper), // HRM
            (1, 2, HidKeys::O, HidKeys::ModifierAlt),   // HRM
            (1, 3, HidKeys::E, HidKeys::ModifierControl), // HRM
            (1, 4, HidKeys::U, HidKeys::ModifierShift), // HRM
            (1, 5, HidKeys::I, HidKeys::Undefined),
            (1, 6, HidKeys::D, HidKeys::Undefined),
            (1, 7, HidKeys::H, HidKeys::ModifierShift), // HRM
            (1, 8, HidKeys::T, HidKeys::ModifierControl), // HRM
            (1, 9, HidKeys::N, HidKeys::ModifierAlt),   // HRM
            (1, 10, HidKeys::S, HidKeys::ModifierSuper), // HRM
            (1, 11, HidKeys::Minus, HidKeys::Undefined),
            (2, 0, HidKeys::ModifierControl, HidKeys::Undefined),
            (2, 1, HidKeys::SemiColon, HidKeys::Undefined),
            (2, 2, HidKeys::Q, HidKeys::Undefined),
            (2, 3, HidKeys::J, HidKeys::Undefined),
            (2, 4, HidKeys::K, HidKeys::Undefined),
            (2, 5, HidKeys::X, HidKeys::Undefined),
            (2, 6, HidKeys::B, HidKeys::Undefined),
            (2, 7, HidKeys::M, HidKeys::Undefined),
            (2, 8, HidKeys::W, HidKeys::Undefined),
            (2, 9, HidKeys::V, HidKeys::Undefined),
            (2, 10, HidKeys::Z, HidKeys::Undefined),
            (2, 11, HidKeys::Equal, HidKeys::Undefined),
            (3, 0, HidKeys::Undefined, HidKeys::Undefined),
            (3, 1, HidKeys::Undefined, HidKeys::Undefined),
            (3, 2, HidKeys::Undefined, HidKeys::Undefined),
            (3, 3, HidKeys::ModifierAlt, HidKeys::Undefined),
            (3, 4, HidKeys::Space, HidKeys::Undefined),
            (3, 5, HidKeys::ModifierShift, HidKeys::Undefined),
            (3, 6, HidKeys::Tab, HidKeys::Undefined),
            (3, 7, HidKeys::Enter, HidKeys::Undefined),
            (3, 8, HidKeys::UpperLayer, HidKeys::Undefined),
            (3, 9, HidKeys::Undefined, HidKeys::Undefined),
            (3, 10, HidKeys::Undefined, HidKeys::Undefined),
            (3, 11, HidKeys::Undefined, HidKeys::Undefined),
        ];

        for entry in base_layer_keys.iter() {
            layout
                .base
                .insert(
                    Key {
                        row: entry.0,
                        col: entry.1,
                    },
                    KeyCode {
                        hid_key: entry.2,
                        hid_modifier: entry.3,
                    },
                )
                .unwrap();
        }
    }

    {
        // UPPER LAYER LAYOUT
        let upper_layer_keys = [
            (0, 0, HidKeys::Escape, HidKeys::Undefined),
            (0, 1, HidKeys::ModifierSuper, HidKeys::Undefined),
            (0, 2, HidKeys::Num7, HidKeys::Undefined),
            (0, 3, HidKeys::Num8, HidKeys::Undefined),
            (0, 4, HidKeys::Num9, HidKeys::Undefined),
            (0, 5, HidKeys::PrintScreen, HidKeys::Undefined),
            (0, 6, HidKeys::MacroExclamationMark, HidKeys::Undefined),
            (0, 7, HidKeys::MacroAt, HidKeys::Undefined),
            (0, 8, HidKeys::MacroHash, HidKeys::Undefined),
            (0, 9, HidKeys::MacroDollar, HidKeys::Undefined),
            (0, 10, HidKeys::MacroModul, HidKeys::Undefined),
            (0, 11, HidKeys::MacroCaret, HidKeys::Undefined),
            (1, 0, HidKeys::BackSpace, HidKeys::Undefined),
            (1, 1, HidKeys::KpDot, HidKeys::Undefined),
            (1, 2, HidKeys::Num4, HidKeys::Undefined),
            (1, 3, HidKeys::Num5, HidKeys::Undefined),
            (1, 4, HidKeys::Num6, HidKeys::Undefined),
            (1, 5, HidKeys::Delete, HidKeys::Undefined),
            (1, 6, HidKeys::MacroAmpersand, HidKeys::Undefined),
            (1, 7, HidKeys::ArrowLeft, HidKeys::Undefined),
            (1, 8, HidKeys::ArrowDown, HidKeys::Undefined),
            (1, 9, HidKeys::ArrowUp, HidKeys::Undefined),
            (1, 10, HidKeys::ArrowRight, HidKeys::Undefined),
            (1, 11, HidKeys::MacroAsterix, HidKeys::Undefined),
            (2, 0, HidKeys::ModifierControl, HidKeys::Undefined),
            (2, 1, HidKeys::Num0, HidKeys::Undefined),
            (2, 2, HidKeys::Num1, HidKeys::Undefined),
            (2, 3, HidKeys::Num2, HidKeys::Undefined),
            (2, 4, HidKeys::Num3, HidKeys::Undefined),
            (2, 5, HidKeys::Paste, HidKeys::Undefined),
            (2, 6, HidKeys::Backslash, HidKeys::Undefined),
            (2, 7, HidKeys::LeftBracket, HidKeys::Undefined),
            (2, 8, HidKeys::RightBracket, HidKeys::Undefined),
            (2, 9, HidKeys::MacroLeftParenthesis, HidKeys::Undefined),
            (2, 10, HidKeys::MacroRightParenthesis, HidKeys::Undefined),
            (2, 11, HidKeys::Grave, HidKeys::Undefined),
            (3, 0, HidKeys::Undefined, HidKeys::Undefined),
            (3, 1, HidKeys::Undefined, HidKeys::Undefined),
            (3, 2, HidKeys::Undefined, HidKeys::Undefined),
            (3, 3, HidKeys::ModifierAlt, HidKeys::Undefined),
            (3, 4, HidKeys::Space, HidKeys::Undefined),
            (3, 5, HidKeys::ModifierShift, HidKeys::Undefined),
            (3, 6, HidKeys::Tab, HidKeys::Undefined),
            (3, 7, HidKeys::Enter, HidKeys::Undefined),
            (3, 8, HidKeys::UpperLayer, HidKeys::Undefined),
            (3, 9, HidKeys::Undefined, HidKeys::Undefined),
            (3, 10, HidKeys::Undefined, HidKeys::Undefined),
            (3, 11, HidKeys::Undefined, HidKeys::Undefined),
        ];

        for entry in upper_layer_keys.iter() {
            layout
                .upper
                .insert(
                    Key {
                        row: entry.0,
                        col: entry.1,
                    },
                    KeyCode {
                        hid_key: entry.2,
                        hid_modifier: entry.3,
                    },
                )
                .unwrap();
        }
    }
    // return layout
    layout
}

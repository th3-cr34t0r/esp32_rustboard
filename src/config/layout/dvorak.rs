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

pub fn layout() -> Layout {
    let mut layout = Layout::default();

    {
        // BASE LAYER LAYOUT
        let base_layer_keys = [
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
            (3, 0, HidKeys::Undefined),
            (3, 1, HidKeys::Undefined),
            (3, 2, HidKeys::Undefined),
            (3, 3, HidKeys::ModifierAlt),
            (3, 4, HidKeys::Space),
            (3, 5, HidKeys::ModifierShift),
            (3, 6, HidKeys::Tab),
            (3, 7, HidKeys::Enter),
            (3, 8, HidKeys::UpperLayer),
            (3, 9, HidKeys::Undefined),
            (3, 10, HidKeys::Undefined),
            (3, 11, HidKeys::Undefined),
        ];

        for (row, col, key) in base_layer_keys.iter() {
            layout
                .base
                .insert(
                    KeyPos {
                        row: *row,
                        col: *col,
                    },
                    *key,
                )
                .unwrap();
        }
    }

    {
        // UPPER LAYER LAYOUT
        let upper_layer_keys = [
            (0, 0, HidKeys::Escape),
            (0, 1, HidKeys::ModifierSuper),
            (0, 2, HidKeys::Num7),
            (0, 3, HidKeys::Num8),
            (0, 4, HidKeys::Num9),
            (0, 5, HidKeys::Pscreen),
            (0, 6, HidKeys::MacroExclamationMark),
            (0, 7, HidKeys::MacroAt),
            (0, 8, HidKeys::MacroHash),
            (0, 9, HidKeys::MacroDollar),
            (0, 10, HidKeys::MacroModul),
            (0, 11, HidKeys::MacroCaret),
            (1, 0, HidKeys::BackSpace),
            (1, 1, HidKeys::KpDot),
            (1, 2, HidKeys::Num4),
            (1, 3, HidKeys::Num5),
            (1, 4, HidKeys::Num6),
            (1, 5, HidKeys::Delete),
            (1, 6, HidKeys::MacroAmpersand),
            (1, 7, HidKeys::ArrowLeft),
            (1, 8, HidKeys::ArrowDown),
            (1, 9, HidKeys::ArrowUp),
            (1, 10, HidKeys::ArrowRight),
            (1, 11, HidKeys::MacroAsterix),
            (2, 0, HidKeys::ModifierControl),
            (2, 1, HidKeys::Num0),
            (2, 2, HidKeys::Num1),
            (2, 3, HidKeys::Num2),
            (2, 4, HidKeys::Num3),
            (2, 5, HidKeys::Paste),
            (2, 6, HidKeys::BackSlash),
            (2, 7, HidKeys::LeftBracket),
            (2, 8, HidKeys::RightBracket),
            (2, 9, HidKeys::MacroLeftParenthesis),
            (2, 10, HidKeys::MacroRightParenthesis),
            (2, 11, HidKeys::Grave),
            (3, 0, HidKeys::Undefined),
            (3, 1, HidKeys::Undefined),
            (3, 2, HidKeys::Undefined),
            (3, 3, HidKeys::ModifierAlt),
            (3, 4, HidKeys::Space),
            (3, 5, HidKeys::ModifierShift),
            (3, 6, HidKeys::Tab),
            (3, 7, HidKeys::Enter),
            (3, 8, HidKeys::UpperLayer),
            (3, 9, HidKeys::Undefined),
            (3, 10, HidKeys::Undefined),
            (3, 11, HidKeys::Undefined),
        ];

        for (row, col, key) in upper_layer_keys.iter() {
            layout
                .upper
                .insert(
                    KeyPos {
                        row: *row,
                        col: *col,
                    },
                    *key,
                )
                .unwrap();
        }
    }

    // return layout
    layout
}

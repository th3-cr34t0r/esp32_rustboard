//
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
//
use crate::config::{enums::*, layout::*};

pub fn layout() -> Layout {
    let mut layout = Layout::default();
    let mut layer_keymap = [[(0, 0, HidKeys::Undefined); ROWS * COLS * 2]; LAYERS];

    {
        // BASE LAYER LAYOUT
        layer_keymap[0] = [
            (0, 0, HidKeys::Escape),
            (0, 1, HidKeys::Q),
            (0, 2, HidKeys::W),
            (0, 3, HidKeys::E),
            (0, 4, HidKeys::R),
            (0, 5, HidKeys::T),
            (0, 6, HidKeys::Y),
            (0, 7, HidKeys::U),
            (0, 8, HidKeys::I),
            (0, 9, HidKeys::O),
            (0, 10, HidKeys::P),
            (0, 11, HidKeys::LeftBracket),
            (1, 0, HidKeys::BackSpace),
            (1, 1, HidKeys::A),
            (1, 2, HidKeys::S),
            (1, 3, HidKeys::D),
            (1, 4, HidKeys::F),
            (1, 5, HidKeys::G),
            (1, 6, HidKeys::H),
            (1, 7, HidKeys::J),
            (1, 8, HidKeys::K),
            (1, 9, HidKeys::L),
            (1, 10, HidKeys::SemiColon),
            (1, 11, HidKeys::Quote),
            (2, 0, HidKeys::ModifierControl),
            (2, 1, HidKeys::Z),
            (2, 2, HidKeys::X),
            (2, 3, HidKeys::C),
            (2, 4, HidKeys::V),
            (2, 5, HidKeys::B),
            (2, 6, HidKeys::N),
            (2, 7, HidKeys::M),
            (2, 8, HidKeys::Comma),
            (2, 9, HidKeys::Period),
            (2, 10, HidKeys::ForwardSlash),
            (2, 11, HidKeys::Minus),
            (3, 0, HidKeys::Undefined),
            (3, 1, HidKeys::Undefined),
            (3, 2, HidKeys::Undefined),
            (3, 3, HidKeys::ModifierAlt),
            (3, 4, HidKeys::Space),
            (3, 5, HidKeys::ModifierShift),
            (3, 6, HidKeys::Tab),
            (3, 7, HidKeys::Enter),
            (3, 8, HidKeys::Layer1),
            (3, 9, HidKeys::Undefined),
            (3, 10, HidKeys::Undefined),
            (3, 11, HidKeys::Undefined),
        ];

        for (row, col, key) in layer_keymap[0].iter() {
            layout.keymap[0]
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
        layer_keymap[1] = [
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
            (2, 5, HidKeys::MacroSuperLock),
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
            (3, 8, HidKeys::Layer1),
            (3, 9, HidKeys::Undefined),
            (3, 10, HidKeys::Undefined),
            (3, 11, HidKeys::Undefined),
        ];

        for (row, col, key) in layer_keymap[1].iter() {
            layout.keymap[1]
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

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

    // BASE LAYER LAYOUT

    layout
        .base
        .insert((0, 0), KeyCode::new(HidKeys::Escape, HidKeys::Escape))
        .unwrap(); // Escape
    layout
        .base
        .insert((0, 1), KeyCode::new(HidKeys::Quote, HidKeys::Quote))
        .unwrap(); // '
    layout
        .base
        .insert((0, 2), KeyCode::new(HidKeys::Comma, HidKeys::Comma))
        .unwrap(); // ,
    layout
        .base
        .insert((0, 3), KeyCode::new(HidKeys::Period, HidKeys::Period))
        .unwrap(); // .
    layout
        .base
        .insert((0, 4), KeyCode::new(HidKeys::P, HidKeys::P))
        .unwrap(); // p
    layout
        .base
        .insert((0, 5), KeyCode::new(HidKeys::Y, HidKeys::Y))
        .unwrap(); // y
    layout
        .base
        .insert((0, 6), KeyCode::new(HidKeys::F, HidKeys::F))
        .unwrap(); // f
    layout
        .base
        .insert((0, 7), KeyCode::new(HidKeys::G, HidKeys::G))
        .unwrap(); // g
    layout
        .base
        .insert((0, 8), KeyCode::new(HidKeys::C, HidKeys::C))
        .unwrap(); // c
    layout
        .base
        .insert((0, 9), KeyCode::new(HidKeys::R, HidKeys::R))
        .unwrap(); // r
    layout
        .base
        .insert((0, 10), KeyCode::new(HidKeys::L, HidKeys::L))
        .unwrap(); // l
    layout
        .base
        .insert((0, 11), KeyCode::new(HidKeys::Slash, HidKeys::Slash))
        .unwrap(); // /

    layout
        .base
        .insert((1, 0), KeyCode::new(HidKeys::Bspace, HidKeys::Bspace))
        .unwrap(); // Backspace
    layout
        .base
        .insert((1, 1), KeyCode::new(HidKeys::A, HidKeys::A))
        .unwrap(); // a
    layout
        .base
        .insert((1, 2), KeyCode::new(HidKeys::O, HidKeys::O))
        .unwrap(); // o
    layout
        .base
        .insert((1, 3), KeyCode::new(HidKeys::E, HidKeys::E))
        .unwrap(); // e
    layout
        .base
        .insert((1, 4), KeyCode::new(HidKeys::U, HidKeys::ModifierShift))
        .unwrap(); // u
    layout
        .base
        .insert((1, 5), KeyCode::new(HidKeys::I, HidKeys::I))
        .unwrap(); // i
    layout
        .base
        .insert((1, 6), KeyCode::new(HidKeys::D, HidKeys::D))
        .unwrap(); // d
    layout
        .base
        .insert((1, 7), KeyCode::new(HidKeys::H, HidKeys::H))
        .unwrap(); // h
    layout
        .base
        .insert((1, 8), KeyCode::new(HidKeys::T, HidKeys::T))
        .unwrap(); // t
    layout
        .base
        .insert((1, 9), KeyCode::new(HidKeys::N, HidKeys::N))
        .unwrap(); // n
    layout
        .base
        .insert((1, 10), KeyCode::new(HidKeys::S, HidKeys::S))
        .unwrap(); // s
    layout
        .base
        .insert((1, 11), KeyCode::new(HidKeys::Minus, HidKeys::Minus))
        .unwrap(); // -

    layout
        .base
        .insert(
            (2, 0),
            KeyCode::new(HidKeys::ModifierControl, HidKeys::ModifierControl),
        )
        .unwrap(); // CTRL
    layout
        .base
        .insert((2, 1), KeyCode::new(HidKeys::SemiColon, HidKeys::SemiColon))
        .unwrap(); // ;
    layout
        .base
        .insert((2, 2), KeyCode::new(HidKeys::Q, HidKeys::Q))
        .unwrap(); // q
    layout
        .base
        .insert((2, 3), KeyCode::new(HidKeys::J, HidKeys::J))
        .unwrap(); // j
    layout
        .base
        .insert((2, 4), KeyCode::new(HidKeys::K, HidKeys::K))
        .unwrap(); // k
    layout
        .base
        .insert((2, 5), KeyCode::new(HidKeys::X, HidKeys::X))
        .unwrap(); // x
    layout
        .base
        .insert((2, 6), KeyCode::new(HidKeys::B, HidKeys::B))
        .unwrap(); // b
    layout
        .base
        .insert((2, 7), KeyCode::new(HidKeys::M, HidKeys::M))
        .unwrap(); // m
    layout
        .base
        .insert((2, 8), KeyCode::new(HidKeys::W, HidKeys::W))
        .unwrap(); // w
    layout
        .base
        .insert((2, 9), KeyCode::new(HidKeys::V, HidKeys::V))
        .unwrap(); // v
    layout
        .base
        .insert((2, 10), KeyCode::new(HidKeys::Z, HidKeys::Z))
        .unwrap(); // z
    layout
        .base
        .insert((2, 11), KeyCode::new(HidKeys::Equal, HidKeys::Equal))
        .unwrap(); // =

    layout
        .base
        .insert((3, 0), KeyCode::new(HidKeys::Undefined, HidKeys::Undefined))
        .unwrap(); //
    layout
        .base
        .insert((3, 1), KeyCode::new(HidKeys::Undefined, HidKeys::Undefined))
        .unwrap(); //
    layout
        .base
        .insert((3, 2), KeyCode::new(HidKeys::Undefined, HidKeys::Undefined))
        .unwrap(); //
    layout
        .base
        .insert(
            (3, 3),
            KeyCode::new(HidKeys::ModifierAlt, HidKeys::ModifierAlt),
        )
        .unwrap(); // Alt Layout
    layout
        .base
        .insert((3, 4), KeyCode::new(HidKeys::Space, HidKeys::Space))
        .unwrap(); // Space
    layout
        .base
        .insert(
            (3, 5),
            KeyCode::new(HidKeys::ModifierShift, HidKeys::ModifierShift),
        )
        .unwrap(); // Shift
    layout
        .base
        .insert((3, 6), KeyCode::new(HidKeys::Tab, HidKeys::Tab))
        .unwrap(); // Tab
    layout
        .base
        .insert((3, 7), KeyCode::new(HidKeys::Enter, HidKeys::Enter))
        .unwrap(); // Enter
    layout
        .base
        .insert(
            (3, 8),
            KeyCode::new(HidKeys::UpperLayer, HidKeys::UpperLayer),
        )
        .unwrap(); // Upper Layer
    layout
        .base
        .insert((3, 9), KeyCode::new(HidKeys::Undefined, HidKeys::Undefined))
        .unwrap(); // Undefined
    layout
        .base
        .insert(
            (3, 10),
            KeyCode::new(HidKeys::Undefined, HidKeys::Undefined),
        )
        .unwrap(); // Undefined
    layout
        .base
        .insert(
            (3, 11),
            KeyCode::new(HidKeys::Undefined, HidKeys::Undefined),
        )
        .unwrap(); // Undefined

    // UPPER LAYER LAYOUT
    layout
        .upper
        .insert((0, 0), KeyCode::new(HidKeys::Escape, HidKeys::Undefined))
        .unwrap(); // Escape
    layout
        .upper
        .insert(
            (0, 1),
            KeyCode::new(HidKeys::ModifierSuper, HidKeys::Undefined),
        )
        .unwrap(); // Super
    layout
        .upper
        .insert((0, 2), KeyCode::new(HidKeys::Num7, HidKeys::Undefined))
        .unwrap(); // 7
    layout
        .upper
        .insert((0, 3), KeyCode::new(HidKeys::Num8, HidKeys::Undefined))
        .unwrap(); // 8
    layout
        .upper
        .insert((0, 4), KeyCode::new(HidKeys::Num9, HidKeys::Undefined))
        .unwrap(); // 9
    layout
        .upper
        .insert((0, 5), KeyCode::new(HidKeys::Pscreen, HidKeys::Undefined))
        .unwrap(); // Print Screen
    layout
        .upper
        .insert(
            (0, 6),
            KeyCode::new(HidKeys::MacroExclamationMark, HidKeys::Undefined),
        )
        .unwrap(); // !
    layout
        .upper
        .insert((0, 7), KeyCode::new(HidKeys::MacroAt, HidKeys::Undefined))
        .unwrap(); // @
    layout
        .upper
        .insert((0, 8), KeyCode::new(HidKeys::MacroHash, HidKeys::Undefined))
        .unwrap(); // #
    layout
        .upper
        .insert(
            (0, 9),
            KeyCode::new(HidKeys::MacroDollar, HidKeys::Undefined),
        )
        .unwrap(); // $
    layout
        .upper
        .insert(
            (0, 10),
            KeyCode::new(HidKeys::MacroModul, HidKeys::Undefined),
        )
        .unwrap(); // %
    layout
        .upper
        .insert(
            (0, 11),
            KeyCode::new(HidKeys::MacroCaret, HidKeys::Undefined),
        )
        .unwrap(); // ^

    layout
        .upper
        .insert((1, 0), KeyCode::new(HidKeys::Bspace, HidKeys::Undefined))
        .unwrap(); // Backspace
    layout
        .upper
        .insert((1, 1), KeyCode::new(HidKeys::Period, HidKeys::Undefined))
        .unwrap(); // Period
    layout
        .upper
        .insert((1, 2), KeyCode::new(HidKeys::Num4, HidKeys::Undefined))
        .unwrap(); // 4
    layout
        .upper
        .insert((1, 3), KeyCode::new(HidKeys::Num5, HidKeys::Undefined))
        .unwrap(); // 5
    layout
        .upper
        .insert((1, 4), KeyCode::new(HidKeys::Num6, HidKeys::Undefined))
        .unwrap(); // 6
    layout
        .upper
        .insert((1, 5), KeyCode::new(HidKeys::Delete, HidKeys::Undefined))
        .unwrap(); // Delete

    layout
        .upper
        .insert(
            (1, 6),
            KeyCode::new(HidKeys::MacroAmpersand, HidKeys::Undefined),
        )
        .unwrap(); // &
    layout
        .upper
        .insert((1, 7), KeyCode::new(HidKeys::Left, HidKeys::Undefined))
        .unwrap(); // Left
    layout
        .upper
        .insert((1, 8), KeyCode::new(HidKeys::Down, HidKeys::Undefined))
        .unwrap(); // Down
    layout
        .upper
        .insert((1, 9), KeyCode::new(HidKeys::Up, HidKeys::Undefined))
        .unwrap(); // Up
    layout
        .upper
        .insert((1, 10), KeyCode::new(HidKeys::Right, HidKeys::Undefined))
        .unwrap(); // Right
    layout
        .upper
        .insert(
            (1, 11),
            KeyCode::new(HidKeys::MacroAsterix, HidKeys::Undefined),
        )
        .unwrap(); // *

    layout
        .upper
        .insert(
            (2, 0),
            KeyCode::new(HidKeys::ModifierControl, HidKeys::Undefined),
        )
        .unwrap(); // CONTROL
    layout
        .upper
        .insert((2, 1), KeyCode::new(HidKeys::Num0, HidKeys::Undefined))
        .unwrap(); // 0
    layout
        .upper
        .insert((2, 2), KeyCode::new(HidKeys::Num1, HidKeys::Undefined))
        .unwrap(); // 1
    layout
        .upper
        .insert((2, 3), KeyCode::new(HidKeys::Num2, HidKeys::Undefined))
        .unwrap(); // 2
    layout
        .upper
        .insert((2, 4), KeyCode::new(HidKeys::Num3, HidKeys::Undefined))
        .unwrap(); // 3
    layout
        .upper
        .insert(
            (2, 5),
            KeyCode::new(HidKeys::MacroPaste, HidKeys::Undefined),
        )
        .unwrap(); // MACRO PASTE
    layout
        .upper
        .insert((2, 6), KeyCode::new(HidKeys::Backslash, HidKeys::Undefined))
        .unwrap(); // \
    layout
        .upper
        .insert((2, 7), KeyCode::new(HidKeys::Lbracket, HidKeys::Undefined))
        .unwrap(); // [
    layout
        .upper
        .insert((2, 8), KeyCode::new(HidKeys::Rbracket, HidKeys::Undefined))
        .unwrap(); // ]
    layout
        .upper
        .insert(
            (2, 9),
            KeyCode::new(HidKeys::MacroOpenedBracket, HidKeys::Undefined),
        )
        .unwrap(); // (
    layout
        .upper
        .insert(
            (2, 10),
            KeyCode::new(HidKeys::MacroClosedBracket, HidKeys::Undefined),
        )
        .unwrap(); // )
    layout
        .upper
        .insert((2, 11), KeyCode::new(HidKeys::Grave, HidKeys::Undefined))
        .unwrap(); // ` or ~ with Shift

    layout
        .upper
        .insert((3, 0), KeyCode::new(HidKeys::Undefined, HidKeys::Undefined))
        .unwrap(); // Undefined
    layout
        .upper
        .insert((3, 1), KeyCode::new(HidKeys::Undefined, HidKeys::Undefined))
        .unwrap(); // Undefined
    layout
        .upper
        .insert((3, 2), KeyCode::new(HidKeys::Undefined, HidKeys::Undefined))
        .unwrap(); // Undefined
    layout
        .upper
        .insert(
            (3, 3),
            KeyCode::new(HidKeys::ModifierAlt, HidKeys::ModifierAlt),
        )
        .unwrap(); // Alt
    layout
        .upper
        .insert((3, 4), KeyCode::new(HidKeys::Space, HidKeys::Undefined))
        .unwrap(); // Space
    layout
        .upper
        .insert(
            (3, 5),
            KeyCode::new(HidKeys::ModifierShift, HidKeys::Undefined),
        )
        .unwrap(); // Shift
    layout
        .upper
        .insert((3, 6), KeyCode::new(HidKeys::Tab, HidKeys::Undefined))
        .unwrap(); // Tab
    layout
        .upper
        .insert((3, 7), KeyCode::new(HidKeys::Enter, HidKeys::Undefined))
        .unwrap(); // Enter
    layout
        .upper
        .insert(
            (3, 8),
            KeyCode::new(HidKeys::UpperLayer, HidKeys::Undefined),
        )
        .unwrap(); // Upper Layer
    layout
        .upper
        .insert((3, 9), KeyCode::new(HidKeys::Undefined, HidKeys::Undefined))
        .unwrap(); // Undefined
    layout
        .upper
        .insert(
            (3, 10),
            KeyCode::new(HidKeys::Undefined, HidKeys::Undefined),
        )
        .unwrap(); // Undefined
    layout
        .upper
        .insert(
            (3, 11),
            KeyCode::new(HidKeys::Undefined, HidKeys::Undefined),
        )
        .unwrap(); // Undefined

    // LOWER LAYER LAYOUT
    // layout.lower.insert((0, 0), KeyCode::Escape).unwrap(); // Escape
    // layout
    //     .lower
    //     .insert((0, 1), KeyCode::MacroSuperLock)
    //     .unwrap(); // Macro Lock Desktop
    // layout.lower.insert((0, 2), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((0, 3), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((0, 4), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((0, 5), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((0, 6), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((0, 7), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((0, 8), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((0, 9), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((0, 10), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((0, 11), KeyCode::Undefined).unwrap(); // Undefined

    // layout.lower.insert((1, 0), KeyCode::Bspace).unwrap(); // Backspace
    // layout.lower.insert((1, 1), KeyCode::ModifierAlt).unwrap(); // Alt
    // layout.lower.insert((1, 2), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((1, 3), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((1, 4), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((1, 5), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((1, 6), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((1, 7), KeyCode::Left).unwrap(); // Left
    // layout.lower.insert((1, 8), KeyCode::Down).unwrap(); // Down
    // layout.lower.insert((1, 9), KeyCode::Up).unwrap(); // Up
    // layout.lower.insert((1, 10), KeyCode::Right).unwrap(); // Right
    // layout.lower.insert((1, 11), KeyCode::Undefined).unwrap(); // Undefined

    // layout
    //     .lower
    //     .insert((2, 0), KeyCode::ModifierControl)
    //     .unwrap(); // Modifier
    // layout.lower.insert((2, 1), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((2, 2), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((2, 3), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((2, 4), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((2, 5), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((2, 6), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((2, 7), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((2, 8), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((2, 9), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((2, 10), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((2, 11), KeyCode::Undefined).unwrap(); // Undefined

    // layout.lower.insert((3, 0), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((3, 1), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((3, 2), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((3, 3), KeyCode::ModifierAlt).unwrap(); // Lower Layer
    // layout.lower.insert((3, 4), KeyCode::Space).unwrap(); // Space
    // layout.lower.insert((3, 5), KeyCode::ModifierShift).unwrap(); // Shift
    // layout.lower.insert((3, 6), KeyCode::Tab).unwrap(); // Tab
    // layout.lower.insert((3, 7), KeyCode::Enter).unwrap(); // Enter
    // layout.lower.insert((3, 8), KeyCode::UpperLayer).unwrap(); // Upper Layer
    // layout.lower.insert((3, 9), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((3, 10), KeyCode::Undefined).unwrap(); // Undefined
    // layout.lower.insert((3, 11), KeyCode::Undefined).unwrap(); // Undefined

    // //return the layout
    layout
}

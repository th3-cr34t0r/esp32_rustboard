//
//*********************************************************************************************
//BASE LAYER:
//
//X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |           X \ Y|  6  |  7  |  8  |  9  |  10 |  11 |
//   0 |_ESC_|__q__|__w__|__e__|__r__|__t__|              0 |__y__|__u__|__i__|__o__|__p__|__[__|
//   1 |_BSP_|__a__|__s__|__d__|__f__|__g__|              1 |__h__|__j__|__k__|__l__|__;__|__'__|
//   2 |_CTL_|__z__|__x__|__c__|__v__|__b__|              2 |__n__|__m__|__,__|__.__|__/__|__]__|
//   3                   |_LYR_|_SPC_|_SFT_|              3 |_ALT_|_ENT_|_LYR_|
//
//*****************************************************************************
//UPPER LAYER:
//
//X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |           X \ Y|  6  |  7  |  8  |  9  |  10 |  11 |
//   0 |_ESC_|_SUP_|__7__|__8__|__9__|_PScr|              0 |__!__|__@__|__#__|__$__|__%__|__^__|
//   1 |_BSP_|__.__|__4__|__5__|__6__|_DEL_|              1 |__&__|_left|_down|__up_|_rght|__*__|
//   2 |_CTL_|__0__|__1__|__2__|__3__|_PST_|              2 |__\__|__[__|__]__|__(__|__)__|__`__|
//   3                   |_ALT_|_SPC_|_SFT_|              3 |_TAB_|_ENT_|_LYR_|
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
    layout.base.insert((0, 0), HidKeys::Escape).unwrap();
    layout.base.insert((0, 1), HidKeys::Q).unwrap();
    layout.base.insert((0, 2), HidKeys::W).unwrap();
    layout.base.insert((0, 3), HidKeys::E).unwrap();
    layout.base.insert((0, 4), HidKeys::R).unwrap();
    layout.base.insert((0, 5), HidKeys::T).unwrap();
    layout.base.insert((0, 6), HidKeys::Y).unwrap();
    layout.base.insert((0, 7), HidKeys::U).unwrap();
    layout.base.insert((0, 8), HidKeys::I).unwrap();
    layout.base.insert((0, 9), HidKeys::O).unwrap();
    layout.base.insert((0, 10), HidKeys::P).unwrap();
    layout.base.insert((0, 11), HidKeys::Lbracket).unwrap();

    layout.base.insert((1, 0), HidKeys::Bspace).unwrap();
    layout.base.insert((1, 1), HidKeys::A).unwrap();
    layout.base.insert((1, 2), HidKeys::S).unwrap();
    layout.base.insert((1, 3), HidKeys::D).unwrap();
    layout.base.insert((1, 4), HidKeys::F).unwrap();
    layout.base.insert((1, 5), HidKeys::G).unwrap();
    layout.base.insert((1, 6), HidKeys::H).unwrap();
    layout.base.insert((1, 7), HidKeys::J).unwrap();
    layout.base.insert((1, 8), HidKeys::K).unwrap();
    layout.base.insert((1, 9), HidKeys::L).unwrap();
    layout.base.insert((1, 10), HidKeys::SemiColon).unwrap();
    layout.base.insert((1, 11), HidKeys::Quote).unwrap();

    layout
        .base
        .insert((2, 0), HidKeys::ModifierControl)
        .unwrap(); // Control
    layout.base.insert((2, 1), HidKeys::Z).unwrap();
    layout.base.insert((2, 2), HidKeys::X).unwrap();
    layout.base.insert((2, 3), HidKeys::C).unwrap();
    layout.base.insert((2, 4), HidKeys::V).unwrap();
    layout.base.insert((2, 5), HidKeys::B).unwrap();
    layout.base.insert((2, 6), HidKeys::N).unwrap();
    layout.base.insert((2, 7), HidKeys::M).unwrap();
    layout.base.insert((2, 8), HidKeys::Comma).unwrap();
    layout.base.insert((2, 9), HidKeys::Period).unwrap();
    layout.base.insert((2, 10), HidKeys::Slash).unwrap();
    layout.base.insert((2, 11), HidKeys::Rbracket).unwrap();

    layout.base.insert((3, 0), HidKeys::Undefined).unwrap();
    layout.base.insert((3, 1), HidKeys::Undefined).unwrap();
    layout.base.insert((3, 2), HidKeys::Undefined).unwrap();
    layout.base.insert((3, 3), HidKeys::ModifierAlt).unwrap();
    layout.base.insert((3, 4), HidKeys::Space).unwrap();
    layout.base.insert((3, 5), HidKeys::ModifierShift).unwrap();
    layout.base.insert((3, 6), HidKeys::Tab).unwrap();
    layout.base.insert((3, 7), HidKeys::Enter).unwrap();
    layout.base.insert((3, 8), HidKeys::UpperLayer).unwrap();
    layout.base.insert((3, 9), HidKeys::Undefined).unwrap();
    layout.base.insert((3, 10), HidKeys::Undefined).unwrap();
    layout.base.insert((3, 11), HidKeys::Undefined).unwrap();

    // UPPER LAYER LAYOUT
    layout.upper.insert((0, 0), HidKeys::Escape).unwrap();
    layout.upper.insert((0, 1), HidKeys::ModifierSuper).unwrap();
    layout.upper.insert((0, 2), HidKeys::Num7).unwrap();
    layout.upper.insert((0, 3), HidKeys::Num8).unwrap();
    layout.upper.insert((0, 4), HidKeys::Num9).unwrap();
    layout.upper.insert((0, 5), HidKeys::Pscreen).unwrap();
    layout
        .upper
        .insert((0, 6), HidKeys::MacroExclamationMark)
        .unwrap();
    layout.upper.insert((0, 7), HidKeys::MacroAt).unwrap();
    layout.upper.insert((0, 8), HidKeys::MacroHash).unwrap();
    layout.upper.insert((0, 9), HidKeys::MacroDollar).unwrap();
    layout.upper.insert((0, 10), HidKeys::MacroModul).unwrap();
    layout.upper.insert((0, 11), HidKeys::MacroCaret).unwrap();

    layout.upper.insert((1, 0), HidKeys::Bspace).unwrap();
    layout.upper.insert((1, 1), HidKeys::Period).unwrap();
    layout.upper.insert((1, 2), HidKeys::Num4).unwrap();
    layout.upper.insert((1, 3), HidKeys::Num5).unwrap();
    layout.upper.insert((1, 4), HidKeys::Num6).unwrap();
    layout.upper.insert((1, 5), HidKeys::Delete).unwrap();

    layout
        .upper
        .insert((1, 6), HidKeys::MacroAmpersand)
        .unwrap();
    layout.upper.insert((1, 7), HidKeys::Left).unwrap();
    layout.upper.insert((1, 8), HidKeys::Down).unwrap();
    layout.upper.insert((1, 9), HidKeys::Up).unwrap();
    layout.upper.insert((1, 10), HidKeys::Right).unwrap();
    layout.upper.insert((1, 11), HidKeys::MacroAsterix).unwrap();

    layout
        .upper
        .insert((2, 0), HidKeys::ModifierControl)
        .unwrap();
    layout.upper.insert((2, 1), HidKeys::Num0).unwrap();
    layout.upper.insert((2, 2), HidKeys::Num1).unwrap();
    layout.upper.insert((2, 3), HidKeys::Num2).unwrap();
    layout.upper.insert((2, 4), HidKeys::Num3).unwrap();
    layout.upper.insert((2, 5), HidKeys::MacroPaste).unwrap();
    layout.upper.insert((2, 6), HidKeys::Backslash).unwrap();
    layout.upper.insert((2, 7), HidKeys::Lbracket).unwrap();
    layout.upper.insert((2, 8), HidKeys::Rbracket).unwrap();
    layout
        .upper
        .insert((2, 9), HidKeys::MacroOpenedBracket)
        .unwrap();
    layout
        .upper
        .insert((2, 10), HidKeys::MacroClosedBracket)
        .unwrap();
    layout.upper.insert((2, 11), HidKeys::Grave).unwrap();

    layout.upper.insert((3, 0), HidKeys::Undefined).unwrap();
    layout.upper.insert((3, 1), HidKeys::Undefined).unwrap();
    layout.upper.insert((3, 2), HidKeys::Undefined).unwrap();
    layout.upper.insert((3, 3), HidKeys::ModifierAlt).unwrap();
    layout.upper.insert((3, 4), HidKeys::Space).unwrap();
    layout.upper.insert((3, 5), HidKeys::ModifierShift).unwrap();
    layout.upper.insert((3, 6), HidKeys::Tab).unwrap();
    layout.upper.insert((3, 7), HidKeys::Enter).unwrap();
    layout.upper.insert((3, 8), HidKeys::UpperLayer).unwrap();
    layout.upper.insert((3, 9), HidKeys::Undefined).unwrap();
    layout.upper.insert((3, 10), HidKeys::Undefined).unwrap();
    layout.upper.insert((3, 11), HidKeys::Undefined).unwrap();

    // LOWER LAYER LAYOUT
    layout.lower.insert((0, 0), HidKeys::Escape).unwrap(); // Escape
    layout
        .lower
        .insert((0, 1), HidKeys::MacroSuperLock)
        .unwrap(); // Macro Lock Desktop
    layout.lower.insert((0, 2), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((0, 3), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((0, 4), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((0, 5), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((0, 6), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((0, 7), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((0, 8), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((0, 9), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((0, 10), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((0, 11), HidKeys::Undefined).unwrap(); // Undefined

    layout.lower.insert((1, 0), HidKeys::Bspace).unwrap(); // Backspace
    layout.lower.insert((1, 1), HidKeys::ModifierAlt).unwrap(); // Alt
    layout.lower.insert((1, 2), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((1, 3), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((1, 4), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((1, 5), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((1, 6), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((1, 7), HidKeys::Left).unwrap(); // Left
    layout.lower.insert((1, 8), HidKeys::Down).unwrap(); // Down
    layout.lower.insert((1, 9), HidKeys::Up).unwrap(); // Up
    layout.lower.insert((1, 10), HidKeys::Right).unwrap(); // Right
    layout.lower.insert((1, 11), HidKeys::Undefined).unwrap(); // Undefined

    layout
        .lower
        .insert((2, 0), HidKeys::ModifierControl)
        .unwrap(); // Modifier
    layout.lower.insert((2, 1), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((2, 2), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((2, 3), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((2, 4), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((2, 5), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((2, 6), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((2, 7), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((2, 8), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((2, 9), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((2, 10), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((2, 11), HidKeys::Undefined).unwrap(); // Undefined

    layout.lower.insert((3, 0), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((3, 1), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((3, 2), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((3, 3), HidKeys::ModifierAlt).unwrap(); // Lower Layer
    layout.lower.insert((3, 4), HidKeys::Space).unwrap(); // Space
    layout.lower.insert((3, 5), HidKeys::ModifierShift).unwrap(); // Shift
    layout.lower.insert((3, 6), HidKeys::Tab).unwrap(); // Tab
    layout.lower.insert((3, 7), HidKeys::Enter).unwrap(); // Enter
    layout.lower.insert((3, 8), HidKeys::UpperLayer).unwrap(); // Upper Layer
    layout.lower.insert((3, 9), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((3, 10), HidKeys::Undefined).unwrap(); // Undefined
    layout.lower.insert((3, 11), HidKeys::Undefined).unwrap(); // Undefined

    //return the layout
    layout
}

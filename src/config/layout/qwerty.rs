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
    // layout.base.insert((0, 0), KeyCode::Escape).unwrap();
    // layout.base.insert((0, 1), KeyCode::Q).unwrap();
    // layout.base.insert((0, 2), KeyCode::W).unwrap();
    // layout.base.insert((0, 3), KeyCode::E).unwrap();
    // layout.base.insert((0, 4), KeyCode::R).unwrap();
    // layout.base.insert((0, 5), KeyCode::T).unwrap();
    // layout.base.insert((0, 6), KeyCode::Y).unwrap();
    // layout.base.insert((0, 7), KeyCode::U).unwrap();
    // layout.base.insert((0, 8), KeyCode::I).unwrap();
    // layout.base.insert((0, 9), KeyCode::O).unwrap();
    // layout.base.insert((0, 10), KeyCode::P).unwrap();
    // layout.base.insert((0, 11), KeyCode::Lbracket).unwrap();

    // layout.base.insert((1, 0), KeyCode::Bspace).unwrap();
    // layout.base.insert((1, 1), KeyCode::A).unwrap();
    // layout.base.insert((1, 2), KeyCode::S).unwrap();
    // layout.base.insert((1, 3), KeyCode::D).unwrap();
    // layout.base.insert((1, 4), KeyCode::F).unwrap();
    // layout.base.insert((1, 5), KeyCode::G).unwrap();
    // layout.base.insert((1, 6), KeyCode::H).unwrap();
    // layout.base.insert((1, 7), KeyCode::J).unwrap();
    // layout.base.insert((1, 8), KeyCode::K).unwrap();
    // layout.base.insert((1, 9), KeyCode::L).unwrap();
    // layout.base.insert((1, 10), KeyCode::SemiColon).unwrap();
    // layout.base.insert((1, 11), KeyCode::Quote).unwrap();

    // layout
    //     .base
    //     .insert((2, 0), KeyCode::ModifierControl)
    //     .unwrap(); // Control
    // layout.base.insert((2, 1), KeyCode::Z).unwrap();
    // layout.base.insert((2, 2), KeyCode::X).unwrap();
    // layout.base.insert((2, 3), KeyCode::C).unwrap();
    // layout.base.insert((2, 4), KeyCode::V).unwrap();
    // layout.base.insert((2, 5), KeyCode::B).unwrap();
    // layout.base.insert((2, 6), KeyCode::N).unwrap();
    // layout.base.insert((2, 7), KeyCode::M).unwrap();
    // layout.base.insert((2, 8), KeyCode::Comma).unwrap();
    // layout.base.insert((2, 9), KeyCode::Period).unwrap();
    // layout.base.insert((2, 10), KeyCode::Slash).unwrap();
    // layout.base.insert((2, 11), KeyCode::Rbracket).unwrap();

    // layout.base.insert((3, 0), KeyCode::Undefined).unwrap();
    // layout.base.insert((3, 1), KeyCode::Undefined).unwrap();
    // layout.base.insert((3, 2), KeyCode::Undefined).unwrap();
    // layout.base.insert((3, 3), KeyCode::ModifierAlt).unwrap();
    // layout.base.insert((3, 4), KeyCode::Space).unwrap();
    // layout.base.insert((3, 5), KeyCode::ModifierShift).unwrap();
    // layout.base.insert((3, 6), KeyCode::Tab).unwrap();
    // layout.base.insert((3, 7), KeyCode::Enter).unwrap();
    // layout.base.insert((3, 8), KeyCode::UpperLayer).unwrap();
    // layout.base.insert((3, 9), KeyCode::Undefined).unwrap();
    // layout.base.insert((3, 10), KeyCode::Undefined).unwrap();
    // layout.base.insert((3, 11), KeyCode::Undefined).unwrap();

    // // UPPER LAYER LAYOUT
    // layout.upper.insert((0, 0), KeyCode::Escape).unwrap();
    // layout.upper.insert((0, 1), KeyCode::ModifierSuper).unwrap();
    // layout.upper.insert((0, 2), KeyCode::Num7).unwrap();
    // layout.upper.insert((0, 3), KeyCode::Num8).unwrap();
    // layout.upper.insert((0, 4), KeyCode::Num9).unwrap();
    // layout.upper.insert((0, 5), KeyCode::Pscreen).unwrap();
    // layout
    //     .upper
    //     .insert((0, 6), KeyCode::MacroExclamationMark)
    //     .unwrap();
    // layout.upper.insert((0, 7), KeyCode::MacroAt).unwrap();
    // layout.upper.insert((0, 8), KeyCode::MacroHash).unwrap();
    // layout.upper.insert((0, 9), KeyCode::MacroDollar).unwrap();
    // layout.upper.insert((0, 10), KeyCode::MacroModul).unwrap();
    // layout.upper.insert((0, 11), KeyCode::MacroCaret).unwrap();

    // layout.upper.insert((1, 0), KeyCode::Bspace).unwrap();
    // layout.upper.insert((1, 1), KeyCode::Period).unwrap();
    // layout.upper.insert((1, 2), KeyCode::Num4).unwrap();
    // layout.upper.insert((1, 3), KeyCode::Num5).unwrap();
    // layout.upper.insert((1, 4), KeyCode::Num6).unwrap();
    // layout.upper.insert((1, 5), KeyCode::Delete).unwrap();

    // layout
    //     .upper
    //     .insert((1, 6), KeyCode::MacroAmpersand)
    //     .unwrap();
    // layout.upper.insert((1, 7), KeyCode::Left).unwrap();
    // layout.upper.insert((1, 8), KeyCode::Down).unwrap();
    // layout.upper.insert((1, 9), KeyCode::Up).unwrap();
    // layout.upper.insert((1, 10), KeyCode::Right).unwrap();
    // layout.upper.insert((1, 11), KeyCode::MacroAsterix).unwrap();

    // layout
    //     .upper
    //     .insert((2, 0), KeyCode::ModifierControl)
    //     .unwrap();
    // layout.upper.insert((2, 1), KeyCode::Num0).unwrap();
    // layout.upper.insert((2, 2), KeyCode::Num1).unwrap();
    // layout.upper.insert((2, 3), KeyCode::Num2).unwrap();
    // layout.upper.insert((2, 4), KeyCode::Num3).unwrap();
    // layout.upper.insert((2, 5), KeyCode::MacroPaste).unwrap();
    // layout.upper.insert((2, 6), KeyCode::Backslash).unwrap();
    // layout.upper.insert((2, 7), KeyCode::Lbracket).unwrap();
    // layout.upper.insert((2, 8), KeyCode::Rbracket).unwrap();
    // layout
    //     .upper
    //     .insert((2, 9), KeyCode::MacroOpenedBracket)
    //     .unwrap();
    // layout
    //     .upper
    //     .insert((2, 10), KeyCode::MacroClosedBracket)
    //     .unwrap();
    // layout.upper.insert((2, 11), KeyCode::Grave).unwrap();

    // layout.upper.insert((3, 0), KeyCode::Undefined).unwrap();
    // layout.upper.insert((3, 1), KeyCode::Undefined).unwrap();
    // layout.upper.insert((3, 2), KeyCode::Undefined).unwrap();
    // layout.upper.insert((3, 3), KeyCode::ModifierAlt).unwrap();
    // layout.upper.insert((3, 4), KeyCode::Space).unwrap();
    // layout.upper.insert((3, 5), KeyCode::ModifierShift).unwrap();
    // layout.upper.insert((3, 6), KeyCode::Tab).unwrap();
    // layout.upper.insert((3, 7), KeyCode::Enter).unwrap();
    // layout.upper.insert((3, 8), KeyCode::UpperLayer).unwrap();
    // layout.upper.insert((3, 9), KeyCode::Undefined).unwrap();
    // layout.upper.insert((3, 10), KeyCode::Undefined).unwrap();
    // layout.upper.insert((3, 11), KeyCode::Undefined).unwrap();

    // // LOWER LAYER LAYOUT
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

    //return the layout
    layout
}

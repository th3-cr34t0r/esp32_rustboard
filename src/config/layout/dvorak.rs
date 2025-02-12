/*
*********************************************************************************************
BASE LAYER:

X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |           X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |
   0 |_ESC_|__'__|__,__|__.__|__p__|__y__|              0 |__f__|__g__|__c__|__r__|__l__|__/__|
   1 |_BSP_|__a__|__o__|__e__|__u__|__i__|              1 |__d__|__h__|__t__|__n__|__s__|__-__|
   2 |_CTL_|__;__|__q__|__j__|__k__|__x__|              2 |__b__|__m__|__w__|__v__|__z__|__=__|
   3                   |_LYR_|_SPC_|_SFT_|              3 |_ALT_|_ENT_|_LYR_|

*********************************************************************************************
UPPER LAYER:

X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |           X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |
   0 |_ESC_|_SUP_|__7__|__8__|__9__|_CPY_|              0 |__!__|__@__|__#__|__$__|__%__|__^__|
   1 |_BSP_|_ALT_|__4__|__5__|__6__|_DEL_|              1 |__&__|_left|_down|__up_|_rght|__*__|
   2 |_CTL_|__0__|__1__|__2__|__3__|_PST_|              2 |__\__|__[__|__]__|__(__|__)__|_____|
   3                   |_LYR_|_SPC_|_SFT_|              3 |_TAB_|_ENT_|_LYR_|

*********************************************************************************************
*/
use crate::config::{enums::*, layout::*};

pub fn layout() -> Layers {
    let mut layout = Layers::new();

    #[cfg(feature = "left-side")]
    {
        /* BASE LAYER LAYOUT */
        layout.base.insert((0, 0), HidKeys::Escape).unwrap(); // ESC
        layout.base.insert((0, 1), HidKeys::Quote).unwrap(); // '
        layout.base.insert((0, 2), HidKeys::Comma).unwrap(); // ,
        layout.base.insert((0, 3), HidKeys::Period).unwrap(); // .
        layout.base.insert((0, 4), HidKeys::P).unwrap(); // p
        layout.base.insert((0, 5), HidKeys::Y).unwrap(); // y

        layout.base.insert((1, 0), HidKeys::Bspace).unwrap(); // BACKSPACE
        layout.base.insert((1, 1), HidKeys::A).unwrap(); // a
        layout.base.insert((1, 2), HidKeys::O).unwrap(); // o
        layout.base.insert((1, 3), HidKeys::E).unwrap(); // e
        layout.base.insert((1, 4), HidKeys::U).unwrap(); // u
        layout.base.insert((1, 5), HidKeys::I).unwrap(); // i

        layout
            .base
            .insert((2, 0), HidKeys::ModifierControl)
            .unwrap(); // CONTROL
        layout.base.insert((2, 1), HidKeys::SemiColon).unwrap(); // ;
        layout.base.insert((2, 2), HidKeys::Q).unwrap(); // q
        layout.base.insert((2, 3), HidKeys::J).unwrap(); // j
        layout.base.insert((2, 4), HidKeys::K).unwrap(); // k
        layout.base.insert((2, 5), HidKeys::X).unwrap(); // x

        layout.base.insert((3, 0), HidKeys::Undefined).unwrap(); //
        layout.base.insert((3, 1), HidKeys::Undefined).unwrap(); //
        layout.base.insert((3, 2), HidKeys::Undefined).unwrap(); //
        layout.base.insert((3, 3), HidKeys::LayerKey).unwrap(); // LAYER
        layout.base.insert((3, 4), HidKeys::Space).unwrap(); // SPACE
        layout.base.insert((3, 5), HidKeys::ModifierShift).unwrap(); // SHIFT

        /* UPPER LAYER LAYOUT */
        layout.upper.insert((0, 0), HidKeys::Escape).unwrap(); // ESC
        layout.upper.insert((0, 1), HidKeys::ModifierSuper).unwrap(); // Super
        layout.upper.insert((0, 2), HidKeys::Num7).unwrap(); // 7
        layout.upper.insert((0, 3), HidKeys::Num8).unwrap(); // 8
        layout.upper.insert((0, 4), HidKeys::Num9).unwrap(); // 9
        layout.upper.insert((0, 5), HidKeys::MacroCopy).unwrap(); // MACRO COPY

        layout.upper.insert((1, 0), HidKeys::Bspace).unwrap(); // BACKSPACE
        layout.upper.insert((1, 1), HidKeys::ModifierAlt).unwrap(); // ALT
        layout.upper.insert((1, 2), HidKeys::Num4).unwrap(); // 4
        layout.upper.insert((1, 3), HidKeys::Num5).unwrap(); // 5
        layout.upper.insert((1, 4), HidKeys::Num6).unwrap(); // 6
        layout.upper.insert((1, 5), HidKeys::Delete).unwrap(); // Delete
        layout
            .upper
            .insert((2, 0), HidKeys::ModifierControl)
            .unwrap(); // CONTROL
        layout.upper.insert((2, 1), HidKeys::Num0).unwrap(); // 0
        layout.upper.insert((2, 2), HidKeys::Num1).unwrap(); // 1
        layout.upper.insert((2, 3), HidKeys::Num2).unwrap(); // 2
        layout.upper.insert((2, 4), HidKeys::Num3).unwrap(); // 3
        layout.upper.insert((2, 5), HidKeys::MacroPaste).unwrap(); // MACRO PASTE

        layout.upper.insert((3, 0), HidKeys::Undefined).unwrap(); // Undefined
        layout.upper.insert((3, 1), HidKeys::Undefined).unwrap(); // Undefined
        layout.upper.insert((3, 2), HidKeys::Undefined).unwrap(); // Undefined
        layout.upper.insert((3, 3), HidKeys::LayerKey).unwrap(); // LAYER
        layout.upper.insert((3, 4), HidKeys::Space).unwrap(); // SPACE
        layout.upper.insert((3, 5), HidKeys::ModifierShift).unwrap(); // SHIFT
    }

    #[cfg(feature = "right-side")]
    {
        /* BASE LAYER LAYOUT */
        layout.base.insert((0, 0), HidKeys::F).unwrap(); // f
        layout.base.insert((0, 1), HidKeys::G).unwrap(); // g
        layout.base.insert((0, 2), HidKeys::C).unwrap(); // c
        layout.base.insert((0, 3), HidKeys::R).unwrap(); // r
        layout.base.insert((0, 4), HidKeys::L).unwrap(); // l
        layout.base.insert((0, 5), HidKeys::Slash).unwrap(); // /

        layout.base.insert((1, 0), HidKeys::D).unwrap(); // d
        layout.base.insert((1, 1), HidKeys::H).unwrap(); // h
        layout.base.insert((1, 2), HidKeys::T).unwrap(); // t
        layout.base.insert((1, 3), HidKeys::N).unwrap(); // n
        layout.base.insert((1, 4), HidKeys::S).unwrap(); // s
        layout.base.insert((1, 5), HidKeys::Minus).unwrap(); // -

        layout.base.insert((2, 0), HidKeys::B).unwrap(); // b
        layout.base.insert((2, 1), HidKeys::M).unwrap(); // m
        layout.base.insert((2, 2), HidKeys::W).unwrap(); // w
        layout.base.insert((2, 3), HidKeys::V).unwrap(); // v
        layout.base.insert((2, 4), HidKeys::Z).unwrap(); // z
        layout.base.insert((2, 5), HidKeys::Equal).unwrap(); // =

        layout.base.insert((3, 0), HidKeys::Tab).unwrap(); // TAB
        layout.base.insert((3, 1), HidKeys::Enter).unwrap(); // ENTER
        layout.base.insert((3, 2), HidKeys::LayerKey).unwrap(); // LAYER
        layout.base.insert((3, 3), HidKeys::Undefined).unwrap(); // Undefined
        layout.base.insert((3, 4), HidKeys::Undefined).unwrap(); // Undefined
        layout.base.insert((3, 5), HidKeys::Undefined).unwrap(); // Undefined

        /* UPPER LAYER LAYOUT */
        layout
            .upper
            .insert((0, 0), HidKeys::MacroExclamationMark)
            .unwrap(); // !
        layout.upper.insert((0, 1), HidKeys::MacroAt).unwrap(); // @
        layout.upper.insert((0, 2), HidKeys::MacroHash).unwrap(); // #
        layout.upper.insert((0, 3), HidKeys::MacroDollar).unwrap(); // $
        layout.upper.insert((0, 4), HidKeys::MacroModul).unwrap(); // %
        layout.upper.insert((0, 5), HidKeys::MacroCaret).unwrap(); // ^

        layout
            .upper
            .insert((1, 0), HidKeys::MacroAmpersand)
            .unwrap(); // &
        layout.upper.insert((1, 1), HidKeys::Left).unwrap(); // LEFT
        layout.upper.insert((1, 2), HidKeys::Down).unwrap(); // DOWN
        layout.upper.insert((1, 3), HidKeys::Up).unwrap(); // UP
        layout.upper.insert((1, 4), HidKeys::Right).unwrap(); // RIGHT
        layout.upper.insert((1, 5), HidKeys::MacroAsterix).unwrap(); // *

        layout.upper.insert((2, 0), HidKeys::Backslash).unwrap(); // \
        layout.upper.insert((2, 1), HidKeys::Lbracket).unwrap(); // [
        layout.upper.insert((2, 2), HidKeys::Rbracket).unwrap(); // ]
        layout
            .upper
            .insert((2, 3), HidKeys::MacroOpenedBracket)
            .unwrap(); // (
        layout
            .upper
            .insert((2, 4), HidKeys::MacroClosedBracket)
            .unwrap(); // )
        layout.upper.insert((2, 5), HidKeys::Grave).unwrap(); // ` or ~ with SHIFT

        layout.upper.insert((3, 0), HidKeys::Tab).unwrap(); // TAB
        layout.upper.insert((3, 1), HidKeys::Enter).unwrap(); // ENTER
        layout.upper.insert((3, 2), HidKeys::LayerKey).unwrap(); // LAYER
        layout.upper.insert((3, 3), HidKeys::Undefined).unwrap(); // Undefined
        layout.upper.insert((3, 4), HidKeys::Undefined).unwrap(); // Undefined
        layout.upper.insert((3, 5), HidKeys::Undefined).unwrap(); // Undefined
    }

    /* return the layot */
    layout
}

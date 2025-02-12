/*
*********************************************************************************************
BASE LAYER:

X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |           X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |
   0 |_ESC_|__'__|__,__|__.__|__p__|__y__|              0 |__f__|__g__|__c__|__r__|__l__|__/__|
   1 |_BSP_|__a__|__o__|__e__|__u__|__i__|              1 |__d__|__h__|__t__|__n__|__s__|__-__|
   2 |_CTL_|__;__|__q__|__j__|__k__|__x__|              2 |__b__|__m__|__w__|__v__|__z__|__=__|
   3                   |_LYR_|_SPC_|_SFT_|              3 |_ALT_|_ENT_|_LYR_|*****************************************************************************
UPPER LAYER:

X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |           X \ Y|  0  |  1  |  2  |  3  |  4  |  5  |
 0 |_ESC_|__1__|__2__|__3__|__4__|__5__|              0 |__6__|__7__|__8__|__9__|__0__|_____|
   1 |_BSP_|_____|_____|_____|copy_|paste|              1 |_____|_left|_down|__up_|_rght|_____|
   2 |_CTL_|_____|_____|_____|_____|prtsc|              2 |__\__|__[__|__]__|_____|_____|_____|
   3                   |_LYR_|_SPC_|_SFT_|              3 |_ALT_|_ENT_|_LYR_|

*********************************************************************************************
*/
use crate::config::{enums::*, layout::*};

pub fn layout() -> Layers {
    let mut layout = Layers::new();

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
    layout.base.insert((3, 3), HidKeys::Undefined).unwrap(); // LAYER
    layout.base.insert((3, 4), HidKeys::Space).unwrap(); // SPACE
    layout.base.insert((3, 5), HidKeys::ModifierShift).unwrap(); // SHIFT

    layout.upper.insert((0, 0), HidKeys::Escape).unwrap(); // ESC
    layout.upper.insert((0, 1), HidKeys::Num1).unwrap(); // 1
    layout.upper.insert((0, 2), HidKeys::Num2).unwrap(); // 2
    layout.upper.insert((0, 3), HidKeys::Num3).unwrap(); // 3
    layout.upper.insert((0, 4), HidKeys::Num4).unwrap(); // 4
    layout.upper.insert((0, 5), HidKeys::Num5).unwrap(); // 5

    layout.upper.insert((1, 0), HidKeys::Bspace).unwrap(); // BACKSPACE
    layout.upper.insert((1, 1), HidKeys::Undefined).unwrap(); // Undefined
    layout.upper.insert((1, 2), HidKeys::Undefined).unwrap(); // Undefined
    layout.upper.insert((1, 3), HidKeys::Undefined).unwrap(); // Undefined
    layout.upper.insert((1, 4), HidKeys::Copy).unwrap(); // COPY
    layout.upper.insert((1, 5), HidKeys::Paste).unwrap(); // PASTE

    layout
        .upper
        .insert((2, 0), HidKeys::ModifierControl)
        .unwrap(); // CONTROL
    layout.upper.insert((2, 1), HidKeys::Undefined).unwrap(); // Undefined
    layout.upper.insert((2, 2), HidKeys::Undefined).unwrap(); // Undefined
    layout.upper.insert((2, 3), HidKeys::Undefined).unwrap(); // Undefined
    layout.upper.insert((2, 4), HidKeys::Undefined).unwrap(); // Undefined
    layout.upper.insert((2, 5), HidKeys::Pscreen).unwrap(); // PSCREEN

    layout.upper.insert((3, 0), HidKeys::Undefined).unwrap(); // Undefined
    layout.upper.insert((3, 1), HidKeys::Undefined).unwrap(); // Undefined
    layout.upper.insert((3, 2), HidKeys::Undefined).unwrap(); // Undefined
    layout.upper.insert((3, 3), HidKeys::Undefined).unwrap(); // LAYER
    layout.upper.insert((3, 4), HidKeys::Space).unwrap(); // SPACE
    layout.upper.insert((3, 5), HidKeys::ModifierShift).unwrap(); // SHIFT

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

    layout.base.insert((3, 0), HidKeys::ModifierAlt).unwrap(); // ALT
    layout.base.insert((3, 1), HidKeys::Enter).unwrap(); // ENTER
    layout.base.insert((3, 2), HidKeys::Undefined).unwrap(); // LAYER
    layout.base.insert((3, 3), HidKeys::Undefined).unwrap(); // Undefined
    layout.base.insert((3, 4), HidKeys::Undefined).unwrap(); // Undefined
    layout.base.insert((3, 5), HidKeys::Undefined).unwrap(); // Undefined

    layout.upper.insert((0, 0), HidKeys::Num6).unwrap(); // 6
    layout.upper.insert((0, 1), HidKeys::Num7).unwrap(); // 7
    layout.upper.insert((0, 2), HidKeys::Num8).unwrap(); // 8
    layout.upper.insert((0, 3), HidKeys::Num9).unwrap(); // 9
    layout.upper.insert((0, 4), HidKeys::Num0).unwrap(); // 0
    layout.upper.insert((0, 5), HidKeys::Undefined).unwrap(); // Undefined

    layout.upper.insert((1, 0), HidKeys::Undefined).unwrap(); // Undefined
    layout.upper.insert((1, 1), HidKeys::Left).unwrap(); // LEFT
    layout.upper.insert((1, 2), HidKeys::Down).unwrap(); // DOWN
    layout.upper.insert((1, 3), HidKeys::Up).unwrap(); // UP
    layout.upper.insert((1, 4), HidKeys::Right).unwrap(); // RIGHT
    layout.upper.insert((1, 5), HidKeys::Undefined).unwrap(); // Undefined

    layout.upper.insert((2, 0), HidKeys::Backslash).unwrap(); // \
    layout.upper.insert((2, 1), HidKeys::Lbracket).unwrap(); // [
    layout.upper.insert((2, 2), HidKeys::Rbracket).unwrap(); // ]
    layout.upper.insert((2, 3), HidKeys::Undefined).unwrap(); // Undefined
    layout.upper.insert((2, 4), HidKeys::Undefined).unwrap(); // Undefined
    layout.upper.insert((2, 5), HidKeys::Undefined).unwrap(); // Undefined

    layout.upper.insert((3, 0), HidKeys::ModifierAlt).unwrap(); // ALT
    layout.upper.insert((3, 1), HidKeys::Enter).unwrap(); // ENTER
    layout.upper.insert((3, 2), HidKeys::Undefined).unwrap(); // LAYER
    layout.upper.insert((3, 3), HidKeys::Undefined).unwrap(); // Undefined
    layout.upper.insert((3, 4), HidKeys::Undefined).unwrap(); // Undefined
    layout.upper.insert((3, 5), HidKeys::Undefined).unwrap(); // Undefined

    /* return the layot */
    layout
}

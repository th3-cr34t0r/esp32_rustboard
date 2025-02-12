/* Scan codes - HID Keyboard: https://gist.github.com/MightyPork/6da26e382a7ad91b5496ee55fdc73db2 */

use heapless::Vec;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HidKeys {
    None = 0x00,
    Undefined = 0x03,
    A = 0x04,
    B = 0x05,
    C = 0x06,
    D = 0x07,
    E = 0x08,
    F = 0x09,
    G = 0x0A,
    H = 0x0B,
    I = 0x0C,
    J = 0x0D,
    K = 0x0E,
    L = 0x0F,
    M = 0x10,
    N = 0x11,
    O = 0x12,
    P = 0x13,
    Q = 0x14,
    R = 0x15,
    S = 0x16,
    T = 0x17,
    U = 0x18,
    V = 0x19,
    W = 0x1A,
    X = 0x1B,
    Y = 0x1C,
    Z = 0x1D,
    Num1 = 0x1E,
    Num2 = 0x1F,
    Num3 = 0x20,
    Num4 = 0x21,
    Num5 = 0x22,
    Num6 = 0x23,
    Num7 = 0x24,
    Num8 = 0x25,
    Num9 = 0x26,
    Num0 = 0x27,
    Enter = 0x28,
    Escape = 0x29,
    Bspace = 0x2A,
    Tab = 0x2B,
    Space = 0x2C,
    Minus = 0x2D,
    Equal = 0x2E,
    Lbracket = 0x2F,  /* [ and { */
    Rbracket = 0x30,  /* ] and } */
    Backslash = 0x31, /* \ (and |) */
    NonusHash = 0x32, /* Non-US # and ~ (Typically near the Enter key) */
    SemiColon = 0x33, /* ; (and :) */
    Quote = 0x34,     /* ' and " */
    Grave = 0x35,     /* Grave accent and tilde */
    Comma = 0x36,     /*  =  and < */
    Period = 0x37,    /* . and > */
    Slash = 0x38,     /* / and ? */
    Capslock = 0x39,
    F1 = 0x3A,
    F2 = 0x3B,
    F3 = 0x3C,
    F4 = 0x3D,
    F5 = 0x3E,
    F6 = 0x3F,
    F7 = 0x40,
    F8 = 0x41,
    F9 = 0x42,
    F10 = 0x43,
    F11 = 0x44,
    F12 = 0x45,
    Pscreen = 0x46,
    Scrolllock = 0x47,
    Pause = 0x48,
    Insert = 0x49,
    Home = 0x4A,
    Pgup = 0x4B,
    Delete = 0x4C,
    End = 0x4D,
    Pgdown = 0x4E,
    Right = 0x4F,
    Left = 0x50,
    Down = 0x51,
    Up = 0x52,
    Numlock = 0x53,
    KpSlash = 0x54,
    KpAsterisk = 0x55,
    KpMinus = 0x56,
    KpPlus = 0x57,
    KpEnter = 0x58,
    Kp1 = 0x59,
    Kp2 = 0x5A,
    Kp3 = 0x5B,
    Kp4 = 0x5C,
    Kp5 = 0x5D,
    Kp6 = 0x5E,
    Kp7 = 0x5F,
    Kp8 = 0x60,
    Kp9 = 0x61,
    Kp0 = 0x62,
    KpDot = 0x63,
    NonusBslash = 0x64, /* Non-US \ and | (Typically near the Left-Shift key) */
    Application = 0x65,
    Power = 0x66,
    KpEqual = 0x67,
    F13 = 0x68,
    F14 = 0x69,
    F15 = 0x6A,
    F16 = 0x6B,
    F17 = 0x6C,
    F18 = 0x6D,
    F19 = 0x6E,
    F20 = 0x6F,
    F21 = 0x70,
    F22 = 0x71,
    F23 = 0x72,
    F24 = 0x73,
    Execute = 0x74,
    Help = 0x75,
    Menu = 0x76,
    Select = 0x77,
    Stop = 0x78,
    Again = 0x79,
    Undo = 0x7A,
    Cut = 0x7B,
    Copy = 0x7C,
    Paste = 0x7D,
    Find = 0x7E,
    Mute = 0x7F,
    Volup = 0x80,
    Voldown = 0x81,
    LockingCaps = 0x82,   /* locking Caps Lock */
    LockingNum = 0x83,    /* locking Num Lock */
    LockingScroll = 0x84, /* locking Scroll Lock */
    KpComma = 0x85,
    KpEqualAs400 = 0x86, /* equal sign on AS/400 */
    Int1 = 0x87,
    Int2 = 0x88,
    Int3 = 0x89,
    Int4 = 0x8A,
    Int5 = 0x8B,
    Int6 = 0x8C,
    Int7 = 0x8D,
    Int8 = 0x8E,
    Int9 = 0x8F,
    Lang1 = 0x90,
    Lang2 = 0x91,
    Lang3 = 0x92,
    Lang4 = 0x93,
    Lang5 = 0x94,
    Lang6 = 0x95,
    Lang7 = 0x96,
    Lang8 = 0x97,
    Lang9 = 0x98,
    AltErase = 0x99,
    Sysreq = 0x9A,
    Cancel = 0x9B,
    Clear = 0x9C,
    Prior = 0x9D,
    Return = 0x9E,
    Separator = 0x9F,
    Out = 0xA0,
    Oper = 0xA1,
    ClearAgain = 0xA2,
    Crsel = 0xA3,
    Exsel = 0xA4,

    /* dummy layer */
    LayerKey = 0xA5,

    /* dummy modifiers */
    ModifierShift = 0xB1,
    ModifierControl = 0xB2,
    ModifierAlt = 0xB3,
    ModifierSuper = 0xB4,

    /* dummy macros */
    MacroOpenedBracket = 0xC1,
    MacroClosedBracket = 0xC2,
    MacroCopy = 0xC3,
    MacroPaste = 0xC4,
    MacroExclamationMark = 0xC5,
    MacroAt = 0xC6,
    MacroHash = 0xC7,
    MacroDollar = 0xC8,
    MacroModul = 0xC9,
    MacroCaret = 0xD0,
    MacroAmpersand = 0xD1,
    MacroAsterix = 0xD2,
}

pub enum KeyType {
    Macro,
    Modifier,
    Key,
    Layer,
}

impl KeyType {
    pub fn check_type(key: &HidKeys) -> KeyType {
        match *key {
            HidKeys::MacroOpenedBracket
            | HidKeys::MacroClosedBracket
            | HidKeys::MacroCopy
            | HidKeys::MacroPaste
            | HidKeys::MacroExclamationMark
            | HidKeys::MacroAt
            | HidKeys::MacroHash
            | HidKeys::MacroDollar
            | HidKeys::MacroModul
            | HidKeys::MacroCaret
            | HidKeys::MacroAmpersand
            | HidKeys::MacroAsterix => KeyType::Macro,

            HidKeys::LayerKey => KeyType::Layer,

            HidKeys::ModifierShift
            | HidKeys::ModifierControl
            | HidKeys::ModifierAlt
            | HidKeys::ModifierSuper => KeyType::Modifier,

            _ => KeyType::Key,
        }
    }
}

pub enum HidModifiers {
    None = 0x00,
    Control = 0x01,
    Shift = 0x02,
    Alt = 0x04,
    Super = 0x08,
}
impl HidModifiers {
    pub fn get_modifier(key: &HidKeys) -> u8 {
        /* set the modifier */
        match *key {
            HidKeys::ModifierShift => HidModifiers::Shift as u8,
            HidKeys::ModifierControl => HidModifiers::Control as u8,
            HidKeys::ModifierAlt => HidModifiers::Alt as u8,
            HidKeys::ModifierSuper => HidModifiers::Super as u8,
            _ => 0,
        }
    }
}

impl HidKeys {
    pub fn get_macro_sequence(key: &HidKeys) -> Vec<HidKeys, 16> {
        let mut vec: Vec<HidKeys, 16> = Vec::new();

        match key {
            HidKeys::MacroCopy => {
                vec.push(HidKeys::ModifierControl).unwrap();
                vec.push(HidKeys::C).unwrap();
                vec
            }
            HidKeys::MacroPaste => {
                vec.push(HidKeys::ModifierControl).unwrap();
                vec.push(HidKeys::V).unwrap();
                vec
            }

            HidKeys::MacroClosedBracket => {
                vec.push(HidKeys::ModifierShift).unwrap();
                vec.push(HidKeys::Num0).unwrap();
                vec
            }
            HidKeys::MacroExclamationMark => {
                vec.push(HidKeys::ModifierShift).unwrap();
                vec.push(HidKeys::Num1).unwrap();
                vec
            }
            HidKeys::MacroAt => {
                vec.push(HidKeys::ModifierShift).unwrap();
                vec.push(HidKeys::Num2).unwrap();
                vec
            }
            HidKeys::MacroHash => {
                vec.push(HidKeys::ModifierShift).unwrap();
                vec.push(HidKeys::Num3).unwrap();
                vec
            }
            HidKeys::MacroDollar => {
                vec.push(HidKeys::ModifierShift).unwrap();
                vec.push(HidKeys::Num4).unwrap();
                vec
            }
            HidKeys::MacroModul => {
                vec.push(HidKeys::ModifierShift).unwrap();
                vec.push(HidKeys::Num5).unwrap();
                vec
            }
            HidKeys::MacroCaret => {
                vec.push(HidKeys::ModifierShift).unwrap();
                vec.push(HidKeys::Num6).unwrap();
                vec
            }
            HidKeys::MacroAmpersand => {
                vec.push(HidKeys::ModifierShift).unwrap();
                vec.push(HidKeys::Num7).unwrap();
                vec
            }
            HidKeys::MacroAsterix => {
                vec.push(HidKeys::ModifierShift).unwrap();
                vec.push(HidKeys::Num8).unwrap();
                vec
            }
            HidKeys::MacroOpenedBracket => {
                vec.push(HidKeys::ModifierShift).unwrap();
                vec.push(HidKeys::Num9).unwrap();
                vec
            }
            _ => vec,
        }
    }
}

/* Scan codes - HID Keyboard: https://gist.github.com/MightyPork/6da26e382a7ad91b5496ee55fdc73db2 */

use heapless::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Kc {
    None = 0x00, // None
    #[default]
    Undf = 0x03, // Undefined
    A = 0x04,    // A
    B = 0x05,    // B
    C = 0x06,    // C
    D = 0x07,    // D
    E = 0x08,    // E
    F = 0x09,    // F
    G = 0x0A,    // G
    H = 0x0B,    // H
    I = 0x0C,    // I
    J = 0x0D,    // J
    K = 0x0E,    // K
    L = 0x0F,    // L
    M = 0x10,    // M
    N = 0x11,    // N
    O = 0x12,    // O
    P = 0x13,    // P
    Q = 0x14,    // Q
    R = 0x15,    // R
    S = 0x16,    // S
    T = 0x17,    // T
    U = 0x18,    // U
    V = 0x19,    // V
    W = 0x1A,    // W
    X = 0x1B,    // X
    Y = 0x1C,    // Y
    Z = 0x1D,    // Z
    N1 = 0x1E,   // Num1
    N2 = 0x1F,   // Num2
    N3 = 0x20,   // Num3
    N4 = 0x21,   // Num4
    N5 = 0x22,   // Num5
    N6 = 0x23,   // Num6
    N7 = 0x24,   // Num7
    N8 = 0x25,   // Num8
    N9 = 0x26,   // Num9
    N0 = 0x27,   // Num0
    Entr = 0x28, // Enter
    Esc = 0x29,  // Escape
    Bksp = 0x2A, // BackSpace
    Tab = 0x2B,  // Tab
    Spac = 0x2C, // Space
    Mns = 0x2D,  // Minus
    Eq = 0x2E,   // Equal
    Lbrk = 0x2F, // LeftBracket
    Rbrk = 0x30, // RightBracket
    Bksl = 0x31, // BackSlash
    Nsh = 0x32,  // NonusHash
    Scn = 0x33,  // SemiColon
    Qte = 0x34,  // Quote
    Grav = 0x35, // Grave
    Com = 0x36,  // Comma
    Per = 0x37,  // Period
    Fsl = 0x38,  // ForwardSlash
    Caps = 0x39, // Capslock
    F1 = 0x3A,   // F1
    F2 = 0x3B,   // F2
    F3 = 0x3C,   // F3
    F4 = 0x3D,   // F4
    F5 = 0x3E,   // F5
    F6 = 0x3F,   // F6
    F7 = 0x40,   // F7
    F8 = 0x41,   // F8
    F9 = 0x42,   // F9
    F10 = 0x43,  // F10
    F11 = 0x44,  // F11
    F12 = 0x45,  // F12
    Pscr = 0x46, // Pscreen
    Scll = 0x47, // Scrolllock
    Pse = 0x48,  // Pause
    Ins = 0x49,  // Insert
    Home = 0x4A, // Home
    Pgup = 0x4B, // Pgup
    Del = 0x4C,  // Delete
    End = 0x4D,  // End
    Pgdn = 0x4E, // Pgdown
    ArR = 0x4F,  // ArrowRight
    ArL = 0x50,  // ArrowLeft
    ArD = 0x51,  // ArrowDown
    ArU = 0x52,  // ArrowUp
    Nlk = 0x53,  // Numlock
    KpS = 0x54,  // KpSlash
    KpA = 0x55,  // KpAsterisk
    KpM = 0x56,  // KpMinus
    KpP = 0x57,  // KpPlus
    KpE = 0x58,  // KpEnter
    Kp1 = 0x59,  // Kp1
    Kp2 = 0x5A,  // Kp2
    Kp3 = 0x5B,  // Kp3
    Kp4 = 0x5C,  // Kp4
    Kp5 = 0x5D,  // Kp5
    Kp6 = 0x5E,  // Kp6
    Kp7 = 0x5F,  // Kp7
    Kp8 = 0x60,  // Kp8
    Kp9 = 0x61,  // Kp9
    Kp0 = 0x62,  // Kp0
    KpD = 0x63,  // KpDot
    Nbl = 0x64,  // NonusBslash
    App = 0x65,  // Application
    Pwr = 0x66,  // Power
    KpEql = 0x67, // KpEqual
    F13 = 0x68,  // F13
    F14 = 0x69,  // F14
    F15 = 0x6A,  // F15
    F16 = 0x6B,  // F16
    F17 = 0x6C,  // F17
    F18 = 0x6D,  // F18
    F19 = 0x6E,  // F19
    F20 = 0x6F,  // F20
    F21 = 0x70,  // F21
    F22 = 0x71,  // F22
    F23 = 0x72,  // F23
    F24 = 0x73,  // F24
    Exe = 0x74,  // Execute
    Help = 0x75, // Help
    Menu = 0x76, // Menu
    Sel = 0x77,  // Select
    Stp = 0x78,  // Stop
    Agn = 0x79,  // Again
    Und = 0x7A,  // Undo
    Cut = 0x7B,  // Cut
    Cop = 0x7C,  // Copy
    Pas = 0x7D,  // Paste
    Fin = 0x7E,  // Find
    Mute = 0x7F, // Mute
    Vup = 0x80,  // Volup
    Vdown = 0x81, // Voldown
    LckC = 0x82, // LockingCaps
    LckN = 0x83, // LockingNum
    LckS = 0x84, // LockingScroll
    KpC = 0x85,  // KpComma
    KpEql400 = 0x86, // KpEqualAs400
    Int1 = 0x87, // Int1
    Int2 = 0x88, // Int2
    Int3 = 0x89, // Int3
    Int4 = 0x8A, // Int4
    Int5 = 0x8B, // Int5
    Int6 = 0x8C, // Int6
    Int7 = 0x8D, // Int7
    Int8 = 0x8E, // Int8
    Int9 = 0x8F, // Int9
    Lg1 = 0x90,  // Lang1
    Lg3 = 0x92,  // Lang3
    Lg4 = 0x93,  // Lang4
    Lg5 = 0x94,  // Lang5
    Lg6 = 0x95,  // Lang6
    Lg7 = 0x96,  // Lang7
    Lg8 = 0x97,  // Lang8
    Lg9 = 0x98,  // Lang9
    AltE = 0x99, // AltErase
    Sys = 0x9A,  // Sysreq
    Canc = 0x9B, // Cancel
    Clr = 0x9C,  // Clear
    Pri = 0x9D,  // Prior
    Ret = 0x9E,  // Return
    Sep = 0x9F,  // Separator
    Out = 0xA0,  // Out
    Oper = 0xA1, // Oper
    ClrA = 0xA2, // ClearAgain
    Crs = 0xA3,  // Crsel
    Exs = 0xA4,  // Exsel

    // dummy layer
    L1 = 0xA5, // Layer1
    L2 = 0xA6, // Layer2
    L3 = 0xA7, // Layer3
    L4 = 0xA8, // Layer4
    L5 = 0xA9, // Layer5

    // dummy modifiers
    ModSh = 0xB0, // ModifierShift
    ModCo = 0xB1, // ModifierControl
    ModAl = 0xB2, // ModifierAlt
    ModSu = 0xB3, // ModifierSuper

    // dummy macros
    MaLP = 0xC0,   // MacroLeftParenthesis
    MaRP = 0xC1,   // MacroRightParenthesis
    MaCp = 0xC2,   // MacroCopy
    MaPa = 0xC3,   // MacroPaste
    MaEx = 0xC4,   // MacroExclamationMark
    MaAt = 0xC5,   // MacroAt
    MaHs = 0xC6,   // MacroHash
    MaDl = 0xC7,   // MacroDollar
    MaMd = 0xC8,   // MacroModul
    MaCa = 0xC9,   // MacroCaret
    MaAmp = 0xCA,  // MacroAmpersand
    MaAst = 0xCB,  // MacroAsterix
    MaSL = 0xCC,   // MacroSuperLock
    MaLB = 0xCD,   // MacroLeftBrace
    MaRB = 0xCE,   // MacroRightBrace
    MaPipe = 0xDD, // MacroPipe

    // dummy mouse controls
    MoGL = 0xD0, // MouseGoLeft
    MoGD = 0xD1, // MouseGoDown
    MoGU = 0xD2, // MouseGoUp
    MoGR = 0xD3, // MouseGoRight
    MoLC = 0xD4, // MouseLeftClick
    MoRC = 0xD5, // MouseRightClick
    MoSL = 0xD6, // MouseScrollLeft
    MoSR = 0xD7, // MouseScrollRight
    MoSU = 0xD8, // MouseScrollUp
    MoSD = 0xD9, // MouseScrollDown
    MoCF = 0xDA, // MouseCursorFast
    MoCN = 0xDB, // MouseCursorNormal
    MoCS = 0xDC, // MouseCursorSlow

    // dummy combos
    ComboCtrlD = 0xF0, // ComboControlD = ctrl+backspace
}
impl Kc {
    pub fn get_macro_sequence(key: &Kc) -> Vec<Kc, 16> {
        let mut vec: Vec<Kc, 16> = Vec::new();

        match key {
            Kc::MaCp => {
                vec.push(Kc::ModCo).unwrap();
                vec.push(Kc::C).unwrap();
                vec
            }
            Kc::MaPa => {
                vec.push(Kc::ModCo).unwrap();
                vec.push(Kc::V).unwrap();
                vec
            }

            Kc::MaRP => {
                vec.push(Kc::ModSh).unwrap();
                vec.push(Kc::N0).unwrap();
                vec
            }
            Kc::MaEx => {
                vec.push(Kc::ModSh).unwrap();
                vec.push(Kc::N1).unwrap();
                vec
            }
            Kc::MaAt => {
                vec.push(Kc::ModSh).unwrap();
                vec.push(Kc::N2).unwrap();
                vec
            }
            Kc::MaHs => {
                vec.push(Kc::ModSh).unwrap();
                vec.push(Kc::N3).unwrap();
                vec
            }
            Kc::MaDl => {
                vec.push(Kc::ModSh).unwrap();
                vec.push(Kc::N4).unwrap();
                vec
            }
            Kc::MaMd => {
                vec.push(Kc::ModSh).unwrap();
                vec.push(Kc::N5).unwrap();
                vec
            }
            Kc::MaCa => {
                vec.push(Kc::ModSh).unwrap();
                vec.push(Kc::N6).unwrap();
                vec
            }
            Kc::MaAmp => {
                vec.push(Kc::ModSh).unwrap();
                vec.push(Kc::N7).unwrap();
                vec
            }
            Kc::MaAst => {
                vec.push(Kc::ModSh).unwrap();
                vec.push(Kc::N8).unwrap();
                vec
            }
            Kc::MaLP => {
                vec.push(Kc::ModSh).unwrap();
                vec.push(Kc::N9).unwrap();
                vec
            }
            Kc::MaSL => {
                vec.push(Kc::ModSu).unwrap();
                vec.push(Kc::L).unwrap();
                vec
            }
            Kc::MaLB => {
                vec.push(Kc::ModSh).unwrap();
                vec.push(Kc::Lbrk).unwrap();
                vec
            }
            Kc::MaRB => {
                vec.push(Kc::ModSh).unwrap();
                vec.push(Kc::Rbrk).unwrap();
                vec
            }
            Kc::MaPipe => {
                vec.push(Kc::ModSh).unwrap();
                vec.push(Kc::Bksl).unwrap();
                vec
            }
            _ => vec,
        }
    }

    pub fn get_combo(combo: &Kc) -> (Vec<Kc, 12>, Vec<Kc, 12>) {
        let mut combo_vec: Vec<Kc, 12> = Vec::new();
        let mut keys_to_change: Vec<Kc, 12> = Vec::new();

        match combo {
            Kc::ComboCtrlD => {
                keys_to_change.push(Kc::ModCo).unwrap();
                keys_to_change.push(Kc::D).unwrap();

                combo_vec.push(Kc::ModCo).unwrap();
                combo_vec.push(Kc::Bksp).unwrap();
                (combo_vec, keys_to_change)
            }
            _ => (combo_vec, keys_to_change),
        }
    }
}

pub enum KeyType {
    Combo,
    Macro,
    Modifier,
    Mouse,
    Key,
    Layer,
}

impl KeyType {
    pub fn check_type(key: &Kc) -> KeyType {
        match *key {
            // return Macro key type
            Kc::MaLP
            | Kc::MaRP
            | Kc::MaCp
            | Kc::MaPa
            | Kc::MaEx
            | Kc::MaAt
            | Kc::MaHs
            | Kc::MaDl
            | Kc::MaMd
            | Kc::MaCa
            | Kc::MaAmp
            | Kc::MaAst
            | Kc::MaSL
            | Kc::MaLB
            | Kc::MaRB
            | Kc::MaPipe => KeyType::Macro,

            // return Layer key type
            Kc::L1 | Kc::L2 | Kc::L3 | Kc::L4 | Kc::L5 => KeyType::Layer,

            // return Modifier key type
            Kc::ModSh | Kc::ModCo | Kc::ModAl | Kc::ModSu => KeyType::Modifier,

            // return Mouse key type
            Kc::MoGL
            | Kc::MoGD
            | Kc::MoGU
            | Kc::MoGR
            | Kc::MoLC
            | Kc::MoRC
            | Kc::MoSL
            | Kc::MoSR
            | Kc::MoSU
            | Kc::MoSD
            | Kc::MoCF
            | Kc::MoCN
            | Kc::MoCS => KeyType::Mouse,

            // return Combo key type
            Kc::ComboCtrlD => KeyType::Combo,

            _ => KeyType::Key,
        }
    }
}

pub enum HidModifiers {
    None = 0x00,
    Control = 0x01,
    ControlShift = 0x03,
    Shift = 0x02,
    Alt = 0x04,
    Super = 0x08,
}
impl HidModifiers {
    pub fn get_modifier(key: &Kc) -> u8 {
        // set the modifier
        match *key {
            Kc::ModSh => HidModifiers::Shift as u8,
            Kc::ModCo => HidModifiers::Control as u8,
            Kc::ModAl => HidModifiers::Alt as u8,
            Kc::ModSu => HidModifiers::Super as u8,
            _ => 0,
        }
    }
}

pub enum HidMouseKeys {
    LeftClick = 0x01,
    RightClick = 0x02,
    MiddleClick = 0x04,
    Back = 0x08,
    Forward = 0x10,
}

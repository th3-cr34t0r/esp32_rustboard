pub const SHIFT: isize = 0x80;

pub enum Layer {
    Base,
    Shift,
    Upper,
}

#[derive(Clone, Copy, Debug)]
pub enum HidMapings {
    No = 0x00,
    RollOver,              /* 0x01 */
    PostFail,              /* 0x02 */
    Undefined,             /* 0x03 */
    LowerA,                /* 0x04 */
    LowerB,                /* 0x05 */
    LowerC,                /* 0x06 */
    LowerD,                /* 0x07 */
    LowerE,                /* 0x08 */
    LowerF,                /* 0x09 */
    LowerG,                /* 0x0A */
    LowerH,                /* 0x0B */
    LowerI,                /* 0x0C */
    LowerJ,                /* 0x0D */
    LowerK,                /* 0x0E */
    LowerL,                /* 0x0F */
    LowerM,                /* 0x10 */
    LowerN,                /* 0x11 */
    LowerO,                /* 0x12 */
    LowerP,                /* 0x13 */
    LowerQ,                /* 0x14 */
    LowerR,                /* 0x15 */
    LowerS,                /* 0x16 */
    LowerT,                /* 0x17 */
    LowerU,                /* 0x18 */
    LowerV,                /* 0x19 */
    LowerW,                /* 0x1A */
    LowerX,                /* 0x1B */
    LowerY,                /* 0x1C */
    LowerZ,                /* 0x1D */
    UpperA = 0x04 | SHIFT, /* 0x04 */
    UpperB = 0x05 | SHIFT, /* 0x05 */
    UpperC = 0x06 | SHIFT, /* 0x06 */
    UpperD = 0x07 | SHIFT, /* 0x07 */
    UpperE = 0x08 | SHIFT, /* 0x08 */
    UpperF = 0x09 | SHIFT, /* 0x09 */
    UpperG = 0x0A | SHIFT, /* 0x0A */
    UpperH = 0x0B | SHIFT, /* 0x0B */
    UpperI = 0x0C | SHIFT, /* 0x0C */
    UpperJ = 0x0D | SHIFT, /* 0x0D */
    UpperK = 0x0E | SHIFT, /* 0x0E */
    UpperL = 0x0F | SHIFT, /* 0x0F */
    UpperM = 0x10 | SHIFT, /* 0x10 */
    UpperN = 0x11 | SHIFT, /* 0x11 */
    UpperO = 0x12 | SHIFT, /* 0x12 */
    UpperP = 0x13 | SHIFT, /* 0x13 */
    UpperQ = 0x14 | SHIFT, /* 0x14 */
    UpperR = 0x15 | SHIFT, /* 0x15 */
    UpperS = 0x16 | SHIFT, /* 0x16 */
    UpperT = 0x17 | SHIFT, /* 0x17 */
    UpperU = 0x18 | SHIFT, /* 0x18 */
    UpperV = 0x19 | SHIFT, /* 0x19 */
    UpperW = 0x1A | SHIFT, /* 0x1A */
    UpperX = 0x1B | SHIFT, /* 0x1B */
    UpperY = 0x1C | SHIFT, /* 0x1C */
    UpperZ = 0x1D | SHIFT, /* 0x1D */
    Num1,                  /* 0x1E */
    Num2,                  /* 0x1F */
    Num3,                  /* 0x20 */
    Num4,                  /* 0x21 */
    Num5,                  /* 0x22 */
    Num6,                  /* 0x23 */
    Num7,                  /* 0x24 */
    Num8,                  /* 0x25 */
    Num9,                  /* 0x26 */
    Num0,                  /* 0x27 */
    Enter,                 /* 0x28 */
    Escape,                /* 0x29 */
    Bspace,                /* 0x2A */
    Tab,                   /* 0x2B */
    Space,                 /* 0x2C */
    Minus,                 /* 0x2D */
    Equal,                 /* 0x2E */
    Lbracket,              /* 0x2F */
    Rbracket,              /* 0x30 */
    Bslash,                /* 0x31 *//* \ (and |) */
    NonusHash,             /* 0x32 *//* Non-US # and ~ (Typically near the Enter key) */
    Scolon,                /* 0x33 *//* ; (and :) */
    Quote,                 /* 0x34 *//* ' and " */
    Grave,                 /* 0x35 *//* Grave accent and tilde */
    Comma,                 /* 0x36 *//* , and < */
    Dot,                   /* 0x37 *//* . and > */
    Slash,                 /* 0x38 *//* / and ? */
    Capslock,              /* 0x39 */
    F1,                    /* 0x3A */
    F2,                    /* 0x3B */
    F3,                    /* 0x3C */
    F4,                    /* 0x3D */
    F5,                    /* 0x3E */
    F6,                    /* 0x3F */
    F7,                    /* 0x40 */
    F8,                    /* 0x41 */
    F9,                    /* 0x42 */
    F10,                   /* 0x43 */
    F11,                   /* 0x44 */
    F12,                   /* 0x45 */
    Pscreen,               /* 0x46 */
    Scrolllock,            /* 0x47 */
    Pause,                 /* 0x48 */
    Insert,                /* 0x49 */
    Home,                  /* 0x4A */
    Pgup,                  /* 0x4B */
    Delete,                /* 0x4C */
    End,                   /* 0x4D */
    Pgdown,                /* 0x4E */
    Right,                 /* 0x4F */
    Left,                  /* 0x50 */
    Down,                  /* 0x51 */
    Up,                    /* 0x52 */
    Numlock,               /* 0x53 */
    KpSlash,               /* 0x54 */
    KpAsterisk,            /* 0x55 */
    KpMinus,               /* 0x56 */
    KpPlus,                /* 0x57 */
    KpEnter,               /* 0x58 */
    Kp1,                   /* 0x59 */
    Kp2,                   /* 0x5A */
    Kp3,                   /* 0x5B */
    Kp4,                   /* 0x5C */
    Kp5,                   /* 0x5D */
    Kp6,                   /* 0x5E */
    Kp7,                   /* 0x5F */
    Kp8,                   /* 0x60 */
    Kp9,                   /* 0x61 */
    Kp0,                   /* 0x62 */
    KpDot,                 /* 0x63 */
    NonusBslash,           /* 0x64 *//* Non-US \ and | (Typically near the Left-Shift key) */
    Application,           /* 0x65 */
    Power,                 /* 0x66 */
    KpEqual,               /* 0x67 */
    F13,                   /* 0x68 */
    F14,                   /* 0x69 */
    F15,                   /* 0x6A */
    F16,                   /* 0x6B */
    F17,                   /* 0x6C */
    F18,                   /* 0x6D */
    F19,                   /* 0x6E */
    F20,                   /* 0x6F */
    F21,                   /* 0x70 */
    F22,                   /* 0x71 */
    F23,                   /* 0x72 */
    F24,                   /* 0x73 */
    Execute,               /* 0x74 */
    Help,                  /* 0x75 */
    Menu,                  /* 0x76 */
    Select,                /* 0x77 */
    Stop,                  /* 0x78 */
    Again,                 /* 0x79 */
    Undo,                  /* 0x7A */
    Cut,                   /* 0x7B */
    Copy,                  /* 0x7C */
    Paste,                 /* 0x7D */
    Find,                  /* 0x7E */
    Mute,                  /* 0x7F */
    Volup,                 /* 0x80 */
    Voldown,               /* 0x81 */
    LockingCaps,           /* 0x82 *//* locking Caps Lock */
    LockingNum,            /* 0x83 *//* locking Num Lock */
    LockingScroll,         /* 0x84 *//* locking Scroll Lock */
    KpComma,               /* 0x85 */
    KpEqualAs400,          /* 0x86 *//* equal sign on AS/400 */
    Int1,                  /* 0x87 */
    Int2,                  /* 0x88 */
    Int3,                  /* 0x89 */
    Int4,                  /* 0x8A */
    Int5,                  /* 0x8B */
    Int6,                  /* 0x8C */
    Int7,                  /* 0x8D */
    Int8,                  /* 0x8E */
    Int9,                  /* 0x8F */
    Lang1,                 /* 0x90 */
    Lang2,                 /* 0x91 */
    Lang3,                 /* 0x92 */
    Lang4,                 /* 0x93 */
    Lang5,                 /* 0x94 */
    Lang6,                 /* 0x95 */
    Lang7,                 /* 0x96 */
    Lang8,                 /* 0x97 */
    Lang9,                 /* 0x98 */
    AltErase,              /* 0x99 */
    Sysreq,                /* 0x9A */
    Cancel,                /* 0x9B */
    Clear,                 /* 0x9C */
    Prior,                 /* 0x9D */
    Return,                /* 0x9E */
    Separator,             /* 0x9F */
    Out,                   /* 0xA0 */
    Oper,                  /* 0xA1 */
    ClearAgain,            /* 0xA2 */
    Crsel,                 /* 0xA3 */
    Exsel,                 /* 0xA4 */
}

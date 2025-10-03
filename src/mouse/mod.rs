use crate::config::{
    enums::{HidMouseKeys, Kc},
    user_config::{CURSOR_PARAM_FAST, CURSOR_PARAM_NORMAL, CURSOR_PARAM_SLOW},
};

#[derive(Default, Clone, Copy, PartialEq)]
enum CursorSpeed {
    Fast,
    #[default]
    Normal,
    Slow,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct MouseKeyReport {
    buttons: u8,
    x: u8,
    y: u8,
    v_wheel: u8,
    h_wheel: u8,
    speed: CursorSpeed,
}

impl MouseKeyReport {
    /// Store the struct in an array that is ready to be sent
    pub fn construct(self) -> [u8; 5] {
        [self.buttons, self.x, self.y, self.v_wheel, self.h_wheel]
    }

    /// Translate the HidKey to a mouse command
    pub fn set_command(&mut self, valid_key: &Kc) {
        match *valid_key {
            Kc::MoGL => self.go_left(),
            Kc::MoGD => self.go_down(),
            Kc::MoGU => self.go_up(),
            Kc::MoGR => self.go_right(),
            Kc::MoLC => self.click(HidMouseKeys::LeftClick),
            Kc::MoRC => self.click(HidMouseKeys::RightClick),
            Kc::MoSL => self.scroll_left(),
            Kc::MoSR => self.scroll_right(),
            Kc::MoSU => self.scroll_up(),
            Kc::MoSD => self.scroll_down(),
            Kc::MoCF => self.speed = CursorSpeed::Fast,
            Kc::MoCN => self.speed = CursorSpeed::Normal,
            Kc::MoCS => self.speed = CursorSpeed::Slow,

            _ => {} // do nothing
        }
    }

    /// Reset last pressed mouse key
    pub fn reset_keypress(&mut self, valid_key: &Kc) {
        match *valid_key {
            Kc::MoGL | Kc::MoGR => self.x = 0,
            Kc::MoGD | Kc::MoGU => self.y = 0,
            Kc::MoLC | Kc::MoRC => self.buttons = 0,
            Kc::MoSU | Kc::MoSD => self.v_wheel = 0,
            Kc::MoSL | Kc::MoSR => self.h_wheel = 0,
            Kc::MoCF | Kc::MoCS => self.speed = CursorSpeed::Normal,

            _ => {} // do nothing
        }
    }
    // Reset mouse report
    pub fn reset_report(&mut self) {
        self.buttons = 0;
        self.x = 0;
        self.y = 0;
        self.v_wheel = 0;
        self.h_wheel = 0;
    }

    /// check if cursor position is changed
    pub fn is_cursor_position_changed(&mut self) -> bool {
        (self.x | self.y != 0) || (self.v_wheel | self.h_wheel != 0)
    }

    fn go_left(&mut self) {
        match self.speed {
            CursorSpeed::Fast => self.x = 255 - CURSOR_PARAM_FAST,
            CursorSpeed::Normal => self.x = 255 - CURSOR_PARAM_NORMAL,
            CursorSpeed::Slow => self.x = 255 - CURSOR_PARAM_SLOW,
        }
    }
    fn go_right(&mut self) {
        match self.speed {
            CursorSpeed::Fast => self.x = 1 + CURSOR_PARAM_FAST,
            CursorSpeed::Normal => self.x = 1 + CURSOR_PARAM_NORMAL,
            CursorSpeed::Slow => self.x = 1 + CURSOR_PARAM_SLOW,
        }
    }
    fn go_up(&mut self) {
        match self.speed {
            CursorSpeed::Fast => self.y = 255 - CURSOR_PARAM_FAST,
            CursorSpeed::Normal => self.y = 255 - CURSOR_PARAM_NORMAL,
            CursorSpeed::Slow => self.y = 255 - CURSOR_PARAM_SLOW,
        }
    }
    fn go_down(&mut self) {
        match self.speed {
            CursorSpeed::Fast => self.y = 1 + CURSOR_PARAM_FAST,
            CursorSpeed::Normal => self.y = 1 + CURSOR_PARAM_NORMAL,
            CursorSpeed::Slow => self.y = 1 + CURSOR_PARAM_SLOW,
        }
    }

    // Method for sending the key pressed
    fn click(&mut self, button: HidMouseKeys) {
        self.buttons = button as u8;
    }

    fn scroll_left(&mut self) {
        self.h_wheel = 255;
    }
    fn scroll_right(&mut self) {
        self.h_wheel = 1;
    }
    fn scroll_up(&mut self) {
        self.v_wheel = 255;
    }
    fn scroll_down(&mut self) {
        self.v_wheel = 1;
    }
}

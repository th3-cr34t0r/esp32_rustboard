use crate::config::{
    enums::{HidKeys, HidMouseKeys},
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
    pub fn set_command(&mut self, valid_key: &HidKeys) {
        match *valid_key {
            HidKeys::MouseGoLeft => self.go_left(),
            HidKeys::MouseGoDown => self.go_down(),
            HidKeys::MouseGoUp => self.go_up(),
            HidKeys::MouseGoRight => self.go_right(),
            HidKeys::MouseLeftClick => self.click(HidMouseKeys::LeftClick),
            HidKeys::MouseRightClick => self.click(HidMouseKeys::RightClick),
            HidKeys::MouseScrollLeft => self.scroll_left(),
            HidKeys::MouseScrollRight => self.scroll_right(),
            HidKeys::MouseScrollUp => self.scroll_up(),
            HidKeys::MouseScrollDown => self.scroll_down(),
            HidKeys::MouseCursorFast => self.speed = CursorSpeed::Fast,
            HidKeys::MouseCursorNormal => self.speed = CursorSpeed::Normal,
            HidKeys::MouseCursorSlow => self.speed = CursorSpeed::Slow,

            _ => {} // do nothing
        }
    }

    /// Reset last pressed mouse key
    pub fn reset_keypress(&mut self, valid_key: &HidKeys) {
        match *valid_key {
            HidKeys::MouseGoLeft | HidKeys::MouseGoRight => self.x = 0,
            HidKeys::MouseGoDown | HidKeys::MouseGoUp => self.y = 0,
            HidKeys::MouseLeftClick | HidKeys::MouseRightClick => self.buttons = 0,
            HidKeys::MouseScrollUp | HidKeys::MouseScrollDown => self.v_wheel = 0,
            HidKeys::MouseScrollLeft | HidKeys::MouseScrollRight => self.h_wheel = 0,
            HidKeys::MouseCursorFast | HidKeys::MouseCursorSlow => self.speed = CursorSpeed::Normal,

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

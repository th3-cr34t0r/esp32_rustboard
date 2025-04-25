use crate::config::{
    enums::{HidKeys, HidMouseKeys},
    user_config::{CURSOR_PARAM_FAST, CURSOR_PARAM_NORMAL, CURSOR_PARAM_SLOW},
};

#[derive(Clone, Copy, PartialEq)]
enum CursorSpeed {
    Fast,
    Normal,
    Slow,
}

impl Default for CursorSpeed {
    fn default() -> Self {
        CursorSpeed::Normal
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct MouseReport {
    buttons: u8,
    x: u8,
    y: u8,
    v_wheel: u8,
    h_wheel: u8,
    speed: CursorSpeed,
}

impl MouseReport {
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

    /// Reset the mouse info
    pub fn reset_report(&mut self, valid_key: &HidKeys) {
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

    /// check if cursor position is changed
    pub fn is_cursor_position_changed(&mut self) -> bool {
        if self.x | self.y != 0 {
            true
        } else {
            false
        }
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
            CursorSpeed::Fast => self.x = 0 + CURSOR_PARAM_FAST,
            CursorSpeed::Normal => self.x = 0 + CURSOR_PARAM_NORMAL,
            CursorSpeed::Slow => self.x = 0 + CURSOR_PARAM_SLOW,
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
            CursorSpeed::Fast => self.y = 0 + CURSOR_PARAM_FAST,
            CursorSpeed::Normal => self.y = 0 + CURSOR_PARAM_NORMAL,
            CursorSpeed::Slow => self.y = 0 + CURSOR_PARAM_SLOW,
        }
    }
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

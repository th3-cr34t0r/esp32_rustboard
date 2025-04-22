use crate::config::{
    enums::{HidKeys, HidMouseKeys},
    user_config::FAST_CURSOR_VALUE,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct MouseReport {
    buttons: u8,
    x: u8,
    y: u8,
    v_wheel: u8,
    h_wheel: u8,
    fast_cursor: bool,
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
            HidKeys::MouseFastCursor => self.fast_cursor = true,

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
            HidKeys::MouseFastCursor => self.fast_cursor = false,

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
        match self.fast_cursor {
            true => self.x = 255 - FAST_CURSOR_VALUE,
            false => self.x = 255,
        }
    }
    fn go_right(&mut self) {
        match self.fast_cursor {
            true => self.x = 1 + FAST_CURSOR_VALUE,
            false => self.x = 1,
        }
    }
    fn go_up(&mut self) {
        self.y = 250;
        match self.fast_cursor {
            true => self.y = 255 - FAST_CURSOR_VALUE,
            false => self.y = 255,
        }
    }
    fn go_down(&mut self) {
        match self.fast_cursor {
            true => self.y = 1 + FAST_CURSOR_VALUE,
            false => self.y = 1,
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

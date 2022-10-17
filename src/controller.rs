use std::ops::{Index,IndexMut};
use std::time::Duration;

use sdl2::keyboard::Keycode;

#[derive(PartialEq, Copy, Clone)]
pub enum Button {
    Forward,
    Left,
    Right,
    Fire,
}
pub const BUTTON_COUNT: usize = 4;

pub struct ButtonsState {
    down: [bool; BUTTON_COUNT],
}

impl ButtonsState {
    pub fn new() -> ButtonsState {
        let down = [false; BUTTON_COUNT];

        ButtonsState {
            down,
        }
    }
}

impl Index<Button> for ButtonsState {
    type Output = bool;

    fn index(&self, index: Button) -> &Self::Output {
        &self.down[index as usize % BUTTON_COUNT]
    }
}

impl IndexMut<Button> for ButtonsState {
    fn index_mut(&mut self, index: Button) -> &mut Self::Output {
        &mut self.down[index as usize % BUTTON_COUNT]
    }
}

pub struct Controller {
    keycode_button_map: Vec<Option<Button>>,
}

impl Controller {
    pub fn new() -> Controller {
        let keycode_button_map = Vec::new();

        return Controller {
            keycode_button_map,
        }
    }

    fn keycode_to_index(keycode: Keycode) -> Result<usize, String> {
        let mut integer = keycode as i32;
        if integer < 0 {
            return Err("keycode less than 0".to_string());
        }
        if integer >= 0x40000000 {
            integer -= 0x40000000;
        }
        if integer >= 1024 {
            return Err("keycode greater than 1024".to_string());
        }
        return Ok(integer as usize);
    }

    pub fn map(&mut self, keycode: Keycode, button: Button) -> Result<(), String> {
        let index = Self::keycode_to_index(keycode)?;
        if index >= self.keycode_button_map.len() {
            self.keycode_button_map.resize(index + 1, None);
        }
        self.keycode_button_map[index] = Some(button);
        Ok(())
    }

    pub fn get_button(&self, keycode: Keycode) -> Option<Button> {
        let mut button = None;
        let mut keycode_integer = keycode as i32;
        if keycode_integer >= 0 {
            if keycode_integer >= 0x40000000 {
                keycode_integer -= 0x40000000;
            }
            if (keycode_integer as usize) < self.keycode_button_map.len() {
                button = self.keycode_button_map[keycode_integer as usize];
            }
        }
        return button;
    }
}

pub trait HandleButton {
    fn handle_button(&mut self, instant: Duration, button: Button, down: bool);
}

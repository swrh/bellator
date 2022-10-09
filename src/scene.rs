use std::time::Duration;

use sdl2::keyboard::Keycode;

pub struct Scene {
}

impl Scene {
    pub fn new() -> Result<Scene, String> {
        Ok(Scene {
        })
    }

    pub fn handle_keydown(&self, instant: Duration, keycode: Keycode) {
        println!("handle_keydown({}, {});", instant.as_millis(), keycode);
    }

    pub fn handle_keyup(&self, instant: Duration, keycode: Keycode) {
        println!("handle_keyup({}, {});", instant.as_millis(), keycode);
    }

    pub fn update(&self, _instant: Duration) {
    }

    pub fn render(&self) {
    }
}

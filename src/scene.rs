use std::time::Duration;

use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::entity::Entity;
use crate::player::Player;

pub struct Scene {
    entities: Vec<Box<dyn Entity>>,
    player: Player,
}

impl Scene {
    pub fn new() -> Result<Scene, String> {
        let entities: Vec<Box<dyn Entity>> = Vec::new();

        let player = Player::new()?;

        Ok(Scene {
            entities,
            player,
        })
    }

    pub fn handle_key(&self, instant: Duration, keycode: Keycode, down: bool) {
        //println!("handle_key({}, {}, {});", instant.as_millis(), keycode, down);
        match keycode {
            Keycode::A | Keycode::Left => self.player.handle_key_left(instant, down),
            Keycode::D | Keycode::Right => self.player.handle_key_right(instant, down),
            Keycode::W | Keycode::Up => self.player.handle_key_up(instant, down),
            Keycode::S | Keycode::Down => self.player.handle_key_down(instant, down),
            _ => {},
        }
    }

    pub fn update(&self, instant: Duration) {
        for e in &self.entities {
            e.update(instant);
        }

        self.player.update(instant);
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        for e in &self.entities {
            e.render(canvas);
        }

        self.player.render(canvas);
    }
}

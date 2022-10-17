use std::time::Duration;

use rand::prelude::*;

use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::controller::{Button,Controller,HandleButton};
use crate::entity::Entity;
use crate::player::Player;
use crate::point2f::Point2f;
use crate::rock::Rock;

pub struct Scene {
    entities: Vec<Box<dyn Entity>>,
    player: Player,
    controller: Controller,
}

impl Scene {
    pub fn new() -> Result<Scene, String> {
        let mut entities: Vec<Box<dyn Entity>> = Vec::new();
        entities.reserve_exact(5);

        let mut rng = thread_rng();
        for _ in 0..entities.capacity() {
            entities.push(Box::new(Rock::new(
                        Point2f { x: rng.gen_range(0.0..640.0), y: rng.gen_range(0.0..480.0), },
                        Point2f { x: rng.gen_range(-0.5..0.5), y: rng.gen_range(-0.5..0.5), },
                        rng.gen_range(10..20),
                        rng.gen_range(0.01..0.5),
                        )));
        }

        let player = Player::new()?;

        let mut controller = Controller::new();

        controller.map(Keycode::A, Button::Left)?;
        controller.map(Keycode::D, Button::Right)?;
        controller.map(Keycode::W, Button::Forward)?;
        controller.map(Keycode::Space, Button::Fire)?;

        controller.map(Keycode::Left, Button::Left)?;
        controller.map(Keycode::Right, Button::Right)?;
        controller.map(Keycode::Up, Button::Forward)?;
        controller.map(Keycode::Return, Button::Fire)?;

        Ok(Scene {
            entities,
            player,
            controller,
        })
    }

    pub fn handle_key(&mut self, instant: Duration, keycode: Keycode, down: bool) {
        match self.controller.get_button(keycode) {
            Some(button) => self.player.handle_button(instant, button, down),
            None => {},
        }
    }

    pub fn update(&mut self, instant: Duration, delta: Duration) {
        for e in &mut self.entities {
            e.update(instant, delta);
        }

        self.player.update(instant, delta);
    }

    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        for e in &mut self.entities {
            e.render(canvas);
        }

        self.player.render(canvas);
    }
}

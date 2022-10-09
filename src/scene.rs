use std::time::Duration;

use sdl2::keyboard::Keycode;

use crate::entity::Entity;

pub struct Scene {
    entities: Vec<Entity>,
}

impl Scene {
    pub fn new() -> Result<Scene, String> {
        let mut entities = Vec::new();

        entities.push(Entity::new()?);

        Ok(Scene {
            entities,
        })
    }

    pub fn handle_keydown(&self, instant: Duration, keycode: Keycode) {
        println!("handle_keydown({}, {});", instant.as_millis(), keycode);
    }

    pub fn handle_keyup(&self, instant: Duration, keycode: Keycode) {
        println!("handle_keyup({}, {});", instant.as_millis(), keycode);
    }

    pub fn update(&self, instant: Duration) {
        for e in &self.entities {
            e.update(instant);
        }
    }

    pub fn render(&self) {
        for e in &self.entities {
            e.render();
        }
    }
}

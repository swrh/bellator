use std::time::Duration;

use crate::entity::Entity;

pub struct Player {
}

impl Player {
    pub fn new() -> Result<Player, String> {
        Ok(Player {
        })
    }

    pub fn handle_key_left(&self, instant: Duration, down: bool) {
        println!("handle_key_left({}, {});", instant.as_millis(), down);
    }

    pub fn handle_key_right(&self, instant: Duration, down: bool) {
        println!("handle_key_right({}, {});", instant.as_millis(), down);
    }

    pub fn handle_key_up(&self, instant: Duration, down: bool) {
        println!("handle_key_up({}, {});", instant.as_millis(), down);
    }

    pub fn handle_key_down(&self, instant: Duration, down: bool) {
        println!("handle_key_down({}, {});", instant.as_millis(), down);
    }
}

impl Entity for Player {
    fn update(&self, _instant: Duration) {
    }

    fn render(&self) {
    }
}

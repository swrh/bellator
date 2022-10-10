use std::time::Duration;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

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

    fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.draw_line(Point::new(0, 0), Point::new(10, 10)).unwrap();
    }
}

use std::time::Duration;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::entity::Entity;

pub struct Player {
    lines: [Point; 4],
}

impl Player {
    pub fn new() -> Result<Player, String> {
        let lines = [
            Point::new(5, 0),
            Point::new(10, 15),
            Point::new(0, 15),
            Point::new(5, 0),
        ];

        Ok(Player {
            lines,
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
        canvas.draw_lines(&self.lines[..]).unwrap();
    }
}

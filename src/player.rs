use std::f64::consts::PI;
use std::time::Duration;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::entity::Entity;

pub struct Player {
    points: [Point; 3],
    theta: f64,
    position: Point,
    left: bool,
    right: bool,
}

impl Player {
    pub fn new() -> Result<Player, String> {
        let points = [
            Point::new(0, -50),
            Point::new(-25, 25),
            Point::new(25, 25),
        ];

        let theta: f64 = 0.0;

        let position = Point::new(100, 100);

        Ok(Player {
            points,
            theta,
            position,
            left: false,
            right: false,
        })
    }

    pub fn handle_key_left(&mut self, instant: Duration, down: bool) {
        println!("handle_key_left({}, {});", instant.as_millis(), down);
        self.left = down;
    }

    pub fn handle_key_right(&mut self, instant: Duration, down: bool) {
        println!("handle_key_right({}, {});", instant.as_millis(), down);
        self.right = down;
    }

    pub fn handle_key_up(&mut self, instant: Duration, down: bool) {
        println!("handle_key_up({}, {});", instant.as_millis(), down);
    }

    pub fn handle_key_down(&mut self, instant: Duration, down: bool) {
        println!("handle_key_down({}, {});", instant.as_millis(), down);
    }
}

impl Entity for Player {
    fn update(&mut self, _instant: Duration) {
        if self.left != self.right {
            let mut shift = 0.01;
            if self.left {
                shift *= -1.0;
            }
            self.theta += shift;
        }
    }

    fn render(&self, canvas: &mut Canvas<Window>) {
        let mut lines = Vec::new();
        let cos_theta = (self.theta * PI).cos();
        let sin_theta = (self.theta * PI).sin();
        lines.reserve_exact(self.points.len() + 1);
        for point in self.points {
            lines.push(Point::new(
                ((point.x as f64 * cos_theta) - (point.y as f64 * sin_theta)) as i32 + self.position.x,
                ((point.x as f64 * sin_theta) + (point.y as f64 * cos_theta)) as i32 + self.position.y,
            ));
        }
        lines.push(lines[0]);

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.draw_lines(&lines[..]).unwrap();
    }
}

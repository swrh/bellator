use std::f64::consts::PI;
use std::time::Duration;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::entity::Entity;
use crate::point2f::Point2f;

pub struct Player {
    points: [Point2f; 4],
    lines: Vec<Point>,
    theta: f64,
    position: Point2f,
    left: bool,
    right: bool,
    last_update: Duration,
}

impl Player {
    pub fn new() -> Result<Player, String> {
        let points = [
            Point2f { x: 0.0, y: -20.0, },
            Point2f { x: -10.0, y: 10.0, },
            Point2f { x: 0.0, y: 5.0, },
            Point2f { x: 10.0, y: 10.0, },
        ];

        let mut lines = Vec::new();
        lines.reserve_exact(points.len() + 1);

        let theta: f64 = 0.0;

        let position = Point2f { x: 100.0, y: 100.0, };

        Ok(Player {
            points,
            lines,
            theta,
            position,
            left: false,
            right: false,
            last_update: Duration::ZERO,
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
    fn update(&mut self, instant: Duration) {
        let delta = instant - self.last_update;

        if self.left != self.right {
            let shift = PI * 0.001 * delta.as_millis() as f64;
            if self.left {
                self.theta -= shift;
            } else {
                self.theta += shift;
            }
        }

        self.last_update = instant;
    }

    fn render(&mut self, canvas: &mut Canvas<Window>) {
        let cos_theta = self.theta.cos();
        let sin_theta = self.theta.sin();
        self.lines.clear();
        for point in &self.points {
            self.lines.push(Point::new(
                (point.x * cos_theta - point.y * sin_theta + self.position.x) as i32,
                (point.x * sin_theta + point.y * cos_theta + self.position.y) as i32,
            ));
        }
        self.lines.push(self.lines[0]);

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_lines(&self.lines[..]).unwrap();
    }
}

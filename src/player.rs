use std::f64::consts::PI;
use std::time::Duration;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::entity::Entity;
use crate::point2f::Point2f;

pub struct Player {
    ship_points: [Point2f; 4],
    ship_lines: Vec<Point>,
    fire_points: [Point2f; 3],
    fire_lines: Vec<Point>,
    theta: f64,
    position: Point2f,
    velocity: Point2f,
    left: bool,
    right: bool,
    up: bool,
    last_update: Duration,
}

impl Player {
    pub fn new() -> Result<Player, String> {
        let ship_points = [
            Point2f { x: 0.0, y: -20.0, },
            Point2f { x: -10.0, y: 10.0, },
            Point2f { x: 0.0, y: 5.0, },
            Point2f { x: 10.0, y: 10.0, },
        ];

        let mut ship_lines = Vec::new();
        ship_lines.reserve_exact(ship_points.len() + 1);

        let fire_points = [
            Point2f { x: -5.0, y: 7.5, },
            Point2f { x: 0.0, y: 17.5, },
            Point2f { x: 5.0, y: 7.5, },
        ];

        let mut fire_lines = Vec::new();
        fire_lines.reserve_exact(fire_points.len());

        let theta: f64 = 0.0;

        let position = Point2f { x: 100.0, y: 100.0, };

        Ok(Player {
            ship_points,
            ship_lines,
            fire_points,
            fire_lines,
            theta,
            position,
            velocity: Point2f { x: 0.0, y: 0.0, },
            left: false,
            right: false,
            up: false,
            last_update: Duration::ZERO,
        })
    }

    pub fn handle_key_left(&mut self, _instant: Duration, down: bool) {
        self.left = down;
    }

    pub fn handle_key_right(&mut self, _instant: Duration, down: bool) {
        self.right = down;
    }

    pub fn handle_key_up(&mut self, _instant: Duration, down: bool) {
        self.up = down;
    }

    pub fn handle_key_down(&mut self, _instant: Duration, _down: bool) {
    }
}

impl Entity for Player {
    fn update(&mut self, instant: Duration) {
        let millis = (instant - self.last_update).as_millis() as f64;

        if self.left != self.right {
            let shift = PI * 0.001 * millis;
            if self.left {
                self.theta -= shift;
            } else {
                self.theta += shift;
            }
        }

        if self.up {
            self.velocity.x += self.theta.sin() * 0.001 * millis;
            self.velocity.y -= self.theta.cos() * 0.001 * millis;
        }

        self.position.x += self.velocity.x * 0.1 * millis;
        self.position.y += self.velocity.y * 0.1 * millis;

        self.last_update = instant;
    }

    fn render(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        let cos_theta = self.theta.cos();
        let sin_theta = self.theta.sin();

        self.ship_lines.clear();
        for point in &self.ship_points {
            self.ship_lines.push(Point::new(
                (point.x * cos_theta - point.y * sin_theta + self.position.x) as i32,
                (point.x * sin_theta + point.y * cos_theta + self.position.y) as i32,
            ));
        }
        self.ship_lines.push(self.ship_lines[0]);
        canvas.draw_lines(&self.ship_lines[..]).unwrap();

        if self.up {
            self.fire_lines.clear();
            for point in &self.fire_points {
                self.fire_lines.push(Point::new(
                    (point.x * cos_theta - point.y * sin_theta + self.position.x) as i32,
                    (point.x * sin_theta + point.y * cos_theta + self.position.y) as i32,
                ));
            }
            canvas.draw_lines(&self.fire_lines[..]).unwrap();
        }
    }
}

use std::f64::consts::PI;
use std::time::Duration;

use rand::prelude::*;

use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::entity::Entity;
use crate::point2f::Point2f;

pub struct Rock {
    velocity: Point2f,
    points: Vec<Point2f>,
    lines: Vec<Point>,
    position: Point2f,
    theta: f64,
    rotation_per_sec: f64,
}

impl Rock {
    pub fn new(position: Point2f, velocity: Point2f, n_points: usize, rotation_per_sec: f64) -> Rock {
        let mut points = Vec::new();
        points.reserve_exact(n_points);

        let mut rng = thread_rng();
        let radius = rng.gen_range(5.0..25.0);
        for i in 0..n_points {
            let theta = 2. * PI / n_points as f64 * i as f64;
            points.push(Point2f {
                x: theta.sin() * (radius + rng.gen_range(0.0..=10.0)),
                y: theta.cos() * (radius + rng.gen_range(0.0..=10.0)),
            });
        }

        let mut lines = Vec::new();
        lines.reserve_exact(points.len() + 1);

        Rock {
            velocity,
            points,
            lines,
            position,
            theta: 0.,
            rotation_per_sec,
        }
    }
}

impl Entity for Rock {
    fn update(&mut self, _instant: Duration, delta: Duration) {
        let millis = delta.as_millis() as f64;

        self.position.x += self.velocity.x * 0.1 * millis;
        self.position.y += self.velocity.y * 0.1 * millis;

        while self.position.x >= 640.0 { self.position.x -= 640.0; }
        while self.position.x < 0.0 { self.position.x += 640.0; }
        while self.position.y >= 480.0 { self.position.y -= 480.0; }
        while self.position.y < 0.0 { self.position.y += 480.0; }

        self.theta += 2. * PI * self.rotation_per_sec * delta.as_secs_f64();
    }

    fn render(&mut self, canvas: &mut Canvas<Window>) {
        let sin_theta = self.theta.sin();
        let cos_theta = self.theta.cos();

        self.lines.clear();
        for point in &self.points {
            self.lines.push(Point::new(
                (point.x * cos_theta - point.y * sin_theta + self.position.x).round() as i32,
                (point.x * sin_theta + point.y * cos_theta + self.position.y).round() as i32,
            ));
        }
        self.lines.push(self.lines[0]);
        canvas.draw_lines(&self.lines[..]).unwrap();
    }
}

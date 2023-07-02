use std::time::Duration;

use sdl2::rect::Point;

use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::entity::Entity;
use crate::point2f::Point2f;

pub struct Bullet {
    instant: Duration,
    line: [Point2f; 2],
    theta: f64,
}

impl Bullet {
    pub fn new(instant: Duration, position: &Point2f, theta: f64) -> Bullet {
        let line = [
            Point2f { x: position.x, y: position.y, },
            Point2f { x: position.x + theta.sin() * 5.0, y: position.y - theta.cos() * 5.0, },
        ];

        Bullet {
            instant,
            line,
            theta,
        }
    }

    pub fn instant(&self) -> Duration {
        self.instant
    }
}

impl Entity for Bullet {
    fn update(&mut self, _instant: Duration, delta: Duration) {
        let millis = delta.as_millis() as f64;

        for line in &mut self.line {
            line.x += self.theta.sin() * 0.5 * millis;
            line.y -= self.theta.cos() * 0.5 * millis;
        }
    }

    fn render(&mut self, canvas: &mut Canvas<Window>) {
        let line = [
            Point::new(self.line[0].x.round() as i32, self.line[0].y.round() as i32),
            Point::new(self.line[1].x.round() as i32, self.line[1].y.round() as i32),
        ];

        canvas.draw_lines(&line[..]).unwrap();
    }
}

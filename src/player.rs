use std::collections::VecDeque;
use std::f64::consts::PI;
use std::time::Duration;

use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::controller::{Button,ButtonsState,HandleButton};
use crate::entity::Entity;
use crate::point2f::Point2f;

struct Bullet {
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

pub struct Player {
    ship_points: [Point2f; 4],
    ship_lines: Vec<Point>,
    fire_points: [Point2f; 3],
    fire_lines: Vec<Point>,
    theta: f64,
    position: Point2f,
    velocity: Point2f,
    buttons: ButtonsState,
    bullets: VecDeque<Bullet>,
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

        let buttons = ButtonsState::new();

        Ok(Player {
            ship_points,
            ship_lines,
            fire_points,
            fire_lines,
            theta,
            position,
            velocity: Point2f { x: 0.0, y: 0.0, },
            buttons,
            bullets: VecDeque::new(),
        })
    }
}

impl HandleButton for Player {
    fn handle_button(&mut self, instant: Duration, button: Button, down: bool) {
        if button == Button::Fire && down {
            let weapon_position = &self.position; // FIXME wrong position: should be the edge of
                                                  // the ship, and not its center
            self.bullets.push_back(Bullet::new(instant, &weapon_position, self.theta));
        }

        self.buttons[button] = down;
    }
}

impl Entity for Player {
    fn update(&mut self, instant: Duration, delta: Duration) {
        let millis = delta.as_millis() as f64;

        if self.buttons[Button::Left] != self.buttons[Button::Right] {
            let shift = PI * 0.001 * millis;
            if self.buttons[Button::Left] {
                self.theta -= shift;
            } else {
                self.theta += shift;
            }
        }

        if self.buttons[Button::Forward] {
            self.velocity.x += self.theta.sin() * 0.001 * millis;
            self.velocity.y -= self.theta.cos() * 0.001 * millis;
        }

        self.position.x += self.velocity.x * 0.1 * millis;
        self.position.y += self.velocity.y * 0.1 * millis;

        while self.position.x >= 640.0 { self.position.x -= 640.0; }
        while self.position.x < 0.0 { self.position.x += 640.0; }
        while self.position.y >= 480.0 { self.position.y -= 480.0; }
        while self.position.y < 0.0 { self.position.y += 480.0; }

        while self.bullets.len() > 0 && (instant - self.bullets.front().unwrap().instant) > Duration::from_secs(5) {
            self.bullets.pop_front();
        }

        for bullet in &mut self.bullets {
            bullet.update(instant, delta);
        }
    }

    fn render(&mut self, canvas: &mut Canvas<Window>) {
        let cos_theta = self.theta.cos();
        let sin_theta = self.theta.sin();

        self.ship_lines.clear();
        for point in &self.ship_points {
            self.ship_lines.push(Point::new(
                (point.x * cos_theta - point.y * sin_theta + self.position.x).round() as i32,
                (point.x * sin_theta + point.y * cos_theta + self.position.y).round() as i32,
            ));
        }
        self.ship_lines.push(self.ship_lines[0]);
        canvas.draw_lines(&self.ship_lines[..]).unwrap();

        if self.buttons[Button::Forward] {
            self.fire_lines.clear();
            for point in &self.fire_points {
                self.fire_lines.push(Point::new(
                    (point.x * cos_theta - point.y * sin_theta + self.position.x).round() as i32,
                    (point.x * sin_theta + point.y * cos_theta + self.position.y).round() as i32,
                ));
            }
            canvas.draw_lines(&self.fire_lines[..]).unwrap();
        }

        for bullet in &mut self.bullets {
            bullet.render(canvas);
        }
    }
}

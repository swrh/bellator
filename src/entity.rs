use std::time::Duration;

use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::point2f::Point2f;

pub trait Entity {
    fn update(&mut self, instant: Duration, delta: Duration);
    fn render(&mut self, canvas: &mut Canvas<Window>);
    fn collides_with(&self, line: &Vec<Point2f>) -> bool;
}

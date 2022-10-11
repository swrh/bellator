use std::time::Duration;

use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait Entity {
    fn update(&mut self, instant: Duration, delta: Duration);
    fn render(&mut self, canvas: &mut Canvas<Window>);
}

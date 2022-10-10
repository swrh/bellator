use std::time::Duration;

use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait Entity {
    fn update(&self, instant: Duration);
    fn render(&self, canvas: &mut Canvas<Window>);
}

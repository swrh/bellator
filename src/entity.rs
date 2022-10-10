use std::time::Duration;

pub trait Entity {
    fn update(&self, instant: Duration);
    fn render(&self);
}

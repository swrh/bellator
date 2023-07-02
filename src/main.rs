mod bullet;
mod controller;
mod entity;
mod game;
mod player;
mod point2f;
mod rock;
mod scene;

fn main() -> Result<(), String> {
    game::Game::new()?.run()
}

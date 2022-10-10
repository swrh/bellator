mod entity;
mod game;
mod player;
mod point2f;
mod scene;

fn main() -> Result<(), String> {
    game::Game::new()?.run()
}

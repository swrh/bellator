mod entity;
mod game;
mod player;
mod scene;

fn main() -> Result<(), String> {
    game::Game::new()?.run()
}

mod entity;
mod game;
mod player;
mod scene;

use game::Game;

fn main() -> Result<(), String> {
    Game::new()?.run()
}

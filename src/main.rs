mod game;
mod scene;

use game::Game;

fn main() -> Result<(), String> {
    Game::new()?.run()
}

mod game;

use game::Game;

fn main() -> Result<(), String> {
    Game::new()?.run()
}

mod game;

use crate::game::Game;

fn main() -> Result<(), String> {
    let game = Game::new()?;

    game.run();

    Ok(())
}

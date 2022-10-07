mod game;

use crate::game::Data;
use crate::game::Game;

fn main() {
    let game = <Data as Game>::new();

    game.run();
}

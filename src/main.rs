mod lib;

use lib::game::Game;

// entry point to our program
fn main() {
    let mut game = Game::new();

    game.run();
}

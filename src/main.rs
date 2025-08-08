#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(dead_code)]

mod app;

use app::game::Game;

fn main() {
    let game = Game::new();
    game.print_state();
    // game.run();
}

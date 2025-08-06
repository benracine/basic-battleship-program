#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod app;

use app::game::GameState;

fn main() {
    let mut game = GameState::new();
    game.run();
}

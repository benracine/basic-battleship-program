use super::grid::OceanGrid;
use super::player::Player;

#[derive(Debug)]
pub struct GameState {
    player1_grid: OceanGrid,
    player2_grid: OceanGrid,
    player1: Player,
    player2: Player,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            player1_grid: OceanGrid::new(),
            player2_grid: OceanGrid::new(),
            player1: Player::new("Player 1"),
            player2: Player::new("Player 2"),
        }
    }

    pub fn run(&self) {
        // Game loop or execution logic
    }
}

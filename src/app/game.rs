use crate::app::player::{Player, PlayerId};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct Game {
    pub current_turn: PlayerId,
    pub players: [Player; 2],
    pub game_phase: GamePhase,
}

impl Game {
    pub fn new() -> Self {
        let player1 = Player::new("Player 1");
        let player2 = Player::new("Player 2");

        Self {
            current_turn: player1.id,
            players: [player1, player2],
            game_phase: GamePhase::Placing,
        }
    }

    pub fn print_state(&self) {
        // Todo
    }

    pub fn run(&self) {
        loop {
            // Sleep for a short duration to simulate game loop timing
            thread::sleep(Duration::from_millis(500));
            // Print that this line was hit
            println!("Game loop iteration");
        }
    }
}

#[derive(Debug)]
enum GamePhase {
    Placing,
    InProgress,
    GameOver { winner: PlayerId },
}

use crate::app::player::{Player, PlayerId};
use std::thread;
use std::time::Duration;

#[derive(Debug, Default)]
enum Phase {
    #[default]
    InProgress,
    GameOver {
        winner: PlayerId,
    },
}

#[derive(Debug)]
pub struct Game {
    current_turn: PlayerId,
    players: [Player; 2],
    phase: Phase,
}

impl Game {
    pub fn new() -> Self {
        let players = [Player::new("Player 1"), Player::new("Player 2")];

        Self {
            current_turn: players[0].id,
            players,
            phase: Phase::default(),
        }
    }

    pub fn print_state(&self) {
        println!("{:#?}", self);
    }

    pub fn run(&self) {
        loop {
            thread::sleep(Duration::from_millis(500));
            println!("Game loop iteration");
        }
    }
}

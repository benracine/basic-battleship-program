use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::app::grid::Grid;
use crate::app::ship::{self, Ship, ShipName};

pub type PlayerId = uuid::Uuid;

#[derive(Debug)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub grid: Grid,
    pub ships: [Ship; 5],
}

impl Player {
    pub fn new(name: &str) -> Self {
        let ships: [Ship; 5] = ShipName::iter()
            .map(|ship_name| {
                let direction = if rand::random() {
                    ship::Direction::Horizontal
                } else {
                    ship::Direction::Vertical
                };
                Ship::new(ship_name, direction)
            })
            .collect::<Vec<_>>()
            .try_into()
            .expect("Expected exactly 5 ship types");
        Self {
            id: Uuid::new_v4(),
            name: name.to_owned(),
            grid: Grid::new(),
            ships,
        }
    }
}

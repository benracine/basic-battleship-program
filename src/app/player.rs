use std::convert::TryInto;

use rand::random;
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::app::grid::Grid;
use crate::app::ship::{Direction, Ship, ShipName};

pub type PlayerId = Uuid;

#[derive(Debug)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub ships: [Ship; 5],
    pub my_grid: Grid,
    // pub tracking_grid: Grid,
}

impl Player {
    pub fn new(name: &str) -> Self {
        let ships: [Ship; 5] = ShipName::iter()
            .map(|ship_name| {
                let direction = if random::<bool>() {
                    Direction::Horizontal
                } else {
                    Direction::Vertical
                };
                Ship::new(ship_name, direction)
            })
            .collect::<Vec<_>>()
            .try_into()
            .expect("Expected exactly 5 ship types");

        let mut grid = Grid::new();

        for ship in &ships {
            grid.place_ship(ship);
        }

        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            ships,
            my_grid: grid,
            // tracking_grid: Grid::new(),
        }
    }
}

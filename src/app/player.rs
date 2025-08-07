use super::{
    grid::Grid,
    ship::{Ship, ShipName},
};
use rand::prelude::*;
use strum::IntoEnumIterator;

pub type PlayerId = u32;

#[derive(Debug)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub grid: Grid,
    pub ships: Vec<Ship>,
}

impl Player {
    pub fn new(name: &str) -> Self {
        let mut rng = rand::rng();
        let grid = Grid::new();
        let rows = grid.cells.len() as u8;
        let cols = grid.cells[0].len() as u8;
        let ships = ShipName::iter()
            .map(|ship_name| Ship::new(ship_name, rows, cols))
            .collect();
        let id: u32 = rng.random::<u32>();

        Self {
            id,
            name: name.to_owned(),
            grid,
            ships,
        }
    }
}

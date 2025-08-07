use crate::app::cell::{Cell, CellState};
use rand::Rng;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, Clone, Copy)]
pub enum ShipName {
    Carrier,
    Battleship,
    Cruiser,
    Submarine,
    Destroyer,
}

pub type ShipId = u32;

#[derive(Debug)]
pub struct Ship {
    pub id: ShipId,
    pub position: ShipPosition,
    pub name: ShipName,
}

impl Ship {
    pub fn is_sunk(&self, grid: &crate::app::grid::Grid) -> bool {
        self.position.cells.iter().all(|cell| {
            grid.cells[cell.y as usize][cell.x as usize].state == crate::app::cell::CellState::Hit
        })
    }
    pub fn new(name: ShipName, grid_width: u8, grid_height: u8) -> Self {
        let position = generate_ship_position(name, grid_width, grid_height);
        Self { name, position }
    }
}

fn generate_ship_position(name: ShipName, grid_width: u8, grid_height: u8) -> ShipPosition {
    let mut rng = rand::rng();
    let horizontal = rng.random_bool(0.5);
    let len = match name {
        ShipName::Carrier => 5,
        ShipName::Battleship => 4,
        ShipName::Cruiser | ShipName::Submarine => 3,
        ShipName::Destroyer => 2,
    } as u8;

    let (max_x, max_y) = if horizontal {
        (grid_width - len, grid_height - 1)
    } else {
        (grid_width - 1, grid_height - len)
    };

    let start_x = rng.random_range(0..=max_x);
    let start_y = rng.random_range(0..=max_y);

    let cells = (0..len)
        .map(|i| {
            if horizontal {
                Cell {
                    x: start_x + i,
                    y: start_y,
                    state: CellState::Occupied,
                }
            } else {
                Cell {
                    x: start_x,
                    y: start_y + i,
                    state: CellState::Occupied,
                }
            }
        })
        .collect();

    ShipPosition { cells }
}

#[derive(Debug)]
pub struct ShipPosition {
    pub cells: Vec<Cell>,
}

#[derive(Debug)]
pub enum StrikeReport {
    Hit,
    Miss,
    Sunk(ShipName),
}

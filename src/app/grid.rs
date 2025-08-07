use super::cell::{Cell, CellState};

#[derive(Debug)]
pub struct Grid {
    pub cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            // Need to lay out all the actual cells in the grid
            // None of this empty vector nonsense
            // file should vary from 'A' to 'J'
            // rank should vary from 1 to 10
            cells: (0u8..10u8)
                .map(|y| {
                    (0u8..10u8)
                        .map(|x| Cell {
                            x,
                            y,
                            state: CellState::default(),
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

#[derive(Debug)]
pub enum StrikeReport {
    Hit(Cell),
    Miss(Cell),
    Sunk,
}

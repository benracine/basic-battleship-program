use crate::app::cell::Cell;
use crate::app::ranges::{ColIndex, RowIndex};

#[derive(Debug)]
pub struct Grid {
    pub cells: [[Cell; 10]; 10],
}

impl Grid {
    pub fn new() -> Self {
        let cells: [[Cell; 10]; 10] = RowIndex::iter()
            .map(|row| {
                ColIndex::iter()
                    .map(|col| Cell::new(row, col))
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("Expected exactly 10 columns")
            })
            .collect::<Vec<_>>()
            .try_into()
            .expect("Expected exactly 10 rows");
        Self { cells }
    }
}

#[derive(Debug)]
pub enum StrikeReport {
    Hit(Cell),
    Miss(Cell),
    Sunk,
}

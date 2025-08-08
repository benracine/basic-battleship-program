use rand;

use crate::app::cell::{Cell, CellStatus};
use crate::app::ranges::{ColIndex, RowIndex};
use crate::app::ship::{Direction, Ship};

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

    pub fn place_ship(&mut self, ship: &Ship) {
        let length = &ship.size();
        let n_rows = self.cells.len();
        let n_cols = self.cells[0].len();

        loop {
            let mut start_row: u8 = rand::random::<u8>() % n_rows as u8;
            let mut start_col: u8 = rand::random::<u8>() % n_cols as u8;
            let fits = self.check_fit(ship, start_row, start_col, length, &ship.direction);
            if (fits) {
                // self.place_ship(ship);
                break;
            }
        }
    }

    pub fn check_fit(
        &self,
        ship: &Ship,
        start_row: u8,
        start_col: u8,
        length: &usize,
        direction: Direction,
    ) -> bool {
        let mut fits = true;
        for i in 0..*length {
            let row = match direction {
                Direction::Horizontal => start_row,
                Direction::Vertical => start_row + i as u8,
            };
            let col = match direction {
                Direction::Horizontal => start_col + i as u8,
                Direction::Vertical => start_col,
            };

            if row >= 10
                || col >= 10
                || self.cells[row as usize][col as usize].status != CellStatus::Empty
            {
                fits = false;
                break;
            }
        }
        fits
    }
}

use crate::app::ranges::{ColIndex, RowIndex};
use crate::app::ship::ShipId;

#[derive(Debug)]
pub struct Cell {
    pub x: RowIndex,
    pub y: ColIndex,
    pub status: CellStatus,
}

impl Cell {
    pub fn new(x: RowIndex, y: ColIndex) -> Self {
        Self {
            x,
            y,
            status: CellStatus::Empty,
        }
    }

    pub fn set_status(&mut self, status: CellStatus) {
        self.status = status;
    }
}

#[derive(Debug, Default, Clone, Copy)]
enum CellStatus {
    #[default]
    Empty,
    Occupied(ShipId),
    Hit(ShipId),
    Miss,
}

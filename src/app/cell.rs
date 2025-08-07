use crate::app::ship::Ship;

#[derive(Debug)]
pub struct Cell {
    pub x: u8,
    pub y: u8,
    pub state: CellState,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum CellState {
    #[default]
    Empty,
    Occupied(Ship),
    Hit,
    Miss,
}

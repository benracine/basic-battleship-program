#[derive(Debug)]
pub struct Cell {
    pub file: char,
    pub rank: u8,
    pub state: CellState,
}

#[derive(Default, Debug)]
pub enum CellState {
    #[default]
    Empty,
    Occupied,
    Hit,
    Miss,
}

#[derive(Debug)]
pub struct OceanGrid {
    pub cells: Vec<Vec<Cell>>,
}

impl OceanGrid {
    pub fn new() -> Self {
        Self {
            cells: vec![vec![]],
        }
    }
}

#[derive(Debug)]
pub enum StrikeReport {
    Hit(Cell),
    Miss(Cell),
    Sunk,
}

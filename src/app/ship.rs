use strum_macros::EnumIter;

#[derive(Debug)]
pub struct Ship {
    pub position: Option<ShipPosition>,
    pub name: ShipName,
}

impl Ship {
    pub fn new(name: ShipName) -> Self {
        Self {
            name,
            position: None,
        }
    }

    pub fn length(&self) -> usize {
        match self.name {
            ShipName::Carrier => 5,
            ShipName::Battleship => 4,
            ShipName::Cruiser => 3,
            ShipName::Submarine => 3,
            ShipName::Destroyer => 2,
        }
    }
}

#[derive(Debug)]
pub struct ShipPosition {
    pub cells: Vec<super::grid::Cell>,
}

#[derive(EnumIter, Debug, Clone, Copy)]
pub enum ShipName {
    Carrier,
    Battleship,
    Cruiser,
    Submarine,
    Destroyer,
}

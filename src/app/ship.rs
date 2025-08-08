use strum_macros::EnumIter;
use uuid::Uuid;

#[derive(EnumIter, Debug, Clone, Copy)]
pub enum ShipName {
    Carrier,
    Battleship,
    Cruiser,
    Submarine,
    Destroyer,
}

impl ShipName {
    pub fn len(&self) -> usize {
        match self {
            ShipName::Carrier => 5,
            ShipName::Battleship => 10,
            ShipName::Cruiser => 8,
            ShipName::Submarine => 6,
            ShipName::Destroyer => 7,
        }
    }
}

#[derive(Debug, Default)]
pub enum Direction {
    #[default]
    Horizontal,
    Vertical,
}

pub type ShipId = Uuid;

#[derive(Debug)]
pub struct Ship {
    pub id: ShipId,
    pub name: ShipName,
    pub direction: Direction,
}

impl Ship {
    pub fn new(name: ShipName, direction: Direction) -> Self {
        Self {
            name: name,
            direction: direction,
            id: Uuid::new_v4(),
        }
    }
}

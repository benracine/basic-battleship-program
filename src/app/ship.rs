use strum_macros::EnumIter;
use uuid::Uuid;

pub type ShipId = Uuid;

#[derive(EnumIter, Debug, Clone, Copy)]
pub enum ShipName {
    Battleship,
    Carrier,
    Cruiser,
    Destroyer,
    Submarine,
}

impl ShipName {
    pub const fn len(self) -> usize {
        match self {
            Self::Carrier => 5,
            Self::Battleship => 4,
            Self::Cruiser => 3,
            Self::Submarine => 3,
            Self::Destroyer => 2,
        }
    }
}

#[derive(Debug, Default)]
pub enum Direction {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Debug)]
pub struct Ship {
    pub id: ShipId,
    pub name: ShipName,
    pub direction: Direction,
}

impl Ship {
    pub fn new(name: ShipName, direction: Direction) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            direction,
        }
    }

    pub fn size(&self) -> usize {
        self.name.len()
    }
}

use super::ship::{Ship, ShipName};
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub ships: Vec<Ship>,
}

impl Player {
    pub fn new(name: &str) -> Self {
        let ships = ShipName::iter().map(Ship::new).collect();
        Self {
            name: name.to_string(),
            ships,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RowIndex(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColIndex(u8);

impl RowIndex {
    pub const fn new(val: u8) -> Option<Self> {
        if val <= 9 { Some(Self(val)) } else { None }
    }

    pub const fn get(self) -> u8 {
        self.0
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        (0..10).map(RowIndex)
    }
}

impl ColIndex {
    pub const fn new(val: u8) -> Option<Self> {
        if val <= 9 { Some(Self(val)) } else { None }
    }

    pub const fn get(self) -> u8 {
        self.0
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        (0..10).map(ColIndex)
    }
}

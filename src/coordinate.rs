use crate::{masks::offset_board, player::Player};

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Coordinate(u8);

impl Coordinate {
    pub fn new_index(index: u8) -> Coordinate {
        Coordinate(index)
    }

    pub fn new(ring: u8, cell: u8) -> Coordinate {
        Coordinate(ring * 8 + cell)
    }

    pub fn as_mask(&self, player: Player) -> u64 {
        (1 << self.0) << offset_board(player)
    }

    pub fn ring(&self) -> u8 {
        self.0 / 8
    }

    pub fn cell(&self) -> u8 {
        self.0 % 8
    }
}

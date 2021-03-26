use crate::{masks::offset_board, player::Player};

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Coordinate(u64);

impl Coordinate {
    pub fn new_index(index: u64) -> Coordinate {
        Coordinate(index)
    }

    pub fn new(ring: u64, cell: u64) -> Coordinate {
        Coordinate(ring * 8 + cell)
    }

    pub fn as_mask(&self, player: Player) -> u64 {
        (1 << self.0) << offset_board(player)
    }
}

use crate::{masks::{OFFSET_BLACK, OFFSET_WHITE}, player::Player};

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Coordinate(u64);

impl Coordinate {
    pub fn new_index(index: u64) -> Coordinate {
        Coordinate(index)
    }

    pub fn new(ring: u64, cell: u64) -> Coordinate {
        Coordinate(ring * 8 + cell)
    }

    pub fn as_player(&self, player: &Player) -> u64 {
        match player {
            Player::White => self.as_white(),
            Player::Black => self.as_black(),
        }
    }

    fn as_mask(&self) -> u64 {
        1 << self.0
    }

    pub fn as_white(&self) -> u64 {
        self.as_mask() << OFFSET_WHITE
    }

    pub fn as_black(&self) -> u64 {
        self.as_mask() << OFFSET_BLACK
    }
}
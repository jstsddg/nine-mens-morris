use std::fmt::Display;

use crate::{masks::offset_board, player::Player};

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Coordinate(u8);

impl From<u8> for Coordinate {
    fn from(from: u8) -> Self {
        Coordinate(from)
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coordinate({}, {})", self.ring(), self.cell())
    }
}

impl Coordinate {
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

    pub fn neighbours(&self) -> Vec<Coordinate> {
        let mut vec = vec![
            Coordinate::new(self.ring(), (self.cell()+8-1) %8),
            Coordinate::new(self.ring(), (self.cell()+1) %8),
        ];
        if (self.cell() % 2) == 1 {
            if self.ring() > 0 {
                vec.push(Coordinate::new(self.ring()-1, self.cell()))
            }
            if self.ring() < 2 {
                vec.push(Coordinate::new(self.ring()+1, self.cell()))
            }
        }
        vec
    }
}

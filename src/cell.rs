use std::fmt::Display;

use crate::player::Player;

#[derive(PartialEq)]
pub enum Cell {
    White,
    Black,
    Empty
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.into())
    }
}

impl Into<&str> for &Cell {
    fn into(self) -> &'static str {
        match self {
            Cell::White => "w",
            Cell::Black => "b",
            Cell::Empty => " ",
        }
    }
}

impl Into<Player> for Cell {
    fn into(self) -> Player {
        match self {
            Cell::White => Player::White,
            Cell::Black => Player::Black,
            _ => panic!("Unable to convert '{}' into Player", self),
        }
    }
}

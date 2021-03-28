use std::fmt::Display;

use crate::cell::Cell;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Player {
    White,
    Black
}

impl Player {
    pub fn opponent(&self) -> Player {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
}

impl Into<Cell> for Player {
    fn into(self) -> Cell {
        match self {
            Player::White => Cell::White,
            Player::Black => Cell::Black,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::White => write!(f, "w"),
            Player::Black => write!(f, "b"),
        }
    }
}

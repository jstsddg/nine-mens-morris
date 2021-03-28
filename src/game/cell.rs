use std::fmt::Display;

use super::{coordinate::Coordinate, player::Player, state::State};

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

impl State {
    pub fn get(&self, coordinate: &Coordinate) -> Cell {
        if self.0 & coordinate.as_mask(Player::White) != 0 {
            Cell::White
        }
        else if self.0 & coordinate.as_mask(Player::Black) != 0 {
            Cell::Black
        }
        else {
            Cell::Empty
        }
    }

    pub fn get_cells(&self, cell: Cell) -> Vec<Coordinate> {
        (0..24)
            .map(|index| Coordinate::from(index))
            .filter(|coordinate| self.get(coordinate) == cell)
            .collect()
    }

    pub fn get_neighbours(&self, coordinate: &Coordinate, cell: Cell) -> Vec<Coordinate> {
        coordinate.neighbours().into_iter()
            .filter(|c| self.get(c) == cell)
            .collect()
    }

    pub fn place(&mut self, coordinate: &Coordinate, cell: Cell) {
        match cell {
            Cell::White => self.0 |= coordinate.as_mask(Player::White),
            _ => self.0 &= !coordinate.as_mask(Player::White),
        }
        match cell {
            Cell::Black => self.0 |= coordinate.as_mask(Player::Black),
            _ => self.0 &= !coordinate.as_mask(Player::Black),
        }
    }

    pub fn move_to(&mut self, from: &Coordinate, to: &Coordinate, player: Player) {
        self.place(from, Cell::Empty);
        self.place(to, player.into());
    }
}

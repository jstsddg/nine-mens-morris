use std::{collections::HashSet, fmt::Display};

use crate::{cell::Cell, coordinate::Coordinate, masks::{BITS_BOARD, BITS_STASH, MASK_BOARD, MASK_BOARD_BLACK, MASK_BOARD_WHITE, MASK_STASH_BLACK, MASK_STASH_WHITE}, mill::Mill, player::Player};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State(u64);

impl Into<u64> for &State {
    fn into(self) -> u64 {
        self.0
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut blueprint = String::from("
            A-----B-----C  white: 1 [3]
            | I---J---K |  black: 2 [4]
            | | Q-R-S | |
            H-P-X   T-L-D
            | | W-V-U | |
            | O---N---M |
            G-----F-----E
        ");
        for i in 0..24 {
            blueprint = blueprint.replace(
                (0x41 as u8 + i) as char,
                (&self.get(&Coordinate::new_index(i.into()))).into()
            );
        }
        blueprint = blueprint.replace("1", &self.get_stash(Player::White).to_string()[..]);
        blueprint = blueprint.replace("2", &self.get_stash(Player::Black).to_string()[..]);
        blueprint = blueprint.replace("3", &self.count_stones(Player::White).to_string()[..]);
        blueprint = blueprint.replace("4", &self.count_stones(Player::Black).to_string()[..]);
        write!(f, "{}{:#64b}", blueprint, self.0)
    }
}

impl State {
    pub fn new() -> State {
        let mut state = State(0);
        state.set_stash(9, Player::White).set_stash(9, Player::Black);
        state
    }

    pub fn set_stash(&mut self, stones: u64, player: Player) -> &mut Self {
        match player {
            Player::White => {
                self.0 &= MASK_STASH_BLACK | MASK_BOARD;
                self.0 |= stones << 2*BITS_BOARD + BITS_STASH;
            },
            Player::Black => {
                self.0 &= MASK_STASH_WHITE | MASK_BOARD;
                self.0 |= stones << 2*BITS_BOARD;
            },
        }
        self
    }

    pub fn get_stash(&self, player: Player) -> u64 {
        match player {
            Player::White => (self.0 & MASK_STASH_WHITE) >> 2*BITS_BOARD + BITS_STASH,
            Player::Black => (self.0 & MASK_STASH_BLACK) >> 2*BITS_BOARD,
        }
    }

    pub fn decrement_stash(&mut self, player: Player) -> &mut Self {
        match player {
            Player::White => self.0 -= 1 << (2*BITS_BOARD+BITS_STASH),
            Player::Black => self.0 -= 1 << (2*BITS_BOARD),
        }
        self
    }

    pub fn has_stash(&self, player: Player) -> bool {
        match player {
            Player::White => (self.0 & MASK_STASH_WHITE) != 0,
            Player::Black => (self.0 & MASK_STASH_BLACK) != 0,
        }
    }

    pub fn get(&self, coordinate: &Coordinate) -> Cell {
        if self.0 & coordinate.as_white() != 0 {
            Cell::White
        }
        else if self.0 & coordinate.as_black() != 0 {
            Cell::Black
        }
        else {
            Cell::Empty
        }
    }

    pub fn place(&mut self, coordinate: &Coordinate, cell: Cell) -> &mut Self {
        match cell {
            Cell::White => self.0 |= coordinate.as_white(),
            _ => self.0 &= !coordinate.as_white(),
        }
        match cell {
            Cell::Black => self.0 |= coordinate.as_black(),
            _ => self.0 &= !coordinate.as_black(),
        }
        self
    }

    pub fn switch(&mut self, from: &Coordinate, to: &Coordinate) -> &mut Self {
        let source = self.get(from);
        let destination = self.get(to);
        self.place(from, destination)
            .place(to, source)
    }

    pub fn count_stones(&self, player: Player) -> u32 {
        match player {
            Player::White => (self.0 & MASK_BOARD_WHITE).count_ones(),
            Player::Black => (self.0 & MASK_BOARD_BLACK).count_ones(),
        }
    }

    pub fn get_cells(&self, cell: Cell) -> Vec<Coordinate> {
        (0..24)
            .map(|i| Coordinate::new_index(i))
            .filter(|c| self.get(c) == cell)
            .collect()
    }

    pub fn can_jump(&self, player: Player) -> bool {
        self.count_stones(player) <= 3
    }

    pub fn has_enough_stones(&self, player: Player) -> bool {
        self.has_stash(player) || self.count_stones(player) >= 3
    }

    pub fn has_mill(&self, player: Player, mill: &Mill) -> bool {
        (self.0 & mill.as_player(&player)) ^ mill.as_player(&player) == 0
    }

    pub fn get_mills(&self, player: Player) -> Vec<Mill> {
        (0..16)
            .map(|i| Mill::new(i))
            .filter(|m| self.has_mill(player, &m))
            .collect()
    }

    pub fn get_mills_cells(&self, player: Player) -> Vec<Coordinate> {
        self.get_mills(player).iter()
            .map(|m| m.get_coordinates())
            .flatten()
            .collect()
    }

    pub fn pound_stones(&self, opponent: Player, count: usize) -> HashSet<State> {
        if count <= 0 {
            return vec![self.clone()].into_iter().collect();
        }

        let stones_placed: HashSet<Coordinate> = self.get_cells(opponent.into()).into_iter().collect();
        let stones_mills:  HashSet<Coordinate> = self.get_mills_cells(opponent).into_iter().collect();
        
        let stones_difference = stones_placed.difference(&stones_mills);

        stones_difference.into_iter()
            .map(|c| -> State {
                let mut s = self.clone();
                s.place(c, Cell::Empty);
                s
            })
            .map(|c| c.pound_stones(opponent, count - 1))
            .flatten()
            .collect()
    }
}
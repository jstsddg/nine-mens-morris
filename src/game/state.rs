use std::{fmt::Display};

use super::{masks::{mask_board, mask_stash, offset_stash}, player::Player, coordinate::Coordinate};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State(pub(super) u64);

impl Into<u64> for &State {
    fn into(self) -> u64 {
        self.0
    }
}

impl From<u64> for State {
    fn from(from: u64) -> Self {
        State(from)
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut blueprint = String::from("
            A-----B-----C  white: *1
            | I---J---K |  black: *2
            | | Q-R-S | |
            H-P-X   T-L-D
            | | W-V-U | |
            | O---N---M |
            G-----F-----E
        ");
        for index in 0..24 {
            blueprint = blueprint.replace(
                (0x41 as u8 + index) as char,
                (&self.get(&Coordinate::from(index))).into()
            );
        }
        blueprint = blueprint.replace("*1", &self.get_stash(Player::White).to_string());
        blueprint = blueprint.replace("*2", &self.get_stash(Player::Black).to_string());
        write!(f, "{}", blueprint)
    }
}

impl State {
    pub fn new() -> State {
        let mut state = State(0);
        state.set_stash(9, Player::White);
        state.set_stash(9, Player::Black);
        state
    }

    pub fn set_stash(&mut self, stones: u8, player: Player) {
        self.0 &= !mask_stash(player);
        self.0 |= (stones as u64) << offset_stash(player);
    }

    pub fn get_stash(&self, player: Player) -> u8 {
        ((self.0 & mask_stash(player)) >> offset_stash(player)) as u8
    }

    pub fn decrement_stash(&mut self, player: Player) {
        self.0 -= 1 << offset_stash(player);
    }

    pub fn has_stash(&self, player: Player) -> bool {
        (self.0 & mask_stash(player)) != 0
    }

    pub fn count_stones(&self, player: Player) -> u8 {
        (self.0 & mask_board(player)).count_ones() as u8
    }

    pub fn has_enough_stones(&self, player: Player) -> bool {
        self.has_stash(player) || self.count_stones(player) >= 3
    }
}

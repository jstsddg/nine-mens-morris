use std::{fmt::Display};

use crate::{coordinate::Coordinate, masks::{mask_board, mask_stash, offset_stash}, player::Player};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State(pub u64);

impl Into<u64> for &State {
    fn into(self) -> u64 {
        self.0
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut blueprint = String::from("
            A-----B-----C  white: 1 (3)
            | I---J---K |  black: 2 (4)
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
        blueprint = blueprint.replace("1", &self.get_stash(Player::White).to_string());
        blueprint = blueprint.replace("2", &self.get_stash(Player::Black).to_string());
        blueprint = blueprint.replace("3", &self.phase(Player::White).to_string());
        blueprint = blueprint.replace("4", &self.phase(Player::Black).to_string());
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
        self.0 &= !mask_stash(player);
        self.0 |= stones << offset_stash(player);
        self
    }

    pub fn get_stash(&self, player: Player) -> u64 {
        (self.0 & mask_stash(player)) >> offset_stash(player)
    }

    pub fn decrement_stash(&mut self, player: Player) -> &mut Self {
        self.0 -= 1 << offset_stash(player);
        self
    }

    pub fn has_stash(&self, player: Player) -> bool {
        (self.0 & mask_stash(player)) != 0
    }

    pub fn count_stones(&self, player: Player) -> u64 {
        (self.0 & mask_board(player)).count_ones().into()
    }

    pub fn has_enough_stones(&self, player: Player) -> bool {
        self.has_stash(player) || self.count_stones(player) >= 3
    }
}

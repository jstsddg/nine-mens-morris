use std::collections::HashSet;

use crate::{cell::Cell, coordinate::Coordinate, masks::{MASK_MILLS, mask_board, offset_board}, player::Player, state::State};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Mill(usize);

impl Mill {
    pub fn new(index: usize) -> Mill {
        Mill(index)
    }

    pub fn as_mask(&self, player: Player) -> u64 {
        MASK_MILLS[self.0] << offset_board(player)
    }

    pub fn get_coordinates(&self) -> Vec<Coordinate> {
        (0..24)
            .filter(|i| self.as_mask(Player::Black) & (1 << i) != 0)
            .map(|i| Coordinate::new_index(i))
            .take(3)
            .collect()
    }
}

impl State {
    pub fn has_mill(&self, player: Player, mill: &Mill) -> bool {
        (self.0 & mill.as_mask(player)) ^ mill.as_mask(player) == 0
    }

    pub fn get_mills(&self, player: Player) -> Vec<Mill> {
        (0..16)
            .map(|index| Mill::new(index))
            .filter(|mill| self.has_mill(player, &mill))
            .collect()
    }

    pub fn get_poundable_stones(&self, player: Player) -> Vec<Coordinate> {
        let mut bits = self.0 & mask_board(player);
        // Remove all mills from the board
        for mill in self.get_mills(player) {
            bits ^= mill.as_mask(player);
        }

        // Check if any stones are left
        if bits == 0 {
            bits = self.0 & mask_board(player);
        }

        (0..24)
            .map(|index| Coordinate::new_index(index))
            .filter(|coordinate| (coordinate.as_mask(player) & bits) != 0)
            .collect()
    }

    pub fn pound_stones(&self, opponent: Player, count: usize) -> HashSet<State> {
        if count <= 0 {
            return vec![self.clone()].into_iter().collect();
        }

        self.get_poundable_stones(opponent).into_iter()
            .map(|coordinate| -> State {
                let mut state = self.clone();
                state.place(&coordinate, Cell::Empty);
                state
            })
            .map(|state| state.pound_stones(opponent, count - 1))
            .flatten()
            .collect()
    }

    /// Pound a stone for every new mill from player
    pub fn pound_mills(&self, player: Player, mills_before: &Vec<Mill>) -> HashSet<State> {
        let mills = self.get_mills(player);
        let mills: HashSet<_> = mills.iter().collect();
        let mills_before: HashSet<_> = mills_before.into_iter().collect();

        let count = mills.difference(&mills_before).count();
        self.pound_stones(player.opponent(), count)
    }
}

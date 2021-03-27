use std::collections::HashSet;

use crate::{cell::Cell, coordinate::Coordinate, masks::{MASK_MILLS, offset_board}, player::Player, state::State};

#[derive(Debug)]
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

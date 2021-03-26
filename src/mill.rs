use crate::{coordinate::Coordinate, masks::{MASK_MILLS, offset_board}, player::Player};

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

use crate::{coordinate::Coordinate, masks::{MASK_MILLS, OFFSET_BLACK, OFFSET_WHITE}, player::Player};

#[derive(Debug)]
pub struct Mill(usize);

impl Mill {
    pub fn new(index: usize) -> Mill {
        Mill(index)
    }

    pub fn as_mask(&self) -> u64 {
        MASK_MILLS[self.0]
    }

    pub fn as_player(&self, player: &Player) -> u64 {
        match player {
            Player::White => self.as_mask() << OFFSET_WHITE,
            Player::Black => self.as_mask() << OFFSET_BLACK,
        }
    }

    pub fn get_coordinates(&self) -> Vec<Coordinate> {
        (0..24)
            .filter(|i| self.as_mask() & (1 << i) != 0)
            .map(|i| Coordinate::new_index(i))
            .collect()
    }
}
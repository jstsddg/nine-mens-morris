use std::fmt::Display;

use crate::{player::Player, state::State};


pub enum Phase {
    Placing,
    Moving,
    Flying,
}

impl Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Phase::Placing => write!(f, "placing"),
            Phase::Moving => write!(f, "moving"),
            Phase::Flying => write!(f, "flying"),
        }
    }
}

impl State {
    pub fn phase(&self, player: Player) -> Phase {
        if self.has_stash(player) {
            Phase::Placing
        }
        else if self.count_stones(player) <= 3 {
            Phase::Flying
        }
        else {
            Phase::Moving
        }
    }
}

use crate::{player::Player, state::State};


impl State {
    pub fn heuristic(&self, _player: Player) -> f64 {
        0.0
    }    
}

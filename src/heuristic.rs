use crate::{player::Player, state::State};


impl State {
    pub fn heuristic(&self, _player: Player) -> f32 {
        0.0
    }    
}

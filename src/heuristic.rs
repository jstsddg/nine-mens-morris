use crate::{player::Player, state::State};


impl State {
    pub fn heuristic(&self, player: Player) -> f32 {
        let mut score = self.count_stones(player) + self.get_stash(player);
        score -= self.count_stones(player.opponent()) + self.get_stash(player.opponent());
        
        f32::from(score) / 10.0
    }    
}

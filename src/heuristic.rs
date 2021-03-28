use crate::{player::Player, state::State};


impl State {
    pub fn heuristic(&self, player: Player) -> i16 {
        let score_p: i16 = (self.count_stones(player) + self.get_stash(player)).into();
        let score_o: i16 = (self.count_stones(player.opponent()) + self.get_stash(player.opponent())).into();
        
        score_p - score_o
    }    
}

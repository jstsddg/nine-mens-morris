use super::{player::Player, state::State};

#[derive(Debug, Clone)]
pub struct HeuristicWeights {
    pub stones: u8,
    pub stash: u8,
    pub mills: u8,
    pub uncompleted_mills: u8,
}

impl Default for HeuristicWeights{
    fn default() -> Self {
        HeuristicWeights {
            stones: 2,
            stash: 2,
            mills: 2,
            uncompleted_mills: 1,
        }
    }
}

impl State {
    pub fn heuristic(&self, player: Player, options: &HeuristicWeights) -> i16 {
        let score_player: i16 = (
            options.stones * self.count_stones(player) +
            options.stash * self.get_stash(player) +
            options.mills * self.get_mills(player).len() as u8 +
            options.uncompleted_mills * self.get_uncompleted_mills(player).len() as u8
        ).into();
        let score_opponent: i16 = (
            options.stones * self.count_stones(player.opponent()) +
            options.stash * self.get_stash(player.opponent()) +
            options.mills * self.get_mills(player.opponent()).len() as u8 +
            options.uncompleted_mills * self.get_uncompleted_mills(player.opponent()).len() as u8
        ).into();
        
        score_player - score_opponent
    }    
}

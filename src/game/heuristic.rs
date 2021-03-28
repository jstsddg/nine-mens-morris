use super::{player::Player, state::State};

#[derive(Debug, Clone)]
pub struct HeuristicWeights {
    pub stones: u8,
    pub stash: u8,
}

impl Default for HeuristicWeights{
    fn default() -> Self {
        HeuristicWeights {
            stones: 1,
            stash: 1,
        }
    }
}

impl State {
    pub fn heuristic(&self, player: Player, options: &HeuristicWeights) -> i16 {
        let score_p: i16 = (
            options.stones * self.count_stones(player) +
            options.stash * self.get_stash(player)
        ).into();
        let score_o: i16 = (
            options.stones * self.count_stones(player.opponent()) +
            options.stash * self.get_stash(player.opponent())
        ).into();
        
        score_p - score_o
    }    
}

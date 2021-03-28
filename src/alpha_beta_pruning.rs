use crate::{heuristic::HeuristicWeights, player::Player, state::State};


#[derive(Debug)]
pub struct AlphaBetaPruningResult {
    states: Vec<State>,
    value: i16,
    visited: u32,
}

#[derive(Debug, Clone)]
pub struct AlphaBetaPruningOptions {
    pub limit: u8,
    pub weights: HeuristicWeights,
}

impl Default for AlphaBetaPruningOptions {
    fn default() -> Self {
        AlphaBetaPruningOptions {
            limit: 3,
            weights: Default::default(),
        }
    }
}

pub struct AlphaBetaPruning {
    visited: u32,
    options: AlphaBetaPruningOptions,
}

impl AlphaBetaPruning {
    pub fn new(options: AlphaBetaPruningOptions) -> Self {
        AlphaBetaPruning {
            visited: 0,
            options: options
        }
    }

    fn value(&mut self, state: &State, player: Player, mut alpha: i16, beta: i16, limit: u8) -> i16 {
        self.visited += 1;
        
        if state.finished(player) {
            return state.utility(player);
        }
        if limit == 0 {
            return state.heuristic(player, &self.options.weights);
        }

        let mut value = alpha;
        for next_state in state.next_states(player) {
            value = value.max(
                -self.value(&next_state, player.opponent(), -beta, -alpha, limit - 1)
            );

            if value >= beta {
                return value;
            }
            alpha = value.max(alpha);
        }

        value
    }

    pub fn alpha_beta_pruning(&mut self, state: State, player: Player) -> AlphaBetaPruningResult {
        self.visited = 0;

        let values: Vec<(i16, State)> = state.next_states(player).into_iter()
            .map(|s| (self.value(&s, player, -100, 100, self.options.limit), s))
            .collect();
        let max = values.iter()
            .map(|value| value.0)
            .max().expect("Unable to find maximum");
        let states = values.into_iter()
            .filter(|value| value.0 == max)
            .map(|value| value.1)
            .collect();

        AlphaBetaPruningResult {
            states: states,
            visited: self.visited,
            value: max,
        }
    }
}

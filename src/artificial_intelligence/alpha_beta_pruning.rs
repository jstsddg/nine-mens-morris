use std::collections::HashMap;

use crate::game::{heuristic::HeuristicWeights, player::Player, state::State};

#[derive(Debug)]
pub struct AlphaBetaPruningResult {
    states: Vec<State>,
    value: i16,
    visited: u32,
}

#[derive(Debug, Clone)]
pub struct AlphaBetaPruningOptions {
    pub cache: bool,
    pub limit: u8,
    pub weights: HeuristicWeights,
}

impl Default for AlphaBetaPruningOptions {
    fn default() -> Self {
        AlphaBetaPruningOptions {
            cache: true,
            limit: 3,
            weights: Default::default(),
        }
    }
}

pub struct AlphaBetaPruning {
    cache: HashMap<(State, Player, u8), (i16, i16, i16)>,
    visited: u32,
    options: AlphaBetaPruningOptions,
}

enum Cache {
    Hit(i16),
    Miss(i16, i16),
}

impl AlphaBetaPruning {
    pub fn new(options: AlphaBetaPruningOptions) -> Self {
        AlphaBetaPruning {
            cache: HashMap::new(),
            visited: 0,
            options: options
        }
    }

    fn get_cache(&self, state: &State, player: Player, alpha: i16, beta: i16, limit: u8) -> Cache {
        if !self.options.cache {
            return Cache::Miss(alpha, beta)
        }

        if let Some((value, cache_alpha, cache_beta)) = self.cache.get(&(state.clone(), player, limit)) {
            if cache_alpha <= &alpha && &beta <= cache_beta {
                Cache::Hit(*value)
            } else {
                Cache::Miss(alpha.min(*cache_alpha), beta.max(*cache_beta))
            }
        } else {
            Cache::Miss(alpha, beta)
        }
    }

    fn set_cache(&mut self, state: &State, player: Player, limit: u8, value: i16, alpha: i16, beta: i16) {
        if self.options.cache {
            self.cache.insert((state.clone(), player, limit), (value, alpha, beta));
        }
    }

    fn value(&mut self, state: &State, player: Player, mut alpha: i16, mut beta: i16, limit: u8) -> i16 {
        self.visited += 1;

        match self.get_cache(state, player, alpha, beta, limit) {
            Cache::Hit(value) => return value,
            Cache::Miss(new_alpha, new_beta) => {
                alpha = new_alpha;
                beta = new_beta;
            }
        }
        
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
                self.set_cache(state, player, limit, value, alpha, beta);
                return value;
            }
            alpha = value.max(alpha);
        }

        self.set_cache(state, player, limit, value, alpha, beta);
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
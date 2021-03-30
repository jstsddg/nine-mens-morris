use std::{collections::HashMap};

use crate::game::{heuristic::HeuristicWeights, player::Player, state::State};

use super::{ArtificialIntelligence, ArtificialIntelligenceResult};

#[derive(Debug, Clone, Copy)]
pub struct Counter {
    visisted: u32,
    cache_hit: u32,
    cache_miss: u32,
    move_ordering_hit: u32,
    move_ordering_miss: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            visisted: 0,
            cache_hit: 0,
            cache_miss: 0,
            move_ordering_hit: 0,
            move_ordering_miss: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AlphaBetaPruningOptions {
    pub cache: bool,
    pub limit: u8,
    pub weights: HeuristicWeights,
    pub move_ordering: bool,
    pub move_ordering_offset: u8,
    pub iterative_deepening: bool,
}

impl Default for AlphaBetaPruningOptions {
    fn default() -> Self {
        AlphaBetaPruningOptions {
            cache: true,
            limit: 3,
            weights: Default::default(),
            move_ordering: true,
            move_ordering_offset: 1,
            iterative_deepening: true,
        }
    }
}

pub struct AlphaBetaPruning {
    cache: HashMap<(State, Player, u8), (i16, i16, i16)>,
    counter: Counter,
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
            counter: Counter::new(),
            options: options,
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

    fn order_moves(&mut self, mut next_states: Vec<State>, player: Player, limit: u8) -> Vec<State> {
        if self.options.move_ordering && limit >= self.options.move_ordering_offset {
            next_states.sort_by_cached_key(|state| {
                if let Some(cache) = self.cache.get(&(state.clone(), player.opponent(), limit-self.options.move_ordering_offset)) {
                    self.counter.move_ordering_hit += 1;
                    cache.0
                } else {
                    self.counter.move_ordering_miss += 1;
                    0
                }
            });
        }
        next_states
    }

    fn value(&mut self, state: &State, player: Player, mut alpha: i16, mut beta: i16, limit: u8) -> i16 {
        self.counter.visisted += 1;

        match self.get_cache(state, player, alpha, beta, limit) {
            Cache::Hit(value) => {
                self.counter.cache_hit += 1;
                return value
            },
            Cache::Miss(new_alpha, new_beta) => {
                self.counter.cache_miss += 1;
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
        for next_state in self.order_moves(state.next_states(player), player, limit) {
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
}

impl ArtificialIntelligence for AlphaBetaPruning {
    type Counter = Counter;

    fn best_moves(&mut self, state: State, player: Player) -> ArtificialIntelligenceResult<Counter> {
        self.counter = Counter::new();

        let start = if self.options.iterative_deepening { 0 } else { self.options.limit };

        let next_states = state.next_states(player);

        let values = (start..=self.options.limit).into_iter()
            .map(|limit| -> Vec<(i16, &State)> {
                next_states.iter()
                    .map(|s| (self.value(s, player, -100, 100, limit), s))
                    .collect()
            })
            .last()
            .expect("No value after iterative deepening");
         
        let max = values.iter()
            .map(|value| value.0)
            .max().expect("Unable to find maximum");
        let states = values.into_iter()
            .filter(|value| value.0 == max)
            .map(|value| value.1.clone())
            .collect();

        ArtificialIntelligenceResult {
            states: states,
            value: max,
            counter: self.counter.clone(),
        }
    }
}

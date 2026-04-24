use std::collections::HashMap;

use crate::game::{heuristic::HeuristicWeights, player::Player, state::State};

use super::{ArtificialIntelligence, BestMoves};

#[cfg(feature = "counter")]
#[derive(Debug, Clone, Copy)]
pub struct Counter {
    visisted: u32,
    cache_hit: u32,
    cache_miss: u32,
}

#[cfg(feature = "counter")]
impl Counter {
    fn new() -> Counter {
        Counter {
            visisted: 0,
            cache_hit: 0,
            cache_miss: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MinimaxOptions {
    pub cache: bool,
    pub limit: u8,
    pub weights: HeuristicWeights,
}

impl Default for MinimaxOptions {
    fn default() -> Self {
        MinimaxOptions {
            cache: true,
            limit: 3,
            weights: Default::default(),
        }
    }
}

pub struct MinimaxBestMoves {
    states: Vec<State>,
    value: i16,

    #[cfg(feature = "counter")]
    counter: Counter,
}

impl BestMoves for MinimaxBestMoves {
    fn states(&self) -> Vec<&State> {
        self.states.iter().collect()
    }

    fn value(&self) -> i16 {
        self.value
    }
}

#[cfg(feature = "counter")]
impl MinimaxBestMoves {
    pub fn counter(&self) -> Counter {
        self.counter
    }
}

pub struct Minimax {
    cache: HashMap<(State, Player, u8), i16>,
    options: MinimaxOptions,

    #[cfg(feature = "counter")]
    counter: Counter,
}

impl Minimax {
    pub fn new(options: MinimaxOptions) -> Self {
        Minimax {
            cache: HashMap::new(),
            options: options,

            #[cfg(feature = "counter")]
            counter: Counter::new(),
        }
    }

    fn get_cache(&self, state: &State, player: Player, limit: u8) -> Option<i16> {
        match self.options.cache {
            true => self.cache.get(&(state.clone(), player, limit)).map(|value| *value),
            false => None,
        }
    }

    fn set_cache(&mut self, state: &State, player: Player, limit: u8, value: i16) {
        if self.options.cache {
            self.cache.insert((state.clone(), player, limit), value);
        }
    }

    fn value(&mut self, state: &State, player: Player, limit: u8) -> i16 {
        increment!(self.counter.visisted);
        
        if let Some(cache) = self.get_cache(state, player, limit) {
            increment!(self.counter.cache_hit);
            return cache;
        }
        increment!(self.counter.cache_miss);
        
        if state.finished(player) {
            return state.utility(player);
        }
        if limit == 0 {
            return state.heuristic(player, &self.options.weights);
        }

        let value = state.next_states(player).iter()
            .map(|s| -self.value(s, player.opponent(), limit-1))
            .max()
            .expect("next_state was empty, but finished was false");
        self.set_cache(state, player, limit, value);

        value
    }
}

impl ArtificialIntelligence for Minimax {
    fn best_moves(&mut self, state: State, player: Player) -> Box<dyn BestMoves> {
        #[cfg(feature = "counter")]
        { self.counter = Counter::new(); }

        let values: Vec<(i16, State)> = state.next_states(player).into_iter()
            .map(|s| (self.value(&s, player, self.options.limit), s))
            .collect();
        let max = values.iter()
            .map(|value| value.0)
            .max().expect("Unable to find maximum");
        let states = values.into_iter()
            .filter(|value| value.0 == max)
            .map(|value| value.1)
            .collect();

        Box::new(MinimaxBestMoves {
            states: states,
            value: max,

            #[cfg(feature = "counter")]
            counter: self.counter.clone(),
        })
    }
}

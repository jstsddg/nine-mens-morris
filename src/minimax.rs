use std::collections::HashMap;

use crate::{heuristic::HeuristicWeights, player::Player, state::State};


#[derive(Debug)]
pub struct MinimaxResult {
    states: Vec<State>,
    value: i16,
    visited: u32,
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

struct MinimaxRuntime {
    cache: HashMap<(State, Player, u8), i16>,
    visited: u32,
    options: MinimaxOptions,
}

impl MinimaxRuntime {
    fn new(options: MinimaxOptions) -> Self {
        MinimaxRuntime {
            cache: HashMap::new(),
            visited: 0,
            options: options
        }
    }

    fn get_cache(&self, state: &State, player: Player, limit: u8) -> Option<&i16> {
        match self.options.cache {
            true => self.cache.get(&(state.clone(), player, limit)),
            false => None,
        }
    }

    fn set_cache(&mut self, state: &State, player: Player, limit: u8, value: i16) -> i16 {
        if self.options.cache {
            self.cache.insert((state.clone(), player, limit), value);
        }
        value
    }

    fn value(&mut self, state: &State, player: Player, limit: u8) -> i16 {
        self.visited += 1;
        
        if let Some(cache) = self.get_cache(state, player, limit) {
            return *cache;
        }
        
        if state.finished(player) {
            return state.utility(player);
        }
        if limit == 0 {
            return state.heuristic(player, &self.options.weights);
        }

        let value = state.next_states(player).iter()
            .map(|s| self.value(s, player.opponent(), limit-1))
            .max()
            .expect("next_state was empty, but finished was false");
        self.set_cache(state, player, limit, value)
    }
}

pub fn minimax(state: State, player: Player, options: MinimaxOptions) -> MinimaxResult {
    let mut runtime = MinimaxRuntime::new(options);

    let values: Vec<(i16, State)> = state.next_states(player).into_iter()
        .map(|s| (runtime.value(&s, player, runtime.options.limit), s))
        .collect();
    let max = values.iter()
        .map(|value| value.0)
        .max().expect("Unable to find maximum");
    let states = values.into_iter()
        .filter(|value| value.0 == max)
        .map(|value| value.1)
        .collect();

    MinimaxResult {
        states: states,
        visited: runtime.visited,
        value: max,
    }
}

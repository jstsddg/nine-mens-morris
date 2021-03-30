use crate::game::{player::Player, state::State};

pub mod minimax;
pub mod alpha_beta_pruning;

pub trait ArtificialIntelligence {
    type Counter;

    fn best_moves(&mut self, state: State, player: Player) -> ArtificialIntelligenceResult<Self::Counter>;
}

#[derive(Debug)]
pub struct ArtificialIntelligenceResult<Counter> {
    states: Vec<State>,
    value: i16,
    counter: Counter,
}

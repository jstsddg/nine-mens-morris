use crate::game::{player::Player, state::State};

pub mod minimax;
pub mod alpha_beta_pruning;

pub trait ArtificialIntelligence {
    fn best_moves(&mut self, state: State, player: Player) -> ArtificialIntelligenceResult;
}

#[derive(Debug)]
pub struct ArtificialIntelligenceResult {
    states: Vec<State>,
    visited: u32,
    value: i16,
}

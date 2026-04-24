use crate::game::{player::Player, state::State};

#[cfg(feature = "counter")]
#[macro_export]
macro_rules! increment{
    ($a:expr) => {
        $a += 1
    }
}

#[cfg(not(feature = "counter"))]
#[macro_export]
macro_rules! increment{
    ($a:expr) => {}
}

#[macro_use]
pub mod minimax;
#[macro_use]
pub mod alpha_beta_pruning;

pub trait ArtificialIntelligence {
    fn best_moves(&mut self, state: State, player: Player) -> Box<dyn BestMoves>;
}

pub trait BestMoves {
    fn states(&self) -> Vec<&State>;
    fn value(&self) -> i16;
}

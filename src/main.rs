mod game;
mod artificial_intelligence;

use artificial_intelligence::{ArtificialIntelligence, alpha_beta_pruning::{AlphaBetaPruning, AlphaBetaPruningOptions}, minimax::{Minimax, MinimaxOptions}};
use game::{player::Player, state::State};

fn main() {
    minimax();
    alpha_beta_pruning();
}

fn minimax() {
    let options = MinimaxOptions {
        limit: 5,
        ..Default::default()
    };
    let mut ai = Minimax::new(options.clone());
    println!(
        "{:?}\n{:?}",
        options,
        ai.best_moves(State::new(), Player::White)
    );
}

fn alpha_beta_pruning() {
    let options = AlphaBetaPruningOptions {
        limit: 5,
        ..Default::default()
    };
    let mut ai = AlphaBetaPruning::new(options.clone());
    println!(
        "{:?}\n{:?}",
        options,
        ai.best_moves(State::new(), Player::White)
    );
}

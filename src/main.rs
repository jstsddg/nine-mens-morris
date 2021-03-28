mod game;
mod artificial_intelligence;

use artificial_intelligence::{alpha_beta_pruning::{AlphaBetaPruning, AlphaBetaPruningOptions}, minimax::{Minimax, MinimaxOptions}};
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
    let mut minimax = Minimax::new(options.clone());
    println!(
        "{:?}\n{:?}",
        options,
        minimax.minimax(State::new(), Player::White)
    );
}

fn alpha_beta_pruning() {
    let options = AlphaBetaPruningOptions {
        limit: 5,
        ..Default::default()
    };
    let mut alpha_beta_pruning = AlphaBetaPruning::new(options.clone());
    println!(
        "{:?}\n{:?}",
        options,
        alpha_beta_pruning.alpha_beta_pruning(State::new(), Player::White)
    );
}

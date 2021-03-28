mod cell;
mod coordinate;
mod player;
mod masks;
mod mill;
mod state;
mod phase;
mod heuristic;
mod game;
mod minimax;
mod alpha_beta_pruning;

use alpha_beta_pruning::{AlphaBetaPruning, AlphaBetaPruningOptions};
use minimax::{Minimax, MinimaxOptions};
use player::Player;
use state::State;

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

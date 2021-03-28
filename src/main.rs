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

use minimax::{Minimax, MinimaxOptions};
use player::Player;
use state::State;

fn main() {
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

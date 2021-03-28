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

use minimax::{MinimaxOptions, minimax};
use player::Player;
use state::State;

fn main() {
    let options = MinimaxOptions {
        limit: 5,
        ..Default::default()
    };
    println!(
        "{:?}\n{:?}",
        options,
        minimax(State::new(), Player::White, options.clone())
    );
}

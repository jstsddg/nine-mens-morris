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
        cache: true,
        limit: 3,
    };
    println!(
        "{:?}\n{:?}",
        options,
        minimax(State::new(), Player::White, options.clone())
    );
}

mod cell;
mod coordinate;
mod player;
mod masks;
mod mill;
mod state;
mod phase;
mod heuristic;
mod game;

use cell::Cell;
use coordinate::Coordinate;
use player::Player;
use state::State;

fn main() {
    let mut state = State::new();
    println!("0. State: Start {}", state);
    
    state.place(&Coordinate::new(0,0), Cell::White);
    println!("1. State: Place (0,0)=w {}", state);
    
    state.place(&Coordinate::new_index(23), Cell::White);
    println!("2. State: Place 23=w {}", state);
    
    state.place(&Coordinate::new_index(22), Cell::Black);
    println!("3. State: Place 22=b {}", state);
    
    state.place(&Coordinate::new_index(22), Cell::Black);
    println!("4. State: Place 22=b {}", state);
    
    state.set_stash(9, Player::Black);
    println!("5. State: Stash b=9 {}", state);
    
    state.set_stash(1, Player::White);
    println!("6. State: Stash w=1 {}", state);

    state.decrement_stash(Player::Black);
    println!("7. State: Stash b-- {}", state);

    state.decrement_stash(Player::White);
    state.place(&Coordinate::new_index(7), Cell::White);
    state.place(&Coordinate::new_index(15), Cell::White);
    state.place(&Coordinate::new_index(2), Cell::White);
    println!("8. State: Stash w-- {}", state);
}

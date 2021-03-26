mod cell;
mod coordinate;
mod player;
mod masks;
mod mill;
mod state;

use cell::Cell;
use coordinate::Coordinate;
use masks::{MASK_BOARD, MASK_BOARD_BLACK, MASK_BOARD_WHITE, MASK_STASH, MASK_STASH_BLACK, MASK_STASH_WHITE};
use player::Player;
use state::State;

fn main() {
    println!("MASK_STASH_WHITE: {:#b}", MASK_STASH_WHITE | (1<<63));
    println!("MASK_STASH_BLACK: {:#b}", MASK_STASH_BLACK | (1<<63));
    println!("MASK_STASH:       {:#b}", MASK_STASH       | (1<<63));
    println!("MASK_BOARD_WHITE: {:#b}", MASK_BOARD_WHITE | (1<<63));
    println!("MASK_BOARD_BLACK: {:#b}", MASK_BOARD_BLACK | (1<<63));
    println!("MASK_BOARD:       {:#b}", MASK_BOARD       | (1<<63));

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
    state.place(&Coordinate::new_index(1), Cell::White);
    println!("8. State: Stash w-- {}", state);

    println!("White: {:?}", state.get_cells(Cell::White));
    println!("Black: {:?}", state.get_cells(Cell::Black));
    println!("Mills White: {:?}", state.get_mills(Player::White));
    println!("Mills Black: {:?}", state.get_mills(Player::Black));
}

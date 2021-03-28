use crate::{player::Player, state::State, cell::Cell, phase::Phase};


impl State {
    pub fn next_states(&self, player: Player) -> Vec<State> {
        match self.phase(player) {
            Phase::Placing => {
                let mut clone = self.clone();
                clone.decrement_stash(player);

                let mills_before = clone.get_mills(player);

                clone.get_cells(Cell::Empty).into_iter()
                    .flat_map(|coordinate| {
                        let mut state = clone.clone();
                        state.place(&coordinate, player.into());
                        state.pound_mills(player, &mills_before)
                    })
                    .collect()
            },
            Phase::Moving => {
                let mills_before = self.get_mills(player);

                self.get_cells(player.into()).into_iter()
                    .flat_map(|from| -> Vec<State> {
                        self.get_neighbours(&from, Cell::Empty).into_iter()
                            .flat_map(|to| {
                                let mut state = self.clone();
                                state.move_to(&from, &to, player);
                                state.pound_mills(player, &mills_before)
                            })
                            .collect()
                    })
                    .collect()
            },
            Phase::Flying => {
                let mills_before = self.get_mills(player);
                let cells_empty = self.get_cells(Cell::Empty);

                self.get_cells(player.into()).into_iter()
                    .flat_map(|from| -> Vec<State> {
                        cells_empty.iter()
                            .flat_map(|to| {
                                let mut state = self.clone();
                                state.move_to(&from, &to, player);
                                state.pound_mills(player, &mills_before)
                            })
                            .collect()
                    })
                    .collect()
            },
        }
    }

    pub fn finished(&self, player: Player) -> bool {
        !self.has_enough_stones(player)
        || !self.has_enough_stones(player.opponent())
        || self.next_states(player).is_empty()
    }

    pub fn utility(&self, player: Player) -> i16 {
        if self.has_enough_stones(player) {
            -100
        }
        else if self.has_enough_stones(player.opponent()) {
            100
        }
        else if self.next_states(player).is_empty() {
            -100
        }
        else {
            0
        }
    }
}

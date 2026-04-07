use crate::engine::{direction, prelude::*};
use super::pcg::RandPCG;
use super::ALL_DIRECTIONS;

#[allow(dead_code)]
pub struct Rngesus {
    id: PlayerId,
    pcg: RandPCG,
}

pub fn not_instant_crash_directions(
    id: PlayerId,
    game_state: &GameState,
) -> impl Iterator<Item = Direction> {
    let grid = game_state.current_grid();
    let my_pos = grid.player_head_position(id);

    Direction::all().filter(move |d| {
        my_pos
            .after_moved(*d)
            .filter(|p| p.is_empty(grid))
            .is_some()
    })
}

impl Bot for Rngesus {
    fn new(my_player_id: PlayerId) -> Self {
        Self {id: my_player_id, pcg: Default::default() }
    }
    fn next_action(&mut self, game_state: &GameState) -> Direction {
        let possibilities: Vec<Direction> = not_instant_crash_directions(self.id, game_state).collect();
        let rand = self.pcg.next_u32() as usize;
        if possibilities.is_empty() {
            eprintln!("Rngesus about to crash: {}", self.id);
            ALL_DIRECTIONS[rand % 4]
        } else {
            possibilities[rand % possibilities.len()]
        }
    }
}


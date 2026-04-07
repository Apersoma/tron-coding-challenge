use super::*;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct Lunar {
    id: PlayerId
}

impl Bot for Lunar {
    fn new(my_player_id: PlayerId) -> Self {
        Self {id: my_player_id}
    }
    fn next_action(&mut self, game_state: &GameState) -> Direction {
        todo!()
    }
}


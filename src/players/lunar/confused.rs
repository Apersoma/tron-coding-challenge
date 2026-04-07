use crate::engine::prelude::*;


/// This bot thinks its the other player.
/// Probably one of the worst bots, unless 
/// it is using the human player as its insides.
/// Then nothing changes bc it doesn't care about the id.
pub struct Confused<B: Bot>(B);

impl<B: Bot> Bot for Confused<B> {
    fn new(my_player_id: PlayerId) -> Self {
        Self(B::new(my_player_id.other()))
    }
    fn next_action(&mut self, game_state: &GameState) -> Direction {
        self.0.next_action(game_state)
    }
}
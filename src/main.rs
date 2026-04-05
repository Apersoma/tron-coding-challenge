use crate::engine::prelude::*;

mod engine;
mod players;
mod rand;

fn main() {
    use players::example_bot::ExampleBot;
    // use players::bot_template::BotTemplate;
    use players::human_controlled_bot::HumanControlledBot;
    let mut game: GameEngine<ExampleBot, HumanControlledBot> = GameEngine::new();
    game.run_game();
}

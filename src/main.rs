use crate::engine::prelude::*;

mod engine;
mod players;
use players::lunar::{Lunar, Rngesus};

fn main() {
    use players::example_bot::ExampleBot;
    // use players::bot_template::BotTemplate;
    use players::human_controlled_bot::HumanControlledBot;
    GameEngine::<ExampleBot, Rngesus<3>>::new().run_game();
    // GameEngine::<ExampleBot, HumanControlledBot>::new().run_game();
    GameEngine::<HumanControlledBot, ExampleBot>::new().run_game();
    // GameEngine::<ExampleBot, Rngesus>::new().run_game();
}

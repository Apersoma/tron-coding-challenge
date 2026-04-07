#![allow(unused_imports)]
use crate::{engine::prelude::*, players::{example_bot::ExampleBot, jack_papel_bots::hallucinator::Hallucinator}};
use players::human_controlled_bot::HumanControlledBot;
mod engine;
mod players;
use players::lunar::{Lunar, Rngesus, ScaredyCat};

macro_rules! fight {
    ($a:ty, $b:ty) => {{
        println!("{} vs {}", stringify!($a), stringify!($b));
        GameEngine::<$a, $b>::new().run_game();
    }};
}

fn main() {
    if cfg!(test) {
        fight!(ExampleBot, HumanControlledBot);
        fight!(ExampleBot, Rngesus);
        fight!(ExampleBot, ScaredyCat<true>);
        fight!(ExampleBot, ScaredyCat<false>);
        fight!(ScaredyCat<true>, Rngesus);
        fight!(ScaredyCat<false>, Rngesus);
        fight!(ScaredyCat<false>, Hallucinator);
    }
    
    fight!(ExampleBot, ScaredyCat<true>);
}
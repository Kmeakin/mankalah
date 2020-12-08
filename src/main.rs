use mankalah::{agent::{Agent, AlphaBeta, MiniMax}, heuristics::CurrentScore};
//use clap::{Arg, App, SubCommand};

pub fn main() {
    let mut agent = Agent::new();
    agent.run::<CurrentScore, AlphaBeta>();
}

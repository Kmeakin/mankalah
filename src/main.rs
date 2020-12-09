use clap::{App, Arg};
use mankalah::{
    agent::Agent,
    eval::{AlphaBeta, MiniMax},
    heuristics::Weights,
};
use std::{convert::TryInto, str::FromStr};

fn main() {
    let args = App::new("Mankalah")
        .version("1.0")
        .author("Karl Meakin & Ben Maxwell")
        .arg(
            Arg::with_name("search")
                .long("search")
                .possible_values(&["minimax", "alpha-beta"])
                .default_value("alpha-beta"),
        )
        .arg(
            Arg::with_name("weight")
                .long("weights")
                .number_of_values(6)
                .required(true),
        )
        .arg(Arg::with_name("depth").long("depth").default_value("10"))
        .get_matches();
    let depth: usize = args.value_of("depth").unwrap().parse().unwrap();
    let weights: Vec<f32> = args
        .values_of("weight")
        .unwrap()
        .map(|w| f32::from_str(w).unwrap())
        .collect();
    let weights: Weights = weights.try_into().unwrap();
    let mut agent = Agent::new();
    match args.value_of("search") {
        Some("minimax") => agent.run::<MiniMax>(depth, weights),
        Some("alpha-beta") => agent.run::<AlphaBeta>(depth, weights),
        _ => unreachable!(),
    }
}

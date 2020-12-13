use clap::{App, Arg};
use flexi_logger::Logger;
use mankalah::{
    agent::Agent,
    eval::{AlphaBeta, MiniMax},
    heuristics::Weights,
};
use std::{convert::TryInto, str::FromStr};

fn main() {
    // run with `RUST_LOG=debug cargo run --bin mankalah ...`
    // output is saved to mankalah_YYYY-MM-DD_HH-mm-ss.log
    Logger::with_env().log_to_file().start().unwrap();

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
                .number_of_values(5)
                .required(true),
        )
        .arg(
            Arg::with_name("depth")
                .long("depth")
                .takes_value(true)
                .required(true),
        )
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

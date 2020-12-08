use clap::{App, Arg};
use mankalah::{
    agent::Agent,
    eval::{AlphaBeta, MiniMax},
    heuristics::CurrentScore,
};

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
            Arg::with_name("heuristic")
                .long("heuristic")
                .possible_values(&["current-score"])
                .default_value("current-score"),
        )
        // .arg(Arg::with_name("depth").long("depth").default_value("10"))
        .get_matches();
    // let depth = args.value_of("depth");
    let mut agent = Agent::new();
    match (args.value_of("heuristic"), args.value_of("search")) {
        (Some("current-score"), Some("minimax")) => agent.run::<CurrentScore, MiniMax>(),
        (Some("current-score"), Some("alpha-beta")) => agent.run::<CurrentScore, AlphaBeta>(),
        _ => unreachable!(),
    }
}

#![feature(format_args_capture)]

use std::{fmt, process::Command};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Winner {
    Draw,
    North,
    South,
}

impl fmt::Display for Winner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:?}", self) }
}

#[derive(Debug, Copy, Clone)]
struct BenchmarkData {
    winner: Winner,
    score: i32,
    depth: usize,
    our_time: u32,
    thier_time: u32,
}

fn main() {
    for opp in &[
        "java -jar Test_Agents/error404.jar",
        "java -jar Test_Agents/JimmyPlayer.jar",
        "java -jar Test_Agents/Group2Agent.jar",
    ] {
        for side in &[false] {
            println!(
                "Depth|Winner|Score|Our time|Thier time (we are {}, agaisnt \"{}\")",
                if *side { "NORTH" } else { "SOUTH" },
                opp
            );
            println!("-----|-----|-----|-----|-----");

            for depth in 1..12 {
                let BenchmarkData {
                    score,
                    depth,
                    winner,
                    our_time,
                    thier_time,
                } = benchmark(depth, [1.0, 0.6, 0.0, 0.95, 0.59], opp, *side);
                println!("{depth}|{winner}|{score}|{our_time}|{thier_time}");
            }
        }
    }
}

fn benchmark(depth: usize, weights: [f32; 5], opponent: &str, is_north: bool) -> BenchmarkData {
    let us: &str = &format!(
        "cargo run --release --bin mankalah -- --search=alpha-beta --depth={} --weights {}",
        depth,
        weights
            .iter()
            .map(|w| format!("{:?}", w))
            .collect::<Vec<_>>()
            .join(" ")
    );
    let them = opponent;

    let (south, north) = if is_north { (them, us) } else { (us, them) };
    let output = Command::new("java")
        .arg("-jar")
        .arg("ManKalah.jar")
        .arg(south)
        .arg(north)
        .output()
        .unwrap();

    let stderr = String::from_utf8(output.stderr).unwrap();
    let stderr = stderr.lines().collect::<Vec<_>>();
    let len = stderr.len();

    let winner_score_str = stderr[len - 5];
    let winner_score: i32 = winner_score_str["SCORE: ".len()..].parse().unwrap();

    let winner_str = stderr[len - 6];
    println!("{} -- {}",winner_str, south);
    let (winner, score) = if winner_str.starts_with("DRAW") {
        (Winner::Draw, 0)
    } else if winner_str.starts_with("WINNER: Player 2") {
        (
            Winner::North,
            if is_north {
                winner_score
            } else {
                -winner_score
            },
        )
    } else if winner_str.starts_with("WINNER: Player 1") {
        (
            Winner::South,
            if !is_north {
                winner_score
            } else {
                -winner_score
            },
        )
    } else {
        unreachable!()
    };

    let north_str = stderr[len - 3];
    let south_str = stderr[len - 2];

    let north_words: Vec<_> = north_str.split_whitespace().collect();
    let south_words: Vec<_> = south_str.split_whitespace().collect();
    let north_len = north_words.len();
    let south_len = south_words.len();
    let north_time: u32 = north_words[north_len - 4].parse().unwrap();
    let south_time: u32 = south_words[south_len - 4].parse().unwrap();

    let (our_time, thier_time) = if is_north {
        (north_time, south_time)
    } else {
        (south_time, north_time)
    };

    BenchmarkData {
        winner,
        score,
        depth,
        our_time,
        thier_time,
    }
}

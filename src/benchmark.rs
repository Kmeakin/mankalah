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
}

fn main() {
    println!("Depth|Winner|Score");
    println!("-----|-----|-----");
    for depth in 0..50 {
        let BenchmarkData {
            score,
            depth,
            winner,
        } = benchmark(depth, [1.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        println!("{depth}|{winner}|{score}");
    }
}

fn benchmark(depth: usize, weights: [f32; 6]) -> BenchmarkData {
    let output = Command::new("java")
        .arg("-jar")
        .arg("ManKalah.jar")
        .arg("java -jar MKRefAgent.jar")
        .arg(format!(
            "cargo run --release --bin mankalah -- --search=alpha-beta --depth={} --weights {}",
            depth,
            weights
                .iter()
                .map(|w| format!("{:?}", w))
                .collect::<Vec<_>>()
                .join(" ")
        ))
        .output()
        .unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    let stderr = stderr.lines().collect::<Vec<_>>();
    let len = stderr.len();
    let winner_str = stderr[len - 6];

    let winner = if winner_str.starts_with("DRAW") {
        Winner::Draw
    } else if winner_str.starts_with("WINNER: Player 1") {
        Winner::North
    } else if winner_str.starts_with("WINNER: Player 2") {
        Winner::South
    } else {
        unreachable!()
    };

    let winner_score_str = stderr[len - 5];
    let winner_score: i32 = winner_score_str["SCORE: ".len()..].parse().unwrap();

    let score = if winner == Winner::North {
        -winner_score
    } else {
        winner_score
    };

    BenchmarkData {
        winner,
        score,
        depth,
    }
}

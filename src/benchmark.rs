use std::{convert::TryInto, process::Command};

fn main() { benchmark(1, [1.0, 0.0, 0.0, 0.0, 0.0, 0.0]) }

fn benchmark(depth: usize, weights: [f32; 6]) {
    let output = Command::new("java")
        .arg("-jar")
        .arg("ManKalah.jar")
        .arg("java -jar MKRefAgent.jar")
        .arg(format!(
            "cargo run --release --bin mankalah -- --search=alpha-beta --weights {}",
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

    let winner = stderr[len - 6];
    assert!(winner.starts_with("WINNER: "));
    let winner_score = stderr[len - 5];
    assert!(winner_score.starts_with("SCORE: "));

    let player2 = stderr[len - 3];
    assert!(player2.starts_with("Player 2 "));
    let player1 = stderr[len - 2];
    assert!(player1.starts_with("Player 1 "));

    let p2_pits = stderr[len - 8].split_whitespace().collect::<Vec<_>>();
    let p2_score: usize = p2_pits[p2_pits.len() - 1].parse().unwrap();

    let p1_pits = stderr[len - 9].split_whitespace().collect::<Vec<_>>();
    let p1_score: usize = p1_pits[0].parse().unwrap();

    let p1_moves: usize = player1.split(":").collect::<Vec<_>>()[1]
        .split_whitespace()
        .collect::<Vec<_>>()[0]
        .parse()
        .unwrap();
    let p1_time_per_move: usize = player1.split(":").collect::<Vec<_>>()[1]
        .split_whitespace()
        .collect::<Vec<_>>()[2]
        .parse()
        .unwrap();
    let p2_moves: usize = player2.split(":").collect::<Vec<_>>()[1]
        .split_whitespace()
        .collect::<Vec<_>>()[0]
        .parse()
        .unwrap();
    let p2_time_per_move: usize = player2.split(":").collect::<Vec<_>>()[1]
        .split_whitespace()
        .collect::<Vec<_>>()[2]
        .parse()
        .unwrap();

    println!(
        r#"
player 1 score: {} ({} moves, {}ms per move)
player 2 score: {} ({} moves, {}ms per move)
{}
{}
{}"#,
        p1_score,
        p1_moves,
        p1_time_per_move,
        p2_score,
        p2_moves,
        p2_time_per_move,
        winner_score,
        player1,
        player2
    );
}

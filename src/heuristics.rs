use crate::board::{BoardState, Position, TOTAL_PITS};
use ordered_float::OrderedFloat;

pub type Score = OrderedFloat<f32>;
pub const NUM_HEURISTICS: usize = 6;
pub const HEURISTICS: [fn(&BoardState) -> i8; NUM_HEURISTICS] = [
    current_score,
    offensive_capture,
    defensive_capture,
    offensive_capture,
    chaining_potential,
    hoarding,
];
pub type Weights = [f32; NUM_HEURISTICS];

pub(crate) fn weighted_heuristic(weights: Weights, board: &BoardState) -> Score {
    let mut score = 0.0;
    for (h, weight) in HEURISTICS.iter().zip(weights.iter()) {
        if *weight != 0.0_f32 {
            score += h(board) as f32 * weight;
        }
    }
    OrderedFloat(score)
}

/// Difference between mancalas (score)
fn current_score(board: &BoardState) -> i8 {
    let south_seeds = board[Position::South].score as i8;
    let north_seeds = board[Position::North].score as i8;
    south_seeds - north_seeds
}

/// Offensive Capture: incentivise choosing boards with more capture
/// opportunites
fn offensive_capture(board: &BoardState) -> i8 {
    fn count_captures(board: &BoardState, pos: Position) -> i8 {
        let mut n_captures = 0;
        for (idx, n_stones) in board[pos]
            .pits
            .iter()
            .enumerate()
            .filter(|(_, n_stones)| **n_stones > 0)
        {
            if let Some(n_opposite) = board[!pos]
                .pits
                .get(idx + (*n_stones as usize % TOTAL_PITS) as usize)
            {
                n_captures += n_opposite + 1; // plus one for capturing seed
            }
        }
        n_captures as i8
    }
    let north_captures = count_captures(board, Position::North);
    let south_captures = count_captures(board, Position::South);
    south_captures - north_captures
}

/// Offensive Capture: incentivise choosing boards with less capture
/// opportunites for opponent.
fn defensive_capture(board: &BoardState) -> i8 { -offensive_capture(board) }

/// Chaining Potential: incentivise moves that repeat your turn.
fn chaining_potential(board: &BoardState) -> i8 {
    fn count_chains(board: &BoardState, pos: Position) -> u8 {
        board
            .child_boards(pos, false)
            .filter(|child| child.1 == pos)
            .count() as u8
    }
    let south_chains = count_chains(board, Position::South) as i8;
    let north_chains = count_chains(board, Position::North) as i8;
    south_chains - north_chains
}

/// Hoarding Stategy: look to pick boards that maximise the number of seeds in
/// the 2 pits closest to our mancala.
fn hoarding(board: &BoardState) -> i8 {
    let n_south = board[Position::South].pits.iter().rev().take(2).sum::<u8>() as i8;
    let n_north = board[Position::North].pits.iter().rev().take(2).sum::<u8>() as i8;
    n_south - n_north
}

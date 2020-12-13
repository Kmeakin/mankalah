use crate::board::{BoardState, Position, PITS_PER_PLAYER, TOTAL_PITS};
use ordered_float::OrderedFloat;

pub type Score = OrderedFloat<f32>;
pub const NUM_HEURISTICS: usize = 5;
pub const HEURISTICS: [fn(&BoardState) -> i8; NUM_HEURISTICS] = [
    current_score,
    offensive_capture,
    defensive_capture,
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
        for (starting_pit, &n_stones) in board[pos]
            .pits
            .iter()
            .enumerate()
            .filter(|(_, &n_stones)| n_stones > 0)
        {
            let final_pit = (starting_pit + n_stones as usize) % (TOTAL_PITS - 1);
            match (
                board[pos].pits.get(final_pit),
                board[!pos].pits.get(final_pit),
            ) {
                (Some(&n_landed), Some(&n_opposite)) => {
                    let n_stones_deposited_in_starting_pit = n_stones as usize / (TOTAL_PITS - 1);

                    // player can capture if:
                    let a = n_landed == 0; // the stone lands in an empty pit
                    let b = final_pit == starting_pit; // or the stone lands in the pit where he started
                    let c = n_stones_deposited_in_starting_pit == 1; // but only when that stone is the first stone deposited in the pit where he
                                                                     // started
                    let d = n_opposite > 0; // and there are more than 0 stones in the pit opposite

                    if (a || (b && c)) && d {
                        n_captures += n_opposite + 1
                    }
                }
                _ => {}
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
            .filter(|child| child.2 == pos)
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::board::{PlayerState, PITS_PER_PLAYER};

    #[track_caller]
    fn test_offensive_capture(
        north: [u8; PITS_PER_PLAYER],
        south: [u8; PITS_PER_PLAYER],
        expected: i8,
    ) {
        let board = BoardState {
            north: PlayerState {
                score: 0,
                pits: north,
            },
            south: PlayerState {
                score: 0,
                pits: south,
            },
        };
        let got = offensive_capture(&board);
        assert_eq!(got, expected);
    }

    #[test]
    fn neither_side_can_capture() { test_offensive_capture([1; 7], [1; 7], 0); }

    #[test]
    fn north_can_capture_1() {
        test_offensive_capture([1, 1, 1, 1, 1, 1, 0], [1; 7], -2);
        test_offensive_capture([1, 1, 1, 1, 1, 0, 1], [1; 7], -2);
        test_offensive_capture([1, 1, 1, 1, 0, 1, 1], [1; 7], -2);
        test_offensive_capture([1, 1, 1, 0, 1, 1, 1], [1; 7], -2);
        test_offensive_capture([1, 1, 0, 1, 1, 1, 1], [1; 7], -2);
        test_offensive_capture([1, 0, 1, 1, 1, 1, 1], [1; 7], -2);
    }

    #[test]
    fn south_can_capture_1() {
        test_offensive_capture([1; 7], [1, 1, 1, 1, 1, 1, 0], 2);
        test_offensive_capture([1; 7], [1, 1, 1, 1, 1, 0, 1], 2);
        test_offensive_capture([1; 7], [1, 1, 1, 1, 0, 1, 1], 2);
        test_offensive_capture([1; 7], [1, 1, 1, 0, 1, 1, 1], 2);
        test_offensive_capture([1; 7], [1, 1, 0, 1, 1, 1, 1], 2);
        test_offensive_capture([1; 7], [1, 0, 1, 1, 1, 1, 1], 2);
    }

    #[test]
    fn both_can_capture_1() {
        test_offensive_capture(
            [1, 1, 1, 1, 1, 1, 0], // north
            [1, 0, 1, 1, 1, 1, 1], // south
            0,
        );
    }

    #[test]
    fn north_can_capture_3() {
        test_offensive_capture(
            [1, 0, 1, 0, 1, 0, 1], // north
            [1, 1, 1, 1, 1, 1, 1], // south
            -6,
        );
    }

    #[test]
    fn south_can_capture_3() {
        test_offensive_capture(
            [1, 1, 1, 1, 1, 1, 1], // north
            [1, 0, 1, 0, 1, 0, 1], // south
            6,
        );
    }

    #[test]
    fn north_can_capture_by_wrapping_around() {
        test_offensive_capture(
            [0, 1, 1, 1, 1, 10, 1], // north
            [1, 1, 1, 1, 1, 1, 1],  // south
            -2,
        );
    }

    #[test]
    fn north_can_capture_by_wrapping_around_twice() {
        test_offensive_capture(
            [0, 1, 1, 1, 1, 25, 1], // north
            [1, 1, 1, 1, 1, 1, 1],  // south
            -2,
        );
    }

    #[test]
    fn north_can_capture_by_wrapping_around_twice_and_landing_on_the_same_pit_where_he_started() {
        test_offensive_capture(
            [0, 1, 1, 1, 1, 15, 1], // north
            [1, 1, 1, 1, 1, 1, 1],  // south
            -2,
        );
    }

    #[test]
    fn north_cannot_capture_by_wrapping_around_thrice_and_landing_on_the_same_pit_where_he_started()
    {
        test_offensive_capture(
            [0, 1, 1, 1, 1, 30, 1], // north
            [1, 1, 1, 1, 1, 1, 1],  // south
            0,
        );
    }
}

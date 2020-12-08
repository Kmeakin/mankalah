use crate::board::{BoardState, Nat, PlayerMove, Position, PITS_PER_PLAYER, TOTAL_PITS};

pub type Score = i8;

pub trait Heuristic {
    fn goodness(board: &BoardState) -> Score;
}

#[derive(Debug, Copy, Clone)]
pub enum CurrentScore {}

/// Difference between mancalas (score)
impl Heuristic for CurrentScore {
    fn goodness(board: &BoardState) -> Score {
        let south_seeds = board[Position::South].score as Score;
        let north_seeds = board[Position::North].score as Score;
        // south_seeds += board[Position::South].pits.iter().sum::<u8>() as i8;
        // north_seeds += board[Position::North].pits.iter().sum::<u8>() as i8;
        south_seeds - north_seeds
    }
}

#[derive(Debug, Copy, Clone)]
pub enum OffensiveCapture {}

/// Offensive Capture: incentivise choosing boards with more capture
/// opportunites for yourself.
impl Heuristic for OffensiveCapture {
    fn goodness(board: &BoardState) -> Score {
        fn count_captures(board: &BoardState, pos: Position) -> Score {
            let mut n_captures = 0;
            for (idx, n_stones) in board[pos]
                .pits
                .iter()
                .enumerate()
                .filter(|(_, n_stones)| **n_stones > 0)
            {
                if let Some(n_opposite) = board[!pos].pits.get(idx + (*n_stones as usize % TOTAL_PITS) as usize)
                {
                    n_captures += n_opposite + 1; // plus one for capturing seed
                }
            }
            n_captures as i8
        }
        count_captures(board, Position::South) - count_captures(board, Position::North)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum DefensiveCapture {}

/// Offensive Capture: incentivise choosing boards with less capture
/// opportunites for opponent.
impl Heuristic for DefensiveCapture {
    fn goodness(board: &BoardState) -> Score { -OffensiveCapture::goodness(board) }
}

#[derive(Debug, Copy, Clone)]
pub enum ChainingCapture {}

/// Chaining Potential: incentivise moves that repeat your turn.
impl Heuristic for ChainingCapture {
    fn goodness(board: &BoardState) -> Score { todo!() }
}

#[derive(Debug, Copy, Clone)]
pub enum Hoarding {}

/// Hoarding Stategy: look to pick boards that maximise the number of seeds in
/// the 2 pits closest to our mancala.
impl Heuristic for Hoarding {
    fn goodness(board: &BoardState) -> Score {
        let n_south = board[Position::South].pits.iter().rev().take(2).sum::<u8>();
        let n_north = board[Position::North].pits.iter().rev().take(2).sum::<u8>();
        n_south as i8 - n_north as i8
    }
}

use crate::board::{BoardState, Nat, PlayerMove, Position, PITS_PER_PLAYER};

pub type Score = i8;

pub trait Heuristic {
    fn goodness(board: &BoardState) -> Score;
}

#[derive(Debug, Copy, Clone)]
pub enum CurrentScore {}

/// Difference between mancalas (score)
impl Heuristic for CurrentScore {
    fn goodness(board: &BoardState) -> Score {
        let mut south_seeds = board[Position::South].score as Score;
        let mut north_seeds = board[Position::North].score as Score;
        south_seeds += board[Position::South].pits.iter().sum::<u8>() as i8;
        north_seeds += board[Position::North].pits.iter().sum::<u8>() as i8;
        south_seeds - north_seeds
    }
}

/// Offensive Capture: incentivise choosing boards with more capture
/// opportunites for yourself.
fn offensive_capture(board: &BoardState) -> Score { todo!() }

pub enum OffensiveCapture {}

#[cfg(FALSE)]
impl Heuristic for OffensiveCapture {
    fn goodness(board: &BoardState) -> Score {
        let mut north_captures = 0;
        let mut south_captures = 0;
        for (idx, n_stones) in board[Position::North].pits.iter().enumerate() {
            if let Some(n_opposite) = board[Position::South].pits.get(idx + n_stones) {
                north_captures += n_opposite;
            }
        }
        for (idx, n_stones) in board[Position::North].pits.iter().enumerate() {
            if let Some(n_opposite) = board[Position::South].pits.get(idx + n_stones) {
                south_captures += n_opposite;
            }
        }
    }
}

/// Defensive Capture: incentivise moves that reduce the number of capture
/// opportunies for your opponent.
fn defensive_capture(board: &BoardState) -> Score { todo!() }

/// Chaining Potential: incentivise moves that repeat your turn.
fn chaining_capture(board: &BoardState) -> Score { todo!() }

/// Hoarding Stategy: look to pick boards that maximise the number of seeds in
/// the 2 pits closest to our mancala.
fn hoarding_strategy(board: &BoardState) -> Score { todo!() }

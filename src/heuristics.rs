use crate::board::{BoardState, Nat, PlayerMove, Position, PITS_PER_PLAYER};

pub trait Heuristic = Fn(&BoardState) -> Score;
pub type Score = i8;

/// Difference between mancalas (score)
pub fn current_score(board: &BoardState) -> Score {
    board[Position::South].score as Score - board[Position::North].score as Score
}

/// Offensive Capture: incentivise choosing boards with more capture
/// opportunites for yourself.
fn offensive_capture(board: &BoardState) -> Score { todo!() }

/// Defensive Capture: incentivise moves that reduce the number of capture
/// opportunies for your opponent.
fn defensive_capture(board: &BoardState) -> Score { todo!() }

/// Chaining Potential: incentivise moves that repeat your turn.
fn chaining_capture(board: &BoardState) -> Score { todo!() }

/// Hoarding Stategy: look to pick boards that maximise the number of seeds in
/// the 2 pits closest to our mancala.
fn hoarding_strategy(board: &BoardState) -> Score { todo!() }

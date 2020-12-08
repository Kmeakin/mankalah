use std::cmp;

use crate::{
    board::{BoardState, PlayerMove, Position},
    heuristics::{Heuristic, Score},
    minimax::MAX_DEPTH,
};

pub trait Evaluator<H: Heuristic> {
    fn eval(board: BoardState, pos: Position, depth: usize, first_move: bool) -> Score;
}

#[derive(Debug, Copy, Clone)]
pub enum MiniMax {}

impl<H: Heuristic> Evaluator<H> for MiniMax {
    fn eval(board: BoardState, pos: Position, depth: usize, _first_move: bool) -> Score {
        minimax::<H>(board, pos, depth)
    }
}

fn minimax<H: Heuristic>(board: BoardState, position: Position, depth: usize) -> Score {
    if let Some(payoff) = board.is_terminal(position) {
        payoff
    } else if depth >= MAX_DEPTH {
        H::goodness(&board)
    } else {
        // FIXME! First move stuff
        let iter = board[position]
            .moves_iter()
            .map(|player_move| board.clone().apply_move(player_move, position, false))
            .map(|(board, child_position, _)| minimax::<H>(board, child_position, depth + 1));

        match position {
            Position::South => iter.max().unwrap(), // player 1
            Position::North => iter.min().unwrap(), // player 2
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum AlphaBeta {}

impl<H: Heuristic> Evaluator<H> for AlphaBeta {
    fn eval(board: BoardState, pos: Position, depth: usize, first_move: bool) -> Score {
        let alpha = Score::MIN;
        let beta = Score::MAX;
        alpha_beta::<H>(&board, depth, alpha, beta, pos, first_move)
    }
}

fn alpha_beta<H: Heuristic>(
    board: &BoardState,
    depth: usize,
    mut alpha: Score,
    mut beta: Score,
    pos: Position,
    first_move: bool,
) -> Score {
    if let Some(payoff) = board.is_terminal(pos) {
        payoff
    } else if depth >= MAX_DEPTH {
        H::goodness(board)
    } else {
        match pos {
            Position::South => {
                let mut value = Score::MIN;
                for (child, next_pos, next_fist_move) in board.child_boards(pos, first_move) {
                    value = cmp::max(
                        value,
                        alpha_beta::<H>(board, depth + 1, alpha, beta, next_pos, next_fist_move),
                    );
                    alpha = cmp::max(alpha, value);
                    if alpha >= beta {
                        break;
                    }
                }
                value
            }
            Position::North => {
                let mut value = Score::MAX;
                let boards = board.child_boards(pos, first_move);
                let boards = if first_move {
                    boards.chain(Some(board.do_move(
                        PlayerMove::Swap,
                        Position::North,
                        first_move,
                    )))
                } else {
                    boards.chain(None)
                };

                for (child, next_pos, next_first_move) in boards {
                    value = cmp::min(
                        value,
                        alpha_beta::<H>(&child, depth + 1, alpha, beta, next_pos, next_first_move),
                    );
                    beta = cmp::min(beta, value);
                    if beta <= alpha {
                        break;
                    }
                }
                value
            }
        }
    }
}

use crate::{
    board::{BoardState, PlayerMove, Position},
    heuristics::{weighted_heuristic, Score, Weights},
};
use std::cmp;
use ordered_float::OrderedFloat;

pub trait Evaluator {
    fn eval(
        board: BoardState,
        pos: Position,
        depth: usize,
        first_move: bool,
        max_depth: usize,
        weights: Weights,
    ) -> Score;
}

#[derive(Debug, Copy, Clone)]
pub enum MiniMax {}

impl Evaluator for MiniMax {
    fn eval(
        board: BoardState,
        pos: Position,
        depth: usize,
        first_move: bool,
        max_depth: usize,
        weights: Weights,
    ) -> Score {
        minimax(board, pos, depth, first_move, max_depth, weights)
    }
}

fn minimax(
    board: BoardState,
    position: Position,
    depth: usize,
    first_move: bool,
    max_depth: usize,
    weights: Weights,
) -> Score {
    if let Some(payoff) = board.is_terminal(position) {
        payoff
    } else if depth >= max_depth {
        weighted_heuristic(weights, &board)
    } else {
        let iter = board.child_boards(position, first_move).map(
            |(board, child_position, next_first_move)| {
                minimax(
                    board,
                    child_position,
                    depth + 1,
                    next_first_move,
                    max_depth,
                    weights,
                )
            },
        );

        match position {
            Position::South => iter.max().unwrap(), // player 1
            Position::North => iter.min().unwrap(), // player 2
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum AlphaBeta {}

impl Evaluator for AlphaBeta {
    fn eval(
        board: BoardState,
        pos: Position,
        depth: usize,
        first_move: bool,
        max_depth: usize,
        weights: Weights,
    ) -> Score {
        let alpha = OrderedFloat(-f32::INFINITY);
        let beta = OrderedFloat(f32::INFINITY);
        alpha_beta(
            board, depth, alpha, beta, pos, first_move, max_depth, weights,
        )
    }
}

fn alpha_beta(
    board: BoardState,
    depth: usize,
    mut alpha: Score,
    mut beta: Score,
    pos: Position,
    first_move: bool,
    max_depth: usize,
    weights: Weights,
) -> Score {
    if let Some(payoff) = board.is_terminal(pos) {
        payoff
    } else if depth >= max_depth {
        weighted_heuristic(weights, &board)
    } else {
        match pos {
            Position::South => {
                let mut value = OrderedFloat(-f32::INFINITY);
                for (child, next_pos, next_fist_move) in board.child_boards(pos, first_move) {
                    value = cmp::max(
                        value,
                        alpha_beta(
                            child,
                            depth + 1,
                            alpha,
                            beta,
                            next_pos,
                            next_fist_move,
                            max_depth,
                            weights,
                        ),
                    );
                    alpha = cmp::max(alpha, value);
                    if alpha >= beta {
                        break;
                    }
                }
                value
            }
            Position::North => {
                let mut value = OrderedFloat(f32::INFINITY);
                for (child, next_pos, next_first_move) in board.child_boards(pos, first_move) {
                    value = cmp::min(
                        value,
                        alpha_beta(
                            child,
                            depth + 1,
                            alpha,
                            beta,
                            next_pos,
                            next_first_move,
                            max_depth,
                            weights,
                        ),
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

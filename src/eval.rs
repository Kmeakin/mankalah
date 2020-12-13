use crate::{
    board::{BoardState, Position},
    heuristics::{weighted_heuristic, Score, Weights},
};
use ordered_float::OrderedFloat;
use std::cmp;

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
    log::debug!(
        "{:depth$}alpha = {alpha}, beta = {beta}, pos = {pos}, first_move = {first_move}, \
         max_depth = {max_depth}",
        // pad the empty string with 2 * depth spaces
        "",
        depth = depth * 2,
        // provide values for string interpolation. `#![feature(format_args_capture)]` should do
        // this automatically, but it doesnt in logging for some reason
        alpha = alpha,
        beta = beta,
        pos = pos,
        first_move = first_move,
        max_depth = max_depth
    );
    if let Some(score) = board.is_terminal(pos) {
        log::debug!(
            "{:depth$}board is terminal: score = {score}",
            "",
            depth = depth * 2,
            score = score
        );
        score
    } else if depth >= max_depth {
        let score = weighted_heuristic(weights, &board);
        log::debug!(
            "{:depth$}max depth exceeded, using heuristics: score = {score}",
            "",
            depth = depth * 2,
            score = score
        );
        score
    } else {
        match pos {
            Position::South => {
                let mut value = OrderedFloat(-f32::INFINITY);
                for (child, next_pos, next_fist_move) in board.child_boards(pos, first_move) {
                    log::debug!(
                        "{:depth$}child_board = {child:?}",
                        "",
                        depth = depth * 2,
                        child = child,
                    );
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
                    let new_alpha = cmp::max(alpha, value);
                    log::debug!(
                        "{:depth$}alpha = max({alpha}, {value}) = {new_alpha}",
                        "",
                        depth = depth * 2,
                        alpha = alpha,
                        value = value,
                        new_alpha = new_alpha
                    );
                    alpha = new_alpha;
                    if alpha >= beta {
                        log::debug!(
                            "{:depth$}alpha >= beta ({alpha} > {beta}), breaking",
                            "",
                            depth = depth * 2,
                            alpha = alpha,
                            beta = beta
                        );
                        break;
                    }
                }
                value
            }
            Position::North => {
                let mut value = OrderedFloat(f32::INFINITY);
                for (child, next_pos, next_first_move) in board.child_boards(pos, first_move) {
                    log::debug!(
                        "{:depth$}child_board = {child:?}",
                        "",
                        depth = depth * 2,
                        child = child,
                    );
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
                    let new_beta = cmp::min(beta, value);
                    log::debug!(
                        "{:depth$}beta = min({beta}, {value}) = {new_beta}",
                        "",
                        depth = depth * 2,
                        beta = beta,
                        value = value,
                        new_beta = new_beta
                    );
                    beta = new_beta;
                    if beta <= alpha {
                        log::debug!(
                            "{:depth$}beta <= alpha ({beta} <= {alpha}), breaking",
                            "",
                            depth = depth * 2,
                            beta = beta,
                            alpha = alpha
                        );
                        break;
                    }
                }
                log::debug!(
                    "{:depth$}value = {value}",
                    "",
                    depth = depth * 2,
                    value = value
                );
                value
            }
        }
    }
}

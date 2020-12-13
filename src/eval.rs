use crate::{board::{BoardState, PlayerMove, Position}, heuristics::{weighted_heuristic, Score, Weights}};
use ordered_float::OrderedFloat;
use std::cmp;

type Evaluation = (Option<PlayerMove>, Score);
pub trait Evaluator {
    fn eval(
        board: BoardState,
        pos: Position,
        depth: usize,
        first_move: bool,
        max_depth: usize,
        weights: Weights,
    ) -> Evaluation;
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
    ) -> Evaluation {
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
) -> Evaluation {
    if let Some(payoff) = board.is_terminal(position) {
        (None, payoff)
    } else if depth >= max_depth {
        (None, weighted_heuristic(weights, &board))
    } else {
        let iter = board.child_boards(position, first_move).map(
            |(the_move, board, child_position, next_first_move)| {
              let (_, score) = minimax(
                board,
                child_position,
                depth + 1,
                next_first_move,
                max_depth,
                weights,
              );
              (Some(the_move), score)
            },
        );

        match position {
            Position::South => iter.max_by_key(|&(_, score)| score).unwrap(), // player 1
            Position::North => iter.min_by_key(|&(_, score)| score).unwrap(), // player 2
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
    ) -> Evaluation {
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
) -> Evaluation {
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
        (None, score)
    } else if depth >= max_depth {
        let score = weighted_heuristic(weights, &board);
        log::debug!(
            "{:depth$}max depth exceeded, using heuristics: score = {score}",
            "",
            depth = depth * 2,
            score = score
        );
        (None, score)
    } else {
        match pos {
            Position::South => {
                let mut score = OrderedFloat(-f32::INFINITY);
                let mut value = score;
                let mut best_move: Option<PlayerMove> = None;
                for (the_move, child, next_pos, next_fist_move) in board.child_boards(pos, first_move) {
                    log::debug!(
                        "{:depth$}child_board = {child:?}",
                        "",
                        depth = depth * 2,
                        child = child,
                    );

                    let (_, child_score) = alpha_beta(
                      child,
                      depth + 1,
                      alpha,
                      beta,
                      next_pos,
                      next_fist_move,
                      max_depth,
                      weights,
                    );

                    if child_score > score {
                      score = child_score;
                      best_move = Some(the_move);
                    }

                    value = cmp::max(value ,score);
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
                (best_move, value)
            }
            Position::North => {
                let mut score = OrderedFloat(f32::INFINITY);
                let mut value = score;
                let mut best_move: Option<PlayerMove> = None;
                for (the_move, child, next_pos, next_first_move) in board.child_boards(pos, first_move) {
                    log::debug!(
                        "{:depth$}the_move = {the_move:?} child_board = {child:?}",
                        "",
                        depth = depth * 2,
                        child = child,
                        the_move = the_move
                    );
                    let (_, child_score) =  alpha_beta(
                      child,
                      depth + 1,
                      alpha,
                      beta,
                      next_pos,
                      next_first_move,
                      max_depth,
                      weights,
                    );

                    if child_score < score {
                      score = child_score;
                      best_move = Some(the_move);
                    }

                    value = cmp::min(value,score);

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
                (best_move, value)
            }
        }
    }
}

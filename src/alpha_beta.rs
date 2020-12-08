use crate::{
    board::{BoardState, PlayerMove, Position},
    heuristics::{Heuristic, Score},
    minimax::MAX_DEPTH,
};
use std::cmp;

impl BoardState {
    pub fn alpha_beta<H: Heuristic>(
        &self,
        depth: usize,
        mut alpha: Score,
        mut beta: Score,
        pos: Position,
        first_move: bool,
    ) -> Score {
        if let Some(payoff) = self.is_terminal(pos) {
            payoff
        } else if depth >= MAX_DEPTH {
            H::goodness(self)
        } else {
            match pos {
                Position::South => {
                    let mut value = Score::MIN;
                    for (child, next_pos, next_fist_move) in self.child_boards(pos, first_move) {
                        value = cmp::max(
                            value,
                            child.alpha_beta::<H>(depth + 1, alpha, beta, next_pos, next_fist_move),
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
                    // if first_move {
                    // dbg!(first_move);
                    // }
                    let boards = self.child_boards(pos, first_move);
                    let boards = if first_move {
                        boards.chain(Some(self.do_move(
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
                            child.alpha_beta::<H>(
                                depth + 1,
                                alpha,
                                beta,
                                next_pos,
                                next_first_move,
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
}

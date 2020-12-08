use crate::{
    board::{BoardState, Position},
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
    ) -> Score {
        if let Some(payoff) = self.is_terminal(pos) {
            payoff
        } else if depth >= MAX_DEPTH {
            H::goodness(self)
        } else {
            match pos {
                Position::South => {
                    let mut value = Score::MIN;
                    for (child, next_pos) in self.child_boards(pos) {
                        value = cmp::max(
                            value,
                            child.alpha_beta::<H>(depth + 1, alpha, beta, next_pos),
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
                    for (child, next_pos) in self.child_boards(pos) {
                        value = cmp::min(
                            value,
                            child.alpha_beta::<H>(depth + 1, alpha, beta, next_pos),
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

use crate::{
    agent::Agent,
    board::{BoardState, Nat, PlayerMove, Position, PITS_PER_PLAYER},
};
use std::{char::from_u32, ops::Not};

const MAX_DEPTH: usize = 10;

pub type Value = u32;
pub trait Heuristic = Fn(&BoardState) -> i8;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Mode {
    Max,
    Min,
}

impl Not for Position {
    type Output = Position;

    fn not(self) -> Self::Output {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
        }
    }
}

/*
                        0   1   2   3   4   5   6   7
                                    North
                        [7] [7] [7] [7] [7] [7] [7] [7]
North's score -> [x]                                        [x] <- South's score
                        [7] [7] [7] [7] [7] [7] [7] [7]
                                    South
                        0   1   2   3   4   5   6   7

*/

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FinalLocation {
    SouthScore,
    NorthScore,
    North(Nat),
    South(Nat),
}

impl BoardState {
    fn clockwise_iter(
        &mut self,
        position: Position,
        n: usize,
    ) -> Box<dyn Iterator<Item = (usize, &mut u8)> + '_> {
        debug_assert!(n <= PITS_PER_PLAYER);
        match position {
            Position::South => Box::new(
                self.south
                    .pits
                    .iter_mut()
                    .chain(std::iter::once(&mut self.south.score))
                    .chain(self.north.pits.iter_mut().rev())
                    .enumerate()
                    .skip(n),
            ),
            Position::North => Box::new(
                self.north
                    .pits
                    .iter_mut()
                    .rev()
                    .chain(std::iter::once(&mut self.north.score))
                    .chain(self.south.pits.iter_mut())
                    .enumerate()
                    .skip(PITS_PER_PLAYER - n + 1),
            ),
        }
    }

    fn sow_seeds(&mut self, pos: Position, n: Nat) -> FinalLocation {
        let mut n = n as usize;
        let mut stones_left = self[pos].pits[n];
        self[pos].pits[n] = 0;
        n += 1;
        debug_assert!(stones_left > 0);
        loop {
            for (idx, pit) in self.clockwise_iter(pos, n) {
                *pit += 1;
                if stones_left <= 1 {
                    return match pos {
                        Position::North => match idx {
                            0..=6 => FinalLocation::North((PITS_PER_PLAYER - 1 - idx) as Nat),
                            7 => FinalLocation::NorthScore,
                            _ => FinalLocation::South(idx as Nat),
                        },
                        Position::South => match idx {
                            0..=6 => FinalLocation::South(idx as Nat),
                            7 => FinalLocation::SouthScore,
                            _ => FinalLocation::North((idx - PITS_PER_PLAYER - 1) as Nat),
                        },
                    };
                }
                stones_left -= 1;
            }
            n = if pos == Position::South {
                0
            } else {
                PITS_PER_PLAYER + 1
            };
        }
    }

    fn try_capture(&mut self, position: Position, final_pit: Nat) {
        let final_pit = final_pit as usize;
        if self[position].pits[final_pit] == 1 {
            // must have been 0 before
            self[position].pits[final_pit] = 0;
            let captured = self[!position].pits[final_pit];
            self[position].score += captured + 1;
            self[!position].pits[final_pit] = 0;
        }
    }

    fn apply_move(&mut self, moove: PlayerMove, position: Position) -> (Self, Position) {
        match moove {
            PlayerMove::Move { n } => match (position, self.sow_seeds(position, n)) {
                (Position::South, FinalLocation::SouthScore) => (*self, position),
                (Position::North, FinalLocation::NorthScore) => (*self, position),
                (Position::South, FinalLocation::South(n))
                | (Position::North, FinalLocation::North(n)) => {
                    self.try_capture(position, n);
                    (*self, !position)
                }
                (_, _) => (*self, !position),
            },
            PlayerMove::Swap => {
                std::mem::swap(&mut self.north, &mut self.south);
                (*self, position)
            }
        }
    }

    fn child_boards(
        &self,
        position: Position,
        can_swap: bool,
    ) -> impl Iterator<Item = (BoardState, Position)> + '_ {
        self[position]
            .moves_iter(can_swap)
            .map(move |player_move| self.clone().apply_move(player_move, position))
    }

    fn is_terminal(&self, pos: Position, can_swap: bool) -> Option<i8> {
        match self[pos].moves_iter(can_swap).next() {
            None => {
                let our_score = self[pos].score;
                let mut opp_score = self[!pos].score;
                opp_score += self[!pos].pits.iter().sum::<u8>();
                Some(our_score as i8 - opp_score as i8)
            }
            _ => None,
        }
    }

    fn minimax(&self, h: &impl Heuristic, position: Position, can_swap: bool, depth: usize) -> i8 {
        if let Some(payoff) = self.is_terminal(position, can_swap) {
            payoff
        } else if depth == MAX_DEPTH {
            h(self)
        } else {
            let iter = self[position]
                .moves_iter(can_swap)
                .map(|player_move| self.clone().apply_move(player_move, position))
                .map(|(board, position)| board.minimax(h, position, can_swap, depth + 1));
            match position {
                Position::North => iter.max().unwrap(),
                Position::South => iter.min().unwrap(),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::board::PlayerState;

    use super::*;

    #[test]
    fn test_sow_start_of_game_south_1() {
        let mut board_state = BoardState::default();
        board_state.sow_seeds(Position::South, 1);

        assert_eq!(
            board_state,
            BoardState {
                north: PlayerState {
                    score: 0,
                    pits: [7, 7, 7, 7, 7, 7, 8]
                },
                south: PlayerState {
                    score: 1,
                    pits: [7, 0, 8, 8, 8, 8, 8]
                }
            }
        );
    }

    #[test]
    fn test_sow_start_of_game_north_4() {
        let mut board_state = BoardState::default();
        board_state.sow_seeds(Position::North, 4);
        assert_eq!(
            board_state,
            BoardState {
                north: PlayerState {
                    score: 1,
                    pits: [8, 8, 8, 8, 0, 7, 7]
                },
                south: PlayerState {
                    score: 0,
                    pits: [8, 8, 7, 7, 7, 7, 7]
                }
            }
        )
    }

    #[test]
    fn test_sow_a_lot() {
        let mut board_state = BoardState {
            north: PlayerState {
                score: 1,
                pits: [0, 0, 0, 8, 0, 7, 7],
            },
            south: PlayerState {
                score: 0,
                pits: [8, 8, 31, 7, 7, 7, 7],
            },
        };
        board_state.sow_seeds(Position::South, 2);
        assert_eq!(
            board_state,
            BoardState {
                north: PlayerState {
                    score: 1,
                    pits: [2, 2, 2, 10, 2, 9, 9]
                },
                south: PlayerState {
                    score: 2,
                    pits: [10, 10, 2, 10, 9, 9, 9]
                }
            }
        )
    }

    #[test]
    fn test_apply_move_capture() {
        let mut board_state = BoardState {
            north: PlayerState {
                score: 0,
                pits: [0, 7, 0, 0, 0, 0, 0],
            },
            south: PlayerState {
                score: 0,
                pits: [1, 0, 0, 0, 0, 0, 0],
            },
        };

        board_state.apply_move(PlayerMove::Move { n: 0 }, Position::South);
        assert_eq!(
            board_state,
            BoardState {
                north: PlayerState {
                    score: 0,
                    pits: [0, 0, 0, 0, 0, 0, 0],
                },
                south: PlayerState {
                    score: 8,
                    pits: [0, 0, 0, 0, 0, 0, 0],
                }
            }
        )
    }

    #[test]
    fn example_play() {
        let mut board_state = BoardState {
            north: PlayerState {
                score: 0,
                pits: [2, 2, 2, 3, 0, 0, 1],
            },
            south: PlayerState {
                score: 0,
                pits: [2, 2, 2, 0, 0, 2, 3],
            },
        };

        board_state.sow_seeds(Position::North, 0);
        assert_eq!(
            board_state,
            BoardState {
                north: PlayerState {
                    pits: [0, 2, 2, 3, 0, 0, 1],
                    score: 1,
                },
                south: PlayerState {
                    score: 0,
                    pits: [3, 2, 2, 0, 0, 2, 3],
                }
            }
        );

        assert_eq!(
            board_state.sow_seeds(Position::South, 5),
            FinalLocation::SouthScore
        );
        assert_eq!(
            board_state,
            BoardState {
                north: PlayerState {
                    pits: [0, 2, 2, 3, 0, 0, 1],
                    score: 1,
                },
                south: PlayerState {
                    score: 1,
                    pits: [3, 2, 2, 0, 0, 0, 4],
                }
            }
        );

        assert_eq!(
            board_state.apply_move(PlayerMove::Move { n: 1 }, Position::South),
            (
                BoardState {
                    north: PlayerState {
                        pits: [0, 2, 2, 0, 0, 0, 1],
                        score: 1,
                    },
                    south: PlayerState {
                        score: 5,
                        pits: [3, 0, 3, 0, 0, 0, 4],
                    }
                },
                Position::North
            )
        );
    }
}

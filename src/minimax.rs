use crate::heuristics::Heuristic;
use crate::{
    agent::Agent,
    board::{BoardState, Nat, PlayerMove, Position, PITS_PER_PLAYER},
    heuristics::Score
};
use std::ops::Not;

const MAX_DEPTH: usize = 10;

pub type Value = u32;


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

#[derive(Debug)]
struct SowSeedsIterator {
    board: BoardState,
    position: Position,
    index: isize,
}

impl SowSeedsIterator {
    const IPITS_PER_PLAYER: isize = PITS_PER_PLAYER as isize;
    const SOUTH_SCORE: isize = SowSeedsIterator::IPITS_PER_PLAYER + 1;
    const NORTH_SCORE: isize = 0;

    fn new(board: BoardState, pos: Position, start_at: Nat) -> SowSeedsIterator {
        // start_at is 1 based for which pit the player picks
        // zero is used when the sowing loops around (meaning we don't skip any pits)
        let start_index = if start_at == 0 {
            1
        } else {
            (start_at as isize) + if pos == Position::South { 1 } else { -1 }
        };
        SowSeedsIterator {
            board,
            position: pos,
            index: start_index,
        }
    }

    fn our_side(&self) -> bool { self.index >= 1 && self.index <= Self::IPITS_PER_PLAYER }
    fn their_side(&self) -> bool { self.index <= -1 && self.index >= -Self::IPITS_PER_PLAYER }

    fn to_location(&self, index: isize) -> FinalLocation {
        match (self.position, index) {
            (Position::South, 1..=7) => FinalLocation::South((index - 1) as Nat),
            (Position::South, Self::SOUTH_SCORE) => FinalLocation::SouthScore,
            (Position::South, -7..=-1) => FinalLocation::North((index - 1) as Nat),
            (Position::North, 1..=7) => FinalLocation::North((index - 1) as Nat),
            (Position::North, Self::NORTH_SCORE) => FinalLocation::NorthScore,
            (Position::North, -7..=-1) => FinalLocation::South((index - 1) as Nat),
            (_, _) => unreachable!(),
        }
    }
}

impl Iterator for SowSeedsIterator {
    type Item = FinalLocation;
    fn next(&mut self) -> Option<Self::Item> {
        let visited = self.index;
        match self.position {
            Position::South => {
                if self.our_side() {
                    self.board.south.pits[(visited - 1) as usize] += 1;
                    self.index += 1;
                } else if self.index == Self::IPITS_PER_PLAYER + 1 {
                    self.board.south.score += 1;
                    self.index = -7;
                } else if self.their_side() {
                    self.board.north.pits[(visited.abs() - 1) as usize] += 1;
                    self.index += 1;
                } else {
                    return None;
                }
            }
            Position::North => {
                if self.our_side() {
                    self.board.north.pits[(visited - 1) as usize] += 1;
                    self.index -= 1;
                } else if self.index == 0 {
                    self.board.north.score += 1;
                    self.index = -1;
                } else if self.their_side() {
                    self.board.south.pits[(visited.abs() - 1) as usize] += 1;
                    self.index -= 1;
                } else {
                    return None;
                }
            }
        }
        Some(self.to_location(visited))
    }
}

impl BoardState {
    fn sow_seeds(&mut self, pos: Position, n: Nat) -> FinalLocation {
        let mut n = n;
        let mut stones_left = self[pos].pits[n as usize];
        self[pos].pits[n as usize] = 0;
        n += 1; // make n a 1 based index for the seed sow iterator (only for first cycle)
        loop {
            let mut sow_iter = SowSeedsIterator::new(*self, pos, n);
            while let Some(pit) = sow_iter.next() {
                // mutate self here (since making a mutable iterator is too hard/unsafe?)
                *self = sow_iter.board;
                if stones_left == 1 {
                    return pit;
                }
                stones_left -= 1;
            }
            // zero for all others (meaning no pits skipped)
            n = 0;
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

    pub fn apply_move(&mut self, moove: PlayerMove, position: Position) -> (Self, Position) {
        let end_position = match moove {
            PlayerMove::Move { n } => match (position, self.sow_seeds(position, n)) {
                (Position::South, FinalLocation::SouthScore)
                | (Position::North, FinalLocation::NorthScore) => position,
                (Position::South, FinalLocation::South(n))
                | (Position::North, FinalLocation::North(n)) => {
                    self.try_capture(position, n);
                    !position
                }
                (_, _) => !position,
            },
            PlayerMove::Swap => {
                std::mem::swap(&mut self.north, &mut self.south);
                position
            }
        };
        (*self, end_position)
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

    /// Return Some(score) if board state is terminal
    /// Else None
    fn is_terminal(&self,pos: Position, can_swap: bool) -> Option<Score> {
        match self[pos].moves_iter(can_swap).next() {
            None => {
                let our_score = self[pos].score as i8;
                let mut opp_score = self[!pos].score as i8;
                opp_score += self[!pos].pits.iter().sum::<u8>() as i8;
                let (p1_score, p2_score) = match pos {
                    Position::South => (our_score,opp_score),
                    Position::North => (opp_score,our_score),
                };
                Some(p1_score-p2_score)
            }
            Some(_) => None,
        }
    }

    pub fn minimax(&self, h: &impl Heuristic, position: Position, can_swap: bool, depth: usize) -> Score {
        dbg!(depth);
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
                Position::South => iter.max().unwrap(), // player 1
                Position::North => iter.min().unwrap(), // player 2
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

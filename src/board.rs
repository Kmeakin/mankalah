use std::{
    fmt,
    ops::{Index, IndexMut, Not},
};

use crate::heuristics::Score;
pub type Nat = u8;
pub const PITS_PER_PLAYER: usize = 7;
pub const TOTAL_PITS: usize = 2 * (PITS_PER_PLAYER + 1);
use ordered_float::OrderedFloat;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PlayerState {
    pub score: Nat,
    pub pits: [Nat; PITS_PER_PLAYER],
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState {
            score: 0,
            pits: [7; PITS_PER_PLAYER],
        }
    }
}

impl PlayerState {
    /// Returns an iterator of the possible moves that can be made from this
    /// PlayerState
    pub fn moves_iter(&self) -> impl Iterator<Item = PlayerMove> + '_ {
        self.pits.iter().enumerate().filter_map(|(idx, stones)| {
            if *stones > 0 {
                Some(PlayerMove::Move { n: idx as Nat })
            } else {
                None
            }
        })
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct BoardState {
    pub north: PlayerState,
    pub south: PlayerState,
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

/// South always starts the game, so "South" means the agent is the first player
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Position {
    North,
    South,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:?}", self) }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PlayerMove {
    Move { n: Nat },
    Swap,
}

impl fmt::Display for PlayerMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlayerMove::Move { n } => write!(f, "MOVE;{}\n", n + 1),
            PlayerMove::Swap => write!(f, "SWAP\n"),
        }
    }
}

impl Index<Position> for BoardState {
    type Output = PlayerState;

    fn index(&self, index: Position) -> &Self::Output {
        match index {
            Position::North => &self.north,
            Position::South => &self.south,
        }
    }
}

impl IndexMut<Position> for BoardState {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        match index {
            Position::North => &mut self.north,
            Position::South => &mut self.south,
        }
    }
}

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
    const SCORING_PIT: isize = SowSeedsIterator::IPITS_PER_PLAYER + 1;

    fn new(board: BoardState, pos: Position, start_at: Nat) -> SowSeedsIterator {
        // start_at is 1 based for which pit the player picks
        // zero is used when the sowing loops around (meaning we don't skip any pits)
        let start_index = if start_at == 0 {
            1
        } else {
            (start_at as isize) + 1
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
            (Position::South, Self::SCORING_PIT) => FinalLocation::SouthScore,
            (Position::South, -7..=-1) => FinalLocation::North((index - 1) as Nat),
            (Position::North, 1..=7) => FinalLocation::North((index - 1) as Nat),
            (Position::North, Self::SCORING_PIT) => FinalLocation::NorthScore,
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
                    self.index = -1;
                } else if self.their_side() {
                    self.board.north.pits[(visited.abs() - 1) as usize] += 1;
                    self.index -= 1;
                } else {
                    return None;
                }
            }
            Position::North => {
                if self.our_side() {
                    self.board.north.pits[(visited - 1) as usize] += 1;
                    self.index += 1;
                } else if self.index == Self::IPITS_PER_PLAYER + 1 {
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
            let opp_bit = 6 - final_pit;
            let captured = self[!position].pits[opp_bit];
            if captured > 0 {
                self[position].pits[final_pit] = 0;
                self[position].score += captured + 1;
                self[!position].pits[opp_bit] = 0;
            }
        }
    }

    pub fn apply_move(
        &mut self,
        moove: PlayerMove,
        position: Position,
        mut first_move: bool,
    ) -> (Self, Position, bool) {
        let mut end_position = match moove {
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
        if first_move {
            if position == Position::South {
                end_position = Position::North;
            } else {
                first_move = false;
            }
        }

        (*self, end_position, first_move)
    }

    pub fn do_move(
        &self,
        moove: PlayerMove,
        position: Position,
        first_move: bool,
    ) -> (Self, Position, bool) {
        self.clone().apply_move(moove, position, first_move)
    }

    pub fn child_boards(
        &self,
        position: Position,
        first_move: bool,
    ) -> impl Iterator<Item = (PlayerMove, BoardState, Position, bool)> + '_ {
        let boards = self[position].moves_iter().map(move |player_move| {
            let (board, next_position, next_first_move) =
                self.do_move(player_move, position, first_move);
            (player_move, board, next_position, next_first_move)
        });
        if first_move && position == Position::North {
            let (board, next_position, next_first_move) =
                self.do_move(PlayerMove::Swap, Position::North, first_move);
            boards.chain(Some((
                PlayerMove::Swap,
                board,
                next_position,
                next_first_move,
            )))
        } else {
            boards.chain(None)
        }
    }

    /// Return Some(score) if board state is terminal
    /// Else None
    pub fn is_terminal(&self, pos: Position) -> Option<Score> {
        match self[pos].moves_iter().next() {
            None => {
                let our_score = self[pos].score as i8;
                let mut opp_score = self[!pos].score as i8;
                opp_score += self[!pos].pits.iter().sum::<u8>() as i8;
                let (p1_score, p2_score) = match pos {
                    Position::South => (our_score, opp_score),
                    Position::North => (opp_score, our_score),
                };
                Some(OrderedFloat((p1_score - p2_score) as f32))
            }
            Some(_) => None,
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
                    pits: [8, 7, 7, 7, 7, 7, 7]
                },
                south: PlayerState {
                    pits: [7, 0, 8, 8, 8, 8, 8],
                    score: 1,
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
                    pits: [7, 7, 7, 7, 0, 8, 8],
                },
                south: PlayerState {
                    score: 0,
                    pits: [8, 8, 8, 8, 7, 7, 7]
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

        board_state.apply_move(PlayerMove::Move { n: 0 }, Position::South, false);
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
                pits: [2, 2, 2, 3, 0, 0, 1],
                score: 0,
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
                    pits: [0, 3, 3, 3, 0, 0, 1],
                    score: 0,
                },
                south: PlayerState {
                    pits: [2, 2, 2, 0, 0, 2, 3],
                    score: 0,
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
                    score: 0,
                    pits: [0, 3, 3, 3, 0, 0, 1],
                },
                south: PlayerState {
                    pits: [2, 2, 2, 0, 0, 0, 4],
                    score: 1,
                }
            }
        );

        assert_eq!(
            board_state.apply_move(PlayerMove::Move { n: 1 }, Position::South, false),
            (
                BoardState {
                    north: PlayerState {
                        score: 0,
                        pits: [0, 3, 3, 0, 0, 0, 1],
                    },
                    south: PlayerState {
                        pits: [2, 0, 3, 0, 0, 0, 4],
                        score: 5,
                    }
                },
                Position::North,
                false
            )
        );
    }

    #[test]
    fn test_pie_rule() {
        let player_state = PlayerState::default();

        assert_eq!(
            player_state.moves_iter().collect::<Vec<PlayerMove>>(),
            vec![
                PlayerMove::Move { n: 0 },
                PlayerMove::Move { n: 1 },
                PlayerMove::Move { n: 2 },
                PlayerMove::Move { n: 3 },
                PlayerMove::Move { n: 4 },
                PlayerMove::Move { n: 5 },
                PlayerMove::Move { n: 6 },
            ]
        );
    }

    #[test]
    fn test_no_possible_moves() {
        let player_state = PlayerState {
            score: 0,
            pits: [0; PITS_PER_PLAYER],
        };

        assert_eq!(
            player_state.moves_iter().collect::<Vec<PlayerMove>>(),
            vec![]
        );
    }

    #[test]
    fn north_one_seed_in_6() {
        let mut board_state = BoardState {
            north: PlayerState {
                pits: [6, 5, 0, 3, 3, 1, 0],
                score: 7,
            },
            south: PlayerState {
                score: 27,
                pits: [0, 3, 18, 3, 0, 2, 20],
            },
        };

        assert_eq!(
            board_state.apply_move(PlayerMove::Move { n: 5 }, Position::North, false),
            (
                BoardState {
                    north: PlayerState {
                        pits: [6, 5, 0, 3, 3, 0, 1],
                        score: 7,
                    },
                    south: PlayerState {
                        score: 27,
                        pits: [0, 3, 18, 3, 0, 2, 20],
                    },
                },
                Position::South,
                false
            )
        );
    }

    #[test]
    fn some_possible_moves() {
        let player_state = PlayerState {
            score: 0,
            pits: [0, 0, 4, 0, 2, 8, 0],
        };
        assert_eq!(
            player_state.moves_iter().collect::<Vec<PlayerMove>>(),
            vec![
                PlayerMove::Move { n: 2 },
                PlayerMove::Move { n: 4 },
                PlayerMove::Move { n: 5 },
            ]
        );

        fn wrap_around() {
            let player_state = PlayerState {
                score: 0,
                pits: [0, 0, 4, 0, 2, 8, 0],
            };
            assert_eq!(
                player_state.moves_iter().collect::<Vec<PlayerMove>>(),
                vec![
                    PlayerMove::Move { n: 2 },
                    PlayerMove::Move { n: 4 },
                    PlayerMove::Move { n: 5 },
                ]
            );
        }
    }
}

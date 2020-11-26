use std::ops::{Index, IndexMut};
pub type Nat = u8;
pub const PITS_PER_PLAYER: usize = 7;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct PlayerState {
    pub score: Nat,
    pub pits: [Nat; PITS_PER_PLAYER],
}

impl PlayerState {
    fn moves_iter(&self, pie_rule_active: bool) -> PlayerMoveIterator {
        PlayerMoveIterator {
            pie_rule: pie_rule_active,
            state: *self,
            index: 0,
        }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct BoardState {
    pub north: PlayerState,
    pub south: PlayerState,
}

/// South always starts the game, so "South" means the agent is the first player
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Position {
    North,
    South,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PlayerMove {
    Move { n: Nat },
    Swap,
}

impl ToString for PlayerMove {
    fn to_string(&self) -> String {
        match *self {
            Self::Swap => "SWAP\n".into(),
            Self::Move { n } => format!("MOVE;{}\n", n + 1),
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

struct PlayerMoveIterator {
    pie_rule: bool,
    state: PlayerState,
    index: usize,
}

impl Iterator for PlayerMoveIterator {
    type Item = PlayerMove;
    fn next(&mut self) -> Option<PlayerMove> {
        let possible_move = if self.pie_rule {
            self.pie_rule = false;
            PlayerMove::Swap
        } else {
            // Skip empty holes
            loop {
                if self.index >= PITS_PER_PLAYER {
                    return None;
                }
                if self.state.pits[self.index] > 0 {
                    break;
                }
                self.index += 1;
            }
            self.index += 1;
            PlayerMove::Move {
                n: (self.index - 1) as Nat,
            }
        };
        Some(possible_move)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pie_rule() {
        let player_state = PlayerState {
            score: 0,
            pits: [7; PITS_PER_PLAYER],
        };

        assert_eq!(
            player_state.moves_iter(true).collect::<Vec<PlayerMove>>(),
            vec![
                PlayerMove::Swap,
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
            player_state.moves_iter(false).collect::<Vec<PlayerMove>>(),
            vec![]
        );
    }

    #[test]
    fn some_possible_moves() {
        let player_state = PlayerState {
            score: 0,
            pits: [0, 0, 4, 0, 2, 8, 0],
        };
        assert_eq!(
            player_state.moves_iter(false).collect::<Vec<PlayerMove>>(),
            vec![
                PlayerMove::Move { n: 2 },
                PlayerMove::Move { n: 4 },
                PlayerMove::Move { n: 5 },
            ]
        );
    }
}

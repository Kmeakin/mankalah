use std::{
    fmt,
    ops::{Index, IndexMut},
};
pub type Nat = u8;
pub const PITS_PER_PLAYER: usize = 7;
pub const TOTAL_PITS: usize = 2*(PITS_PER_PLAYER + 1);

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pie_rule() {
        let player_state = PlayerState::default();

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

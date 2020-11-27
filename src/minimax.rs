use crate::{
    agent::Agent,
    board::{BoardState, Nat, PlayerMove, Position, PITS_PER_PLAYER},
};
use std::ops::Not;

pub type Value = u32;
pub trait Heuristic = FnMut(BoardState) -> Value;

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

impl BoardState {
    #[cfg(FALSE)]
    fn clockwise_iter(
        &mut self,
        position: Position,
        n: usize,
    ) -> Box<dyn Iterator<Item = &mut u8> + '_> {
        debug_assert!(n <= PITS_PER_PLAYER);

        match position {
            Position::South => Box::new(
                &mut self
                    .south
                    .pits
                    .iter_mut()
                    .chain(std::iter::once(&mut self.south.score))
                    .chain(self.north.pits.iter_mut().rev())
                    .skip(n),
            ),
            Position::North => Box::new(
                &mut self
                    .north
                    .pits
                    .iter_mut()
                    .rev()
                    .chain(std::iter::once(&mut self.north.score))
                    .chain(self.south.pits.iter_mut())
                    .skip(7 - n),
            ),
        }
    }

    fn sow_seeds(&mut self, pos: Position, n: Nat) -> (Self, Position) {
        let n = n as usize;
        let mut stones_left = self[pos].pits[n];
        debug_assert!(stones_left > 0);

        // first loop: skip first n pits
        self[pos].pits.iter().skip(n)

        while stones_left > 0 {

        }
        (*self, pos)
    }

    fn apply_move(&mut self, moove: PlayerMove, position: Position) -> (Self, Position) {
        match moove {
            PlayerMove::Move { n } => self.sow_seeds(position, n),
            PlayerMove::Swap => {
                std::mem::swap(&mut self.north, &mut self.south);
                (*self, position)
            }
        }
    }

    fn child_boards(&mut self, position: Position) { todo!() }
}

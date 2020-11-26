use crate::protocol::{Nat, Position, PITS_PER_PLAYER, MoveSwap};
use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct PlayerState {
    pub score: Nat,
    pub pits: [Nat; PITS_PER_PLAYER],
}

impl PlayerState {
  fn get_moves(&self, pie_rule_active: bool) -> PlayerMoveIterator {
    PlayerMoveIterator {
      pie_rule: pie_rule_active,
      state: *self,
      index: 0
    }
  }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct BoardState {
    pub north: PlayerState,
    pub south: PlayerState,
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
  index: usize
}

impl Iterator for PlayerMoveIterator {
  type Item = MoveSwap;
  fn next(&mut self) -> Option<MoveSwap> {
    let possible_move = if self.pie_rule {
      self.pie_rule = false;
      MoveSwap::Swap
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
      MoveSwap::Move {n: (self.index - 1) as Nat }
    };
    Some(possible_move)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_pie_rule() {
    let player_state = PlayerState{ score: 0, pits: [7; PITS_PER_PLAYER]};

    assert_eq!(
      player_state.get_moves(true).collect::<Vec<MoveSwap>>(),
      vec![
        MoveSwap::Swap,
        MoveSwap::Move{ n: 0 },
        MoveSwap::Move{ n: 1 },
        MoveSwap::Move{ n: 2 },
        MoveSwap::Move{ n: 3 },
        MoveSwap::Move{ n: 4 },
        MoveSwap::Move{ n: 5 },
        MoveSwap::Move{ n: 6 },
      ]);
  }

  #[test]
  fn test_no_possible_moves() {
    let player_state = PlayerState{ score: 0, pits: [0; PITS_PER_PLAYER]};

    assert_eq!(
      player_state.get_moves(false).collect::<Vec<MoveSwap>>(),
      vec![]);
  }

  #[test]
  fn some_possible_moves() {
    let player_state = PlayerState { score: 0, pits: [0, 0, 4, 0, 2, 8, 0]};
    assert_eq!(
      player_state.get_moves(false).collect::<Vec<MoveSwap>>(),
      vec![
        MoveSwap::Move{ n: 2 },
        MoveSwap::Move{ n: 4 },
        MoveSwap::Move{ n: 5 },
      ]);
  }
}

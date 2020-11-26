use crate::board::BoardState;

pub type Nat = u8;
pub const PITS_PER_PLAYER: usize = 7;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EngineMessage {
    NewMatch {
        pos: Position,
    },
    StateChange {
        move_or_swap: MoveSwap,
        state: BoardState,
        turn: Turn,
    },
    GameOver,
}

pub type MoveSwap = AgentMessage;
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AgentMessage {
    Move { n: Nat },
    Swap,
}

/// South always starts the game, so "South" means the agent is the first player
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Position {
    North,
    South,
}

impl ToString for MoveSwap {
    fn to_string(&self) -> String {
        match *self {
            Self::Swap => "SWAP\n".into(),
            Self::Move { n } => format!("MOVE;{}\n", n + 1),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Turn {
    You,
    Opponent,
    End,
}

use crate::board::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EngineMessage {
    NewMatch {
        pos: Position,
    },
    StateChange {
        player_move: PlayerMove,
        state: BoardState,
        turn: Turn,
    },
    GameOver,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Turn {
    You,
    Opponent,
    End,
}

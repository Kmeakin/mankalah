#![allow(non_snake_case)]

use super::{board::*, protocol::*};
use std::{convert::TryInto, str::FromStr};

peg::parser! {
    pub grammar ProtocolGrammar() for str {
        rule Nat() -> Nat
            = n: $(['0'..='9']+) {Nat::from_str(n).unwrap()}

        /// Messages sent from the engine to the agent
        pub rule EngineMessage() -> EngineMessage
            = NewMatch() / StateChange() / GameOver()

        rule NewMatch() -> EngineMessage
            = "START" ";" pos: Position() "\n"
            {EngineMessage::NewMatch{pos}}

        rule Position() -> Position
            = "North" {Position::North}
            / "South" {Position::South}

        rule StateChange() -> EngineMessage
            = "CHANGE" ";" move_or_swap: MoveSwap() ";" state: State() ";" turn: Turn() "\n"
            {EngineMessage::StateChange{move_or_swap, state, turn}}

        rule MoveSwap() -> MoveSwap
            = n: Nat() {MoveSwap::Move{n: n - 1}}
            / "SWAP" {MoveSwap::Swap}

        rule State() -> BoardState
            = north: PlayerState() "," south: PlayerState()
            {BoardState{north, south}}

        rule PlayerState() -> PlayerState
            = pits: Nat() ** <{PITS_PER_PLAYER}> ","
              "," score: Nat()
              {PlayerState{pits: pits.try_into().unwrap(), score}}

        rule Turn() -> Turn
            = "YOU" {Turn::You}
            / "OPP" {Turn::Opponent}
            / "END" {Turn::End}

        rule GameOver() -> EngineMessage
            = "END" "\n"
            {EngineMessage::GameOver}

        /// Messages sent from the agent to the engine
        pub rule AgentMessage() -> AgentMessage
            = AgentMove() / AgentSwap()

        rule AgentMove() -> AgentMessage
            = "MOVE" ";" n: Nat() "\n"
            {AgentMessage::Move{n}}

        rule AgentSwap() -> AgentMessage
            = "SWAP" "\n"
            {AgentMessage::Swap}
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use peg::{error::ParseError, str::LineCol};

    #[track_caller]
    fn test_engine_message(input: &str, expected: Result<EngineMessage, ParseError<LineCol>>) {
        let got = ProtocolGrammar::EngineMessage(input);
        assert_eq!(got, expected);
    }

    #[test]
    fn new_match_north() {
        test_engine_message(
            "START;North\n",
            Ok(EngineMessage::NewMatch {
                pos: Position::North,
            }),
        )
    }

    #[test]
    fn new_match_south() {
        test_engine_message(
            "START;South\n",
            Ok(EngineMessage::NewMatch {
                pos: Position::South,
            }),
        )
    }

    #[test]
    fn state_change_swap() {
        test_engine_message(
            "CHANGE;SWAP;1,2,3,4,5,6,7,99,7,6,5,4,3,2,1,99;YOU\n",
            Ok(EngineMessage::StateChange {
                move_or_swap: MoveSwap::Swap,
                state: BoardState {
                    north: PlayerState {
                        pits: [1, 2, 3, 4, 5, 6, 7],
                        score: 99,
                    },
                    south: PlayerState {
                        pits: [7, 6, 5, 4, 3, 2, 1],
                        score: 99,
                    },
                },
                turn: Turn::You,
            }),
        )
    }

    #[test]
    fn state_change_move() {
        test_engine_message(
            "CHANGE;1;1,2,3,4,5,6,7,99,7,6,5,4,3,2,1,99;OPP\n",
            Ok(EngineMessage::StateChange {
                move_or_swap: MoveSwap::Move { n: 1 },
                state: BoardState {
                    north: PlayerState {
                        pits: [1, 2, 3, 4, 5, 6, 7],
                        score: 99,
                    },
                    south: PlayerState {
                        pits: [7, 6, 5, 4, 3, 2, 1],
                        score: 99,
                    },
                },
                turn: Turn::Opponent,
            }),
        )
    }

    #[test]
    fn state_change_end() {
        test_engine_message(
            "CHANGE;1;1,2,3,4,5,6,7,99,7,6,5,4,3,2,1,99;END\n",
            Ok(EngineMessage::StateChange {
                move_or_swap: MoveSwap::Move { n: 1 },
                state: BoardState {
                    north: PlayerState {
                        pits: [1, 2, 3, 4, 5, 6, 7],
                        score: 99,
                    },
                    south: PlayerState {
                        pits: [7, 6, 5, 4, 3, 2, 1],
                        score: 99,
                    },
                },
                turn: Turn::End,
            }),
        )
    }

    #[test]
    fn game_over() { test_engine_message("END\n", Ok(EngineMessage::GameOver)) }

    #[track_caller]
    fn test_agent_message(input: &str, expected: Result<AgentMessage, ParseError<LineCol>>) {
        let got = ProtocolGrammar::AgentMessage(input);
        assert_eq!(got, expected);
    }

    #[test]
    fn agent_move() { test_agent_message("MOVE;10\n", Ok(AgentMessage::Move { n: 10 })) }

    #[test]
    fn agent_swap() { test_agent_message("SWAP\n", Ok(AgentMessage::Swap)) }
}

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
            = "CHANGE" ";" player_move: PlayerMove() ";" state: State() ";" turn: Turn() "\n"
            {EngineMessage::StateChange{player_move, state, turn}}

        rule PlayerMove() -> PlayerMove
            // player moves are 1-based
            = n: Nat() {PlayerMove::Move{n: n - 1}}
            / "SWAP" {PlayerMove::Swap}

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
                player_move: PlayerMove::Swap,
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
                player_move: PlayerMove::Move { n: 0 },
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
                player_move: PlayerMove::Move { n: 0 },
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
}

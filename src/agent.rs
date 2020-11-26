use crate::grammar::ProtocolGrammar;
use std::io::BufRead;

use crate::{board::*, protocol::*};

fn read_line() -> String {
    let mut line = String::new();
    let stdin = std::io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line
}

fn send_move(chosen_move: PlayerMove) {
    println!("{}", chosen_move);
}

fn read_engine_message() -> EngineMessage {
    let line = read_line();
    ProtocolGrammar::EngineMessage(&line).unwrap()
}

#[derive(Debug, Copy, Clone)]
pub struct Agent {
    position: Position,
    state: BoardState,
    first_move: bool,
}

impl Agent {
    pub fn new() -> Self {
        Agent {
            position: Position::South,
            state: BoardState::default(),
            first_move: true,
        }
    }

    fn our_state(&self) -> PlayerState { self.state[self.position] }

    fn make_move(&mut self) {
        let pie_rule_active = self.first_move && self.position == Position::North;
        let chosen_move = self.our_state().moves_iter(pie_rule_active).next().unwrap();
        if let PlayerMove::Swap = chosen_move {
            self.swap_sides();
        }
        send_move(chosen_move);
        self.first_move = false;
    }

    fn swap_sides(&mut self) {
        self.position = match self.position {
            Position::North => Position::South,
            Position::South => Position::North,
        }
    }

    pub fn run(&mut self) {
        let mut message = read_engine_message();
        match message {
            EngineMessage::NewMatch { pos } => {
                self.position = pos;
                if pos == Position::South {
                    self.make_move();
                }
            }
            EngineMessage::GameOver => {
                return;
            }
            _ => unreachable!(),
        }

        loop {
            message = read_engine_message();
            let our_turn;
            match message {
                EngineMessage::GameOver => {
                    return;
                }
                EngineMessage::StateChange {
                    player_move,
                    state,
                    turn,
                } => {
                    self.state = state;
                    match player_move {
                        PlayerMove::Swap => {
                            self.swap_sides();
                        }
                        PlayerMove::Move { .. } => { /* Ignore their move for now */ }
                    }

                    our_turn = match turn {
                        Turn::You => true,
                        Turn::Opponent => false,
                        Turn::End => {
                            return;
                        }
                    }
                }
                _ => unreachable!(),
            }
            if our_turn {
                self.make_move();
            }
        }
    }
}

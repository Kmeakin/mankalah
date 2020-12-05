use crate::{
    board::{BoardState, PlayerMove, PlayerState, Position},
    grammar::ProtocolGrammar,
    protocol::*,
};
use std::io::BufRead;

fn read_line() -> String {
    let mut line = String::new();
    let stdin = std::io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line
}

fn send_move(chosen_move: PlayerMove) {
    print!("{}", chosen_move);
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

impl Default for Agent {
    fn default() -> Self {
        Self {
            position: Position::South,
            state: BoardState::default(),
            first_move: true,
        }
    }
}

impl Agent {
    pub fn new() -> Self { Self::default() }

    pub fn our_state(&self) -> PlayerState { self.state[self.position] }

    pub fn can_swap(&self) -> bool { self.first_move && self.position == Position::North }

    fn make_move(&mut self) {
        let pie_rule_active = self.can_swap();
        let state = self.our_state();
        let pos = self.position;
        let potential_moves = state.moves_iter(pie_rule_active).map(|the_move| {
            let (board, pos) = self.state.apply_move(the_move, self.position);
            let score = board.minimax(&crate::heuristics::current_score, pos, pie_rule_active, 0);
            (the_move, score)
        });
        let (chosen_move, _score) = match pos {
            Position::South => potential_moves.max_by_key(|&(_, score)| score),
            Position::North => potential_moves.min_by_key(|&(_, score)| score),
        }
        .unwrap();

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

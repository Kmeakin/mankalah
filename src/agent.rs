use crate::{
    board::{BoardState, PlayerMove, PlayerState, Position},
    eval::Evaluator,
    grammar::ProtocolGrammar,
    heuristics::{Score, Weights},
    protocol::*,
};
use std::{
    io::BufRead,
    time::{Duration, Instant},
};

fn read_line() -> String {
    let mut line = String::new();
    let stdin = std::io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line
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

    fn send_move(&mut self, chosen_move: PlayerMove) {
        // if let PlayerMove::Move { .. } = chosen_move {
        //   self.state.apply_move( chosen_move, self.position, true);
        // }
        print!("{}", chosen_move);
    }

    fn get_move<E: Evaluator>(&self, max_depth: usize, weights: Weights) -> PlayerMove {
        let (chosen_move, _) = E::eval(
            self.state,
            self.position,
            0,
            self.first_move,
            max_depth,
            weights,
        );
        chosen_move.unwrap()
    }

    fn make_move<E: Evaluator>(&mut self, max_depth: usize, weights: Weights) -> bool {
        let start = Instant::now();
        // let mut depth = 5;
        // let chosen_move = loop {
        //   let picked_move = self.get_move::<E>(depth, weights);
        //   if start.elapsed().as_secs() > 3 || depth >= 30 {
        //     break picked_move;
        //   }
        //   depth += 1;
        // };
        log::debug!("Getting move: pos = {:?}", self.position);
        let chosen_move = self.get_move::<E>(max_depth, weights);
        log::debug!("chosen_move = {:?}", chosen_move);

        // Does not look like the engine tells us if we swap
        let swapped = if let PlayerMove::Swap = chosen_move {
            self.swap_sides();
            log::debug!("Done swap {:?}", self.position);
            true
        } else {
            false
        };
        self.send_move(chosen_move);
        self.first_move = false;
        swapped
    }

    fn swap_sides(&mut self) {
        self.position = match self.position {
            Position::North => Position::South,
            Position::South => Position::North,
        }
    }

    fn set_state(&mut self, engine_state: BoardState) {
        log::debug!(
            "our state: {:?}, engine_state: {:?}",
            self.state,
            engine_state
        );
        assert_eq!(self.state, engine_state);
        self.state = engine_state;
    }

    pub fn run<E: Evaluator>(&mut self, max_depth: usize, weights: Weights) {
        let mut message = read_engine_message();
        let mut was_our_move = false;
        match message {
            EngineMessage::NewMatch { pos } => {
                self.position = pos;
                if pos == Position::South {
                    self.make_move::<E>(max_depth, weights);
                    was_our_move = true;
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
                    log::debug!("state: {:?}", self.state);
                    log::debug!("our pos {:?}", self.position);
                    match player_move {
                        PlayerMove::Swap => {
                            log::debug!("swappy?");
                            self.swap_sides();
                        }
                        PlayerMove::Move { .. } => {
                            let move_pos = if was_our_move {
                                self.position
                            } else {
                                !self.position
                            };
                            log::debug!(
                                "here? {:?} {:?} -- us {:?} {:?}",
                                player_move,
                                move_pos,
                                self.position,
                                was_our_move
                            );
                            self.state.apply_move(player_move, move_pos, false);
                        }
                    }
                    self.set_state(state);

                    our_turn = match turn {
                        Turn::You => true,
                        Turn::Opponent => {
                            was_our_move = false;
                            false
                        }
                        Turn::End => {
                            return;
                        }
                    }
                }
                _ => unreachable!(),
            }
            if our_turn {
                was_our_move = !self.make_move::<E>(max_depth, weights);
            }
        }
    }
}

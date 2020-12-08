use crate::{
    board::{BoardState, PlayerMove, PlayerState, Position},
    grammar::ProtocolGrammar,
    heuristics::{Heuristic, Score},
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

pub trait Evaluator<H: Heuristic> {
    fn eval(board: BoardState, pos: Position, depth: usize, first_move: bool) -> Score;
}

#[derive(Debug, Copy, Clone)]
pub enum MiniMax {}

impl<H: Heuristic> Evaluator<H> for MiniMax {
    fn eval(board: BoardState, pos: Position, depth: usize, first_move: bool) -> Score {
        board.minimax::<H>(pos, depth)
    }
}
#[derive(Debug, Copy, Clone)]
pub enum AlphaBeta {}

impl<H: Heuristic> Evaluator<H> for AlphaBeta {
    fn eval(board: BoardState, pos: Position, depth: usize, first_move: bool) -> Score {
        let alpha = Score::MIN;
        let beta = Score::MAX;
        board.alpha_beta::<H>(depth, alpha, beta, pos, first_move)
    }
}

impl Agent {
    pub fn new() -> Self { Self::default() }

    pub fn our_state(&self) -> PlayerState { self.state[self.position] }

    pub fn can_swap(&self) -> bool { self.first_move && self.position == Position::North }

    // fn do_first_move<H: Heuristic, E: Evaluator<H>>(&mut self) {
    //     let player_state = self.our_state();
    //     let potential_moves = player_state.moves_iter();
    //     let (chosen_move, score) = match self.position {
    //         Position::South => {
    //             potential_moves
    //                 .map(|the_move| {
    //                     // Our move: next pos ALWAYS north
    //                     let (board, _next_pos) = self.state.do_move(the_move,
    // Position::South);

    //                     // North's first moves
    //                     let norths_moves = self.state[Position::North]
    //                         .moves_iter()
    //                         .chain(Some(PlayerMove::Swap));

    //                       E::eval(board, pos: Position::North, first_move)
    //                     let norths_score = norths_moves
    //                         .map(|north_move| {
    //                             let (child_board, next_pos) =
    //                                 board.do_move(north_move, Position::North);
    //                             E::eval(child_board, next_pos, 0)
    //                         })
    //                         .min()
    //                         .unwrap();
    //                     (the_move, norths_score)
    //                 })
    //                 .max_by_key(|&(the_move, score)| score)
    //                 .unwrap()
    //         }
    //         Position::North => potential_moves
    //             .chain(Some(PlayerMove::Swap))
    //             .map(|the_move| {
    //                 let (board, next_pos) = self.state.do_move(the_move,
    // Position::North);                 (the_move, E::eval(board, next_pos, 0))
    //             })
    //             .min_by_key(|&(the_move, score)| dbg!(score))
    //             .unwrap(),
    //     };
    //     if let PlayerMove::Swap = chosen_move {
    //         self.swap_sides();
    //     }
    //     send_move(chosen_move);
    // }

    fn make_move<H: Heuristic, E: Evaluator<H>>(&mut self) {
        let player_state = self.our_state();
        let moves =  player_state.moves_iter();
        let moves = if self.first_move && self.position == Position::North {
          moves.chain(Some(PlayerMove::Swap))
        } else {
          moves.chain(None)
        };

        let potential_moves = moves.map(|the_move| {
            let (board, next_pos, next_first_move) =
                self.state.do_move(the_move, self.position, self.first_move);
            let score = E::eval(board, next_pos, 0, next_first_move);
            (the_move, score)
        });
        let (chosen_move, _score) = match self.position {
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

    pub fn run<H: Heuristic, E: Evaluator<H>>(&mut self) {
        let mut message = read_engine_message();
        match message {
            EngineMessage::NewMatch { pos } => {
                self.position = pos;
                if pos == Position::South {
                    self.make_move::<H, E>();
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
                self.make_move::<H, E>();
            }
        }
    }
}

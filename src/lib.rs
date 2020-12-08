#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    rust_2018_idioms,
    clippy::all
)]
#![deny(bare_trait_objects)]

pub mod agent;
pub mod alpha_beta;
pub mod board;
pub mod grammar;
pub mod heuristics;
pub mod minimax;
pub mod protocol;

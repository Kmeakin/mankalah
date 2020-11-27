#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    rust_2018_idioms,
    clippy::all
)]
#![deny(bare_trait_objects)]
#![feature(trait_alias)]

pub mod agent;
pub mod board;
pub mod grammar;
pub mod protocol;
pub mod minimax;

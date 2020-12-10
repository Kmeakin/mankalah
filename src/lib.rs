#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    rust_2018_idioms,
    clippy::all
)]
#![deny(bare_trait_objects)]
#![feature(format_args_capture)]

// TODOS:
// [x] more heuristics
// [ ] parralelism
// [x] benchmarking script to detect regressions
// [ ] presentation
// [ ] heuristic weights

pub mod agent;
pub mod benchmark;
pub mod board;
pub mod eval;
pub mod grammar;
pub mod heuristics;
pub mod protocol;

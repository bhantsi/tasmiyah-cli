//! Library root for `tasmiyah-cli`.
//!
//! The real logic lives in the binary's modules; this thin lib exists
//! so build-time tooling (the `gen` binary that produces shell
//! completions and the man page) can build the `clap` `Command` without
//! duplicating the argument definitions.

pub mod banner;
pub mod cli;
pub mod config;
pub mod font;
pub mod phrases;
pub mod terminal;
pub mod theme;
pub mod update;

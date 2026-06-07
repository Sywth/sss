#![allow(dead_code)]

pub mod args;
pub mod error;
pub mod solver;
pub mod util;

pub use error::CliError;
pub use util::as_colored;

pub use structures::containers::{ClauseDisjunctiveBasic, FormulaConjunctiveBasic};

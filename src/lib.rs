#![allow(dead_code)]

pub mod parser;
pub mod sat;
pub mod util;

pub use parser::{parse_dimacs_file, FormulaTranslator};
pub use sat::SatFormula;
pub use structures::containers::{ClauseDisjunctiveBasic, FormulaConjunctiveBasic};
pub use util::as_colored;

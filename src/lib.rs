pub mod parser;
pub mod sat;
pub mod structures;

pub use parser::parse_dimacs_file;
pub use structures::FormulaConjunctive;

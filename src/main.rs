use sss::{parse_dimacs_file, FormulaTranslator, SatFormula};
use std::path::Path;
use std::process::ExitCode;
use tracing::{debug, error, Level};

// More of the front end
const STR_ERROR: &str = "ERR";
const STR_SAT: &str = "SAT";
const STR_UNSAT: &str = "UNSAT";

fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_max_level(Level::ERROR)
        .with_ansi(true)
        .init();

    let mut args = std::env::args();
    let this_script = args.next().unwrap_or_else(|| "main".into());
    let dimacs_path = match args.next() {
        Some(p) => p,
        None => {
            eprintln!("Usage: {this_script} <dimacs_file>");
            println!("{}", STR_ERROR);
            return ExitCode::from(1);
        }
    };

    let fp_dimacs = Path::new(&dimacs_path);

    let formula: FormulaTranslator = match parse_dimacs_file(fp_dimacs) {
        Ok(f) => f,
        Err(e) => {
            error!(path = ?fp_dimacs, error = ?e,);
            println!("{}", STR_ERROR);
            return ExitCode::from(1);
        }
    };

    debug!("parsed formula as {:#?}", formula);
    let result = match formula.is_sat() {
        true => STR_SAT,
        false => STR_UNSAT,
    };
    println!("{}", result);

    ExitCode::from(0)
}

// --------------------------------
// Unit Tests
// --------------------------------

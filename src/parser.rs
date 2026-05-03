use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use tracing::{debug, error};

use crate::structures::{FormulaTranslator, SwInt};

pub fn parse_dimacs_file<T: SwInt, P: AsRef<Path>>(
    fp: P,
) -> Result<FormulaTranslator<T, u32>, std::io::Error> {
    let file = File::open(fp)?;
    let reader = std::io::BufReader::new(file);

    let mut disjunction_stack: Vec<T> = Vec::new();
    let mut conjunctions: Vec<Vec<T>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut it = line.split_whitespace().peekable();

        match it.peek() {
            Some(&"c") => {
                debug!(comment = %line);
                continue;
            }
            Some(&"p") => {
                debug!(header = %line);
                continue;
            }
            None => {
                debug!("skipping empty line");
            }
            _ => (),
        }

        for token in it {
            match token.parse::<T>() {
                Ok(value) => {
                    debug!("parsed {} as an integer", value);
                    if value != T::zero() {
                        disjunction_stack.push(value);
                        continue;
                    }

                    conjunctions.push(disjunction_stack.clone());
                    disjunction_stack.clear();
                }
                Err(_) => {
                    error!(token = token, "failed to parse token");
                }
            }
        }
    }

    Ok(FormulaTranslator::new(conjunctions))
}

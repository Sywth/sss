use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use tracing::{debug, error};

use num_traits::{PrimInt, Signed, ToPrimitive};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
    str::FromStr,
};
use tracing::field::debug;

use crate::{structures::SwUint, FormulaConjunctive};

// Pure front end here. Only goal is get data into shape for backend. Performance cost is not
// important here
pub trait SwInt: PrimInt + Signed + Hash + FromStr + Display + Debug + ToPrimitive {}
impl<T> SwInt for T where T: PrimInt + Signed + Hash + FromStr + Display + Debug + ToPrimitive {}

#[derive(Debug)]
pub struct FormulaTranslator<K: SwInt, V: SwUint> {
    pub dimacs_id_to_sw_id: HashMap<K, V>,
    pub cnf: FormulaConjunctive<V>,
}

/// K is the front end id type (e.g. i32)
/// V is the solver's type for atoms probably (e.g. u32)
impl<K: SwInt, V: SwUint> FormulaTranslator<K, V> {
    /// Given some 2D iterator e.g. [[526,334], [-334,3,74]]  we should be able
    /// spit back a normalized formula like so
    /// [[1,2], [-2, 3, 4]], {526: 1, 334: 2, 3: 3, 74: 4}
    pub fn new<I: IntoIterator<Item = J>, J: IntoIterator<Item = K>>(_clause_iterator: I) -> Self {
        debug("TODO Implement this man");

        Self {
            dimacs_id_to_sw_id: HashMap::new(),
            cnf: FormulaConjunctive {
                clauses: Vec::new(),
            },
        }
    }
}

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

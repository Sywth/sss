use std::io::BufRead;
use std::path::Path;
use std::{collections::hash_map, fs::File};
use tracing::{debug, error};

use crate::{structures::SwUint, FormulaConjunctive};
use num_traits::{PrimInt, Signed, ToPrimitive};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
    str::FromStr,
};

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
    pub fn new<I: IntoIterator<Item = J> + Clone, J: IntoIterator<Item = K>>(
        clause_iterator: I,
    ) -> Self {
        let mut dimacs_id_to_sw_id: HashMap<K, V> = HashMap::new();
        let mut curr_sw_id = V::one();

        for disjunction in clause_iterator.clone().into_iter() {
            for k_litearl in disjunction.into_iter() {
                let k_atom = k_litearl.abs();

                if let hash_map::Entry::Vacant(e) = dimacs_id_to_sw_id.entry(k_atom) {
                    e.insert(curr_sw_id);
                    curr_sw_id = curr_sw_id.add(V::one());
                }
            }
        }

        let mut clauses: Vec<Vec<(V, bool)>> = Vec::new();
        for disjunction in clause_iterator.into_iter() {
            let mut litearls: Vec<(V, bool)> = Vec::new();
            for k_litearl in disjunction.into_iter() {
                let k_atom = k_litearl.abs();
                let v_atom = *dimacs_id_to_sw_id.get(&k_atom).expect("bad logic");

                let polarity: bool = match k_litearl.signum().to_i8().unwrap() {
                    1 => true,
                    -1 => false,
                    _ => panic!("malformed formula, found literal with value 0"),
                };
                litearls.push((v_atom, polarity));
            }
            clauses.push(litearls);
        }

        let cnf = FormulaConjunctive::new(clauses);
        Self {
            dimacs_id_to_sw_id,
            cnf,
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
                // value is int+
                Ok(value) if value != T::zero() => {
                    disjunction_stack.push(value);
                }
                // value == 0
                Ok(_) => {
                    // OPTIMIZATION: You can replace this with a mem take?
                    conjunctions.push(disjunction_stack.clone());
                    disjunction_stack.clear();
                }
                // value cannot be parsed
                Err(_) => {
                    error!(token = token, "failed to parse token");
                }
            }
        }
    }

    Ok(FormulaTranslator::new(conjunctions))
}

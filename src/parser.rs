use std::io::BufRead;
use std::path::Path;
use std::{collections::hash_map, fs::File};
use tracing::{debug, error};

use std::collections::HashMap;
use structures::containers::FormulaConjunctiveBasic;
use structures::primitives::{FLiteral, SAtom};

//pub enum Formula<VarType, FomrulaIdType> {
//    Top,
//    Bot,
//
//    Con(FomrulaIdType, FomrulaIdType),
//    Dis(FomrulaIdType, FomrulaIdType),
//
//    Imp(FomrulaIdType, FomrulaIdType),
//
//    ForAll(VarType, FomrulaIdType),
//    Exists(VarType, FomrulaIdType),
//}

#[derive(Debug)]
pub struct FormulaTranslator {
    pub dimacs_id_to_sw_id: HashMap<FLiteral, SAtom>,
    pub cnf: FormulaConjunctiveBasic,
}

impl FormulaTranslator {
    /// Given some 2D iterator e.g. [[526,334], [-334,3,74]]  we should be able
    /// spit back a normalized formula like so
    /// [[1,2], [-2, 3, 4]], {526: 1, 334: 2, 3: 3, 74: 4}
    pub fn new<I: IntoIterator<Item = J> + Clone, J: IntoIterator<Item = FLiteral>>(
        clause_iterator: I,
    ) -> Self {
        let mut dimacs_id_to_sw_id: HashMap<FLiteral, SAtom> = HashMap::new();
        let mut curr_sw_id = SAtom::ONE;

        for disjunction in clause_iterator.clone().into_iter() {
            for k_litearl in disjunction.into_iter() {
                let k_atom = k_litearl.abs();

                if let hash_map::Entry::Vacant(e) = dimacs_id_to_sw_id.entry(k_atom) {
                    e.insert(curr_sw_id);
                    curr_sw_id = curr_sw_id + SAtom::ONE;
                }
            }
        }

        let mut clauses: Vec<Vec<(SAtom, bool)>> = Vec::new();
        for disjunction in clause_iterator.into_iter() {
            let mut litearls: Vec<(SAtom, bool)> = Vec::new();
            for k_litearl in disjunction.into_iter() {
                let k_atom = k_litearl.abs();
                let v_atom = *dimacs_id_to_sw_id.get(&k_atom).expect("bad logic");

                let polarity = match k_litearl {
                    n if n > FLiteral::ZERO => true,
                    n if n < FLiteral::ZERO => false,
                    _ => panic!("malformed formula, found literal with value 0"),
                };
                litearls.push((v_atom, polarity));
            }
            clauses.push(litearls);
        }

        let cnf = FormulaConjunctiveBasic::new(clauses);
        Self {
            dimacs_id_to_sw_id,
            cnf,
        }
    }
}

pub fn parse_dimacs_file<P: AsRef<Path>>(fp: P) -> Result<FormulaTranslator, std::io::Error> {
    let file = File::open(fp)?;
    let reader = std::io::BufReader::new(file);

    let mut disjunction_stack: Vec<FLiteral> = Vec::new();
    let mut conjunctions: Vec<Vec<FLiteral>> = Vec::new();

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
            match token.parse::<FLiteral>() {
                // value is positive natural
                Ok(value) if value != FLiteral::ZERO => {
                    disjunction_stack.push(value);
                }
                // value is 0
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

// --------------------------------
// Unit Tests
// --------------------------------

#[test]
#[cfg(test)]
fn translator_normalizes_polarity() {
    let formula = FormulaTranslator::new(vec![
        vec![FLiteral::from(5), FLiteral::from(-322)],
        vec![FLiteral::from(17)],
    ]);
    let sw_1 = *formula.dimacs_id_to_sw_id.get(&FLiteral::from(5)).unwrap();
    let sw_2 = *formula
        .dimacs_id_to_sw_id
        .get(&FLiteral::from(322))
        .unwrap();
    let sw_3 = *formula.dimacs_id_to_sw_id.get(&FLiteral::from(17)).unwrap();
    let clauses: Vec<Vec<(SAtom, bool)>> = formula
        .cnf
        .clone()
        .into_iter()
        .map(|c| c.into_iter().collect())
        .collect();
    assert_eq!(clauses[0], vec![(sw_1, true), (sw_2, false)]);
    assert_eq!(clauses[1], vec![(sw_3, true)]);
}

#[test]
#[cfg(test)]
fn translator_polarity_unsat() {
    use crate::sat::SatFormula;
    // (1) ∧ (¬1) is unsat, basic polarity test
    let formula = FormulaTranslator::new(vec![vec![FLiteral::from(1)], vec![FLiteral::from(-1)]]);
    assert!(!formula.cnf.is_sat());
}

#[test]
#[cfg(test)]
fn translator_large_id_normalized() {
    let formula = FormulaTranslator::new(vec![vec![FLiteral::from(242), FLiteral::from(-1)]]);
    let sw_242 = *formula
        .dimacs_id_to_sw_id
        .get(&FLiteral::from(242))
        .unwrap();
    let sw_1 = *formula.dimacs_id_to_sw_id.get(&FLiteral::from(1)).unwrap();
    let clauses: Vec<Vec<(SAtom, bool)>> = formula
        .cnf
        .clone()
        .into_iter()
        .map(|c| c.into_iter().collect())
        .collect();
    assert_eq!(clauses[0], vec![(sw_242, true), (sw_1, false)]);
}

#[test]
#[cfg(test)]
fn translator_0v_0c_no_clauses() {
    let formula = FormulaTranslator::new(Vec::<Vec<FLiteral>>::new());
    assert_eq!(formula.cnf.into_iter().count(), 0);
    assert!(formula.dimacs_id_to_sw_id.is_empty());
}

#[test]
#[cfg(test)]
fn translator_0v_1c_empty_clause() {
    let formula = FormulaTranslator::new(vec![Vec::<FLiteral>::new()]);
    let clauses: Vec<Vec<(SAtom, bool)>> = formula
        .cnf
        .into_iter()
        .map(|c| c.into_iter().collect())
        .collect();
    assert_eq!(clauses.len(), 1);
    assert!(clauses[0].is_empty());
    assert!(formula.dimacs_id_to_sw_id.is_empty());
}

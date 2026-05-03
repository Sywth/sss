use num_traits::{PrimInt, Signed, ToPrimitive, Unsigned};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
    str::FromStr,
};
use tracing::field::debug;

// Backend

pub trait SwUint: PrimInt + Unsigned + Hash + FromStr + Display + Debug + ToPrimitive {}
impl<T> SwUint for T where T: PrimInt + Unsigned + Hash + FromStr + Display + Debug + ToPrimitive {}

#[derive(Debug)]
pub struct ClauseDisjunctive<T: SwUint> {
    // Raw propositional atoms
    pub atoms: Vec<T>,
    // Truth value required by the atom for for this clause to be satisfied
    pub truthiness: Vec<bool>,
}

impl<T: SwUint> ClauseDisjunctive<T> {
    pub fn new(atoms: Vec<T>, truthiness: Vec<bool>) -> Self {
        debug_assert_eq!(atoms.len(), truthiness.len());
        Self { atoms, truthiness }
    }
}

#[derive(Debug)]
pub struct FormulaConjunctive<T: SwUint> {
    pub clauses: Vec<ClauseDisjunctive<T>>,
}

impl<T: SwUint> FormulaConjunctive<T> {
    pub fn new<I: IntoIterator<Item = J>, J: IntoIterator<Item = (T, bool)>>(
        clause_iterator: I,
    ) -> Self {
        Self {
            clauses: clause_iterator
                .into_iter()
                .map(|clause| {
                    let (atoms, truthiness): (Vec<T>, Vec<bool>) = clause.into_iter().unzip();
                    ClauseDisjunctive::new(atoms, truthiness)
                })
                .collect(),
        }
    }
}

// Front End
pub trait SwInt: PrimInt + Signed + Hash + FromStr + Display + Debug + ToPrimitive {}
impl<T> SwInt for T where T: PrimInt + Signed + Hash + FromStr + Display + Debug + ToPrimitive {}

#[derive(Debug)]
pub struct FormulaTranslator<K: SwInt, V: SwUint> {
    pub dimacs_id_to_sw_id: HashMap<K, V>,
    pub cnf: FormulaConjunctive<V>,
}

/// K is the front end id type (e.g. string, i32, foo)
/// V is the solver's type for atoms probably u32
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

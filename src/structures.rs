use num_traits::{PrimInt, ToPrimitive, Unsigned};
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    str::FromStr,
};

// Pure backend. We care mainly about performance, software design is not important here.
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
        let clauses = clause_iterator
            .into_iter()
            .map(|clause| {
                let (atoms, truthiness): (Vec<T>, Vec<bool>) = clause.into_iter().unzip();
                ClauseDisjunctive::new(atoms, truthiness)
            })
            .collect();

        Self { clauses }
    }
}

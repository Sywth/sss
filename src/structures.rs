use num_traits::{PrimInt, ToPrimitive, Unsigned};
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    iter::{self},
    ops::{Index, IndexMut},
    str::FromStr,
    vec,
};

// Pure backend. We care mainly about performance, software design is not important here.
pub trait SwUint: PrimInt + Unsigned + Hash + FromStr + Display + Debug + ToPrimitive {}
impl<T> SwUint for T where T: PrimInt + Unsigned + Hash + FromStr + Display + Debug + ToPrimitive {}

#[inline]
pub fn get_atom_idx<T: SwUint>(atom: T) -> usize {
    (T::sub(atom, T::one()))
        .to_usize()
        .expect("given atom could not be cast to usize, was it 0?")
}

// -------------------------------
// Assignment
// -------------------------------
pub trait Assignment<T: SwUint>: Clone {
    fn new(size: usize) -> Self;
    fn get(&self, atom: T) -> &Option<bool>;
    fn is_set(&self, atom: T) -> bool;
    fn set(&mut self, atom: T, value: bool);
    fn clear(&mut self, atom: T);
}

#[derive(Debug, Clone)]
pub struct AssignmentBasic {
    gamma: Vec<Option<bool>>,
}

impl<T: SwUint> Assignment<T> for AssignmentBasic {
    fn new(size: usize) -> Self {
        AssignmentBasic {
            gamma: vec![None; size],
        }
    }

    fn get(&self, atom: T) -> &Option<bool> {
        let idx = get_atom_idx(atom);
        self.gamma
            .get(idx)
            .expect("bad logic, gamma should have a slot for every atom")
    }

    fn is_set(&self, atom: T) -> bool {
        let idx = get_atom_idx(atom);
        let Some(_) = self.gamma.get(idx) else {
            return false;
        };

        true
    }

    fn set(&mut self, atom: T, truthiness: bool) {
        let idx = get_atom_idx(atom);
        self.gamma.insert(idx, Some(truthiness));
    }

    fn clear(&mut self, atom: T) {
        let idx = get_atom_idx(atom);
        self.gamma.insert(idx, None);
    }
}

// -------------------------------
// Clause
// -------------------------------
pub trait ClauseDisjunctive<T: SwUint>:
    IntoIterator<Item = (T, bool)> + FromIterator<(T, bool)>
{
    fn iter(&self) -> impl Iterator<Item = (T, bool)>;
}

#[derive(Debug, Clone)]
pub struct ClauseDisjunctiveBasic<T: SwUint> {
    // Raw propositional atoms
    atoms: Vec<T>,
    // Truth value required by the atom for for this clause to be satisfied
    truthiness: Vec<bool>,
}

impl<T: SwUint> ClauseDisjunctiveBasic<T> {
    pub fn new(atoms: Vec<T>, truthiness: Vec<bool>) -> Self {
        debug_assert_eq!(atoms.len(), truthiness.len());
        Self { atoms, truthiness }
    }
}

impl<T: SwUint> ClauseDisjunctive<T> for ClauseDisjunctiveBasic<T> {
    fn iter(&self) -> impl Iterator<Item = (T, bool)> {
        // TODO: OPTIMIZE: This is not how iter should be implemented right?
        // iter should be reference based no?
        self.atoms
            .iter()
            .copied()
            .zip(self.truthiness.iter().copied())
    }
}

impl<T: SwUint> IntoIterator for ClauseDisjunctiveBasic<T> {
    type Item = (T, bool);
    type IntoIter = iter::Zip<vec::IntoIter<T>, vec::IntoIter<bool>>;

    fn into_iter(self) -> Self::IntoIter {
        self.atoms.into_iter().zip(self.truthiness)
    }
}

impl<T: SwUint> FromIterator<(T, bool)> for ClauseDisjunctiveBasic<T> {
    fn from_iter<I: IntoIterator<Item = (T, bool)>>(it: I) -> Self {
        let (atoms, truthiness) = it.into_iter().unzip();
        Self::new(atoms, truthiness)
    }
}

// -------------------------------
// Formula
// -------------------------------
pub trait FormulaConjunctive<T: SwUint>:
    IntoIterator<Item = Self::Clause> + FromIterator<Self::Clause>
{
    type Clause: ClauseDisjunctive<T>;
    // TODO: OPTIMIZE: Add lifetime later so we can return a reference
    fn iter(&self) -> impl Iterator<Item = Self::Clause>;
}

#[derive(Debug, Clone)]
pub struct FormulaConjunctiveBasic<T: SwUint> {
    clauses: Vec<ClauseDisjunctiveBasic<T>>,
}

impl<T: SwUint> FormulaConjunctiveBasic<T> {
    pub fn new<I: IntoIterator<Item = J>, J: IntoIterator<Item = (T, bool)>>(it: I) -> Self {
        it.into_iter().map(|c| c.into_iter().collect()).collect()
    }
}

impl<T: SwUint> FormulaConjunctive<T> for FormulaConjunctiveBasic<T> {
    type Clause = ClauseDisjunctiveBasic<T>;

    fn iter(&self) -> impl Iterator<Item = Self::Clause> {
        self.clauses.iter().cloned()
    }
}

impl<T: SwUint> IntoIterator for FormulaConjunctiveBasic<T> {
    type Item = ClauseDisjunctiveBasic<T>;
    type IntoIter = vec::IntoIter<ClauseDisjunctiveBasic<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.clauses.into_iter()
    }
}

impl<T: SwUint> FromIterator<ClauseDisjunctiveBasic<T>> for FormulaConjunctiveBasic<T> {
    fn from_iter<I: IntoIterator<Item = ClauseDisjunctiveBasic<T>>>(it: I) -> Self {
        Self {
            clauses: it.into_iter().collect(),
        }
    }
}

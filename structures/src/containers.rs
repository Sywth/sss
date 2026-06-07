use std::{
    fmt::{Debug, Display},
    iter::{self},
    vec,
};

use crate::primitives::{Atom, AtomType};

#[inline]
pub fn get_idx_from_atom(atom: Atom) -> usize {
    debug_assert!(atom > Atom::ZERO, "atom must be non-zero");
    usize::from(atom) - 1
}

#[inline]
pub fn get_atom_from_idx(idx: usize) -> Atom {
    Atom::from((idx + 1) as AtomType)
}

// OPTIMIZE: consider using SSO here?
pub mod symbols {
    pub mod debug {
        pub const NEG: &str = "\u{00AC}";
        pub const CONJ: &str = "\u{2227}";
        pub const DISJ: &str = "\u{2228}";
        pub const L_PAREN: &str = "(";
        pub const R_PAREN: &str = ")";
    }
    pub mod parser {
        pub const NEG: &str = "not";
        pub const CONJ: &str = "and";
        pub const DISJ: &str = "or";
        pub const EQ: &str = "equal";
        pub const IMPL: &str = "implies";
        pub const IFF: &str = "iff";
    }
}

// -------------------------------
// Assignment
// -------------------------------
pub trait Assignment: Clone + Debug + IntoIterator<Item = Option<bool>> {
    fn new(size: usize) -> Self;
    fn get(&self, atom: Atom) -> Option<bool>;
    fn is_set(&self, atom: Atom) -> bool;
    fn set(&mut self, atom: Atom, value: bool);
    fn clear(&mut self, atom: Atom);
    fn as_formatted_str(&self) -> String;
    fn get_num_atoms(&self) -> usize;
}

#[derive(Clone, Debug)]
pub struct AssignmentBasic {
    gamma: Vec<Option<bool>>,
}

impl Assignment for AssignmentBasic {
    fn new(size: usize) -> Self {
        AssignmentBasic {
            gamma: vec![None; size],
        }
    }

    fn get(&self, atom: Atom) -> Option<bool> {
        let idx = get_idx_from_atom(atom);
        *self
            .gamma
            .get(idx)
            .expect("bad logic, gamma should have a slot for every atom")
    }

    fn is_set(&self, atom: Atom) -> bool {
        let idx = get_idx_from_atom(atom);
        let Some(Some(_)) = self.gamma.get(idx) else {
            return false;
        };

        true
    }

    fn set(&mut self, atom: Atom, truthiness: bool) {
        let idx = get_idx_from_atom(atom);
        self.gamma[idx] = Some(truthiness);
    }

    fn clear(&mut self, atom: Atom) {
        let idx = get_idx_from_atom(atom);
        self.gamma[idx] = None;
    }

    fn as_formatted_str(&self) -> String {
        let mut ss = String::with_capacity(self.gamma.len());
        self.gamma.iter().for_each(|t_opt| match t_opt {
            Some(true) => ss.push('1'),
            Some(false) => ss.push('0'),
            None => ss.push('-'),
        });

        ss
    }

    fn get_num_atoms(&self) -> usize {
        self.gamma.len()
    }
}

impl Display for AssignmentBasic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dbg_str = self
            .gamma
            .iter()
            .enumerate()
            .filter_map(|(i, &t_opt)| {
                t_opt.map(|t| {
                    format!(
                        "{}{}",
                        if t { "" } else { symbols::debug::NEG },
                        get_atom_from_idx(i)
                    )
                })
            })
            .collect::<Vec<_>>()
            .join(",");

        write!(f, "[{}]", dbg_str)
    }
}

impl IntoIterator for AssignmentBasic {
    type Item = Option<bool>;
    type IntoIter = std::vec::IntoIter<Option<bool>>;

    fn into_iter(self) -> Self::IntoIter {
        self.gamma.into_iter()
    }
}

// -------------------------------
// Clause
// -------------------------------
pub trait ClauseDisjunctive:
    IntoIterator<Item = (Atom, bool)> + FromIterator<(Atom, bool)> + Debug
{
    fn iter_copied(&self) -> impl Iterator<Item = (Atom, bool)>;
}

#[derive(Clone, Debug)]
pub struct ClauseDisjunctiveBasic {
    // Raw propositional atoms
    atoms: Vec<Atom>,
    // Truth value required by the atom for for this clause to be satisfied
    truthiness: Vec<bool>,
}

impl ClauseDisjunctiveBasic {
    pub fn new(atoms: Vec<Atom>, truthiness: Vec<bool>) -> Self {
        debug_assert_eq!(atoms.len(), truthiness.len());
        Self { atoms, truthiness }
    }
}

impl ClauseDisjunctive for ClauseDisjunctiveBasic {
    fn iter_copied(&self) -> impl Iterator<Item = (Atom, bool)> {
        self.atoms
            .iter()
            .cloned()
            .zip(self.truthiness.iter().copied())
    }
}

impl IntoIterator for ClauseDisjunctiveBasic {
    type Item = (Atom, bool);
    type IntoIter = iter::Zip<vec::IntoIter<Atom>, vec::IntoIter<bool>>;

    fn into_iter(self) -> Self::IntoIter {
        self.atoms.into_iter().zip(self.truthiness)
    }
}

impl FromIterator<(Atom, bool)> for ClauseDisjunctiveBasic {
    fn from_iter<I: IntoIterator<Item = (Atom, bool)>>(it: I) -> Self {
        let (atoms, truthiness) = it.into_iter().unzip();
        Self::new(atoms, truthiness)
    }
}

impl Display for ClauseDisjunctiveBasic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dbg_str = self
            .atoms
            .iter()
            .enumerate()
            .map(|(i, &a)| {
                let t = self.truthiness[i];
                format!("{}{}", if t { "" } else { symbols::debug::NEG }, a)
            })
            .collect::<Vec<_>>()
            .join(symbols::debug::DISJ);

        write!(f, "{}", dbg_str)
    }
}

// -------------------------------
// Formula
// -------------------------------
pub trait FormulaConjunctive:
    IntoIterator<Item = Self::Clause> + FromIterator<Self::Clause> + Debug
{
    type Clause: ClauseDisjunctive;
    fn iter(&self) -> impl Iterator<Item = &Self::Clause>;
}

#[derive(Clone, Debug)]
pub struct FormulaConjunctiveBasic {
    clauses: Vec<ClauseDisjunctiveBasic>,
}

impl FormulaConjunctiveBasic {
    pub fn new<I: IntoIterator<Item = J>, J: IntoIterator<Item = (Atom, bool)>>(it: I) -> Self {
        it.into_iter().map(|c| c.into_iter().collect()).collect()
    }
}

impl FormulaConjunctive for FormulaConjunctiveBasic {
    type Clause = ClauseDisjunctiveBasic;

    fn iter(&self) -> impl Iterator<Item = &Self::Clause> {
        self.clauses.iter()
    }
}

impl IntoIterator for FormulaConjunctiveBasic {
    type Item = ClauseDisjunctiveBasic;
    type IntoIter = vec::IntoIter<ClauseDisjunctiveBasic>;

    fn into_iter(self) -> Self::IntoIter {
        self.clauses.into_iter()
    }
}

impl FromIterator<ClauseDisjunctiveBasic> for FormulaConjunctiveBasic {
    fn from_iter<I: IntoIterator<Item = ClauseDisjunctiveBasic>>(it: I) -> Self {
        Self {
            clauses: it.into_iter().collect(),
        }
    }
}

impl Display for FormulaConjunctiveBasic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dbg_str = self
            .clauses
            .iter()
            .map(|c| format!("({})", c))
            .collect::<Vec<_>>()
            .join(symbols::debug::CONJ);

        write!(f, "{}", dbg_str)
    }
}

// --------------------------------
// Unit Tests
// --------------------------------

#[test]
#[cfg(test)]
fn assignment_set_get_clear() {
    let mut gamma = AssignmentBasic::new(3);
    assert_eq!(gamma.get(Atom::from(1)), None);
    assert!(!gamma.is_set(Atom::from(1)));
    gamma.set(Atom::from(1), true);
    assert_eq!(gamma.get(Atom::from(1)), Some(true));
    assert!(gamma.is_set(Atom::from(1)));
    gamma.clear(Atom::from(1));
    assert_eq!(gamma.get(Atom::from(1)), None);
    assert!(!gamma.is_set(Atom::from(1)));
}

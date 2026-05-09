use std::collections::HashMap;

use crate::{
    parser::SwInt,
    structures::{FormulaConjunctiveBasic, SwUint},
    FormulaTranslator,
};

pub trait SatFormula {
    // Core algorithmic logic lives here
    fn is_sat(&self) -> bool;
}

pub struct Assignment<T: SwUint> {
    pub gamma: HashMap<T, bool>,
}

fn up<T: SwUint>(phi: &FormulaConjunctiveBasic<T>, gamma: Assignment<T>) -> Option<Assignment<T>> {
    todo!()
}

fn is_valid<T: SwUint>(phi: &FormulaConjunctiveBasic<T>, gamma: Assignment<T>) -> bool {
    todo!()
}

// OPTIMIZE we should cache this no, then minimal penalty on calling it
fn get_atoms<T: SwUint>(phi: &FormulaConjunctiveBasic<T>) -> Vec<T> {
    let mut atoms = Vec::new();
    for clause in phi.clauses.iter() {
        // OPTIMIZE : Can we avoid the copy here, does it matter?
        atoms.extend(clause.atoms.iter().copied());
    }

    atoms
}

fn dpll<T: SwUint>(phi: &FormulaConjunctiveBasic<T>, mut gamma: Assignment<T>) -> bool {
    let res = up(phi, gamma);
    if res.is_none() {
        return false;
    }

    gamma = res.unwrap();
    if is_valid(phi, gamma) {
        return true;
    }

    let atoms = get_atoms(phi);
    // TODO : Figure out how to get rng without init-ing it every time then just random index via
    // the rng random number on each call. Idm having it be global but an LLM said to use threadlocal

    todo!()
}

impl<T: SwUint> SatFormula for FormulaConjunctiveBasic<T> {
    fn is_sat(&self) -> bool {
        // in the future we might swap this out for cdcl
        dpll(self)
    }
}

impl<K: SwInt, V: SwUint> SatFormula for FormulaTranslator<K, V> {
    fn is_sat(&self) -> bool {
        self.cnf.is_sat()
    }
}

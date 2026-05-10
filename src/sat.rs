use std::collections::HashMap;

use rand::RngExt;

use crate::{
    parser::SwInt,
    structures::{FormulaConjunctiveBasic, SwUint},
    FormulaTranslator,
};

pub trait SatFormula {
    fn is_sat(&self) -> bool;
}

pub struct Assignment<T: SwUint> {
    pub gamma: HashMap<T, bool>,
}

impl<T: SwUint> Default for Assignment<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: SwUint> Assignment<T> {
    pub fn new() -> Self {
        Assignment {
            gamma: HashMap::new(),
        }
    }

    pub fn assign(&mut self, atom: T, truth_value: bool) {
        self.gamma.insert(atom, truth_value);
    }
}

// TODO: OPTIMIZE: Find a better method for this asap
fn get_atoms_phi<T: SwUint>(phi: &FormulaConjunctiveBasic<T>) -> Vec<T> {
    return Vec::new();
}

// TODO: OPTIMIZE: Find a better method for this asap
fn get_atoms_gamma<T: SwUint>(gamma: &Assignment<T>) -> Vec<T> {
    return Vec::new();
}

// OPTIMIZE: In future we might just want to return a reference the Assignment if there are
// lots of atoms per formula
fn up<T: SwUint>(phi: &FormulaConjunctiveBasic<T>, gamma: &Assignment<T>) -> Option<Assignment<T>> {
    // TODO: OPTIMIZE: Find a better method for this asap
    let atoms_phi = get_atoms_phi(phi);
    let atoms_gamma = get_atoms_gamma(gamma);

    for clause in phi.clauses.iter() {
        // if there is a literal in this clause that is
        // met in gamma skip as its satisfied

        let mut is_clause_met = false;
        for (i, atom) in clause.atoms.iter().enumerate() {
            let truthiness = clause.truthiness.get(i).expect("bad logic");
            let Some(curr_assigned_truthiness) = gamma.gamma.get(atom) else {
                // this atom has not been closed yet, skip
                continue;
            };

            // this atom has been closed but is
            // it what we need to meet this clause
            if truthiness == curr_assigned_truthiness {
                is_clause_met = true;
                break;
            }
        }

        if is_clause_met {
            continue;
        }

        let mut unassigned = Vec::new();
        for atom in &atoms_phi {
            if !atoms_gamma.contains(atom) {
                unassigned.push(*atom);
            }
        }

        if unassigned.len() == 0 {
            return None;
        }

        if unassigned.len() == 1 {
            // TODO: Figure out how to get this element popped out
            //todo!()
        }

        // unassigned len > 1 hence many branches hence just skip this clause for now
    }

    // TODO: Figure out what to return
    Some(gamma)
}

fn is_valid<T: SwUint>(phi: &FormulaConjunctiveBasic<T>, gamma: &Assignment<T>) -> bool {
    todo!()
}

// OPTIMIZE: we should cache this no, then minimal penalty on calling it
fn get_open_atoms<T: SwUint>(phi: &FormulaConjunctiveBasic<T>, gamma: &Assignment<T>) -> Vec<T> {
    let mut atoms = Vec::new();
    for clause in phi.clauses.iter() {
        for a in &clause.atoms {
            if !gamma.gamma.contains_key(a) {
                atoms.push(*a);
            }
        }
    }

    atoms
}

fn dpll<T: SwUint>(phi: &FormulaConjunctiveBasic<T>, mut gamma: &Assignment<T>) -> bool {
    let res = up(phi, gamma);
    if res.is_none() {
        return false;
    }

    let mut gamma = res.expect("bad if logic");

    if is_valid(phi, &gamma) {
        return true;
    }

    let open_atoms: &Vec<T> = &get_open_atoms(phi, &gamma);
    let idx = rand::rng().random_range(0..open_atoms.len());
    let a = open_atoms.get(idx).expect("bad range logic");

    gamma.assign(*a, true);
    if dpll(phi, &gamma) {
        return true;
    }

    gamma.assign(*a, false);
    dpll(phi, &gamma)
}

impl<T: SwUint> SatFormula for FormulaConjunctiveBasic<T> {
    fn is_sat(&self) -> bool {
        // in the future we might swap this out for cdcl
        dpll(self, &Assignment::new())
    }
}

impl<K: SwInt, V: SwUint> SatFormula for FormulaTranslator<K, V> {
    fn is_sat(&self) -> bool {
        self.cnf.is_sat()
    }
}

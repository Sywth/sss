use rand::RngExt;

use crate::{
    parser::SwInt,
    structures::{ClauseDisjunctive, FormulaConjunctive, FormulaConjunctiveBasic, SwUint},
    FormulaTranslator,
};

pub trait SatFormula {
    fn is_sat(&self) -> bool;
}

pub struct Assignment {
    gamma: Vec<Option<bool>>,
}

pub fn get_atom_idx<T: SwUint>(atom: T) -> usize {
    (T::sub(atom, T::one()))
        .to_usize()
        .expect("given atom could not be cast to usize, was it 0?")
}

impl Assignment {
    pub fn new(size: usize) -> Self {
        Assignment {
            gamma: vec![None; size],
        }
    }

    pub fn get<T: SwUint>(&self, atom: T) -> Option<&Option<bool>> {
        let atom_idx = get_atom_idx(atom);
        self.gamma.get(atom_idx)
    }

    fn insert<T: SwUint>(&mut self, atom: T, truthiness: Option<bool>) {
        let atom_idx = get_atom_idx(atom);
        self.gamma.insert(atom_idx, truthiness);
    }
}

fn get_atoms_phi<T: SwUint>(phi: &FormulaConjunctiveBasic<T>) -> Vec<T> {
    // note this is a weird way of doing [1..phi.max_atom()]
    let mut atoms: Vec<T> = Vec::new();
    for clause in phi.iter() {
        for (atom, _) in clause.iter() {
            if !atoms.contains(&atom) {
                atoms.push(atom);
            }
        }
    }

    #[cfg(debug_assertions)]
    {
        let max_atom = atoms
            .iter()
            .reduce(|a, b| if a > b { a } else { b })
            .expect("bad logic");
        let atoms_len_eq_max_atom = atoms.len() == max_atom.to_usize().expect("bad logic");
        assert!(
            atoms_len_eq_max_atom,
            "this should just be [1, 2, ..., max atom in phi]"
        );
    }

    atoms
}

// OPTIMIZE: In future we might just want to return a reference the Assignment if there are
// lots of atoms per formula
fn up<T: SwUint>(phi: &FormulaConjunctiveBasic<T>, gamma: &mut Assignment) -> bool {
    for clause in phi.iter() {
        // if there is a literal in this clause that is
        // met in gamma skip as its satisfied

        let mut is_clause_met = false;
        for (atom, truthiness) in clause.iter() {
            let curr_atom_assingment_opt = gamma
                .get(atom)
                .expect("bad logic, this should already be fully initialized");

            // if current atom is not closed by gamma, skip
            if curr_atom_assingment_opt.is_none() {
                continue;
            }
            let curr_atom_assingment = curr_atom_assingment_opt.expect("bad logic");

            // this literal is in gamma, hence clause met
            if truthiness == curr_atom_assingment {
                is_clause_met = true;
                break;
            }
        }

        // this clause is met, hence onto the next
        if is_clause_met {
            continue;
        }

        // build a set of atoms not closed by gamma for this clause
        let mut unassigned: Option<(T, bool)> = None;
        let mut is_unit_clause: bool = false;
        for (atom, required_truthiness) in clause.iter() {
            let atom_in_gamma = gamma.get(atom);

            if atom_in_gamma.is_none() {
                // we have more than one unassigned literal in this clause
                if unassigned.is_some() {
                    is_unit_clause = false;
                    break;
                }

                unassigned = Some((atom, required_truthiness));
                is_unit_clause = true;
            }
        }

        // clause has not been met and there are no open
        // literals left to be met, try different branch
        if unassigned.is_none() {
            assert!(!is_unit_clause);
            return false;
        }

        // we have two or more open literals
        if !is_unit_clause {
            continue;
        }

        assert!(is_unit_clause && unassigned.is_some());
        let unassigned = unassigned.expect("bad logic");
        gamma.insert(unassigned.0, Some(unassigned.1));
        // OPTIMIZE: Tail recursion with become keyword
        return up(phi, gamma);
    }

    true
}

fn is_valid<T: SwUint>(phi: &FormulaConjunctiveBasic<T>, gamma: &Assignment) -> bool {
    for clause in phi.iter() {
        let mut is_clause_met = false;
        for (atom, expected_truthiness) in clause.iter() {
            let atom_curr_truthiness_opt = gamma.get(atom).expect("bad logic");
            if atom_curr_truthiness_opt.is_none() {
                continue;
            }

            let atom_curr_truthiness = atom_curr_truthiness_opt.expect("bad logic");
            if atom_curr_truthiness == expected_truthiness {
                is_clause_met = true;
                break;
            }
        }

        // every clause must be met to have
        // gamma |= phi
        if !is_clause_met {
            return false;
        }
    }

    true
}

fn get_open_atoms<T: SwUint>(atoms_phi: &Vec<T>, gamma: &Assignment) -> Vec<T> {
    let mut open_atoms = Vec::new();
    for a in atoms_phi {
        let a_idx = a.to_usize().expect("bad logic");
        let a_opt = gamma
            .gamma
            .get(a_idx)
            .expect("bad logic, gamma should contain slot for every atom");

        if a_opt.is_none() {
            open_atoms.push(*a);
        }
    }

    open_atoms
}

fn dpll<T: SwUint>(
    phi: &FormulaConjunctiveBasic<T>,
    atoms_phi: &Vec<T>,
    gamma: &mut Assignment,
) -> bool {
    let res = up(phi, gamma);
    if !res {
        return false;
    }

    if is_valid(phi, gamma) {
        return true;
    }

    let open_atoms: &Vec<T> = &get_open_atoms(atoms_phi, gamma);
    let idx = rand::rng().random_range(0..open_atoms.len());
    let a_idx = open_atoms
        .get(idx)
        .expect("bad range logic")
        .to_usize()
        .expect("bad logic");

    gamma.gamma.insert(a_idx, Some(true));
    if dpll(phi, atoms_phi, gamma) {
        return true;
    }

    gamma.gamma.insert(a_idx, Some(false));
    dpll(phi, atoms_phi, gamma)
}

impl<T: SwUint> SatFormula for FormulaConjunctiveBasic<T> {
    fn is_sat(&self) -> bool {
        // in the future we might swap this out for cdcl
        let atoms_phi = get_atoms_phi(self);
        let mut assignment = Assignment::new(atoms_phi.len());
        dpll(self, &atoms_phi, &mut assignment)
    }
}

impl<K: SwInt, V: SwUint> SatFormula for FormulaTranslator<K, V> {
    fn is_sat(&self) -> bool {
        self.cnf.is_sat()
    }
}

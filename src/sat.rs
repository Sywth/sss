use rand::RngExt;

use crate::{as_colored, util::AnsiColor};
use structures::containers::{
    Assignment, AssignmentBasic, ClauseDisjunctive, FormulaConjunctive, FormulaConjunctiveBasic,
};
use structures::primitives::Atom;

const PRETTY_OUTPUT: bool = true;

pub trait SatFormula {
    // TODO: Instead this should have signature
    // fn is_sat(&sefl) -> Assignment | UnsatProof
    // but then we also have to define a proof unsat
    // and also ideally build verifiers for both to verify our proofs
    // make sense
    fn is_sat(&self) -> bool;
}

fn to_str_gamma(gamma: &impl Assignment) -> String {
    let mut ss = String::with_capacity(gamma.get_num_atoms());
    for a_opt in gamma.clone() {
        let Some(a) = a_opt else {
            ss.push('-');
            continue;
        };

        let colored_atom_str = if a {
            as_colored("1", AnsiColor::Green)
        } else {
            as_colored("0", AnsiColor::Red)
        };
        ss.push_str(&colored_atom_str);
    }

    ss
}

fn get_atoms_phi(phi: &FormulaConjunctiveBasic) -> Vec<Atom> {
    // note this is a weird way of doing [1..phi.max_atom()]
    let mut atoms: Vec<Atom> = Vec::new();
    for clause in phi.iter() {
        for (atom, _) in clause.iter_copied() {
            if !atoms.contains(&atom) {
                atoms.push(atom);
            }
        }
    }

    #[cfg(debug_assertions)]
    {
        let Some(max_atom) = atoms.iter().reduce(|a, b| if a > b { a } else { b }) else {
            // zero variables
            return atoms;
        };
        let atoms_len_eq_max_atom = atoms.len() == usize::from(*max_atom);
        assert!(
            atoms_len_eq_max_atom,
            "this should just be [1, 2, ..., max atom in phi]"
        );
    }

    atoms
}

// OPTIMIZE: In future we might just want to return a reference the Assignment if there are
// lots of atoms per formula
fn up(phi: &FormulaConjunctiveBasic, gamma: &mut impl Assignment) -> bool {
    for clause in phi.iter() {
        // if there is a literal in this clause that is
        // met in gamma skip as its satisfied

        let mut is_clause_met = false;
        for (atom, truthiness) in clause.iter_copied() {
            let Some(curr_atom_assingment) = gamma.get(atom) else {
                // if current atom is not closed by gamma, skip
                continue;
            };

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

        // figure out if this is a unit clause, and if so what literal
        let mut unit_literal: Option<(Atom, bool)> = None;
        let mut is_not_unit_literal = false;
        for (atom, required_truthiness) in clause.iter_copied() {
            if !gamma.is_set(atom) {
                // we have more than one unassigned literal in this clause
                if unit_literal.is_some() {
                    is_not_unit_literal = true;
                    break;
                }

                unit_literal = Some((atom, required_truthiness));
            }
        }

        // clause has not been met and there are no open
        // literals left to be met, try different branch
        if unit_literal.is_none() {
            debug_assert!(!is_not_unit_literal);
            return false;
        }

        // we have two or more open literals
        if is_not_unit_literal {
            continue;
        }

        debug_assert!(!is_not_unit_literal && unit_literal.is_some());
        let unassigned = unit_literal.expect("bad logic");
        gamma.set(unassigned.0, unassigned.1);
        // OPTIMIZE: Tail recursion with become keyword
        return up(phi, gamma);
    }

    true
}

fn is_valid(phi: &FormulaConjunctiveBasic, gamma: &impl Assignment) -> bool {
    for clause in phi.iter() {
        let mut is_clause_met = false;
        for (atom, expected_truthiness) in clause.iter_copied() {
            let Some(atom_curr_truthiness) = gamma.get(atom) else {
                // unassigned
                continue;
            };

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

fn get_open_atoms(atoms_phi: &[Atom], gamma: &impl Assignment) -> Vec<Atom> {
    atoms_phi
        .iter()
        .filter_map(|&a| {
            if gamma.get(a).is_none() {
                return Some(a);
            }
            None
        })
        .collect()
}

fn dpll<A: Assignment + std::fmt::Display>(
    phi: &FormulaConjunctiveBasic,
    atoms_phi: &Vec<Atom>,
    gamma: &mut A,
) -> bool {
    if PRETTY_OUTPUT {
        let dbg_str = to_str_gamma(gamma);
        eprintln!("{}", dbg_str);
    }

    let res = up(phi, gamma);
    if !res {
        return false;
    }

    if is_valid(phi, gamma) {
        if PRETTY_OUTPUT {
            eprintln!("Valid Assignment:\n{}", gamma.as_formatted_str());
        }
        return true;
    }

    // TODO: We already say we're using FormulaConjunctiveBasic
    // lets just skip the atoms_phi thing and say our vars 1..=open_atoms.len()
    // hence a = choice(1..=open_atoms.len())
    let open_atoms: &Vec<Atom> = &get_open_atoms(atoms_phi, gamma);
    let idx = rand::rng().random_range(0..open_atoms.len());
    let a = open_atoms[idx];

    let mut gamma_cloned = gamma.clone();
    gamma_cloned.set(a, true);
    if dpll(phi, atoms_phi, &mut gamma_cloned) {
        return true;
    }

    gamma.set(a, false);
    dpll(phi, atoms_phi, gamma)
}

impl SatFormula for FormulaConjunctiveBasic {
    fn is_sat(&self) -> bool {
        // in the future we might swap this out for cdcl
        let atoms_phi = get_atoms_phi(self);
        let mut assignment = <AssignmentBasic as Assignment>::new(atoms_phi.len());
        dpll(self, &atoms_phi, &mut assignment)
    }
}

impl SatFormula for FormulaTranslator {
    fn is_sat(&self) -> bool {
        self.cnf.is_sat()
    }
}

// --------------------------------
// Unit Tests
// --------------------------------

#[test]
#[cfg(test)]
fn dpll_sat_tautological_clause() {
    // (1∨¬1) — tautology, SAT without forcing any assignment
    let phi = FormulaConjunctiveBasic::new([vec![(Atom::from(1), true), (Atom::from(1), false)]]);
    assert!(phi.is_sat());
}

#[test]
#[cfg(test)]
fn dpll_unsat_up_contradiction() {
    // (1∨2) ∧ (¬1∨2) ∧ (¬2) — UP forces 2=false then 1=true then clause 2 fails
    let phi = FormulaConjunctiveBasic::new([
        vec![(Atom::from(1), true), (Atom::from(2), true)],
        vec![(Atom::from(1), false), (Atom::from(2), true)],
        vec![(Atom::from(2), false)],
    ]);
    assert!(!phi.is_sat());
}

#[test]
#[cfg(test)]
fn dpll_sat_3v_3c() {
    // (1∨2) ∧ (¬1∨2) ∧ (¬2∨3)
    let phi = FormulaConjunctiveBasic::new([
        vec![(Atom::from(1), true), (Atom::from(2), true)],
        vec![(Atom::from(1), false), (Atom::from(2), true)],
        vec![(Atom::from(2), false), (Atom::from(3), true)],
    ]);
    assert!(phi.is_sat());
}

#[test]
#[cfg(test)]
fn dpll_sat_empty_formula() {
    // an empty formula is always true as its an empty conjunction
    // therefore SAT
    let phi = FormulaConjunctiveBasic::new(Vec::<Vec<(Atom, bool)>>::new());
    assert!(phi.is_sat());
}

#[test]
#[cfg(test)]
fn dpll_unsat_empty_clause() {
    // an empty clauses is always false as its an empty disjunction
    // therefore UNSAT
    let phi = FormulaConjunctiveBasic::new([Vec::<(Atom, bool)>::new()]);
    assert!(!phi.is_sat());
}

#[test]
#[cfg(test)]
fn dpll_unsat_3v_3c() {
    // (1) ∧ (¬1) ∧ (2∨3)
    let phi = FormulaConjunctiveBasic::new([
        vec![(Atom::from(1), true)],
        vec![(Atom::from(1), false)],
        vec![(Atom::from(2), true), (Atom::from(3), true)],
    ]);
    assert!(!phi.is_sat());
}

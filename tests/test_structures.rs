use sss::FormulaTranslator;

#[test]
fn clause_vectors_same_length() {
    let input: Vec<Vec<i32>> = vec![vec![1, -2, 3], vec![-1], vec![]];
    let formula: FormulaTranslator<i32, u32> = FormulaTranslator::new(input);
    for clause in &formula.cnf.clauses {
        assert_eq!(clause.atoms.len(), clause.truthiness.len());
    }
}

#[test]
fn map_covers_all_atoms() {
    let input: Vec<Vec<i32>> = vec![vec![1, -2, 3], vec![-100, 2]];
    let formula: FormulaTranslator<i32, u32> = FormulaTranslator::new(input.clone());
    for clause in &input {
        for &lit in clause {
            assert!(formula.dimacs_id_to_sw_id.contains_key(&lit.abs()));
        }
    }
}

#[test]
fn polarity_matches_sign() {
    let input: Vec<Vec<i32>> = vec![vec![1, -2, 3, -3]];
    let formula: FormulaTranslator<i32, u32> = FormulaTranslator::new(input);
    let clause = &formula.cnf.clauses[0];
    assert_eq!(clause.truthiness, vec![true, false, true, false]);
}

#[test]
fn clause_atoms_match_map() {
    let input: Vec<Vec<i32>> = vec![vec![5, -99, 5]];
    let formula: FormulaTranslator<i32, u32> = FormulaTranslator::new(input.clone());
    let clause = &formula.cnf.clauses[0];
    for (i, &lit) in input[0].iter().enumerate() {
        let expected = *formula.dimacs_id_to_sw_id.get(&lit.abs()).unwrap();
        assert_eq!(clause.atoms[i], expected);
    }
}

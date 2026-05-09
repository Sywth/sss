use sss::{parse_dimacs_file, structures::ClauseDisjunctiveBasic, FormulaTranslator};

// --------------------------------------------
// Parser Test
// --------------------------------------------
#[test]
fn clause_new_collects_elements() {
    let clause = ClauseDisjunctiveBasic::<u32>::new(vec![1, 2, 3], vec![true, false, false]);
    assert_eq!(clause.atoms, vec![1, 2, 3]);
    assert_eq!(clause.truthiness, vec![true, false, false]);
}

#[test]
fn parse_simple_dimacs() {
    let dimacs = "\
        c example
        p cnf 3 2
        1 -2 0
        3 0
    ";

    let path = std::env::temp_dir().join("dimacs_simple.cnf");
    std::fs::write(&path, dimacs).unwrap();

    let formula: FormulaTranslator<i32, u32> = match parse_dimacs_file(&path) {
        Ok(f) => f,
        Err(e) => panic!("failed to read dimacs file {:?}", e),
    };

    let sw_1 = *formula.dimacs_id_to_sw_id.get(&1).unwrap();
    let sw_2 = *formula.dimacs_id_to_sw_id.get(&2).unwrap();
    let sw_3 = *formula.dimacs_id_to_sw_id.get(&3).unwrap();
    assert_eq!(formula.cnf.clauses.len(), 2);
    assert_eq!(formula.cnf.clauses[0].atoms, vec![sw_1, sw_2]);
    assert_eq!(formula.cnf.clauses[0].truthiness, vec![true, false]);
    assert_eq!(formula.cnf.clauses[1].atoms, vec![sw_3]);
    assert_eq!(formula.cnf.clauses[1].truthiness, vec![true]);
}

#[test]
fn parse_complex_dimacs() {
    let dimacs = "\
        c example
        p cnf 4 6
        1 -2 1
        3 0
        2 -242 0
        0
        0
        3 0 
        0
    ";

    let path = std::env::temp_dir().join("dimacs_complex.cnf");
    std::fs::write(&path, dimacs).unwrap();

    let formula: FormulaTranslator<i32, u32> = match parse_dimacs_file(&path) {
        Ok(f) => f,
        Err(e) => panic!("failed to read dimacs file {:?}", e),
    };

    let sw_1 = *formula.dimacs_id_to_sw_id.get(&1).unwrap();
    let sw_2 = *formula.dimacs_id_to_sw_id.get(&2).unwrap();
    let sw_3 = *formula.dimacs_id_to_sw_id.get(&3).unwrap();
    let sw_242 = *formula.dimacs_id_to_sw_id.get(&242).unwrap();
    assert_eq!(formula.cnf.clauses.len(), 6);
    assert_eq!(formula.cnf.clauses[0].atoms, vec![sw_1, sw_2, sw_1, sw_3]);
    assert_eq!(
        formula.cnf.clauses[0].truthiness,
        vec![true, false, true, true]
    );
    assert_eq!(formula.cnf.clauses[1].atoms, vec![sw_2, sw_242]);
    assert_eq!(formula.cnf.clauses[1].truthiness, vec![true, false]);
    assert_eq!(formula.cnf.clauses[2].atoms, vec![]);
    assert_eq!(formula.cnf.clauses[3].atoms, vec![]);
    assert_eq!(formula.cnf.clauses[4].atoms, vec![sw_3]);
    assert_eq!(formula.cnf.clauses[4].truthiness, vec![true]);
    assert_eq!(formula.cnf.clauses[5].atoms, vec![]);
}
#[test]
fn empty_file_parses_as_empty_formula() {
    let path = std::env::temp_dir().join("dimacs_empty.cnf");
    std::fs::write(&path, "").unwrap();

    let formula: FormulaTranslator<i32, u32> = match parse_dimacs_file(&path) {
        Ok(f) => f,
        Err(e) => panic!("failed to read dimacs file {:?}", e),
    };
    assert_eq!(formula.cnf.clauses.len(), 0);
}

#[test]
fn singleton_formula_parses_correctly() {
    let dimacs = "p cnf 1 1\n1 0\n";
    let path = std::env::temp_dir().join("dimacs_singleton.cnf");
    std::fs::write(&path, dimacs).unwrap();

    let formula: FormulaTranslator<i32, u32> = match parse_dimacs_file(&path) {
        Ok(f) => f,
        Err(e) => panic!("failed to read dimacs file {:?}", e),
    };
    let sw_1 = *formula.dimacs_id_to_sw_id.get(&1).unwrap();
    assert_eq!(formula.cnf.clauses.len(), 1);
    assert_eq!(formula.cnf.clauses[0].atoms, vec![sw_1]);
    assert_eq!(formula.cnf.clauses[0].truthiness, vec![true]);
}

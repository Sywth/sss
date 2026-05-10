#![cfg(never)]
use sss::{parse_dimacs_file, FormulaTranslator, SatFormula};

//

//

// --------------------------------------------
// Logic Tests
// --------------------------------------------
#[test]
fn test_dimacs_sat_1() {
    let dimacs = "\
        c example of a sat formula 
        p cnf 1 1 
        1 0
    ";

    let path = std::env::temp_dir().join("dimacs_sat_1.cnf");
    std::fs::write(&path, dimacs).unwrap();

    let formula: FormulaTranslator<i32, u32> = match parse_dimacs_file(&path) {
        Ok(f) => f,
        Err(e) => panic!("failed to read dimacs file {:?}", e),
    };

    let result = formula.is_sat();
    assert!(result);
}

#[test]
fn test_dimacs_sat_2() {
    let dimacs = "\
        c example of a sat formula 
        p cnf 1 1 
        1 -1 0
    ";

    let path = std::env::temp_dir().join("dimacs_sat_2.cnf");
    std::fs::write(&path, dimacs).unwrap();

    let formula: FormulaTranslator<i32, u32> = match parse_dimacs_file(&path) {
        Ok(f) => f,
        Err(e) => panic!("failed to read dimacs file {:?}", e),
    };

    let result = formula.is_sat();
    assert!(result);
}

#[test]
fn test_dimacs_sat_3() {
    let dimacs = "\
        c example of a sat formula 
        p cnf 1 2 
        1 0
        -1 0
    ";

    let path = std::env::temp_dir().join("dimacs_sat_3.cnf");
    std::fs::write(&path, dimacs).unwrap();

    let formula: FormulaTranslator<i32, u32> = match parse_dimacs_file(&path) {
        Ok(f) => f,
        Err(e) => panic!("failed to read dimacs file {:?}", e),
    };

    let result = formula.is_sat();
    assert!(!result);
}

#[test]
fn test_dimacs_sat_4() {
    let dimacs = "\
        c example of a sat formula 
        p cnf 2 3 
        1 2 0
        -1 2 0
        -2 0
    ";

    let path = std::env::temp_dir().join("dimacs_sat_4.cnf");
    std::fs::write(&path, dimacs).unwrap();

    let formula: FormulaTranslator<i32, u32> = match parse_dimacs_file(&path) {
        Ok(f) => f,
        Err(e) => panic!("failed to read dimacs file {:?}", e),
    };

    let result = formula.is_sat();
    assert!(!result);
}

#[test]
fn test_dimacs_sat_5() {
    let dimacs = "\
        c example of a sat formula 
        p cnf 3 3 
        1 2 0
        -1 2 0
        -2 3 0 
    ";

    let path = std::env::temp_dir().join("dimacs_sat_5.cnf");
    std::fs::write(&path, dimacs).unwrap();

    let formula: FormulaTranslator<i32, u32> = match parse_dimacs_file(&path) {
        Ok(f) => f,
        Err(e) => panic!("failed to read dimacs file {:?}", e),
    };

    let result = formula.is_sat();
    assert!(result);
}

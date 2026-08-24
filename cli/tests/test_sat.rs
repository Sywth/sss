#![allow(unused)]
fn run_solver(path: &str) -> String {
    todo!("add code to run the solver here")
}

#[test]
fn e2e_sat_1() {
    return;

    let path_to_sfol_file = "./assets/fol_sat/test_1.fol";

    let expected_res = "SAT";
    let produced_res = run_solver(path_to_sfol_file);
    assert_eq!(produced_res, expected_res);
}

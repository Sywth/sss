mod common;

#[test]
fn e2e_unsat_1() {
    common::decide("sat", &common::asset("fol_unsat/test_2.fol"), "UNSAT");
}

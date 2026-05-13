use std::process::Command;

fn asset(rel: &str) -> String {
    format!("{}/{}", env!("CARGO_MANIFEST_DIR"), rel)
}

fn run_solver(path: &str) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_sss"))
        .arg(path)
        .output()
        .unwrap();
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

#[test]
fn e2e_sat_1() {
    assert_eq!(run_solver(&asset("assets/dimacs_sat/50v_80c.cnf")), "SAT");
}

#[test]
fn e2e_sat_0v_0c() {
    assert_eq!(run_solver(&asset("assets/dimacs_sat/0v_0c.cnf")), "SAT");
}

#[test]
fn e2e_unsat_0v_1c() {
    assert_eq!(run_solver(&asset("assets/dimacs_unsat/0v_1c.cnf")), "UNSAT");
}

#[test]
fn e2e_sat_3v_3c() {
    assert_eq!(run_solver(&asset("assets/dimacs_sat/3v_3c.cnf")), "SAT");
}

#[test]
fn e2e_unsat_2() {
    assert_eq!(run_solver(&asset("assets/dimacs_unsat/60v_160c.cnf")), "UNSAT");
}

#[test]
fn e2e_unsat_3v_3c() {
    assert_eq!(run_solver(&asset("assets/dimacs_unsat/3v_3c.cnf")), "UNSAT");
}

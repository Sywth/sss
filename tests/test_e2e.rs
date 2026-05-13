use std::process::Command;

#[test]
fn e2e_sat_1() {
    let out = Command::new(env!("CARGO_BIN_EXE_sss"))
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/dimacs_sat/50v_80c.cnf"
        ))
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
    assert_eq!(stdout, "SAT", "got: '{stdout}'");
}

#[test]
fn e2e_unsat_1() {
    let out = Command::new(env!("CARGO_BIN_EXE_sss"))
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/dimacs_sat/mytest-01.cnf"
        ))
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
    assert_eq!(stdout, "UNSAT", "got: '{stdout}'");
}

#[test]
fn e2e_sat_0v_0c() {
    let out = Command::new(env!("CARGO_BIN_EXE_sss"))
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/dimacs_sat/0v_0c.cnf"
        ))
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
    assert_eq!(stdout, "SAT", "got: '{stdout}'");
}

#[test]
fn e2e_unsat_0v_1c() {
    let out = Command::new(env!("CARGO_BIN_EXE_sss"))
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/dimacs_unsat/0v_1c.cnf"
        ))
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
    assert_eq!(stdout, "UNSAT", "got: '{stdout}'");
}

#[test]
fn e2e_unsat_2() {
    let out = Command::new(env!("CARGO_BIN_EXE_sss"))
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/dimacs_unsat/60v_160c.cnf"
        ))
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
    assert_eq!(stdout, "UNSAT", "got: '{stdout}'");
}

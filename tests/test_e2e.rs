use std::process::Command;

#[test]
fn e2e_sat() {
    let out = Command::new(env!("CARGO_BIN_EXE_sss"))
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/dimacs_sat/50v_80c.cnf"
        ))
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("SAT"), "got: '{stdout}'");
}

#[test]
fn e2e_unsat() {
    let out = Command::new(env!("CARGO_BIN_EXE_sss"))
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/dimacs_unsat/60v_160c.cnf"
        ))
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("UNSAT"), "got: '{stdout}'");
}

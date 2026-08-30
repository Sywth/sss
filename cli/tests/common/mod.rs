use std::path::PathBuf;
use std::process::Command;

pub fn asset(rel: &str) -> String {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .expect("cli should have a parent workspace dir")
        .join("assets")
        .join(rel)
        .to_string_lossy()
        .into_owned()
}

pub fn run(args: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_cli"))
        .args(args)
        .output()
        .expect("failed to run cli");
    String::from_utf8_lossy(&out.stdout).into_owned()
}

pub fn decide(problem: &str, fp: &str, expected: &str) {
    let produced = run(&["--decide", problem, fp]);
    assert_eq!(produced, expected);
}

use std::io::Write;
use std::process::{Command, Stdio};

fn run_constructive(input: &str) -> String {
    let mut child = Command::new("cargo")
        .args(["run", "--quiet", "--bin", "constructive"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run constructive");

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn test_no_case() {
    let stdout = run_constructive("6\n");
    assert!(stdout.starts_with("NO"));
}

#[test]
fn test_yes_case() {
    let stdout = run_constructive("7\n");
    assert!(stdout.starts_with("YES"));
}

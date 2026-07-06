use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn test_no_case() {
    let mut child = Command::new("cargo")
        .args(["run", "--bin", "greedy"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run greedy");

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"6\n")
        .unwrap();

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.starts_with("NO"));
}

#[test]
fn test_yes_case() {
    let mut child = Command::new("cargo")
        .args(["run", "--bin", "greedy"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run greedy");

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"7\n")
        .unwrap();

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.starts_with("YES"));
}
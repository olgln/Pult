//! Runs the real `pult` binary.

use std::process::Command;

fn pult(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_pult"))
        .args(args)
        .output()
        .expect("failed to run pult")
}

#[test]
fn version_flag() {
    let out = pult(&["--version"]);
    assert!(out.status.success());
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        format!("pult {}\n", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn no_arguments_prints_help() {
    let out = pult(&[]);
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("Usage: pult"));
}

#[test]
fn unknown_argument_fails_with_2() {
    let out = pult(&["--nope"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(out.stdout.is_empty());
    assert!(String::from_utf8_lossy(&out.stderr).contains("unknown argument '--nope'"));
}

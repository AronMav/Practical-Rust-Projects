use std::process::Command;
use assert_cmd::prelude::*;

#[test]
fn run_with_default() {
    Command::cargo_bin("catsay")
        .expect("binary exists")
        .assert()
        .success()
        .stdout(predicates::str::contains("Meow!"));
}

#[test]
fn fail_on_non_existing_file()
    -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("catsay")
        .expect("binary exists")
        .args(&["-f", "no/such/file.txt"])
        .assert()
        .failure();
    Ok(())
}
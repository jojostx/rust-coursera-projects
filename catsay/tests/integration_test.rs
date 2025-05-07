use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn run_with_defaults() {
    Command::cargo_bin("catsay")
        .expect("binary doesn't exists")
        .assert()
        .success()
        .stdout(predicate::str::contains("Meow!"));
}

#[test]
fn fail_on_non_existent_file() {
    Command::cargo_bin("catsay")
        .expect("unable to run binary")
        .arg("-f")
        .arg("/path/to/file")
        .assert()
        .failure();
}

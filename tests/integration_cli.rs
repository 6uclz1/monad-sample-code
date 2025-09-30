use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;

#[test]
fn cli_processes_stdin() -> Result<(), Box<dyn Error>> {
    Command::cargo_bin("monadic-pipeline")?
        .arg("--in")
        .arg("-")
        .arg("--strict-email")
        .write_stdin("Alice,30,alice@example.com\n")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Alice (30, 30s) -> username=alice",
        ));
    Ok(())
}

#[test]
fn cli_reports_validation_error() -> Result<(), Box<dyn Error>> {
    Command::cargo_bin("monadic-pipeline")?
        .arg("--in")
        .arg("-")
        .arg("--min-age")
        .arg("40")
        .write_stdin("Alice,30,alice@example.com\n")
        .assert()
        .failure()
        .stderr(predicate::str::contains("below configured minimum"));
    Ok(())
}

#[test]
fn cli_reads_from_file() -> Result<(), Box<dyn Error>> {
    let fixture = std::path::Path::new("tests/data/users.csv");
    Command::cargo_bin("monadic-pipeline")?
        .arg("--in")
        .arg(fixture)
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice (30, 30s)"))
        .stdout(predicate::str::contains("Bob (45, 40s)"));
    Ok(())
}

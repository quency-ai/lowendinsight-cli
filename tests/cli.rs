use assert_cmd::prelude::*; // Add methods on commands
//use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn test_default_configuration_write() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("lei")?;
    cmd.arg("analyze");
    cmd.arg("https://github.com/kitplummer/gbtestee");
    cmd.assert()
        //.failure()
        .stdout(predicates::str::contains("Analyzing https://github.com/kitplummer/gbtestee"));
    Ok(())
}

#[test]
fn test_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("lei")?;
    cmd.arg("help");
    cmd.assert()
        //.failure()
        .stdout(predicates::str::contains("A command-line interface to LowEndInsight (https://lowendinsight.dev)"));
    Ok(())
}
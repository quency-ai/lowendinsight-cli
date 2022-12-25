use assert_cmd::prelude::*; // Add methods on commands
//use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn test_default_configuration_file_error() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("lei")?;
    cmd.arg("-c");
    cmd.arg("lei.toml");
    cmd.arg("analyze");
    cmd.arg("https://github.com/kitplummer/gbtestee");
    cmd.assert()
        //.failure()
        .stdout(predicates::str::contains("could not read config file"));
    Ok(())
}

#[test]
fn test_default_configuration_content_error() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("lei")?;
    cmd.arg("-c");
    cmd.arg("tests/lei.toml");
    cmd.arg("analyze");
    cmd.arg("https://github.com/kitplummer/gbtestee");
    cmd.assert()
        //.failure()
        .stdout(predicates::str::contains("could not read config contents"));
    Ok(())
}

#[test]
fn test_default_configuration_key_error() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("lei")?;
    cmd.arg("-c");
    cmd.arg("tests/lei.content.toml");
    cmd.arg("analyze");
    cmd.arg("https://github.com/kitplummer/gbtestee");
    cmd.assert()
        //.failure()
        .stdout(predicates::str::contains("rapid_key"));
    Ok(())
}


#[test]
fn test_basic_analysis_post() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("lei")?;
    cmd.arg("analyze");
    cmd.arg("https://github.com/kitplummer/gbtestee");
    cmd.assert()
        .stdout(predicates::str::contains("complete"));
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
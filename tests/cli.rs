use assert_cmd::cargo::*;
use predicates::prelude::*;
use std::fs;

macro_rules! expect_match_fixture {
    ($output_file_name:literal) => {
        let expected_output_path = format!("tests/fixtures/input/{}", $output_file_name);
        let output_fixture_path = format!("tests/fixtures/output/{}", $output_file_name);
        assert!(fs::exists(expected_output_path.clone()).is_ok_and(|_| true));

        let expected_output = fs::read_to_string(expected_output_path.clone())?;
        let output_fixture =
            fs::read_to_string(output_fixture_path).expect("Fixture does not exist.");
        assert_eq!(expected_output, output_fixture);

        // clean up generated file
        fs::remove_file(expected_output_path).unwrap();
    };
}

#[test]
fn file_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("compiler");
    cmd.arg("test/file/does/not/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read input file"));

    Ok(())
}

#[test]
fn ch1_simple_input() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("compiler");
    cmd.arg("tests/fixtures/input/ch1_simple_input.i");
    cmd.assert().success();
    expect_match_fixture!("ch1_simple_input.s");

    Ok(())
}

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

pub fn run_and_compare_output(args: &[&str], expected_output_file: &str, success_expected: bool) {
    let output = Command::new("./target/debug/cachesim")
        .args(args)
        .output()
        .expect("Failed to execute command");

    if success_expected {
        assert!(output.status.success(), "Command failed unexpectedly");
    } else {
        assert!(!output.status.success(), "Command succeeded unexpectedly");
    }

    if !expected_output_file.is_empty() {
        let stdout = String::from_utf8(output.stdout).expect("Not valid UTF-8");
        let stdout_lines: Vec<&str> = stdout.lines().collect();

        let file = File::open(expected_output_file).expect("Failed to open file");
        let reader = BufReader::new(file);
        let file_lines: Vec<String> = reader
            .lines()
            .map(|l| l.expect("Couldn't read line"))
            .collect();

        assert_eq!(file_lines.len(), stdout_lines.len(), "Line count mismatch");
        for (expected, actual) in file_lines.iter().zip(stdout_lines.iter()) {
            assert_eq!(expected, actual, "Line mismatch");
        }
    }
}

pub fn verify_error_output(args: &[&str]) {
    let output = Command::new("./target/debug/cachesim")
        .args(args)
        .output()
        .expect("Failed to execute command");

    assert!(
        !output.status.success(),
        "Command succeeded when it should have failed"
    );
    assert!(
        !output.stderr.is_empty(),
        "Expected error output but got none"
    );
}

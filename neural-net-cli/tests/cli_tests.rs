// Integration tests for CLI scaffold
use std::process::Command;

#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "neural-net-cli", "--", "--help"])
        .output()
        .expect("Failed to run CLI");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Neural Network"), "Help should mention neural network");
    assert!(stdout.contains("train"), "Help should list train command");
    assert!(stdout.contains("eval"), "Help should list eval command");
    assert!(stdout.contains("list"), "Help should list list command");
}

#[test]
fn test_cli_version() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "neural-net-cli", "--", "--version"])
        .output()
        .expect("Failed to run CLI");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("0.1.0"), "Version should be 0.1.0");
}

#[test]
fn test_list_command() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "neural-net-cli", "--", "list"])
        .output()
        .expect("Failed to run CLI");

    assert!(output.status.success(), "List command should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("and"), "Should list AND example");
    assert!(stdout.contains("or"), "Should list OR example");
    assert!(stdout.contains("xor"), "Should list XOR example");
}

#[test]
fn test_train_command_requires_example() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "neural-net-cli", "--", "train"])
        .output()
        .expect("Failed to run CLI");

    assert!(!output.status.success(), "Train without example should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("required") || stderr.contains("example"),
        "Error should mention required example argument"
    );
}

#[test]
fn test_eval_command_requires_model() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "neural-net-cli", "--", "eval"])
        .output()
        .expect("Failed to run CLI");

    assert!(!output.status.success(), "Eval without model should fail");
}

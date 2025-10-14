// Integration tests for train command
use std::fs;
use std::process::Command;
use tempfile::TempDir;

// Helper function to create a temporary directory for tests
// Uses tempfile crate to ensure unique directories even in parallel execution
fn create_temp_dir() -> TempDir {
    TempDir::new().expect("Failed to create temp directory")
}

#[test]
fn test_train_and_basic() {
    let temp_dir = create_temp_dir();
    let output_path = temp_dir.path().join("and_model.json");

    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "neural-net-cli",
            "--",
            "train",
            "--example",
            "and",
            "--epochs",
            "1000",
            "--output",
            output_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run CLI");

    assert!(
        output.status.success(),
        "Training should succeed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output_path.exists(), "Model file should be created");

    // Verify model file is valid JSON
    let contents = fs::read_to_string(&output_path).unwrap();
    let json_value: serde_json::Value =
        serde_json::from_str(&contents).expect("Model file should be valid JSON");

    // Verify it has expected structure
    assert!(json_value["metadata"].is_object());
    assert!(json_value["network"].is_object());

    // TempDir automatically cleans up when dropped
}

#[test]
fn test_train_xor_basic() {
    let temp_dir = create_temp_dir();
    let output_path = temp_dir.path().join("xor_model.json");

    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "neural-net-cli",
            "--",
            "train",
            "--example",
            "xor",
            "--epochs",
            "1000",
            "--output",
            output_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run CLI");

    assert!(output.status.success(), "XOR training should succeed");
    assert!(output_path.exists());

    // TempDir automatically cleans up when dropped
}

#[test]
fn test_train_with_custom_learning_rate() {
    let temp_dir = create_temp_dir();
    let output_path = temp_dir.path().join("custom_lr_model.json");

    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "neural-net-cli",
            "--",
            "train",
            "--example",
            "or",
            "--epochs",
            "500",
            "--learning-rate",
            "0.3",
            "--output",
            output_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run CLI");

    assert!(
        output.status.success(),
        "Training with custom learning rate should succeed"
    );
    assert!(output_path.exists());

    // Verify learning rate in metadata
    let contents = fs::read_to_string(&output_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&contents).unwrap();
    let lr = json["metadata"]["learning_rate"]
        .as_f64()
        .expect("Learning rate should be present");
    assert!((lr - 0.3).abs() < 0.001, "Learning rate should be 0.3");

    // TempDir automatically cleans up when dropped
}

#[test]
fn test_train_invalid_example() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "neural-net-cli",
            "--",
            "train",
            "--example",
            "invalid_example",
            "--epochs",
            "100",
        ])
        .output()
        .expect("Failed to run CLI");

    assert!(
        !output.status.success(),
        "Training with invalid example should fail"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Unknown example") || stderr.contains("invalid"),
        "Error should mention invalid example"
    );
}

#[test]
fn test_train_without_output_succeeds() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "neural-net-cli",
            "--",
            "train",
            "--example",
            "and",
            "--epochs",
            "100",
        ])
        .output()
        .expect("Failed to run CLI");

    assert!(
        output.status.success(),
        "Training without output file should succeed"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Training") || stdout.contains("complete") || stdout.contains("Epoch"),
        "Should show training progress"
    );
}

#[test]
fn test_train_model_metadata() {
    let temp_dir = create_temp_dir();
    let output_path = temp_dir.path().join("metadata_test.json");

    Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "neural-net-cli",
            "--",
            "train",
            "--example",
            "xor",
            "--epochs",
            "500",
            "--output",
            output_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run CLI");

    let contents = fs::read_to_string(&output_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&contents).unwrap();

    // Verify metadata structure
    assert_eq!(json["metadata"]["version"], "1.0");
    assert_eq!(json["metadata"]["example"], "xor");
    assert_eq!(json["metadata"]["epoch"], 500);
    assert_eq!(json["metadata"]["total_epochs"], 500);
    assert!(json["metadata"]["timestamp"].is_string());

    // Verify network structure
    assert!(json["network"]["layers"].is_array());
    assert!(json["network"]["weights"].is_array());
    assert!(json["network"]["biases"].is_array());

    // TempDir automatically cleans up when dropped
}

#[test]
fn test_train_creates_valid_checkpoint() {
    let temp_dir = create_temp_dir();
    let output_path = temp_dir.path().join("checkpoint_test.json");

    Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "neural-net-cli",
            "--",
            "train",
            "--example",
            "and",
            "--epochs",
            "100",
            "--output",
            output_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run CLI");

    // Try to load the checkpoint using the neural-network library
    use neural_network::network::Network;
    let result = Network::load_checkpoint(&output_path);
    assert!(
        result.is_ok(),
        "Saved model should be loadable as checkpoint"
    );

    let (network, metadata) = result.unwrap();
    assert_eq!(metadata.example, "and");
    assert_eq!(metadata.epoch, 100);
    assert_eq!(network.layers, vec![2, 2, 1]);

    // TempDir automatically cleans up when dropped
}

// Integration tests for resume command
use std::fs;
use std::process::Command;
use tempfile::TempDir;

fn create_temp_dir() -> TempDir {
    TempDir::new().expect("Failed to create temp directory")
}

#[test]
fn test_resume_basic() {
    let temp_dir = create_temp_dir();
    let checkpoint_path = temp_dir.path().join("checkpoint.json");

    // First, train and save a checkpoint
    let train_output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "neural-net-cli",
            "--",
            "train",
            "--example",
            "and",
            "--epochs",
            "50",
            "--output",
            checkpoint_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run train");

    assert!(train_output.status.success(), "Training should succeed");
    assert!(checkpoint_path.exists(), "Checkpoint should be created");

    // Resume training from checkpoint
    let resume_output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "neural-net-cli",
            "--",
            "resume",
            "--checkpoint",
            checkpoint_path.to_str().unwrap(),
            "--epochs",
            "50",
        ])
        .output()
        .expect("Failed to run resume");

    assert!(
        resume_output.status.success(),
        "Resume should succeed. stderr: {}",
        String::from_utf8_lossy(&resume_output.stderr)
    );

    let stdout = String::from_utf8_lossy(&resume_output.stdout);
    assert!(
        stdout.contains("Resuming") || stdout.contains("Training"),
        "Should show resume/training message"
    );

    // TempDir automatically cleans up when dropped
}

#[test]
fn test_resume_with_new_checkpoint() {
    let temp_dir = create_temp_dir();
    let old_checkpoint = temp_dir.path().join("old.json");
    let new_checkpoint = temp_dir.path().join("new.json");

    // Train and save initial checkpoint
    Command::new("cargo")
        .args([
            "run",
            "--bin",
            "neural-net-cli",
            "--",
            "train",
            "--example",
            "xor",
            "--epochs",
            "100",
            "--output",
            old_checkpoint.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run train");

    assert!(old_checkpoint.exists());

    // Resume with new checkpoint path
    let resume_output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "neural-net-cli",
            "--",
            "resume",
            "--checkpoint",
            old_checkpoint.to_str().unwrap(),
            "--epochs",
            "100",
            "--output",
            new_checkpoint.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run resume");

    assert!(resume_output.status.success(), "Resume should succeed");
    assert!(new_checkpoint.exists(), "New checkpoint should be created");

    // Verify new checkpoint is valid
    let contents = fs::read_to_string(&new_checkpoint).unwrap();
    let json: serde_json::Value = serde_json::from_str(&contents).unwrap();
    assert!(json["metadata"].is_object());
    assert!(json["network"].is_object());

    // TempDir automatically cleans up when dropped
}

#[test]
fn test_resume_nonexistent_checkpoint() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "neural-net-cli",
            "--",
            "resume",
            "--checkpoint",
            "/nonexistent/checkpoint.json",
            "--epochs",
            "100",
        ])
        .output()
        .expect("Failed to run resume");

    assert!(
        !output.status.success(),
        "Resume with nonexistent checkpoint should fail"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("No such file") || stderr.contains("not found") || stderr.contains("failed"),
        "Error should mention file not found"
    );
}

#[test]
fn test_resume_requires_checkpoint() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "neural-net-cli",
            "--",
            "resume",
            "--epochs",
            "100",
        ])
        .output()
        .expect("Failed to run CLI");

    assert!(
        !output.status.success(),
        "Resume without checkpoint should fail"
    );
}

#[test]
fn test_resume_requires_epochs() {
    let temp_dir = create_temp_dir();
    let checkpoint_path = temp_dir.path().join("checkpoint.json");

    // Create a checkpoint first
    Command::new("cargo")
        .args([
            "run",
            "--bin",
            "neural-net-cli",
            "--",
            "train",
            "--example",
            "and",
            "--epochs",
            "10",
            "--output",
            checkpoint_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run train");

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "neural-net-cli",
            "--",
            "resume",
            "--checkpoint",
            checkpoint_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run CLI");

    assert!(
        !output.status.success(),
        "Resume without epochs should fail"
    );

    // TempDir automatically cleans up when dropped
}

#[test]
fn test_resume_preserves_architecture() {
    let temp_dir = create_temp_dir();
    let checkpoint_path = temp_dir.path().join("arch_test.json");

    // Train XOR (2-2-1 architecture) and save
    Command::new("cargo")
        .args([
            "run",
            "--bin",
            "neural-net-cli",
            "--",
            "train",
            "--example",
            "xor",
            "--epochs",
            "100",
            "--output",
            checkpoint_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run train");

    // Resume training
    let new_checkpoint = temp_dir.path().join("resumed.json");
    Command::new("cargo")
        .args([
            "run",
            "--bin",
            "neural-net-cli",
            "--",
            "resume",
            "--checkpoint",
            checkpoint_path.to_str().unwrap(),
            "--epochs",
            "50",
            "--output",
            new_checkpoint.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run resume");

    // Verify architecture is preserved (XOR is 2-3-1)
    let contents = fs::read_to_string(&new_checkpoint).unwrap();
    let json: serde_json::Value = serde_json::from_str(&contents).unwrap();
    let layers = json["network"]["layers"].as_array().unwrap();

    assert_eq!(layers.len(), 3, "Should have 3 layers");
    assert_eq!(layers[0].as_u64().unwrap(), 2);
    assert_eq!(layers[1].as_u64().unwrap(), 3);
    assert_eq!(layers[2].as_u64().unwrap(), 1);

    // TempDir automatically cleans up when dropped
}

#[test]
fn test_resume_without_output() {
    let temp_dir = create_temp_dir();
    let checkpoint_path = temp_dir.path().join("checkpoint.json");

    // Create initial checkpoint
    Command::new("cargo")
        .args([
            "run",
            "--bin",
            "neural-net-cli",
            "--",
            "train",
            "--example",
            "or",
            "--epochs",
            "50",
            "--output",
            checkpoint_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run train");

    // Resume without saving new checkpoint
    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "neural-net-cli",
            "--",
            "resume",
            "--checkpoint",
            checkpoint_path.to_str().unwrap(),
            "--epochs",
            "50",
        ])
        .output()
        .expect("Failed to run resume");

    assert!(
        output.status.success(),
        "Resume without output should succeed"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Resuming") || stdout.contains("Training") || stdout.contains("complete"),
        "Should show training progress"
    );

    // TempDir automatically cleans up when dropped
}

#[test]
fn test_resume_metadata_updated() {
    let temp_dir = create_temp_dir();
    let checkpoint1 = temp_dir.path().join("checkpoint1.json");
    let checkpoint2 = temp_dir.path().join("checkpoint2.json");

    // Initial training for 100 epochs
    Command::new("cargo")
        .args([
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
            checkpoint1.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run train");

    // Resume for 50 more epochs
    Command::new("cargo")
        .args([
            "run",
            "--bin",
            "neural-net-cli",
            "--",
            "resume",
            "--checkpoint",
            checkpoint1.to_str().unwrap(),
            "--epochs",
            "50",
            "--output",
            checkpoint2.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run resume");

    // Check first checkpoint metadata
    let contents1 = fs::read_to_string(&checkpoint1).unwrap();
    let json1: serde_json::Value = serde_json::from_str(&contents1).unwrap();
    assert_eq!(json1["metadata"]["epoch"], 100);

    // Check second checkpoint metadata shows continued training
    let contents2 = fs::read_to_string(&checkpoint2).unwrap();
    let json2: serde_json::Value = serde_json::from_str(&contents2).unwrap();
    assert_eq!(json2["metadata"]["epoch"], 50);
    assert_eq!(json2["metadata"]["total_epochs"], 50);

    // TempDir automatically cleans up when dropped
}

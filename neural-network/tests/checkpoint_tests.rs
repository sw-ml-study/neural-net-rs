// Integration tests for checkpoint functionality
use neural_network::network::Network;
use neural_network::activations::SIGMOID;
use neural_network::checkpoint::CheckpointMetadata;
use std::fs;
use tempfile::TempDir;

// Helper function to create a temporary directory for tests
// Uses tempfile crate to ensure unique directories even in parallel execution
fn create_temp_dir() -> TempDir {
    TempDir::new().expect("Failed to create temp directory")
}

#[test]
fn test_create_checkpoint() {
    let network = Network::new(vec![2, 3, 1], SIGMOID, 0.5);
    let metadata = CheckpointMetadata {
        version: "1.0".to_string(),
        example: "xor".to_string(),
        epoch: 1000,
        total_epochs: 10000,
        learning_rate: 0.5,
        timestamp: "2025-10-13T12:00:00Z".to_string(),
    };

    let checkpoint = network.to_checkpoint(metadata.clone());

    assert_eq!(checkpoint.metadata.epoch, 1000);
    assert_eq!(checkpoint.metadata.example, "xor");
    assert_eq!(checkpoint.metadata.version, "1.0");
}

#[test]
fn test_checkpoint_from_network() {
    let mut network = Network::new(vec![2, 3, 1], SIGMOID, 0.5);

    // Train a bit so weights aren't random
    let inputs = vec![vec![0.0, 0.0]];
    let targets = vec![vec![0.0]];
    network.train(inputs, targets, 10);

    let metadata = CheckpointMetadata {
        version: "1.0".to_string(),
        example: "test".to_string(),
        epoch: 10,
        total_epochs: 100,
        learning_rate: 0.5,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    let checkpoint = network.to_checkpoint(metadata);
    let restored = Network::from_checkpoint(checkpoint).expect("Should restore from checkpoint");

    // Should have same architecture
    assert_eq!(network.layers, restored.layers);
    assert_eq!(network.weights.len(), restored.weights.len());
    assert_eq!(network.biases.len(), restored.biases.len());
}

#[test]
fn test_save_and_load_checkpoint() {
    let temp_dir = create_temp_dir();
    let checkpoint_path = temp_dir.path().join("test_checkpoint.json");

    let mut network = Network::new(vec![2, 3, 1], SIGMOID, 0.5);

    // Train to get non-random weights
    network.train(vec![vec![0.0, 0.0]], vec![vec![0.0]], 10);

    let metadata = CheckpointMetadata {
        version: "1.0".to_string(),
        example: "xor".to_string(),
        epoch: 100,
        total_epochs: 1000,
        learning_rate: 0.5,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    // Save checkpoint
    network.save_checkpoint(&checkpoint_path, metadata.clone())
        .expect("Save should succeed");

    assert!(checkpoint_path.exists());

    // Load checkpoint
    let (_restored, restored_meta) = Network::load_checkpoint(&checkpoint_path)
        .expect("Load should succeed");

    assert_eq!(restored_meta.epoch, 100);
    assert_eq!(restored_meta.example, "xor");

    // TempDir automatically cleans up when dropped
}

#[test]
fn test_checkpoint_preserves_predictions() {
    let temp_dir = create_temp_dir();
    let checkpoint_path = temp_dir.path().join("predictions_test.json");

    let mut network = Network::new(vec![2, 3, 1], SIGMOID, 0.5);
    network.train(vec![vec![0.5, 0.5]], vec![vec![0.5]], 100);

    // Get prediction before saving
    let test_input = neural_network::matrix::Matrix::from(vec![0.5, 0.5]);
    let pred_before = network.feed_forward(test_input.clone());

    let metadata = CheckpointMetadata {
        version: "1.0".to_string(),
        example: "test".to_string(),
        epoch: 100,
        total_epochs: 1000,
        learning_rate: 0.5,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    network.save_checkpoint(&checkpoint_path, metadata).unwrap();

    // Load and check prediction
    let (mut restored, _) = Network::load_checkpoint(&checkpoint_path).unwrap();
    let pred_after = restored.feed_forward(test_input);

    // Predictions should be identical
    assert_eq!(pred_before.data, pred_after.data);

    // TempDir automatically cleans up when dropped
}

#[test]
fn test_checkpoint_file_format() {
    let temp_dir = create_temp_dir();
    let checkpoint_path = temp_dir.path().join("format_test.json");

    let network = Network::new(vec![2, 3, 1], SIGMOID, 0.5);
    let metadata = CheckpointMetadata {
        version: "1.0".to_string(),
        example: "xor".to_string(),
        epoch: 100,
        total_epochs: 1000,
        learning_rate: 0.5,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    network.save_checkpoint(&checkpoint_path, metadata).unwrap();

    // Verify JSON structure
    let contents = fs::read_to_string(&checkpoint_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&contents).unwrap();

    assert!(json["metadata"].is_object());
    assert!(json["network"].is_object());
    assert_eq!(json["metadata"]["version"], "1.0");
    assert_eq!(json["metadata"]["epoch"], 100);
    assert_eq!(json["metadata"]["example"], "xor");

    // TempDir automatically cleans up when dropped
}

#[test]
fn test_invalid_checkpoint_version() {
    let temp_dir = create_temp_dir();
    let checkpoint_path = temp_dir.path().join("bad_version.json");

    // Create checkpoint with unsupported version
    let bad_json = r#"{
        "metadata": {
            "version": "999.0",
            "example": "test",
            "epoch": 0,
            "total_epochs": 100,
            "learning_rate": 0.5,
            "timestamp": "2025-01-01T00:00:00Z"
        },
        "network": {
            "layers": [2, 3, 1],
            "weights": [],
            "biases": [],
            "activation": "sigmoid",
            "learning_rate": 0.5
        }
    }"#;

    fs::write(&checkpoint_path, bad_json).unwrap();

    let result = Network::load_checkpoint(&checkpoint_path);
    assert!(result.is_err());

    if let Err(e) = result {
        assert!(e.to_string().contains("version") || e.to_string().contains("Unsupported"));
    }

    // TempDir automatically cleans up when dropped
}

#[test]
fn test_checkpoint_with_corrupted_file() {
    let temp_dir = create_temp_dir();
    let checkpoint_path = temp_dir.path().join("corrupted.json");

    fs::write(&checkpoint_path, "not valid json {{{").unwrap();

    let result = Network::load_checkpoint(&checkpoint_path);
    assert!(result.is_err());

    // TempDir automatically cleans up when dropped
}

#[test]
fn test_checkpoint_nonexistent_file() {
    let checkpoint_path = std::path::PathBuf::from("/nonexistent/path/checkpoint.json");
    let result = Network::load_checkpoint(&checkpoint_path);
    assert!(result.is_err());
}

#[test]
fn test_checkpoint_metadata_fields() {
    let metadata = CheckpointMetadata {
        version: "1.0".to_string(),
        example: "xor".to_string(),
        epoch: 5000,
        total_epochs: 10000,
        learning_rate: 0.5,
        timestamp: "2025-10-13T12:34:56Z".to_string(),
    };

    // All fields should be accessible
    assert_eq!(metadata.version, "1.0");
    assert_eq!(metadata.example, "xor");
    assert_eq!(metadata.epoch, 5000);
    assert_eq!(metadata.total_epochs, 10000);
    assert_eq!(metadata.learning_rate, 0.5);
    assert!(!metadata.timestamp.is_empty());
}

#[test]
fn test_resume_training_from_checkpoint() {
    let temp_dir = create_temp_dir();
    let checkpoint_path = temp_dir.path().join("resume_test.json");

    let mut network = Network::new(vec![2, 2, 1], SIGMOID, 0.5);

    // Train for 100 epochs
    let inputs = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ];
    let targets = vec![vec![0.0], vec![0.0], vec![0.0], vec![1.0]];

    network.train(inputs.clone(), targets.clone(), 100);

    let metadata = CheckpointMetadata {
        version: "1.0".to_string(),
        example: "and".to_string(),
        epoch: 100,
        total_epochs: 500,
        learning_rate: 0.5,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    network.save_checkpoint(&checkpoint_path, metadata).unwrap();

    // Load checkpoint and continue training
    let (mut restored, loaded_meta) = Network::load_checkpoint(&checkpoint_path).unwrap();
    assert_eq!(loaded_meta.epoch, 100);

    // Continue training for another 100 epochs
    restored.train(inputs, targets, 100);

    // Network should have improved (or at least still work)
    let test_pred = restored.feed_forward(neural_network::matrix::Matrix::from(vec![1.0, 1.0]));
    assert!(test_pred.data[0] >= 0.0 && test_pred.data[0] <= 1.0);

    // TempDir automatically cleans up when dropped
}

#[test]
fn test_checkpoint_serialization_is_deterministic() {
    let temp_dir = create_temp_dir();
    let path1 = temp_dir.path().join("checkpoint1.json");
    let path2 = temp_dir.path().join("checkpoint2.json");

    let network = Network::new(vec![2, 3, 1], SIGMOID, 0.5);
    let metadata = CheckpointMetadata {
        version: "1.0".to_string(),
        example: "xor".to_string(),
        epoch: 100,
        total_epochs: 1000,
        learning_rate: 0.5,
        timestamp: "2025-10-13T12:00:00Z".to_string(), // Fixed timestamp for determinism
    };

    network.save_checkpoint(&path1, metadata.clone()).unwrap();
    network.save_checkpoint(&path2, metadata).unwrap();

    let contents1 = fs::read_to_string(&path1).unwrap();
    let contents2 = fs::read_to_string(&path2).unwrap();

    // Should produce identical JSON
    assert_eq!(contents1, contents2);

    // TempDir automatically cleans up when dropped
}

// Integration tests for resuming training from checkpoints
use neural_network::activations::SIGMOID;
use neural_network::checkpoint::CheckpointMetadata;
use neural_network::network::Network;
use neural_network::training::{TrainingConfig, TrainingController};
use std::sync::{Arc, Mutex};
use tempfile::TempDir;

fn create_temp_dir() -> TempDir {
    TempDir::new().expect("Failed to create temp directory")
}

#[test]
fn test_resume_from_checkpoint_basic() {
    let temp_dir = create_temp_dir();
    let checkpoint_path = temp_dir.path().join("resume_test.json");

    // Train for 50 epochs and save checkpoint
    let network = Network::new(vec![2, 2, 1], SIGMOID, 0.5);
    let config = TrainingConfig {
        epochs: 50,
        checkpoint_interval: Some(50),
        checkpoint_path: Some(checkpoint_path.clone()),
        verbose: false,
    };

    let mut controller = TrainingController::new(network, config);
    let inputs = vec![vec![0.0, 0.0], vec![1.0, 1.0]];
    let targets = vec![vec![0.0], vec![1.0]];

    controller.train(inputs.clone(), targets.clone()).unwrap();
    assert!(checkpoint_path.exists());

    // Resume from checkpoint and train for 50 more epochs
    let resumed_controller = TrainingController::from_checkpoint(
        &checkpoint_path,
        TrainingConfig {
            epochs: 50,
            checkpoint_interval: None,
            checkpoint_path: None,
            verbose: false,
        },
    )
    .expect("Should load from checkpoint");

    let loaded_network = resumed_controller.network();
    assert_eq!(loaded_network.layers, vec![2, 2, 1]);

    // TempDir automatically cleans up when dropped
}

#[test]
fn test_resume_preserves_network_state() {
    let temp_dir = create_temp_dir();
    let checkpoint_path = temp_dir.path().join("state_test.json");

    // Train and save
    let mut network = Network::new(vec![2, 3, 1], SIGMOID, 0.5);
    let inputs = vec![vec![0.5, 0.5]];
    let targets = vec![vec![0.8]];
    network.train(inputs.clone(), targets.clone(), 100);

    let metadata = CheckpointMetadata {
        version: "1.0".to_string(),
        example: "test".to_string(),
        epoch: 100,
        total_epochs: 200,
        learning_rate: 0.5,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    network.save_checkpoint(&checkpoint_path, metadata).unwrap();

    // Get prediction before resume
    let test_input = neural_network::matrix::Matrix::from(vec![0.5, 0.5]);
    let pred_before = network.feed_forward(test_input.clone());

    // Resume from checkpoint
    let controller = TrainingController::from_checkpoint(
        &checkpoint_path,
        TrainingConfig {
            epochs: 0, // Don't train, just load
            checkpoint_interval: None,
            checkpoint_path: None,
            verbose: false,
        },
    )
    .unwrap();

    let mut restored_network = controller.into_network();
    let pred_after = restored_network.feed_forward(test_input);

    // Predictions should be identical
    assert_eq!(pred_before.data, pred_after.data);

    // TempDir automatically cleans up when dropped
}

#[test]
fn test_resume_with_continued_training() {
    let temp_dir = create_temp_dir();
    let checkpoint_path = temp_dir.path().join("continued_training.json");

    // Initial training
    let network = Network::new(vec![2, 2, 1], SIGMOID, 0.5);
    let config = TrainingConfig {
        epochs: 50,
        checkpoint_interval: Some(50),
        checkpoint_path: Some(checkpoint_path.clone()),
        verbose: false,
    };

    let mut controller = TrainingController::new(network, config);
    let inputs = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ];
    let targets = vec![vec![0.0], vec![0.0], vec![0.0], vec![1.0]];

    controller.train(inputs.clone(), targets.clone()).unwrap();

    // Resume and continue training
    let mut resumed_controller = TrainingController::from_checkpoint(
        &checkpoint_path,
        TrainingConfig {
            epochs: 50,
            checkpoint_interval: None,
            checkpoint_path: None,
            verbose: false,
        },
    )
    .unwrap();

    let result = resumed_controller.train(inputs, targets);
    assert!(result.is_ok());

    // Network should still be functional
    let network = resumed_controller.network();
    assert_eq!(network.layers, vec![2, 2, 1]);

    // TempDir automatically cleans up when dropped
}

#[test]
fn test_resume_with_callbacks() {
    let temp_dir = create_temp_dir();
    let checkpoint_path = temp_dir.path().join("callback_resume.json");

    // Initial training
    let network = Network::new(vec![2, 2, 1], SIGMOID, 0.5);
    let config = TrainingConfig {
        epochs: 10,
        checkpoint_interval: Some(10),
        checkpoint_path: Some(checkpoint_path.clone()),
        verbose: false,
    };

    let mut controller = TrainingController::new(network, config);
    controller.train(vec![vec![0.0, 0.0]], vec![vec![0.0]]).unwrap();

    // Resume with callback
    let mut resumed_controller = TrainingController::from_checkpoint(
        &checkpoint_path,
        TrainingConfig {
            epochs: 10,
            checkpoint_interval: None,
            checkpoint_path: None,
            verbose: false,
        },
    )
    .unwrap();

    let callback_count = Arc::new(Mutex::new(0));
    let count_clone = callback_count.clone();

    resumed_controller.add_callback(Box::new(move |_e, _l, _n| {
        *count_clone.lock().unwrap() += 1;
    }));

    resumed_controller.train(vec![vec![0.0, 0.0]], vec![vec![0.0]]).unwrap();

    assert_eq!(*callback_count.lock().unwrap(), 10);

    // TempDir automatically cleans up when dropped
}

#[test]
fn test_resume_nonexistent_checkpoint() {
    let checkpoint_path = std::path::PathBuf::from("/nonexistent/checkpoint.json");
    let config = TrainingConfig {
        epochs: 10,
        checkpoint_interval: None,
        checkpoint_path: None,
        verbose: false,
    };

    let result = TrainingController::from_checkpoint(&checkpoint_path, config);
    assert!(result.is_err());
}

#[test]
fn test_resume_with_new_checkpoint_path() {
    let temp_dir = create_temp_dir();
    let old_checkpoint = temp_dir.path().join("old.json");
    let new_checkpoint = temp_dir.path().join("new.json");

    // Initial training with checkpoint
    let network = Network::new(vec![2, 2, 1], SIGMOID, 0.5);
    let config = TrainingConfig {
        epochs: 10,
        checkpoint_interval: Some(10),
        checkpoint_path: Some(old_checkpoint.clone()),
        verbose: false,
    };

    let mut controller = TrainingController::new(network, config);
    controller.train(vec![vec![0.0, 0.0]], vec![vec![0.0]]).unwrap();
    assert!(old_checkpoint.exists());

    // Resume and save to new checkpoint path
    let mut resumed_controller = TrainingController::from_checkpoint(
        &old_checkpoint,
        TrainingConfig {
            epochs: 10,
            checkpoint_interval: Some(10),
            checkpoint_path: Some(new_checkpoint.clone()),
            verbose: false,
        },
    )
    .unwrap();

    resumed_controller.train(vec![vec![0.0, 0.0]], vec![vec![0.0]]).unwrap();
    assert!(new_checkpoint.exists());

    // TempDir automatically cleans up when dropped
}

#[test]
fn test_resume_metadata_continuity() {
    let temp_dir = create_temp_dir();
    let checkpoint_path = temp_dir.path().join("metadata_continuity.json");

    // Initial training
    let network = Network::new(vec![2, 2, 1], SIGMOID, 0.5);
    let config = TrainingConfig {
        epochs: 50,
        checkpoint_interval: Some(50),
        checkpoint_path: Some(checkpoint_path.clone()),
        verbose: false,
    };

    let mut controller = TrainingController::new(network, config);
    controller.train(vec![vec![0.0, 0.0]], vec![vec![0.0]]).unwrap();

    // Load and check metadata
    let (_, metadata) = Network::load_checkpoint(&checkpoint_path).unwrap();
    assert_eq!(metadata.epoch, 50);

    // Resume should be able to access the loaded epoch information
    let resumed_controller = TrainingController::from_checkpoint(
        &checkpoint_path,
        TrainingConfig {
            epochs: 0,
            checkpoint_interval: None,
            checkpoint_path: None,
            verbose: false,
        },
    )
    .unwrap();

    // Should have loaded the network successfully
    assert_eq!(resumed_controller.network().layers, vec![2, 2, 1]);

    // TempDir automatically cleans up when dropped
}

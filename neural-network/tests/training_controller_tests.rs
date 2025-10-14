// Integration tests for TrainingController
use neural_network::activations::SIGMOID;
use neural_network::network::Network;
use std::sync::{Arc, Mutex};
use tempfile::TempDir;

#[test]
fn test_training_controller_basic() {
    let network = Network::new(vec![2, 2, 1], SIGMOID, 0.5);
    let config = neural_network::training::TrainingConfig {
        epochs: 100,
        checkpoint_interval: None,
        checkpoint_path: None,
        verbose: false,
    };

    let mut controller = neural_network::training::TrainingController::new(network, config);

    let inputs = vec![vec![0.0, 0.0], vec![1.0, 1.0]];
    let targets = vec![vec![0.0], vec![1.0]];

    let result = controller.train(inputs, targets);
    assert!(result.is_ok(), "Training should succeed");
}

#[test]
fn test_training_controller_with_callbacks() {
    let network = Network::new(vec![2, 2, 1], SIGMOID, 0.5);
    let config = neural_network::training::TrainingConfig {
        epochs: 10,
        checkpoint_interval: None,
        checkpoint_path: None,
        verbose: false,
    };

    let mut controller = neural_network::training::TrainingController::new(network, config);

    let callback_invocations = Arc::new(Mutex::new(0));
    let invocations_clone = callback_invocations.clone();

    controller.add_callback(Box::new(move |_epoch, _loss, _network| {
        *invocations_clone.lock().unwrap() += 1;
    }));

    let inputs = vec![vec![0.0, 0.0]];
    let targets = vec![vec![0.0]];

    controller.train(inputs, targets).unwrap();

    assert_eq!(*callback_invocations.lock().unwrap(), 10, "Callback should be invoked for each epoch");
}

#[test]
fn test_training_controller_with_multiple_callbacks() {
    let network = Network::new(vec![2, 2, 1], SIGMOID, 0.5);
    let config = neural_network::training::TrainingConfig {
        epochs: 5,
        checkpoint_interval: None,
        checkpoint_path: None,
        verbose: false,
    };

    let mut controller = neural_network::training::TrainingController::new(network, config);

    let counter1 = Arc::new(Mutex::new(0));
    let counter2 = Arc::new(Mutex::new(0));
    let c1 = counter1.clone();
    let c2 = counter2.clone();

    controller.add_callback(Box::new(move |_e, _l, _n| {
        *c1.lock().unwrap() += 1;
    }));
    controller.add_callback(Box::new(move |_e, _l, _n| {
        *c2.lock().unwrap() += 1;
    }));

    controller.train(vec![vec![0.0, 0.0]], vec![vec![0.0]]).unwrap();

    assert_eq!(*counter1.lock().unwrap(), 5, "First callback should run 5 times");
    assert_eq!(*counter2.lock().unwrap(), 5, "Second callback should run 5 times");
}

#[test]
fn test_training_controller_auto_checkpoint() {
    let temp_dir = TempDir::new().unwrap();
    let checkpoint_path = temp_dir.path().join("auto_checkpoint.json");

    let network = Network::new(vec![2, 2, 1], SIGMOID, 0.5);
    let config = neural_network::training::TrainingConfig {
        epochs: 100,
        checkpoint_interval: Some(25),
        checkpoint_path: Some(checkpoint_path.clone()),
        verbose: false,
    };

    let mut controller = neural_network::training::TrainingController::new(network, config);
    controller.train(vec![vec![0.0, 0.0]], vec![vec![0.0]]).unwrap();

    assert!(checkpoint_path.exists(), "Checkpoint should be created");

    // Verify checkpoint metadata
    let (_, metadata) = Network::load_checkpoint(&checkpoint_path).unwrap();
    assert!(metadata.epoch >= 75, "Should have checkpointed at epoch 75 or 100");
}

#[test]
fn test_training_controller_verbose_mode() {
    let network = Network::new(vec![2, 2, 1], SIGMOID, 0.5);
    let config = neural_network::training::TrainingConfig {
        epochs: 10,
        checkpoint_interval: None,
        checkpoint_path: None,
        verbose: true,
    };

    let mut controller = neural_network::training::TrainingController::new(network, config);

    // Verbose mode should work without errors
    let result = controller.train(vec![vec![0.0, 0.0]], vec![vec![0.0]]);
    assert!(result.is_ok(), "Verbose training should succeed");
}

#[test]
fn test_training_controller_returns_trained_network() {
    let network = Network::new(vec![2, 2, 1], SIGMOID, 0.5);
    let config = neural_network::training::TrainingConfig {
        epochs: 100,
        checkpoint_interval: None,
        checkpoint_path: None,
        verbose: false,
    };

    let mut controller = neural_network::training::TrainingController::new(network, config);

    let inputs = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ];
    let targets = vec![vec![0.0], vec![0.0], vec![0.0], vec![1.0]];

    controller.train(inputs, targets).unwrap();

    // Should be able to access the network after training
    let network = controller.network();
    assert_eq!(network.layers, vec![2, 2, 1]);
}

#[test]
fn test_training_config_defaults() {
    let config = neural_network::training::TrainingConfig {
        epochs: 1000,
        checkpoint_interval: None,
        checkpoint_path: None,
        verbose: false,
    };

    assert_eq!(config.epochs, 1000);
    assert!(config.checkpoint_interval.is_none());
    assert!(config.checkpoint_path.is_none());
    assert!(!config.verbose);
}

#[test]
fn test_training_controller_checkpoint_at_final_epoch() {
    let temp_dir = TempDir::new().unwrap();
    let checkpoint_path = temp_dir.path().join("final_checkpoint.json");

    let network = Network::new(vec![2, 2, 1], SIGMOID, 0.5);
    let config = neural_network::training::TrainingConfig {
        epochs: 50,
        checkpoint_interval: Some(50), // Checkpoint at final epoch
        checkpoint_path: Some(checkpoint_path.clone()),
        verbose: false,
    };

    let mut controller = neural_network::training::TrainingController::new(network, config);
    controller.train(vec![vec![0.0, 0.0]], vec![vec![0.0]]).unwrap();

    assert!(checkpoint_path.exists());
    let (_, metadata) = Network::load_checkpoint(&checkpoint_path).unwrap();
    assert_eq!(metadata.epoch, 50);
}

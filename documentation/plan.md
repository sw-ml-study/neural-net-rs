# Implementation Plan

## Neural Network Demonstration Platform - Development Roadmap

**Version:** 1.0
**Last Updated:** 2025-10-13
**Methodology:** Test-Driven Development (TDD)

## Table of Contents

1. [Overview](#overview)
2. [TDD Approach](#tdd-approach)
3. [Phase Breakdown](#phase-breakdown)
4. [Detailed Tasks](#detailed-tasks)
5. [Testing Strategy](#testing-strategy)
6. [Dependencies](#dependencies)
7. [Risk Management](#risk-management)

## Overview

This plan follows a Test-Driven Development (TDD) approach, implementing features in small, testable increments. Each phase includes:

1. **Red:** Write failing tests
2. **Green:** Write minimal code to pass tests
3. **Refactor:** Improve code while keeping tests green
4. **Document:** Update documentation

### Principles

- Write tests before implementation
- Each commit should have passing tests
- Maintain test coverage > 80%
- Integration tests validate complete features
- Manual testing checklist for UI components

## TDD Approach

### Test-First Workflow

```
For each feature:
1. Write test case(s) describing expected behavior
2. Run tests - verify they fail (Red)
3. Write minimal implementation
4. Run tests - verify they pass (Green)
5. Refactor code for clarity/performance
6. Run tests - verify still passing
7. Commit changes
8. Update documentation
```

### Test Levels

1. **Unit Tests** - Individual functions and modules
2. **Integration Tests** - Component interactions
3. **End-to-End Tests** - Complete user workflows
4. **Manual Tests** - UI/UX validation

## Phase Breakdown

### Phase 1: Core Extensions (Week 1)
**Goal:** Add examples, serialization, and basic CLI structure

**Estimated Time:** 20-25 hours

**Deliverables:**
- Training examples (AND, OR, XOR)
- Network serialization (Checkpoint + Model)
- Basic CLI scaffold with `train` command
- Unit tests for all new modules

### Phase 2: Checkpointing & Training Controller (Week 2)
**Goal:** Implement training controller with checkpoint support

**Estimated Time:** 15-20 hours

**Deliverables:**
- TrainingController with callback system
- Checkpoint save/load functionality
- CLI commands for resume and eval
- Integration tests for training workflows

### Phase 3: CLI Completion (Week 2-3)
**Goal:** Complete all CLI commands and user experience

**Estimated Time:** 10-15 hours

**Deliverables:**
- All CLI commands implemented
- Progress bars and output formatting
- Error handling and user messages
- CLI integration tests

### Phase 4: Web Server (Week 3)
**Goal:** HTTP server with SSE for real-time updates

**Estimated Time:** 15-20 hours

**Deliverables:**
- Axum server with REST API
- SSE event stream
- Embedded static files
- Server integration tests

### Phase 5: WASM & Web UI (Week 3-4)
**Goal:** Browser-based training visualization

**Estimated Time:** 20-25 hours

**Deliverables:**
- WASM bindings for Network
- HTML/CSS/JS interface
- Real-time chart rendering
- Manual UI tests

### Phase 6: Integration & Polish (Week 4-5)
**Goal:** End-to-end testing, documentation, and refinement

**Estimated Time:** 15-20 hours

**Deliverables:**
- Complete end-to-end tests
- Presentation script
- Updated README and docs
- Performance optimizations

## Detailed Tasks

### Phase 1: Core Extensions

#### Task 1.1: Examples Module (TDD)

**Test Cases:**
```rust
// tests/unit/examples_tests.rs

#[test]
fn test_get_and_example() {
    let ex = get_example("and").expect("AND example should exist");
    assert_eq!(ex.name, "and");
    assert_eq!(ex.inputs.len(), 4);
    assert_eq!(ex.targets.len(), 4);
    assert_eq!(ex.recommended_arch, vec![2, 2, 1]);
}

#[test]
fn test_get_or_example() {
    let ex = get_example("or").expect("OR example should exist");
    assert_eq!(ex.name, "or");
    assert_eq!(ex.recommended_arch, vec![2, 2, 1]);
}

#[test]
fn test_get_xor_example() {
    let ex = get_example("xor").expect("XOR example should exist");
    assert_eq!(ex.name, "xor");
    assert_eq!(ex.recommended_arch, vec![2, 3, 1]);
}

#[test]
fn test_list_examples() {
    let examples = list_examples();
    assert_eq!(examples.len(), 3);
    assert!(examples.contains(&"and"));
    assert!(examples.contains(&"or"));
    assert!(examples.contains(&"xor"));
}

#[test]
fn test_invalid_example() {
    assert!(get_example("invalid").is_none());
}

#[test]
fn test_example_data_validity() {
    let ex = get_example("xor").unwrap();

    // All inputs should be 2D
    for input in &ex.inputs {
        assert_eq!(input.len(), 2);
    }

    // All targets should be 1D
    for target in &ex.targets {
        assert_eq!(target.len(), 1);
    }

    // Input count should match target count
    assert_eq!(ex.inputs.len(), ex.targets.len());
}
```

**Implementation Steps:**
1. Create `neural-network/src/examples.rs`
2. Define `Example` struct
3. Implement `get_example()` with static data
4. Implement `list_examples()`
5. Run tests - all should pass
6. Commit: "Add training examples module"

**Time:** 2-3 hours

#### Task 1.2: Network Serialization (TDD)

**Test Cases:**
```rust
// tests/unit/serialization_tests.rs

#[test]
fn test_serialize_network() {
    let network = Network::new(vec![2, 3, 1], SIGMOID, 0.5);
    let json = serde_json::to_string(&network).expect("Serialization failed");
    assert!(json.contains("layers"));
    assert!(json.contains("weights"));
}

#[test]
fn test_deserialize_network() {
    let network = Network::new(vec![2, 3, 1], SIGMOID, 0.5);
    let json = serde_json::to_string(&network).unwrap();
    let restored: Network = serde_json::from_str(&json).expect("Deserialization failed");

    assert_eq!(network.layers, restored.layers);
}

#[test]
fn test_serialization_roundtrip() {
    let mut network = Network::new(vec![2, 3, 1], SIGMOID, 0.5);

    // Train a bit to get non-random weights
    let inputs = vec![vec![0.0, 0.0]];
    let targets = vec![vec![0.0]];
    network.train(inputs, targets, 10);

    // Serialize and deserialize
    let json = serde_json::to_string(&network).unwrap();
    let restored: Network = serde_json::from_str(&json).unwrap();

    // Predictions should be identical
    let test_input = Matrix::from(vec![0.5, 0.5]);
    let pred1 = network.feed_forward(test_input.clone());
    let pred2 = restored.feed_forward(test_input);

    assert_eq!(pred1.data, pred2.data);
}
```

**Implementation Steps:**
1. Add `serde` feature to `matrix` crate
2. Derive `Serialize`, `Deserialize` for `Matrix`
3. Add `serde` to `neural-network` crate
4. Derive `Serialize`, `Deserialize` for `Network`
5. Handle `Activation` serialization (function pointers -> string)
6. Run tests - all should pass
7. Commit: "Add serde serialization for Network and Matrix"

**Time:** 3-4 hours

#### Task 1.3: Checkpoint Module (TDD)

**Test Cases:**
```rust
// tests/unit/checkpoint_tests.rs

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

    let checkpoint = network.to_checkpoint(metadata);
    assert_eq!(checkpoint.metadata.epoch, 1000);
}

#[test]
fn test_save_and_load_checkpoint() {
    let mut network = Network::new(vec![2, 3, 1], SIGMOID, 0.5);
    network.train(vec![vec![0.0, 0.0]], vec![vec![0.0]], 10);

    let temp_dir = tempdir::TempDir::new("checkpoints").unwrap();
    let path = temp_dir.path().join("test_checkpoint.json");

    let metadata = CheckpointMetadata {
        version: "1.0".to_string(),
        example: "xor".to_string(),
        epoch: 100,
        total_epochs: 1000,
        learning_rate: 0.5,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    network.save_checkpoint(&path, metadata.clone()).expect("Save failed");
    assert!(path.exists());

    let (restored, restored_meta) = Network::load_checkpoint(&path).expect("Load failed");
    assert_eq!(restored_meta.epoch, 100);

    // Predictions should match
    let input = Matrix::from(vec![0.5, 0.5]);
    assert_eq!(
        network.feed_forward(input.clone()).data,
        restored.feed_forward(input).data
    );
}

#[test]
fn test_checkpoint_file_format() {
    let network = Network::new(vec![2, 3, 1], SIGMOID, 0.5);
    let temp_dir = tempdir::TempDir::new("checkpoints").unwrap();
    let path = temp_dir.path().join("test.json");

    let metadata = CheckpointMetadata {
        version: "1.0".to_string(),
        example: "xor".to_string(),
        epoch: 100,
        total_epochs: 1000,
        learning_rate: 0.5,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    network.save_checkpoint(&path, metadata).unwrap();

    // Verify JSON structure
    let contents = std::fs::read_to_string(&path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&contents).unwrap();

    assert!(json["metadata"].is_object());
    assert!(json["network"].is_object());
    assert_eq!(json["version"], "1.0");
}

#[test]
fn test_invalid_checkpoint_version() {
    let temp_dir = tempdir::TempDir::new("checkpoints").unwrap();
    let path = temp_dir.path().join("bad.json");

    std::fs::write(&path, r#"{"version":"999.0","metadata":{},"network":{}}"#).unwrap();

    let result = Network::load_checkpoint(&path);
    assert!(result.is_err());
}
```

**Implementation Steps:**
1. Create `neural-network/src/checkpoint.rs`
2. Define `Checkpoint` and `CheckpointMetadata` structs
3. Implement `Network::to_checkpoint()`
4. Implement `Network::from_checkpoint()`
5. Implement `Network::save_checkpoint()` (file I/O)
6. Implement `Network::load_checkpoint()` (file I/O + validation)
7. Add version checking logic
8. Run tests - all should pass
9. Commit: "Add checkpoint module with save/load"

**Time:** 4-5 hours

#### Task 1.4: Basic CLI Scaffold (TDD)

**Test Cases:**
```rust
// tests/integration/cli_tests.rs

#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "neural-net-cli", "--", "--help"])
        .output()
        .expect("Failed to run CLI");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Neural Network Demonstration Platform"));
    assert!(stdout.contains("train"));
    assert!(stdout.contains("eval"));
    assert!(stdout.contains("serve"));
}

#[test]
fn test_cli_version() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "neural-net-cli", "--", "--version"])
        .output()
        .expect("Failed to run CLI");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_train_command_requires_example() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "neural-net-cli", "--", "train"])
        .output()
        .expect("Failed to run CLI");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("required"));
}

#[test]
fn test_list_command() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "neural-net-cli", "--", "list"])
        .output()
        .expect("Failed to run CLI");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("and"));
    assert!(stdout.contains("or"));
    assert!(stdout.contains("xor"));
}
```

**Implementation Steps:**
1. Create `neural-net-cli` crate with binary
2. Add `clap` dependency with derive feature
3. Define CLI structure with subcommands
4. Implement `list` command (easiest first)
5. Implement basic `train` command scaffold (no training yet)
6. Run integration tests
7. Commit: "Add CLI scaffold with clap"

**Time:** 3-4 hours

#### Task 1.5: Train Command Implementation (TDD)

**Test Cases:**
```rust
// tests/integration/cli_tests.rs

#[test]
fn test_train_xor_basic() {
    let temp_dir = tempdir::TempDir::new("models").unwrap();
    let output_path = temp_dir.path().join("xor_model.json");

    let output = Command::new("cargo")
        .args(&[
            "run", "--bin", "neural-net-cli", "--",
            "train",
            "--example", "xor",
            "--epochs", "1000",
            "--output", output_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run CLI");

    assert!(output.status.success());
    assert!(output_path.exists());

    // Verify model file is valid JSON
    let contents = std::fs::read_to_string(&output_path).unwrap();
    let _model: serde_json::Value = serde_json::from_str(&contents).unwrap();
}

#[test]
fn test_train_with_custom_architecture() {
    let temp_dir = tempdir::TempDir::new("models").unwrap();
    let output_path = temp_dir.path().join("model.json");

    let output = Command::new("cargo")
        .args(&[
            "run", "--bin", "neural-net-cli", "--",
            "train",
            "--example", "xor",
            "--epochs", "500",
            "--hidden-layers", "4,4",
            "--learning-rate", "0.3",
            "--output", output_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run CLI");

    assert!(output.status.success());
}

#[test]
fn test_train_invalid_example() {
    let output = Command::new("cargo")
        .args(&[
            "run", "--bin", "neural-net-cli", "--",
            "train",
            "--example", "invalid",
            "--epochs", "100",
        ])
        .output()
        .expect("Failed to run CLI");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown example"));
}
```

**Implementation Steps:**
1. Implement `commands/train.rs`
2. Parse and validate arguments
3. Load example data
4. Create network with specified architecture
5. Run training loop
6. Save model if `--output` specified
7. Print results to stdout
8. Add error handling
9. Run tests
10. Commit: "Implement train command"

**Time:** 4-5 hours

**Phase 1 Total:** 16-21 hours

### Phase 2: Training Controller & Checkpointing

#### Task 2.1: TrainingController (TDD)

**Test Cases:**
```rust
// tests/unit/training_controller_tests.rs

#[test]
fn test_training_controller_basic() {
    let network = Network::new(vec![2, 2, 1], SIGMOID, 0.5);
    let config = TrainingConfig {
        epochs: 100,
        checkpoint_interval: None,
        checkpoint_path: None,
        verbose: false,
    };

    let mut controller = TrainingController::new(network, config);

    let inputs = vec![vec![0.0, 0.0], vec![1.0, 1.0]];
    let targets = vec![vec![0.0], vec![1.0]];

    controller.train(inputs, targets).expect("Training failed");
}

#[test]
fn test_training_controller_callbacks() {
    let network = Network::new(vec![2, 2, 1], SIGMOID, 0.5);
    let config = TrainingConfig {
        epochs: 10,
        checkpoint_interval: None,
        checkpoint_path: None,
        verbose: false,
    };

    let mut controller = TrainingController::new(network, config);

    let callback_invocations = Arc::new(Mutex::new(0));
    let invocations_clone = callback_invocations.clone();

    controller.add_callback(Box::new(move |_epoch, _loss, _preds| {
        *invocations_clone.lock().unwrap() += 1;
    }));

    controller.train(vec![vec![0.0, 0.0]], vec![vec![0.0]]).unwrap();

    assert_eq!(*callback_invocations.lock().unwrap(), 10);
}

#[test]
fn test_training_controller_auto_checkpoint() {
    let temp_dir = tempdir::TempDir::new("checkpoints").unwrap();
    let checkpoint_path = temp_dir.path().join("auto_checkpoint.json");

    let network = Network::new(vec![2, 2, 1], SIGMOID, 0.5);
    let config = TrainingConfig {
        epochs: 100,
        checkpoint_interval: Some(25),
        checkpoint_path: Some(checkpoint_path.clone()),
        verbose: false,
    };

    let mut controller = TrainingController::new(network, config);
    controller.train(vec![vec![0.0, 0.0]], vec![vec![0.0]]).unwrap();

    assert!(checkpoint_path.exists());

    // Verify checkpoint metadata
    let (_, metadata) = Network::load_checkpoint(&checkpoint_path).unwrap();
    assert!(metadata.epoch >= 75); // Should be 75 or 100
}
```

**Implementation Steps:**
1. Create `neural-network/src/training.rs`
2. Define `TrainingController` struct
3. Define `TrainingCallback` trait
4. Implement `TrainingController::new()`
5. Implement `add_callback()`
6. Implement `train()` with callback invocation
7. Add checkpoint logic (if configured)
8. Run tests
9. Commit: "Add TrainingController with callbacks"

**Time:** 5-6 hours

#### Task 2.2: Resume from Checkpoint (TDD)

**Test Cases:**
```rust
// tests/integration/checkpoint_resume_tests.rs

#[test]
fn test_resume_training() {
    let temp_dir = tempdir::TempDir::new("checkpoints").unwrap();
    let checkpoint_path = temp_dir.path().join("resume_test.json");

    // Train for 500 epochs and checkpoint
    let network = Network::new(vec![2, 3, 1], SIGMOID, 0.5);
    let config = TrainingConfig {
        epochs: 500,
        checkpoint_interval: Some(500),
        checkpoint_path: Some(checkpoint_path.clone()),
        verbose: false,
    };

    let mut controller1 = TrainingController::new(network, config);
    let example = get_example("xor").unwrap();
    controller1.train(example.inputs.clone(), example.targets.clone()).unwrap();

    // Resume and train for another 500
    let mut controller2 = TrainingController::resume_from_checkpoint(&checkpoint_path).unwrap();
    controller2.config.epochs = 1000;
    controller2.train(example.inputs, example.targets).unwrap();

    // Check final epoch count in new checkpoint
    let (_, metadata) = Network::load_checkpoint(&checkpoint_path).unwrap();
    assert_eq!(metadata.epoch, 1000);
}

#[test]
fn test_resume_preserves_learning() {
    let temp_dir = tempdir::TempDir::new("checkpoints").unwrap();
    let checkpoint_path = temp_dir.path().join("learning_test.json");

    let example = get_example("and").unwrap();

    // Train for 100 epochs
    let network = Network::new(vec![2, 2, 1], SIGMOID, 0.5);
    let config = TrainingConfig {
        epochs: 100,
        checkpoint_interval: Some(100),
        checkpoint_path: Some(checkpoint_path.clone()),
        verbose: false,
    };

    let mut controller = TrainingController::new(network, config);
    controller.train(example.inputs.clone(), example.targets.clone()).unwrap();

    // Get predictions after initial training
    let (mut network_checkpoint, _) = Network::load_checkpoint(&checkpoint_path).unwrap();
    let pred1 = network_checkpoint.feed_forward(Matrix::from(vec![1.0, 1.0]));

    // Resume and train more
    let mut controller2 = TrainingController::resume_from_checkpoint(&checkpoint_path).unwrap();
    controller2.config.epochs = 200;
    controller2.train(example.inputs, example.targets).unwrap();

    let (mut network_final, _) = Network::load_checkpoint(&checkpoint_path).unwrap();
    let pred2 = network_final.feed_forward(Matrix::from(vec![1.0, 1.0]));

    // Prediction should be closer to target (1.0) after more training
    assert!(pred2.data[0] > pred1.data[0]);
}
```

**Implementation Steps:**
1. Implement `TrainingController::resume_from_checkpoint()`
2. Load network and metadata from checkpoint
3. Adjust epoch counter to continue from checkpoint
4. Validate configuration compatibility
5. Run tests
6. Commit: "Add resume from checkpoint functionality"

**Time:** 3-4 hours

#### Task 2.3: CLI Resume Command (TDD)

**Test Cases:**
```rust
// tests/integration/cli_resume_tests.rs

#[test]
fn test_cli_train_with_checkpoint() {
    let temp_dir = tempdir::TempDir::new("checkpoints").unwrap();
    let checkpoint_path = temp_dir.path().join("cli_checkpoint.json");

    let output = Command::new("cargo")
        .args(&[
            "run", "--bin", "neural-net-cli", "--",
            "train",
            "--example", "xor",
            "--epochs", "1000",
            "--checkpoint", checkpoint_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run CLI");

    assert!(output.status.success());
    assert!(checkpoint_path.exists());
}

#[test]
fn test_cli_resume_training() {
    let temp_dir = tempdir::TempDir::new("checkpoints").unwrap();
    let checkpoint_path = temp_dir.path().join("resume.json");

    // Initial training
    Command::new("cargo")
        .args(&[
            "run", "--bin", "neural-net-cli", "--",
            "train",
            "--example", "xor",
            "--epochs", "500",
            "--checkpoint", checkpoint_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run CLI");

    // Resume training
    let output = Command::new("cargo")
        .args(&[
            "run", "--bin", "neural-net-cli", "--",
            "train",
            "--resume", checkpoint_path.to_str().unwrap(),
            "--epochs", "1000",
        ])
        .output()
        .expect("Failed to run CLI");

    assert!(output.status.success());

    // Verify epoch count increased
    let (_, metadata) = Network::load_checkpoint(&checkpoint_path).unwrap();
    assert_eq!(metadata.epoch, 1000);
}
```

**Implementation Steps:**
1. Update `commands/train.rs` to handle `--checkpoint` flag
2. Add periodic checkpoint saves during training
3. Handle `--resume` flag
4. Load checkpoint and resume training
5. Add clear output messages (epoch X of Y, resumed from checkpoint)
6. Run tests
7. Commit: "Add checkpoint and resume flags to train command"

**Time:** 3-4 hours

**Phase 2 Total:** 11-14 hours

### Phase 3: CLI Completion

#### Task 3.1: Eval Command (TDD)

**Test Cases:**
```rust
// tests/integration/cli_eval_tests.rs

#[test]
fn test_eval_single_input() {
    // First train a model
    let temp_dir = tempdir::TempDir::new("models").unwrap();
    let model_path = temp_dir.path().join("eval_test.json");

    Command::new("cargo")
        .args(&[
            "run", "--bin", "neural-net-cli", "--",
            "train",
            "--example", "and",
            "--epochs", "5000",
            "--output", model_path.to_str().unwrap(),
        ])
        .output()
        .expect("Training failed");

    // Evaluate with single input
    let output = Command::new("cargo")
        .args(&[
            "run", "--bin", "neural-net-cli", "--",
            "eval",
            "--model", model_path.to_str().unwrap(),
            "--input", "1.0,1.0",
        ])
        .output()
        .expect("Eval failed");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should output prediction close to 1.0
    assert!(stdout.contains("Output:"));
}

#[test]
fn test_eval_test_all() {
    let temp_dir = tempdir::TempDir::new("models").unwrap();
    let model_path = temp_dir.path().join("eval_all_test.json");

    Command::new("cargo")
        .args(&[
            "run", "--bin", "neural-net-cli", "--",
            "train",
            "--example", "xor",
            "--epochs", "10000",
            "--output", model_path.to_str().unwrap(),
        ])
        .output()
        .expect("Training failed");

    let output = Command::new("cargo")
        .args(&[
            "run", "--bin", "neural-net-cli", "--",
            "eval",
            "--model", model_path.to_str().unwrap(),
            "--test-all",
        ])
        .output()
        .expect("Eval failed");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("[0.0, 0.0]"));
    assert!(stdout.contains("[0.0, 1.0]"));
    assert!(stdout.contains("[1.0, 0.0]"));
    assert!(stdout.contains("[1.0, 1.0]"));
}
```

**Implementation Steps:**
1. Create `commands/eval.rs`
2. Implement `--input` parsing
3. Load model from file
4. Run prediction
5. Implement `--test-all` flag
6. Load example metadata from model
7. Run all test cases and display results
8. Format output (table format)
9. Run tests
10. Commit: "Add eval command"

**Time:** 3-4 hours

#### Task 3.2: Info Command (TDD)

**Test Cases:**
```rust
// tests/integration/cli_info_tests.rs

#[test]
fn test_info_command() {
    let temp_dir = tempdir::TempDir::new("models").unwrap();
    let model_path = temp_dir.path().join("info_test.json");

    Command::new("cargo")
        .args(&[
            "run", "--bin", "neural-net-cli", "--",
            "train",
            "--example", "xor",
            "--epochs", "1000",
            "--output", model_path.to_str().unwrap(),
        ])
        .output()
        .expect("Training failed");

    let output = Command::new("cargo")
        .args(&[
            "run", "--bin", "neural-net-cli", "--",
            "info",
            model_path.to_str().unwrap(),
        ])
        .output()
        .expect("Info failed");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Architecture"));
    assert!(stdout.contains("[2, 3, 1]"));
    assert!(stdout.contains("Epochs"));
}
```

**Implementation Steps:**
1. Create `commands/info.rs`
2. Load model from file
3. Extract metadata
4. Format and display:
   - Architecture
   - Training epochs
   - Example name
   - Timestamp
   - File size
5. Run tests
6. Commit: "Add info command"

**Time:** 2-3 hours

#### Task 3.3: Progress Bars & Formatting (Manual Testing)

**Implementation Steps:**
1. Add `indicatif` dependency
2. Create progress bar callback
3. Update train command to use progress bar
4. Add verbose flag for detailed output
5. Format final output (accuracy, loss)
6. Manual testing:
   - Train XOR with progress bar
   - Verify smooth updates
   - Test verbose mode
   - Test non-TTY output (pipe to file)
7. Commit: "Add progress bars and formatted output"

**Time:** 3-4 hours

**Phase 3 Total:** 8-11 hours

### Phase 4: Web Server

#### Task 4.1: Server Scaffold (TDD)

**Test Cases:**
```rust
// tests/integration/server_tests.rs

#[tokio::test]
async fn test_server_starts() {
    let server = spawn_test_server().await;

    let resp = reqwest::get(&format!("{}/", server.addr))
        .await
        .expect("Failed to fetch");

    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn test_api_examples_endpoint() {
    let server = spawn_test_server().await;

    let resp = reqwest::get(&format!("{}/api/examples", server.addr))
        .await
        .expect("Failed to fetch");

    assert_eq!(resp.status(), 200);

    let examples: Vec<String> = resp.json().await.unwrap();
    assert_eq!(examples.len(), 3);
}
```

**Implementation Steps:**
1. Create `neural-net-cli/src/server/mod.rs`
2. Add Axum dependencies
3. Create basic server with root route
4. Implement `/api/examples` endpoint
5. Add error handling
6. Run tests
7. Commit: "Add Axum server scaffold"

**Time:** 3-4 hours

#### Task 4.2: Training API Endpoints (TDD)

**Test Cases:**
```rust
// tests/integration/server_training_tests.rs

#[tokio::test]
async fn test_start_training() {
    let server = spawn_test_server().await;

    let request = TrainRequest {
        example: "and".to_string(),
        epochs: 100,
        learning_rate: 0.5,
        hidden_layers: vec![2],
    };

    let client = reqwest::Client::new();
    let resp = client
        .post(&format!("{}/api/train/start", server.addr))
        .json(&request)
        .send()
        .await
        .expect("Failed to post");

    assert_eq!(resp.status(), 200);

    let response: TrainResponse = resp.json().await.unwrap();
    assert!(!response.session_id.is_empty());
    assert_eq!(response.status, "started");
}

#[tokio::test]
async fn test_stop_training() {
    let server = spawn_test_server().await;
    let session_id = start_training_session(&server, "and", 10000).await;

    tokio::time::sleep(Duration::from_millis(100)).await;

    let client = reqwest::Client::new();
    let resp = client
        .post(&format!("{}/api/train/stop", server.addr))
        .send()
        .await
        .expect("Failed to post");

    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn test_concurrent_training_rejected() {
    let server = spawn_test_server().await;

    // Start first session
    let _session1 = start_training_session(&server, "xor", 50000).await;

    // Try to start second session
    let request = TrainRequest {
        example: "or".to_string(),
        epochs: 100,
        learning_rate: 0.5,
        hidden_layers: vec![2],
    };

    let client = reqwest::Client::new();
    let resp = client
        .post(&format!("{}/api/train/start", server.addr))
        .json(&request)
        .send()
        .await
        .expect("Failed to post");

    assert_eq!(resp.status(), 409); // Conflict
}
```

**Implementation Steps:**
1. Create `server/routes.rs` with training endpoints
2. Implement `POST /api/train/start`
3. Implement `POST /api/train/stop`
4. Add server state with `Arc<Mutex<Option<TrainingSession>>>`
5. Handle concurrent request rejection
6. Spawn training in background task
7. Run tests
8. Commit: "Add training API endpoints"

**Time:** 5-6 hours

#### Task 4.3: Server-Sent Events (TDD)

**Test Cases:**
```rust
// tests/integration/server_sse_tests.rs

#[tokio::test]
async fn test_sse_stream() {
    let server = spawn_test_server().await;

    // Start training
    start_training_session(&server, "and", 1000).await;

    // Connect to SSE stream
    let resp = reqwest::get(&format!("{}/api/events", server.addr))
        .await
        .expect("Failed to connect to SSE");

    assert_eq!(resp.status(), 200);
    assert_eq!(
        resp.headers().get("content-type").unwrap(),
        "text/event-stream"
    );

    // Read first few events
    let mut stream = resp.bytes_stream();
    let mut events = Vec::new();

    for _ in 0..5 {
        if let Some(Ok(bytes)) = stream.next().await {
            let text = String::from_utf8_lossy(&bytes);
            if text.contains("event:") {
                events.push(text.to_string());
            }
        }
    }

    assert!(!events.is_empty());
}

#[tokio::test]
async fn test_sse_training_events() {
    let server = spawn_test_server().await;

    // Spawn SSE listener
    let events_task = tokio::spawn(async move {
        collect_sse_events(&server, Duration::from_secs(5)).await
    });

    // Start training
    tokio::time::sleep(Duration::from_millis(100)).await;
    start_training_session(&server, "and", 1000).await;

    // Collect events
    let events = events_task.await.unwrap();

    // Verify we got training events
    assert!(events.iter().any(|e| e.contains("training_started")));
    assert!(events.iter().any(|e| e.contains("epoch_complete")));
}
```

**Implementation Steps:**
1. Add `tower-http` and SSE support
2. Create `GET /api/events` endpoint
3. Use `broadcast::channel` for event distribution
4. Emit events from training callback
5. Format events as SSE (event: type\ndata: json\n\n)
6. Handle client disconnection
7. Run tests
8. Commit: "Add Server-Sent Events for training updates"

**Time:** 5-6 hours

#### Task 4.4: Static File Serving (TDD)

**Test Cases:**
```rust
// tests/integration/server_static_tests.rs

#[tokio::test]
async fn test_serve_index_html() {
    let server = spawn_test_server().await;

    let resp = reqwest::get(&format!("{}/", server.addr))
        .await
        .expect("Failed to fetch");

    assert_eq!(resp.status(), 200);
    assert_eq!(
        resp.headers().get("content-type").unwrap(),
        "text/html"
    );

    let body = resp.text().await.unwrap();
    assert!(body.contains("<!DOCTYPE html>"));
    assert!(body.contains("Neural Network Demo"));
}

#[tokio::test]
async fn test_serve_wasm() {
    let server = spawn_test_server().await;

    let resp = reqwest::get(&format!("{}/app.wasm", server.addr))
        .await
        .expect("Failed to fetch");

    assert_eq!(resp.status(), 200);
    assert_eq!(
        resp.headers().get("content-type").unwrap(),
        "application/wasm"
    );
}

#[tokio::test]
async fn test_404_for_missing_file() {
    let server = spawn_test_server().await;

    let resp = reqwest::get(&format!("{}/nonexistent.js", server.addr))
        .await
        .expect("Failed to fetch");

    assert_eq!(resp.status(), 404);
}
```

**Implementation Steps:**
1. Create `server/static_files.rs`
2. Use `include_bytes!` to embed files at compile time
3. Create route handler for static files
4. Set appropriate MIME types
5. Handle 404s gracefully
6. Run tests
7. Commit: "Add static file serving with embedded assets"

**Time:** 2-3 hours

**Phase 4 Total:** 15-19 hours

### Phase 5: WASM & Web UI

#### Task 5.1: WASM Bindings (TDD)

**Test Cases:**
```rust
// neural-net-wasm/tests/wasm_tests.rs

#[wasm_bindgen_test]
fn test_create_network() {
    let network = NetworkHandle::new(vec![2, 3, 1], 0.5);
    // If we get here without panic, creation succeeded
    assert!(true);
}

#[wasm_bindgen_test]
fn test_predict() {
    let mut network = NetworkHandle::new(vec![2, 2, 1], 0.5);
    let result = network.predict(vec![0.0, 0.0]);

    assert_eq!(result.len(), 1);
    assert!(result[0] >= 0.0 && result[0] <= 1.0);
}

#[wasm_bindgen_test]
fn test_train_step() {
    let mut network = NetworkHandle::new(vec![2, 2, 1], 0.5);

    let loss1 = network.train_step(vec![0.0, 0.0], vec![0.0]);
    let loss2 = network.train_step(vec![0.0, 0.0], vec![0.0]);

    // Loss should decrease (or at least not increase significantly)
    assert!(loss2 <= loss1 * 1.1);
}

#[wasm_bindgen_test]
fn test_serialization() {
    let mut network = NetworkHandle::new(vec![2, 3, 1], 0.5);
    network.train_step(vec![0.0, 0.0], vec![0.0]);

    let json = network.to_json();
    assert!(!json.is_empty());

    let restored = NetworkHandle::from_json(&json).expect("Deserialization failed");
    let pred1 = network.predict(vec![0.5, 0.5]);
    let pred2 = restored.predict(vec![0.5, 0.5]);

    assert_eq!(pred1, pred2);
}
```

**Implementation Steps:**
1. Create `neural-net-wasm` crate
2. Add wasm-bindgen dependencies
3. Implement `NetworkHandle` wrapper
4. Implement `new`, `predict`, `train_step`
5. Add serialization methods
6. Setup wasm-bindgen-test
7. Run tests with `wasm-pack test --node`
8. Commit: "Add WASM bindings for Network"

**Time:** 5-6 hours

#### Task 5.2: Web UI HTML/CSS (Manual Testing)

**Implementation Steps:**
1. Create `neural-net-wasm/web/index.html`
2. Design layout:
   - Header with title
   - Left panel: controls
   - Center panel: visualizations
   - Right panel: results
3. Create `styles.css`
4. Add responsive design
5. Test in multiple browsers
6. Commit: "Add web UI HTML and CSS"

**Time:** 4-5 hours

#### Task 5.3: Web UI JavaScript (Manual Testing)

**Implementation Steps:**
1. Create `neural-net-wasm/web/app.js`
2. Initialize WASM module
3. Implement example selection
4. Implement training controls (start/pause/stop)
5. Connect to SSE endpoint
6. Update UI on events
7. Handle errors gracefully
8. Test interactions manually
9. Commit: "Add web UI JavaScript logic"

**Time:** 6-7 hours

#### Task 5.4: Chart Visualization (Manual Testing)

**Implementation Steps:**
1. Create `neural-net-wasm/web/chart.js`
2. Use Canvas API for line chart
3. Implement real-time updates
4. Add axis labels
5. Color-code accuracy zones
6. Add zoom/pan (optional)
7. Test performance with rapid updates
8. Commit: "Add loss chart visualization"

**Time:** 4-5 hours

**Phase 5 Total:** 19-23 hours

### Phase 6: Integration & Polish

#### Task 6.1: End-to-End Testing (TDD)

**Test Cases:**
```rust
// tests/e2e/full_workflow_tests.rs

#[test]
fn test_complete_training_workflow() {
    // Train -> Save -> Eval
    let temp_dir = tempdir::TempDir::new("e2e").unwrap();
    let model_path = temp_dir.path().join("model.json");

    // Step 1: Train
    let train_output = Command::new("cargo")
        .args(&[
            "run", "--bin", "neural-net-cli", "--",
            "train",
            "--example", "and",
            "--epochs", "5000",
            "--output", model_path.to_str().unwrap(),
        ])
        .output()
        .expect("Training failed");

    assert!(train_output.status.success());
    assert!(model_path.exists());

    // Step 2: Eval
    let eval_output = Command::new("cargo")
        .args(&[
            "run", "--bin", "neural-net-cli", "--",
            "eval",
            "--model", model_path.to_str().unwrap(),
            "--test-all",
        ])
        .output()
        .expect("Eval failed");

    assert!(eval_output.status.success());
    let stdout = String::from_utf8_lossy(&eval_output.stdout);

    // Verify predictions are reasonable
    assert!(stdout.contains("1.0, 1.0"));
    // AND gate: 1,1 should output close to 1.0
}

#[test]
fn test_checkpoint_resume_workflow() {
    let temp_dir = tempdir::TempDir::new("e2e").unwrap();
    let checkpoint_path = temp_dir.path().join("checkpoint.json");
    let model_path = temp_dir.path().join("final_model.json");

    // Step 1: Train 500 epochs with checkpoint
    Command::new("cargo")
        .args(&[
            "run", "--bin", "neural-net-cli", "--",
            "train",
            "--example", "xor",
            "--epochs", "500",
            "--checkpoint", checkpoint_path.to_str().unwrap(),
        ])
        .output()
        .expect("Initial training failed");

    assert!(checkpoint_path.exists());

    // Step 2: Resume and train to 1000 epochs
    let resume_output = Command::new("cargo")
        .args(&[
            "run", "--bin", "neural-net-cli", "--",
            "train",
            "--resume", checkpoint_path.to_str().unwrap(),
            "--epochs", "1000",
            "--output", model_path.to_str().unwrap(),
        ])
        .output()
        .expect("Resume training failed");

    assert!(resume_output.status.success());
    assert!(model_path.exists());

    // Step 3: Verify model works
    let eval_output = Command::new("cargo")
        .args(&[
            "run", "--bin", "neural-net-cli", "--",
            "eval",
            "--model", model_path.to_str().unwrap(),
            "--input", "0.0,1.0",
        ])
        .output()
        .expect("Eval failed");

    assert!(eval_output.status.success());
}
```

**Implementation Steps:**
1. Create comprehensive e2e test suite
2. Test all command combinations
3. Test error scenarios
4. Verify file I/O correctness
5. Run tests
6. Fix any issues found
7. Commit: "Add end-to-end integration tests"

**Time:** 5-6 hours

#### Task 6.2: Presentation Script

**Implementation Steps:**
1. Create `documentation/presentation-script.md`
2. Write step-by-step demo script
3. Include expected outputs
4. Add troubleshooting tips
5. Test script execution
6. Commit: "Add presentation script"

**Time:** 3-4 hours

#### Task 6.3: Documentation Updates

**Implementation Steps:**
1. Update `README.md` with new features
2. Add installation instructions
3. Add usage examples for all commands
4. Document web UI usage
5. Add screenshots (manual)
6. Update `CLAUDE.md`
7. Commit: "Update documentation"

**Time:** 4-5 hours

#### Task 6.4: Performance Optimization

**Manual Testing:**
1. Profile training performance
2. Optimize hot paths if needed
3. Test WASM bundle size
4. Optimize chart rendering
5. Test on slower machines
6. Commit: "Performance optimizations"

**Time:** 3-4 hours

**Phase 6 Total:** 15-19 hours

## Testing Strategy

### Test Pyramid

```
                 E2E Tests (10%)
                /              \
           Integration Tests (30%)
          /                        \
      Unit Tests (60%)
```

### Coverage Goals

- **Unit Tests:** > 85% line coverage
- **Integration Tests:** All CLI commands, all API endpoints
- **E2E Tests:** Complete user workflows
- **Manual Tests:** UI/UX, browser compatibility

### Continuous Testing

```bash
# Run all tests
cargo test --workspace

# Run with coverage
cargo tarpaulin --workspace --out Html

# Run WASM tests
cd neural-net-wasm && wasm-pack test --node

# Run clippy
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Check formatting
cargo fmt --all -- --check
```

### Pre-Commit Checklist

- [ ] All tests pass
- [ ] No clippy warnings
- [ ] Code formatted
- [ ] Documentation updated
- [ ] Commit message descriptive

## Dependencies

### Workspace Dependencies (Cargo.toml)

```toml
[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
thiserror = "1"
tokio = { version = "1", features = ["full"] }
clap = { version = "4", features = ["derive"] }
```

### Build Tools

```bash
# Install wasm-pack
cargo install wasm-pack

# Install cargo-watch (optional, for development)
cargo install cargo-watch

# Install tarpaulin (optional, for coverage)
cargo install cargo-tarpaulin
```

## Risk Management

### Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| WASM compatibility issues | Medium | High | Early testing, fallback to server-side |
| Performance in browser | Medium | Medium | Optimize hot paths, web workers |
| Checkpoint format changes | Low | High | Version checking, migration support |
| SSE browser compatibility | Low | Medium | Test on all major browsers |

### Schedule Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Underestimated complexity | Medium | Medium | 20% buffer in estimates |
| Testing takes longer | Medium | Low | Prioritize unit tests first |
| Web UI polish takes time | High | Low | MVP first, enhance later |

### Mitigation Strategies

1. **Incremental Delivery:** Each phase delivers working features
2. **Automated Testing:** Catch regressions early
3. **Time Boxes:** Limit time spent on polish features
4. **MVP Focus:** Core functionality before nice-to-haves
5. **Regular Reviews:** Daily progress check against plan

## Success Metrics

### Phase Completion Criteria

Each phase is considered complete when:
- [ ] All unit tests pass
- [ ] Integration tests pass (if applicable)
- [ ] Code review complete
- [ ] Documentation updated
- [ ] Zero compiler warnings
- [ ] Zero clippy warnings

### Final Acceptance Criteria

Project is ready for presentation when:
- [ ] All three examples (AND, OR, XOR) train successfully in CLI
- [ ] Web UI displays training in real-time
- [ ] Models can be checkpointed and resumed
- [ ] Presentation script executes without errors
- [ ] Test coverage > 80%
- [ ] All documentation complete
- [ ] README has clear usage instructions
- [ ] Binary builds successfully on macOS, Linux, Windows

## Next Steps

1. **Review this plan** with stakeholders
2. **Setup development environment** (install tools)
3. **Create feature branch** for Phase 1
4. **Begin Task 1.1** (Examples Module)
5. **Follow TDD workflow** strictly
6. **Daily progress updates** in this document

## Appendix

### Useful Commands

```bash
# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run integration tests only
cargo test --test integration_tests

# Watch mode
cargo watch -x test

# Build WASM
cd neural-net-wasm && wasm-pack build --target web

# Build CLI
cargo build --release --bin neural-net-cli

# Serve locally for testing
python -m http.server 8000 -d neural-net-wasm/pkg
```

### Resources

- [Rust TDD Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [wasm-bindgen Book](https://rustwasm.github.io/wasm-bindgen/)
- [Axum Examples](https://github.com/tokio-rs/axum/tree/main/examples)
- [Clap Derive Reference](https://docs.rs/clap/latest/clap/_derive/)

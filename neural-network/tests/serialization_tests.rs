// Integration tests for Network and Matrix serialization
use neural_network::network::Network;
use neural_network::activations::SIGMOID;
use neural_network::matrix::Matrix;

#[test]
fn test_serialize_matrix() {
    let matrix = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let json = serde_json::to_string(&matrix).expect("Serialization failed");

    assert!(json.contains("rows"));
    assert!(json.contains("cols"));
    assert!(json.contains("data"));
}

#[test]
fn test_deserialize_matrix() {
    let matrix = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let json = serde_json::to_string(&matrix).unwrap();
    let restored: Matrix = serde_json::from_str(&json).expect("Deserialization failed");

    assert_eq!(matrix.rows, restored.rows);
    assert_eq!(matrix.cols, restored.cols);
    assert_eq!(matrix.data, restored.data);
}

#[test]
fn test_matrix_serialization_roundtrip() {
    let original = Matrix::random(3, 4);

    // Serialize and deserialize
    let json = serde_json::to_string(&original).unwrap();
    let restored: Matrix = serde_json::from_str(&json).unwrap();

    // Should be identical (or very close due to JSON floating point precision)
    assert_eq!(original.rows, restored.rows);
    assert_eq!(original.cols, restored.cols);
    assert_eq!(original.data.len(), restored.data.len());

    // Check values are approximately equal (within floating point precision)
    for (orig, rest) in original.data.iter().zip(restored.data.iter()) {
        assert!((orig - rest).abs() < 1e-10, "Values differ: {} vs {}", orig, rest);
    }
}

#[test]
fn test_serialize_network() {
    let network = Network::new(vec![2, 3, 1], SIGMOID, 0.5);
    let json = serde_json::to_string(&network).expect("Serialization failed");

    assert!(json.contains("layers"));
    assert!(json.contains("weights"));
    assert!(json.contains("biases"));
}

#[test]
fn test_deserialize_network() {
    let network = Network::new(vec![2, 3, 1], SIGMOID, 0.5);
    let json = serde_json::to_string(&network).unwrap();
    let restored: Network = serde_json::from_str(&json).expect("Deserialization failed");

    assert_eq!(network.layers, restored.layers);
    assert_eq!(network.weights.len(), restored.weights.len());
    assert_eq!(network.biases.len(), restored.biases.len());
}

#[test]
fn test_network_serialization_roundtrip() {
    let mut network = Network::new(vec![2, 3, 1], SIGMOID, 0.5);

    // Train a bit to get non-random weights
    let inputs = vec![vec![0.0, 0.0]];
    let targets = vec![vec![0.0]];
    network.train(inputs, targets, 10);

    // Serialize and deserialize
    let json = serde_json::to_string(&network).unwrap();
    let mut restored: Network = serde_json::from_str(&json).unwrap();

    // Predictions should be approximately equal (within floating point precision)
    let test_input = Matrix::from(vec![0.5, 0.5]);
    let pred1 = network.feed_forward(test_input.clone());
    let pred2 = restored.feed_forward(test_input);

    for (p1, p2) in pred1.data.iter().zip(pred2.data.iter()) {
        assert!((p1 - p2).abs() < 1e-10, "Predictions differ: {} vs {}", p1, p2);
    }
}

#[test]
fn test_network_preserves_training_state() {
    let mut network = Network::new(vec![2, 2, 1], SIGMOID, 0.5);

    // Train on AND gate
    let inputs = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ];
    let targets = vec![vec![0.0], vec![0.0], vec![0.0], vec![1.0]];

    network.train(inputs.clone(), targets.clone(), 1000);

    // Get predictions before serialization
    let pred_before = network.feed_forward(Matrix::from(vec![1.0, 1.0]));

    // Serialize and deserialize
    let json = serde_json::to_string(&network).unwrap();
    let mut restored: Network = serde_json::from_str(&json).unwrap();

    // Get predictions after deserialization
    let pred_after = restored.feed_forward(Matrix::from(vec![1.0, 1.0]));

    // Should produce identical predictions
    assert_eq!(pred_before.data, pred_after.data);

    // Should be able to continue training
    restored.train(inputs, targets, 100);
    let pred_final = restored.feed_forward(Matrix::from(vec![1.0, 1.0]));

    // Prediction should still be valid (between 0 and 1)
    assert!(pred_final.data[0] >= 0.0 && pred_final.data[0] <= 1.0);
}

#[test]
fn test_serialize_pretty_json() {
    let network = Network::new(vec![2, 2, 1], SIGMOID, 0.5);
    let json = serde_json::to_string_pretty(&network).expect("Serialization failed");

    // Should be human-readable
    assert!(json.contains('\n'));
    assert!(json.contains("  ")); // indentation
}

#[test]
fn test_deserialize_invalid_json() {
    let invalid_json = r#"{"invalid": "data"}"#;
    let result: Result<Network, _> = serde_json::from_str(invalid_json);

    assert!(result.is_err());
}

#[test]
fn test_matrix_from_json_string() {
    let json = r#"{
        "rows": 2,
        "cols": 3,
        "data": [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
    }"#;

    let matrix: Matrix = serde_json::from_str(json).expect("Should parse valid JSON");
    assert_eq!(matrix.rows, 2);
    assert_eq!(matrix.cols, 3);
    assert_eq!(matrix.data.len(), 6);
}

#[test]
fn test_network_json_structure() {
    let network = Network::new(vec![2, 3, 1], SIGMOID, 0.5);
    let json = serde_json::to_string(&network).unwrap();
    let value: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Verify JSON structure
    assert!(value["layers"].is_array());
    assert!(value["weights"].is_array());
    assert!(value["biases"].is_array());
    assert!(value["learning_rate"].is_number());

    // Verify array lengths
    assert_eq!(value["layers"].as_array().unwrap().len(), 3);
    assert_eq!(value["weights"].as_array().unwrap().len(), 2); // n-1 weight matrices
    assert_eq!(value["biases"].as_array().unwrap().len(), 2);  // n-1 bias vectors
}

// Integration tests for examples module
use neural_network::examples::{get_example, list_examples};

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
    assert_eq!(examples.len(), 8);
    assert!(examples.contains(&"and"));
    assert!(examples.contains(&"or"));
    assert!(examples.contains(&"xor"));
    assert!(examples.contains(&"parity3"));
    assert!(examples.contains(&"quadrant"));
    assert!(examples.contains(&"adder2"));
    assert!(examples.contains(&"iris"));
    assert!(examples.contains(&"pattern3x3"));
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

#[test]
fn test_and_logic() {
    let ex = get_example("and").unwrap();

    // Verify AND truth table
    // [0, 0] -> 0
    assert_eq!(ex.inputs[0], vec![0.0, 0.0]);
    assert_eq!(ex.targets[0], vec![0.0]);

    // [0, 1] -> 0
    assert_eq!(ex.inputs[1], vec![0.0, 1.0]);
    assert_eq!(ex.targets[1], vec![0.0]);

    // [1, 0] -> 0
    assert_eq!(ex.inputs[2], vec![1.0, 0.0]);
    assert_eq!(ex.targets[2], vec![0.0]);

    // [1, 1] -> 1
    assert_eq!(ex.inputs[3], vec![1.0, 1.0]);
    assert_eq!(ex.targets[3], vec![1.0]);
}

#[test]
fn test_or_logic() {
    let ex = get_example("or").unwrap();

    // Verify OR truth table
    // [0, 0] -> 0
    assert_eq!(ex.targets[0], vec![0.0]);

    // [0, 1] -> 1
    assert_eq!(ex.targets[1], vec![1.0]);

    // [1, 0] -> 1
    assert_eq!(ex.targets[2], vec![1.0]);

    // [1, 1] -> 1
    assert_eq!(ex.targets[3], vec![1.0]);
}

#[test]
fn test_xor_logic() {
    let ex = get_example("xor").unwrap();

    // Verify XOR truth table
    // [0, 0] -> 0
    assert_eq!(ex.targets[0], vec![0.0]);

    // [0, 1] -> 1
    assert_eq!(ex.targets[1], vec![1.0]);

    // [1, 0] -> 1
    assert_eq!(ex.targets[2], vec![1.0]);

    // [1, 1] -> 0
    assert_eq!(ex.targets[3], vec![0.0]);
}

#[test]
fn test_example_descriptions() {
    let and = get_example("and").unwrap();
    assert!(!and.description.is_empty());

    let or = get_example("or").unwrap();
    assert!(!or.description.is_empty());

    let xor = get_example("xor").unwrap();
    assert!(!xor.description.is_empty());
}

#[test]
fn test_recommended_epochs() {
    let and = get_example("and").unwrap();
    assert!(and.recommended_epochs > 0);

    let or = get_example("or").unwrap();
    assert!(or.recommended_epochs > 0);

    let xor = get_example("xor").unwrap();
    assert!(xor.recommended_epochs > 0);
    // XOR should require more epochs than AND/OR
    assert!(xor.recommended_epochs >= and.recommended_epochs);
}

#[test]
fn test_recommended_learning_rate() {
    let ex = get_example("xor").unwrap();
    assert!(ex.recommended_lr > 0.0);
    assert!(ex.recommended_lr <= 1.0);
}

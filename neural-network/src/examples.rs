/// Training examples for logic gates
///
/// This module provides pre-configured examples of classic machine learning problems:
/// AND, OR, and XOR logic gates. Each example includes the training data, recommended
/// architecture, and hyperparameters.
/// Represents a training example with inputs, targets, and recommended configuration
#[derive(Debug, Clone)]
pub struct Example {
    /// Name of the example (e.g., "and", "or", "xor")
    pub name: &'static str,

    /// Description of what this example demonstrates
    pub description: &'static str,

    /// Training inputs - each inner vec is one input sample
    pub inputs: Vec<Vec<f64>>,

    /// Training targets - each inner vec is the expected output
    pub targets: Vec<Vec<f64>>,

    /// Recommended network architecture [input_size, hidden_size, output_size]
    pub recommended_arch: Vec<usize>,

    /// Recommended number of training epochs
    pub recommended_epochs: u32,

    /// Recommended learning rate
    pub recommended_lr: f64,
}

/// Get an example by name
///
/// # Arguments
///
/// * `name` - The name of the example ("and", "or", or "xor")
///
/// # Returns
///
/// `Some(Example)` if the example exists, `None` otherwise
///
/// # Examples
///
/// ```
/// use neural_network::examples::get_example;
///
/// let xor = get_example("xor").expect("XOR example should exist");
/// assert_eq!(xor.inputs.len(), 4);
/// ```
pub fn get_example(name: &str) -> Option<Example> {
    match name {
        "and" => Some(Example {
            name: "and",
            description: "Logical AND gate - outputs 1 only when both inputs are 1. This is a linearly separable problem.",
            inputs: vec![
                vec![0.0, 0.0],
                vec![0.0, 1.0],
                vec![1.0, 0.0],
                vec![1.0, 1.0],
            ],
            targets: vec![
                vec![0.0],
                vec![0.0],
                vec![0.0],
                vec![1.0],
            ],
            recommended_arch: vec![2, 2, 1],
            recommended_epochs: 5000,
            recommended_lr: 0.5,
        }),

        "or" => Some(Example {
            name: "or",
            description: "Logical OR gate - outputs 1 when at least one input is 1. This is a linearly separable problem.",
            inputs: vec![
                vec![0.0, 0.0],
                vec![0.0, 1.0],
                vec![1.0, 0.0],
                vec![1.0, 1.0],
            ],
            targets: vec![
                vec![0.0],
                vec![1.0],
                vec![1.0],
                vec![1.0],
            ],
            recommended_arch: vec![2, 2, 1],
            recommended_epochs: 5000,
            recommended_lr: 0.5,
        }),

        "xor" => Some(Example {
            name: "xor",
            description: "Logical XOR gate - outputs 1 when inputs are different. This is NOT linearly separable and requires a hidden layer.",
            inputs: vec![
                vec![0.0, 0.0],
                vec![0.0, 1.0],
                vec![1.0, 0.0],
                vec![1.0, 1.0],
            ],
            targets: vec![
                vec![0.0],
                vec![1.0],
                vec![1.0],
                vec![0.0],
            ],
            recommended_arch: vec![2, 3, 1],
            recommended_epochs: 10000,
            recommended_lr: 0.5,
        }),

        _ => None,
    }
}

/// List all available example names
///
/// # Returns
///
/// A vector of example names that can be passed to `get_example()`
///
/// # Examples
///
/// ```
/// use neural_network::examples::list_examples;
///
/// let examples = list_examples();
/// assert!(examples.contains(&"xor"));
/// ```
pub fn list_examples() -> Vec<&'static str> {
    vec!["and", "or", "xor"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_examples_exist() {
        for name in list_examples() {
            assert!(get_example(name).is_some(), "Example {} should exist", name);
        }
    }

    #[test]
    fn test_all_examples_have_valid_data() {
        for name in list_examples() {
            let ex = get_example(name).unwrap();

            // Must have 4 test cases (all combinations of 2 binary inputs)
            assert_eq!(ex.inputs.len(), 4);
            assert_eq!(ex.targets.len(), 4);

            // All inputs must be 2D
            for input in &ex.inputs {
                assert_eq!(input.len(), 2);
            }

            // All targets must be 1D
            for target in &ex.targets {
                assert_eq!(target.len(), 1);
            }

            // Architecture must have at least 3 layers
            assert!(ex.recommended_arch.len() >= 3);

            // First layer should be 2 (2 inputs)
            assert_eq!(ex.recommended_arch[0], 2);

            // Last layer should be 1 (1 output)
            assert_eq!(ex.recommended_arch[ex.recommended_arch.len() - 1], 1);

            // Reasonable hyperparameters
            assert!(ex.recommended_epochs > 0);
            assert!(ex.recommended_lr > 0.0 && ex.recommended_lr <= 1.0);
        }
    }
}

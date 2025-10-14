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

        "parity3" => Some(Example {
            name: "parity3",
            description: "3-bit parity - outputs 1 when an odd number of inputs are 1. Extension of XOR to 3 inputs.",
            inputs: vec![
                vec![0.0, 0.0, 0.0],
                vec![0.0, 0.0, 1.0],
                vec![0.0, 1.0, 0.0],
                vec![0.0, 1.0, 1.0],
                vec![1.0, 0.0, 0.0],
                vec![1.0, 0.0, 1.0],
                vec![1.0, 1.0, 0.0],
                vec![1.0, 1.0, 1.0],
            ],
            targets: vec![
                vec![0.0], // 0 ones -> even
                vec![1.0], // 1 one -> odd
                vec![1.0], // 1 one -> odd
                vec![0.0], // 2 ones -> even
                vec![1.0], // 1 one -> odd
                vec![0.0], // 2 ones -> even
                vec![0.0], // 2 ones -> even
                vec![1.0], // 3 ones -> odd
            ],
            recommended_arch: vec![3, 4, 1],
            recommended_epochs: 15000,
            recommended_lr: 0.5,
        }),

        "quadrant" => Some(Example {
            name: "quadrant",
            description: "Quadrant classification - classifies 2D points into 4 quadrants. First multi-class output example.",
            inputs: vec![
                // Quadrant I: x > 0, y > 0 -> [1, 0, 0, 0]
                vec![1.0, 1.0],
                vec![0.8, 0.6],
                vec![0.5, 0.9],
                // Quadrant II: x < 0, y > 0 -> [0, 1, 0, 0]
                vec![-1.0, 1.0],
                vec![-0.8, 0.6],
                vec![-0.5, 0.9],
                // Quadrant III: x < 0, y < 0 -> [0, 0, 1, 0]
                vec![-1.0, -1.0],
                vec![-0.8, -0.6],
                vec![-0.5, -0.9],
                // Quadrant IV: x > 0, y < 0 -> [0, 0, 0, 1]
                vec![1.0, -1.0],
                vec![0.8, -0.6],
                vec![0.5, -0.9],
            ],
            targets: vec![
                // Quadrant I
                vec![1.0, 0.0, 0.0, 0.0],
                vec![1.0, 0.0, 0.0, 0.0],
                vec![1.0, 0.0, 0.0, 0.0],
                // Quadrant II
                vec![0.0, 1.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0, 0.0],
                // Quadrant III
                vec![0.0, 0.0, 1.0, 0.0],
                vec![0.0, 0.0, 1.0, 0.0],
                vec![0.0, 0.0, 1.0, 0.0],
                // Quadrant IV
                vec![0.0, 0.0, 0.0, 1.0],
                vec![0.0, 0.0, 0.0, 1.0],
                vec![0.0, 0.0, 0.0, 1.0],
            ],
            recommended_arch: vec![2, 4, 4],
            recommended_epochs: 10000,
            recommended_lr: 0.5,
        }),

        "adder2" => Some(Example {
            name: "adder2",
            description: "2-bit binary adder - adds two 2-bit numbers. Demonstrates arithmetic learning with multi-bit outputs.",
            inputs: vec![
                // Format: [A1, A0, B1, B0] where A = A1*2 + A0, B = B1*2 + B0
                vec![0.0, 0.0, 0.0, 0.0], // 0 + 0 = 0
                vec![0.0, 0.0, 0.0, 1.0], // 0 + 1 = 1
                vec![0.0, 0.0, 1.0, 0.0], // 0 + 2 = 2
                vec![0.0, 0.0, 1.0, 1.0], // 0 + 3 = 3
                vec![0.0, 1.0, 0.0, 0.0], // 1 + 0 = 1
                vec![0.0, 1.0, 0.0, 1.0], // 1 + 1 = 2
                vec![0.0, 1.0, 1.0, 0.0], // 1 + 2 = 3
                vec![0.0, 1.0, 1.0, 1.0], // 1 + 3 = 4
                vec![1.0, 0.0, 0.0, 0.0], // 2 + 0 = 2
                vec![1.0, 0.0, 0.0, 1.0], // 2 + 1 = 3
                vec![1.0, 0.0, 1.0, 0.0], // 2 + 2 = 4
                vec![1.0, 0.0, 1.0, 1.0], // 2 + 3 = 5
                vec![1.0, 1.0, 0.0, 0.0], // 3 + 0 = 3
                vec![1.0, 1.0, 0.0, 1.0], // 3 + 1 = 4
                vec![1.0, 1.0, 1.0, 0.0], // 3 + 2 = 5
                vec![1.0, 1.0, 1.0, 1.0], // 3 + 3 = 6
            ],
            targets: vec![
                // Output: [S2, S1, S0] where sum = S2*4 + S1*2 + S0
                vec![0.0, 0.0, 0.0], // 000 = 0
                vec![0.0, 0.0, 1.0], // 001 = 1
                vec![0.0, 1.0, 0.0], // 010 = 2
                vec![0.0, 1.0, 1.0], // 011 = 3
                vec![0.0, 0.0, 1.0], // 001 = 1
                vec![0.0, 1.0, 0.0], // 010 = 2
                vec![0.0, 1.0, 1.0], // 011 = 3
                vec![1.0, 0.0, 0.0], // 100 = 4
                vec![0.0, 1.0, 0.0], // 010 = 2
                vec![0.0, 1.0, 1.0], // 011 = 3
                vec![1.0, 0.0, 0.0], // 100 = 4
                vec![1.0, 0.0, 1.0], // 101 = 5
                vec![0.0, 1.0, 1.0], // 011 = 3
                vec![1.0, 0.0, 0.0], // 100 = 4
                vec![1.0, 0.0, 1.0], // 101 = 5
                vec![1.0, 1.0, 0.0], // 110 = 6
            ],
            recommended_arch: vec![4, 8, 3],
            recommended_epochs: 20000,
            recommended_lr: 0.5,
        }),

        "iris" => Some(Example {
            name: "iris",
            description: "Iris flower classification - classic ML dataset with real botanical measurements. Classifies iris species based on sepal and petal dimensions.",
            inputs: vec![
                // Format: [sepal_length, sepal_width, petal_length, petal_width] in cm
                // Iris Setosa (20 samples)
                vec![5.1, 3.5, 1.4, 0.2], vec![4.9, 3.0, 1.4, 0.2], vec![4.7, 3.2, 1.3, 0.2],
                vec![4.6, 3.1, 1.5, 0.2], vec![5.0, 3.6, 1.4, 0.2], vec![5.4, 3.9, 1.7, 0.4],
                vec![4.6, 3.4, 1.4, 0.3], vec![5.0, 3.4, 1.5, 0.2], vec![4.4, 2.9, 1.4, 0.2],
                vec![4.9, 3.1, 1.5, 0.1], vec![5.4, 3.7, 1.5, 0.2], vec![4.8, 3.4, 1.6, 0.2],
                vec![4.8, 3.0, 1.4, 0.1], vec![4.3, 3.0, 1.1, 0.1], vec![5.8, 4.0, 1.2, 0.2],
                vec![5.7, 4.4, 1.5, 0.4], vec![5.4, 3.9, 1.3, 0.4], vec![5.1, 3.5, 1.4, 0.3],
                vec![5.7, 3.8, 1.7, 0.3], vec![5.1, 3.8, 1.5, 0.3],
                // Iris Versicolor (20 samples)
                vec![7.0, 3.2, 4.7, 1.4], vec![6.4, 3.2, 4.5, 1.5], vec![6.9, 3.1, 4.9, 1.5],
                vec![5.5, 2.3, 4.0, 1.3], vec![6.5, 2.8, 4.6, 1.5], vec![5.7, 2.8, 4.5, 1.3],
                vec![6.3, 3.3, 4.7, 1.6], vec![4.9, 2.4, 3.3, 1.0], vec![6.6, 2.9, 4.6, 1.3],
                vec![5.2, 2.7, 3.9, 1.4], vec![5.0, 2.0, 3.5, 1.0], vec![5.9, 3.0, 4.2, 1.5],
                vec![6.0, 2.2, 4.0, 1.0], vec![6.1, 2.9, 4.7, 1.4], vec![5.6, 2.9, 3.6, 1.3],
                vec![6.7, 3.1, 4.4, 1.4], vec![5.6, 3.0, 4.5, 1.5], vec![5.8, 2.7, 4.1, 1.0],
                vec![6.2, 2.2, 4.5, 1.5], vec![5.6, 2.5, 3.9, 1.1],
                // Iris Virginica (20 samples)
                vec![6.3, 3.3, 6.0, 2.5], vec![5.8, 2.7, 5.1, 1.9], vec![7.1, 3.0, 5.9, 2.1],
                vec![6.3, 2.9, 5.6, 1.8], vec![6.5, 3.0, 5.8, 2.2], vec![7.6, 3.0, 6.6, 2.1],
                vec![4.9, 2.5, 4.5, 1.7], vec![7.3, 2.9, 6.3, 1.8], vec![6.7, 2.5, 5.8, 1.8],
                vec![7.2, 3.6, 6.1, 2.5], vec![6.5, 3.2, 5.1, 2.0], vec![6.4, 2.7, 5.3, 1.9],
                vec![6.8, 3.0, 5.5, 2.1], vec![5.7, 2.5, 5.0, 2.0], vec![5.8, 2.8, 5.1, 2.4],
                vec![6.4, 3.2, 5.3, 2.3], vec![6.5, 3.0, 5.5, 1.8], vec![7.7, 3.8, 6.7, 2.2],
                vec![7.7, 2.6, 6.9, 2.3], vec![6.0, 2.2, 5.0, 1.5],
            ],
            targets: vec![
                // One-hot encoding: [Setosa, Versicolor, Virginica]
                // Setosa (20 samples)
                vec![1.0, 0.0, 0.0], vec![1.0, 0.0, 0.0], vec![1.0, 0.0, 0.0],
                vec![1.0, 0.0, 0.0], vec![1.0, 0.0, 0.0], vec![1.0, 0.0, 0.0],
                vec![1.0, 0.0, 0.0], vec![1.0, 0.0, 0.0], vec![1.0, 0.0, 0.0],
                vec![1.0, 0.0, 0.0], vec![1.0, 0.0, 0.0], vec![1.0, 0.0, 0.0],
                vec![1.0, 0.0, 0.0], vec![1.0, 0.0, 0.0], vec![1.0, 0.0, 0.0],
                vec![1.0, 0.0, 0.0], vec![1.0, 0.0, 0.0], vec![1.0, 0.0, 0.0],
                vec![1.0, 0.0, 0.0], vec![1.0, 0.0, 0.0],
                // Versicolor (20 samples)
                vec![0.0, 1.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 1.0, 0.0],
                vec![0.0, 1.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 1.0, 0.0],
                vec![0.0, 1.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 1.0, 0.0],
                vec![0.0, 1.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 1.0, 0.0],
                vec![0.0, 1.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 1.0, 0.0],
                vec![0.0, 1.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 1.0, 0.0],
                vec![0.0, 1.0, 0.0], vec![0.0, 1.0, 0.0],
                // Virginica (20 samples)
                vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 1.0],
                vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 1.0],
                vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 1.0],
                vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 1.0],
                vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 1.0],
                vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 1.0],
                vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 1.0],
            ],
            recommended_arch: vec![4, 8, 3],
            recommended_epochs: 10000,
            recommended_lr: 0.3,
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
    vec!["and", "or", "xor", "parity3", "quadrant", "adder2", "iris"]
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

            // Must have at least one test case
            assert!(!ex.inputs.is_empty(), "Example {} has no inputs", name);
            assert_eq!(ex.inputs.len(), ex.targets.len(),
                "Example {} has mismatched inputs/targets", name);

            // All inputs must have consistent dimensions
            let input_size = ex.inputs[0].len();
            for input in &ex.inputs {
                assert_eq!(input.len(), input_size,
                    "Example {} has inconsistent input dimensions", name);
            }

            // All targets must have consistent dimensions
            let output_size = ex.targets[0].len();
            for target in &ex.targets {
                assert_eq!(target.len(), output_size,
                    "Example {} has inconsistent target dimensions", name);
            }

            // Architecture must have at least 3 layers
            assert!(ex.recommended_arch.len() >= 3,
                "Example {} needs at least 3 layers", name);

            // First layer should match input size
            assert_eq!(ex.recommended_arch[0], input_size,
                "Example {} first layer should be {}", name, input_size);

            // Last layer should match output size
            assert_eq!(ex.recommended_arch[ex.recommended_arch.len() - 1], output_size,
                "Example {} last layer should be {}", name, output_size);

            // Reasonable hyperparameters
            assert!(ex.recommended_epochs > 0);
            assert!(ex.recommended_lr > 0.0 && ex.recommended_lr <= 1.0);
        }
    }
}

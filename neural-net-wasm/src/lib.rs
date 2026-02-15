// Neural Network WASM Bindings
// Provides JavaScript-friendly API for neural network training and evaluation

use wasm_bindgen::prelude::*;
use neural_network::{
    activations::SIGMOID,
    examples,
    network::Network,
    training::{TrainingConfig, TrainingController},
};
use serde::{Deserialize, Serialize};

/// Initialize for WASM execution
#[wasm_bindgen(start)]
pub fn init() {
    // Initialization logic can be added here if needed
}

/// Example information for JavaScript
#[derive(Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct ExampleInfo {
    pub name: String,
    pub description: String,
    pub architecture: Vec<usize>,
}

/// Full example data including training inputs and targets
#[derive(Serialize, Deserialize)]
pub struct ExampleData {
    pub name: String,
    pub description: String,
    pub architecture: Vec<usize>,
    pub inputs: Vec<Vec<f64>>,
    pub targets: Vec<Vec<f64>>,
}

/// Training progress update
#[derive(Serialize, Deserialize, Clone)]
pub struct TrainingProgress {
    pub epoch: u32,
    pub loss: f64,
}

/// WASM-friendly neural network wrapper
#[wasm_bindgen]
pub struct NeuralNetwork {
    network: Network,
    example_name: Option<String>,
}

#[wasm_bindgen]
impl NeuralNetwork {
    /// Create a new neural network with specified architecture
    /// If seed is provided, uses seeded random initialization for reproducibility
    #[wasm_bindgen(constructor)]
    pub fn new(layers: Vec<usize>, learning_rate: f64, seed: Option<u64>) -> Result<NeuralNetwork, JsValue> {
        let network = match seed {
            Some(s) => Network::new_seeded(layers, SIGMOID, learning_rate, s),
            None => Network::new(layers, SIGMOID, learning_rate),
        };
        Ok(NeuralNetwork {
            network,
            example_name: None,
        })
    }

    /// Create a network from a built-in example
    /// If seed is provided, uses seeded random initialization for reproducibility
    #[wasm_bindgen(js_name = fromExample)]
    pub fn from_example(example_name: &str, learning_rate: f64, seed: Option<u64>) -> Result<NeuralNetwork, JsValue> {
        let example = examples::get_example(example_name)
            .ok_or_else(|| JsValue::from_str(&format!("Unknown example: {}", example_name)))?;

        let network = match seed {
            Some(s) => Network::new_seeded(example.recommended_arch, SIGMOID, learning_rate, s),
            None => Network::new(example.recommended_arch, SIGMOID, learning_rate),
        };
        Ok(NeuralNetwork {
            network,
            example_name: Some(example_name.to_string()),
        })
    }

    /// Train the network on a built-in example
    /// Accepts an optional JavaScript callback for progress updates
    pub fn train(&mut self, example_name: &str, epochs: u32, progress_callback: Option<js_sys::Function>) -> Result<(), JsValue> {
        let example = examples::get_example(example_name)
            .ok_or_else(|| JsValue::from_str(&format!("Unknown example: {}", example_name)))?;

        // Store example name
        self.example_name = Some(example_name.to_string());

        // Create training config
        let config = TrainingConfig {
            epochs,
            checkpoint_interval: None,
            checkpoint_path: None,
            verbose: false,
            example_name: Some(example_name.to_string()),
        };

        let mut controller = TrainingController::new(self.network.clone(), config);

        // Add callback to call JavaScript progress function
        if let Some(callback) = progress_callback {
            controller.add_callback(Box::new(move |epoch, loss, _network| {
                let this = JsValue::null();
                let epoch_js = JsValue::from_f64(epoch as f64);
                let loss_js = JsValue::from_f64(loss);

                // Call the JavaScript callback with (epoch, loss)
                let _ = callback.call2(&this, &epoch_js, &loss_js);
            }));
        }

        // Train the network
        controller
            .train(example.inputs.clone(), example.targets.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Update internal network
        self.network = controller.into_network();

        Ok(())
    }

    /// Train with custom inputs and targets
    #[wasm_bindgen(js_name = trainCustom)]
    pub fn train_custom(
        &mut self,
        inputs_flat: Vec<f64>,
        targets_flat: Vec<f64>,
        input_size: usize,
        target_size: usize,
        epochs: u32,
    ) -> Result<(), JsValue> {
        // Convert flat arrays to matrices
        let num_samples = inputs_flat.len() / input_size;
        let mut inputs = Vec::new();
        let mut targets = Vec::new();

        for i in 0..num_samples {
            let input_start = i * input_size;
            let input_end = input_start + input_size;
            inputs.push(inputs_flat[input_start..input_end].to_vec());

            let target_start = i * target_size;
            let target_end = target_start + target_size;
            targets.push(targets_flat[target_start..target_end].to_vec());
        }

        let config = TrainingConfig {
            epochs,
            checkpoint_interval: None,
            checkpoint_path: None,
            verbose: false,
            example_name: None,
        };

        let mut controller = TrainingController::new(self.network.clone(), config);

        controller
            .train(inputs, targets)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        self.network = controller.into_network();

        Ok(())
    }

    /// Evaluate the network on a single input
    pub fn evaluate(&mut self, input: Vec<f64>) -> Result<Vec<f64>, JsValue> {
        // Validate input dimensions
        if input.len() != self.network.layers[0] {
            return Err(JsValue::from_str(&format!(
                "Invalid input dimensions: expected {}, got {}",
                self.network.layers[0],
                input.len()
            )));
        }

        let input_matrix = neural_network::matrix::Matrix::from(input);
        let output = self.network.feed_forward(input_matrix);

        Ok(output.data)
    }

    /// Get the network architecture
    pub fn get_architecture(&self) -> Vec<usize> {
        self.network.layers.clone()
    }

    /// Get the total number of parameters (weights + biases)
    #[wasm_bindgen(js_name = getParameterCount)]
    pub fn get_parameter_count(&self) -> usize {
        let mut total = 0;
        for weight in &self.network.weights {
            total += weight.rows * weight.cols;
        }
        for bias in &self.network.biases {
            total += bias.rows;
        }
        total
    }

    /// Get all layer activations from the last evaluate() call
    /// Returns array of arrays, one per layer (including input)
    #[wasm_bindgen(js_name = getActivations)]
    pub fn get_activations(&self) -> Result<JsValue, JsValue> {
        let activations = self.network.get_activations();
        serde_wasm_bindgen::to_value(&activations)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Get weight matrices as flat arrays
    #[wasm_bindgen(js_name = getWeights)]
    pub fn get_weights(&self) -> Result<JsValue, JsValue> {
        let weights = self.network.get_weights();
        serde_wasm_bindgen::to_value(&weights)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Get weight matrix shapes [(rows, cols), ...]
    #[wasm_bindgen(js_name = getWeightShapes)]
    pub fn get_weight_shapes(&self) -> Result<JsValue, JsValue> {
        let shapes = self.network.get_weight_shapes();
        serde_wasm_bindgen::to_value(&shapes)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Serialize the network to JSON string
    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string(&self.network)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Deserialize a network from JSON string
    #[wasm_bindgen(js_name = fromJSON)]
    pub fn from_json(json: &str) -> Result<NeuralNetwork, JsValue> {
        let network: Network = serde_json::from_str(json)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(NeuralNetwork {
            network,
            example_name: None,
        })
    }
}

/// List all available examples
#[wasm_bindgen(js_name = listExamples)]
pub fn list_examples() -> Result<JsValue, JsValue> {
    let example_names = examples::list_examples();
    let examples_info: Vec<ExampleInfo> = example_names
        .into_iter()
        .filter_map(|name| {
            examples::get_example(name).map(|ex| ExampleInfo {
                name: ex.name.to_string(),
                description: ex.description.to_string(),
                architecture: ex.recommended_arch.clone(),
            })
        })
        .collect();

    serde_wasm_bindgen::to_value(&examples_info)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Get details about a specific example
#[wasm_bindgen(js_name = getExampleInfo)]
pub fn get_example_info(name: &str) -> Result<JsValue, JsValue> {
    let example = examples::get_example(name)
        .ok_or_else(|| JsValue::from_str(&format!("Unknown example: {}", name)))?;

    let info = ExampleInfo {
        name: example.name.to_string(),
        description: example.description.to_string(),
        architecture: example.recommended_arch.clone(),
    };

    serde_wasm_bindgen::to_value(&info)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Get full example data including training inputs and targets
#[wasm_bindgen(js_name = getExampleData)]
pub fn get_example_data(name: &str) -> Result<JsValue, JsValue> {
    let example = examples::get_example(name)
        .ok_or_else(|| JsValue::from_str(&format!("Unknown example: {}", name)))?;

    let data = ExampleData {
        name: example.name.to_string(),
        description: example.description.to_string(),
        architecture: example.recommended_arch.clone(),
        inputs: example.inputs.clone(),
        targets: example.targets.clone(),
    };

    serde_wasm_bindgen::to_value(&data)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_network() {
        let network = NeuralNetwork::new(vec![2, 3, 1], 0.5, None);
        assert!(network.is_ok());
        let net = network.unwrap();
        assert_eq!(net.get_architecture(), vec![2, 3, 1]);
    }

    #[test]
    fn test_create_network_seeded() {
        // Test that seeded networks are reproducible
        let network1 = NeuralNetwork::new(vec![2, 3, 1], 0.5, Some(12345));
        let network2 = NeuralNetwork::new(vec![2, 3, 1], 0.5, Some(12345));
        assert!(network1.is_ok());
        assert!(network2.is_ok());
        let json1 = network1.unwrap().to_json().unwrap();
        let json2 = network2.unwrap().to_json().unwrap();
        assert_eq!(json1, json2);
    }

    #[test]
    fn test_from_example() {
        let network = NeuralNetwork::from_example("xor", 0.5, None);
        assert!(network.is_ok());
        let net = network.unwrap();
        assert_eq!(net.example_name, Some("xor".to_string()));
    }

    #[test]
    fn test_evaluate() {
        let mut network = NeuralNetwork::new(vec![2, 3, 1], 0.5, None).unwrap();
        let result = network.evaluate(vec![1.0, 0.0]);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.len(), 1);
    }

    #[test]
    fn test_parameter_count() {
        let network = NeuralNetwork::new(vec![2, 3, 1], 0.5, None).unwrap();
        // Weights: 2->3 = 6, 3->1 = 3
        // Biases: 3 + 1 = 4
        // Total = 6 + 3 + 4 = 13
        assert_eq!(network.get_parameter_count(), 13);
    }

    #[test]
    fn test_serialization() {
        let network = NeuralNetwork::new(vec![2, 3, 1], 0.5, None).unwrap();
        let json = network.to_json();
        assert!(json.is_ok());

        let json_str = json.unwrap();
        let restored = NeuralNetwork::from_json(&json_str);
        assert!(restored.is_ok());
        assert_eq!(restored.unwrap().get_architecture(), vec![2, 3, 1]);
    }
}

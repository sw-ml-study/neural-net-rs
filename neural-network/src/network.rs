use matrix::matrix::Matrix;
use rand::SeedableRng;
use rand::rngs::StdRng;
use serde::{Serialize, Deserialize};

use crate::activations::Activation;


#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct Network {
    pub layers: Vec<usize>, // amount of neurons in each layer, [72,16,10]
    pub weights: Vec<Matrix>,
    pub biases: Vec<Matrix>,
    #[serde(skip)]
    data: Vec<Matrix>,
    pub activation: Activation,
    pub learning_rate: f64,
}

impl Network {

    pub fn new(layers: Vec<usize>,activation:Activation,learning_rate:f64 ) -> Self {

        let mut weights = vec![];

        let mut biases = vec![];

        for i in 0..layers.len() - 1 {
            weights.push(Matrix::random(layers[i+1], layers[i]));
            biases.push(Matrix::random(layers[i+1], 1));
        }


        Network {
            layers,
            weights,
            biases,
            data: vec![],
            activation,
            learning_rate
        }


    }

    /// Create a new network with a specific seed for reproducible initialization
    pub fn new_seeded(layers: Vec<usize>, activation: Activation, learning_rate: f64, seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut weights = vec![];
        let mut biases = vec![];

        for i in 0..layers.len() - 1 {
            weights.push(Matrix::random_seeded(layers[i+1], layers[i], &mut rng));
            biases.push(Matrix::random_seeded(layers[i+1], 1, &mut rng));
        }

        Network {
            layers,
            weights,
            biases,
            data: vec![],
            activation,
            learning_rate,
        }
    }

    pub fn feed_forward(&mut self, inputs: Matrix) -> Matrix {

        assert!(self.layers[0] == inputs.data.len(), "Invalid Number of Inputs");

        let mut current = inputs;

        self.data = vec![current.clone()];


      for i in 0..self.layers.len() -1 {
            current = self.weights[i]
            .dot_multiply(&current)
            .add(&self.biases[i]).map(self.activation.function);
            
            self.data.push(current.clone());
      }


       current

    }

    /// Get all layer activations from the last feed_forward call
    /// Returns a vector of vectors, one per layer (including input layer)
    pub fn get_activations(&self) -> Vec<Vec<f64>> {
        self.data.iter().map(|m| m.data.clone()).collect()
    }

    /// Get the weight matrices
    pub fn get_weights(&self) -> Vec<Vec<f64>> {
        self.weights.iter().map(|m| m.data.clone()).collect()
    }

    /// Get weight matrix dimensions (rows, cols) for each layer
    pub fn get_weight_shapes(&self) -> Vec<(usize, usize)> {
        self.weights.iter().map(|m| (m.rows, m.cols)).collect()
    }

    pub fn back_propogate(&mut self, inputs:Matrix, targets:Matrix) {

        let mut errors = targets.subtract(&inputs);

        let mut gradients = inputs.clone().map(self.activation.derivative);


      

        for i in (0..self.layers.len() -1).rev(){
           
            gradients = gradients.elementwise_multiply(&errors).map(|x| x * self.learning_rate);
           
           
            
           
            self.weights[i] = self.weights[i].add(&gradients.dot_multiply(&self.data[i].transpose()));

            
            
        
            self.biases[i] = self.biases[i].add(&gradients);

            errors = self.weights[i].transpose().dot_multiply(&errors);
            gradients = self.data[i].map(self.activation.derivative);

        }      
    }

    pub fn train(&mut self, inputs: Vec<Vec<f64>>, targets: Vec<Vec<f64>>, epochs: u32) {
		for i in 1..=epochs {
			if epochs < 100 || i % (epochs / 100) == 0 {
				println!("Epoch {} of {}", i, epochs);
			}
			for j in 0..inputs.len() {
				let outputs = self.feed_forward(Matrix::from(inputs[j].clone()));
				self.back_propogate(outputs,Matrix::from( targets[j].clone()));
			}
		}
	}




}
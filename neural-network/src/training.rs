/// Training controller for managing neural network training with callbacks and checkpointing
use crate::checkpoint::CheckpointMetadata;
use crate::network::Network;
use crate::matrix::Matrix;
use std::path::PathBuf;

/// Configuration for training a neural network
pub struct TrainingConfig {
    pub epochs: u32,
    pub checkpoint_interval: Option<u32>,
    pub checkpoint_path: Option<PathBuf>,
    pub verbose: bool,
}

/// Callback function type for training progress
pub type TrainingCallback = Box<dyn FnMut(u32, f64, &Network)>;

/// Controller for training neural networks with advanced features
pub struct TrainingController {
    network: Network,
    config: TrainingConfig,
    callbacks: Vec<TrainingCallback>,
}

impl TrainingController {
    /// Create a new training controller
    pub fn new(network: Network, config: TrainingConfig) -> Self {
        Self {
            network,
            config,
            callbacks: Vec::new(),
        }
    }

    /// Add a callback function to be called after each epoch
    pub fn add_callback(&mut self, callback: TrainingCallback) {
        self.callbacks.push(callback);
    }

    /// Calculate mean squared error loss
    fn calculate_loss(&mut self, inputs: &[Vec<f64>], targets: &[Vec<f64>]) -> f64 {
        let mut total_loss = 0.0;
        for i in 0..inputs.len() {
            let output = self.network.feed_forward(Matrix::from(inputs[i].clone()));
            let target = Matrix::from(targets[i].clone());

            // Calculate MSE
            for j in 0..output.data.len() {
                let error = target.data[j] - output.data[j];
                total_loss += error * error;
            }
        }
        total_loss / (inputs.len() as f64)
    }

    /// Train the network with the configured settings
    pub fn train(
        &mut self,
        inputs: Vec<Vec<f64>>,
        targets: Vec<Vec<f64>>,
    ) -> anyhow::Result<()> {
        for epoch in 1..=self.config.epochs {
            // Train one epoch
            for j in 0..inputs.len() {
                let outputs = self.network.feed_forward(Matrix::from(inputs[j].clone()));
                self.network.back_propogate(outputs, Matrix::from(targets[j].clone()));
            }

            // Calculate loss for callbacks
            let loss = self.calculate_loss(&inputs, &targets);

            // Verbose output
            if self.config.verbose
                && (self.config.epochs < 100 || epoch % (self.config.epochs / 100) == 0) {
                    println!("Epoch {} of {}: loss = {:.6}", epoch, self.config.epochs, loss);
                }

            // Call callbacks
            for callback in &mut self.callbacks {
                callback(epoch, loss, &self.network);
            }

            // Save checkpoint if needed
            if let (Some(interval), Some(path)) = (self.config.checkpoint_interval, &self.config.checkpoint_path)
                && epoch % interval == 0 {
                    let metadata = CheckpointMetadata {
                        version: "1.0".to_string(),
                        example: "training".to_string(),
                        epoch,
                        total_epochs: self.config.epochs,
                        learning_rate: self.network.learning_rate,
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    };
                    self.network.save_checkpoint(path, metadata)?;
                }
        }

        Ok(())
    }

    /// Get a reference to the trained network
    pub fn network(&self) -> &Network {
        &self.network
    }

    /// Create a training controller from a checkpoint file
    pub fn from_checkpoint(
        checkpoint_path: &std::path::Path,
        config: TrainingConfig,
    ) -> anyhow::Result<Self> {
        let (network, _metadata) = Network::load_checkpoint(checkpoint_path)?;
        Ok(Self {
            network,
            config,
            callbacks: Vec::new(),
        })
    }

    /// Consume the controller and return the network
    pub fn into_network(self) -> Network {
        self.network
    }
}

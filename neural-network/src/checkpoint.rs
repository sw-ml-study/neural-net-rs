/// Checkpoint functionality for saving and resuming training
///
/// This module provides structures and methods for checkpointing neural network
/// training sessions. Checkpoints include both the network state (weights, biases)
/// and metadata about the training session (epoch, timestamp, etc.).
use crate::network::Network;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Supported checkpoint format version
const CHECKPOINT_VERSION: &str = "1.0";

/// Metadata about a training checkpoint
///
/// Contains information about when and where the checkpoint was created,
/// including training progress and hyperparameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointMetadata {
    /// Checkpoint format version (for future compatibility)
    pub version: String,

    /// Name of the example/problem being trained
    pub example: String,

    /// Current epoch number (how far training has progressed)
    pub epoch: u32,

    /// Total planned epochs
    pub total_epochs: u32,

    /// Learning rate used during training
    pub learning_rate: f64,

    /// ISO 8601 timestamp of when checkpoint was created
    pub timestamp: String,
}

/// Complete checkpoint containing network state and metadata
///
/// This structure can be serialized to JSON and saved to disk, then
/// loaded later to resume training from the same point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    /// Metadata about the training session
    pub metadata: CheckpointMetadata,

    /// The neural network state (weights, biases, architecture)
    pub network: Network,
}

impl Network {
    /// Create a checkpoint from the current network state
    ///
    /// # Arguments
    ///
    /// * `metadata` - Metadata describing the current training state
    ///
    /// # Returns
    ///
    /// A `Checkpoint` containing the network and metadata
    ///
    /// # Examples
    ///
    /// ```
    /// use neural_network::network::Network;
    /// use neural_network::activations::SIGMOID;
    /// use neural_network::checkpoint::CheckpointMetadata;
    ///
    /// let network = Network::new(vec![2, 3, 1], SIGMOID, 0.5);
    /// let metadata = CheckpointMetadata {
    ///     version: "1.0".to_string(),
    ///     example: "xor".to_string(),
    ///     epoch: 100,
    ///     total_epochs: 1000,
    ///     learning_rate: 0.5,
    ///     timestamp: chrono::Utc::now().to_rfc3339(),
    /// };
    ///
    /// let checkpoint = network.to_checkpoint(metadata);
    /// ```
    pub fn to_checkpoint(&self, metadata: CheckpointMetadata) -> Checkpoint {
        Checkpoint {
            metadata,
            network: self.clone(),
        }
    }

    /// Restore a network from a checkpoint
    ///
    /// # Arguments
    ///
    /// * `checkpoint` - The checkpoint to restore from
    ///
    /// # Returns
    ///
    /// A `Network` restored from the checkpoint, or an error if the checkpoint
    /// format version is unsupported
    ///
    /// # Errors
    ///
    /// Returns an error if the checkpoint version is not supported
    ///
    /// # Examples
    ///
    /// ```
    /// use neural_network::network::Network;
    /// use neural_network::activations::SIGMOID;
    /// use neural_network::checkpoint::{Checkpoint, CheckpointMetadata};
    ///
    /// let network = Network::new(vec![2, 3, 1], SIGMOID, 0.5);
    /// let metadata = CheckpointMetadata {
    ///     version: "1.0".to_string(),
    ///     example: "xor".to_string(),
    ///     epoch: 100,
    ///     total_epochs: 1000,
    ///     learning_rate: 0.5,
    ///     timestamp: chrono::Utc::now().to_rfc3339(),
    /// };
    ///
    /// let checkpoint = network.to_checkpoint(metadata);
    /// let restored = Network::from_checkpoint(checkpoint).expect("Should restore");
    /// ```
    pub fn from_checkpoint(checkpoint: Checkpoint) -> Result<Self> {
        // Validate checkpoint version
        if checkpoint.metadata.version != CHECKPOINT_VERSION {
            anyhow::bail!(
                "Unsupported checkpoint version: {}. Expected: {}",
                checkpoint.metadata.version,
                CHECKPOINT_VERSION
            );
        }

        Ok(checkpoint.network)
    }

    /// Save a checkpoint to a file
    ///
    /// Serializes the network and metadata to JSON format and writes to the
    /// specified path. The file is created if it doesn't exist, or overwritten
    /// if it does.
    ///
    /// # Arguments
    ///
    /// * `path` - File path where the checkpoint should be saved
    /// * `metadata` - Metadata describing the current training state
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful, or an error if file operations fail
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be created or written
    /// - Serialization fails
    /// - The directory doesn't exist
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use neural_network::network::Network;
    /// use neural_network::activations::SIGMOID;
    /// use neural_network::checkpoint::CheckpointMetadata;
    /// use std::path::Path;
    ///
    /// let network = Network::new(vec![2, 3, 1], SIGMOID, 0.5);
    /// let metadata = CheckpointMetadata {
    ///     version: "1.0".to_string(),
    ///     example: "xor".to_string(),
    ///     epoch: 100,
    ///     total_epochs: 1000,
    ///     learning_rate: 0.5,
    ///     timestamp: chrono::Utc::now().to_rfc3339(),
    /// };
    ///
    /// network.save_checkpoint(Path::new("checkpoint.json"), metadata)
    ///     .expect("Failed to save checkpoint");
    /// ```
    pub fn save_checkpoint(&self, path: &Path, metadata: CheckpointMetadata) -> Result<()> {
        let checkpoint = self.to_checkpoint(metadata);

        let json = serde_json::to_string_pretty(&checkpoint)
            .context("Failed to serialize checkpoint")?;

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory {}", parent.display()))?;
        }

        fs::write(path, json)
            .with_context(|| format!("Failed to write checkpoint to {}", path.display()))?;

        Ok(())
    }

    /// Load a checkpoint from a file
    ///
    /// Reads and deserializes a checkpoint from the specified JSON file, then
    /// restores the network state.
    ///
    /// # Arguments
    ///
    /// * `path` - File path to load the checkpoint from
    ///
    /// # Returns
    ///
    /// A tuple of `(Network, CheckpointMetadata)` if successful, or an error
    /// if loading fails
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file doesn't exist or can't be read
    /// - The file contains invalid JSON
    /// - The checkpoint version is unsupported
    /// - Deserialization fails
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use neural_network::network::Network;
    /// use std::path::Path;
    ///
    /// let (network, metadata) = Network::load_checkpoint(Path::new("checkpoint.json"))
    ///     .expect("Failed to load checkpoint");
    ///
    /// println!("Resumed from epoch {}", metadata.epoch);
    /// ```
    pub fn load_checkpoint(path: &Path) -> Result<(Self, CheckpointMetadata)> {
        let contents = fs::read_to_string(path)
            .with_context(|| format!("Failed to read checkpoint from {}", path.display()))?;

        let checkpoint: Checkpoint = serde_json::from_str(&contents)
            .context("Failed to deserialize checkpoint")?;

        let metadata = checkpoint.metadata.clone();
        let network = Self::from_checkpoint(checkpoint)?;

        Ok((network, metadata))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::activations::SIGMOID;

    #[test]
    fn test_checkpoint_version_constant() {
        assert_eq!(CHECKPOINT_VERSION, "1.0");
    }

    #[test]
    fn test_checkpoint_metadata_creation() {
        let metadata = CheckpointMetadata {
            version: "1.0".to_string(),
            example: "test".to_string(),
            epoch: 50,
            total_epochs: 100,
            learning_rate: 0.5,
            timestamp: "2025-10-13T12:00:00Z".to_string(),
        };

        assert_eq!(metadata.version, "1.0");
        assert_eq!(metadata.epoch, 50);
    }

    #[test]
    fn test_to_checkpoint() {
        let network = Network::new(vec![2, 3, 1], SIGMOID, 0.5);
        let metadata = CheckpointMetadata {
            version: "1.0".to_string(),
            example: "xor".to_string(),
            epoch: 100,
            total_epochs: 1000,
            learning_rate: 0.5,
            timestamp: "2025-10-13T12:00:00Z".to_string(),
        };

        let checkpoint = network.to_checkpoint(metadata);

        assert_eq!(checkpoint.metadata.epoch, 100);
        assert_eq!(checkpoint.network.layers, vec![2, 3, 1]);
    }

    #[test]
    fn test_from_checkpoint_valid_version() {
        let network = Network::new(vec![2, 3, 1], SIGMOID, 0.5);
        let metadata = CheckpointMetadata {
            version: "1.0".to_string(),
            example: "xor".to_string(),
            epoch: 100,
            total_epochs: 1000,
            learning_rate: 0.5,
            timestamp: "2025-10-13T12:00:00Z".to_string(),
        };

        let checkpoint = network.to_checkpoint(metadata);
        let restored = Network::from_checkpoint(checkpoint).expect("Should succeed");

        assert_eq!(restored.layers, vec![2, 3, 1]);
    }

    #[test]
    fn test_from_checkpoint_invalid_version() {
        let network = Network::new(vec![2, 3, 1], SIGMOID, 0.5);
        let metadata = CheckpointMetadata {
            version: "999.0".to_string(),
            example: "xor".to_string(),
            epoch: 100,
            total_epochs: 1000,
            learning_rate: 0.5,
            timestamp: "2025-10-13T12:00:00Z".to_string(),
        };

        let checkpoint = network.to_checkpoint(metadata);
        let result = Network::from_checkpoint(checkpoint);

        assert!(result.is_err());
    }
}

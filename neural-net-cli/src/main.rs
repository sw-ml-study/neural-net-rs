/// Neural Network Demonstration Platform CLI
///
/// Command-line interface for training and evaluating neural networks
/// on classic logic gate problems (AND, OR, XOR).
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "neural-net")]
#[command(about = "Neural Network Demonstration Platform", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List available training examples
    List,

    /// Train a neural network on an example
    Train {
        /// Example to train on (and, or, xor)
        #[arg(short, long)]
        example: String,

        /// Number of training epochs
        #[arg(short = 'n', long, default_value = "10000")]
        epochs: u32,

        /// Learning rate
        #[arg(short, long, default_value = "0.5")]
        learning_rate: f64,

        /// Output file path for trained model
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Resume training from a checkpoint
    Resume {
        /// Path to checkpoint file
        #[arg(short, long)]
        checkpoint: String,

        /// Number of additional training epochs
        #[arg(short = 'n', long)]
        epochs: u32,

        /// Output file path for updated model
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Evaluate a trained model
    Eval {
        /// Path to trained model file
        #[arg(short, long)]
        model: String,

        /// Input values (comma-separated)
        #[arg(short, long)]
        input: Option<String>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            cmd_list()?;
        }
        Commands::Train {
            example,
            epochs,
            learning_rate,
            output,
        } => {
            cmd_train(&example, epochs, learning_rate, output)?;
        }
        Commands::Resume {
            checkpoint,
            epochs,
            output,
        } => {
            cmd_resume(&checkpoint, epochs, output)?;
        }
        Commands::Eval { model, input } => {
            cmd_eval(&model, input)?;
        }
    }

    Ok(())
}

/// List available training examples
fn cmd_list() -> anyhow::Result<()> {
    use neural_network::examples;

    println!("Available Examples:");
    println!();

    for name in examples::list_examples() {
        let example = examples::get_example(name).unwrap();
        println!("  {} - {}", name, example.description);
    }

    Ok(())
}

/// Train a neural network
fn cmd_train(
    example: &str,
    epochs: u32,
    learning_rate: f64,
    output: Option<String>,
) -> anyhow::Result<()> {
    use neural_network::{activations::SIGMOID, checkpoint::CheckpointMetadata, examples, network::Network};
    use std::path::Path;

    // Load example
    let ex = examples::get_example(example)
        .ok_or_else(|| anyhow::anyhow!("Unknown example: {}. Use 'list' to see available examples.", example))?;

    println!("Training {} network", ex.name);
    println!("Architecture: {:?}", ex.recommended_arch);
    println!("Epochs: {}", epochs);
    println!("Learning rate: {}", learning_rate);
    println!();

    // Create network with recommended architecture
    let mut network = Network::new(ex.recommended_arch.clone(), SIGMOID, learning_rate);

    // Train network
    println!("Training...");
    network.train(ex.inputs.clone(), ex.targets.clone(), epochs);
    println!("Training complete!");

    // Save model if output path specified
    if let Some(output_path) = output {
        println!();
        println!("Saving model to: {}", output_path);

        let metadata = CheckpointMetadata {
            version: "1.0".to_string(),
            example: ex.name.to_string(),
            epoch: epochs,
            total_epochs: epochs,
            learning_rate,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        network.save_checkpoint(Path::new(&output_path), metadata)?;
        println!("Model saved successfully!");
    }

    Ok(())
}

/// Resume training from a checkpoint
fn cmd_resume(checkpoint: &str, epochs: u32, output: Option<String>) -> anyhow::Result<()> {
    use neural_network::{network::Network, training::{TrainingConfig, TrainingController}};
    use std::path::Path;

    let checkpoint_path = Path::new(checkpoint);

    println!("Resuming training from checkpoint: {}", checkpoint);
    println!("Additional epochs: {}", epochs);
    println!();

    // Load checkpoint to get training data info
    let (network, metadata) = Network::load_checkpoint(checkpoint_path)?;

    println!("Loaded checkpoint:");
    println!("  Architecture: {:?}", network.layers);
    println!("  Previous epochs: {}", metadata.epoch);
    println!("  Example: {}", metadata.example);
    println!("  Learning rate: {}", metadata.learning_rate);
    println!();

    // Get training data from example
    use neural_network::examples;
    let ex = examples::get_example(&metadata.example)
        .ok_or_else(|| anyhow::anyhow!("Example '{}' not found", metadata.example))?;

    // Create training config
    let config = TrainingConfig {
        epochs,
        checkpoint_interval: if output.is_some() { Some(epochs) } else { None },
        checkpoint_path: output.as_ref().map(|p| Path::new(p).to_path_buf()),
        verbose: false,
    };

    // Resume training
    let mut controller = TrainingController::from_checkpoint(checkpoint_path, config)?;

    println!("Resuming training...");
    controller.train(ex.inputs.clone(), ex.targets.clone())?;
    println!("Training complete!");

    // Save if output specified
    if let Some(output_path) = output {
        println!();
        println!("Model saved to: {}", output_path);
    }

    Ok(())
}

/// Evaluate a model (scaffold implementation)
fn cmd_eval(_model: &str, _input: Option<String>) -> anyhow::Result<()> {
    println!("Eval command scaffold");
    println!("(Full eval implementation coming in Phase 3)");

    Ok(())
}


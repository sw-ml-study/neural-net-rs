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

        /// Random seed for reproducible training
        #[arg(short, long)]
        seed: Option<u64>,

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

        /// Input values (comma-separated, e.g., "0.5,1.0" or "-1.0,0.5")
        #[arg(short, long, allow_hyphen_values = true)]
        input: Option<String>,
    },

    /// Display detailed model information
    Info {
        /// Path to model file
        #[arg(short, long)]
        model: String,
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
            seed,
            output,
        } => {
            cmd_train(&example, epochs, learning_rate, seed, output)?;
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
        Commands::Info { model } => {
            cmd_info(&model)?;
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
    seed: Option<u64>,
    output: Option<String>,
) -> anyhow::Result<()> {
    use indicatif::{ProgressBar, ProgressStyle};
    use neural_network::{activations::SIGMOID, examples, network::Network, training::{TrainingConfig, TrainingController}};
    use std::path::Path;

    // Load example
    let ex = examples::get_example(example)
        .ok_or_else(|| anyhow::anyhow!("Unknown example: {}. Use 'list' to see available examples.", example))?;

    println!("Training {} network", ex.name);
    println!("Architecture: {:?}", ex.recommended_arch);
    println!("Epochs: {}", epochs);
    println!("Learning rate: {}", learning_rate);
    if let Some(s) = seed {
        println!("Seed: {}", s);
    }
    println!();

    // Create network with recommended architecture
    let network = if let Some(s) = seed {
        Network::new_seeded(ex.recommended_arch.clone(), SIGMOID, learning_rate, s)
    } else {
        Network::new(ex.recommended_arch.clone(), SIGMOID, learning_rate)
    };

    // Create training config
    let config = TrainingConfig {
        epochs,
        checkpoint_interval: if output.is_some() { Some(epochs) } else { None },
        checkpoint_path: output.as_ref().map(|p| Path::new(p).to_path_buf()),
        verbose: false,
        example_name: Some(ex.name.to_string()),
    };

    // Create training controller
    let mut controller = TrainingController::new(network, config);

    // Setup progress bar
    let pb = ProgressBar::new(epochs as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.set_message("Training");

    // Add progress callback (clone pb for the closure)
    let pb_clone = pb.clone();
    controller.add_callback(Box::new(move |epoch, loss, _network| {
        pb_clone.set_position(epoch as u64);
        if epoch % 100 == 0 || epoch == 1 {
            pb_clone.set_message(format!("Training (loss: {:.6})", loss));
        }
    }));

    // Train network
    controller.train(ex.inputs.clone(), ex.targets.clone())?;
    pb.finish_with_message("Training complete!");

    // Save model if output path specified
    if let Some(output_path) = output {
        println!();
        println!("Saving model to: {}", output_path);
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
        example_name: Some(metadata.example.clone()),
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

/// Evaluate a trained model
fn cmd_eval(model: &str, input: Option<String>) -> anyhow::Result<()> {
    use neural_network::network::Network;
    use std::path::Path;

    let model_path = Path::new(model);

    // Load model
    let (mut network, metadata) = Network::load_checkpoint(model_path)?;

    // Display model info
    println!("Loaded model: {}", model);
    println!("  Example: {}", metadata.example);
    println!("  Architecture: {:?}", network.layers);
    println!("  Training epochs: {}", metadata.epoch);
    println!("  Learning rate: {}", metadata.learning_rate);
    println!();

    // Parse input if provided
    if let Some(input_str) = input {
        let inputs: Result<Vec<f64>, _> = input_str
            .split(',')
            .map(|s| s.trim().parse::<f64>())
            .collect();

        let inputs = inputs.map_err(|e| {
            anyhow::anyhow!("Invalid input format: {}. Expected comma-separated numbers (e.g., '0.0,1.0')", e)
        })?;

        // Validate input dimensions
        if inputs.len() != network.layers[0] {
            anyhow::bail!(
                "Invalid input dimensions: expected {} inputs, got {}",
                network.layers[0],
                inputs.len()
            );
        }

        // Run prediction
        let input_matrix = neural_network::matrix::Matrix::from(inputs.clone());
        let output = network.feed_forward(input_matrix);

        // Display results
        println!("Input: {:?}", inputs);
        println!("Output: {:?}", output.data);
    } else {
        println!("No input provided. Use --input <values> to make a prediction.");
        println!("Example: --input 0.0,1.0");
    }

    Ok(())
}

/// Display detailed model information
fn cmd_info(model: &str) -> anyhow::Result<()> {
    use neural_network::network::Network;
    use std::path::Path;

    let model_path = Path::new(model);

    // Load model
    let (network, metadata) = Network::load_checkpoint(model_path)?;

    // Display header
    println!("Model Information");
    println!("================");
    println!();

    // Display metadata
    println!("Metadata:");
    println!("  Version: {}", metadata.version);
    println!("  Example: {}", metadata.example);
    println!("  Training Epochs: {}", metadata.epoch);
    println!("  Total Epochs: {}", metadata.total_epochs);
    println!("  Learning Rate: {}", metadata.learning_rate);
    println!("  Timestamp: {}", metadata.timestamp);
    println!();

    // Display architecture
    println!("Architecture:");
    println!("  Layers: {:?}", network.layers);
    println!("  Input neurons: {}", network.layers[0]);
    println!("  Output neurons: {}", network.layers[network.layers.len() - 1]);
    if network.layers.len() > 2 {
        println!("  Hidden layers: {}", network.layers.len() - 2);
    }
    println!();

    // Display weight matrices
    println!("Weights:");
    let mut total_params = 0;
    for (i, weight) in network.weights.iter().enumerate() {
        let params = weight.rows * weight.cols;
        total_params += params;
        println!("  Layer {} -> {}: {}x{} ({} parameters)",
            i, i + 1, weight.rows, weight.cols, params);
    }
    println!();

    // Display bias vectors
    println!("Biases:");
    for (i, bias) in network.biases.iter().enumerate() {
        let params = bias.rows;
        total_params += params;
        println!("  Layer {}: {}x{} ({} parameters)",
            i + 1, bias.rows, bias.cols, params);
    }
    println!();

    // Display total parameters
    println!("Total Parameters: {}", total_params);

    Ok(())
}

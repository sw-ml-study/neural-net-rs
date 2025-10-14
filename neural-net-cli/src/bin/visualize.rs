// Network Visualization Tool
// Generates SVG visualization of neural network weights and architecture from checkpoint files

use anyhow::{Context, Result};
use clap::Parser;
use std::fs;

/// Visualize neural network architecture and weights from a checkpoint file
#[derive(Parser, Debug)]
#[command(name = "visualize")]
#[command(about = "Generate SVG visualization of neural network weights")]
struct Args {
    /// Path to checkpoint file
    #[arg(short, long)]
    checkpoint: String,

    /// Output SVG file path
    #[arg(short, long)]
    output: String,

    /// Width of SVG canvas in pixels
    #[arg(long, default_value = "1200")]
    width: u32,

    /// Height of SVG canvas in pixels
    #[arg(long, default_value = "800")]
    height: u32,

    /// Show weight values as text
    #[arg(long, default_value = "false")]
    show_values: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Neural Network Visualizer");
    println!("Reading checkpoint: {}", args.checkpoint);

    // Load checkpoint
    let checkpoint_data = fs::read_to_string(&args.checkpoint)
        .with_context(|| format!("Failed to read checkpoint file: {}", args.checkpoint))?;

    let checkpoint = serde_json::from_str::<neural_network::checkpoint::Checkpoint>(&checkpoint_data)
        .with_context(|| "Failed to parse checkpoint JSON")?;

    println!("Checkpoint metadata:");
    println!("  Example: {}", checkpoint.metadata.example);
    println!("  Epochs: {}/{}", checkpoint.metadata.epoch, checkpoint.metadata.total_epochs);
    println!("  Learning rate: {}", checkpoint.metadata.learning_rate);
    println!();

    let network = &checkpoint.network;
    println!("Network architecture: {:?}", network.layers);

    // Calculate statistics
    let mut total_weights = 0;
    let mut total_biases = 0;
    for weight in &network.weights {
        total_weights += weight.rows * weight.cols;
    }
    for bias in &network.biases {
        total_biases += bias.rows;
    }
    println!("Total parameters: {} ({} weights + {} biases)",
             total_weights + total_biases, total_weights, total_biases);
    println!();

    // Generate SVG
    let svg = generate_svg(network, &args)?;

    // Write SVG file
    fs::write(&args.output, svg)
        .with_context(|| format!("Failed to write SVG file: {}", args.output))?;

    println!("SVG visualization saved to: {}", args.output);
    println!("Open in browser to view interactive, zoomable visualization");

    Ok(())
}

fn generate_svg(network: &neural_network::network::Network, args: &Args) -> Result<String> {
    let width = args.width;
    let height = args.height;
    let margin = 80;

    let num_layers = network.layers.len();
    let _max_neurons = *network.layers.iter().max().unwrap();

    // Calculate spacing
    let layer_spacing = (width - 2 * margin) / (num_layers as u32 - 1);
    let neuron_size = 20;

    let mut svg = String::new();

    // SVG header with viewBox for zoomability
    svg.push_str(&format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" width="{}" height="{}">
<defs>
  <style>
    .neuron {{ fill: #4a90e2; stroke: #2c5aa0; stroke-width: 2; }}
    .neuron:hover {{ fill: #5ba3ff; cursor: pointer; }}
    .weight-line {{ stroke-opacity: 0.6; }}
    .weight-line:hover {{ stroke-opacity: 1.0; stroke-width: 3; }}
    .layer-label {{ font-family: Arial, sans-serif; font-size: 14px; fill: #333; }}
    .neuron-label {{ font-family: Arial, sans-serif; font-size: 10px; fill: #666; }}
    .weight-label {{ font-family: Arial, sans-serif; font-size: 8px; fill: #888; }}
    .title {{ font-family: Arial, sans-serif; font-size: 20px; font-weight: bold; fill: #333; }}
    .subtitle {{ font-family: Arial, sans-serif; font-size: 14px; fill: #666; }}
  </style>
</defs>

<!-- Background -->
<rect width="{}" height="{}" fill="#f5f7fa"/>

<!-- Title -->
<text x="{}" y="30" class="title" text-anchor="middle">Neural Network Architecture</text>
<text x="{}" y="50" class="subtitle" text-anchor="middle">{:?}</text>

"##,
        width, height, width, height, width, height,
        width / 2, width / 2, network.layers
    ));

    // Calculate neuron positions for each layer
    let mut neuron_positions = Vec::new();
    for (layer_idx, &num_neurons) in network.layers.iter().enumerate() {
        let x = margin + layer_idx as u32 * layer_spacing;
        let layer_height = num_neurons as u32 * (neuron_size * 3);
        let start_y = (height - layer_height) / 2;

        let mut layer_neurons = Vec::new();
        for neuron_idx in 0..num_neurons {
            let y = start_y + neuron_idx as u32 * (neuron_size * 3);
            layer_neurons.push((x, y));
        }
        neuron_positions.push(layer_neurons);
    }

    // Draw connections (weights) first so they appear behind neurons
    svg.push_str("<!-- Weight connections -->\n");
    for weight_idx in 0..network.weights.len() {
        let weights = &network.weights[weight_idx];
        let from_layer = &neuron_positions[weight_idx];
        let to_layer = &neuron_positions[weight_idx + 1];

        // Calculate weight statistics for this layer
        let mut min_weight = f64::INFINITY;
        let mut max_weight = f64::NEG_INFINITY;
        for row in 0..weights.rows {
            for col in 0..weights.cols {
                let w = weights.data[row * weights.cols + col];
                min_weight = min_weight.min(w.abs());
                max_weight = max_weight.max(w.abs());
            }
        }

        for from_idx in 0..from_layer.len() {
            for to_idx in 0..to_layer.len() {
                let (x1, y1) = from_layer[from_idx];
                let (x2, y2) = to_layer[to_idx];

                // Get weight value (weights are stored as [to_neurons x from_neurons])
                let weight = weights.data[to_idx * weights.cols + from_idx];
                let weight_abs = weight.abs();

                // Map weight to color and thickness
                let normalized = if max_weight > min_weight {
                    (weight_abs - min_weight) / (max_weight - min_weight)
                } else {
                    0.5
                };

                let color = if weight >= 0.0 {
                    // Positive weights: blue gradient
                    let intensity = (normalized * 200.0) as u8 + 55;
                    format!("rgb(55, {}, {})", intensity, 255)
                } else {
                    // Negative weights: red gradient
                    let intensity = (normalized * 200.0) as u8 + 55;
                    format!("rgb(255, {}, {})", intensity, intensity)
                };

                let thickness = 0.5 + normalized * 3.0;

                svg.push_str(&format!(
                    r##"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="{:.2}" class="weight-line">
  <title>Weight: {:.4} (from L{} N{} to L{} N{})</title>
</line>
"##,
                    x1, y1 + neuron_size / 2, x2, y2 + neuron_size / 2,
                    color, thickness, weight,
                    weight_idx, from_idx, weight_idx + 1, to_idx
                ));

                // Optionally show weight values
                if args.show_values && thickness > 2.0 { // Only show significant weights
                    let mid_x = (x1 + x2) / 2;
                    let mid_y = (y1 + y2) / 2 + neuron_size / 2;
                    svg.push_str(&format!(
                        r##"<text x="{}" y="{}" class="weight-label" text-anchor="middle">{:.2}</text>
"##,
                        mid_x, mid_y, weight
                    ));
                }
            }
        }
    }

    // Draw neurons
    svg.push_str("<!-- Neurons -->\n");
    let _layer_names = vec!["Input", "Hidden", "Hidden", "Hidden", "Hidden", "Output"];
    for (layer_idx, layer_neurons) in neuron_positions.iter().enumerate() {
        let layer_name = if layer_idx == 0 {
            "Input"
        } else if layer_idx == network.layers.len() - 1 {
            "Output"
        } else {
            "Hidden"
        };

        // Layer label
        let (first_x, first_y) = layer_neurons[0];
        svg.push_str(&format!(
            r##"<text x="{}" y="{}" class="layer-label" text-anchor="middle">Layer {}: {}</text>
"##,
            first_x, first_y - 20, layer_idx, layer_name
        ));

        for (neuron_idx, &(x, y)) in layer_neurons.iter().enumerate() {
            // Get bias if not input layer
            let bias_text = if layer_idx > 0 && layer_idx - 1 < network.biases.len() {
                let biases_for_layer = &network.biases[layer_idx - 1];
                if neuron_idx < biases_for_layer.data.len() {
                    let bias = biases_for_layer.data[neuron_idx];
                    format!("Bias: {:.4}", bias)
                } else {
                    "No bias".to_string()
                }
            } else {
                "Input node".to_string()
            };

            svg.push_str(&format!(
                r##"<circle cx="{}" cy="{}" r="{}" class="neuron">
  <title>L{} N{} - {}</title>
</circle>
<text x="{}" y="{}" class="neuron-label" text-anchor="middle">{}</text>
"##,
                x, y + neuron_size / 2, neuron_size,
                layer_idx, neuron_idx, bias_text,
                x, y + neuron_size / 2 + 5, neuron_idx
            ));
        }
    }

    // Legend
    svg.push_str(&format!(
        r##"
<!-- Legend -->
<g transform="translate({}, {})">
  <text x="0" y="0" class="layer-label">Legend:</text>
  <line x1="0" y1="15" x2="50" y2="15" stroke="rgb(55, 155, 255)" stroke-width="3" class="weight-line"/>
  <text x="60" y="20" class="neuron-label">Positive weight</text>
  <line x1="0" y1="35" x2="50" y2="35" stroke="rgb(255, 155, 155)" stroke-width="3" class="weight-line"/>
  <text x="60" y="40" class="neuron-label">Negative weight</text>
  <text x="0" y="60" class="neuron-label">Line thickness = weight magnitude</text>
  <text x="0" y="75" class="neuron-label">Hover over elements for details</text>
</g>

"##,
        margin, height - 100
    ));

    svg.push_str("</svg>");

    Ok(svg)
}

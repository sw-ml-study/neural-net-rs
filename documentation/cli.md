# CLI Documentation

The `neural-net-cli` provides a full-featured command-line interface for training and evaluating neural networks.

## Installation

```bash
# Build the project
cargo build --release

# Run the CLI
cargo run --bin neural-net-cli -- --help
```

## Commands Overview

| Command | Description |
|---------|-------------|
| `list` | List available training examples |
| `train` | Train a new network |
| `resume` | Resume training from checkpoint |
| `eval` | Evaluate a trained model |
| `info` | Display model information |

## Command Reference

### `list` - List Available Examples

Shows all built-in training examples with descriptions.

```bash
cargo run --bin neural-net-cli -- list
```

**Output:**
```
Available examples:
  and       - Logical AND operation [2, 2, 1]
  or        - Logical OR operation [2, 2, 1]
  xor       - Logical XOR operation [2, 3, 1]
  parity3   - 3-bit parity check [3, 6, 1]
  quadrant  - 2D quadrant classification [2, 8, 4]
  adder2    - 2-bit binary adder [4, 8, 3]
  iris      - Iris flower classification [4, 8, 3]
  pattern3x3 - 3x3 visual pattern recognition [9, 6, 4]
```

### `train` - Train a New Network

Train a neural network on one of the built-in examples.

```bash
cargo run --bin neural-net-cli -- train [OPTIONS]
```

**Options:**

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--example <NAME>` | `-e` | Example to train on (and, or, xor, etc.) | required |
| `--epochs <N>` | `-n` | Number of training epochs | 10000 |
| `--learning-rate <RATE>` | `-l` | Learning rate | 0.5 |
| `--output <FILE>` | `-o` | Output file path for trained model | none |
| `--seed <N>` | `-s` | Random seed for reproducibility | random |

**Examples:**

```bash
# Train XOR network with visual progress bar
cargo run --bin neural-net-cli -- train --example xor --epochs 10000

# Save the trained model
cargo run --bin neural-net-cli -- train --example xor --epochs 10000 --output checkpoints/xor_model.json

# Customize learning rate and use specific seed
cargo run --bin neural-net-cli -- train --example xor --epochs 10000 --learning-rate 0.3 --seed 42
```

**Features:**
- Visual progress bar with ETA
- Real-time loss tracking
- Automatic checkpoint saving (when --output specified)

### `resume` - Resume Training from Checkpoint

Continue training a previously saved model.

```bash
cargo run --bin neural-net-cli -- resume [OPTIONS]
```

**Options:**

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--checkpoint <FILE>` | `-c` | Path to checkpoint file | required |
| `--epochs <N>` | `-n` | Number of additional training epochs | required |
| `--output <FILE>` | `-o` | Output file path for updated model | none |

**Examples:**

```bash
# Resume training from checkpoint
cargo run --bin neural-net-cli -- resume --checkpoint checkpoints/xor_model.json --epochs 5000 --output checkpoints/xor_continued.json
```

### `eval` - Evaluate a Trained Model

Run inference on a trained model with specific inputs.

```bash
cargo run --bin neural-net-cli -- eval [OPTIONS]
```

**Options:**

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--model <FILE>` | `-m` | Path to trained model file | required |
| `--input <VALUES>` | `-i` | Input values (comma-separated) | required |

**Examples:**

```bash
# Evaluate XOR network
cargo run --bin neural-net-cli -- eval --model checkpoints/xor_model.json --input 1.0,0.0
# Output: [0.95]  (close to 1.0, correct XOR result)

# Evaluate all XOR combinations
cargo run --bin neural-net-cli -- eval --model checkpoints/xor_model.json --input 0.0,0.0  # ~0.0
cargo run --bin neural-net-cli -- eval --model checkpoints/xor_model.json --input 0.0,1.0  # ~1.0
cargo run --bin neural-net-cli -- eval --model checkpoints/xor_model.json --input 1.0,0.0  # ~1.0
cargo run --bin neural-net-cli -- eval --model checkpoints/xor_model.json --input 1.0,1.0  # ~0.0
```

### `info` - Display Model Information

Show detailed information about a saved model.

```bash
cargo run --bin neural-net-cli -- info [OPTIONS]
```

**Options:**

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--model <FILE>` | `-m` | Path to model file | required |

**Example Output:**

```
Model Information
=================
Version: 1.0.0
Example: xor
Epochs: 10000
Learning Rate: 0.5
Timestamp: 2025-10-14T01:35:30.210Z
Seed: 42

Network Architecture
====================
Layer 0: 2 neurons (input)
Layer 1: 3 neurons (hidden)
Layer 2: 1 neuron (output)

Parameters: 13 total
  Weights: 9
  Biases: 4
```

## Example Workflows

### Training XOR (Classic Non-Linear Problem)

```bash
# Train XOR with 10,000 epochs
cargo run --bin neural-net-cli -- train \
  --example xor \
  --epochs 10000 \
  --learning-rate 0.5 \
  --output checkpoints/xor.json

# Evaluate all combinations
cargo run --bin neural-net-cli -- eval --model checkpoints/xor.json --input 0.0,0.0  # ~0.0
cargo run --bin neural-net-cli -- eval --model checkpoints/xor.json --input 0.0,1.0  # ~1.0
cargo run --bin neural-net-cli -- eval --model checkpoints/xor.json --input 1.0,0.0  # ~1.0
cargo run --bin neural-net-cli -- eval --model checkpoints/xor.json --input 1.0,1.0  # ~0.0
```

### Long Training with Resume

```bash
# Initial training (5000 epochs)
cargo run --bin neural-net-cli -- train \
  --example xor \
  --epochs 5000 \
  --output checkpoints/xor_partial.json

# Check progress
cargo run --bin neural-net-cli -- info --model checkpoints/xor_partial.json

# Resume for another 5000 epochs
cargo run --bin neural-net-cli -- resume \
  --checkpoint checkpoints/xor_partial.json \
  --epochs 5000 \
  --output checkpoints/xor_full.json
```

## Recommended Training Settings

Each example has recommended hyperparameters tuned for reliable convergence:

| Example | Architecture | Epochs | Learning Rate | Seed | Difficulty |
|---------|-------------|--------|---------------|------|------------|
| **AND** | [2, 2, 1] | 5,000 | 0.5 | 42 | Easy |
| **OR** | [2, 2, 1] | 5,000 | 0.5 | 42 | Easy |
| **XOR** | [2, 3, 1] | 10,000 | 0.5 | 42 | Moderate |
| **Parity3** | [3, 6, 1] | 20,000 | 0.5 | 123 | Hard |
| **Quadrant** | [2, 8, 4] | 15,000 | 0.3 | 42 | Moderate |
| **Adder2** | [4, 8, 3] | 20,000 | 0.5 | 42 | Hard |
| **Iris** | [4, 8, 3] | 15,000 | 0.3 | 42 | Moderate |
| **Pattern3x3** | [9, 6, 4] | 15,000 | 0.5 | 42 | Moderate |

**Tips for Training:**
- If training stalls (loss stays flat), try a different seed
- Lower learning rates (0.1-0.3) are more stable but slower
- Higher learning rates (0.5-0.8) converge faster but may oscillate
- More epochs always help, but diminishing returns after convergence
- Complex examples (Parity3, Adder2) represent genuinely hard problems for small networks

**Batch Testing:**
Run `./scripts/batch-test.sh` to test all examples with their recommended settings and verify accuracy.

## Network Visualization

Generate SVG visualizations of trained networks:

```bash
# Train a network and save checkpoint
cargo run --bin neural-net-cli -- train --example xor --epochs 10000 --output checkpoints/xor_checkpoint.json

# Generate interactive SVG visualization
cargo run --bin visualize -- --checkpoint checkpoints/xor_checkpoint.json --output network.svg

# Open in browser to zoom and interact
open network.svg
```

**Visualization Options:**
- `--checkpoint <FILE>`: Path to checkpoint file (required)
- `--output <FILE>`: Output SVG file path (required)
- `--width <PIXELS>`: Canvas width (default: 1200)
- `--height <PIXELS>`: Canvas height (default: 800)
- `--show-values`: Display weight values as text on connections

**Visualization Features:**
- Color-coded weights: Blue = positive, Red = negative
- Weight magnitude shown by line thickness
- Interactive tooltips on hover (in browser)
- Zoomable SVG for detailed inspection
- Network statistics display

See the main [README](../README.md#network-architecture-visualizations) for example visualizations of each network type.

# Web UI Documentation

The project includes a modern, interactive web interface for training and evaluating neural networks directly in your browser.

**[Live Demo](https://sw-ml-study.github.io/neural-net-rs/)** - Try it now!

## Features

### Dual Training Modes

- **Local (WASM)**: Train neural networks directly in the browser using WebAssembly
  - No server required for training
  - All computation runs client-side
  - Full neural network implementation compiled to WASM
- **Remote (API)**: Train on the server with real-time progress via Server-Sent Events
  - Live training progress updates
  - Streaming loss metrics
  - Progress visualization

### Interactive Features

- Real-time loss chart with Canvas visualization
- Network architecture display
- Interactive testing interface
- Truth table evaluation with error highlighting
- Training metrics (epoch, loss, elapsed time)
- Progress bar with percentage completion
- Example selection (AND, OR, XOR, Quadrant, Parity, Adder, Iris, Pattern3x3)
- Configurable training parameters (epochs, learning rate)

## Running the Web UI

### Using the Run Script (Recommended)

```bash
# Start with WASM build, cache-busting updates, and server
./scripts/run.sh
```

This script handles everything: builds WASM, copies to static directories, updates cache-busting timestamps, and starts the server on port 2421.

### Manual Start

```bash
# Start the web server
cargo run --bin neural-net-server

# Open your browser
open http://localhost:2421
```

The web UI will load at `http://localhost:2421` and provide:
1. **Training Configuration Panel**: Select examples, configure parameters, choose training mode
2. **Training Progress Visualization**: Real-time loss chart and metrics
3. **Network Testing**: Interactive input/output testing
4. **Architecture Display**: Visual representation of network layers

## WebAssembly Integration

All neural network logic runs in Rust/WASM:
- Network creation and initialization
- Forward propagation
- Backpropagation and training
- Gradient computation
- Weight updates

JavaScript is minimal - only for:
- WASM module bootstrapping
- DOM manipulation
- Canvas chart drawing
- SSE connection handling

**WASM Module Size**: ~248KB (optimized for size with LTO)

## Example Workflows

### Local WASM Training

1. Select "Local (WASM)" mode
2. Choose an example (e.g., XOR)
3. Set epochs (e.g., 1000) and learning rate (e.g., 0.5)
4. Click "Start Training"
5. Watch real-time loss chart
6. Test the network with different inputs
7. View truth table results

### Remote API Training

1. Select "Remote API (SSE)" mode
2. Choose an example
3. Configure parameters
4. Click "Start Training"
5. Receive live progress updates from server
6. View streaming loss metrics
7. Test the trained network

## Screenshots

### Main Interface

![Neural Network Training Platform](../images/main-quadrant-2026-02-15T14-43-05-642Z.png?ts=1771166656000)
*Interactive web interface demonstrating QUADRANT multi-class classification with dynamic inputs, training loss curve, and evaluation results*

### Demonstrated Use Case: Multi-Class Classification

The screenshot above shows the **QUADRANT Gate (2-8-4)** example, which demonstrates several advanced features:

- **Multi-Class Classification**: Unlike simple binary outputs (AND, OR, XOR), this network classifies 2D points into one of four quadrants using one-hot encoding
- **Dynamic UI Adaptation**: The interface automatically adjusts input fields and output displays based on the selected example's architecture
- **Training Visualization**: The loss curve shows convergence over 1000 epochs, demonstrating successful learning
- **Real-Time Evaluation**: Testing inputs (0.8, 0.8) correctly predicts the quadrant with high confidence
- **Truth Table Analysis**: All four quadrant classifications are displayed with expected vs. predicted classes and confidence scores
- **Architecture Display**: Visual representation shows the 2-8-4 network structure (2 inputs, 8 hidden neurons, 4 output classes)

### UI Walkthrough

**Initial Interface**
![Web UI Initial View](../images/01-initial-2026-02-15T14-40-23-021Z.png?ts=1771166656000)
*Clean, modern interface with training configuration, visualization panels, and network testing*

**XOR Configuration**
![Configuration with XOR](../images/02-config-xor-2026-02-15T14-40-33-693Z.png?ts=1771166656000)
*Select an example and configure training parameters*

**Training in Progress**
![Training in Progress](../images/03-training-progress-2026-02-15T14-41-25-943Z.png?ts=1771166656000)
*Real-time loss chart updates as training progresses*

**Training Completed**
![Training Completed](../images/04-training-complete-2026-02-15T14-41-35-172Z.png?ts=1771166656000)
*Complete training history with loss curve and final metrics*

**Network Testing & Truth Table**
![Network Testing](../images/05-network-testing-2026-02-15T14-42-39-239Z.png?ts=1771166656000)
*Interactive testing interface with truth table showing all input combinations and prediction accuracy*

**Remote API Mode**
![Remote API Mode](../images/06-api-mode-2026-02-15T14-43-15-555Z.png?ts=1771166656000)
*Select Remote API mode for server-side training with SSE streaming*

## Technical Implementation

- **Framework**: Axum 0.7 for async web server
- **Runtime**: Tokio for async operations
- **SSE Streaming**: `spawn_blocking` for CPU-bound training with `std::sync::mpsc` channels
- **State Management**: Thread-safe `Arc<Mutex<HashMap>>` for model storage
- **CORS**: Permissive CORS for development
- **Static Files**: Tower-HTTP for serving web UI assets

## REST API Reference

See the main [README](../README.md#api-endpoints) for full API documentation.

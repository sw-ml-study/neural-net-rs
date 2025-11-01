# Architecture Documentation

## Neural Network Demonstration Platform - Technical Design

**Version:** 1.0
**Last Updated:** 2025-10-13

## Table of Contents

1. [System Overview](#system-overview)
2. [Architecture Diagram](#architecture-diagram)
3. [Component Design](#component-design)
4. [Data Flow](#data-flow)
5. [API Specifications](#api-specifications)
6. [Technology Stack](#technology-stack)
7. [Testing Strategy](#testing-strategy)
8. [Security Considerations](#security-considerations)

## System Overview

The platform consists of four major components:

1. **Core Library** (`matrix`, `neural-network`) - Unchanged neural network implementation
2. **CLI Application** (`neural-net-cli`) - Command-line interface for training and serving
3. **Web Server** (`neural-net-server`) - HTTP/SSE server for web UI
4. **Web UI** (`neural-net-wasm`) - WebAssembly frontend for visualization

### Design Principles

- **Separation of Concerns:** Core ML logic independent of presentation layers
- **Testability:** All components have clear interfaces and comprehensive tests
- **Modularity:** Features can be developed and tested independently
- **Progressive Enhancement:** CLI works without web UI, web UI enhances experience
- **Single Binary:** All components compile into one executable

## Architecture Diagram

```
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+                        User Interaction                      +
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
                +                        +
        ++++++++>+++++++++      ++++++++>+++++++++
        +   CLI Commands +      +   Web Browser  +
        +   (clap)       +      +   (localhost)  +
        ++++++++++++++++++      ++++++++++++++++++
                +                        +
                +                ++++++++>+++++++++
                +                +  Static Files  +
                +                +  (embedded)    +
                +                ++++++++++++++++++
        ++++++++>++++++++++++++++++++++++>+++++++++
        +         CLI Application                  +
        +  ++++++++++++++++++++++++++++++++++++++ +
        +  +  Command Handlers                   + +
        +  +  - train    - serve                 + +
        +  +  - eval     - list                  + +
        +  +  - info                             + +
        +  ++++++++++++++++++++++++++++++++++++++ +
        +            +                             +
        +  ++++++++++>+++++++++++++++++++++++++++ +
        +  +  Training Controller                + +
        +  +  - Progress tracking                + +
        +  +  - Checkpoint management            + +
        +  +  - Event emission                   + +
        +  ++++++++++++++++++++++++++++++++++++++ +
        +            +                             +
        +  ++++++++++>+++++++++++++++++++++++++++ +
        +  +  Web Server (Optional)              + +
        +  +  - HTTP endpoints                   + +
        +  +  - SSE event stream                 + +
        +  +  - WASM/JS/HTML serving            + +
        +  ++++++++++++++++++++++++++++++++++++++ +
        +++++++++++++++++++++++++++++++++++++++++++++
                     +
        +++++++++++++>+++++++++++++++++++++++++++
        +      Core Neural Network Library      +
        +  ++++++++++++++++++++++++++++++++++++ +
        +  +  Examples (AND, OR, XOR)          + +
        +  +  - Dataset definitions            + +
        +  +  - Recommended architectures      + +
        +  ++++++++++++++++++++++++++++++++++++ +
        +  ++++++++++++++++++++++++++++++++++++ +
        +  +  Network + Serialization          + +
        +  +  - Training loop                  + +
        +  +  - Checkpointing (serde)          + +
        +  +  - Model I/O                      + +
        +  ++++++++++++++++++++++++++++++++++++ +
        +  ++++++++++++++++++++++++++++++++++++ +
        +  +  Matrix Operations                + +
        +  +  - Linear algebra                 + +
        +  ++++++++++++++++++++++++++++++++++++ +
        +++++++++++++++++++++++++++++++++++++++++
```

## Component Design

### 1. Core Library Extensions

#### 1.1 `neural-network` Crate Extensions

**New: `examples.rs`**
```rust
pub struct Example {
    pub name: &'static str,
    pub description: &'static str,
    pub inputs: Vec<Vec<f64>>,
    pub targets: Vec<Vec<f64>>,
    pub recommended_arch: Vec<usize>,
    pub recommended_epochs: u32,
    pub recommended_lr: f64,
}

pub fn get_example(name: &str) -> Option<Example>;
pub fn list_examples() -> Vec<&'static str>;
```

**New: `checkpoint.rs`**
```rust
#[derive(Serialize, Deserialize)]
pub struct Checkpoint {
    pub metadata: CheckpointMetadata,
    pub network_state: NetworkState,
}

#[derive(Serialize, Deserialize)]
pub struct CheckpointMetadata {
    pub version: String,
    pub example: String,
    pub epoch: u32,
    pub total_epochs: u32,
    pub learning_rate: f64,
    pub timestamp: String,
}

impl Network {
    pub fn to_checkpoint(&self, metadata: CheckpointMetadata) -> Checkpoint;
    pub fn from_checkpoint(checkpoint: Checkpoint) -> Result<Self>;
    pub fn save_checkpoint(&self, path: &Path, metadata: CheckpointMetadata) -> Result<()>;
    pub fn load_checkpoint(path: &Path) -> Result<(Self, CheckpointMetadata)>;
}
```

**New: `training.rs` - Training Controller**
```rust
pub struct TrainingController {
    network: Network,
    config: TrainingConfig,
    callbacks: Vec<Box<dyn TrainingCallback>>,
}

pub trait TrainingCallback: Send {
    fn on_epoch_end(&mut self, epoch: u32, loss: f64, predictions: &[f64]);
    fn on_training_end(&mut self);
}

pub struct TrainingConfig {
    pub epochs: u32,
    pub checkpoint_interval: Option<u32>,
    pub checkpoint_path: Option<PathBuf>,
    pub verbose: bool,
}

impl TrainingController {
    pub fn new(network: Network, config: TrainingConfig) -> Self;
    pub fn add_callback(&mut self, callback: Box<dyn TrainingCallback>);
    pub fn train(&mut self, inputs: Vec<Vec<f64>>, targets: Vec<Vec<f64>>) -> Result<()>;
    pub fn resume_from_checkpoint(path: &Path) -> Result<Self>;
}
```

### 2. CLI Application (`neural-net-cli` crate)

**Structure:**
```
neural-net-cli/
+++ src/
+   +++ main.rs              # Entry point, CLI setup
+   +++ commands/
+   +   +++ mod.rs
+   +   +++ train.rs         # Train command handler
+   +   +++ eval.rs          # Eval command handler
+   +   +++ serve.rs         # Serve command handler
+   +   +++ list.rs          # List command handler
+   +   +++ info.rs          # Info command handler
+   +++ callbacks/
+   +   +++ mod.rs
+   +   +++ progress.rs      # Terminal progress bar
+   +   +++ sse.rs           # Server-sent events emitter
+   +++ server/
+   +   +++ mod.rs
+   +   +++ routes.rs        # HTTP endpoints
+   +   +++ state.rs         # Shared server state
+   +   +++ static_files.rs  # Embedded assets
+   +++ error.rs             # Error types
+++ Cargo.toml
+++ build.rs                 # Embed static files at compile time
```

**CLI Commands Definition:**
```rust
#[derive(Parser)]
#[command(name = "neural-net")]
#[command(about = "Neural Network Demonstration Platform")]
enum Cli {
    /// Train a neural network on an example
    Train(TrainArgs),

    /// Evaluate a trained model
    Eval(EvalArgs),

    /// Start web server for interactive training
    Serve(ServeArgs),

    /// List available examples
    List,

    /// Show information about a model
    Info(InfoArgs),
}

#[derive(Args)]
struct TrainArgs {
    /// Example to train: and, or, xor
    #[arg(short, long)]
    example: String,

    /// Number of training epochs
    #[arg(long, default_value = "10000")]
    epochs: u32,

    /// Learning rate
    #[arg(long, default_value = "0.5")]
    learning_rate: f64,

    /// Hidden layer sizes (comma-separated)
    #[arg(long)]
    hidden_layers: Option<String>,

    /// Save checkpoint during training
    #[arg(long)]
    checkpoint: Option<PathBuf>,

    /// Resume from checkpoint
    #[arg(long, conflicts_with = "example")]
    resume: Option<PathBuf>,

    /// Save trained model
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Show verbose output
    #[arg(short, long)]
    verbose: bool,
}
```

### 3. Web Server (`neural-net-server` module)

**Technology:** Axum (lightweight, modern, good async support)

**Endpoints:**

```rust
// GET /              - Serve main HTML page
// GET /app.js        - Serve WASM glue code
// GET /app.wasm      - Serve WASM binary
// GET /styles.css    - Serve CSS

// POST /api/train/start        - Start training session
// POST /api/train/stop         - Stop current training
// POST /api/train/pause        - Pause current training
// POST /api/train/resume       - Resume paused training
// GET  /api/examples           - List available examples
// GET  /api/models/:id         - Get model state
// GET  /api/events             - SSE event stream

// Request/Response Types
#[derive(Serialize, Deserialize)]
struct TrainRequest {
    example: String,
    epochs: u32,
    learning_rate: f64,
    hidden_layers: Vec<usize>,
}

#[derive(Serialize, Deserialize)]
struct TrainResponse {
    session_id: String,
    status: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum ServerEvent {
    TrainingStarted { session_id: String, config: TrainRequest },
    EpochComplete { epoch: u32, total: u32, loss: f64, predictions: Vec<f64> },
    TrainingComplete { session_id: String, final_accuracy: f64 },
    TrainingError { error: String },
}
```

**Server State:**
```rust
struct ServerState {
    active_session: Arc<Mutex<Option<TrainingSession>>>,
    event_tx: broadcast::Sender<ServerEvent>,
}

struct TrainingSession {
    id: String,
    controller: TrainingController,
    status: TrainingStatus,
    cancel_token: CancellationToken,
}

enum TrainingStatus {
    Running,
    Paused,
    Completed,
    Error(String),
}
```

### 4. WebAssembly Frontend (`neural-net-wasm` crate)

**Structure:**
```
neural-net-wasm/
+++ src/
+   +++ lib.rs               # WASM bindings
+   +++ network_wrapper.rs   # Network interface for JS
+   +++ utils.rs             # WASM utilities
+++ web/
+   +++ index.html           # Main page
+   +++ app.js               # Application logic
+   +++ chart.js             # Chart rendering (vanilla JS)
+   +++ styles.css           # Styling
+++ Cargo.toml
```

**WASM API:**
```rust
#[wasm_bindgen]
pub struct NetworkHandle {
    network: Network,
    training_thread: Option<JoinHandle<()>>,
}

#[wasm_bindgen]
impl NetworkHandle {
    #[wasm_bindgen(constructor)]
    pub fn new(layers: Vec<usize>, learning_rate: f64) -> Self;

    pub fn train_step(&mut self, inputs: Vec<f64>, targets: Vec<f64>) -> f64;

    pub fn predict(&mut self, inputs: Vec<f64>) -> Vec<f64>;

    pub fn get_weights(&self) -> JsValue;

    pub fn to_json(&self) -> String;

    pub fn from_json(json: &str) -> Result<NetworkHandle, JsValue>;
}

// JavaScript-facing functions
#[wasm_bindgen]
pub fn create_network_for_example(example: &str) -> NetworkHandle;

#[wasm_bindgen]
pub fn get_example_data(example: &str) -> JsValue;
```

**Web UI Components:**

1. **Control Panel**
   - Example selector dropdown
   - Start/Pause/Stop buttons
   - Architecture input (e.g., "2,3,1")
   - Learning rate slider
   - Epoch counter display

2. **Visualization Panel**
   - Training progress bar
   - Loss chart (canvas-based, real-time updates)
   - Current epoch / total epochs
   - Estimated time remaining

3. **Results Panel**
   - Truth table with predictions
   - Accuracy percentage
   - Color-coded correctness (green/red)

4. **Network Diagram** (Simple)
   - Visual representation of layers
   - Node counts per layer

## Data Flow

### Training Flow (CLI)

```
User Input (CLI args)
    ->
Parse & Validate
    ->
Load Example Data
    ->
Create/Load Network
    ->
Configure TrainingController
    ->
Attach Callbacks (Progress, Checkpoint)
    ->
Start Training Loop ++++++++++
    ->                        +
Epoch Iteration             +
    ->                        +
Callback: on_epoch_end      +
    ->                        +
Update Progress Bar         +
    ->                        +
Save Checkpoint? ++++++++++++
    ->
Training Complete
    ->
Callback: on_training_end
    ->
Save Model (if requested)
    ->
Display Results
```

### Training Flow (Web UI)

```
User Action (UI Button)
    ->
Fetch POST /api/train/start
    ->
Server: Validate & Create Session
    ->
Server: Start Training in Background Task
    ->
Client: Connect to SSE /api/events
    ->
Server: Emit TrainingStarted Event
    ->
Training Loop +++++++++++++++++
    ->                         +
Epoch Complete               +
    ->                         +
Emit EpochComplete Event     +
    ->                         +
Client: Receive SSE Event    +
    ->                         +
Update UI (progress, chart)  +
    ->                         +
Continue? ++++++++++++++++++++
    ->
Training Complete
    ->
Emit TrainingComplete Event
    ->
Client: Update UI (final results)
```

### Checkpoint Flow

```
Training in Progress
    ->
Checkpoint Interval Reached
    ->
Serialize Network State
    {
        metadata: { epoch, example, timestamp, ... },
        weights: [...],
        biases: [...],
        architecture: [2, 3, 1],
        ...
    }
    ->
Write to File (JSON)
    ->
Continue Training

--- Resume ---

User: --resume checkpoint.json
    ->
Load & Parse File
    ->
Validate Architecture
    ->
Deserialize Network State
    ->
Restore Network
    ->
Continue from Checkpoint Epoch
```

## API Specifications

### REST API

#### POST /api/train/start
**Request:**
```json
{
  "example": "xor",
  "epochs": 20000,
  "learning_rate": 0.5,
  "hidden_layers": [3]
}
```

**Response:**
```json
{
  "session_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "started"
}
```

#### GET /api/events (Server-Sent Events)
**Stream Format:**
```
event: training_started
data: {"session_id":"...","config":{...}}

event: epoch_complete
data: {"epoch":100,"total":20000,"loss":0.125,"predictions":[0.01,0.98,0.02,0.99]}

event: training_complete
data: {"session_id":"...","final_accuracy":0.995}
```

### File Formats

#### Checkpoint Format (JSON)
```json
{
  "version": "1.0",
  "metadata": {
    "example": "xor",
    "epoch": 5000,
    "total_epochs": 20000,
    "learning_rate": 0.5,
    "timestamp": "2025-10-13T12:00:00Z"
  },
  "network": {
    "layers": [2, 3, 1],
    "weights": [
      {"rows": 3, "cols": 2, "data": [...]},
      {"rows": 1, "cols": 3, "data": [...]}
    ],
    "biases": [
      {"rows": 3, "cols": 1, "data": [...]},
      {"rows": 1, "cols": 1, "data": [...]}
    ],
    "activation": "sigmoid"
  }
}
```

#### Model Format (JSON)
```json
{
  "version": "1.0",
  "metadata": {
    "name": "XOR Model",
    "trained_epochs": 20000,
    "final_accuracy": 0.995,
    "created": "2025-10-13T12:00:00Z"
  },
  "network": { /* same as checkpoint */ }
}
```

## Technology Stack

### Core Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `clap` | 4.x | CLI parsing with derive macros |
| `serde` | 1.x | Serialization framework |
| `serde_json` | 1.x | JSON format support |
| `anyhow` | 1.x | Error handling |
| `thiserror` | 1.x | Error derive macros |
| `tokio` | 1.x | Async runtime |
| `axum` | 0.7.x | Web server framework |
| `tower-http` | 0.5.x | HTTP middleware (CORS, compression) |
| `wasm-bindgen` | 0.2.x | WebAssembly bindings |
| `web-sys` | 0.3.x | Browser API bindings |
| `console_error_panic_hook` | 0.1.x | Better WASM errors |
| `indicatif` | 0.17.x | Progress bars |

### Build Tools

- `wasm-pack` - Build WASM packages
- `trunk` - WASM bundler (optional, for development)
- `cargo-watch` - Auto-rebuild during development

## Testing Strategy

### Unit Tests

**Coverage Target:** > 80% for all modules

**Test Organization:**
```
tests/
+++ unit/
+   +++ examples_tests.rs
+   +++ checkpoint_tests.rs
+   +++ training_controller_tests.rs
+   +++ serialization_tests.rs
+++ integration/
+   +++ cli_tests.rs
+   +++ server_tests.rs
+   +++ end_to_end_tests.rs
+++ fixtures/
    +++ checkpoints/
    +++ models/
```

**Key Test Cases:**

1. **Examples Module**
   - All examples return valid data
   - Architectures are valid
   - Input/output dimensions match

2. **Checkpointing**
   - Save and load preserve network state
   - Serialization roundtrip is lossless
   - Corrupted files are detected
   - Version compatibility checks work

3. **Training Controller**
   - Callbacks are invoked correctly
   - Checkpoints save at correct intervals
   - Training can be resumed
   - Ctrl+C saves checkpoint before exit

4. **CLI Commands**
   - All argument combinations work
   - Error messages are correct
   - File I/O works as expected
   - Exit codes are appropriate

5. **Server**
   - All endpoints return correct status codes
   - SSE stream emits correct events
   - Concurrent request handling works
   - Static files are served correctly

### Integration Tests

**Scenarios:**

1. **End-to-End Training (CLI)**
   ```bash
   neural-net train --example xor --epochs 1000 --output model.json
   neural-net eval --model model.json --test-all
   ```

2. **Checkpoint Resume**
   ```bash
   neural-net train --example xor --epochs 5000 --checkpoint cp.json
   # Simulate interrupt
   neural-net train --resume cp.json --epochs 10000
   ```

3. **Web UI Training**
   ```bash
   neural-net serve --port 8080
   # Automated browser testing with headless Chrome
   ```

### Performance Tests

**Benchmarks:**
- Training 10,000 epochs of XOR should complete in < 5 seconds
- Checkpoint save/load should be < 100ms
- Server response time should be < 50ms
- WASM initialization should be < 500ms

## Security Considerations

### Input Validation

1. **CLI Arguments**
   - Validate file paths (no directory traversal)
   - Validate numeric ranges (epochs > 0, lr in 0.0-1.0)
   - Validate architecture (reasonable layer sizes)

2. **Server Requests**
   - Validate JSON structure
   - Rate limiting (max 1 training session per connection)
   - Timeout long-running requests
   - Sanitize error messages (no path disclosure)

3. **File Operations**
   - Validate file extensions
   - Check file sizes before loading
   - Use safe path joining
   - Handle symbolic links carefully

### Resource Limits

1. **Memory**
   - Limit maximum network size
   - Stream large files instead of loading fully
   - Clear training data after completion

2. **CPU**
   - Allow training cancellation (Ctrl+C)
   - Yield to runtime periodically in training loop
   - Limit concurrent training sessions to 1

3. **Disk**
   - Limit checkpoint file size
   - Clean up temporary files
   - Validate disk space before writing

### WASM Security

1. **Data Isolation**
   - No sensitive data in WASM module
   - All computation is client-side
   - No external network calls from WASM

2. **Error Handling**
   - Don't expose internal errors to console
   - Validate all JS-to-Rust calls
   - Use Result types consistently

## Deployment Considerations

### Binary Distribution

**Single Binary:**
- Embed all assets (HTML, CSS, JS, WASM)
- Use `include_bytes!` macro
- Compress assets at build time

**Build Script:**
```bash
# Build WASM
cd neural-net-wasm && wasm-pack build --target web

# Build CLI (embeds WASM artifacts)
cd .. && cargo build --release --bin neural-net-cli

# Result: single binary at target/release/neural-net-cli
```

### Cross-Platform Support

- Use `std::path::Path` for all paths
- Detect OS for terminal capabilities
- Test on macOS, Linux, Windows
- Provide platform-specific instructions

## Future Architecture Improvements

1. **Plugin System:** Allow custom examples via dynamic loading
2. **Distributed Training:** Support for training across multiple machines
3. **Model Registry:** Central repository for trained models
4. **Advanced Visualizations:** 3D weight visualization, activation maps
5. **Export Formats:** ONNX, TensorFlow Lite support
6. **Performance:** SIMD optimizations, GPU acceleration

## References

- [Clap Documentation](https://docs.rs/clap/)
- [Axum Examples](https://github.com/tokio-rs/axum/tree/main/examples)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [Server-Sent Events Spec](https://html.spec.whatwg.org/multipage/server-sent-events.html)

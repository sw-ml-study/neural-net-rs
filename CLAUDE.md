# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## CRITICAL: Review Learnings Document FIRST

**WARNING: BEFORE STARTING ANY WORK, READ documentation/learnings.md WARNING**

Since AI assistants cannot learn from experience across sessions, `documentation/learnings.md` serves as institutional memory tracking mistakes, corrections, and best practices. Every Claude Code instance MUST:

1. **Read documentation/learnings.md** at the start of each session
2. **Update documentation/learnings.md** when new mistakes are discovered or patterns emerge
3. **Review relevant sections** before performing similar tasks

Key documented patterns:
- TDD methodology (RED has two components: build AND test phases)
- Use `tempfile::TempDir` for ALL test temp directories (not manual timestamp-based dirs)
- Run clippy before committing
- Export new modules in lib.rs immediately
- Put dependencies in correct section (dependencies vs dev-dependencies)

## CRITICAL: Language Constraints

**NO PYTHON, JAVASCRIPT, OR TYPESCRIPT PERMITTED**

This is a Rust project (with WASM for browser). Do not use:
- Python scripts for any purpose (including serving files)
- JavaScript for build/test automation
- TypeScript for any tooling

Use Rust alternatives:
- For HTTP serving: `basic-http-server` (Rust-based)
- For tooling: Rust binaries or shell scripts only
- Web UI uses vanilla JS only because it runs in the browser (not Node.js)

## Workspace Structure

This is a Cargo workspace with seven crates (Rust 2024 edition, resolver = "2"):

**Core libraries:**
- **matrix**: Core linear algebra library providing `Matrix` type and operations
- **neural-network**: Neural network implementation (depends on matrix)

**Applications:**
- **neural-net-cli**: Full-featured CLI for training/evaluating networks
- **neural-net-server**: Axum-based REST API with SSE streaming
- **neural-net-wasm**: WebAssembly bindings for browser execution
- **consumer_binary**: Simple XOR training example
- **visualize** (binary in neural-network): SVG network visualization

## Build Commands

```bash
# Build all workspace members
cargo build

# Build release version
cargo build --release

# Run tests
cargo test

# Run tests for specific crate
cargo test -p matrix
cargo test -p neural-network
cargo test -p neural-net-cli
cargo test -p neural-net-server

# Run a single test by name
cargo test -p neural-network test_name_here

# Run clippy (REQUIRED before commits)
cargo clippy --all-targets --all-features

# Auto-fix clippy warnings
cargo clippy --fix --allow-dirty --all-targets --all-features
```

## Running Applications

```bash
# CLI: Train XOR network
cargo run --bin neural-net-cli -- train --example xor --epochs 10000

# CLI: List available examples
cargo run --bin neural-net-cli -- list

# CLI: Evaluate trained model
cargo run --bin neural-net-cli -- eval --model checkpoints/model.json --input 1.0,0.0

# Server: Start web server (default localhost:3000)
cargo run --bin neural-net-server

# Server: Custom host/port
cargo run --bin neural-net-server -- --host 0.0.0.0 --port 2421

# Visualize: Generate network SVG
cargo run --bin visualize -- --checkpoint checkpoints/xor.json --output network.svg
```

## WASM Build

```bash
# Install wasm-pack if needed
cargo install wasm-pack

# Build WASM module (outputs to neural-net-wasm/pkg/)
cd neural-net-wasm && wasm-pack build --target web

# Copy to server static directory
cp neural-net-wasm/pkg/* neural-net-server/static/wasm/
```

## Build Info & Cache Busting

**IMPORTANT:** Run the build script before deploying or committing UI changes:

```bash
./scripts/update-build-info.sh
```

This script updates:
- **Build info** in HTML footers (git hash, hostname, timestamp)
- **Copyright year** (automatically extends range, e.g., 2025-2026)
- **Cache-busting** timestamps (`?ts=<epoch-ms>`) on app.js and WASM imports

Cache-busting uses epoch milliseconds to ensure browsers load fresh assets after updates. The script modifies:
- `neural-net-server/static/index.html`
- `neural-net-server/static/app.js`
- `docs/index.html`
- `docs/app.js`

## Architecture

### Matrix Library (matrix crate)

- **Matrix struct**: Stores data in flat `Vec<f64>` with row-major ordering (access via `i * cols + j`)
- **Core operations**: Element-wise multiply, dot product, transpose, addition, subtraction
- **Generic `map` function**: Accepts closures for element-wise transformations (essential for backprop)
- **Construction**: `Matrix::new()`, `Matrix::zeros()`, `Matrix::random()`, or `matrix!` macro

### Neural Network (neural-network crate)

Feed-forward networks with backpropagation:

- **Network struct**: Configurable layers, activation function, learning rate
- **Training flow**: `feed_forward()` → stores activations in `self.data` → `back_propogate()` uses stored activations for gradients
- **Checkpoint system**: JSON serialization for save/resume training
- **TrainingController**: Callback support, auto-checkpointing, progress tracking
- **Examples module**: Built-in AND, OR, XOR, parity, quadrant, adder, iris, pattern problems

### Server Architecture (neural-net-server)

- **Axum 0.7** web framework with Tokio async runtime
- **REST API**: `/api/examples`, `/api/train`, `/api/eval`, `/api/models/:id`
- **SSE streaming**: `/api/train/stream` for real-time training progress
- **State**: Thread-safe `Arc<Mutex<HashMap>>` for model storage
- **Static files**: Serves web UI from `static/` directory

### WASM Module (neural-net-wasm)

- Compiles neural network to WebAssembly via `wasm-bindgen`
- Same training logic runs client-side in browser
- Progress callbacks via JavaScript interop

## Important Implementation Details

### Rust 2024 Edition

- `env::set_var` requires an `unsafe` block
- Workspace resolver 2 is required

### Serialization

Function pointers (like activation functions) cannot be serialized directly. The `Activation` struct uses custom `Serialize`/`Deserialize` implementations that serialize the function name as a string.

### Nested Struct Derives

When a struct derives `Debug`/`Clone`/`Serialize`, all contained types must also implement those traits. This commonly affects `Network` contained in `Checkpoint`.

## Testing Strategy

- Tests colocated with implementation in `#[cfg(test)]` modules
- Integration tests in `tests/` directories
- **ALWAYS use `tempfile::TempDir`** for test directories (see learnings.md)
- 136+ tests total across all crates

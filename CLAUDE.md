# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

neural-net-rs is an educational neural network framework implemented in Rust. The project demonstrates fundamental concepts of neural networks through a simple, flexible architecture suitable for learning and experimentation.

## CRITICAL: Review Learnings Document

**⚠️ BEFORE STARTING ANY WORK, READ docs/learnings.md ⚠️**

Since AI assistants cannot learn from experience across sessions, `docs/learnings.md` serves as institutional memory tracking mistakes, corrections, and best practices. Every Claude Code instance MUST:

1. **Read docs/learnings.md** at the start of each session
2. **Update docs/learnings.md** when new mistakes are discovered or patterns emerge
3. **Review relevant sections** before performing similar tasks

Key sections in learnings.md:
- TDD methodology (RED has two components: build AND test phases)
- Clippy checks to run before committing
- Module export patterns
- Dependency management (dependencies vs dev-dependencies)
- Common Rust compilation errors and fixes

Failing to review learnings.md may result in repeating previously solved mistakes.

## Workspace Structure

This is a Cargo workspace with three crates:

- **matrix**: Core linear algebra library providing `Matrix` type and operations
- **neural-network**: Neural network implementation that depends on `matrix`
- **consumer_binary**: Example binary demonstrating XOR problem training

The workspace uses Rust 2024 edition and resolver = "2".

## Build Commands

```bash
# Build all workspace members
cargo build

# Build release version
cargo build --release

# Run the example binary (XOR training demo)
cargo run --bin consumer_binary

# Run tests
cargo test

# Run tests for specific crate
cargo test -p matrix
cargo test -p neural-network

# Run clippy linter
cargo clippy --all-targets --all-features

# Clean build artifacts
cargo clean
```

## Architecture

### Matrix Library

The `matrix` crate provides the foundation for neural network operations:

- **Matrix struct**: Stores data in a flat `Vec<f64>` with row-major ordering (access via `i * cols + j`)
- **Core operations**: Element-wise multiply, dot product, transpose, addition, subtraction
- **Functional operations**: Generic `map` function accepts closures for element-wise transformations
- **Construction**: `Matrix::new()`, `Matrix::zeros()`, `Matrix::random()`, or use the `matrix!` macro
- **`matrix!` macro**: Provides clean syntax for matrix literals (defined in `matrix/src/macros.rs`)

### Neural Network

The `neural-network` crate implements feed-forward networks with backpropagation:

- **Network struct**: Configurable via layers (neuron counts per layer), activation function, and learning rate
- **Activation functions**: Defined as structs with function/derivative pairs (see `activations.rs`). Currently implements SIGMOID
- **Training flow**:
  1. `feed_forward()`: Propagates inputs through layers, stores intermediate activations in `self.data`
  2. `back_propogate()`: Updates weights and biases using stored activations and computed gradients
  3. `train()`: Convenience method for multiple epochs over dataset
- **Weight initialization**: Random values in [0, 1) via `Matrix::random()`

### Data Flow

The network stores intermediate activations during forward pass in the `data` field, which are then used during backpropagation to compute gradients. Weights connect layer i to layer i+1, and biases are applied to each layer (except input).

## Important Implementation Details

### Map Function Signature

The `Matrix::map` function uses generics to accept closures:
```rust
pub fn map<F>(&mut self, func: F) -> Matrix
where
    F: Fn(&f64) -> f64
```

This allows capturing values (e.g., `self.learning_rate`) in closures, which is essential for the backpropagation implementation.

### Rust 2024 Edition Considerations

- `env::set_var` requires an `unsafe` block (see `consumer_binary/src/main.rs`)
- Workspace resolver 2 is required for proper dependency resolution across workspace members

## Module Organization

- `matrix` crate: `matrix.rs` (core impl), `macros.rs` (matrix! macro), `lib.rs` (module exports)
- `neural-network` crate: `network.rs` (Network impl), `activations.rs` (activation functions), `lib.rs` (re-exports matrix for convenience)
- `consumer_binary`: `main.rs` (example usage - trains XOR problem)

## Testing Strategy

Tests are colocated with implementation in `#[cfg(test)]` modules. The matrix library has comprehensive tests for all operations. When adding features, maintain this pattern of inline tests.

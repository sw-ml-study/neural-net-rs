# Product Requirements Document (PRD)

## Neural Network Demonstration Platform

**Version:** 1.0
**Date:** 2025-10-13
**Status:** Planning

## Executive Summary

Transform neural-net-rs into an interactive educational platform that enables presenters to demonstrate machine learning concepts through a command-line interface and web-based visualization tool. The platform will support multiple training examples, live training visualization, and model persistence.

## Goals

### Primary Goals
1. Enable educators and presenters to demonstrate neural network training interactively
2. Provide visual feedback on training progress and results
3. Support multiple classic ML problems (AND, OR, XOR) with pre-configured examples
4. Allow model checkpointing for interrupted training sessions
5. Create an accessible web interface for non-technical audiences

### Success Metrics
- CLI can train and visualize all three logic gate problems
- Web UI updates training progress in real-time
- Models can be saved and resumed without data loss
- Complete presentation script executable in under 15 minutes
- Zero-setup web UI accessible via localhost

## Target Users

### Primary Personas

**1. Technical Presenter (Primary)**
- Background: Software engineer, data scientist, or educator
- Needs: Quick setup, reliable demos, clear visualizations
- Use case: Conference talks, university lectures, technical workshops

**2. Student/Learner (Secondary)**
- Background: Learning ML fundamentals
- Needs: Visual understanding, experimentation capability
- Use case: Self-paced learning, homework assignments

**3. Curious Observer (Tertiary)**
- Background: Non-technical audience member
- Needs: Accessible visualizations, no installation required
- Use case: Watching live demonstrations

## Functional Requirements

### FR-1: Command-Line Interface

**FR-1.1: Training Commands**
```bash
neural-net train <EXAMPLE> [OPTIONS]
  --example, -e <TYPE>        Logic gate: and, or, xor [required]
  --epochs <N>                Number of training epochs [default: 10000]
  --learning-rate <RATE>      Learning rate [default: 0.5]
  --hidden-layers <LAYERS>    Hidden layer sizes [default: "3"]
  --checkpoint <PATH>         Save checkpoint to path
  --resume <PATH>             Resume from checkpoint
  --output <PATH>             Save trained model
  --verbose, -v               Show detailed progress
```

**FR-1.2: Evaluation Commands**
```bash
neural-net eval <MODEL> [OPTIONS]
  --model, -m <PATH>          Path to trained model [required]
  --input <VALUES>            Input values (e.g., "0.0,1.0")
  --test-all                  Run all test cases for the example
```

**FR-1.3: Server Commands**
```bash
neural-net serve [OPTIONS]
  --port <PORT>               Server port [default: 8080]
  --open                      Open browser automatically
  --example <TYPE>            Pre-load example [optional]
```

**FR-1.4: Information Commands**
```bash
neural-net list                # List available examples
neural-net info <MODEL>        # Show model architecture
neural-net --help              # Show help
neural-net --version           # Show version
```

### FR-2: Training Examples

**FR-2.1: AND Gate**
- Inputs: All combinations of 2 binary inputs
- Output: Logical AND (only 1,1 -> 1)
- Architecture: [2, 2, 1] (simpler, linearly separable)
- Expected convergence: ~5,000 epochs

**FR-2.2: OR Gate**
- Inputs: All combinations of 2 binary inputs
- Output: Logical OR (any 1 -> 1)
- Architecture: [2, 2, 1] (simpler, linearly separable)
- Expected convergence: ~5,000 epochs

**FR-2.3: XOR Gate**
- Inputs: All combinations of 2 binary inputs
- Output: Logical XOR (different -> 1)
- Architecture: [2, 3, 1] (needs hidden layer, non-linear)
- Expected convergence: ~20,000 epochs

**FR-2.4: Example Metadata**
Each example includes:
- Name and description
- Input/output dataset
- Recommended architecture
- Expected training time
- Explanation of the problem (for UI)

### FR-3: Model Checkpointing

**FR-3.1: Checkpoint Format**
- Serializable format (JSON or binary)
- Contains: weights, biases, architecture, training state
- Metadata: epoch count, loss history, hyperparameters

**FR-3.2: Save Operations**
- Auto-save at intervals during training
- Manual save on completion
- Save on interrupt (Ctrl+C handling)

**FR-3.3: Resume Operations**
- Load full network state
- Continue training from last epoch
- Validate architecture compatibility

### FR-4: Web User Interface

**FR-4.1: Training View**
- Real-time training progress (current epoch, total epochs)
- Loss curve visualization (line chart)
- Live accuracy metrics
- Current predictions for all test cases
- Visual representation of input/output mapping

**FR-4.2: Interactive Controls**
- Start/pause/stop training
- Adjust learning rate (with retrain)
- Change architecture (with retrain)
- Step through epochs manually
- Reset to initial state

**FR-4.3: Visualization Components**
- Input/output truth table with predictions
- Network architecture diagram (simple)
- Loss graph with epoch markers
- Color-coded accuracy indicators (green/red)
- Training status indicators

**FR-4.4: Example Selection**
- Dropdown to switch between AND, OR, XOR
- Quick-start buttons for each example
- Description panel explaining the problem

### FR-5: Presentation Support

**FR-5.1: Progress Indicators**
- Clear terminal output during training
- Percentage completion
- Estimated time remaining
- Final accuracy summary

**FR-5.2: Demonstration Modes**
- Fast mode: Reduced output for quick demos
- Verbose mode: Detailed metrics for education
- Interactive mode: Web UI for visual learners

**FR-5.3: Reproducibility**
- Optional seed parameter for deterministic training
- Save/load presentation configurations
- Quick reset to demo-ready state

## Non-Functional Requirements

### NFR-1: Performance
- Training should complete in reasonable demo time (< 30 seconds for XOR)
- Web UI updates should be smooth (30+ FPS)
- WASM bundle should be < 5MB
- Server startup should be < 2 seconds

### NFR-2: Usability
- CLI help text should be comprehensive
- Error messages should be actionable
- Web UI should work on modern browsers without plugins
- No external dependencies required for basic usage

### NFR-3: Reliability
- Graceful handling of Ctrl+C during training
- Checkpoint corruption detection
- Network validation before training
- Clear error messages for invalid configurations

### NFR-4: Maintainability
- Modular architecture (separate concerns)
- Comprehensive test coverage (>80%)
- API documentation for all public interfaces
- Example code for extending with new problems

### NFR-5: Portability
- CLI works on macOS, Linux, Windows
- Web UI works on Chrome, Firefox, Safari, Edge
- Single binary distribution
- No runtime dependencies

## Technical Constraints

### TC-1: Technology Stack
- Rust for CLI and core library
- WebAssembly (wasm-bindgen) for web frontend
- HTML/CSS/JavaScript for UI (vanilla, no frameworks)
- Actix-web or Axum for HTTP server
- Clap for CLI parsing
- Serde for serialization

### TC-2: Browser Compatibility
- Support browsers with WebAssembly support (2017+)
- Graceful degradation for older browsers
- Mobile-responsive design (bonus, not required)

### TC-3: Data Format
- JSON for checkpoints and models (human-readable)
- WebSocket or SSE for real-time updates
- REST API for control commands

## Out of Scope (V1)

- Multi-layer perceptron with >3 layers
- Different activation functions (only SIGMOID for V1)
- Batch training or mini-batches
- GPU acceleration
- Distributed training
- Additional example problems beyond logic gates
- Mobile native applications
- User authentication or multi-user support
- Cloud deployment or hosting
- Advanced optimizers (Adam, RMSprop, etc.)

## Future Considerations (V2+)

- Additional activation functions (ReLU, tanh)
- More complex examples (MNIST digits, simple regression)
- Network architecture visualization (animated)
- Export trained models to other formats (ONNX)
- Batch training support
- Learning rate schedules
- Regularization techniques (dropout, L2)
- Performance profiling tools

## Dependencies

### External Crates (Estimated)
- `clap` - CLI argument parsing
- `serde`, `serde_json` - Serialization
- `actix-web` or `axum` - Web server
- `tokio` - Async runtime
- `wasm-bindgen` - WebAssembly bindings
- `web-sys` - Browser APIs
- `console_error_panic_hook` - WASM error handling
- `anyhow` - Error handling
- `thiserror` - Error types

## Open Questions

1. Should we use WebSockets or Server-Sent Events for real-time updates?
   - **Recommendation:** SSE (simpler, unidirectional, sufficient for this use case)

2. Should checkpoints be JSON or binary format?
   - **Recommendation:** JSON (human-readable, easier debugging, file size not critical)

3. Should we embed the web UI in the binary or serve from disk?
   - **Recommendation:** Embed in binary (single-file distribution, easier deployment)

4. Should we support custom datasets via CSV/JSON input?
   - **Recommendation:** V2 feature (keeps V1 focused)

5. How should we handle concurrent training requests in server mode?
   - **Recommendation:** Single active training session (reject concurrent requests)

## Acceptance Criteria

### CLI Functionality
- [ ] All three examples (AND, OR, XOR) train successfully
- [ ] Models can be saved and loaded without error
- [ ] Training can be resumed from checkpoint
- [ ] Help text is comprehensive and accurate
- [ ] Error messages are clear and actionable

### Web UI Functionality
- [ ] Training progress updates in real-time
- [ ] Loss curve displays correctly
- [ ] Predictions update during training
- [ ] All three examples can be trained via UI
- [ ] UI is responsive and performant

### Presentation Support
- [ ] Presentation script executes without errors
- [ ] Demo completes in under 15 minutes
- [ ] Visual output is clear and professional
- [ ] Can recover from common demo issues (Ctrl+C, restart)

### Quality Metrics
- [ ] Test coverage > 80%
- [ ] All public APIs documented
- [ ] Zero clippy warnings
- [ ] Zero compiler warnings
- [ ] README updated with new features
- [ ] Architecture documentation complete

## Timeline (Estimated)

- **Week 1:** Core refactoring, CLI implementation, examples
- **Week 2:** Checkpointing, serialization, tests
- **Week 3:** WASM setup, basic web UI
- **Week 4:** Real-time updates, polish, documentation
- **Week 5:** Integration testing, presentation script, final polish

## Stakeholder Approval

- [ ] Technical Lead: _________________
- [ ] Product Owner: _________________
- [ ] Date: _________________

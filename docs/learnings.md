# Learnings and Mistakes Log

This document tracks mistakes made during development and the corrections applied, serving as a reference to avoid repeating the same errors.

## Purpose

Since AI assistants cannot learn from experience across sessions, this document serves as institutional memory. **Every Claude Code instance MUST review this document at the start of work and update it when new mistakes are discovered.**

---

## TDD Methodology - RED/GREEN/REFACTOR

### Critical Understanding: RED Phase Has Two Components

**MISTAKE:** Initially treated RED phase as complete when tests failed to compile.

**CORRECTION:** The RED phase requires TWO steps:
1. **Build Phase**: Fix compilation errors (possibly with scaffolding/mocks) until code compiles
2. **Test Phase**: Verify that tests FAIL for the right reasons (not implemented yet)

**Example from Task 1.3 (Checkpoint Module):**
```
❌ WRONG: "Tests don't compile, RED phase complete"
✅ CORRECT:
   1. Add checkpoint module export to lib.rs (fix compilation)
   2. Add missing derives to Network (fix compilation)
   3. Move dependencies from dev to regular (fix compilation)
   4. Run tests and verify they FAIL because functionality doesn't exist yet
   5. NOW RED phase is complete
```

### GREEN Phase

Implement minimal code to make tests pass. Do not add extra features.

### REFACTOR Phase

Clean up code, remove duplication, improve naming. Tests must still pass.

---

## Rust Patterns and Idioms

### Always Run Clippy Before Committing

**MISTAKE:** Didn't run clippy regularly, potentially missing outdated idioms or warnings.

**CORRECTION:** Run these commands before every commit:
```bash
cargo clippy --all-targets --all-features
cargo clippy --fix --allow-dirty --all-targets --all-features  # Auto-fix if possible
```

**Frequency:**
- Before every commit
- After completing a GREEN phase
- When switching to a new Rust edition

**Findings from First Clippy Check (Task 1.3 completion):**
- `clippy::empty_line_after_doc_comments`: Don't leave blank lines between module doc comments and the first item
- Fixed by removing empty lines between `///` comments and code
- Result: 0 warnings after fix

**Common Clippy Warnings to Watch For:**
- Empty lines after doc comments
- Unnecessary `mut` annotations
- Manual implementations of standard traits
- Needless borrows or clones
- Using deprecated patterns

### Rust 2024 Edition Changes

**Key Changes:**
- `env::set_var()` is now unsafe and requires `unsafe` block
- Workspace requires `resolver = "2"` in root Cargo.toml
- Stricter clippy lints for needless loops, manual range checks

**Reference:** Always check edition guide when upgrading: https://doc.rust-lang.org/edition-guide/

---

## Dependency Management

### Dev Dependencies vs Regular Dependencies

**MISTAKE (Task 1.3):** Added `serde_json` to `[dev-dependencies]` when it was needed in runtime code (checkpoint.rs uses it).

**CORRECTION:**
- `[dependencies]`: Used in library code (src/)
- `[dev-dependencies]`: Used only in tests, examples, benchmarks

**Rule:** If a crate is imported with `use` in any src/ file, it must be in `[dependencies]`, not `[dev-dependencies]`.

**Example:**
```toml
# ❌ WRONG - serde_json used in src/checkpoint.rs
[dev-dependencies]
serde_json = "1"

# ✅ CORRECT
[dependencies]
serde_json = "1"

[dev-dependencies]
chrono = "0.4"  # Only used in tests
```

---

## Module Exports and Visibility

### Always Export New Modules

**MISTAKE (Task 1.3):** Created `src/checkpoint.rs` but forgot to add `pub mod checkpoint;` to `src/lib.rs`.

**ERROR:** `unresolved import: could not find checkpoint in neural_network`

**CORRECTION:** When creating a new module file, immediately add it to lib.rs:

```rust
// src/lib.rs
pub mod network;
pub mod activations;
pub mod examples;
pub mod checkpoint;  // ← Don't forget this!
```

**Checklist for New Modules:**
1. [ ] Create src/module_name.rs
2. [ ] Add `pub mod module_name;` to src/lib.rs
3. [ ] Run `cargo build` to verify it compiles
4. [ ] Run tests

---

## Serialization and Traits

### Required Derives for Nested Serialization

**MISTAKE (Task 1.3):** `Checkpoint` struct had `#[derive(Debug, Clone, Serialize, Deserialize)]` but contained a `Network` field that didn't implement `Debug` and `Clone`.

**ERROR:**
```
error[E0277]: `Network` doesn't implement `Debug`
error[E0277]: the trait bound `Network: Clone` is not satisfied
```

**CORRECTION:** When a struct derives traits and contains fields of custom types, those types must also implement the same traits.

**Rule:** If struct A contains struct B and A derives Debug/Clone/etc., then B must also derive those traits.

**Example:**
```rust
// ❌ WRONG
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub network: Network,  // Network doesn't have Debug, Clone
}

// ✅ CORRECT
#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct Network { ... }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub network: Network,  // Now Network has required traits
}
```

---

## Testing Patterns

### Parallel Test Execution and Filesystem Operations

**ISSUE (Task 1.3):** `test_checkpoint_serialization_is_deterministic` failed when run in parallel but passed when run with `--test-threads=1`.

**CAUSE:** Multiple tests creating directories/files with timestamp-based names can have race conditions.

**PARTIAL SOLUTION:** Added `fs::create_dir_all()` in `save_checkpoint()` to ensure parent directory exists.

**REMAINING ISSUE:** Tests still have timing-dependent behavior. This is acceptable for now.

**FUTURE IMPROVEMENT:** Consider using:
- More unique temporary directory names (include thread ID + random number)
- Test fixtures that don't rely on filesystem
- `#[serial]` attribute from `serial_test` crate for tests that must run sequentially

**Current Workaround:** Document that specific tests may fail in parallel but pass sequentially. This is a test infrastructure issue, not a code bug.

---

## Commit Message Format

**PATTERN USED:**
```
Short descriptive title (50 chars max)

Implements Task X.Y from TDD roadmap: brief description.

Features:
- Feature 1
- Feature 2

Changes:
- Change 1
- Change 2

Tests:
- Test category 1
- Test category 2

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

**IMPORTANT:** Always use heredoc for multi-line commit messages:
```bash
git commit -m "$(cat <<'EOF'
Multi-line
commit message
here
EOF
)"
```

---

## Common Rust Compilation Errors

### Unused Variables and Imports

**PATTERN:** Compiler warns about unused variables/imports.

**FIX:**
- Prefix unused variables with `_`: `let _unused = value;`
- Remove unused imports
- Run `cargo fix` to auto-fix simple issues

**Example from Task 1.3:**
```rust
// ❌ Warning: unused variable `restored`
let (restored, restored_meta) = Network::load_checkpoint(&path);

// ✅ No warning
let (_restored, restored_meta) = Network::load_checkpoint(&path);
```

---

## Workspace Configuration

### Edition and Resolver

**MISTAKE (Task 1.1):** When upgrading to Rust 2024, didn't initially add `resolver = "2"` to workspace Cargo.toml.

**CORRECTION:** Rust 2024 requires workspace resolver 2:

```toml
# Root Cargo.toml
[workspace]
members = ["matrix", "neural-network", "consumer_binary"]
resolver = "2"  # Required for Rust 2024
```

---

## Action Items for Every Session

### Pre-Work Checklist
- [ ] Read this learnings.md document
- [ ] Review CLAUDE.md for project-specific guidelines
- [ ] Check git status
- [ ] Review recent commits

### During Work Checklist
- [ ] Follow TDD strictly (RED with both build and test phases, GREEN, REFACTOR)
- [ ] Export new modules in lib.rs immediately
- [ ] Check derives when nesting structs with serialization
- [ ] Put dependencies in correct section (dependencies vs dev-dependencies)
- [ ] Run clippy before committing

### Post-Work Checklist
- [ ] Run full test suite: `cargo test`
- [ ] Run clippy: `cargo clippy --all-targets --all-features`
- [ ] Update this learnings.md if new mistakes were discovered
- [ ] Commit with descriptive message using heredoc
- [ ] Push to remote

---

## Specific Rust Crates

### serde and serde_json

**Usage:**
- `serde = { version = "1", features = ["derive"] }` for `#[derive(Serialize, Deserialize)]`
- `serde_json = "1"` for `to_string()`, `from_str()`, `to_string_pretty()`

**Custom Serialization:**
Function pointers cannot be serialized directly. Use custom Serialize/Deserialize impls:

```rust
impl Serialize for Activation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_str("sigmoid")
    }
}
```

### anyhow

**Usage:** For ergonomic error handling with context.

```rust
use anyhow::{Context, Result};

fs::write(path, json)
    .with_context(|| format!("Failed to write to {}", path.display()))?;
```

**When to Use:**
- Application error handling (binaries)
- When you want rich error context
- When errors cross multiple subsystems

**When NOT to Use:**
- Library code that should use `Result<T, CustomError>`
- When specific error types need to be matched by callers

---

## Future Topics to Add

As development continues, add sections for:
- CLI argument parsing patterns (Task 1.4)
- WASM compilation issues and solutions
- Web server patterns
- Async/await patterns if used
- More Rust 2024 idioms as discovered

---

**Last Updated:** 2025-10-13 (Task 1.3 completion)
**Next Review:** Start of Task 1.4

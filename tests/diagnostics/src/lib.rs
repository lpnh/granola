//! Compile-fail tests for the diagnostics emitted by `#[derive(Recipe)]`.
//!
//! The assertions live in `tests/ui.rs`, which drives [`trybuild`] over the
//! fixtures in `tests/ui/`. Each `.rs` fixture is expected to fail compilation
//! with the message captured in its sibling `.stderr` snapshot.

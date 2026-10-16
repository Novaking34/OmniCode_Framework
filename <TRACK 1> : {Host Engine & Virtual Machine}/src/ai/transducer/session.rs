// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( Transduction Session Identity, Bindings Manifest & State Tracking )
// ============================================================================
//! # Transduction Session (`session.rs`)
//!
//! Captures the full state of a natural language $\leftrightarrow$ machine
//! cognitive round-trip, including synthesized code and execution state.

use std::collections::HashMap;
use crate::core::ternary::QuadState;

/// Captures the full context of a single cognitive transduction cycle.
#[derive(Debug, Clone)]
pub struct TransductionSession {
    pub prompt: String,
    pub synthesized_omnicode: String,
    pub execution_state: QuadState,
    pub natural_response: String,
    pub bindings_summary: HashMap<String, String>,
}

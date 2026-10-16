// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( IndexedSession Data Structure for Fast Episodic Retrieval )
// ============================================================================
//! Indexed Episode Metadata Definition

use std::path::PathBuf;
use crate::core::ternary::QuadState;

/// High-level indexed record representing a past execution episode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedSession {
    /// Unique session identifier string.
    pub session_id: String,
    /// Monotonic timestamp in microseconds since epoch.
    pub timestamp_micros: u128,
    /// Domain category (e.g. "character", "number", "sessions", "cortex").
    pub category: String,
    /// Final fourfold commit state.
    pub final_state: QuadState,
    /// Number of failpoints trapped during execution.
    pub failpoint_count: usize,
    /// Number of automated repairs applied by Restorator.
    pub repair_count: usize,
    /// Path to the native .scroll file.
    pub scroll_path: PathBuf,
    /// Path to the traditional .log file if present.
    pub log_path: Option<PathBuf>,
    /// Natural language or execution prompt that initiated the run.
    pub prompt: String,
}

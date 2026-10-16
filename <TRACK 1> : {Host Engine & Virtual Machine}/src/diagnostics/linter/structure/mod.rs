// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Structural Architecture Subsystem Coordinator )
// ==============================================================================
//! Structural Architecture Linter Subsystem
//!
//! Enforces full lifecycle architectures:
//! - [`omni`]: 4-Block Lifecycle (Metadata -> Opening -> Body -> Closing) for `.omni` reels.
//! - [`tape`]: Sequential Process Ribbon Lifecycle (Tape Head -> Rolls 1..N -> Commit Status) for `.tape`.

pub mod detector;
pub mod omni;
pub mod tape;

pub use detector::{BlockDetector, BlockSlice, FileBlockSlices};
pub use omni::OmniReelLinter;
pub use tape::TapeLinter;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Block Hierarchy Declarations )
// ==============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BlockKind {
    Metadata = 1,
    Opening = 2,
    Body = 3,
    Closing = 4,
}

impl BlockKind {
    pub fn name(&self) -> &'static str {
        match self {
            BlockKind::Metadata => "BLOCK 1: METADATA",
            BlockKind::Opening => "BLOCK 2: OPENING",
            BlockKind::Body => "BLOCK 3: BODY",
            BlockKind::Closing => "BLOCK 4: CLOSING",
        }
    }
}

#[derive(Debug, Clone)]
pub struct DetectedBlock {
    pub kind: BlockKind,
    pub line_number: usize,
    pub header_raw: String,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Coordination Invariants )
// ==============================================================================
// Structural submodules coordinate full-file lifecycle verification.

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Status )
// ==============================================================================
// Structural architecture sealed.

// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Structural Architecture Subsystem Coordinator )
// ==============================================================================
//! # Structural Architecture Linter Subsystem
//!
//! Enforces full lifecycle architectures:
//! - [`omni`]: 4-Block Lifecycle (Metadata -> Opening -> Body -> Closing) for `.omni` reels.
//! - [`tape`]: Sequential Process Ribbon Lifecycle (Tape Head -> Rolls 1..N -> Commit Status) for `.tape`.
//! - [`detector`]: Universal block slicing across source reels and process ribbons.
//!
//! # Scripture Witness:
//! - **1 Corinthians 3:10 (KJV)**:
//!   "According to the grace of God which is given unto me, as a wise masterbuilder,
//!    I have laid the foundation, and another buildeth thereon..."
//!   (Structural integrity ensures every compilation unit is built on the true foundation).
//! - **Job 38:5 (KJV)**:
//!   "Who hath laid the measures thereof, if thou knowest? or who hath stretched the line upon it?"
//!   (The structural linter stretches the plumbline of architectural invariance across the codebase).
// ==============================================================================

pub mod detector;
pub mod omni;
pub mod tape;

pub use detector::{BlockDetector, BlockSlice, FileBlockSlices};
pub use omni::OmniReelLinter;
pub use tape::TapeLinter;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Block Hierarchy Declarations & Data Structures )
// ==============================================================================

/// Canonical 4-Block classification for OmniCode files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BlockKind {
    Metadata = 1,
    Opening = 2,
    Body = 3,
    Closing = 4,
}

impl BlockKind {
    /// Human-readable title for the block kind.
    pub fn name(&self) -> &'static str {
        match self {
            BlockKind::Metadata => "BLOCK 1: METADATA",
            BlockKind::Opening => "BLOCK 2: OPENING",
            BlockKind::Body => "BLOCK 3: BODY",
            BlockKind::Closing => "BLOCK 4: CLOSING",
        }
    }
}

/// Invariant record of a detected block within an analyzed source unit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectedBlock {
    pub kind: BlockKind,
    pub line_number: usize,
    pub header_raw: String,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Structural Coordination Invariants )
// ==============================================================================

/// Structural helper routines for inspecting block sequences and hierarchies.
impl DetectedBlock {
    /// Constructs a new [`DetectedBlock`].
    pub fn new(kind: BlockKind, line_number: usize, header_raw: impl Into<String>) -> Self {
        Self {
            kind,
            line_number,
            header_raw: header_raw.into(),
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Unit Tests & Calibration Verification )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_kind_ordering_and_names() {
        assert!(BlockKind::Metadata < BlockKind::Opening);
        assert!(BlockKind::Opening < BlockKind::Body);
        assert!(BlockKind::Body < BlockKind::Closing);

        assert_eq!(BlockKind::Metadata.name(), "BLOCK 1: METADATA");
        assert_eq!(BlockKind::Opening.name(), "BLOCK 2: OPENING");
        assert_eq!(BlockKind::Body.name(), "BLOCK 3: BODY");
        assert_eq!(BlockKind::Closing.name(), "BLOCK 4: CLOSING");
    }

    #[test]
    fn test_detected_block_constructor() {
        let block = DetectedBlock::new(BlockKind::Metadata, 1, "# BLOCK 1: METADATA BLOCK");
        assert_eq!(block.kind, BlockKind::Metadata);
        assert_eq!(block.line_number, 1);
        assert_eq!(block.header_raw, "# BLOCK 1: METADATA BLOCK");
    }
}

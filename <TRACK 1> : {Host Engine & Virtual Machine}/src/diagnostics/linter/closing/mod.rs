// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Block 4 Closing & Commit Status Subsystem Coordinator )
// ==============================================================================
//! OmniCode Block 4 Closing Subsystem
//!
//! Evaluates postcondition verification proofs and commit status completeness:
//! - [`reel`]: Source Reel (`.omni`) Closing block verification.
//! - [`tape_commit`]: Process Tape (`.tape`) terminal commit status roll verification.
//! - [`score`]: `ClosingScore` model on continuous $[-100 \dots +100]$ space.

pub mod reel;
pub mod score;
pub mod tape_commit;

pub use reel::ReelClosingLinter;
pub use score::ClosingScore;
pub use tape_commit::TapeCommitLinter;

use crate::diagnostics::linter::syntax::SpatialLintConfig;
use crate::diagnostics::linter::structure::detector::BlockDetector;

pub struct ClosingLinter;

impl ClosingLinter {
    /// Evaluates and scores Block 4 for either a Source Reel or Process Tape.
    pub fn evaluate(
        file_path: &str,
        content: &str,
        is_tape: bool,
        config: &SpatialLintConfig,
    ) -> ClosingScore {
        let blocks = BlockDetector::detect_blocks(content, is_tape);
        if is_tape {
            TapeCommitLinter::evaluate(file_path, &blocks.closing.content, config)
        } else {
            ReelClosingLinter::evaluate(file_path, &blocks.closing.content, config)
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
    fn test_character_omni_block4_closing_scores_100() {
        let config = SpatialLintConfig::default();
        let path = crate::core::filetype::resolve_track_path("lexicon/01_nouns/ideas/types/character.omni");
        if path.exists() {
            let content = std::fs::read_to_string(&path).expect("Read character.omni");
            let score = ClosingLinter::evaluate("character.omni", &content, false, &config);
            assert_eq!(
                score.total, 100,
                "character.omni Block 4 Closing must score 100/100, got: {} ({:?})",
                score.total, score.breakdown
            );
            assert_eq!(score.rating, "A+ (Covenant Sealed & Verified)");
        }
    }

    #[test]
    fn test_character_tape_block4_commit_scores_100() {
        let config = SpatialLintConfig::default();
        let path = crate::core::filetype::resolve_track_path("lexicon/01_nouns/ideas/types/character.tape");
        if path.exists() {
            let content = std::fs::read_to_string(&path).expect("Read character.tape");
            let score = ClosingLinter::evaluate("character.tape", &content, true, &config);
            assert_eq!(
                score.total, 100,
                "character.tape Roll 6 Commit Status must score 100/100, got: {} ({:?})",
                score.total, score.breakdown
            );
            assert_eq!(score.rating, "A+ (Covenant Sealed & Verified)");
        }
    }
}


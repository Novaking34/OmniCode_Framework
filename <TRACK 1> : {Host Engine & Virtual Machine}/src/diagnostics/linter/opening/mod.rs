// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Block 2 Opening & Setup Subsystem Coordinator )
// ==============================================================================
//! OmniCode Block 2 Opening Subsystem
//!
//! Evaluates precondition equilibrium and execution container setup:
//! - [`reel`]: Source Reel (`.omni`) Opening block verification.
//! - [`tape_setup`]: Process Tape (`.tape`) Roll 1 initial setup verification.
//! - [`score`]: `OpeningScore` model on continuous $[-100 \dots +100]$ space.

pub mod reel;
pub mod score;
pub mod tape_setup;

pub use reel::ReelOpeningLinter;
pub use score::OpeningScore;
pub use tape_setup::TapeSetupLinter;

use crate::diagnostics::linter::syntax::SpatialLintConfig;
use crate::diagnostics::linter::structure::detector::BlockDetector;

pub struct OpeningLinter;

impl OpeningLinter {
    /// Evaluates and scores Block 2 for either a Source Reel or Process Tape.
    pub fn evaluate(
        file_path: &str,
        content: &str,
        is_tape: bool,
        config: &SpatialLintConfig,
    ) -> OpeningScore {
        let blocks = BlockDetector::detect_blocks(content, is_tape);
        if is_tape {
            TapeSetupLinter::evaluate(file_path, &blocks.opening.content, config)
        } else {
            ReelOpeningLinter::evaluate(file_path, &blocks.opening.content, config)
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
    fn test_character_omni_block2_opening_scores_100() {
        let config = SpatialLintConfig::default();
        let path = crate::core::filetype::resolve_track_path("lexicon/01_nouns/ideas/types/character.omni");
        if path.exists() {
            let content = std::fs::read_to_string(&path).expect("Read character.omni");
            let score = OpeningLinter::evaluate("character.omni", &content, false, &config);
            assert_eq!(
                score.total, 100,
                "character.omni Block 2 Opening must score 100/100, got: {} ({:?})",
                score.total, score.breakdown
            );
            assert_eq!(score.rating, "A+ (Equilibrium Certified)");
        }
    }

    #[test]
    fn test_character_tape_block2_setup_scores_100() {
        let config = SpatialLintConfig::default();
        let path = crate::core::filetype::resolve_track_path("lexicon/01_nouns/ideas/types/character.tape");
        if path.exists() {
            let content = std::fs::read_to_string(&path).expect("Read character.tape");
            let score = OpeningLinter::evaluate("character.tape", &content, true, &config);
            assert_eq!(
                score.total, 100,
                "character.tape Roll 1 setup must score 100/100, got: {} ({:?})",
                score.total, score.breakdown
            );
            assert_eq!(score.rating, "A+ (Equilibrium Certified)");
        }
    }
}


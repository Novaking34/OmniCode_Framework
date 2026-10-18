// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Block 3 Body & Tape Rolls Subsystem Coordinator )
// ==============================================================================
//! # OmniCode Block 3 Body Subsystem
//!
//! Evaluates operational transformations, causal pipelines, and tape rolls:
//! - [`reel`]: Source Reel (`.omni`) Body block verification.
//! - [`tape_rolls`]: Process Tape (`.tape`) operational rolls verification.
//! - [`score`]: `BodyScore` model on continuous $[-100 \dots +100]$ space.
//!
//! # Scripture Witness:
//! - **Colossians 2:19 (KJV)**:
//!   "And not holding the Head, from which all the body by joints and bands having nourishment
//!    ministered, and knit together, increaseth with the increase of God."
//!   (The Body block coordinates joints, bands, operational verbs, and causal transitions into one unified living organism).
//! - **Ecclesiastes 3:1 (KJV)**:
//!   "To every thing there is a season, and a time to every purpose under the heaven."
//!   (Operational rolls execute in strict sequential time, advancing causality roll by roll).
// ==============================================================================

pub mod reel;
pub mod score;
pub mod tape_rolls;

pub use reel::ReelBodyLinter;
pub use score::BodyScore;
pub use tape_rolls::TapeRollsLinter;

use crate::diagnostics::linter::syntax::SpatialLintConfig;
use crate::diagnostics::linter::structure::detector::BlockDetector;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( BodyLinter Coordinator Definition )
// ==============================================================================

pub struct BodyLinter;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Polymorphic Dispatch Across Source Reels and Process Tapes )
// ==============================================================================

impl BodyLinter {
    /// Evaluates and scores Block 3 for either a Source Reel or Process Tape.
    pub fn evaluate(
        file_path: &str,
        content: &str,
        is_tape: bool,
        config: &SpatialLintConfig,
    ) -> BodyScore {
        let blocks = BlockDetector::detect_blocks(content, is_tape);
        if is_tape {
            TapeRollsLinter::evaluate(file_path, &blocks.body.content, config)
        } else {
            ReelBodyLinter::evaluate(file_path, &blocks.body.content, config)
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
    fn test_character_omni_block3_body_scores_100() {
        let config = SpatialLintConfig::default();
        let path = crate::core::filetype::resolve_track_path("lexicon/01_nouns/ideas/types/character.omni");
        if path.exists() {
            let content = std::fs::read_to_string(&path).expect("Read character.omni");
            let score = BodyLinter::evaluate("character.omni", &content, false, &config);
            assert_eq!(
                score.total, 100,
                "character.omni Block 3 Body must score 100/100, got: {} ({:?})",
                score.total, score.breakdown
            );
            assert_eq!(score.rating, "A+ (Causally Complete)");
        }
    }

    #[test]
    fn test_character_tape_block3_rolls_scores_100() {
        let config = SpatialLintConfig::default();
        let path = crate::core::filetype::resolve_track_path("lexicon/01_nouns/ideas/types/character.tape");
        if path.exists() {
            let content = std::fs::read_to_string(&path).expect("Read character.tape");
            let score = BodyLinter::evaluate("character.tape", &content, true, &config);
            assert_eq!(
                score.total, 100,
                "character.tape Rolls section must score 100/100, got: {} ({:?})",
                score.total, score.breakdown
            );
            assert_eq!(score.rating, "A+ (Causally Complete)");
        }
    }

    #[test]
    fn test_number_omni_block3_body_scores_100() {
        let config = SpatialLintConfig::default();
        let path = crate::core::filetype::resolve_track_path("lexicon/01_nouns/ideas/types/number.omni");
        if path.exists() {
            let content = std::fs::read_to_string(&path).expect("Read number.omni");
            let score = BodyLinter::evaluate("number.omni", &content, false, &config);
            assert_eq!(
                score.total, 100,
                "number.omni Block 3 Body must score 100/100, got: {} ({:?})",
                score.total, score.breakdown
            );
            assert_eq!(score.rating, "A+ (Causally Complete)");
        }
    }

    #[test]
    fn test_number_tape_block3_rolls_scores_100() {
        let config = SpatialLintConfig::default();
        let path = crate::core::filetype::resolve_track_path("lexicon/01_nouns/ideas/types/number.tape");
        if path.exists() {
            let content = std::fs::read_to_string(&path).expect("Read number.tape");
            let score = BodyLinter::evaluate("number.tape", &content, true, &config);
            assert_eq!(
                score.total, 100,
                "number.tape Rolls section must score 100/100, got: {} ({:?})",
                score.total, score.breakdown
            );
            assert_eq!(score.rating, "A+ (Causally Complete)");
        }
    }
}

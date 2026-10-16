// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Block 1 Metadata & Tape Head Invariant Subsystem Coordinator )
// ==============================================================================
//! OmniCode Block 1 Metadata & Tape Head Subsystem
//!
//! Organizes Block 1 verification into dedicated, domain-specific modules:
//! - [`score`]: `MetadataScore` invariant model, point totals, and rating tiers.
//! - [`reel`]: Source Reel (`.omni`) Module Manifest & Invariant Type inspection.
//! - [`head`]: Process Tape (`.tape`) Tape Head & Cardinal Coordinates inspection.
//!
//! While `.omni` and `.tape` have different syntaxes and structures, both
//! embody the same proper ontological understanding: foundational identity,
//! provenance, covenant witness grounding, invariants/clauses, and spatial hygiene.

pub mod head;
pub mod reel;
pub mod score;

pub use head::TapeHeadLinter;
pub use reel::ReelMetadataLinter;
pub use score::MetadataScore;

use crate::diagnostics::linter::syntax::SpatialLintConfig;
use crate::diagnostics::linter::LintReport;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( MetadataLinter Coordinator Definition )
// ==============================================================================

pub struct MetadataLinter;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Polymorphic Dispatch Across Source Reels and Process Tapes )
// ==============================================================================

impl MetadataLinter {
    /// Evaluates and scores Block 1 for either a Source Reel or Process Tape,
    /// selecting the dedicated linter while preserving unified understanding.
    pub fn evaluate(
        file_path: &str,
        content: &str,
        is_tape: bool,
        config: &SpatialLintConfig,
        _report: &mut LintReport,
    ) -> MetadataScore {
        let blocks = crate::diagnostics::linter::structure::detector::BlockDetector::detect_blocks(content, is_tape);
        if is_tape {
            TapeHeadLinter::evaluate(file_path, &blocks.metadata.content, config)
        } else {
            ReelMetadataLinter::evaluate(file_path, &blocks.metadata.content, config)
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Unit Tests & Calibration Verification )
// ==============================================================================

#[cfg(test)]
mod tests {

    #[test]
    fn test_character_omni_block1_scores_100() {
        let logger = crate::diagnostics::logger::DiagnosticLogger::new(false);
        let linter = crate::diagnostics::linter::OmniLinter::new(logger);
        let path = crate::core::filetype::resolve_track_path("lexicon/01_nouns/ideas/types/character.omni");
        if path.exists() {
            let report = linter.lint_file(&path).expect("Lint character.omni");
            assert!(report.is_valid, "character.omni must be valid");
            let score = report.metadata_score.expect("Metadata score must be present");
            assert_eq!(
                score.total, 100,
                "character.omni Block 1 must score 100/100, got: {} ({:?})",
                score.total, score.breakdown
            );
            assert_eq!(score.rating, "A+ (Exemplary Invariant)");
            assert_eq!(score.identity_score, 20);
            assert_eq!(score.provenance_score, 25);
            assert_eq!(score.witness_score, 20);
            assert_eq!(score.ontology_score, 20);
            assert_eq!(score.spatial_hygiene_score, 15);
        }
    }

    #[test]
    fn test_character_tape_block1_scores_100() {
        let logger = crate::diagnostics::logger::DiagnosticLogger::new(false);
        let linter = crate::diagnostics::linter::OmniLinter::new(logger);
        let path = crate::core::filetype::resolve_track_path("lexicon/01_nouns/ideas/types/character.tape");
        if path.exists() {
            let report = linter.lint_file(&path).expect("Lint character.tape");
            assert!(report.is_valid, "character.tape must be valid");
            let score = report.metadata_score.expect("Metadata score must be present");
            assert_eq!(
                score.total, 100,
                "character.tape Block 1 must score 100/100, got: {} ({:?})",
                score.total, score.breakdown
            );
            assert_eq!(score.rating, "A+ (Exemplary Invariant)");
            assert_eq!(score.identity_score, 20);
            assert_eq!(score.provenance_score, 25);
            assert_eq!(score.witness_score, 20);
            assert_eq!(score.ontology_score, 20);
            assert_eq!(score.spatial_hygiene_score, 15);
        }
    }
}

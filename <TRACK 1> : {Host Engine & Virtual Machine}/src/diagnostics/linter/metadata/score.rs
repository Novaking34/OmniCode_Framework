// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Block 1 Metadata & Tape Head Score Model & Rating Taxonomy )
// ==============================================================================
//! OmniCode Metadata & Tape Head Scoring Taxonomy
//!
//! Provides the unified scoring model for Block 1 across both `.omni` Source Reels
//! and `.tape` Process Ribbons. While the concrete syntax of reels and tapes differs,
//! both express the exact same ontological foundation and covenant understanding:
//! 1. Identity & Formal Declaration (max 20 pts)
//! 2. Provenance & Operational Environment (max 25 pts)
//! 3. Covenant & Scripture Invariant Grounding (max 20 pts)
//! 4. Ontological Invariants / Cardinal Coordinates (max 20 pts)
//! 5. Spatial Geometry & Naming Hygiene (max 15 pts)

/// Comprehensive rubric breakdown and total score for Block 1 (0..=100).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataScore {
    /// Score for formal identity and module/tape declaration (max 20).
    pub identity_score: u32,
    /// Score for provenance, author, version, and environment (max 25).
    pub provenance_score: u32,
    /// Score for scripture/covenant invariant witness grounding (max 20).
    pub witness_score: u32,
    /// Score for ontological dimensions or 6 cardinal clauses (max 20).
    pub ontology_score: u32,
    /// Score for spatial geometry and delimiter naming hygiene (max 15).
    pub spatial_hygiene_score: u32,
    /// Total aggregate score (0..=100).
    pub total: u32,
    /// Letter grade and descriptive rating.
    pub rating: &'static str,
    /// Granular audit notes explaining points awarded or deducted.
    pub breakdown: Vec<String>,
}

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Rating Computation & Rubric Evaluation )
// ==============================================================================

impl MetadataScore {
    /// Computes the letter grade and descriptive status from the total score.
    pub fn compute_rating(total: u32) -> &'static str {
        match total {
            100 => "A+ (Exemplary Invariant)",
            90..=99 => "A (Compliant)",
            80..=89 => "B (Adequate Foundation)",
            70..=79 => "C (Needs Provenance / Witness Grounding)",
            50..=69 => "D (Deficient Metadata)",
            _ => "F (Unanchored / Incomplete)",
        }
    }
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Subsystem Summary & Aggregation Methods )
// ==============================================================================

impl MetadataScore {
    /// Creates a new MetadataScore from component scores and breakdown notes.
    pub fn new(
        identity: u32,
        provenance: u32,
        witness: u32,
        ontology: u32,
        spatial: u32,
        breakdown: Vec<String>,
    ) -> Self {
        let total = identity + provenance + witness + ontology + spatial;
        Self {
            identity_score: identity,
            provenance_score: provenance,
            witness_score: witness,
            ontology_score: ontology,
            spatial_hygiene_score: spatial,
            total,
            rating: Self::compute_rating(total),
            breakdown,
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests for MetadataScore Rating Taxonomy )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_score_rating_tiers() {
        assert_eq!(MetadataScore::compute_rating(100), "A+ (Exemplary Invariant)");
        assert_eq!(MetadataScore::compute_rating(95), "A (Compliant)");
        assert_eq!(MetadataScore::compute_rating(85), "B (Adequate Foundation)");
        assert_eq!(MetadataScore::compute_rating(75), "C (Needs Provenance / Witness Grounding)");
        assert_eq!(MetadataScore::compute_rating(60), "D (Deficient Metadata)");
        assert_eq!(MetadataScore::compute_rating(30), "F (Unanchored / Incomplete)");
    }
}

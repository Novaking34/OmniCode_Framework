// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Universal 4-Block File Audit Score & QuadState Gate Taxonomy )
// ==============================================================================
//! # Universal 4-Block File Audit Score & QuadState Compilation Gate
//!
//! Synthesizes the four block-level invariant scores into a single composite
//! metric on the continuous balanced scale $[-100 \dots +100]$, and maps the result
//! to an operational QuadState compilation gate:
//! - `+1 : PROCEED` (Composite >= +70): Eligible to compile and lower to machine words.
//! - ` 0 : AWAIT`   (Composite +50..+70): Ambient equilibrium hold pending advisories.
//! - `-1 : ROLLBACK` (Composite +20..+50): Precondition/covenant deficit; causal tape rolls back.
//! - `HALT : NULL`  (Composite < +20): Ineligible to compile; execution stops.
//!
//! # Scripture Witness:
//! - **Proverbs 11:1 (KJV)**:
//!   "A false balance is abomination to the LORD: but a just weight is his delight."
//!   (The balanced audit metric ensures mathematical justice across all 4 structural blocks).
//! - **Leviticus 19:36 (KJV)**:
//!   "Just balances, just weights, a just ephah, and a just hin, shall ye have..."
//!   (Every block must carry honest, verified weight before entering execution).
// ==============================================================================

use super::metadata::MetadataScore;
use super::opening::OpeningScore;
use super::body::BodyScore;
use super::closing::ClosingScore;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( QuadStateGate Enumeration & FileAuditScore Data Structures )
// ==============================================================================

/// QuadState compilation gate denoting compilation eligibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuadStateGate {
    Proceed,
    Await,
    Rollback,
    Halt,
}

impl QuadStateGate {
    /// Human-readable descriptor for the compilation gate.
    pub fn name(&self) -> &'static str {
        match self {
            QuadStateGate::Proceed => "+1 : PROCEED (Eligible to Compile)",
            QuadStateGate::Await => " 0 : AWAIT (Advisory Equilibrium Hold)",
            QuadStateGate::Rollback => "-1 : ROLLBACK (Invariant Reversion)",
            QuadStateGate::Halt => "HALT : NULL (Compilation Disqualified)",
        }
    }

    /// Returns `true` if the compilation gate permits advancing forward to compilation.
    pub fn is_proceed(&self) -> bool {
        matches!(self, QuadStateGate::Proceed)
    }
}

/// Composite 4-Block audit score synthesizing invariant health across all four blocks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileAuditScore {
    pub block1_metadata: MetadataScore,
    pub block2_opening: OpeningScore,
    pub block3_body: BodyScore,
    pub block4_closing: ClosingScore,
    pub composite_score: i32,
    pub gate: QuadStateGate,
    pub eligible_to_compile: bool,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Metric Calculation, Clamping, and Gate Resolution )
// ==============================================================================

impl FileAuditScore {
    /// Computes the composite 4-Block audit score and resolves the QuadState gate.
    pub fn calculate(
        block1: MetadataScore,
        block2: OpeningScore,
        block3: BodyScore,
        block4: ClosingScore,
        has_fatal_errors: bool,
    ) -> Self {
        let b1 = block1.total as i32;
        let b2 = block2.total;
        let b3 = block3.total;
        let b4 = block4.total;

        let composite = ((b1 + b2 + b3 + b4) / 4).clamp(-100, 100);

        let gate = if has_fatal_errors || composite < 20 {
            QuadStateGate::Halt
        } else if composite >= 70 {
            QuadStateGate::Proceed
        } else if composite >= 50 {
            QuadStateGate::Await
        } else {
            QuadStateGate::Rollback
        };

        let eligible_to_compile = gate.is_proceed() && !has_fatal_errors;

        Self {
            block1_metadata: block1,
            block2_opening: block2,
            block3_body: block3,
            block4_closing: block4,
            composite_score: composite,
            gate,
            eligible_to_compile,
        }
    }

    /// Formats a human-readable telemetry summary for the audit scroll and logger.
    pub fn format_telemetry(&self) -> String {
        format!(
            "Audit Score: {}/100 => Gate: {} (B1: {}, B2: {}, B3: {}, B4: {})",
            self.composite_score,
            self.gate.name(),
            self.block1_metadata.total,
            self.block2_opening.total,
            self.block3_body.total,
            self.block4_closing.total
        )
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Unit Tests & Mathematical Verification )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_score_perfect_composite_proceed() {
        let b1 = MetadataScore::new(20, 25, 20, 20, 15, vec!["OK".into()]);
        let b2 = OpeningScore::new(25, 20, 20, 20, 15, vec!["OK".into()]);
        let b3 = BodyScore::new(25, 20, 20, 20, 15, vec!["OK".into()]);
        let b4 = ClosingScore::new(20, 20, 20, 25, 15, vec!["OK".into()]);

        let audit = FileAuditScore::calculate(b1, b2, b3, b4, false);
        assert_eq!(audit.composite_score, 100);
        assert_eq!(audit.gate, QuadStateGate::Proceed);
        assert!(audit.eligible_to_compile);
        assert!(audit.format_telemetry().contains("100/100"));
    }

    #[test]
    fn test_audit_score_fatal_error_forces_halt() {
        let b1 = MetadataScore::new(20, 25, 20, 20, 15, vec![]);
        let b2 = OpeningScore::new(25, 20, 20, 20, 15, vec![]);
        let b3 = BodyScore::new(25, 20, 20, 20, 15, vec![]);
        let b4 = ClosingScore::new(20, 20, 20, 25, 15, vec![]);

        // Even with 100% composite score, fatal error triggers HALT
        let audit = FileAuditScore::calculate(b1, b2, b3, b4, true);
        assert_eq!(audit.composite_score, 100);
        assert_eq!(audit.gate, QuadStateGate::Halt);
        assert!(!audit.eligible_to_compile);
    }

    #[test]
    fn test_audit_score_tiers() {
        let b1_low = MetadataScore::new(5, 5, 5, 5, 5, vec![]);
        let b2_low = OpeningScore::new(5, 5, 5, 5, 5, vec![]);
        let b3_low = BodyScore::new(5, 5, 5, 5, 5, vec![]);
        let b4_low = ClosingScore::new(5, 5, 5, 5, 5, vec![]);

        let audit = FileAuditScore::calculate(b1_low, b2_low, b3_low, b4_low, false);
        assert_eq!(audit.composite_score, 25);
        assert_eq!(audit.gate, QuadStateGate::Rollback);
        assert!(!audit.eligible_to_compile);
    }
}

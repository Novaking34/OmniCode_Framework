// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Universal 4-Block File Audit Score & QuadState Gate Taxonomy )
// ==============================================================================
//! Universal 4-Block File Audit Score & QuadState Compilation Gate
//!
//! Synthesizes the four block-level invariant scores into a single composite
//! metric on the continuous balanced scale $[-100 \dots +100]$, and maps the result
//! to an operational QuadState compilation gate:
//! - `+1 : PROCEED` (Composite >= +70): Eligible to compile and lower to machine words.
//! - ` 0 : AWAIT`   (Composite +50..+70): Ambient equilibrium hold pending advisories.
//! - `-1 : ROLLBACK` (Composite +20..+50): Precondition/covenant deficit; causal tape rolls back.
//! - `HALT : NULL`  (Composite < +20): Ineligible to compile; execution stops.

use super::metadata::MetadataScore;
use super::opening::OpeningScore;
use super::body::BodyScore;
use super::closing::ClosingScore;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuadStateGate {
    Proceed,
    Await,
    Rollback,
    Halt,
}

impl QuadStateGate {
    pub fn name(&self) -> &'static str {
        match self {
            QuadStateGate::Proceed => "+1 : PROCEED (Eligible to Compile)",
            QuadStateGate::Await => " 0 : AWAIT (Advisory Equilibrium Hold)",
            QuadStateGate::Rollback => "-1 : ROLLBACK (Invariant Reversion)",
            QuadStateGate::Halt => "HALT : NULL (Compilation Disqualified)",
        }
    }

    pub fn is_proceed(&self) -> bool {
        matches!(self, QuadStateGate::Proceed)
    }
}

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
}

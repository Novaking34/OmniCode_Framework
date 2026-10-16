// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Machine-Applicable Automated Repair Kinds & Concrete Fix Suggestions )
// ==============================================================================
//! OmniCode Diagnostics: Concrete Automated Remediation Telemetry
//!
//! Encapsulates machine-applicable patch descriptions allowing the
//! Restorator or AI coding agents to heal broken source files in place.

/// Categories of automated restorative repairs executed by Restorator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FixKind {
    /// Append a missing closing boundary delimiter (e.g. closing `[` with `]`).
    CloseBoundary,
    /// Substitute erroneous closing delimiter with matching opening partner.
    FixBoundaryMismatch,
    /// Inject missing canonical 4-Block architecture scaffolding headers.
    AddBlockHeader,
    /// Automatically inject missing core library imports (Core.Arithmetic, etc).
    AddImport,
    /// Synthesize and declare an inferred conceptual noun entity.
    DeclareEntityWithInfer,
    /// Modify or satisfy an adversative caveat expression.
    AdjustCaveat,
    /// Convert an immutable Rank 1 invariant `< ... >` into mutable `[ ... ]`.
    ConvertImmutableToMutable,
}

/// Concrete remediation telemetry attached to a failpoint event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixSuggestion {
    /// Categorical classification of the automated remediation.
    pub kind: FixKind,
    /// 1-based source line number where remediation should be applied.
    pub line: usize,
    /// 1-based source column number where the target token resides.
    pub col: usize,
    /// The erroneous or missing token sequence targeted for replacement.
    pub target_token: String,
    /// The proposed patch or substitution string that seals the invariant.
    pub suggested_patch: String,
    /// Human-readable and AI-interpretable rationale explaining the fix.
    pub rationale: String,
}

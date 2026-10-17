// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Machine-Applicable Automated Repair Kinds & Concrete Fix Suggestions )
// ==============================================================================
//! OmniCode Diagnostics: Concrete Automated Remediation Telemetry
//!
//! Encapsulates machine-applicable patch descriptions allowing the
//! Restorator, OmniDebugger, or AI coding agents to heal broken source files
//! and restore system equilibrium without human intervention.
//!
//! # Remediation Invariants
//! 1. Every fix suggestion specifies exact 1-based source coordinates (`line`, `col`).
//! 2. Candidate repairs must be verifiable by executable test oracles before commit.
//! 3. Hardware / HALT remediations discharge saturated dual rails back to `(0, 0)` Await.

use std::fmt;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Categories of Automated Repair & Structured Suggestion Type )
// ==============================================================================

/// Categories of automated restorative repairs executed by Restorator or Runtime.
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
    /// Re-incept machine state by cascading down to a verified landmark upon HALT.
    ReInceptToLandmark,
    /// Discharge saturated dual-rail `(1, 1)` state back to `(0, 0)` homeostasis (Await).
    ResetDualRailSaturation,
    /// Restore balanced ternary ALU symmetry by zeroing leaked register bits.
    RebalanceAlu,
}

/// Concrete remediation telemetry attached to a failpoint event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixSuggestion {
    /// Categorical classification of the automated remediation.
    pub kind: FixKind,
    /// 1-based source line number where remediation should be applied (0 if hardware).
    pub line: usize,
    /// 1-based source column number where the target token resides (0 if hardware).
    pub col: usize,
    /// The erroneous or missing token sequence targeted for replacement.
    pub target_token: String,
    /// The proposed patch or substitution string that seals the invariant.
    pub suggested_patch: String,
    /// Human-readable and AI-interpretable rationale explaining the fix.
    pub rationale: String,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Constructors, Patch Preview Formatters & Remediation Introspection )
// ==============================================================================

impl FixSuggestion {
    /// Constructs a new [`FixSuggestion`].
    pub fn new(
        kind: FixKind,
        line: usize,
        col: usize,
        target_token: impl Into<String>,
        suggested_patch: impl Into<String>,
        rationale: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            line,
            col,
            target_token: target_token.into(),
            suggested_patch: suggested_patch.into(),
            rationale: rationale.into(),
        }
    }

    /// Creates a hardware re-inception fix suggestion for an abrupt HALT.
    pub fn hardware_reinception(landmark_address: u64, rationale: impl Into<String>) -> Self {
        Self {
            kind: FixKind::ReInceptToLandmark,
            line: 0,
            col: 0,
            target_token: "HALT_SATURATION".to_string(),
            suggested_patch: format!("REINCEPT_TO_LANDMARK_{:#08x}", landmark_address),
            rationale: rationale.into(),
        }
    }

    /// Formats a clean patch preview showing target token vs. suggested patch.
    pub fn format_patch_preview(&self) -> String {
        format!(
            "[{:?}] @ {}:{} Target: {:?} => Patch: {:?}",
            self.kind, self.line, self.col, self.target_token, self.suggested_patch
        )
    }

    /// Returns `true` if this suggestion targets hardware or HALT recovery.
    pub fn is_hardware_remediation(&self) -> bool {
        matches!(
            self.kind,
            FixKind::ReInceptToLandmark
                | FixKind::ResetDualRailSaturation
                | FixKind::RebalanceAlu
        )
    }
}

impl FixKind {
    /// Returns the uppercase canonical string identifier for this fix kind.
    pub fn as_str(&self) -> &'static str {
        match self {
            FixKind::CloseBoundary => "FIX_CLOSE_BOUNDARY",
            FixKind::FixBoundaryMismatch => "FIX_BOUNDARY_MISMATCH",
            FixKind::AddBlockHeader => "FIX_ADD_BLOCK_HEADER",
            FixKind::AddImport => "FIX_ADD_IMPORT",
            FixKind::DeclareEntityWithInfer => "FIX_DECLARE_ENTITY_INFER",
            FixKind::AdjustCaveat => "FIX_ADJUST_CAVEAT",
            FixKind::ConvertImmutableToMutable => "FIX_CONVERT_IMMUTABLE_MUTABLE",
            FixKind::ReInceptToLandmark => "FIX_REINCEPT_TO_LANDMARK",
            FixKind::ResetDualRailSaturation => "FIX_RESET_DUAL_RAIL_SATURATION",
            FixKind::RebalanceAlu => "FIX_REBALANCE_ALU",
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Display Trait Implementation & Subsystem Sealing )
// ==============================================================================

impl fmt::Display for FixSuggestion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at {}:{} ('{}' => '{}') [{}]",
            self.kind.as_str(),
            self.line,
            self.col,
            self.target_token,
            self.suggested_patch,
            self.rationale
        )
    }
}

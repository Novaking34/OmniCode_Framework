// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Failpoint Pathology Categorization & Invariant Anomaly Enums )
// ==============================================================================
//! OmniCode Diagnostics: Failpoint Pathology Taxonomy
//!
//! Unlike conventional exceptions that panic process execution, OmniCode
//! treats failpoints as first-class diagnostic signals that guide rollback,
//! caveat evaluation, or autonomous immune restoration via the Restorator.

/// Classifications of trapped failpoints and boundary anomalies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FailpointType {
    /// Delimiter mismatch: opened with one delimiter class, closed with another.
    BoundaryMismatch,
    /// Unclosed boundary delimiter reaching EOF without matching partner.
    BoundaryLeak,
    /// Execution attempted to evaluate void/null state without resolving caveat.
    NullTrap,
    /// An operation or caveat evaluation triggered state rollback (-1:ROLLBACK).
    RollbackTriggered,
    /// An adversative caveat condition (! [ ... ]) vetoed active state.
    CaveatVeto,
    /// Illegal attempt to mutate an immutable Rank 1 invariant axiom < ... >.
    ImmutableMutation,
    /// Recursive or cyclic retroaction loop exceeding debugger threshold.
    CyclicRetroaction,
    /// Structural or syntactic grammar anomaly violating language invariants.
    SyntaxAnomaly,
    /// Hardware or firmware invariant breach detected during cold boot or MMIO operation.
    HardwareFault,
}

impl FailpointType {
    /// Returns the uppercase canonical string identifier for this failpoint.
    pub fn as_str(&self) -> &'static str {
        match self {
            FailpointType::BoundaryMismatch => "FP_BOUNDARY_MISMATCH",
            FailpointType::BoundaryLeak => "FP_BOUNDARY_LEAK",
            FailpointType::NullTrap => "FP_NULL_TRAP",
            FailpointType::RollbackTriggered => "FP_ROLLBACK_TRIGGERED",
            FailpointType::CaveatVeto => "FP_CAVEAT_VETO",
            FailpointType::ImmutableMutation => "FP_IMMUTABLE_MUTATION",
            FailpointType::CyclicRetroaction => "FP_CYCLIC_RETROACTION",
            FailpointType::SyntaxAnomaly => "FP_SYNTAX_ANOMALY",
            FailpointType::HardwareFault => "FP_HARDWARE_FAULT",
        }
    }

    /// Compact single-bit representation for game-dev flight recorder masks.
    pub fn bitmask(&self) -> u32 {
        match self {
            FailpointType::BoundaryMismatch => 1 << 0,
            FailpointType::BoundaryLeak => 1 << 1,
            FailpointType::NullTrap => 1 << 2,
            FailpointType::RollbackTriggered => 1 << 3,
            FailpointType::CaveatVeto => 1 << 4,
            FailpointType::ImmutableMutation => 1 << 5,
            FailpointType::CyclicRetroaction => 1 << 6,
            FailpointType::SyntaxAnomaly => 1 << 7,
            FailpointType::HardwareFault => 1 << 8,
        }
    }
}

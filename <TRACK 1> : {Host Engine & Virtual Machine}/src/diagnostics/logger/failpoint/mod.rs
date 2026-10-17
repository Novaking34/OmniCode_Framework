// =============================================================================
// BLOCK 1: METADATA BLOCK
// ( Failpoint Subsystem Identity, Module Organization & Re-exports )
// =============================================================================
//! OmniCode Diagnostics: Failpoint Pathology, HALT Telemetry & Remediation
//!
//! Decomposed into specialized machine organs:
//! - [`types`]: FailpointType, FailpointGroup, and Subgroup taxonomy.
//! - [`halt`]: HALT physics, dual-rail saturation `(1, 1)`, and Track 0 dumps.
//! - [`remediation`]: Machine-applicable FixSuggestion and FixKind descriptors.

pub mod types;
pub mod halt;
pub mod remediation;

// =============================================================================
// BLOCK 2: OPENING BLOCK
// ( Re-export Declarations for Failpoints & Telemetry )
// =============================================================================
pub use types::{FailpointGroup, FailpointSubgroup, FailpointType};
pub use halt::{HaltDumpFrame, HaltReason, ReInceptionTarget, SiliconRegisters};
pub use remediation::{FixKind, FixSuggestion};

// =============================================================================
// BLOCK 3: BODY BLOCK
// ( Convenience Helpers for Failpoint & HALT Classification )
// =============================================================================
/// Returns `true` if a dual-rail hardware code represents the saturated
/// HALT state (11).
#[inline]
pub fn is_dual_rail_halt(code: u8) -> bool {
    (code & 0b11) == 0b11
}

/// Creates a standardized [`HaltDumpFrame`] capturing an abrupt HALT event
/// and targeting immediate re-inception to a safe landmark.
pub fn create_halt_dump(
    tick_micros: u128,
    reason: HaltReason,
    failpoint: FailpointType,
    landmark_address: u64,
    registers: SiliconRegisters,
    re_inception_target: ReInceptionTarget,
    restart_count: usize,
) -> HaltDumpFrame {
    HaltDumpFrame::new(
        tick_micros,
        reason,
        failpoint,
        landmark_address,
        registers,
        re_inception_target,
        restart_count,
    )
}

// =============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Module Sealing & Invariant Verification )
// =============================================================================
// Failpoint pathology subsystem verified and sealed.

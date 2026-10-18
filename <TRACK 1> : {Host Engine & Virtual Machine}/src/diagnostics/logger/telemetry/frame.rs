// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( RollFrame Telemetry Struct: Deterministic Game-Dev State Vector )
// ==============================================================================
//! OmniCode Telemetry: Discrete Execution RollFrame Vectors
//!
//! # Architectural Scope & Purpose:
//! Encapsulates discrete tick-by-tick state vectors captured during process
//! tape stepping, recording dual-rail SIMD bitplanes, operational roll verbs,
//! QuadStates, and compact failpoint bitmasks:
//! - [`RollFrame`]: Single atomic execution frame representing one tape roll tick.
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**:
//!   "And he gave unto Moses, when he had made an end of communing with him upon mount Sinai,
//!    two tables of testimony, tables of stone, written with the finger of God."
//!   (The immutable inscription of atomic execution frames on physical stone).
//! - **Proverbs 4:26 (KJV)**:
//!   "Ponder the path of thy feet, and let all thy ways be established."
//!   (The discipline of measuring every execution step and recording state transitions).
//! - **1 Corinthians 14:40 (KJV)**:
//!   "Let all things be done decently and in order."
//!   (The chronological sequential ordering of discrete execution frames).
//!
//! # Why Extensive Commenting Matters for AI Agents & Developers:
//! 1. **Game-Dev Roll Architecture**: OmniCode programs execute as ribbons of discrete rolls.
//!    Each roll commits an atomic state transition that can be inspected, stepped backward, or replayed.
//! 2. **Dual-Rail Substrate**: Input and output bitplanes are captured as `(positive_rail, negative_rail)`
//!    tuples, exposing balanced ternary hardware states directly to diagnostic analysis.
//! 3. **Non-Intrusive Telemetry**: `RollFrame` captures complete operational context in a fixed-size
//!    structure suitable for zero-allocation circular ring buffering.
// ==============================================================================

use crate::core::ternary::QuadState;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( RollFrame Struct Definition & State Invariants )
// ==============================================================================

/// Single atomic execution frame captured during roll traversal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RollFrame {
    /// Monotonic elapsed microseconds at roll execution.
    pub tick_micros: u128,
    /// 1-based roll sequence index.
    pub roll_index: usize,
    /// Total rolls declared on this tape ribbon.
    pub total_rolls: usize,
    /// Active operational verb (e.g. "INSCRIBE", "TRANSDUCE", "COMPARE").
    pub verb: String,
    /// Input substrate state as (positive_rail, negative_rail) bitplanes.
    pub dual_rail_in: (u64, u64),
    /// Output substrate state as (positive_rail, negative_rail) bitplanes.
    pub dual_rail_out: (u64, u64),
    /// Fourfold execution state committed by this roll.
    pub quad_state: QuadState,
    /// Compact bitmask representing trapped failpoints (0 = clean).
    pub failpoint_mask: u32,
    /// First-class human/machine intent aside attached to the roll.
    pub intent_aside: Option<String>,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Constructor, Diagnostic Formatters & State Query Methods )
// ==============================================================================

impl RollFrame {
    /// Constructs a new [`RollFrame`].
    pub fn new(
        tick_micros: u128,
        roll_index: usize,
        total_rolls: usize,
        verb: impl Into<String>,
        dual_rail_in: (u64, u64),
        dual_rail_out: (u64, u64),
        quad_state: QuadState,
        failpoint_mask: u32,
        intent_aside: Option<String>,
    ) -> Self {
        Self {
            tick_micros,
            roll_index,
            total_rolls,
            verb: verb.into(),
            dual_rail_in,
            dual_rail_out,
            quad_state,
            failpoint_mask,
            intent_aside,
        }
    }

    /// Returns `true` if this execution frame completed without failpoints or rollback.
    #[inline]
    pub fn is_clean(&self) -> bool {
        self.quad_state == QuadState::Proceed && self.failpoint_mask == 0
    }

    /// Returns `true` if this frame experienced an anomaly, rollback, or failpoint.
    #[inline]
    pub fn has_anomaly(&self) -> bool {
        !self.is_clean()
    }

    /// Returns `true` if the roll resulted in a transactional rollback.
    #[inline]
    pub fn is_rollback(&self) -> bool {
        self.quad_state == QuadState::Rollback
    }

    /// Returns `true` if the output rails are saturated (both positive and negative rails active).
    ///
    /// In balanced ternary dual-rail logic, `(1, 1)` represents saturated state collapse (HALT).
    #[inline]
    pub fn has_dual_rail_saturation(&self) -> bool {
        (self.dual_rail_out.0 & self.dual_rail_out.1) != 0
    }

    /// Formats the frame as a compact structured telemetry row.
    pub fn format_row(&self) -> String {
        let aside_str = self
            .intent_aside
            .as_deref()
            .map(|a| format!(" ({a})"))
            .unwrap_or_default();
        format!(
            "[{:>6} µs] Roll {}/{}: {:<10} Rails: ({:#018x}, {:#018x}) -> State: {:<8} Mask: {:#010x}{}",
            self.tick_micros,
            self.roll_index,
            self.total_rolls,
            self.verb,
            self.dual_rail_out.0,
            self.dual_rail_out.1,
            self.quad_state.macro_op(),
            self.failpoint_mask,
            aside_str
        )
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Test Verification Proofs & RollFrame Invariant Tests )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll_frame_clean_execution() {
        let frame = RollFrame::new(
            142,
            1,
            10,
            "INSCRIBE",
            (0x0001, 0x0000),
            (0x0002, 0x0000),
            QuadState::Proceed,
            0,
            Some("Initialize symbol".to_string()),
        );

        assert!(frame.is_clean());
        assert!(!frame.has_anomaly());
        assert!(!frame.is_rollback());
        assert!(!frame.has_dual_rail_saturation());
        assert_eq!(frame.roll_index, 1);
        assert_eq!(frame.total_rolls, 10);
        assert_eq!(frame.verb, "INSCRIBE");

        let row = frame.format_row();
        assert!(row.contains("[   142 µs] Roll 1/10: INSCRIBE"));
        assert!(row.contains("State: PROCEED"));
        assert!(row.contains("(Initialize symbol)"));
    }

    #[test]
    fn test_roll_frame_anomaly_and_rollback() {
        let frame = RollFrame::new(
            500,
            3,
            10,
            "DIVIDE",
            (0x0005, 0x0000),
            (0x0000, 0x0000),
            QuadState::Rollback,
            0x00000004,
            Some("Division by zero trapped".to_string()),
        );

        assert!(!frame.is_clean());
        assert!(frame.has_anomaly());
        assert!(frame.is_rollback());
        assert_eq!(frame.failpoint_mask, 0x00000004);
    }

    #[test]
    fn test_dual_rail_saturation_detection() {
        // Normal rails: disjoint
        let normal = RollFrame::new(
            100, 1, 1, "OP", (1, 0), (1, 0), QuadState::Proceed, 0, None,
        );
        assert!(!normal.has_dual_rail_saturation());

        // Saturated rails: overlap in bit 0
        let saturated = RollFrame::new(
            200, 2, 2, "OP", (1, 0), (1, 1), QuadState::Halt, 0, None,
        );
        assert!(saturated.has_dual_rail_saturation());
    }
}

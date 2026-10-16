// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( RollFrame Telemetry Struct: Deterministic Game-Dev State Vector )
// ==============================================================================
//! RollFrame Structured Telemetry
//!
//! Encapsulates discrete tick-by-tick state vectors captured during process
//! tape stepping, recording dual-rail bitplanes, roll verbs, and QuadStates.

use crate::core::ternary::QuadState;

/// Single execution frame captured during roll traversal.
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

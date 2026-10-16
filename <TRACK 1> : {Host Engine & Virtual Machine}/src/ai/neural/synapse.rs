// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( Discrete Balanced-Ternary Synapse Identity & Algebraic Invariants )
// ============================================================================
//! # Discrete Ternary Synapse (`synapse.rs`)
//!
//! Models individual balanced-ternary synaptic weights in $\{-1, 0, +1\}$.
//! Governed by discrete trit physics and the fourfold [`QuadState`].

use crate::core::ternary::QuadState;

/// Discrete ternary synaptic connection weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SynapticWeight {
    /// Inhibitory connection (-1)
    Inhibitory,
    /// Neutral / zero-bias connection (0)
    Neutral,
    /// Excitatory connection (+1)
    Excitatory,
}

impl SynapticWeight {
    /// Returns the integer trit scalar representation (-1, 0, +1).
    pub fn to_trit(self) -> i32 {
        match self {
            Self::Inhibitory => -1,
            Self::Neutral => 0,
            Self::Excitatory => 1,
        }
    }

    /// Constructs a synaptic weight from a discrete trit integer.
    pub fn from_trit(trit: i32) -> Self {
        if trit > 0 {
            Self::Excitatory
        } else if trit < 0 {
            Self::Inhibitory
        } else {
            Self::Neutral
        }
    }

    /// Constructs a synaptic weight from a QuadState.
    pub fn from_quad_state(state: QuadState) -> Self {
        match state {
            QuadState::Proceed => Self::Excitatory,
            QuadState::Await | QuadState::Halt => Self::Neutral,
            QuadState::Rollback => Self::Inhibitory,
        }
    }

    /// Converts this synaptic weight to an equivalent QuadState.
    pub fn to_quad_state(self) -> QuadState {
        match self {
            Self::Excitatory => QuadState::Proceed,
            Self::Neutral => QuadState::Await,
            Self::Inhibitory => QuadState::Rollback,
        }
    }
}

// ============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests & Verification )
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synaptic_weight_conversions() {
        assert_eq!(SynapticWeight::Excitatory.to_trit(), 1);
        assert_eq!(SynapticWeight::Neutral.to_trit(), 0);
        assert_eq!(SynapticWeight::Inhibitory.to_trit(), -1);

        assert_eq!(
            SynapticWeight::from_quad_state(QuadState::Proceed),
            SynapticWeight::Excitatory
        );
        assert_eq!(
            SynapticWeight::from_quad_state(QuadState::Rollback),
            SynapticWeight::Inhibitory
        );
        assert_eq!(
            SynapticWeight::from_quad_state(QuadState::Await),
            SynapticWeight::Neutral
        );
    }
}

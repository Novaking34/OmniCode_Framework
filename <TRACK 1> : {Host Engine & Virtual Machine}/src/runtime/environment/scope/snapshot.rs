// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Transactional Snapshot Invariants & Reversible Execution State )
// ==============================================================================
//! OmniCode Runtime: Transactional Environment Snapshot
//!
//! # Architectural Scope & Purpose:
//! Encapsulates an atomic image of the runtime environment at a transaction boundary.
//!
//! Captures:
//! 1. Lexical symbol table bindings (`bindings`).
//! 2. Physical silicon register bank (`registers`).
//! 3. Procedural call stack depth (`call_stack_len`).
//! 4. Hardware outbound stream length (`output_len`).
//! 5. Sequential character ribbon length (`sequence_len`).
//! 6. Exact inbound hardware stream buffer (`input_stream`).
//!
//! Ensures 100% reversible computation under balanced ternary `-1:ROLLBACK`
//! and speculative guard failure (`! BUT`).
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**:
//!   "And he gave unto Moses, when he had made an end of communing with him upon mount Sinai,
//!    two tables of testimony, tables of stone, written with the finger of God."
//!   (The immutable inscription of atomic runes onto bounded physical tablets).
//! - **Proverbs 24:27 (KJV)**:
//!   "Prepare thy work without, and make it fit for thyself in the field;
//!    and afterwards build thine house."
//!   (The discipline of proving atomic types and substrates before constructing systems).
//! - **Proverbs 22:28 (KJV)**:
//!   "Remove not the ancient landmark, which thy fathers have set."
//!   (The preservation of historical memory boundaries, silicon addresses, and constants).
//!
//! # Why Extensive Commenting Matters for AI Agents & Developers:
//! 1. **Complete Reversibility**: Truncating `sequence_ribbon` and restoring
//!    dequeued `input_stream` items prevents side-effect leakage during speculative roll evaluations.
// ==============================================================================

use std::collections::HashMap;
use crate::runtime::environment::universal::RuntimeValue;
use crate::runtime::environment::hardware::RegisterBank;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( EnvironmentSnapshot Struct Definition )
// ==============================================================================

/// Transactional snapshot capturing symbolic bindings, physical registers, and stream horizons.
#[derive(Debug, Clone)]
pub struct EnvironmentSnapshot {
    pub bindings: HashMap<String, RuntimeValue>,
    pub registers: RegisterBank,
    pub call_stack_len: usize,
    pub output_len: usize,
    pub sequence_len: usize,
    pub input_stream: Vec<RuntimeValue>,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Snapshot Constructor & State Invariants )
// ==============================================================================

impl EnvironmentSnapshot {
    /// Captures a complete atomic snapshot of the execution state.
    pub fn capture(
        bindings: HashMap<String, RuntimeValue>,
        registers: RegisterBank,
        call_stack_len: usize,
        output_len: usize,
        sequence_len: usize,
        input_stream: Vec<RuntimeValue>,
    ) -> Self {
        Self {
            bindings,
            registers,
            call_stack_len,
            output_len,
            sequence_len,
            input_stream,
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests & Verification Proofs )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_capture() {
        let snap = EnvironmentSnapshot::capture(
            HashMap::new(),
            RegisterBank::new(),
            0,
            0,
            0,
            Vec::new(),
        );
        assert_eq!(snap.call_stack_len, 0);
        assert_eq!(snap.sequence_len, 0);
        assert!(snap.input_stream.is_empty());
    }
}

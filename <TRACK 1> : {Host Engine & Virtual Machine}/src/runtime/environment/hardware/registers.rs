// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Physical Silicon Register Bank & Canonical Hardware Target Ports )
// ==============================================================================
//! OmniCode Runtime: Silicon Register Substrate & Hardware Ports
//!
//! # Architectural Scope & Purpose:
//! Implements the 13 Named Physical Silicon Registers of the OmniCode Host Engine:
//! - `%head`    (0): Tape Head / Instruction Pointer
//! - `%home`    (1): Base Pointer / Frame Origin
//! - `%spool`   (2): Stack Spool / Activation Record Floor
//! - `%it`      (3): Primary Accumulator / Result
//! - `%self`    (4): Current Instance / Context Entity
//! - `%caller`  (5): Invoker Context Entity
//! - `%other`   (6): Peer / Auxiliary Entity
//! - `%state`   (7): Active QuadState Execution Status
//! - `%turn`    (8): Temporal Cadence / Cycle Counter
//! - `%bound`   (9): Watchdog Horizon / Recursion Limit
//! - `%from`   (10): Inbound Hardware Port / Input Stream
//! - `%into`   (11): Outbound Hardware Port / Output Stream
//! - `%discard`(12): Bit-Bucket / Null Sink (writes are safely dropped)
//!
//! Also defines canonical memory-mapped device target addresses (`OMNI_TARGET_*`).
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
//! 1. **Silicon Parity**: Autonomous coding agents must respect that `%discard`
//!    is an immutable sink; writes to it MUST NEVER mutate state or cause panics.
//! 2. **Physical Addresses**: Canonical constants ensure hardware device ports
//!    remain stable across host architecture revisions.
// ==============================================================================

use crate::core::ternary::QuadState;
use crate::runtime::assembler::MachineRegister;
use crate::runtime::environment::universal::RuntimeValue;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Canonical Hardware Addresses & RegisterBank Definition )
// ==============================================================================

/// Canonical Hardware Target Addresses
pub const OMNI_TARGET_STDOUT: u64 = 0x01;
pub const OMNI_TARGET_VGA: u64 = 0x000B8000;
pub const OMNI_TARGET_COM1: u64 = 0x03F8;

/// Physical Silicon Register Bank hosting the 13 Named Machine Registers.
///
/// Implements immediate hardware cell reading, mutation, and discard-sink semantics.
#[derive(Debug, Clone, PartialEq)]
pub struct RegisterBank {
    pub cells: [RuntimeValue; 13],
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Register Accessors, Mutation, Discard-Sink Semantics & Name Lookup )
// ==============================================================================

impl RegisterBank {
    /// Initializes the silicon register bank to pristine hardware baselines.
    pub fn new() -> Self {
        Self {
            cells: [
                RuntimeValue::Number(0.0),                  // 0: HEAD (Tape Head / IP)
                RuntimeValue::Number(0.0),                  // 1: HOME (Base Pointer / Origin)
                RuntimeValue::Number(0.0),                  // 2: SPOOL (Stack Spool / Floor)
                RuntimeValue::State(QuadState::Proceed),    // 3: IT (Accumulator / Result)
                RuntimeValue::Text("Kernel".to_string()),   // 4: SELF (Instance / Context)
                RuntimeValue::Text("Root".to_string()),     // 5: CALLER (Invoker Context)
                RuntimeValue::Text("Swarm".to_string()),    // 6: OTHER (Peer / Auxiliary)
                RuntimeValue::State(QuadState::Proceed),    // 7: STATE (QuadState Status)
                RuntimeValue::Number(0.0),                  // 8: TURN (Temporal Cadence / Cycle)
                RuntimeValue::Number(0.0),                  // 9: BOUND (Watchdog Limit / Horizon)
                RuntimeValue::Text(String::new()),          // 10: FROM (Input Channel / Stdin)
                RuntimeValue::Text(String::new()),          // 11: INTO (Output Channel / Stdout)
                RuntimeValue::State(QuadState::Proceed),    // 12: DISCARD (Bit-Bucket / Null Sink)
            ],
        }
    }

    /// Reads a register cell by its physical `MachineRegister` enum.
    pub fn read(&self, reg: MachineRegister) -> RuntimeValue {
        self.cells[reg.index() as usize].clone()
    }

    /// Writes a value to a register cell. Writes to `DISCARD` are safely dropped.
    pub fn write(&mut self, reg: MachineRegister, val: RuntimeValue) {
        if reg == MachineRegister::DISCARD {
            return;
        }
        self.cells[reg.index() as usize] = val;
    }

    /// Reads a register by name if recognized, returning `None` otherwise.
    pub fn read_by_name(&self, name: &str) -> Option<RuntimeValue> {
        MachineRegister::from_name(name).map(|r| self.read(r))
    }

    /// Writes to a register by name. Returns true if name was a recognized register.
    pub fn write_by_name(&mut self, name: &str, val: RuntimeValue) -> bool {
        if let Some(reg) = MachineRegister::from_name(name) {
            self.write(reg, val);
            true
        } else {
            false
        }
    }

    /// Resets all registers to pristine hardware baseline.
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Default Trait Implementation & Unit Tests )
// ==============================================================================

impl Default for RegisterBank {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_bank_initial_state() {
        let bank = RegisterBank::new();
        assert_eq!(bank.read(MachineRegister::HEAD), RuntimeValue::Number(0.0));
        assert_eq!(bank.read(MachineRegister::IT), RuntimeValue::State(QuadState::Proceed));
        assert_eq!(bank.read(MachineRegister::SELF), RuntimeValue::Text("Kernel".to_string()));
    }

    #[test]
    fn test_register_discard_sink_invariance() {
        let mut bank = RegisterBank::new();
        bank.write(MachineRegister::DISCARD, RuntimeValue::Number(9999.0));
        assert_eq!(bank.read(MachineRegister::DISCARD), RuntimeValue::State(QuadState::Proceed));
    }

    #[test]
    fn test_register_read_write_by_name() {
        let mut bank = RegisterBank::new();
        assert!(bank.write_by_name("IT", RuntimeValue::Number(42.0)));
        assert_eq!(bank.read_by_name("IT"), Some(RuntimeValue::Number(42.0)));
        assert!(!bank.write_by_name("NON_EXISTENT_REGISTER", RuntimeValue::Number(1.0)));
    }
}

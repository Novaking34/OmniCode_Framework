// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Runtime Environment State, Transactional Scopes & Symbol Table Invariants )
// ==============================================================================
//! OmniCode Runtime: Transactional Execution Environment
//!
//! # Architectural Scope & Purpose:
//! The `Environment` orchestrates scoped ontological symbol tables with native snapshot,
//! transaction nesting, and zero-cost rollback capabilities to support balanced
//! ternary -1 (Rollback) state transitions.
//!
//! In addition to lexical variable bindings, the Environment coordinates the physical
//! Silicon Register Bank (the 13 Named Machine Registers), procedural call frames,
//! hardware transducer ports (`FROM`, `INTO`), continuous text ribbons (`sequence_ribbon`),
//! and state-prime symbol resolution.
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
//! 1. **Epistemic Continuity**: Autonomous coding agents require explicit rationale
//!    for why physical registers are synchronized with symbol bindings and why
//!    state-primes (`Character'`) automatically resolve to base entities (`Character`).
//! 2. **Transactional Soundness**: Full restoration of `bindings`, `registers`, `call_stack`,
//!    `output_stream`, `sequence_ribbon`, and `input_stream` ensures zero state leakage
//!    during reversible execution branches.
// ==============================================================================

use std::collections::HashMap;
use crate::runtime::assembler::MachineRegister;
use crate::runtime::environment::universal::RuntimeValue;
use crate::runtime::environment::hardware::{
    RegisterBank,
    OMNI_TARGET_STDOUT,
};
use crate::runtime::environment::scope::frame::CallFrame;
use crate::runtime::environment::scope::snapshot::EnvironmentSnapshot;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Environment Struct Definition & Initialization )
// ==============================================================================

/// Transactional Runtime Environment managing symbol tables, registers, and execution streams.
#[derive(Debug, Clone)]
pub struct Environment {
    pub registers: RegisterBank,
    bindings: HashMap<String, RuntimeValue>,
    call_stack: Vec<CallFrame>,
    pub output_stream: Vec<RuntimeValue>,
    pub input_stream: Vec<RuntimeValue>,
    pub target_device: u64,
    pub sequence_ribbon: Vec<RuntimeValue>,
    snapshots: Vec<EnvironmentSnapshot>,
}

impl Environment {
    /// Creates a pristine runtime environment with default hardware registers.
    pub fn new() -> Self {
        Self {
            registers: RegisterBank::new(),
            bindings: HashMap::new(),
            call_stack: Vec::new(),
            output_stream: Vec::new(),
            input_stream: Vec::new(),
            target_device: OMNI_TARGET_STDOUT,
            sequence_ribbon: Vec::new(),
            snapshots: Vec::new(),
        }
    }
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Transactional Operations, Binding Mutations, Call Stacks & Transducer I/O )
// ==============================================================================

impl Environment {
    /// Takes an atomic transactional snapshot of current bindings, registers, and streams.
    pub fn begin_transaction(&mut self) {
        self.snapshots.push(EnvironmentSnapshot::capture(
            self.bindings.clone(),
            self.registers.clone(),
            self.call_stack.len(),
            self.output_stream.len(),
            self.sequence_ribbon.len(),
            self.input_stream.clone(),
        ));
    }

    /// Commits the active transaction, discarding the restore snapshot.
    pub fn commit_transaction(&mut self) {
        self.snapshots.pop();
    }

    /// Unwinds and restores the environment state completely to the last snapshot.
    pub fn rollback_transaction(&mut self) -> Result<(), String> {
        if let Some(prev) = self.snapshots.pop() {
            self.bindings = prev.bindings;
            self.registers = prev.registers;
            self.call_stack.truncate(prev.call_stack_len);
            self.output_stream.truncate(prev.output_len);
            self.sequence_ribbon.truncate(prev.sequence_len);
            self.input_stream = prev.input_stream;
            Ok(())
        } else {
            Err("FAILPOINT [RollbackUnderflow]: Attempted rollback with no active transaction snapshot".into())
        }
    }

    /// Returns a reference to the active symbol bindings table.
    pub fn bindings(&self) -> &HashMap<String, RuntimeValue> {
        &self.bindings
    }

    /// Clears all bindings, call stacks, streams, and resets registers.
    pub fn clear(&mut self) {
        self.bindings.clear();
        self.registers.reset();
        self.call_stack.clear();
        self.output_stream.clear();
        self.input_stream.clear();
        self.target_device = OMNI_TARGET_STDOUT;
        self.sequence_ribbon.clear();
        self.snapshots.clear();
    }

    /// Binds a value by name. If the name corresponds to a physical register,
    /// updates the silicon register cell in addition to the symbol table.
    /// If the name contains state-prime affixes (e.g., Character` or Character'),
    /// it binds the primed name and automatically synchronizes the base entity.
    pub fn set(&mut self, name: &str, value: RuntimeValue) {
        if let Some(reg) = MachineRegister::from_name(name) {
            self.registers.write(reg, value.clone());
            if reg != MachineRegister::DISCARD {
                self.bindings.insert(name.to_string(), value);
            }
        } else {
            self.bindings.insert(name.to_string(), value.clone());
            let trimmed = name.trim_end_matches(['`', '\'']);
            if trimmed != name && !trimmed.is_empty() {
                self.bindings.insert(trimmed.to_string(), value);
            }
        }
    }

    /// Retrieves a value by name. If the name matches a silicon register,
    /// yields a reference to the physical register cell.
    /// Supports state-prime fallback: looking up Character` falls back to Character
    /// if the specific prime has not yet been bound.
    pub fn get(&self, name: &str) -> Option<&RuntimeValue> {
        if let Some(reg) = MachineRegister::from_name(name) {
            Some(&self.registers.cells[reg.index() as usize])
        } else if let Some(val) = self.bindings.get(name) {
            Some(val)
        } else {
            let trimmed = name.trim_end_matches(['`', '\'']);
            if trimmed != name && !trimmed.is_empty() {
                self.bindings.get(trimmed)
            } else {
                None
            }
        }
    }

    /// Returns true if the environment contains a binding or register for the given name.
    pub fn contains(&self, name: &str) -> bool {
        self.get(name).is_some()
    }

    /// Pushes a procedural call frame onto the `%spool` stack.
    pub fn push_call_frame(&mut self, caller: RuntimeValue, self_val: RuntimeValue, return_head: usize) {
        let home = self.call_stack.len();
        self.registers.write(MachineRegister::HOME, RuntimeValue::Number(home as f64));
        self.registers.write(MachineRegister::HEAD, RuntimeValue::Number(return_head as f64));
        self.registers.write(MachineRegister::CALLER, caller.clone());
        self.registers.write(MachineRegister::SELF, self_val.clone());
        self.call_stack.push(CallFrame::new(caller, self_val, home, return_head));
        self.registers.write(MachineRegister::SPOOL, RuntimeValue::Number(self.call_stack.len() as f64));
    }

    /// Pops the active call frame from the `%spool` stack, restoring caller context.
    pub fn pop_call_frame(&mut self) -> Option<CallFrame> {
        let popped = self.call_stack.pop()?;
        let current_spool = self.call_stack.len();
        self.registers.write(MachineRegister::SPOOL, RuntimeValue::Number(current_spool as f64));
        if let Some(prev) = self.call_stack.last() {
            self.registers.write(MachineRegister::HOME, RuntimeValue::Number(prev.home as f64));
            self.registers.write(MachineRegister::HEAD, RuntimeValue::Number(prev.head as f64));
            self.registers.write(MachineRegister::CALLER, prev.caller.clone());
            self.registers.write(MachineRegister::SELF, prev.self_val.clone());
        }
        Some(popped)
    }

    /// Emits a runtime value to the `INTO` outbound port and buffers it in output_stream.
    pub fn write_into(&mut self, val: RuntimeValue) {
        self.registers.write(MachineRegister::INTO, val.clone());
        self.output_stream.push(val);
    }

    /// Reads the next input value from `input_stream` into `FROM`, or reads current `FROM`.
    pub fn read_from(&mut self) -> RuntimeValue {
        if !self.input_stream.is_empty() {
            let next_val = self.input_stream.remove(0);
            self.registers.write(MachineRegister::FROM, next_val.clone());
            next_val
        } else {
            self.registers.read(MachineRegister::FROM)
        }
    }

    /// Reads the current value held in the `INTO` output register.
    pub fn read_into(&self) -> RuntimeValue {
        self.registers.read(MachineRegister::INTO)
    }

    /// Enqueues an input value into `input_stream` and sets the `FROM` register.
    pub fn write_from(&mut self, val: RuntimeValue) {
        self.registers.write(MachineRegister::FROM, val.clone());
        self.input_stream.push(val);
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Default Trait Implementation & Unit Tests )
// ==============================================================================

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ternary::QuadState;

    #[test]
    fn test_environment_registers_and_discard_sink() {
        let mut env = Environment::new();
        env.set("INTO", RuntimeValue::from_glyph(b'Z'));
        assert_eq!(env.registers.read(MachineRegister::INTO).character_glyph(), Some(b'Z'));

        env.set("DISCARD", RuntimeValue::Number(999.0));
        assert_eq!(env.registers.read(MachineRegister::DISCARD), RuntimeValue::State(QuadState::Proceed));
    }

    #[test]
    fn test_environment_transactional_snapshots_and_rollback() {
        let mut env = Environment::new();
        env.set("A", RuntimeValue::Number(10.0));
        env.sequence_ribbon.push(RuntimeValue::from_char('a'));
        env.write_from(RuntimeValue::from_char('x'));

        env.begin_transaction();
        env.set("A", RuntimeValue::Number(20.0));
        env.sequence_ribbon.push(RuntimeValue::from_char('b'));
        let _ = env.read_from(); // consume 'x'
        assert_eq!(env.get("A"), Some(&RuntimeValue::Number(20.0)));
        assert_eq!(env.sequence_ribbon.len(), 2);
        assert_eq!(env.input_stream.len(), 0);

        env.rollback_transaction().expect("Rollback");
        assert_eq!(env.get("A"), Some(&RuntimeValue::Number(10.0)));
        assert_eq!(env.sequence_ribbon.len(), 1);
        assert_eq!(env.input_stream.len(), 1); // input queue restored!
    }

    #[test]
    fn test_environment_state_prime_synchronization() {
        let mut env = Environment::new();
        env.set("Character`", RuntimeValue::from_char('A'));
        assert!(env.contains("Character`"));
        assert!(env.contains("Character"));
        assert_eq!(env.get("Character"), env.get("Character`"));
    }
}

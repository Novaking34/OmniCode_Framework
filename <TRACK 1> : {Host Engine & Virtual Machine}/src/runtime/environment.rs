// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Runtime Environment Identity, Value Types, and Snapshot Invariants )
// ==============================================================================
//! OmniCode Runtime: Transactional Environment
//!
//! Provides scoped ontological symbol tables with native snapshot and rollback
//! capabilities to support balanced ternary -1 (Rollback) state transitions.

use std::collections::HashMap;
use crate::core::ternary::QuadState;
use crate::core::cell::Cell;
use crate::core::vessel::Vessel;
use crate::runtime::assembler::MachineRegister;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( RuntimeValue Enum, RegisterBank, CallFrame and Environment Initialization )
// ==============================================================================
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    State(QuadState),
    Number(f64),
    Text(String),
    Cell(Cell),
    Vessel(Vessel),
    EntityRecord {
        name: String,
        is_immutable: bool,
        values: HashMap<String, Box<RuntimeValue>>,
    },
    Vector(Vec<RuntimeValue>),
}

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

impl RegisterBank {
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

impl Default for RegisterBank {
    fn default() -> Self {
        Self::new()
    }
}

/// Call stack frame preserved on `SPOOL` for procedural execution.
#[derive(Debug, Clone, PartialEq)]
pub struct CallFrame {
    pub caller: RuntimeValue,
    pub self_val: RuntimeValue,
    pub home: usize,
    pub head: usize,
    pub locals: HashMap<String, RuntimeValue>,
}

/// Transactional snapshot capturing both symbolic bindings and physical registers.
#[derive(Debug, Clone)]
pub struct EnvironmentSnapshot {
    bindings: HashMap<String, RuntimeValue>,
    registers: RegisterBank,
    call_stack_len: usize,
    output_len: usize,
}

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
// ( Transactional Operations: Begin, Commit, Rollback, Binding Mutations )
// ==============================================================================
impl Environment {
    /// Takes a transactional snapshot of current bindings and physical registers.
    pub fn begin_transaction(&mut self) {
        self.snapshots.push(EnvironmentSnapshot {
            bindings: self.bindings.clone(),
            registers: self.registers.clone(),
            call_stack_len: self.call_stack.len(),
            output_len: self.output_stream.len(),
        });
    }

    /// Commits the active transaction, discarding the restore snapshot.
    pub fn commit_transaction(&mut self) {
        self.snapshots.pop();
    }

    /// Unwinds and restores the environment bindings and registers to the last snapshot.
    pub fn rollback_transaction(&mut self) -> Result<(), String> {
        if let Some(prev) = self.snapshots.pop() {
            self.bindings = prev.bindings;
            self.registers = prev.registers;
            self.call_stack.truncate(prev.call_stack_len);
            self.output_stream.truncate(prev.output_len);
            Ok(())
        } else {
            Err("FAILPOINT [RollbackUnderflow]: Attempted rollback with no active transaction snapshot".into())
        }
    }

    pub fn bindings(&self) -> &HashMap<String, RuntimeValue> {
        &self.bindings
    }

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

    pub fn contains(&self, name: &str) -> bool {
        self.get(name).is_some()
    }

    /// Pushes a procedural call frame onto the `SPOOL` stack.
    pub fn push_call_frame(&mut self, caller: RuntimeValue, self_val: RuntimeValue, return_head: usize) {
        let home = self.call_stack.len();
        self.registers.write(MachineRegister::HOME, RuntimeValue::Number(home as f64));
        self.registers.write(MachineRegister::HEAD, RuntimeValue::Number(return_head as f64));
        self.registers.write(MachineRegister::CALLER, caller.clone());
        self.registers.write(MachineRegister::SELF, self_val.clone());
        self.call_stack.push(CallFrame {
            caller,
            self_val,
            home,
            head: return_head,
            locals: HashMap::new(),
        });
        self.registers.write(MachineRegister::SPOOL, RuntimeValue::Number(self.call_stack.len() as f64));
    }

    /// Pops the active call frame from the `SPOOL` stack, restoring caller context.
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
}

impl RuntimeValue {
    /// Formats the runtime value with ANSI color styling for terminal output.
    pub fn format_colored(&self) -> String {
        match self {
            RuntimeValue::State(QuadState::Proceed) => "\x1b[1;32m+1:PROCEED\x1b[0m".to_string(),
            RuntimeValue::State(QuadState::Await) => "\x1b[1;33m0:AWAIT\x1b[0m".to_string(),
            RuntimeValue::State(QuadState::Rollback) => "\x1b[1;31m-1:ROLLBACK\x1b[0m".to_string(),
            RuntimeValue::State(QuadState::Halt) => "\x1b[1;35mNULL:HALT\x1b[0m".to_string(),
            RuntimeValue::Number(n) => format!("\x1b[1;36m{n}\x1b[0m"),
            RuntimeValue::Text(t) => format!("\x1b[0;32m\"{t}\"\x1b[0m"),
            RuntimeValue::Cell(c) => format!("\x1b[1;35m{c}\x1b[0m"),
            RuntimeValue::Vessel(v) => format!("\x1b[1;34m{v}\x1b[0m"),
            RuntimeValue::EntityRecord { name, is_immutable, values } => {
                let kind = if *is_immutable { "Axiom" } else { "Entity" };
                format!("\x1b[1;34m{kind} < {name} > [ {} fields ]\x1b[0m", values.len())
            }
            RuntimeValue::Vector(elements) => {
                let formatted: Vec<String> = elements.iter().map(|e| e.format_colored()).collect();
                format!("[{}]", formatted.join(", "))
            }
        }
    }
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Default Trait Implementation, Display Formatter & Teardown )
// ==============================================================================
impl std::fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeValue::State(s) => write!(f, "{s}"),
            RuntimeValue::Number(n) => write!(f, "{n}"),
            RuntimeValue::Text(t) => write!(f, "\"{t}\""),
            RuntimeValue::Cell(c) => write!(f, "{c}"),
            RuntimeValue::Vessel(v) => write!(f, "{v}"),
            RuntimeValue::EntityRecord { name, is_immutable, values } => {
                let kind = if *is_immutable { "Axiom" } else { "Entity" };
                write!(f, "{kind} < {name} > [ {} fields ]", values.len())
            }
            RuntimeValue::Vector(elements) => {
                let formatted: Vec<String> = elements.iter().map(|e| e.to_string()).collect();
                write!(f, "[{}]", formatted.join(", "))
            }
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}


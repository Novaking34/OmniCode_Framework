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

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( RuntimeValue Enum and Environment Initialization )
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
}

#[derive(Debug, Clone)]
pub struct Environment {
    bindings: HashMap<String, RuntimeValue>,
    snapshots: Vec<HashMap<String, RuntimeValue>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            snapshots: Vec::new(),
        }
    }
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Transactional Operations: Begin, Commit, Rollback, Binding Mutations )
// ==============================================================================
impl Environment {
    /// Takes a transactional snapshot of current bindings.
    pub fn begin_transaction(&mut self) {
        self.snapshots.push(self.bindings.clone());
    }

    /// Commits the active transaction, discarding the restore snapshot.
    pub fn commit_transaction(&mut self) {
        self.snapshots.pop();
    }

    /// Unwinds and restores the environment bindings to the last snapshot.
    pub fn rollback_transaction(&mut self) -> Result<(), String> {
        if let Some(prev) = self.snapshots.pop() {
            self.bindings = prev;
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
        self.snapshots.clear();
    }

    pub fn set(&mut self, name: &str, value: RuntimeValue) {
        self.bindings.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Option<&RuntimeValue> {
        self.bindings.get(name)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.bindings.contains_key(name)
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
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}


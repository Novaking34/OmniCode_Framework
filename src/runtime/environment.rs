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


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( RuntimeValue Enum and Environment Initialization )
// ==============================================================================
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    State(QuadState),
    Number(f64),
    Text(String),
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


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Default Trait Implementation and Teardown )
// ==============================================================================
impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

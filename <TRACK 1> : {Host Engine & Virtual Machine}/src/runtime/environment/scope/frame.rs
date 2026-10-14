// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Procedural Call Frame & Activation Record Invariants )
// ==============================================================================
//! OmniCode Runtime: Procedural Call Frames
//!
//! # Architectural Scope & Purpose:
//! Defines the `CallFrame` activation record preserved on the `%spool` stack
//! during procedural and functional invocation.
//!
//! Preserves the caller entity context, instance context (`self`), return instruction
//! pointer (`head`), base frame pointer (`home`), and local lexical symbol bindings.
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
//! 1. **Stack Unwinding Safety**: Activation records must unambiguously capture
//!    caller contexts to prevent memory clobbering across reentrant invocations.
// ==============================================================================

use std::collections::HashMap;
use crate::runtime::environment::universal::RuntimeValue;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( CallFrame Struct Definition & Field Declarations )
// ==============================================================================

/// Call stack frame preserved on `%spool` for procedural execution.
#[derive(Debug, Clone, PartialEq)]
pub struct CallFrame {
    /// The identity or value of the invoking caller.
    pub caller: RuntimeValue,
    /// The instance context (`self`) of the active procedure.
    pub self_val: RuntimeValue,
    /// The base stack depth (`%home`) prior to activation.
    pub home: usize,
    /// The return instruction address (`%head`).
    pub head: usize,
    /// Local scoped symbol bindings specific to this activation frame.
    pub locals: HashMap<String, RuntimeValue>,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( CallFrame Initialization & Scope Accessors )
// ==============================================================================

impl CallFrame {
    /// Creates a new activation record with empty local bindings.
    pub fn new(caller: RuntimeValue, self_val: RuntimeValue, home: usize, head: usize) -> Self {
        Self {
            caller,
            self_val,
            home,
            head,
            locals: HashMap::new(),
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
    fn test_call_frame_creation() {
        let frame = CallFrame::new(
            RuntimeValue::Text("Caller".to_string()),
            RuntimeValue::Text("Self".to_string()),
            0,
            100,
        );
        assert_eq!(frame.home, 0);
        assert_eq!(frame.head, 100);
        assert!(frame.locals.is_empty());
    }
}

// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Runtime Subsystem Identity & Component Re-exports )
// ==============================================================================
//! OmniCode Runtime Subsystem
//! Combines Transactional Environment with Quad-State Evaluator.

pub mod environment;
pub mod evaluator;
pub mod assembler;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Re-export Declarations for Runtime Access )
// ==============================================================================
pub use environment::{Environment, RuntimeValue, RegisterBank, CallFrame};
pub use evaluator::Evaluator;
pub use assembler::{
    OmniAssembler, AssembledTape, PhysicalSymbol, AssembledInstruction,
    MachineOp, MachineRegister, transpile_tape_to_omni, transpile_omni_to_tape,
};


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Runtime Execution Framework )
// ==============================================================================
// Runtime environment and evaluator are orchestrated during execution.


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Verification & Subsystem Status )
// ==============================================================================
// Runtime subsystem verified and closed.

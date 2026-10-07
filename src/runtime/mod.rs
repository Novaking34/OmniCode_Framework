// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Runtime Subsystem Identity & Component Re-exports )
// ==============================================================================
//! OmniCode Runtime Subsystem
//! Combines Transactional Environment with Quad-State Evaluator.

pub mod environment;
pub mod evaluator;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Re-export Declarations for Runtime Access )
// ==============================================================================
pub use environment::{Environment, RuntimeValue};
pub use evaluator::Evaluator;


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

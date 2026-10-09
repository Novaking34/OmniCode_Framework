// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Core Subsystem Identity, Module Architecture, and Invariant Re-exports )
// ==============================================================================
//! OmniCode Core Subsystem
//! Encapsulates the algebraic foundation: Ternary Quad-State, Boundaries, and Operators.

pub mod ternary;
pub mod boundaries;
pub mod operators;
pub mod filetype;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Re-export Declarations for Direct Core Access )
// ==============================================================================
pub use ternary::{QuadState, ROLLBACK, AWAIT, PROCEED, HALT};
pub use boundaries::BoundaryType;
pub use operators::{BeingRelation, DirectionalVector, Connective};
pub use filetype::OmniFileType;


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Core Subsystem Invariant Definitions )
// ==============================================================================
// Invariants and operators are actively materialized via re-exported submodules.


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Verification & Subsystem Status )
// ==============================================================================
// Core module namespace verified and closed.

// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( AST Subsystem Identity and Export Manifest )
// ==============================================================================
//! OmniCode AST Subsystem
//! Houses all syntax tree nodes, boundary containers, and expression shapes.

pub mod nodes;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Re-export Declarations for AST Types )
// ==============================================================================
pub use nodes::*;


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( AST Subsystem Manifest Definitions )
// ==============================================================================
// AST definitions are actively mapped in nodes.rs.


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Verification & Module Status )
// ==============================================================================
// AST subsystem verified and sealed.

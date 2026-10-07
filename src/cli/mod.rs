// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( CLI Subsystem Identity, Module Architecture & Core Re-exports )
// ==============================================================================
//! OmniCode Command Line Subsystem
//!
//! Exposes interactive terminal REPL, script runners, structural 4-block linters,
//! and template scaffolding generators.

pub mod repl;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Re-export Declarations & Public Interface )
// ==============================================================================
pub use repl::run_repl;


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( CLI Orchestration Framework )
// ==============================================================================
// Command line entrypoints route to interactive REPL or single-file runners.


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Verification and Sealing )
// ==============================================================================
// CLI subsystem verified and sealed.

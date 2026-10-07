// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Diagnostics Subsystem Identity, Module Architecture & Exports )
// ==============================================================================
//! OmniCode Diagnostics Subsystem
//! Combines Structured Logger, Safety Debugger, and 4-Block Structural Linter.

pub mod logger;
pub mod debugger;
pub mod linter;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Re-export Declarations for Diagnostic Tools )
// ==============================================================================
pub use logger::{DiagnosticLogger, FailpointType, LogEvent};
pub use debugger::OmniDebugger;
pub use linter::{BlockKind, DetectedBlock, LintReport, OmniLinter};


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Subsystem Core Registrations )
// ==============================================================================
// Diagnostic modules are actively coordinated across compiler passes.


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Verification & Subsystem Status )
// ==============================================================================
// Diagnostics subsystem verified and closed.

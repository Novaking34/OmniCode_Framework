// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Diagnostics Subsystem Identity, Module Architecture & Exports )
// ==============================================================================
//! OmniCode Diagnostics Subsystem
//! Combines Structured Logger, Safety Debugger, and 4-Block Structural Linter.

pub mod logger;
pub mod debugger;
pub mod linter;
pub mod restorator;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Re-export Declarations for Diagnostic Tools )
// ==============================================================================
pub use logger::{DiagnosticLogger, FailpointType, LogEvent, FixKind, FixSuggestion, CognitiveNote};
pub use debugger::OmniDebugger;
pub use linter::{BlockKind, DetectedBlock, LintReport, OmniLinter};
pub use restorator::{Restorator, RestorationReport, AppliedRepair};


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

// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Diagnostics Subsystem Identity, Module Architecture & Exports )
// ==============================================================================
//! OmniCode Diagnostics Subsystem
//!
//! # Architecture & Cognitive Cybernetics
//! Coordinates the runtime sensory, debugging, and self-healing immune organs:
//! - [`logger`]: Structured failpoint interceptor, telemetry logger, and AI episodic memory emitter ([`CognitiveNote`]).
//! - [`debugger`]: Boundary stack monitor, immutability inspector, retroaction depth guard, and test verification oracle.
//! - [`linter`]: Structural validator enforcing the canonical 4-Block architecture across compilation units.
//! - [`restorator`]: Autonomous multi-pass code healing engine with test-oracle certification.
//!
//! # IDE Hover Support
//! All diagnostic components are re-exported at the module root for immediate autocompletion
//! and documentation discovery in language servers and coding assistants.

pub mod logger;
pub mod debugger;
pub mod linter;
pub mod restorator;
pub mod hover;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Re-export Declarations for Diagnostic Tools )
// ==============================================================================
pub use logger::{
    DiagnosticLogger, FailpointType, LogEvent, FixKind, FixSuggestion,
    CognitiveNote, GenesisBaseline, GenesisBootLog, RollFrame,
};
pub use debugger::OmniDebugger;
pub use linter::{
    BlockKind, BodyLinter, BodyScore, ClosingLinter, ClosingScore, DetectedBlock, FileAuditScore,
    LintReport, MetadataLinter, MetadataScore, OmniLinter, OpeningLinter, OpeningScore, QuadStateGate,
};
pub use restorator::{Restorator, RestorationReport, AppliedRepair};
pub use hover::{HoverEngine, HoverCard, OmniDefinition, TapeRollContext};


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Subsystem Core Registrations & Coordination Invariants )
// ==============================================================================
// Diagnostic modules are actively coordinated across compiler passes.
// The Logger records incidents, the Restorator synthesizes candidate patches,
// and the Debugger executes verification test oracles before state commitment.


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Verification & Subsystem Status )
// ==============================================================================
// Diagnostics subsystem verified and closed.


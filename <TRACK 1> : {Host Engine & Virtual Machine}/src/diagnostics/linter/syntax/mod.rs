// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Syntax, Spatial & Grammar Subsystem Coordinator )
// ==============================================================================
//! Syntax, Spatial Geometry & Grammar Linter Subsystem
//!
//! Enforces lexical syntax, grammatical AST correctness, and spatial geometry:
//! - [`spatial`]: 80-column IDE dual side-panel ruler and bracket identifier naming hygiene.
//! - [`grammar`]: Lexer token stream hygiene, AST parser verification, SVO verbs, and Living Asides.

pub mod grammar;
pub mod spatial;

pub use grammar::GrammarLinter;
pub use spatial::{SpatialLintConfig, SpatialLinter};

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Coordination Invariants )
// ==============================================================================
// Syntax submodules coordinate pre-compilation syntactic and layout hygiene.

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Subsystem Pipeline )
// ==============================================================================
// Coordinated across passes 1 and 3 of the OmniLinter pipeline.

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Status )
// ==============================================================================
// Syntax subsystem sealed.

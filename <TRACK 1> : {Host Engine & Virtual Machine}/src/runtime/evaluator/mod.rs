// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Evaluator Subsystem Identity & Public Exports )
// ==============================================================================
//! OmniCode Runtime: Evaluator Subsystem Facade
//!
//! # Architectural Scope & Purpose:
//! Modularizes the AST execution engine into specialized domains:
//! - `universal`: Pure AST pattern extractors, trit sequence decoders, and Horner algorithms.
//! - `relations`: Relational algebra (`=`, `!=`, `~=`, `<=>`, `<=`, `>=`) and bounds.
//! - `vectors`: Directional causal propagation vectors (`-->`, `<--`, `^--`, `v--`).
//! - `connectives`: Quad-State logical gates (`! BUT`, `& AND`, `| OR`, `-- NOT`, `::`).
//! - `verbs`: SVO action verbs (`INSCRIBE`, `TRANSDUCE`, `PROJECT`, `SEQUENCE`) and vessels.
//! - `core`: The central `Evaluator` coordinator and module loader.
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
//! 1. **Complete Decoupling**: Modularizing the evaluator prevents monolith regressions
//!    while providing individual sub-engines for specialized testing.
//! 2. **Backward Compatibility**: Re-exports `Evaluator` at the module root, ensuring zero
//!    disruptions to existing test suites, REPL, and CLI commands.
// ==============================================================================

pub mod universal;
pub mod relations;
pub mod vectors;
pub mod connectives;
pub mod verbs;
pub mod core;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Re-export Declarations for Public Evaluator Access )
// ==============================================================================
pub use core::Evaluator;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Subsystem Proofs & Invariants )
// ==============================================================================
// The Evaluator executes on top of the Transactional Environment substrate.

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Verification & Module Closure )
// ==============================================================================
// Evaluator subsystem verified and closed.

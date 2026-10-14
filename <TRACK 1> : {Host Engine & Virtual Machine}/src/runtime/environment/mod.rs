// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Runtime Environment Subsystem Facade & Comprehensive Re-exports )
// ==============================================================================
//! OmniCode Runtime: Transactional Environment & Register Subsystem
//!
//! # Architectural Scope & Purpose:
//! Modularizes the runtime data substrate and execution state into three
//! decoupled, topologically ordered layers:
//! 1. `universal`: Universal `RuntimeValue` enum, scalar unwrappers, cross-substrate
//!    glyph equality, and physical memory capacity metrics.
//! 2. `hardware`: Physical Silicon `RegisterBank` (the 13 Named Machine Registers)
//!    and canonical hardware device target addresses (`OMNI_TARGET_*`).
//! 3. `scope`: Procedural `CallFrame`, transactional `EnvironmentSnapshot`, and the
//!    `Environment` state coordinator.
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
//! 1. **Backward Compatibility**: Re-exports all public types at the root of `environment`
//!    ensuring zero downstream breaking changes for the Evaluator, Assembler, CLI,
//!    and Test suites.
//! 2. **Modular Reusability**: Evaluator modules and external crates can import
//!    `universal::RuntimeValue` without pulling in hardware or transaction management.
// ==============================================================================

pub mod universal;
pub mod hardware;
pub mod scope;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Re-export Declarations for Full Backward Compatibility )
// ==============================================================================

// Universal Value Substrate
pub use universal::RuntimeValue;

// Hardware Silicon Registers & Ports
pub use hardware::{
    RegisterBank,
    OMNI_TARGET_STDOUT,
    OMNI_TARGET_VGA,
    OMNI_TARGET_COM1,
};

// Scopes, Snapshots & Environment State
pub use scope::{
    CallFrame,
    EnvironmentSnapshot,
    Environment,
};

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Subsystem Structural Proofs & Invariants )
// ==============================================================================
// The Environment serves as the foundational data substrate for the Evaluator.

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Verification & Module Closure )
// ==============================================================================
// Runtime environment subsystem verified and closed.

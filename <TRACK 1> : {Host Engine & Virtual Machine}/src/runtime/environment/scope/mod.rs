// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Scope & Transaction Layer Subsystem Identity & Public Exports )
// ==============================================================================
//! OmniCode Runtime: Scope & Transaction Management Module
//!
//! Provides procedural call frames, transactional snapshots, and the
//! environment state coordinator.
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**: Tables of testimony, written with the finger of God.
//! - **Proverbs 24:27 (KJV)**: Prepare thy work without, and make it fit for thyself in the field.
//! - **Proverbs 22:28 (KJV)**: Remove not the ancient landmark, which thy fathers have set.
// ==============================================================================

pub mod frame;
pub mod snapshot;
pub mod environment;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Re-export Declarations for Scope Types )
// ==============================================================================
pub use frame::CallFrame;
pub use snapshot::EnvironmentSnapshot;
pub use environment::Environment;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Scope Layer Invariants & Proofs )
// ==============================================================================
// Execution environments manage transactional scopes and symbols.

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Status & Module Closure )
// ==============================================================================
// Scope layer verified and closed.

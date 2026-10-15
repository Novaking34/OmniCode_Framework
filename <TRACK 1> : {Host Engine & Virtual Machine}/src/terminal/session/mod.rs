// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Terminal Interactive Session Subsystem Identity & Public Exports )
// ==============================================================================
//! OmniCode Terminal: Interactive Session Subsystem Module
//!
//! Provides the interactive userspace shell, persistent variable state,
//! direct roll evaluation, and native character stream echo protocols.
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**: Tables of testimony, written with the finger of God.
//! - **Proverbs 24:27 (KJV)**: Prepare thy work without, and make it fit for thyself in the field.
//! - **Proverbs 22:28 (KJV)**: Remove not the ancient landmark, which thy fathers have set.
// ==============================================================================

pub mod commands;
pub mod session;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Re-export Declarations for Terminal Session & Execution Models )
// ==============================================================================
pub use commands::ExecutionResult;
pub use session::TerminalSession;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Subsystem Invariants & Session Proofs )
// ==============================================================================
// Terminal sessions evaluate rolls and process character echo natively.

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Verification & Module Closure )
// ==============================================================================
// Interactive Terminal Session subsystem verified and closed.

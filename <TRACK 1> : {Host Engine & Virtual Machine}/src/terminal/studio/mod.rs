// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Terminal Studio Subsystem Identity & Public Exports )
// ==============================================================================
//! OmniCode Terminal: Cinema Studio Subsystem Module
//!
//! Provides the real-time visual projection HUD of the 4-directional Memory Reel,
//! showing active tracks, scenes, rolls, and set props (64-bit / 32-trit cells).
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**: Tables of testimony, written with the finger of God.
//! - **Proverbs 24:27 (KJV)**: Prepare thy work without, and make it fit for thyself in the field.
//! - **Proverbs 22:28 (KJV)**: Remove not the ancient landmark, which thy fathers have set.
// ==============================================================================

pub mod hud;
pub mod kinematics;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Re-export Declarations for Cinema Studio )
// ==============================================================================
pub use hud::OmniStudio;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Subsystem Invariants & Cinema Studio Proofs )
// ==============================================================================
// Cinema Studio decouples visual ANSI HUD rendering from memory reel kinematics.

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Verification & Module Closure )
// ==============================================================================
// Cinema Studio subsystem verified and closed.

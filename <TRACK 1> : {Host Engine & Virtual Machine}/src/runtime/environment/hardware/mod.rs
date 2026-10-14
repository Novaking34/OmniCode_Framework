// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Silicon Hardware Layer Subsystem Identity & Public Exports )
// ==============================================================================
//! OmniCode Runtime: Silicon Hardware Substrate Module
//!
//! Provides the physical machine register bank and memory-mapped hardware constants.
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**: Tables of testimony, written with the finger of God.
//! - **Proverbs 24:27 (KJV)**: Prepare thy work without, and make it fit for thyself in the field.
//! - **Proverbs 22:28 (KJV)**: Remove not the ancient landmark, which thy fathers have set.
// ==============================================================================

pub mod registers;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Re-export Declarations for Hardware Register Bank & Target Addresses )
// ==============================================================================
pub use registers::{
    RegisterBank,
    OMNI_TARGET_STDOUT,
    OMNI_TARGET_VGA,
    OMNI_TARGET_COM1,
};

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Hardware Substrate Invariants & Proofs )
// ==============================================================================
// Hardware registers map directly to VM execution state.

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Status & Module Closure )
// ==============================================================================
// Silicon hardware substrate verified and closed.

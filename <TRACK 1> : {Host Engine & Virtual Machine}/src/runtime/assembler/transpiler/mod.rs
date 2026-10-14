// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Multi-Target Transpiler Subsystem Identity & Public Exports )
// ==============================================================================
//! OmniCode Runtime: Multi-Target Transpiler Module
//!
//! Provides text assembly listings, physical silicon hex dumps, structured JSON ASTs,
//! bidirectional `.tape` <--> `.omni` translation, and bare-metal x86/Multiboot assembly.
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**: Tables of testimony, written with the finger of God.
//! - **Proverbs 24:27 (KJV)**: Prepare thy work without, and make it fit for thyself in the field.
//! - **Proverbs 22:28 (KJV)**: Remove not the ancient landmark, which thy fathers have set.
// ==============================================================================

pub mod listing;
pub mod json;
pub mod omni;
pub mod baremetal;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Re-export Declarations for Transpilers & Serializers )
// ==============================================================================
pub use listing::{
    format_assembly_listing,
    format_silicon_words,
};

pub use json::format_tape_json;

pub use omni::{
    transpile_tape_to_omni,
    transpile_omni_to_tape,
};

pub use baremetal::{
    transpile_tape_to_asm,
    transpile_tape_to_multiboot_asm,
    transpile_omni_to_asm,
};

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Subsystem Proofs & Invariants )
// ==============================================================================
// Transpilers project assembled artifacts to external formats without state mutation.

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Verification & Module Closure )
// ==============================================================================
// Multi-target transpiler layer verified and closed.

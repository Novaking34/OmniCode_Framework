// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Universal Evaluator Helpers Subsystem Identity & Public Exports )
// ==============================================================================
//! OmniCode Runtime: Universal Evaluator Helpers Module
//!
//! Re-exports AST decoders, trit extractors, and Horner expansion functions.
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**: Tables of testimony, written with the finger of God.
//! - **Proverbs 24:27 (KJV)**: Prepare thy work without, and make it fit for thyself in the field.
//! - **Proverbs 22:28 (KJV)**: Remove not the ancient landmark, which thy fathers have set.
// ==============================================================================

pub mod decoder;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Re-export Declarations for Universal Decoding Helpers )
// ==============================================================================
pub use decoder::{
    extract_symbol_name,
    extract_trits_from_node,
    trits_to_integer,
};

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Subsystem Proofs & Invariants )
// ==============================================================================
// Decoders are pure and stateless across all evaluation steps.

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Status & Module Closure )
// ==============================================================================
// Universal evaluation decoder layer verified and closed.

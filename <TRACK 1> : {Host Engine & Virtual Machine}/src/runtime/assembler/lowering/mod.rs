// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Lowering Subsystem Identity, Invariants & Scripture Anchors )
// ==============================================================================
//! OmniCode Runtime: Assembler Lowering Subsystem
//!
//! Provides the AST and tape statement lowering pipelines:
//! - `roll`: Lowers atomic procedural roll statements into physical `AssembledInstruction`s.
//! - `tape`: Lowers entire procedural `.tape` process ribbons into `AssembledTape`.
//! - `omni`: Lowers declarative 4-Block `.omni` spatial architecture reels into `AssembledTape`.
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**:
//!   "And he gave unto Moses, when he had made an end of communing with him upon mount Sinai,
//!    two tables of testimony, tables of stone, written with the finger of God."
//!   (The immutable inscription of physical machine code onto bounded hardware substrates).
//! - **Proverbs 24:27 (KJV)**:
//!   "Prepare thy work without, and make it fit for thyself in the field;
//!    and afterwards build thine house."
//!   (The discipline of proving atomic roll lowering invariants before constructing system tapes).
//! - **Proverbs 22:28 (KJV)**:
//!   "Remove not the ancient landmark, which thy fathers have set."
//!   (The inviolable preservation of historical silicon addresses, MMIO ports, and opcodes).
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Module Declarations for Lowering Subsystems )
// ==============================================================================
pub mod roll;
pub mod tape;
pub mod omni;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Subsystem Proofs & Lowering Invariants )
// ==============================================================================
// Lowering methods (`assemble_roll`, `assemble_tape`, `assemble_omni`) are attached
// directly to `OmniAssembler` within their respective domain modules.

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Verification & Module Closure )
// ==============================================================================
// Lowering module sealed in affirmative PROCEED.

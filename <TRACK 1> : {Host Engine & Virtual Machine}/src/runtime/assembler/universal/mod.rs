// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Universal Assembler Subsystem Identity & Public Exports )
// ==============================================================================
//! OmniCode Runtime: Universal Assembler ISA & Types Module
//!
//! Re-exports hardware opcodes, machine registers, word synthesis,
//! physical symbols, and assembled tape structures.
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**: Tables of testimony, written with the finger of God.
//! - **Proverbs 24:27 (KJV)**: Prepare thy work without, and make it fit for thyself in the field.
//! - **Proverbs 22:28 (KJV)**: Remove not the ancient landmark, which thy fathers have set.
// ==============================================================================

pub mod isa;
pub mod types;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Re-export Declarations for Universal ISA & Types )
// ==============================================================================
pub use isa::{
    MachineOp,
    MachineRegister,
    pronoun_to_register,
    pronoun_to_op,
    polarity_to_op,
    compute_word_checksum,
    synthesize_machine_word,
};

pub use types::{
    PhysicalSymbol,
    AssembledInstruction,
    AssembledTape,
    DEFAULT_PARTITION_CELL_CAPACITY,
    DEFAULT_ASSEMBLER_DEBUGGER_DEPTH,
};

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Subsystem Proofs & Invariants )
// ==============================================================================
// Universal types are decoupled from lowering pipelines and transpilers.

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Verification & Module Closure )
// ==============================================================================
// Universal assembler ISA layer verified and closed.

// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Physical Assembler Subsystem Identity, Public Facade & Architectural Invariants )
// ==============================================================================
//! OmniCode Runtime: Physical Assembler Subsystem Facade
//!
//! # Architectural Scope & Purpose:
//! The Physical Assembler acts as the transduction engine between abstract
//! ontological archetypes (`+`) and concrete silicon memory layouts (`-`).
//! It decomposes into four discrete domain modules:
//! 1. `universal`: Stateless hardware instruction set architecture ([`MachineOp`]),
//!    named physical registers ([`MachineRegister`]), 64-bit balanced-ternary machine word
//!    synthesis ([`synthesize_machine_word`]), and core data containers ([`PhysicalSymbol`], [`AssembledTape`]).
//! 2. `lowering`: Procedural `.tape` and declarative `.omni` statement compilation,
//!    memory partition allocation (`@ 0x0000`), hardware register binding, and delimiter stack validation.
//! 3. `transpiler`: Multi-target code emission (annotated assembly listings, silicon hex words,
//!    IDE hovercard JSON ASTs, bidirectional `.tape` <-> `.omni` conversion, 16-bit Real Mode MBR bootloader,
//!    and 32-bit Multiboot ELF assembly).
//! 4. `core`: Main [`OmniAssembler`] coordinator struct, lifecycle methods, and comprehensive test suite.
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**:
//!   "And he gave unto Moses, when he had made an end of communing with him upon mount Sinai,
//!    two tables of testimony, tables of stone, written with the finger of God."
//!   (The immutable inscription of physical machine code onto bounded hardware substrates).
//! - **Proverbs 24:27 (KJV)**:
//!   "Prepare thy work without, and make it fit for thyself in the field;
//!    and afterwards build thine house."
//!   (The discipline of proving atomic types and substrates before constructing systems).
//! - **Proverbs 22:28 (KJV)**:
//!   "Remove not the ancient landmark, which thy fathers have set."
//!   (The inviolable preservation of historical silicon addresses, MMIO ports, and opcodes).
//!
//! # Why Extensive Commenting Matters for AI Agents & Developers:
//! 1. **Complete Compatibility**: Re-exports all universal ISA definitions, memory symbols,
//!    transpilers, and the coordinator struct to preserve 100% backward compatibility for all callers.
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Module Hierarchy Declarations & Public Facade Exports )
// ==============================================================================
pub mod universal;
pub mod lowering;
pub mod transpiler;
pub mod core;

// Re-export universal ISA opcodes, registers, and symbols
pub use universal::{
    MachineOp,
    MachineRegister,
    PhysicalSymbol,
    AssembledInstruction,
    AssembledTape,
    DEFAULT_PARTITION_CELL_CAPACITY,
    DEFAULT_ASSEMBLER_DEBUGGER_DEPTH,
    pronoun_to_register,
    pronoun_to_op,
    polarity_to_op,
    compute_word_checksum,
    synthesize_machine_word,
};

// Re-export transpilers and formatters
pub use transpiler::{
    format_assembly_listing,
    format_silicon_words,
    format_tape_json,
    transpile_tape_to_omni,
    transpile_omni_to_tape,
    transpile_tape_to_asm,
    transpile_tape_to_multiboot_asm,
    transpile_omni_to_asm,
};

// Re-export main coordinator struct
pub use core::OmniAssembler;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Subsystem Proofs & Invariants )
// ==============================================================================
// The Assembler subsystem enforces structural delimiter closure via `OmniDebugger`,
// allocates memory offsets deterministically within storehouse partitions,
// and guarantees that all compiled tapes seal in affirmative `+1:PROCEED`.

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Verification & Module Closure )
// ==============================================================================
// Physical Assembler Subsystem facade verified and sealed in affirmative PROCEED.

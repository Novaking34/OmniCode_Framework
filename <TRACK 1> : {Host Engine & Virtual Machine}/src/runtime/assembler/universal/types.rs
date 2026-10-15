// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Physical Silicon Symbols, Assembled Instructions & Tape Containers )
// ==============================================================================
//! OmniCode Runtime: Assembled Machine Artifacts & Symbol Structures
//!
//! # Architectural Scope & Purpose:
//! Defines the core containers produced by the physical silicon lowering engine:
//! 1. `PhysicalSymbol`: Physical memory allocation record for ontological symbols.
//! 2. `AssembledInstruction`: Discrete physical machine instruction container holding
//!    opcodes, silicon offsets, registers, and the 64-bit machine word ([`Word64`]).
//! 3. `AssembledTape`: The complete assembled physical machine tape ready for execution.
//!
//! Also defines baseline partition capacities and debugger depth constants.
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
//! 1. **Complete Serialization Fidelity**: `AssembledTape` serves as the primary artifact
//!    for both software simulation and bare-metal BIOS/Multiboot execution.
// ==============================================================================

use std::collections::HashMap;
use crate::core::operators::Polarity;
use crate::core::tape::TapeHead;
use crate::core::ternary::QuadState;
use crate::core::word::Word64;
use crate::runtime::assembler::universal::isa::{MachineOp, MachineRegister};
use crate::runtime::assembler::transpiler::{
    format_assembly_listing,
    format_silicon_words,
    format_tape_json,
    transpile_tape_to_omni,
    transpile_tape_to_asm,
    transpile_tape_to_multiboot_asm,
    transpile_tape_to_c,
    transpile_tape_to_lua,
};

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Hardware Baseline Constants & PhysicalSymbol Struct )
// ==============================================================================

/// Baseline cell capacity reserved for unspecified memory partitions.
pub const DEFAULT_PARTITION_CELL_CAPACITY: usize = 64;
/// Maximum delimiter boundary nesting depth for structural invariant assertions.
pub const DEFAULT_ASSEMBLER_DEBUGGER_DEPTH: usize = 100;

/// Physical allocation record for an ontological symbol in silicon memory.
#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalSymbol {
    /// Ontological name identifier.
    pub name: String,
    /// Formal polarity: Abstract (`+`) or Concrete (`-`).
    pub polarity: Polarity,
    /// Explicit zero-based memory cell offset within the designated partition.
    pub offset: Option<usize>,
    /// Number of contiguous physical RAM cells allocated to this symbol.
    pub size_cells: usize,
    /// Name of the enclosing memory partition or storehouse where allocated.
    pub partition_name: Option<String>,
}

impl PhysicalSymbol {
    /// Returns `true` if this symbol denotes a physical silicon allocation (`-`).
    pub fn is_concrete(&self) -> bool {
        self.polarity == Polarity::Concrete
    }

    /// Returns `true` if this symbol denotes an unallocated abstract archetype (`+`).
    pub fn is_abstract(&self) -> bool {
        self.polarity == Polarity::Abstract
    }
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( AssembledInstruction & AssembledTape Containers )
// ==============================================================================

/// A lowered, assembled instruction roll ready for hardware dispatch.
#[derive(Debug, Clone, PartialEq)]
pub struct AssembledInstruction {
    /// 1-based roll position within the execution tape.
    pub roll_index: usize,
    /// 1-based source line number from the original `.tape` or `.omni` document.
    pub source_line: usize,
    /// Raw un-lowered text content of the roll for diagnostics and hovercards.
    pub original_text: String,
    /// Discrete physical machine opcode.
    pub op: MachineOp,
    /// Bound hardware CPU register, if applicable.
    pub register: Option<MachineRegister>,
    /// Target physical memory cell offset, if applicable.
    pub address: Option<usize>,
    /// Packed 64-bit balanced-ternary machine word container.
    pub word: Word64,
    /// QuadState execution status for this instruction (+1:PROCEED).
    pub status: QuadState,
    /// Human-readable operational code string.
    pub op_code: String,
}

/// The complete assembled physical machine tape.
#[derive(Debug, Clone, PartialEq)]
pub struct AssembledTape {
    /// Process covenant manifest header.
    pub head: TapeHead,
    /// Total cell capacity carved for this memory partition.
    pub partition_size: usize,
    /// Resolved physical symbol table mapping identifiers to silicon offsets.
    pub symbols: HashMap<String, PhysicalSymbol>,
    /// Sequential stream of lowered physical machine instructions.
    pub instructions: Vec<AssembledInstruction>,
    /// Final affirmative seal state (+1:PROCEED).
    pub commit_status: QuadState,
}

impl AssembledTape {
    /// Formats the assembled machine tape as a human-readable assembly listing.
    pub fn format_listing(&self) -> String {
        format_assembly_listing(self)
    }

    /// Emits the human-readable physical silicon word stream (Hex, registers, opcodes, QuadState).
    pub fn format_words(&self) -> String {
        format_silicon_words(self)
    }

    /// Emits a structured JSON representation for tooling, IDEs, and diagnostics.
    pub fn format_json(&self) -> String {
        format_tape_json(self)
    }

    /// Transpiles the assembled tape into a declarative 4-Block spatial architecture reel (`.omni`).
    pub fn format_omni(&self) -> String {
        transpile_tape_to_omni(self)
    }

    /// Transpiles the assembled tape into native bare-metal x86 bootstrap assembly (`.asm`).
    pub fn format_asm(&self) -> String {
        transpile_tape_to_asm(self)
    }

    /// Transpiles the assembled tape into 32-bit Multiboot ELF assembly (`.asm`).
    pub fn format_multiboot_asm(&self) -> String {
        transpile_tape_to_multiboot_asm(self)
    }

    /// Transpiles the assembled tape into a standalone C module (`.c`).
    pub fn format_c(&self) -> String {
        transpile_tape_to_c(self)
    }

    /// Transpiles the assembled tape into a standalone Lua script (`.lua`).
    pub fn format_lua(&self) -> String {
        transpile_tape_to_lua(self)
    }

    /// Emits the raw physical Word64 machine word sequence for hardware dispatch.
    pub fn to_machine_words(&self) -> Vec<Word64> {
        self.instructions.iter().map(|i| i.word).collect()
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests & Verification Proofs )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physical_symbol_polarity() {
        let sym_abs = PhysicalSymbol {
            name: "Archetype".to_string(),
            polarity: Polarity::Abstract,
            offset: None,
            size_cells: 1,
            partition_name: None,
        };
        assert!(sym_abs.is_abstract());
        assert!(!sym_abs.is_concrete());

        let sym_conc = PhysicalSymbol {
            name: "Concrete".to_string(),
            polarity: Polarity::Concrete,
            offset: Some(0x10),
            size_cells: 1,
            partition_name: Some("Test".to_string()),
        };
        assert!(sym_conc.is_concrete());
        assert!(!sym_conc.is_abstract());
    }
}

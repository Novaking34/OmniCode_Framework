// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Assembly Listing Formatters & Physical Silicon Word Stream Dumps )
// ==============================================================================
//! OmniCode Runtime: Assembly Listing & Silicon Word Serializers
//!
//! # Architectural Scope & Purpose:
//! Formats assembled physical machine tapes into annotated human-readable listings
//! and physical silicon word streams for telemetry, debugging, and terminal audits.
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
//! 1. **Telemetry Readability**: Explicitly formatted listings allow verification of
//!    memory cell offsets, opcode numbers, and QuadState execution status.
// ==============================================================================

use crate::runtime::assembler::universal::types::AssembledTape;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Function Declarations for Listings & Word Dumps )
// ==============================================================================

/// Formats an assembled machine tape as a human-readable annotated assembly listing.
pub fn format_assembly_listing(tape: &AssembledTape) -> String {
    let mut out = String::new();
    out.push_str("=============================================================\n");
    out.push_str(&format!("ASSEMBLED MACHINE TAPE: [{}]\n", tape.head.name));
    out.push_str(&format!("PRINCIPAL: {} | TARGET: {}\n", tape.head.who, tape.head.what));
    out.push_str(&format!("MEMORY PARTITION: {} cells\n", tape.partition_size));
    out.push_str("-------------------------------------------------------------\n");
    out.push_str("PHYSICAL SYMBOL TABLE:\n");
    for (name, sym) in &tape.symbols {
        let offset_str = sym.offset.map_or("N/A".to_string(), |o| format!("@ 0x{:04X} ({})", o, o));
        out.push_str(&format!(
            "  • {:<16} | Polarity: {:<8} | Size: {:>3} cells | Offset: {}\n",
            name, sym.polarity.symbol(), sym.size_cells, offset_str
        ));
    }
    out.push_str("-------------------------------------------------------------\n");
    out.push_str("ASSEMBLED INSTRUCTION ROLLS:\n");
    for inst in &tape.instructions {
        let addr_str = inst.address.map_or("      ".to_string(), |a| format!("@{:04X}", a));
        let reg_str = inst.register.map_or("     ".to_string(), |r| format!("{:<5}", r.name()));
        out.push_str(&format!(
            "  [{:>2}] {} | {} | {:<20} | Word: 0x{:016X} | Status: {}\n        --> {}\n",
            inst.roll_index, addr_str, reg_str, inst.op_code, inst.word.raw, inst.status.macro_op(), inst.original_text
        ));
    }
    out.push_str("=============================================================\n");
    out.push_str(&format!("COMMIT SEAL: {}\n", tape.commit_status.macro_op()));
    out
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Silicon Word Stream Serializer )
// ==============================================================================

/// Emits the human-readable physical silicon word stream (Hex, registers, opcodes, QuadState).
pub fn format_silicon_words(tape: &AssembledTape) -> String {
    let mut out = String::new();
    out.push_str(&format!("PHYSICAL SILICON WORD STREAM: [{}]\n", tape.head.name));
    out.push_str("-------------------------------------------------------------\n");
    for (idx, inst) in tape.instructions.iter().enumerate() {
        let reg_str = inst.register.map_or("     ".to_string(), |r| format!("{:<5}", r.name()));
        out.push_str(&format!(
            "[{:>3}] 0x{:016X} | {} | {:<20} | Status: {}\n",
            idx + 1,
            inst.word.raw,
            reg_str,
            inst.op.mnemonic(),
            inst.status.macro_op()
        ));
    }
    out.push_str("-------------------------------------------------------------\n");
    out.push_str(&format!("TOTAL WORDS: {}\n", tape.instructions.len()));
    out
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests & Verification Invariants )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::core::tape::TapeHead;
    use crate::core::ternary::QuadState;

    #[test]
    fn test_format_listing_empty() {
        let tape = AssembledTape {
            head: TapeHead {
                name: "TestTape".into(),
                who: "Kernel".into(),
                what: "Test".into(),
                when_clause: "Boot".into(),
                where_clause: "RAM".into(),
                why: "Proof".into(),
                how: "Proceed".into(),
                required_partition_size: 64,
                raw_text: String::new(),
            },
            partition_size: 64,
            symbols: HashMap::new(),
            instructions: Vec::new(),
            commit_status: QuadState::Proceed,
        };
        let listing = format_assembly_listing(&tape);
        assert!(listing.contains("TestTape"));
        assert!(listing.contains("COMMIT SEAL: PROCEED"));
    }
}

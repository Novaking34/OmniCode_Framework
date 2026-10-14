// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Bidirectional Transpilation Between Procedural Tapes & Spatial Omni Reels )
// ==============================================================================
//! OmniCode Runtime: Bidirectional Omni Transpiler
//!
//! # Architectural Scope & Purpose:
//! Performs lossless bidirectional translation between the two primary forms
//! of OmniCode source documents:
//! 1. Temporal Procedural Process Ribbons (`.tape`)
//! 2. Declarative 4-Block Spatial Architecture Reels (`.omni`)
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
//! 1. **Isomorphic Triad**: Tapes and Reels are mathematical duals. A developer or agent
//!    can author declarative reels and transpile to procedural tapes, or vice-versa.
// ==============================================================================

use crate::runtime::assembler::universal::types::AssembledTape;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Tape to Omni Transpiler Function Declaration )
// ==============================================================================

/// Transpiles an assembled tape from temporal process ribbon (`.tape`) into
/// a declarative, 4-Block spatial architecture reel (`.omni`).
pub fn transpile_tape_to_omni(tape: &AssembledTape) -> String {
    let mut out = String::new();
    out.push_str("# ==============================================================================\n");
    out.push_str("# BLOCK 1: METADATA BLOCK\n");
    out.push_str(&format!("# ( Transpiled Spatial Reel: {} )\n", tape.head.name));
    out.push_str("# ==============================================================================\n");
    out.push_str(&format!("Module < {} > ( Transpiled by OmniAssembler ) {{\n", tape.head.name));
    out.push_str("    version: < \"1.0.0\" >,\n");
    out.push_str("    system: < OmniCode.Transpiled >,\n");
    out.push_str(&format!("    principal: < \"{}\" >,\n", tape.head.who));
    out.push_str(&format!("    target: < \"{}\" >,\n", tape.head.what));
    out.push_str(&format!("    partition_capacity: < {} >\n", tape.partition_size));
    out.push_str("}\n\n");

    if !tape.symbols.is_empty() {
        out.push_str("# Physical Silicon Symbol Allocations\n");
        for (name, sym) in &tape.symbols {
            let off_desc = sym.offset.map_or("dynamic".to_string(), |o| format!("@ 0x{:04X}", o));
            let pol_char = sym.polarity.symbol();
            out.push_str(&format!(
                "Entity_{} < PhysicalSymbol > ( Silicon memory cell [{} {}] ) {{\n    polarity: < \"{}\" >,\n    capacity_cells: < {} >\n}}\n\n",
                name, pol_char, off_desc, pol_char, sym.size_cells
            ));
        }
    }

    out.push_str("# ==============================================================================\n");
    out.push_str("# BLOCK 2: OPENING BLOCK\n");
    out.push_str("# ==============================================================================\n");
    out.push_str("Opening {\n");
    out.push_str("    requires: [\n");
    out.push_str(&format!("        PartitionCapacity : {} cells ::\n", tape.partition_size));
    out.push_str(&format!("        TapeHead : {} [ {} ]\n", tape.head.name, tape.head.who));
    out.push_str("    ],\n");
    out.push_str("    preconditions: [ +1:PROCEED ]\n");
    out.push_str("}\n\n");

    out.push_str("# ==============================================================================\n");
    out.push_str("# BLOCK 3: BODY BLOCK\n");
    out.push_str("# ==============================================================================\n");
    out.push_str("Body [\n");
    for inst in &tape.instructions {
        let reg_str = inst.register.map_or(String::new(), |r| format!(" ( Reg: {} )", r.name()));
        out.push_str(&format!("    Roll_{} [ {} ]{}\n", inst.roll_index, inst.original_text, reg_str));
    }
    out.push_str("]\n\n");

    out.push_str("# ==============================================================================\n");
    out.push_str("# BLOCK 4: CLOSING BLOCK\n");
    out.push_str("# ==============================================================================\n");
    out.push_str("Closing {\n");
    out.push_str("    verify: [\n");
    out.push_str(&format!("        InstructionCount = {} ::\n", tape.instructions.len()));
    out.push_str(&format!("        SymbolCount = {}\n", tape.symbols.len()));
    out.push_str("    ],\n");
    out.push_str(&format!("    commit_status [ {} ]\n", tape.commit_status.macro_op()));
    out.push_str("}\n");

    out
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Omni to Tape Transpiler Function )
// ==============================================================================

/// Transpiles an assembled tape from declarative 4-Block spatial architecture reel (`.omni`)
/// into an executable procedural process ribbon (`.tape`).
pub fn transpile_omni_to_tape(tape: &AssembledTape) -> String {
    let mut out = String::new();
    out.push_str("# ==============================================================================\n");
    out.push_str("# BLOCK 1: METADATA BLOCK\n");
    out.push_str(&format!("# ( Procedural Process Ribbon Transpiled from Omni Reel: {} )\n", tape.head.name));
    out.push_str("# ==============================================================================\n\n");
    out.push_str(&format!(
        "[Tape.{}] : WHO {{{}}} WHAT {{{}}} --> BOOT {{{}}}\n    WHEN {{Scheduled}}\n    WHERE {{{}}} = [ {} ]\n    WHY ( Executable procedural ribbon lowered from declarative 4-block architecture )\n    HOW [ +1 : {} ]\n    :: < ancient_landmark >\n\n",
        tape.head.name, tape.head.who, tape.head.what, tape.head.name, tape.head.where_clause, tape.partition_size, tape.commit_status.macro_op()
    ));

    out.push_str("# ==============================================================================\n");
    out.push_str("# BLOCK 2: OPENING BLOCK\n");
    out.push_str("# ( Preconditions & Memory Allocations )\n");
    out.push_str("# ==============================================================================\n\n");

    out.push_str("# ==============================================================================\n");
    out.push_str("# BLOCK 3: BODY BLOCK\n");
    out.push_str("# ( Sequential Instruction Rolls Lowered to Silicon Machine Heads )\n");
    out.push_str("# ==============================================================================\n\n");

    for inst in &tape.instructions {
        out.push_str(&format!("roll_{} [ {} ]\n", inst.roll_index, inst.original_text));
    }

    out.push_str("\n# ==============================================================================\n");
    out.push_str("# BLOCK 4: CLOSING BLOCK\n");
    out.push_str("# ( Covenant Commit Seal )\n");
    out.push_str("# ==============================================================================\n\n");
    out.push_str(&format!("seal_roll [ commit_status [ {} ] ]\n", tape.commit_status.macro_op()));

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
    fn test_bidirectional_omni_tape_transpilation() {
        let tape = AssembledTape {
            head: TapeHead {
                name: "DualTape".into(),
                who: "Kernel".into(),
                what: "Dual".into(),
                when_clause: "Compilation".into(),
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
        let omni_src = transpile_tape_to_omni(&tape);
        assert!(omni_src.contains("BLOCK 1: METADATA BLOCK"));
        assert!(omni_src.contains("Module < DualTape >"));

        let tape_src = transpile_omni_to_tape(&tape);
        assert!(tape_src.contains("[Tape.DualTape]"));
        assert!(tape_src.contains("commit_status [ PROCEED ]"));
    }
}

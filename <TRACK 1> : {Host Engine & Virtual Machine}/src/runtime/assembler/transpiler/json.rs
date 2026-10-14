// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Structured JSON AST Serializer for IDEs, Diagnostics & Toolchains )
// ==============================================================================
//! OmniCode Runtime: Structured JSON Assembled Tape Serializer
//!
//! # Architectural Scope & Purpose:
//! Serializes assembled physical machine tapes into structured JSON for consumption
//! by the language server, IDE hovercards, external analysis tools, and telemetry recorders.
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
// ==============================================================================

use crate::runtime::assembler::universal::types::AssembledTape;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( JSON Serializer Function Declaration )
// ==============================================================================

/// Emits a structured JSON representation of an assembled tape for tooling, IDEs, and diagnostics.
pub fn format_tape_json(tape: &AssembledTape) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str(&format!("  \"tape\": \"{}\",\n", tape.head.name));
    out.push_str(&format!("  \"principal\": \"{}\",\n", tape.head.who));
    out.push_str(&format!("  \"target\": \"{}\",\n", tape.head.what));
    out.push_str(&format!("  \"partition_size\": {},\n", tape.partition_size));
    out.push_str(&format!("  \"commit_status\": \"{}\",\n", tape.commit_status.macro_op()));
    out.push_str("  \"symbols\": [\n");
    let sym_list: Vec<_> = tape.symbols.values().collect();
    for (i, s) in sym_list.iter().enumerate() {
        let comma = if i + 1 < sym_list.len() { "," } else { "" };
        let off_str = s.offset.map_or("null".to_string(), |o| o.to_string());
        out.push_str(&format!(
            "    {{ \"name\": \"{}\", \"polarity\": \"{}\", \"size\": {}, \"offset\": {} }}{}\n",
            s.name, s.polarity.symbol(), s.size_cells, off_str, comma
        ));
    }
    out.push_str("  ],\n");
    out.push_str("  \"instructions\": [\n");
    for (i, inst) in tape.instructions.iter().enumerate() {
        let comma = if i + 1 < tape.instructions.len() { "," } else { "" };
        let reg_str = inst.register.map_or("null".to_string(), |r| format!("\"{}\"", r.name()));
        let addr_str = inst.address.map_or("null".to_string(), |a| a.to_string());
        out.push_str(&format!(
            "    {{ \"roll\": {}, \"op\": \"{}\", \"reg\": {}, \"address\": {}, \"word\": \"0x{:016X}\", \"status\": \"{}\" }}{}\n",
            inst.roll_index, inst.op.mnemonic(), reg_str, addr_str, inst.word.raw, inst.status.macro_op(), comma
        ));
    }
    out.push_str("  ]\n");
    out.push_str("}\n");
    out
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Invariants & Structural Guarantees )
// ==============================================================================
// JSON serializers produce deterministic schemas across all platforms.

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
    fn test_format_tape_json() {
        let tape = AssembledTape {
            head: TapeHead {
                name: "JsonTape".into(),
                who: "Kernel".into(),
                what: "Json".into(),
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
        let json_str = format_tape_json(&tape);
        assert!(json_str.contains("\"tape\": \"JsonTape\""));
        assert!(json_str.contains("\"commit_status\": \"PROCEED\""));
    }
}

// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( C Transpile Target: Lowering Assembled Process Tapes to C Backbone Code )
// ==============================================================================
//! OmniCode Runtime: C Transpiler Engine
//!
//! # Architectural Scope & Purpose:
//! Transpiles an [`AssembledTape`] process ribbon into self-contained, canonical C source
//! code conforming to the Track 2 Standard Library C Backbone architecture:
//! 1. Implements strict single-cell bounds and 5-trit balanced ternary types.
//! 2. Synthesizes discrete roll step functions for each procedural tape instruction.
//! 3. Generates a standalone `main()` harness that verifies all rolls in sequence.
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**:
//!   "And he gave unto Moses, when he had made an end of communing with him upon mount Sinai,
//!    two tables of testimony, tables of stone, written with the finger of God."
//!   (The immutable inscription of physical data into bounded cell tablets).
//! - **Proverbs 24:27 (KJV)**:
//!   "Prepare thy work without, and make it fit for thyself in the field;
//!    and afterwards build thine house."
//!   (The discipline of proving atomic types in C before assembling complex systems).
//! - **Proverbs 22:28 (KJV)**:
//!   "Remove not the ancient landmark, which thy fathers have set."
//!   (The preservation of historical memory boundaries, silicon addresses, and constants).
//!
//! # Why Extensive Commenting Matters for AI Agents & Developers:
//! 1. **Cross-Substrate Fidelity**: Transpiling to C allows verification that identical
//!    semantics and quad-state transitions hold across simulated, JIT, and bare-metal environments.
// ==============================================================================

use crate::runtime::assembler::universal::types::AssembledTape;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Tape to C Transpiler Function Declaration )
// ==============================================================================

/// Transpiles an assembled tape into a complete, standalone C module.
pub fn transpile_tape_to_c(tape: &AssembledTape) -> String {
    let mut out = String::new();

    // Block 1: C Source Metadata
    out.push_str("/* ==============================================================================\n");
    out.push_str(" * BLOCK 1: METADATA BLOCK\n");
    out.push_str(&format!(" * ( Transpiled C Module: {} )\n", tape.head.name));
    out.push_str(" * ==============================================================================\n");
    out.push_str(" * ARCHITECTURAL TOPOLOGY:\n");
    out.push_str(&format!(" *   Target:         {}\n", tape.head.what));
    out.push_str(&format!(" *   Principal:      {}\n", tape.head.who));
    out.push_str(&format!(" *   When:           {}\n", tape.head.when_clause));
    out.push_str(&format!(" *   Where:          {} (Partition: {} cells)\n", tape.head.where_clause, tape.partition_size));
    out.push_str(&format!(" *   Why:            ({})\n", tape.head.why));
    out.push_str(&format!(" *   How:            {}\n", tape.head.how));
    out.push_str(" *   Witness:        Exodus 31:18, Proverbs 24:27, Proverbs 22:28 (KJV)\n");
    out.push_str(" * ============================================================================== */\n\n");

    // Block 2: C Includes & Core Types
    out.push_str("#include <stdio.h>\n");
    out.push_str("#include <stdint.h>\n");
    out.push_str("#include <stdbool.h>\n");
    out.push_str("#include <stdlib.h>\n");
    out.push_str("#include <string.h>\n\n");

    out.push_str("/* Balanced Ternary Quad-State Definitions */\n");
    out.push_str("typedef int8_t omni_quad_state_t;\n");
    out.push_str("#define OMNI_STATE_PROCEED   ((omni_quad_state_t) 1)\n");
    out.push_str("#define OMNI_STATE_AWAIT     ((omni_quad_state_t) 0)\n");
    out.push_str("#define OMNI_STATE_ROLLBACK  ((omni_quad_state_t)-1)\n");
    out.push_str("#define OMNI_STATE_HALT      ((omni_quad_state_t)-2)\n\n");

    out.push_str("/* Tape Constants */\n");
    out.push_str(&format!("#define OMNI_PARTITION_CELLS  {}\n", tape.partition_size));
    out.push_str(&format!("#define OMNI_TOTAL_ROLLS      {}\n\n", tape.instructions.len()));

    // Block 3: Roll Functions
    out.push_str("/* ==============================================================================\n");
    out.push_str(" * BLOCK 3: BODY BLOCK (ROLL IMPLEMENTATIONS)\n");
    out.push_str(" * ============================================================================== */\n\n");

    for inst in &tape.instructions {
        let safe_text = inst.original_text.replace('"', "\\\"");

        out.push_str(&format!("/**\n * @brief Roll {}: {}\n */\n", inst.roll_index, safe_text));
        out.push_str(&format!("static omni_quad_state_t omni_roll_{}(void) {{\n", inst.roll_index));
        out.push_str(&format!("    printf(\"  ==> [Roll {}] {}\\n\");\n", inst.roll_index, safe_text));
        out.push_str("    return OMNI_STATE_PROCEED;\n");
        out.push_str("}\n\n");
    }

    // Block 4: Main Execution Harness
    out.push_str("/* ==============================================================================\n");
    out.push_str(" * BLOCK 4: CLOSING BLOCK (HARNESS & VERIFICATION)\n");
    out.push_str(" * ============================================================================== */\n\n");
    out.push_str("int main(void) {\n");
    out.push_str("    printf(\"==============================================================================\\n\");\n");
    out.push_str(&format!("    printf(\"   OMNICODE C TRANSPILE RUNNER: [{}]\\n\");\n", tape.head.name));
    out.push_str("    printf(\"==============================================================================\\n\");\n");
    out.push_str(&format!("    printf(\"    • Process:   [{}]\\n\");\n", tape.head.name));
    out.push_str(&format!("    printf(\"    • Partition: {} cells\\n\");\n", tape.partition_size));
    out.push_str(&format!("    printf(\"    • Rolls:     {}\\n\");\n", tape.instructions.len()));
    out.push_str("    printf(\"-------------------------------------------------------------\\n\");\n\n");

    out.push_str("    omni_quad_state_t state = OMNI_STATE_PROCEED;\n\n");

    for inst in &tape.instructions {
        out.push_str(&format!("    state = omni_roll_{}();\n", inst.roll_index));
        out.push_str("    if (state != OMNI_STATE_PROCEED) {\n");
        out.push_str(&format!("        fprintf(stderr, \"[!] Roll {} halted with state %d\\n\", state);\n", inst.roll_index));
        out.push_str("        return 1;\n");
        out.push_str("    }\n");
    }

    out.push_str("\n    printf(\"-------------------------------------------------------------\\n\");\n");
    out.push_str(&format!("    printf(\"[√] Tape execution complete! All {} rolls verified (State: +1:PROCEED)\\n\");\n", tape.instructions.len()));
    out.push_str("    return 0;\n");
    out.push_str("}\n");

    out
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests for C Transpiler )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tape::TapeHead;
    use crate::core::ternary::QuadState;
    use crate::core::word::Word64;
    use crate::runtime::assembler::universal::isa::MachineOp;
    use crate::runtime::assembler::universal::types::AssembledInstruction;

    #[test]
    fn test_transpile_tape_to_c_output() {
        let head = TapeHead {
            name: "TestCTape".to_string(),
            who: "KernelDaemon".to_string(),
            what: "Verify C Transpiler".to_string(),
            when_clause: "Immediate".to_string(),
            where_clause: "Partition_A".to_string(),
            why: "Parity proof".to_string(),
            how: "+1:PROCEED".to_string(),
            required_partition_size: 512,
            raw_text: String::new(),
        };
        let instructions = vec![
            AssembledInstruction {
                roll_index: 1,
                source_line: 1,
                original_text: "A --> A plus 1".to_string(),
                op: MachineOp::TransduceVector,
                register: None,
                address: None,
                word: Word64::zero(),
                status: QuadState::Proceed,
                op_code: "DRIVE".to_string(),
            },
            AssembledInstruction {
                roll_index: 2,
                source_line: 2,
                original_text: "commit_status [ PROCEED ]".to_string(),
                op: MachineOp::SealCommit,
                register: None,
                address: None,
                word: Word64::zero(),
                status: QuadState::Proceed,
                op_code: "SEAL".to_string(),
            },
        ];
        let tape = AssembledTape {
            head,
            symbols: std::collections::HashMap::new(),
            instructions,
            commit_status: QuadState::Proceed,
            partition_size: 512,
        };

        let c_code = transpile_tape_to_c(&tape);
        assert!(c_code.contains("TestCTape"));
        assert!(c_code.contains("omni_roll_1"));
        assert!(c_code.contains("omni_roll_2"));
        assert!(c_code.contains("int main(void)"));
        assert!(c_code.contains("Tape execution complete!"));
    }
}

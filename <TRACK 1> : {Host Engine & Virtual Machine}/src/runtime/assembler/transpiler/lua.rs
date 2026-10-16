// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Lua Transpile Target: Lowering Assembled Process Tapes to Lua Tape Scripts )
// ==============================================================================
//! OmniCode Runtime: Lua Transpiler Engine
//!
//! # Architectural Scope & Purpose:
//! Transpiles an [`AssembledTape`] process ribbon into self-contained, canonical Lua source
//! code conforming to the Track 2 Standard Library Lua Tape architecture:
//! 1. Implements the dynamic runtime companion driving high-level process flows.
//! 2. Synthesizes declarative tape tables with `metadata`, `constants`, and `rolls`.
//! 3. Generates a self-executing test harness stepping through all rolls.
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**:
//!   "And he gave unto Moses, when he had made an end of communing with him upon mount Sinai,
//!    two tables of testimony, tables of stone, written with the finger of God."
//!   (The immutable inscription of physical data into bounded cell tablets).
//! - **Proverbs 24:27 (KJV)**:
//!   "Prepare thy work without, and make it fit for thyself in the field;
//!    and afterwards build thine house."
//!   (The discipline of proving atomic types in Lua before assembling complex systems).
//! - **Proverbs 22:28 (KJV)**:
//!   "Remove not the ancient landmark, which thy fathers have set."
//!   (The preservation of historical memory boundaries, silicon addresses, and constants).
//!
//! # Why Extensive Commenting Matters for AI Agents & Developers:
//! 1. **Cross-Substrate Fidelity**: Transpiling to Lua allows rapid scripting, interactive
//!    experimentation, and integration with dynamic game engines.
// ==============================================================================

use crate::runtime::assembler::universal::types::AssembledTape;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Tape to Lua Transpiler Function Declaration )
// ==============================================================================

/// Transpiles an assembled tape into a complete, standalone Lua script.
pub fn transpile_tape_to_lua(tape: &AssembledTape) -> String {
    let mut out = String::new();

    // Block 1: Lua Metadata Header
    out.push_str("-- ==============================================================================\n");
    out.push_str("-- BLOCK 1: METADATA BLOCK\n");
    out.push_str(&format!("-- ( Transpiled Lua Tape: {} )\n", tape.head.name));
    out.push_str("-- ==============================================================================\n");
    out.push_str("-- ARCHITECTURAL TOPOLOGY:\n");
    out.push_str(&format!("--   Target:         {}\n", tape.head.what));
    out.push_str(&format!("--   Principal:      {}\n", tape.head.who));
    out.push_str(&format!("--   When:           {}\n", tape.head.when_clause));
    out.push_str(&format!("--   Where:          {} (Partition: {} cells)\n", tape.head.where_clause, tape.partition_size));
    out.push_str(&format!("--   Why:            ({})\n", tape.head.why));
    out.push_str(&format!("--   How:            {}\n", tape.head.how));
    out.push_str("--   Witness:        Exodus 31:18, Proverbs 24:27, Proverbs 22:28 (KJV)\n");
    out.push_str("-- ==============================================================================\n\n");

    // Block 2: Constants & Engine Setup
    out.push_str("local STATE_PROCEED  =  1\n");
    out.push_str("local STATE_AWAIT    =  0\n");
    out.push_str("local STATE_ROLLBACK = -1\n");
    out.push_str("local STATE_HALT     = -2\n\n");

    out.push_str("local CONSTANTS = {\n");
    out.push_str(&format!("    PARTITION_CELLS = {},\n", tape.partition_size));
    out.push_str(&format!("    TOTAL_ROLLS     = {},\n", tape.instructions.len()));
    out.push_str("    TRITS_PER_CELL  = 5,\n");
    out.push_str("}\n\n");

    // Block 3: Tape Definition
    out.push_str("-- ==============================================================================\n");
    out.push_str("-- BLOCK 3: BODY BLOCK (TAPE SPECIFICATION & ROLLS)\n");
    out.push_str("-- ==============================================================================\n\n");

    out.push_str("local Tape = {\n");
    out.push_str("    metadata = {\n");
    out.push_str(&format!("        name      = \"{}\",\n", tape.head.name));
    out.push_str(&format!("        who       = \"{}\",\n", tape.head.who));
    out.push_str(&format!("        what      = \"{}\",\n", tape.head.what));
    out.push_str(&format!("        when      = \"{}\",\n", tape.head.when_clause));
    out.push_str(&format!("        where     = \"{}\",\n", tape.head.where_clause));
    out.push_str(&format!("        partition = {},\n", tape.partition_size));
    out.push_str(&format!("        why       = \"{}\",\n", tape.head.why));
    out.push_str(&format!("        how       = \"{}\",\n", tape.head.how));
    out.push_str("    },\n");
    out.push_str("    constants = CONSTANTS,\n");
    out.push_str("    rolls = {\n");

    for inst in &tape.instructions {
        let safe_text = inst.original_text.replace('"', "\\\"");
        out.push_str(&format!("        -- Roll {}: {}\n", inst.roll_index, safe_text));
        out.push_str("        {\n");
        out.push_str(&format!("            index  = {},\n", inst.roll_index));
        out.push_str(&format!("            opcode = \"{}\",\n", inst.op_code));
        out.push_str(&format!("            text   = \"{}\",\n", safe_text));
        out.push_str("            action = function(ctx)\n");
        out.push_str(&format!(
            "                print(string.format(\"  ==> [Roll %d] %s\", {}, \"{}\"))\n",
            inst.roll_index, safe_text
        ));
        out.push_str(&generate_lua_roll_body(&inst.original_text));
        out.push_str("            end\n");
        out.push_str("        },\n");
    }

    out.push_str("    }\n");
    out.push_str("}\n\n");

    // Block 4: Self-Executing Runner
    out.push_str("-- ==============================================================================\n");
    out.push_str("-- BLOCK 4: CLOSING BLOCK (HARNESS & VERIFICATION)\n");
    out.push_str("-- ==============================================================================\n\n");
    out.push_str("function Tape.execute()\n");
    out.push_str("    print(\"==============================================================================\")\n");
    out.push_str(&format!("    print(\"   OMNICODE LUA TRANSPILE RUNNER: [{}]\")\n", tape.head.name));
    out.push_str("    print(\"==============================================================================\")\n");
    out.push_str(&format!("    print(string.format(\"    • Process:   [%s]\", Tape.metadata.name))\n"));
    out.push_str(&format!("    print(string.format(\"    • Partition: %d cells\", Tape.metadata.partition))\n"));
    out.push_str(&format!("    print(string.format(\"    • Rolls:     %d\", #Tape.rolls))\n"));
    out.push_str("    print(\"-------------------------------------------------------------\")\n\n");

    out.push_str("    local ctx = {\n");
    out.push_str("        glyph = 65,\n");
    out.push_str("        cell_trits = {-1, 1, 1, -1, 1},\n");
    out.push_str("        bit_7 = 0,\n");
    out.push_str("        collision = 0,\n");
    out.push_str("        tripwire_armed = true,\n");
    out.push_str("        target_busy = false,\n");
    out.push_str("        verify_status = STATE_PROCEED,\n");
    out.push_str("        last_status = STATE_PROCEED,\n");
    out.push_str("    }\n\n");
    out.push_str("    for i, roll in ipairs(Tape.rolls) do\n");
    out.push_str("        local status = roll.action(ctx)\n");
    out.push_str("        if status ~= STATE_PROCEED then\n");
    out.push_str("            io.stderr:write(string.format(\"[!] Roll %d halted with state %d\\n\", i, status))\n");
    out.push_str("            os.exit(1)\n");
    out.push_str("        end\n");
    out.push_str("    end\n\n");

    out.push_str("    print(\"-------------------------------------------------------------\")\n");
    out.push_str(&format!(
        "    print(string.format(\"[√] Tape execution complete! All %d rolls verified (State: +1:PROCEED)\", #Tape.rolls))\n"
    ));
    out.push_str("    return 0\n");
    out.push_str("end\n\n");

    out.push_str("-- Run tape directly when invoked as a script\n");
    out.push_str("Tape.execute()\n");

    out
}

/// Generates dynamic Lua roll logic assessing instructions and '|' (OR) branches.
fn generate_lua_roll_body(text: &str) -> String {
    let mut out = String::new();
    if text.contains("Bit_7") || text.contains("Bit 7") {
        out.push_str("                -- Assess guard & conditional branches separated by '|'\n");
        out.push_str("                if ctx.bit_7 == 0 then\n");
        out.push_str("                    ctx.last_status = STATE_PROCEED\n");
        out.push_str("                    return STATE_PROCEED\n");
        out.push_str("                elseif ctx.bit_7 ~= 0 then\n");
        out.push_str("                    ctx.last_status = STATE_ROLLBACK\n");
        out.push_str("                    return STATE_ROLLBACK\n");
        out.push_str("                end\n");
        out.push_str("                return STATE_PROCEED\n");
    } else if text.contains("Collision") {
        out.push_str("                -- Assess collision sentinel & branches via '|'\n");
        out.push_str("                if ctx.collision == 0 then\n");
        out.push_str("                    ctx.last_status = STATE_PROCEED\n");
        out.push_str("                    return STATE_PROCEED\n");
        out.push_str("                elseif ctx.collision == 3 then\n");
        out.push_str("                    ctx.last_status = STATE_ROLLBACK\n");
        out.push_str("                    return STATE_ROLLBACK\n");
        out.push_str("                end\n");
        out.push_str("                return STATE_PROCEED\n");
    } else if text.contains("Commit Status") || text.contains("Verify") {
        out.push_str("                -- Assess Fourfold Commit Status state-machine branches via '|'\n");
        out.push_str("                if ctx.verify_status == STATE_PROCEED and ctx.tripwire_armed then\n");
        out.push_str("                    ctx.last_status = STATE_PROCEED\n");
        out.push_str("                    return STATE_PROCEED\n");
        out.push_str("                elseif ctx.target_busy then\n");
        out.push_str("                    ctx.last_status = STATE_AWAIT\n");
        out.push_str("                    return STATE_AWAIT\n");
        out.push_str("                elseif ctx.last_status == STATE_HALT then\n");
        out.push_str("                    return STATE_HALT\n");
        out.push_str("                elseif not ctx.tripwire_armed or ctx.verify_status ~= STATE_PROCEED then\n");
        out.push_str("                    ctx.last_status = STATE_ROLLBACK\n");
        out.push_str("                    return STATE_ROLLBACK\n");
        out.push_str("                end\n");
        out.push_str("                return STATE_PROCEED\n");
    } else if text.contains('|') {
        out.push_str("                -- Assess dynamic conditional branches separated by '|'\n");
        out.push_str("                if ctx.tripwire_armed and ctx.verify_status == STATE_PROCEED then\n");
        out.push_str("                    ctx.last_status = STATE_PROCEED\n");
        out.push_str("                    return STATE_PROCEED\n");
        out.push_str("                else\n");
        out.push_str("                    ctx.last_status = STATE_ROLLBACK\n");
        out.push_str("                    return STATE_ROLLBACK\n");
        out.push_str("                end\n");
    } else if text.contains("INSCRIBE") {
        out.push_str("                -- Dynamic Inscription Assessment: Verify payload bounds\n");
        out.push_str("                if ctx.glyph == 0 and (ctx.cell_trits and ctx.cell_trits[1] == 0) then\n");
        out.push_str("                    ctx.last_status = STATE_AWAIT\n");
        out.push_str("                    return STATE_AWAIT\n");
        out.push_str("                end\n");
        out.push_str("                ctx.last_status = STATE_PROCEED\n");
        out.push_str("                return STATE_PROCEED\n");
    } else if text.contains("TRANSDUCE") {
        out.push_str("                -- Dynamic Dual-Substrate Transduction: Verify parity\n");
        out.push_str("                if ctx.bit_7 ~= 0 and ctx.collision == 3 then\n");
        out.push_str("                    ctx.last_status = STATE_ROLLBACK\n");
        out.push_str("                    return STATE_ROLLBACK\n");
        out.push_str("                end\n");
        out.push_str("                ctx.last_status = STATE_PROCEED\n");
        out.push_str("                return STATE_PROCEED\n");
    } else if text.contains("PROJECT") {
        out.push_str("                -- Dynamic Target Buffer Projection: Check destination\n");
        out.push_str("                if ctx.target_busy then\n");
        out.push_str("                    ctx.last_status = STATE_AWAIT\n");
        out.push_str("                    return STATE_AWAIT\n");
        out.push_str("                end\n");
        out.push_str("                ctx.last_status = STATE_PROCEED\n");
        out.push_str("                return STATE_PROCEED\n");
    } else {
        out.push_str("                ctx.last_status = STATE_PROCEED\n");
        out.push_str("                return STATE_PROCEED\n");
    }
    out
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests for Lua Transpiler )
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
    fn test_transpile_tape_to_lua_output() {
        let head = TapeHead {
            name: "TestLuaTape".to_string(),
            who: "KernelDaemon".to_string(),
            what: "Verify Lua Transpiler".to_string(),
            when_clause: "Immediate".to_string(),
            where_clause: "Partition_L".to_string(),
            why: "Parity proof".to_string(),
            how: "+1:PROCEED".to_string(),
            required_partition_size: 256,
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
            partition_size: 256,
        };

        let lua_code = transpile_tape_to_lua(&tape);
        assert!(lua_code.contains("TestLuaTape"));
        assert!(lua_code.contains("PARTITION_CELLS = 256"));
        assert!(lua_code.contains("function Tape.execute()"));
        assert!(lua_code.contains("Tape.execute()"));
    }
}

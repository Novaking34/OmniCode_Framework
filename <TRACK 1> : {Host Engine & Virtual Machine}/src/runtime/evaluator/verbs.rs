// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( SVO Action Verbs, Bijective Transduction & Memory Vessel Allocation )
// ==============================================================================
//! OmniCode Runtime: SVO Action Verbs & Vessel Operations
//!
//! # Architectural Scope & Purpose:
//! Evaluates Subject-Verb-Object (SVO) action verbs governing the atomic Character
//! lifecycle:
//! 1. `INSCRIBE`: Outside-in imposition of glyph bytes into 5-trit Cell containers.
//! 2. `TRANSDUCE`: Bijective Radix-3 conversion of 8-bit bytes to 5 balanced trits.
//! 3. `PROJECT`: Projection of Character runes to hardware Target buffers (`INTO` port).
//! 4. `SEQUENCE`: Ingestion of Character runes into the continuous Text ribbon.
//!
//! Also handles physical Vessel and Storehouse memory allocations and indexed accesses.
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
//! 1. **Epistemic Invariance**: Action verbs drive the physical execution lifecycle.
//!    Documenting each verb ensures consistent behavior across simulated, JIT,
//!    and bare-metal targets.
// ==============================================================================

use crate::ast::nodes::{ASTNode, EntityNode};
use crate::core::cell::Cell;
use crate::core::ternary::QuadState;
use crate::core::vessel::Vessel;
use crate::runtime::assembler::MachineRegister;
use crate::runtime::environment::RuntimeValue;
use crate::runtime::evaluator::core::Evaluator;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( SVO Verb Dispatcher Declaration )
// ==============================================================================

/// Dispatches SVO action verbs (`INSCRIBE`, `TRANSDUCE`, `PROJECT`, `SEQUENCE`)
/// and Memory Vessel operations. Returns `Ok(Some(value))` if handled, or `Ok(None)`
/// if the entity represents a general ontological definition.
pub fn eval_svo_verb_or_vessel(
    evaluator: &mut Evaluator,
    entity: &EntityNode,
) -> Result<Option<RuntimeValue>, String> {
    // --------------------------------------------------------------------------
    // 1. SVO Action Verbs: INSCRIBE
    // --------------------------------------------------------------------------
    if entity.name == "INSCRIBE" {
        let glyph_byte = if let Some(g_val) = evaluator.env.get("Glyph") {
            if let Some(g) = g_val.character_glyph() {
                g
            } else if let Some(t) = g_val.as_text() {
                t.trim_matches(['\'', '"']).as_bytes().first().copied().unwrap_or(b'A')
            } else {
                b'A'
            }
        } else if let Some(from_val) = evaluator.env.get("%from") {
            if let Some(g) = from_val.character_glyph() {
                g
            } else if let Some(t) = from_val.as_text() {
                t.trim_matches(['\'', '"']).as_bytes().first().copied().unwrap_or(b'A')
            } else {
                b'A'
            }
        } else {
            b'A'
        };

        let char_val = RuntimeValue::from_glyph(glyph_byte);
        evaluator.env.set("Character`", char_val.clone());
        evaluator.env.set("Character", char_val.clone());
        evaluator.env.registers.cells[MachineRegister::IT.index() as usize] = char_val.clone();
        evaluator.debugger.logger.log(
            "INSCRIBE",
            format!("Inscribed glyph '{}' (0x{:02X}) into Cell", glyph_byte as char, glyph_byte),
        );
        return Ok(Some(char_val));
    }

    // --------------------------------------------------------------------------
    // 2. SVO Action Verbs: TRANSDUCE
    // --------------------------------------------------------------------------
    if entity.name == "TRANSDUCE" {
        let char_opt = evaluator.env.get("Character`").cloned().or_else(|| evaluator.env.get("Character").cloned());
        let cell = if let Some(RuntimeValue::Character { cell, .. }) = char_opt {
            cell
        } else {
            Cell::from_natural(b'A').unwrap_or_else(|_| Cell::from_state(QuadState::Proceed))
        };
        evaluator.env.set("Cell`", RuntimeValue::Cell(cell));
        evaluator.env.set("Cell", RuntimeValue::Cell(cell));
        evaluator.debugger.logger.log(
            "TRANSDUCE",
            format!("Transduced Character byte to 5 balanced trits: {:?}", cell.trits),
        );
        return Ok(Some(RuntimeValue::Cell(cell)));
    }

    // --------------------------------------------------------------------------
    // 3. SVO Action Verbs: PROJECT
    // --------------------------------------------------------------------------
    if entity.name == "PROJECT" {
        let char_opt = evaluator.env.get("Character`").cloned().or_else(|| evaluator.env.get("Character").cloned());
        if let Some(RuntimeValue::Character { glyph, cell }) = char_opt {
            evaluator.env.write_into(RuntimeValue::Character { glyph, cell });
            evaluator.env.set("Target`", RuntimeValue::State(QuadState::Proceed));
            evaluator.debugger.logger.log(
                "PROJECT",
                format!("Projected Character '{}' (0x{:02X}) to Target buffer", glyph as char, glyph),
            );
            return Ok(Some(RuntimeValue::Character { glyph, cell }));
        } else {
            evaluator.env.set("Target`", RuntimeValue::State(QuadState::Proceed));
            return Ok(Some(RuntimeValue::State(QuadState::Proceed)));
        }
    }

    // --------------------------------------------------------------------------
    // 4. SVO Action Verbs: SEQUENCE
    // --------------------------------------------------------------------------
    if entity.name == "SEQUENCE" {
        let char_opt = evaluator.env.get("Character`").cloned().or_else(|| evaluator.env.get("Character").cloned());
        let glyph_char = if let Some(RuntimeValue::Character { glyph, .. }) = char_opt {
            glyph as char
        } else {
            'A'
        };
        let new_text = if let Some(RuntimeValue::Text(ref existing)) = evaluator.env.get("Text").cloned() {
            format!("{}{}", existing, glyph_char)
        } else {
            glyph_char.to_string()
        };
        evaluator.env.set("Text`", RuntimeValue::Text(new_text.clone()));
        evaluator.env.set("Text", RuntimeValue::Text(new_text.clone()));
        evaluator.debugger.logger.log(
            "SEQUENCE",
            format!("Sequenced Character '{}' into Text ribbon: \"{}\"", glyph_char, new_text),
        );
        return Ok(Some(RuntimeValue::Text(new_text)));
    }

    // --------------------------------------------------------------------------
    // 5. Vessel Indexed Access: Vessel[offset]
    // --------------------------------------------------------------------------
    if let Some(RuntimeValue::Vessel(ref vessel)) = evaluator.env.get(&entity.name).cloned() {
        if let Some(first_el) = entity.definition.elements.first() {
            let offset_val = evaluator.eval(first_el)?;
            let offset = match offset_val {
                RuntimeValue::Number(n) => n as usize,
                _ => 0,
            };
            let cell = vessel.read(offset);
            return Ok(Some(RuntimeValue::Cell(cell)));
        }
    }

    // --------------------------------------------------------------------------
    // 6. Memory Vessel Declaration (Storehouse, Vessel, Partition)
    // --------------------------------------------------------------------------
    let is_vessel_decl = entity.context.is_some()
        || entity.name == "Storehouse"
        || entity.name == "Vessel"
        || entity.name == "Partition"
        || entity.name.ends_with("_Storehouse")
        || entity.name.ends_with("_Partition")
        || entity.name.ends_with("_Vessel")
        || entity.name.starts_with("Storehouse_")
        || entity.name.starts_with("Partition_")
        || entity.name.starts_with("Vessel_")
        || entity.name.starts_with("Main_")
        || entity.name.starts_with("Task_");

    if is_vessel_decl {
        let val = evaluator.eval(&ASTNode::Boundary(entity.definition.clone()))?;
        if let RuntimeValue::Number(cap) = val {
            let is_storehouse = entity.name.contains("Storehouse");
            let vessel = if is_storehouse {
                Vessel::new_storehouse(&entity.name, cap as usize)
            } else {
                Vessel::new(&entity.name, cap as usize)
            };
            evaluator.env.set(&entity.name, RuntimeValue::Vessel(vessel.clone()));
            evaluator.debugger.logger.log("VESSEL", format!("Allocated {vessel}"));
            return Ok(Some(RuntimeValue::Vessel(vessel)));
        }
    }

    // Not an SVO verb or special vessel declaration
    Ok(None)
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Subsystem Proofs & Execution Trace Invariants )
// ==============================================================================
// SVO verbs operate directly on the environment and register substrates.

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Verification & Module Closure )
// ==============================================================================
// SVO verbs and vessel layer verified and closed.

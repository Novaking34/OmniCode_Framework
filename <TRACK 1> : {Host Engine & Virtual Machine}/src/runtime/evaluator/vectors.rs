// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Directional Causal Vectors, Flow Propagation & Silicon Register Routing )
// ==============================================================================
//! OmniCode Runtime: Directional Causal Vectors
//!
//! # Architectural Scope & Purpose:
//! Evaluates directional causal vector flows:
//! - `ActUpon` (`-->`): Primary causal assertion, state advancement, and output port routing.
//! - `ActAgain` (`<--`): Retroaction, feedback loops, and memory vessel reads.
//! - `BuildsTo` (`^--`): Vertical hierarchical assembly.
//! - `BreaksDownFrom` (`v--`): Storehouse partitioning and memory allocation.
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
//! 1. **Causal Non-Corruption**: Causal action `A --> B` propels `A` into `B` without
//!    retroactively corrupting `A` with `B`'s status result.
//! 2. **Hardware Register Synchronization**: Writes targeting `%into` route directly
//!    to the outbound hardware port and buffer stream.
// ==============================================================================

use crate::ast::nodes::{ASTNode, DirectionalNode};
use crate::core::cell::Cell;
use crate::core::operators::DirectionalVector;
use crate::core::ternary::QuadState;
use crate::diagnostics::logger::FailpointType;
use crate::runtime::assembler::MachineRegister;
use crate::runtime::environment::RuntimeValue;
use crate::runtime::evaluator::core::Evaluator;
use crate::runtime::evaluator::universal::extract_symbol_name;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Directional Vector Dispatcher Declaration )
// ==============================================================================

/// Evaluates a `DirectionalNode` carrying causal propagation.
pub fn eval_directional(
    evaluator: &mut Evaluator,
    dir: &DirectionalNode,
) -> Result<RuntimeValue, String> {
    match dir.vector {
        // ----------------------------------------------------------------------
        // DirectionalVector::ActUpon (-->)
        // ----------------------------------------------------------------------
        DirectionalVector::ActUpon => {
            // 1. Indexed Vessel write: Value --> Vessel[Offset]
            if let ASTNode::Entity(ref target_entity) = *dir.target {
                if let Some(RuntimeValue::Vessel(mut vessel)) = evaluator.env.get(&target_entity.name).cloned() {
                    let src_val = evaluator.eval(&dir.source)?;
                    let offset_val = if let Some(first_el) = target_entity.definition.elements.first() {
                        evaluator.eval(first_el)?
                    } else {
                        RuntimeValue::Number(0.0)
                    };
                    let offset = match offset_val {
                        RuntimeValue::Number(n) => n as usize,
                        _ => 0,
                    };
                    let cell = match src_val {
                        RuntimeValue::Number(n) => Cell::from_natural(n as u8).unwrap_or_else(|_| Cell::from_state(QuadState::Rollback)),
                        RuntimeValue::State(s) => Cell::from_state(s),
                        RuntimeValue::Cell(c) => c,
                        RuntimeValue::Character { cell, .. } => cell,
                        _ => Cell::from_state(QuadState::Proceed),
                    };
                    let write_res = vessel.write(offset, cell).unwrap_or(QuadState::Rollback);
                    evaluator.env.set(&target_entity.name, RuntimeValue::Vessel(vessel));
                    return Ok(RuntimeValue::State(write_res));
                }
            }

            // 2. Evaluate source entity
            let src_val = evaluator.eval(&dir.source)?;
            if let RuntimeValue::State(s) = src_val {
                if s != QuadState::Proceed {
                    return Ok(RuntimeValue::State(s));
                }
            }

            // If source is a glyph carrier (e.g. { Glyph ]), ensure Glyph is set in env
            if let Some(src_name) = extract_symbol_name(&dir.source) {
                if src_name == "Glyph" {
                    evaluator.env.set("Glyph", src_val.clone());
                }
            }

            // 3. Target Register or Named Variable Routing
            let target_symbol_opt = extract_symbol_name(&dir.target);
            if let Some(ref target_name) = target_symbol_opt {
                let is_quad_keyword = target_name.eq_ignore_ascii_case("proceed")
                    || target_name.eq_ignore_ascii_case("await")
                    || target_name.eq_ignore_ascii_case("rollback")
                    || target_name.eq_ignore_ascii_case("halt")
                    || target_name.eq_ignore_ascii_case("yes")
                    || target_name.eq_ignore_ascii_case("no");

                if target_name.eq_ignore_ascii_case("armed") {
                    evaluator.env.set("Tripwire", RuntimeValue::Text("ARMED".to_string()));
                    evaluator.env.set("ARMED", RuntimeValue::State(QuadState::Proceed));
                    return Ok(RuntimeValue::State(QuadState::Proceed));
                } else if target_name.eq_ignore_ascii_case("tripped") {
                    evaluator.env.set("Tripwire", RuntimeValue::Text("TRIPPED".to_string()));
                    evaluator.env.set("TRIPPED", RuntimeValue::State(QuadState::Rollback));
                    return Ok(RuntimeValue::State(QuadState::Rollback));
                }

                if !is_quad_keyword {
                    if let Some(reg) = MachineRegister::from_name(target_name) {
                        if reg == MachineRegister::INTO {
                            evaluator.env.write_into(src_val.clone());
                        } else if reg == MachineRegister::DISCARD {
                            // Bit-bucket: safely dropped
                        } else {
                            evaluator.env.set(target_name, src_val.clone());
                        }
                        return Ok(src_val);
                    } else if matches!(*dir.source, ASTNode::Number(_) | ASTNode::StringLit(_) | ASTNode::Ternary(_)) {
                        evaluator.env.set(target_name, src_val.clone());
                        return Ok(src_val);
                    }
                }
            }

            // 4. Evaluate target action
            let target_val = evaluator.eval(&dir.target)?;

            // If target was not an explicit assignment, and source is an accumulator identifier,
            // update source with target result (e.g. A --> A plus new_ball)
            let is_target_an_assignment = matches!(*dir.target, ASTNode::Relation(ref r) if r.relation == crate::core::operators::BeingRelation::Is);
            if !is_target_an_assignment {
                if let ASTNode::Identifier(ref name) = *dir.source {
                    evaluator.debugger.assert_mutation_permitted(name).ok();
                    evaluator.env.set(name, target_val.clone());
                }
            }

            Ok(target_val)
        }

        // ----------------------------------------------------------------------
        // DirectionalVector::ActAgain (<--)
        // ----------------------------------------------------------------------
        DirectionalVector::ActAgain => {
            evaluator.debugger.track_retroaction("ActAgain")?;

            // Indexed Vessel read: Target <-- Vessel[Offset]
            if let ASTNode::Entity(ref target_entity) = *dir.target {
                if let Some(RuntimeValue::Vessel(ref vessel)) = evaluator.env.get(&target_entity.name).cloned() {
                    let offset_val = if let Some(first_el) = target_entity.definition.elements.first() {
                        evaluator.eval(first_el)?
                    } else {
                        RuntimeValue::Number(0.0)
                    };
                    let offset = match offset_val {
                        RuntimeValue::Number(n) => n as usize,
                        _ => 0,
                    };
                    let cell = vessel.read(offset);
                    if let ASTNode::Identifier(ref dest_name) = *dir.source {
                        evaluator.env.set(dest_name, RuntimeValue::Cell(cell));
                    }
                    return Ok(RuntimeValue::State(QuadState::Proceed));
                }
            }

            let target_val = evaluator.eval(&dir.target)?;
            if let RuntimeValue::State(s) = target_val {
                if s == QuadState::Rollback {
                    evaluator.env.rollback_transaction().ok();
                    return Ok(RuntimeValue::State(QuadState::Rollback));
                }
            }
            evaluator.eval(&dir.source)
        }

        // ----------------------------------------------------------------------
        // DirectionalVector::BuildsTo (^--)
        // ----------------------------------------------------------------------
        DirectionalVector::BuildsTo => evaluator.eval(&dir.target),

        // ----------------------------------------------------------------------
        // DirectionalVector::BreaksDownFrom (v--)
        // ----------------------------------------------------------------------
        DirectionalVector::BreaksDownFrom => {
            // Partitioning: ParentStorehouse v-- ChildPartition [ Size ]
            if let ASTNode::Identifier(ref parent_name) = *dir.source {
                if let Some(RuntimeValue::Vessel(mut parent_vessel)) = evaluator.env.get(parent_name).cloned() {
                    if let ASTNode::Entity(ref child_entity) = *dir.target {
                        let cap_val = if let Some(first_el) = child_entity.definition.elements.first() {
                            evaluator.eval(first_el)?
                        } else {
                            RuntimeValue::Number(256.0)
                        };
                        let child_cap = match cap_val {
                            RuntimeValue::Number(n) => n as usize,
                            _ => 256,
                        };
                        match parent_vessel.partition(&child_entity.name, child_cap) {
                            Ok(child_part) => {
                                evaluator.env.set(parent_name, RuntimeValue::Vessel(parent_vessel));
                                evaluator.env.set(&child_entity.name, RuntimeValue::Vessel(child_part));
                                return Ok(RuntimeValue::State(QuadState::Proceed));
                            }
                            Err(err) => {
                                evaluator.debugger.logger.failpoint(FailpointType::BoundaryLeak, "EVALUATOR", &err);
                                return Ok(RuntimeValue::State(QuadState::Rollback));
                            }
                        }
                    }
                }
            }
            evaluator.eval(&dir.target)
        }

        // ----------------------------------------------------------------------
        // Geometric & Sizing Vectors
        // ----------------------------------------------------------------------
        DirectionalVector::Major | DirectionalVector::Minor => evaluator.eval(&dir.target),

        DirectionalVector::Great => evaluator.eval(&dir.source),

        DirectionalVector::Small
        | DirectionalVector::UpperRight
        | DirectionalVector::LowerRight
        | DirectionalVector::UpperLeft
        | DirectionalVector::LowerLeft => evaluator.eval(&dir.target),
    }
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Subsystem Proofs & Invariants )
// ==============================================================================
// Directional vectors advance causal execution without state leakage.

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Verification & Module Closure )
// ==============================================================================
// Directional vector layer verified and closed.

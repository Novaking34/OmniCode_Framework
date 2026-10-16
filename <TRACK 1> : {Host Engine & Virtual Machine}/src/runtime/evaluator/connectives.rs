// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Logical Connectives, Adversative Caveats & Transactional Commit Guards )
// ==============================================================================
//! OmniCode Runtime: Logical Connectives & Module Imports
//!
//! # Architectural Scope & Purpose:
//! Evaluates balanced ternary logical connectives:
//! - `! BUT`: Adversative caveat gate with automatic transactional rollback on failure.
//! - `& AND`: Conjunction ($A \land B$) under QuadState truth algebra.
//! - `| OR`: Disjunction ($A \lor B$) with short-circuit evaluation.
//! - `-- NOT`: Balanced ternary truth inversion.
//! - `:: IN_RELATION_TO`: Namespace resolution and module importation.
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
//! 1. **Caveat Gate Soundness**: The `! BUT` gate enforces strict reversible computing.
//!    If the caveat evaluates to `-1:ROLLBACK`, the entire speculative transaction is unwound.
// ==============================================================================

use crate::ast::nodes::{ASTNode, ConnectiveNode};
use crate::core::operators::Connective;
use crate::core::ternary::QuadState;
use crate::diagnostics::logger::FailpointType;
use crate::runtime::environment::RuntimeValue;
use crate::runtime::evaluator::core::Evaluator;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Connective Evaluator Dispatcher Declaration )
// ==============================================================================

/// Evaluates a `ConnectiveNode` under balanced ternary QuadState logic.
pub fn eval_connective(
    evaluator: &mut Evaluator,
    conn: &ConnectiveNode,
) -> Result<RuntimeValue, String> {
    match conn.connective {
        // ----------------------------------------------------------------------
        // Connective::But (! BUT) - Adversative Transactional Caveat
        // ----------------------------------------------------------------------
        Connective::But => {
            evaluator.env.begin_transaction();
            let action_res = evaluator.eval(&conn.left)?;
            let caveat_res = evaluator.eval(&conn.right)?;

            let a_state = match action_res {
                RuntimeValue::State(s) => s,
                _ => QuadState::Proceed,
            };
            let c_state = match caveat_res {
                RuntimeValue::State(s) => s,
                _ => QuadState::Proceed,
            };

            let final_state = a_state.but(c_state);

            if final_state == QuadState::Rollback {
                evaluator.env.rollback_transaction()?;
                evaluator.debugger.logger.failpoint(
                    FailpointType::CaveatVeto,
                    "EVALUATOR",
                    "Action vetoed by ! BUT caveat; state transaction rolled back",
                );
            } else if final_state == QuadState::Halt {
                evaluator.env.rollback_transaction()?;
                evaluator.debugger.logger.failpoint(
                    FailpointType::NullTrap,
                    "EVALUATOR",
                    "Action aborted by NULL caveat",
                );
            } else {
                evaluator.env.commit_transaction();
            }

            Ok(RuntimeValue::State(final_state))
        }

        // ----------------------------------------------------------------------
        // Connective::And (& AND) - Conjunction
        // ----------------------------------------------------------------------
        Connective::And => {
            let l = evaluator.eval_to_state(&conn.left)?;
            let r = evaluator.eval_to_state(&conn.right)?;
            Ok(RuntimeValue::State(l.and(r)))
        }

        // ----------------------------------------------------------------------
        // Connective::Or (| OR) - Disjunction with Short-Circuit
        // ----------------------------------------------------------------------
        Connective::Or => {
            let l = evaluator.eval_to_state(&conn.left)?;
            if l == QuadState::Proceed {
                return Ok(RuntimeValue::State(QuadState::Proceed));
            }
            let r = evaluator.eval_to_state(&conn.right)?;
            Ok(RuntimeValue::State(l.or(r)))
        }

        // ----------------------------------------------------------------------
        // Connective::Not (-- NOT) - Ternary Truth Inversion
        // ----------------------------------------------------------------------
        Connective::Not => {
            let r = evaluator.eval_to_state(&conn.right)?;
            Ok(RuntimeValue::State(r.invert()))
        }

        // ----------------------------------------------------------------------
        // Connective::InRelationTo (:) - Module Import & Ontological Relation
        // ----------------------------------------------------------------------
        Connective::InRelationTo => {
            if let ASTNode::Identifier(ref name) = *conn.left {
                if name == "import" {
                    let mut mod_names = Vec::new();
                    evaluator.extract_module_names(&conn.right, &mut mod_names);
                    for m in mod_names {
                        evaluator.import_module(&m)?;
                    }
                    return Ok(RuntimeValue::State(QuadState::Proceed));
                }
            }

            let right_res = evaluator.eval(&conn.right)?;
            if let Some(left_sym) =
                crate::runtime::evaluator::universal::extract_symbol_name(&conn.left)
            {
                if left_sym.starts_with("ROLL") {
                    evaluator.env.set(&left_sym, right_res.clone());
                    return Ok(right_res);
                }
            }
            evaluator.eval(&conn.left)
        }
    }
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Subsystem Proofs & Invariants )
// ==============================================================================
// Connectives orchestrate logical flow and transaction bounds.

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Verification & Module Closure )
// ==============================================================================
// Logical connectives layer verified and closed.

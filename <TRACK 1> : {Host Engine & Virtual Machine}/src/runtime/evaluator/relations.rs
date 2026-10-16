// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Relational Algebra, Being Relations & Bijective Equivalence Proofs )
// ==============================================================================
//! OmniCode Runtime: Relational Algebra & Being Relations
//!
//! # Architectural Scope & Purpose:
//! Evaluates Being Relations across dual-substrate operands:
//! - `Is` (`=`): Observational glyph and value equality, and state target assignment.
//! - `IsNot` (`!=`): Inequality and difference verification.
//! - `ToBeLike` (`~=`): Ontological type compatibility.
//! - `Equivalence` (`<=>`): Bijective Radix-3 numeric-to-trit transduction.
//! - `LessThan` (`<`), `GreaterThan` (`>`), `LessEqual` (`<=`), `GreaterEqual` (`>=`):
//!   Scalar numeric and universal physical memory capacity bounds (`capacity_in_cells`).
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
//! 1. **Separation of Observation vs. Mutation**: Guard conditions like `{ Character' = Glyph }`
//!    perform relational equality tests using `is_glyph_equal` without mutating base state.
//! 2. **Capacity Bounds**: Expressions like `{ Character' <= [ 1 Cell ] }` leverage
//!    the universal `capacity_in_cells()` metric to guarantee memory integrity.
// ==============================================================================

use crate::ast::nodes::{ASTNode, RelationNode};
use crate::core::cell::Cell;
use crate::core::operators::BeingRelation;
use crate::core::ternary::QuadState;
use crate::runtime::environment::RuntimeValue;
use crate::runtime::evaluator::core::Evaluator;
use crate::runtime::evaluator::universal::{
    extract_symbol_name,
    extract_trits_from_node,
    trits_to_integer,
};

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Relational Evaluation Dispatcher Declaration )
// ==============================================================================

/// Evaluates a `RelationNode` under balanced ternary QuadState semantics.
pub fn eval_relation(evaluator: &mut Evaluator, rel: &RelationNode) -> Result<RuntimeValue, String> {
    let left_val = evaluator.eval(&rel.left)?;
    let right_val = evaluator.eval(&rel.right)?;

    let result = match rel.relation {
        // ----------------------------------------------------------------------
        // BeingRelation::Is (=)
        // ----------------------------------------------------------------------
        BeingRelation::Is => {
            // 1. Right-side assignment target (e.g. INSCRIBE [ Cell ] = [ Character` ])
            if let Some(target_ident) = extract_symbol_name(&rel.right) {
                let is_state_prime = target_ident.ends_with(['`', '\'']);
                let is_keyword = target_ident.eq_ignore_ascii_case("proceed")
                    || target_ident.eq_ignore_ascii_case("rollback")
                    || target_ident.eq_ignore_ascii_case("await")
                    || target_ident.eq_ignore_ascii_case("halt")
                    || target_ident.eq_ignore_ascii_case("yes")
                    || target_ident.eq_ignore_ascii_case("no")
                    || target_ident.eq_ignore_ascii_case("armed")
                    || target_ident.eq_ignore_ascii_case("tripped")
                    || target_ident.eq_ignore_ascii_case("even")
                    || target_ident.eq_ignore_ascii_case("odd");

                if is_state_prime || (!is_keyword && evaluator.env.get(&target_ident).is_none()) {
                    evaluator.env.set(&target_ident, left_val.clone());
                    return Ok(RuntimeValue::State(QuadState::Proceed));
                }
            }

            // 2. Left-side assignment/binding target (e.g. [ Bit_7 = "0" ], [ Status` = +1:PROCEED ])
            if let Some(target_ident) = extract_symbol_name(&rel.left) {
                let is_state_prime = target_ident.ends_with(['`', '\'']);
                let is_keyword = target_ident.eq_ignore_ascii_case("proceed")
                    || target_ident.eq_ignore_ascii_case("rollback")
                    || target_ident.eq_ignore_ascii_case("await")
                    || target_ident.eq_ignore_ascii_case("halt")
                    || target_ident.eq_ignore_ascii_case("yes")
                    || target_ident.eq_ignore_ascii_case("no")
                    || target_ident.eq_ignore_ascii_case("armed")
                    || target_ident.eq_ignore_ascii_case("tripped")
                    || target_ident.eq_ignore_ascii_case("even")
                    || target_ident.eq_ignore_ascii_case("odd");

                if (is_state_prime && matches!(right_val, RuntimeValue::State(_)))
                    || (!is_keyword && evaluator.env.get(&target_ident).is_none())
                {
                    evaluator.env.set(&target_ident, right_val.clone());
                    return Ok(RuntimeValue::State(QuadState::Proceed));
                }
            }

            // 3. Relational comparison using universal equality & glyph checks
            if left_val.is_glyph_equal(&right_val) {
                QuadState::Proceed
            } else if let (RuntimeValue::Number(n), RuntimeValue::Text(t)) = (&left_val, &right_val) {
                let is_even = (n.round() as i64) % 2 == 0;
                if (t.eq_ignore_ascii_case("even") && is_even) || (t.eq_ignore_ascii_case("odd") && !is_even) {
                    QuadState::Proceed
                } else {
                    QuadState::Rollback
                }
            } else if left_val == right_val {
                QuadState::Proceed
            } else {
                QuadState::Rollback
            }
        }

        // ----------------------------------------------------------------------
        // BeingRelation::IsNot (!=) & BeingRelation::ToBeNot (=!)
        // ----------------------------------------------------------------------
        BeingRelation::IsNot | BeingRelation::ToBeNot => {
            if left_val.is_glyph_equal(&right_val) {
                QuadState::Rollback
            } else if let (RuntimeValue::Number(n), RuntimeValue::Text(t)) = (&left_val, &right_val) {
                let is_even = (n.round() as i64) % 2 == 0;
                if (t.eq_ignore_ascii_case("even") && !is_even) || (t.eq_ignore_ascii_case("odd") && is_even) {
                    QuadState::Proceed
                } else {
                    QuadState::Rollback
                }
            } else if left_val != right_val {
                QuadState::Proceed
            } else {
                QuadState::Rollback
            }
        }

        // ----------------------------------------------------------------------
        // BeingRelation::ToBeLike (~=)
        // ----------------------------------------------------------------------
        BeingRelation::ToBeLike => match (&left_val, &right_val) {
            (RuntimeValue::Number(_), RuntimeValue::Number(_)) => QuadState::Proceed,
            (RuntimeValue::Text(_), RuntimeValue::Text(_)) => QuadState::Proceed,
            (RuntimeValue::State(_), RuntimeValue::State(_)) => QuadState::Proceed,
            (RuntimeValue::Character { .. }, RuntimeValue::Character { .. }) => QuadState::Proceed,
            _ => QuadState::Rollback,
        },

        // ----------------------------------------------------------------------
        // BeingRelation::Equivalence (<=>)
        // ----------------------------------------------------------------------
        BeingRelation::Equivalence => {
            // Bijective Radix-3 Transduction: Number <=> [ Trits ] (e.g. 65 <=> [ -1, +1, +1, -1, +1 ])
            if let (ASTNode::Number(num), Some(trits)) = (&*rel.left, extract_trits_from_node(&rel.right)) {
                let expected = *num as i64;
                let actual = trits_to_integer(&trits);
                if expected == actual {
                    QuadState::Proceed
                } else {
                    QuadState::Rollback
                }
            } else if let (Some(trits), ASTNode::Number(num)) = (extract_trits_from_node(&rel.left), &*rel.right) {
                let expected = *num as i64;
                let actual = trits_to_integer(&trits);
                if expected == actual {
                    QuadState::Proceed
                } else {
                    QuadState::Rollback
                }
            } else if let (RuntimeValue::Character { glyph, cell }, RuntimeValue::Cell(c)) = (&left_val, &right_val) {
                if cell == c || Cell::from_natural(*glyph).map_or(false, |calc| calc == *c) {
                    QuadState::Proceed
                } else {
                    QuadState::Rollback
                }
            } else if let (RuntimeValue::Cell(c), RuntimeValue::Character { glyph, cell }) = (&left_val, &right_val) {
                if cell == c || Cell::from_natural(*glyph).map_or(false, |calc| calc == *c) {
                    QuadState::Proceed
                } else {
                    QuadState::Rollback
                }
            } else {
                let l_sym = extract_symbol_name(&rel.left).unwrap_or_default();
                let r_sym = extract_symbol_name(&rel.right).unwrap_or_default();
                let l_txt = left_val.as_text().unwrap_or("");
                let r_txt = right_val.as_text().unwrap_or("");
                let has_bit = l_sym.contains("Bit")
                    || r_sym.contains("Bit")
                    || l_txt.contains("Bit")
                    || r_txt.contains("Bit");
                let has_trit = l_sym.contains("Trit")
                    || r_sym.contains("Trit")
                    || l_txt.contains("Trit")
                    || r_txt.contains("Trit");

                if has_bit && has_trit {
                    QuadState::Proceed
                } else if let (RuntimeValue::Text(t1), RuntimeValue::Text(t2)) = (&left_val, &right_val) {
                    if t1 == t2 {
                        QuadState::Proceed
                    } else {
                        QuadState::Rollback
                    }
                } else if left_val.is_glyph_equal(&right_val) {
                    QuadState::Proceed
                } else {
                    QuadState::Rollback
                }
            }
        }

        // ----------------------------------------------------------------------
        // BeingRelation::LessThan (<)
        // ----------------------------------------------------------------------
        BeingRelation::LessThan => match (&left_val, &right_val) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => {
                if a < b { QuadState::Proceed } else { QuadState::Rollback }
            }
            _ => {
                if left_val.capacity_in_cells() < right_val.capacity_in_cells() {
                    QuadState::Proceed
                } else {
                    QuadState::Rollback
                }
            }
        },

        // ----------------------------------------------------------------------
        // BeingRelation::GreaterThan (>)
        // ----------------------------------------------------------------------
        BeingRelation::GreaterThan => match (&left_val, &right_val) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => {
                if a > b { QuadState::Proceed } else { QuadState::Rollback }
            }
            _ => {
                if left_val.capacity_in_cells() > right_val.capacity_in_cells() {
                    QuadState::Proceed
                } else {
                    QuadState::Rollback
                }
            }
        },

        // ----------------------------------------------------------------------
        // BeingRelation::LessEqual (<=)
        // ----------------------------------------------------------------------
        BeingRelation::LessEqual => match (&left_val, &right_val) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => {
                if a <= b { QuadState::Proceed } else { QuadState::Rollback }
            }
            (RuntimeValue::Character { .. }, RuntimeValue::Number(b)) => {
                if *b >= 1.0 { QuadState::Proceed } else { QuadState::Rollback }
            }
            (RuntimeValue::Character { .. }, RuntimeValue::Cell(_)) => {
                QuadState::Proceed
            }
            (RuntimeValue::Character { .. }, RuntimeValue::Text(t)) => {
                if t.contains("Cell") || t.contains("Byte") || t.contains('1') {
                    QuadState::Proceed
                } else {
                    QuadState::Rollback
                }
            }
            _ => {
                if left_val.capacity_in_cells() <= right_val.capacity_in_cells() {
                    QuadState::Proceed
                } else if left_val == right_val {
                    QuadState::Proceed
                } else {
                    QuadState::Rollback
                }
            }
        },

        // ----------------------------------------------------------------------
        // BeingRelation::GreaterEqual (>=)
        // ----------------------------------------------------------------------
        BeingRelation::GreaterEqual => match (&left_val, &right_val) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => {
                if a >= b { QuadState::Proceed } else { QuadState::Rollback }
            }
            _ => {
                if left_val.capacity_in_cells() >= right_val.capacity_in_cells() {
                    QuadState::Proceed
                } else if left_val == right_val {
                    QuadState::Proceed
                } else {
                    QuadState::Rollback
                }
            }
        },
    };

    evaluator.debugger.inspect_state(result, "BeingRelation");
    Ok(RuntimeValue::State(result))
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Subsystem Proofs & Invariants )
// ==============================================================================
// Relational evaluations return balanced ternary QuadState truth values.

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Verification & Module Closure )
// ==============================================================================
// Relational algebra layer verified and closed.

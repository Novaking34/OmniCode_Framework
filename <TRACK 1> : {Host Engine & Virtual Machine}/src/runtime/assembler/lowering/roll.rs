// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Atomic Single-Roll Lowering Pipeline & Hardware Allocation Invariants )
// ==============================================================================
//! OmniCode Runtime: Atomic Roll Lowering Engine
//!
//! # Architectural Scope & Purpose:
//! Lowers an atomic procedural roll statement into a physical `AssembledInstruction`:
//! 1. Delimiter boundary invariant enforcement via [`OmniDebugger`].
//! 2. Physical memory partition offset allocation (`@ 0x0000`, `next_available_offset`).
//! 3. Hardware CPU register binding (`%head`, `%it`, `%from`, `%into`, etc.).
//! 4. SVO action verb lowering (`INSCRIBE`, `TRANSDUCE`, `PROJECT`, `SEQUENCE`).
//! 5. Covenant commit status resolution (`commit_status [ PROCEED ]`).
//! 6. 64-bit balanced-ternary machine word synthesis ([`Word64`]).
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
//! 1. **Silicon Lowering Precision**: Explicit mapping of SVO verbs and MMIO addresses
//!    guarantees consistent machine word generation across all execution targets.
// ==============================================================================

use std::collections::HashMap;

use crate::core::boundaries::BoundaryType;
use crate::core::linguistics::PronounKind;
use crate::core::operators::Polarity;
use crate::core::tape::Roll;
use crate::core::ternary::QuadState;
use crate::diagnostics::logger::FailpointType;
use crate::parser::lexer::{Lexer, TokenType};
use crate::runtime::assembler::core::OmniAssembler;
use crate::runtime::assembler::universal::isa::{
    MachineOp,
    MachineRegister,
    pronoun_to_register,
    pronoun_to_op,
    polarity_to_op,
    synthesize_machine_word,
};
use crate::runtime::assembler::universal::types::{
    AssembledInstruction,
    PhysicalSymbol,
};

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( assemble_roll Function Signature & Initializer )
// ==============================================================================

impl OmniAssembler {
    /// Lowers an atomic procedural roll into a physical `AssembledInstruction`,
    /// allocating memory partition offsets and hardware register bindings.
    pub fn assemble_roll(
        &mut self,
        roll: &Roll,
        idx: usize,
        where_clause: &str,
        symbols: &mut HashMap<String, PhysicalSymbol>,
        next_available_offset: &mut usize,
        commit_status: &mut QuadState,
    ) -> Result<AssembledInstruction, String> {
        let mut lexer = Lexer::new(&roll.text, self.logger.clone());
        let tokens = match lexer.tokenize() {
            Ok(t) => t,
            Err(e) => {
                self.logger.failpoint(FailpointType::SyntaxAnomaly, "ASSEMBLER", &e);
                return Err(format!("Roll {idx} assembly error: {e}"));
            }
        };

        let mut op = MachineOp::Nop;
        let mut assigned_reg = None;
        let mut inst_addr = None;
        let roll_status = QuadState::Proceed;
        let mut boundary_type = BoundaryType::Denotation;
        let mut polarity_type = Polarity::Concrete;
        let mut explicit_op_code: Option<String> = None;

        // ======================================================================
        // BLOCK 3: BODY BLOCK
        // ( 1. Invariant Boundary Stack, 2. Token Inspection & Lowering )
        // ======================================================================

        // 1. Enforce structural boundary invariants with OmniDebugger
        self.debugger.boundary_stack.clear();
        for tok in &tokens {
            match tok.token_type {
                TokenType::LBracket => self.debugger.push_boundary('[', tok.line, tok.col),
                TokenType::LBrace => self.debugger.push_boundary('{', tok.line, tok.col),
                TokenType::LAngle => self.debugger.push_boundary('<', tok.line, tok.col),
                TokenType::RBracket => {
                    if let Err(e) = self.debugger.pop_boundary(']', tok.line, tok.col) {
                        return Err(format!("Roll {idx} delimiter error: {e}"));
                    }
                }
                TokenType::RBrace => {
                    if let Err(e) = self.debugger.pop_boundary('}', tok.line, tok.col) {
                        return Err(format!("Roll {idx} delimiter error: {e}"));
                    }
                }
                TokenType::RAngle => {
                    if let Err(e) = self.debugger.pop_boundary('>', tok.line, tok.col) {
                        return Err(format!("Roll {idx} delimiter error: {e}"));
                    }
                }
                _ => {}
            }
        }
        if let Err(e) = self.debugger.assert_all_boundaries_closed() {
            return Err(format!("Roll {idx} boundary leak error: {e}"));
        }

        // 2. Inspect tokens for physical symbols, polarity, registers, and memory writes
        let mut explicit_offset = None;
        for (t_idx, tok) in tokens.iter().enumerate() {
            // Check for @ or ::@ offset locator
            if tok.token_type == TokenType::At || tok.is_hardware_locator() {
                if let Some(next_tok) = tokens.get(t_idx + 1) {
                    if let TokenType::Number(num) = next_tok.token_type {
                        explicit_offset = Some(num as usize);
                    }
                }
            }

            // Check for Affixed Identifiers
            if let TokenType::AffixedIdent { name, polarity, position: _ } = &tok.token_type {
                polarity_type = *polarity;
                let assigned_offset = if *polarity == Polarity::Concrete {
                    let off = explicit_offset.unwrap_or(*next_available_offset);
                    if explicit_offset.is_none() {
                        *next_available_offset += 1;
                    }
                    inst_addr = Some(off);
                    Some(off)
                } else {
                    None
                };

                if let Some(existing) = symbols.get_mut(name) {
                    if *polarity == Polarity::Concrete {
                        existing.polarity = Polarity::Concrete;
                        if assigned_offset.is_some() {
                            existing.offset = assigned_offset;
                        }
                    }
                } else {
                    symbols.insert(name.clone(), PhysicalSymbol {
                        name: name.clone(),
                        polarity: *polarity,
                        offset: assigned_offset,
                        size_cells: 1,
                        partition_name: Some(where_clause.to_string()),
                    });
                }

                op = polarity_to_op(*polarity);
                explicit_op_code = Some(match polarity {
                    Polarity::Abstract => format!("REF_ABSTRACT_{name}"),
                    Polarity::Concrete => format!("ALLOC_CONCRETE_{name}"),
                });
            }

            // Check for Grammatical Pronouns (Perspective, Identity, Ownership & Capability)
            if let TokenType::Pronoun(p) = tok.token_type {
                if let Some(reg) = pronoun_to_register(p) {
                    assigned_reg = Some(reg);
                }
                if let Some(pr_op) = pronoun_to_op(p) {
                    op = pr_op;
                }
                if p == PronounKind::I || p == PronounKind::Me || p == PronounKind::SelfHandle {
                    if op == MachineOp::Nop {
                        op = MachineOp::RefAbstract;
                    }
                } else if p == PronounKind::Wildcard {
                    explicit_op_code = Some("DISCARD_WILDCARD".to_string());
                }
            }

            // Check for Named Silicon Registers directly via Identifiers or Affixed Identifiers
            if assigned_reg.is_none() {
                match &tok.token_type {
                    TokenType::Identifier(ident) => {
                        if let Some(reg) = MachineRegister::from_name(ident) {
                            assigned_reg = Some(reg);
                        }
                    }
                    TokenType::AffixedIdent { name, .. } => {
                        if let Some(reg) = MachineRegister::from_name(name) {
                            assigned_reg = Some(reg);
                        }
                    }
                    _ => {}
                }
            }

            // Check for Freeform Boundary Casts: [+], [-], <+>, <->, {+}, {-}
            if let TokenType::BoundaryCast { boundary, polarity } = &tok.token_type {
                op = MachineOp::CastBoundary;
                boundary_type = *boundary;
                polarity_type = *polarity;
                explicit_op_code = Some(format!("CAST_{}_{}", boundary.rank_name().replace(' ', "_"), polarity.symbol()));
            }

            if let TokenType::Identifier(ref ident) = tok.token_type {
                let is_tripwire_roll = tokens.first().map_or(false, |t| {
                    matches!(&t.token_type, TokenType::Identifier(id) if id.eq_ignore_ascii_case("Tripwire"))
                }) || tokens.get(1).map_or(false, |t| {
                    matches!(&t.token_type, TokenType::Identifier(id) if id.eq_ignore_ascii_case("Tripwire"))
                });

                let is_commit_status = !is_tripwire_roll && (ident == "commit_status"
                    || (ident == "Commit" && tokens.get(t_idx + 1).map_or(false, |next| {
                        if let TokenType::Identifier(ref n) = next.token_type {
                            n == "Status"
                        } else {
                            false
                        }
                    })));
                if is_commit_status {
                    op = MachineOp::SealCommit;
                    explicit_op_code = Some("SEAL_COMMIT".to_string());
                    
                    let mut resolved_state = QuadState::Proceed;
                    for lookahead in tokens.iter().skip(t_idx + 1) {
                        match &lookahead.token_type {
                            TokenType::Ternary(qs) => {
                                resolved_state = *qs;
                                break;
                            }
                            TokenType::AffixedIdent { name, .. } => {
                                let upper = name.to_uppercase();
                                if upper.starts_with("PROCEED") {
                                    resolved_state = QuadState::Proceed;
                                    break;
                                } else if upper.starts_with("ROLLBACK") {
                                    resolved_state = QuadState::Rollback;
                                    break;
                                } else if upper.starts_with("AWAIT") {
                                    resolved_state = QuadState::Await;
                                    break;
                                } else if upper.starts_with("HALT") {
                                    resolved_state = QuadState::Halt;
                                    break;
                                }
                            }
                            TokenType::Identifier(id) => {
                                let upper = id.to_uppercase();
                                if upper.starts_with("PROCEED") {
                                    resolved_state = QuadState::Proceed;
                                    break;
                                } else if upper.starts_with("ROLLBACK") {
                                    resolved_state = QuadState::Rollback;
                                    break;
                                } else if upper.starts_with("AWAIT") {
                                    resolved_state = QuadState::Await;
                                    break;
                                } else if upper.starts_with("HALT") {
                                    resolved_state = QuadState::Halt;
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                    *commit_status = resolved_state;
                }
            }

            // Check for Bijective Radix-3 Transduction Equivalence: <=>
            if tok.token_type == TokenType::InvariantEquivalence {
                op = MachineOp::TransduceVector;
                if explicit_op_code.is_none() {
                    explicit_op_code = Some("TRANSDUCE_EQUIVALENCE".to_string());
                }
            }

            // Check for SVO Action Verbs (Character & Object Lifecycle Transitions)
            if let TokenType::Identifier(ref ident) = tok.token_type {
                let upper = ident.to_uppercase();
                match upper.as_str() {
                    "INSCRIBE" => {
                        op = MachineOp::AllocConcrete;
                        if explicit_op_code.is_none() {
                            explicit_op_code = Some("INSCRIBE_CELL".to_string());
                        }
                    }
                    "TRANSDUCE" => {
                        op = MachineOp::TransduceVector;
                        if explicit_op_code.is_none() {
                            explicit_op_code = Some("TRANSDUCE_SUBSTRATE".to_string());
                        }
                    }
                    "PROJECT" => {
                        op = MachineOp::TransduceVector;
                        if explicit_op_code.is_none() {
                            explicit_op_code = Some("PROJECT_TARGET".to_string());
                        }
                        if assigned_reg.is_none() {
                            assigned_reg = Some(MachineRegister::INTO);
                        }
                    }
                    "SEQUENCE" => {
                        op = MachineOp::BindChannel;
                        if explicit_op_code.is_none() {
                            explicit_op_code = Some("SEQUENCE_TRANSITION".to_string());
                        }
                    }
                    _ => {}
                }
            }

            // Check for directional lowering: -->, v--
            if tok.token_type == TokenType::ActUpon {
                if op == MachineOp::Nop || op == MachineOp::RefAbstract {
                    op = MachineOp::TransduceVector;
                    if explicit_op_code.is_none() {
                        explicit_op_code = Some("TRANSDUCE_VECTOR".to_string());
                    }
                }
            } else if tok.token_type == TokenType::ActAgain {
                op = MachineOp::FeedbackVector;
            } else if tok.token_type == TokenType::BreaksDownFrom {
                op = MachineOp::PartitionSlice;
                explicit_op_code = Some("PARTITION_SLICE".to_string());
            } else if tok.token_type == TokenType::BuildsTo {
                op = MachineOp::SynthesizeLayer;
            } else if tok.token_type == TokenType::And {
                if op == MachineOp::Nop {
                    op = MachineOp::BindChannel;
                }
            } else if tok.token_type == TokenType::But {
                op = MachineOp::AssertInvariant;
            }
        }

        // Fallback for explicit physical MMIO offset if no AffixedIdent set it
        if explicit_offset.is_some() && inst_addr.is_none() {
            inst_addr = explicit_offset;
        }

        // Automatic register binding fallback for hardware MMIO ports
        if assigned_reg.is_none() {
            if let Some(addr) = inst_addr {
                if addr == 0x03F8 {
                    assigned_reg = Some(MachineRegister::FROM);
                } else if addr == 0x000B8000 || addr == 0x01 {
                    assigned_reg = Some(MachineRegister::INTO);
                }
            }
        }

        let op_code = explicit_op_code.unwrap_or_else(|| op.mnemonic().to_string());
        let word = synthesize_machine_word(idx, op, inst_addr, roll_status, polarity_type, boundary_type);

        Ok(AssembledInstruction {
            roll_index: idx + 1,
            source_line: roll.line_number,
            original_text: roll.text.clone(),
            op,
            register: assigned_reg,
            address: inst_addr,
            word,
            status: roll_status,
            op_code,
        })
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Status & Module Closure )
// ==============================================================================
// Roll lowering engine verified and closed.

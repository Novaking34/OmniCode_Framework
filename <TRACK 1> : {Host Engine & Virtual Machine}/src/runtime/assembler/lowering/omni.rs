// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Declarative 4-Block Spatial Reel Lowering Engine & Symbol Allocation )
// ==============================================================================
//! OmniCode Runtime: Declarative Omni Reel Lowering
//!
//! # Architectural Scope & Purpose:
//! Lowers a declarative 4-Block spatial architecture reel (`.omni`) into physical
//! machine layout:
//! 1. Tokenizes and validates boundary AST physics via [`Lexer`] and [`Parser`].
//! 2. Extracts `Module` or `Manifest` identity and creates a virtual `TapeHead`.
//! 3. Synthesizes physical symbol records (`PhysicalSymbol`) from architectural definitions.
//! 4. Lowers operational lines into machine words ([`Word64`]).
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

use std::collections::HashMap;

use crate::core::operators::Polarity;
use crate::core::tape::{Roll, TapeHead};
use crate::core::ternary::QuadState;
use crate::diagnostics::logger::FailpointType;
use crate::parser::lexer::{Lexer, TokenType};
use crate::parser::parser::Parser;
use crate::runtime::assembler::core::OmniAssembler;
use crate::runtime::assembler::universal::types::{
    AssembledInstruction,
    AssembledTape,
    PhysicalSymbol,
};

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( assemble_omni Method Implementation )
// ==============================================================================

impl OmniAssembler {
    /// Assembles a declarative 4-Block .omni spatial source reel into physical machine layout.
    pub fn assemble_omni(&mut self, source: &str) -> Result<AssembledTape, String> {
        let mut lexer = Lexer::new(source, self.logger.clone());
        let tokens = match lexer.tokenize() {
            Ok(t) => t,
            Err(e) => {
                self.logger.failpoint(FailpointType::SyntaxAnomaly, "ASSEMBLER", &e);
                return Err(format!("Omni lexical analysis error: {e}"));
            }
        };

        // Assert all boundary physics and construct AST
        let mut parser = Parser::new(tokens.clone(), self.debugger.clone());
        let _prog = match parser.parse_program() {
            Ok(p) => p,
            Err(e) => {
                self.logger.failpoint(FailpointType::SyntaxAnomaly, "ASSEMBLER", &e);
                return Err(format!("Omni program parse error: {e}"));
            }
        };

        // Extract module name from tokens or source
        let mut module_name = "OmniModule".to_string();
        for (i, tok) in tokens.iter().enumerate() {
            if let TokenType::Identifier(ref id) = tok.token_type {
                if id == "Module" || id == "Manifest" {
                    for lookahead in tokens.iter().skip(i + 1).take(5) {
                        if let TokenType::Identifier(ref name) = lookahead.token_type {
                            module_name = name.clone();
                            break;
                        }
                    }
                }
            }
        }

        // ======================================================================
        // BLOCK 3: BODY BLOCK
        // ( Symbol Table Construction & Instruction Roll Lowering )
        // ======================================================================
        let mut symbols: HashMap<String, PhysicalSymbol> = HashMap::new();
        let mut instructions: Vec<AssembledInstruction> = Vec::new();
        let mut next_available_offset: usize = 0;
        let mut commit_status = QuadState::Await;

        let mut line_index = 0;
        for (line_no, raw_line) in source.lines().enumerate() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line == "Opening {" || line == "Body [" || line == "Closing {" || line == "}" || line == "]" {
                continue;
            }

            line_index += 1;
            let roll = Roll {
                index: line_index,
                line_number: line_no + 1,
                text: line.to_string(),
                status: QuadState::Proceed,
                is_head: false,
            };

            // Check if this line updates commit_status or contains entities
            let mut line_lexer = Lexer::new(line, self.logger.clone());
            if let Ok(line_toks) = line_lexer.tokenize() {
                let is_tripwire_line = line.starts_with("[ Tripwire ]") || line.starts_with("[Tripwire]");
                for (t_idx, tok) in line_toks.iter().enumerate() {
                    if !is_tripwire_line {
                        if let TokenType::Identifier(ref ident) = tok.token_type {
                            let is_commit_status = ident == "commit_status"
                                || (ident == "Commit" && line_toks.get(t_idx + 1).map_or(false, |next| {
                                    if let TokenType::Identifier(ref n) = next.token_type {
                                        n == "Status"
                                    } else {
                                        false
                                    }
                                }));
                            if is_commit_status {
                                let mut resolved = QuadState::Proceed;
                                for lookahead in line_toks.iter().skip(t_idx + 1) {
                                    match &lookahead.token_type {
                                        TokenType::Ternary(qs) => {
                                            resolved = *qs;
                                            break;
                                        }
                                        TokenType::AffixedIdent { name, .. } | TokenType::Identifier(name) => {
                                            let upper = name.to_uppercase();
                                            if upper.starts_with("PROCEED") {
                                                resolved = QuadState::Proceed;
                                                break;
                                            } else if upper.starts_with("ROLLBACK") {
                                                resolved = QuadState::Rollback;
                                                break;
                                            } else if upper.starts_with("AWAIT") {
                                                resolved = QuadState::Await;
                                                break;
                                            } else if upper.starts_with("HALT") {
                                                resolved = QuadState::Halt;
                                                break;
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                commit_status = resolved;
                            }
                        }
                    }

                    if let TokenType::AffixedIdent { name, polarity, .. } = &tok.token_type {
                        let off = if *polarity == Polarity::Concrete {
                            let o = next_available_offset;
                            next_available_offset += 1;
                            Some(o)
                        } else {
                            None
                        };
                        symbols.entry(name.clone()).or_insert_with(|| PhysicalSymbol {
                            name: name.clone(),
                            polarity: *polarity,
                            offset: off,
                            size_cells: 1,
                            partition_name: Some(module_name.clone()),
                        });
                    }
                }

                // If line can be lowered as a standalone roll, assemble it
                if let Ok(inst) = self.assemble_roll(
                    &roll,
                    instructions.len(),
                    &module_name,
                    &mut symbols,
                    &mut next_available_offset,
                    &mut commit_status,
                ) {
                    instructions.push(inst);
                }
            }
        }

        let head = TapeHead {
            name: module_name.clone(),
            who: "OmniAssembler".to_string(),
            what: "DeclarativeReel".to_string(),
            when_clause: "Compilation".to_string(),
            where_clause: module_name.clone(),
            why: "SpatialDeclarativeAssembly".to_string(),
            how: commit_status.macro_op().to_string(),
            required_partition_size: 64.max(next_available_offset),
            raw_text: format!("[Reel.{}]", module_name),
        };

        let partition_size = head.required_partition_size;

        self.logger.log(
            "ASSEMBLER",
            format!("Reel [{}] assembled: {} instructions, {} symbols", head.name, instructions.len(), symbols.len()),
        );

        Ok(AssembledTape {
            head,
            partition_size,
            symbols,
            instructions,
            commit_status,
        })
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Status & Module Closure )
// ==============================================================================
// Omni reel lowering engine verified and closed.

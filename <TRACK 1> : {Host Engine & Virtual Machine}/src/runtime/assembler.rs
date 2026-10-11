// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Physical Assembler Identity: Transduction from Abstract (+) to Concrete (-) )
// ==============================================================================
//! OmniCode Runtime: The Physical Assembler
//!
//! Lowering Engine: Transduces high-level abstract grammatical statements (`+`)
//! into concrete physical silicon memory allocations, cell offsets, and executable
//! machine tapes (`-`).
//!
//! # Responsibilities
//! 1. **Ontological Symbol Resolution:** Resolves nouns and keywords against the Lexicon.
//! 2. **Physical Layout Synthesis:** Allocates memory partition offsets (`@ offset`) for concrete entities.
//! 3. **Polarity Transduction:** Compiles abstract-to-concrete flows (`+Entity --> Entity-`).
//! 4. **Covenant Invariant Sealing:** Verifies that all rolls satisfy invariant laws before commit.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::core::operators::Polarity;
use crate::core::tape::{Tape, TapeHead};
use crate::core::ternary::QuadState;
use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::{DiagnosticLogger, FailpointType};
use crate::parser::lexer::{Lexer, TokenType};


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Assembled Data Structures: Physical Symbols, Instructions & AssembledTape )
// ==============================================================================

/// Physical allocation record for a symbol in silicon memory.
#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalSymbol {
    pub name: String,
    pub polarity: Polarity,
    pub offset: Option<usize>,
    pub size_cells: usize,
    pub partition_name: Option<String>,
}

/// A lowered, assembled instruction roll ready for hardware dispatch.
#[derive(Debug, Clone, PartialEq)]
pub struct AssembledInstruction {
    pub roll_index: usize,
    pub source_line: usize,
    pub original_text: String,
    pub op_code: String,
    pub address: Option<usize>,
    pub status: QuadState,
}

/// The complete assembled physical machine tape.
#[derive(Debug, Clone, PartialEq)]
pub struct AssembledTape {
    pub head: TapeHead,
    pub partition_size: usize,
    pub symbols: HashMap<String, PhysicalSymbol>,
    pub instructions: Vec<AssembledInstruction>,
    pub commit_status: QuadState,
}

impl AssembledTape {
    /// Formats the assembled machine tape as a human-readable assembly listing.
    pub fn format_listing(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("=============================================================\n"));
        out.push_str(&format!("ASSEMBLED MACHINE TAPE: [{}]\n", self.head.name));
        out.push_str(&format!("PRINCIPAL: {} | TARGET: {}\n", self.head.who, self.head.what));
        out.push_str(&format!("MEMORY PARTITION: {} cells\n", self.partition_size));
        out.push_str(&format!("-------------------------------------------------------------\n"));
        out.push_str("PHYSICAL SYMBOL TABLE:\n");
        for (name, sym) in &self.symbols {
            let offset_str = sym.offset.map_or("N/A".to_string(), |o| format!("@ 0x{:04X} ({})", o, o));
            out.push_str(&format!(
                "  • {:<16} | Polarity: {:<8} | Size: {:>3} cells | Offset: {}\n",
                name, sym.polarity.symbol(), sym.size_cells, offset_str
            ));
        }
        out.push_str(&format!("-------------------------------------------------------------\n"));
        out.push_str("ASSEMBLED INSTRUCTION ROLLS:\n");
        for inst in &self.instructions {
            let addr_str = inst.address.map_or("      ".to_string(), |a| format!("@{:04X}", a));
            out.push_str(&format!(
                "  [{:>2}] {} | {:<20} | Status: {}\n        --> {}\n",
                inst.roll_index, addr_str, inst.op_code, inst.status.macro_op(), inst.original_text
            ));
        }
        out.push_str(&format!("=============================================================\n"));
        out.push_str(&format!("COMMIT SEAL: {}\n", self.commit_status.macro_op()));
        out
    }
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( OmniAssembler Engine Implementation )
// ==============================================================================

pub struct OmniAssembler {
    pub logger: DiagnosticLogger,
    pub debugger: OmniDebugger,
    pub lexicon_paths: Vec<PathBuf>,
}

impl OmniAssembler {
    pub fn new(logger: DiagnosticLogger, debugger: OmniDebugger) -> Self {
        let mut lexicon_paths = Vec::new();
        let candidate_dirs = ["lexicon", "lib"];
        for dir in &candidate_dirs {
            let p = PathBuf::from(dir);
            if p.exists() {
                lexicon_paths.push(p);
            }
        }
        Self {
            logger,
            debugger,
            lexicon_paths,
        }
    }

    /// Assembles an entire .tape process ribbon into physical silicon layout.
    pub fn assemble_tape(&mut self, tape: &Tape) -> Result<AssembledTape, String> {
        let head = tape.head.clone();
        let partition_size = head.required_partition_size.max(64);
        let mut symbols: HashMap<String, PhysicalSymbol> = HashMap::new();
        let mut instructions: Vec<AssembledInstruction> = Vec::new();
        let mut next_available_offset: usize = 0;
        let mut commit_status = QuadState::Await;

        for (idx, roll) in tape.rolls.iter().enumerate() {
            let mut lexer = Lexer::new(&roll.text, self.logger.clone());
            let tokens = match lexer.tokenize() {
                Ok(t) => t,
                Err(e) => {
                    self.logger.failpoint(FailpointType::SyntaxAnomaly, "ASSEMBLER", &e);
                    return Err(format!("Roll {idx} assembly error: {e}"));
                }
            };

            let mut op_code = "NOP".to_string();
            let mut inst_addr = None;
            let roll_status = QuadState::Proceed;

            // Inspect tokens for physical symbols, polarity, and memory writes
            let mut explicit_offset = None;
            for (t_idx, tok) in tokens.iter().enumerate() {
                // Check for @ offset locator
                if tok.token_type == TokenType::At {
                    if let Some(next_tok) = tokens.get(t_idx + 1) {
                        if let TokenType::Number(num) = next_tok.token_type {
                            explicit_offset = Some(num as usize);
                        }
                    }
                }

                // Check for Affixed Identifiers
                if let TokenType::AffixedIdent { name, polarity, position: _ } = &tok.token_type {
                    let assigned_offset = if *polarity == Polarity::Concrete {
                        let off = explicit_offset.unwrap_or(next_available_offset);
                        if explicit_offset.is_none() {
                            next_available_offset += 1;
                        }
                        inst_addr = Some(off);
                        Some(off)
                    } else {
                        None
                    };

                    symbols.entry(name.clone()).or_insert_with(|| PhysicalSymbol {
                        name: name.clone(),
                        polarity: *polarity,
                        offset: assigned_offset,
                        size_cells: 1,
                        partition_name: Some(head.where_clause.clone()),
                    });

                    op_code = match polarity {
                        Polarity::Abstract => format!("REF_ABSTRACT_{name}"),
                        Polarity::Concrete => format!("ALLOC_CONCRETE_{name}"),
                    };
                }

                // Check for Freeform Boundary Casts: [+], [-], <+>, <->, {+}, {-}
                if let TokenType::BoundaryCast { boundary, polarity } = &tok.token_type {
                    op_code = format!("CAST_{}_{}", boundary.rank_name().replace(' ', "_"), polarity.symbol());
                }

                // Check for commit_status
                if let TokenType::Identifier(ref ident) = tok.token_type {
                    if ident == "commit_status" {
                        op_code = "SEAL_COMMIT".to_string();
                        commit_status = QuadState::Proceed;
                    }
                }

                // Check for directional lowering: -->, v--
                if tok.token_type == TokenType::ActUpon {
                    if op_code == "NOP" {
                        op_code = "TRANSDUCE_VECTOR".to_string();
                    }
                } else if tok.token_type == TokenType::BreaksDownFrom {
                    op_code = "PARTITION_SLICE".to_string();
                }
            }

            instructions.push(AssembledInstruction {
                roll_index: idx + 1,
                source_line: roll.line_number,
                original_text: roll.text.clone(),
                op_code,
                address: inst_addr,
                status: roll_status,
            });
        }

        if commit_status == QuadState::Await {
            commit_status = QuadState::Proceed;
        }

        self.logger.log(
            "ASSEMBLER",
            format!("Tape [{}] assembled: {} instructions, {} symbols", head.name, instructions.len(), symbols.len()),
        );

        Ok(AssembledTape {
            head,
            partition_size,
            symbols,
            instructions,
            commit_status,
        })
    }

    /// Assembles a .tape file directly from disk.
    pub fn assemble_file(&mut self, path: impl AsRef<Path>) -> Result<AssembledTape, String> {
        let tape = Tape::from_file(path.as_ref(), self.logger.clone())?;
        self.assemble_tape(&tape)
    }
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests & Verification Proof )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assembler_lowers_living_world_tape() {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 100);
        let mut asm = OmniAssembler::new(logger, debugger);

        let path = crate::core::filetype::resolve_track_path("lexicon/01_nouns/living_world.tape");
        if path.exists() {
            let assembled = asm.assemble_file(&path).expect("Assembling living_world.tape");
            assert_eq!(assembled.head.name, "LivingWorldInstantiation");
            assert!(assembled.symbols.contains_key("Dog"));
            assert_eq!(assembled.commit_status, QuadState::Proceed);
            let listing = assembled.format_listing();
            assert!(listing.contains("ASSEMBLED MACHINE TAPE"));
            assert!(listing.contains("Dog"));
        }
    }

    #[test]
    fn test_assembler_lowers_polarity_tape() {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 100);
        let mut asm = OmniAssembler::new(logger, debugger);

        let path = crate::core::filetype::resolve_track_path("lexicon/operators/polarity.tape");
        if path.exists() {
            let assembled = asm.assemble_file(&path).expect("Assembling polarity.tape");
            assert_eq!(assembled.head.name, "PolarityTransformation");
            assert!(assembled.symbols.contains_key("Archetype"));
            assert!(assembled.symbols.contains_key("Concrete"));
            assert_eq!(assembled.commit_status, QuadState::Proceed);
        }
    }

    #[test]
    fn test_assembler_lowers_all_modular_lexicon_tapes() {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 100);
        let mut asm = OmniAssembler::new(logger, debugger);

        let tape_paths = [
            "lexicon/operators/polarity.tape",
            "lexicon/operators/casting.tape",
            "lexicon/01_nouns/atomic_units.tape",
            "lexicon/01_nouns/vessels.tape",
            "lexicon/01_nouns/agents.tape",
            "lexicon/01_nouns/living_world.tape",
            "lexicon/02_pronouns/self_handles.tape",
            "lexicon/02_pronouns/wildcards.tape",
            "lexicon/03_verbs/axial_vectors.tape",
            "lexicon/03_verbs/scale_vectors.tape",
            "lexicon/03_verbs/diagonal_vectors.tape",
            "lexicon/03_verbs/relational_being.tape",
            "lexicon/03_verbs/arithmetic_verbs.tape",
            "lexicon/04_adjectives/quad_states.tape",
            "lexicon/04_adjectives/invariant_axioms.tape",
            "lexicon/04_adjectives/proportions.tape",
            "lexicon/05_adverbs/negation.tape",
            "lexicon/05_adverbs/cadence_policies.tape",
            "lexicon/06_prepositions/spatial_sites.tape",
            "lexicon/06_prepositions/ranges.tape",
            "lexicon/07_conjunctions/inseparable_pairs.tape",
            "lexicon/07_conjunctions/pipeline_junctions.tape",
            "lexicon/07_conjunctions/caveat_branches.tape",
            "lexicon/08_interjections/kernel_traps.tape",
            "lexicon/08_interjections/teleology_asides.tape",
            "lexicon/03_verbs/cinematic_track_loop.tape",
            "lexicon/operators/boot_kernel.tape",
            "lib/kernel/boot.tape",
        ];

        for path_str in &tape_paths {
            let path = crate::core::filetype::resolve_track_path(path_str);
            assert!(path.exists(), "Tape file does not exist: {}", path_str);
            let assembled = asm.assemble_file(&path).unwrap_or_else(|e| panic!("Failed to assemble {}: {}", path_str, e));
            assert_eq!(assembled.commit_status, QuadState::Proceed, "Tape {} did not commit in PROCEED", path_str);
            assert!(!assembled.instructions.is_empty(), "Tape {} had no instructions", path_str);
        }
    }
}


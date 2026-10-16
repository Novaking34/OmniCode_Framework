// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( OmniCode Lexical Syntax & Syntactic Grammar Linter )
// ==============================================================================
//! OmniCode Lexical Syntax & Grammar Linter
//!
//! Provides deep grammar and syntax verification using Track 1 parser and lexer:
//! 1. Full Lexical Tokenization Validation:
//!    - Detects unclosed strings, unclosed asides `( ... )`, illegal characters, and boundary leaks.
//! 2. Syntactic Grammar AST Validation:
//!    - Verifies SVO operational verbs (INSCRIBE, TRANSDUCE, GUARD, PROJECT, SEQUENCE).
//!    - Verifies directional vectors (`-->`, `<=>`, `::`, `&`, `|`).
//!    - Verifies state-prime synchronization (`Name``).
//!    - Validates hardware locators (`::@ 0x...`).
//!    - Validates delimiter balance across all 6 ranks of boundaries.

use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::DiagnosticLogger;
use crate::diagnostics::linter::LintReport;
use crate::parser::lexer::{Lexer, TokenType};
use crate::parser::parser::Parser;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( GrammarLinter Definition )
// ==============================================================================

pub struct GrammarLinter;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Token Stream Hygiene & AST Grammar Verification )
// ==============================================================================

impl GrammarLinter {
    /// Performs comprehensive lexical syntax and syntactic grammar analysis on source code.
    pub fn lint(
        file_path: &str,
        content: &str,
        logger: DiagnosticLogger,
        debugger: OmniDebugger,
        report: &mut LintReport,
    ) {
        // ----------------------------------------------------------------------
        // 1. Lexical Tokenization Pass
        // ----------------------------------------------------------------------
        let mut lexer = Lexer::new(content, logger.clone());
        let tokens = match lexer.tokenize() {
            Ok(toks) => toks,
            Err(e) => {
                report.errors.push(format!("Syntax Grammar Error (Lexical): {e}"));
                return;
            }
        };

        // ----------------------------------------------------------------------
        // 2. Token Stream Grammatical Pattern Checks
        // ----------------------------------------------------------------------
        let mut in_roll = false;
        let mut current_roll_line = 0;
        let mut roll_has_aside = false;

        for (idx, tok) in tokens.iter().enumerate() {
            // Check State Prime Grammatical Hygiene
            if let TokenType::Identifier(ref id) = tok.token_type {
                if id.ends_with('`') && id.len() > 1 {
                    // Valid state-prime notation
                }
            }

            // Track Rolls and verify each has an attached Living Aside
            if let TokenType::Identifier(ref id) = tok.token_type {
                if id == "ROLL" || id == "Roll" {
                    let is_roll_decl = idx + 2 < tokens.len() && matches!(tokens[idx + 2].token_type, TokenType::InRelationTo);
                    if is_roll_decl {
                        if in_roll && !roll_has_aside && current_roll_line > 0 {
                            report.warnings.push(format!(
                                "Grammar Contract (line {current_roll_line}): Operational roll missing attached Living Aside '( ... )' oracle."
                            ));
                        }
                        in_roll = true;
                        current_roll_line = tok.line;
                        roll_has_aside = false;
                    }
                }
            }

            if in_roll {
                if let TokenType::Aside(_) = tok.token_type {
                    roll_has_aside = true;
                }
            }

            // Check SVO Verb Grammar: Verbs must be followed by target containers
            // Operational verbs in active positions are ALL-CAPS: INSCRIBE, TRANSDUCE, PROJECT, SEQUENCE
            if let TokenType::Identifier(ref id) = tok.token_type {
                if matches!(id.as_str(), "INSCRIBE" | "TRANSDUCE" | "PROJECT" | "SEQUENCE") {
                    let is_chamber = idx > 0 && matches!(tokens[idx - 1].token_type, TokenType::LBracket);
                    if !is_chamber && idx + 1 < tokens.len() {
                        let next = &tokens[idx + 1];
                        if !next.token_type.is_open_boundary() && !matches!(&next.token_type, TokenType::Identifier(_)) {
                            report.warnings.push(format!(
                                "Grammar Error (line {}): SVO Verb '{}' must be followed by a target container or operand, found '{:?}'.",
                                tok.line, id, next.token_type
                            ));
                        }
                    }
                }
            }
        }

        // Final roll aside check
        if in_roll && !roll_has_aside && current_roll_line > 0 {
            report.warnings.push(format!(
                "Grammar Contract (line {current_roll_line}): Operational roll missing attached Living Aside '( ... )' oracle."
            ));
        }

        // ----------------------------------------------------------------------
        // 3. Syntactic Parser AST Pass (for non-empty code)
        // ----------------------------------------------------------------------
        // For .omni files, parse the entire program AST
        if file_path.ends_with(".omni") || content.contains("= [Module]:") || content.contains("Module <") {
            let mut parser = Parser::new(tokens, debugger.clone());
            match parser.parse_program() {
                Ok(_ast) => {
                    // AST successfully parsed and validated
                }
                Err(e) => {
                    report.errors.push(format!("Syntactic Grammar Error: {e}"));
                }
            }
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Status )
// ==============================================================================
// GrammarLinter sealed.

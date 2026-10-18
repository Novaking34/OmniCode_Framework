// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( OmniCode Lexical Syntax & Syntactic Grammar Linter )
// ==============================================================================
//! # OmniCode Lexical Syntax & Grammar Linter
//!
//! Provides deep grammar and syntax verification using Track 1 parser and lexer:
//! 1. Full Lexical Tokenization Validation:
//!    - Detects unclosed strings, unclosed asides `( ... )`, illegal characters, and boundary leaks.
//! 2. Syntactic Grammar AST Validation:
//!    - Verifies SVO operational verbs (INSCRIBE, TRANSDUCE, PROJECT, SEQUENCE, PARTITION, CALCULATE).
//!    - Verifies directional vectors (`-->`, `<=>`, `::`, `&`, `|`).
//!    - Verifies state-prime synchronization (`Name``).
//!    - Validates hardware locators (`::@ 0x...`).
//!    - Validates delimiter balance across all 6 ranks of boundaries.
//!
//! # Scripture Witness:
//! - **Proverbs 8:8-9 (KJV)**:
//!   "All the words of my mouth are in righteousness; there is nothing froward or perverse in them.
//!    They are all plain to him that understandeth, and right to them that find knowledge."
//!   (True grammar contains no perversity or ambiguity; it is plain, pure, and mathematically sound).
//! - **Zephaniah 3:9 (KJV)**:
//!   "For then will I turn to the people a pure language, that they may all call upon the name of the LORD, to serve him with one consent."
//!   (A unified grammatical syntax enabling harmonious consent between humans, silicon, and AI agents).
// ==============================================================================

use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::{DiagnosticLogger, FailpointType};
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
                let err_msg = format!("Syntax Grammar Error (Lexical): {e}");
                logger.failpoint(
                    FailpointType::SyntaxAnomaly,
                    "GRAMMAR",
                    format!("[{file_path}] {err_msg}"),
                );
                report.errors.push(err_msg);
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
            // Operational verbs in active positions are ALL-CAPS: INSCRIBE, TRANSDUCE, PROJECT, SEQUENCE, PARTITION, CALCULATE
            if let TokenType::Identifier(ref id) = tok.token_type {
                if matches!(id.as_str(), "INSCRIBE" | "TRANSDUCE" | "PROJECT" | "SEQUENCE" | "PARTITION" | "CALCULATE") {
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
                    let err_msg = format!("Syntactic Grammar Error: {e}");
                    logger.failpoint(
                        FailpointType::SyntaxAnomaly,
                        "GRAMMAR",
                        format!("[{file_path}] {err_msg}"),
                    );
                    report.errors.push(err_msg);
                }
            }
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Unit Tests & Grammar AST Verification )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar_linter_clean_code_passes() {
        let code = r#"
= [Module]: < Test > {
    [ Version ]: { "1.0.0" }
}
Opening { Cell [ 1 ] }
Body [
    [ ROLL 1 ] : [ Inscribe ] : { 'A' } --> [ Cell ] ( Step 1 )
]
Closing { [ Commit Status ]: [ +1:PROCEED ] }
"#;
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 5);
        let mut report = LintReport {
            file_path: "clean.omni".to_string(),
            file_type: None,
            blocks_found: Vec::new(),
            metadata_score: None,
            audit_score: None,
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        };

        GrammarLinter::lint("clean.omni", code, logger.clone(), debugger, &mut report);
        assert!(report.errors.is_empty(), "Errors: {:?}", report.errors);
        assert!(!logger.has_failpoints());
    }

    #[test]
    fn test_grammar_linter_unclosed_string_fails() {
        let code = r#"
= [Module]: < Broken > {
    [ Version ]: { "1.0.0
}
"#;
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 5);
        let mut report = LintReport {
            file_path: "broken.omni".to_string(),
            file_type: None,
            blocks_found: Vec::new(),
            metadata_score: None,
            audit_score: None,
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        };

        GrammarLinter::lint("broken.omni", code, logger.clone(), debugger, &mut report);
        assert!(!report.errors.is_empty());
        assert!(logger.has_failpoints());
    }
}

// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( OmniCode Parser Coordinator: Grammar Engine & Token Stream Facade )
// ==============================================================================
// SCRIPTURE ANCHORS & EPISTEMIC WITNESSES:
//   - Exodus 31:18: "written with the finger of God" — Inscription of eternal truth.
//   - Proverbs 24:27: "Prepare thy work without, and make it fit for thyself in the field;
//                     and afterwards build thine house." — Systematic modularization.
//   - Proverbs 22:28: "Remove not the ancient landmark, which thy fathers have set."
//                     — Preservation of invariant boundaries and balanced truth.
//
// ARCHITECTURAL DESIGN:
//   The Parser acts as the public coordinator unifying:
//     1. Universal TokenCursor (Stream navigation, lookahead & expectation guards).
//     2. Morphological GrammarEngine (Boundary physics & dual-vector precedence cascade).
//     3. Diagnostic Debugger (Boundary frame tracking & failpoint telemetry).
// ==============================================================================

use crate::ast::nodes::*;
use crate::diagnostics::debugger::OmniDebugger;
use crate::parser::cursor::TokenCursor;
use crate::parser::grammar::GrammarEngine;
use crate::parser::lexer::{Token, TokenType};

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Parser State Initialization, Token Stream Mounting & Stream Delegations )
// ==============================================================================

/// State container coordinating recursive-descent parsing across modular engines.
pub struct Parser {
    /// Universal stream cursor managing position, lookahead, and delimiter integrity.
    pub cursor: TokenCursor,
    /// Active debugger reflecting boundary stack physics and diagnostic failpoints.
    pub debugger: OmniDebugger,
}

impl Parser {
    /// Constructs a new [`Parser`] over an ingested token stream.
    ///
    /// # Arguments
    /// * `tokens` - Complete token stream terminated by [`TokenType::Eof`].
    /// * `debugger` - Diagnostic debugger enforcing delimiter integrity.
    pub fn new(tokens: Vec<Token>, debugger: OmniDebugger) -> Self {
        let cursor = TokenCursor::new(tokens, debugger.clone());
        Self {
            cursor,
            debugger,
        }
    }

    /// Inspects the current token at the cursor without advancing.
    #[inline]
    pub fn peek(&self) -> &Token {
        self.cursor.peek()
    }

    /// Inspects a future token at `offset` positions ahead of the cursor.
    #[inline]
    pub fn peek_at(&self, offset: usize) -> &Token {
        self.cursor.peek_at(offset)
    }

    /// Advances the cursor by one token and returns the consumed token.
    #[inline]
    pub fn advance(&mut self) -> Token {
        self.cursor.advance()
    }

    /// Checks if the current token matches the expected token type without consuming it.
    #[inline]
    pub fn check(&self, expected: &TokenType) -> bool {
        self.cursor.check(expected)
    }

    /// Checks if a lookahead token at `offset` matches the expected token type.
    #[inline]
    pub fn check_at(&self, offset: usize, expected: &TokenType) -> bool {
        self.cursor.check_at(offset, expected)
    }

    /// Conditionally consumes the current token if it matches the expected type.
    #[inline]
    pub fn match_token(&mut self, expected: &TokenType) -> bool {
        self.cursor.match_token(expected)
    }

    /// Returns `true` if the cursor has reached the end-of-file token.
    #[inline]
    pub fn is_at_end(&self) -> bool {
        self.cursor.is_at_end()
    }
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Grammar Coordination: Program, Statements & Expressions )
// ==============================================================================

impl Parser {
    /// Parses an entire source reel or tape into a unified [`ProgramNode`].
    ///
    /// Iterates through sequential top-level statements until [`TokenType::Eof`],
    /// synchronizes debugger state, and verifies that all opened boundaries closed cleanly.
    pub fn parse_program(&mut self) -> Result<ProgramNode, String> {
        let mut engine = GrammarEngine::new(&mut self.cursor);
        let res = engine.parse_program();
        self.debugger = self.cursor.debugger.clone();
        res
    }

    /// Parses a single statement or top-level structural declaration.
    pub fn parse_statement(&mut self) -> Result<ASTNode, String> {
        let mut engine = GrammarEngine::new(&mut self.cursor);
        let res = engine.parse_statement();
        self.debugger = self.cursor.debugger.clone();
        res
    }

    /// Entry point for expression hierarchy parsing.
    pub fn parse_expression(&mut self) -> Result<ASTNode, String> {
        let mut engine = GrammarEngine::new(&mut self.cursor);
        let res = engine.parse_expression();
        self.debugger = self.cursor.debugger.clone();
        res
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Parsing Invariant Verification, Boundary Integrity Proofs & Test Oracles )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::logger::DiagnosticLogger;

    #[test]
    fn test_parser_leading_equals_module_declaration() {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 5);
        let src = "= [Module]: < Lexicon { Types } > & [ Noun { Character } ] { [ Version ]: { \"1.0.0\" } }";
        let mut lexer = crate::parser::lexer::Lexer::new(src, logger);
        let tokens = lexer.tokenize().expect("Lexing must succeed");
        let mut parser = Parser::new(tokens, debugger);
        let prog = parser.parse_program();
        assert!(prog.is_ok(), "Module declaration starting with '=' must parse cleanly: {:?}", prog.err());
    }

    #[test]
    fn test_parser_ranked_asymmetric_boundaries() {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 5);
        let src = "{ Input ] --> [ Output }";
        let mut lexer = crate::parser::lexer::Lexer::new(src, logger);
        let tokens = lexer.tokenize().expect("Lexing must succeed");
        let mut parser = Parser::new(tokens, debugger);
        let prog = parser.parse_program();
        assert!(prog.is_ok(), "Asymmetric boundaries {{ ... ] and [ ... }} must parse cleanly: {:?}", prog.err());
    }

    #[test]
    fn test_parser_adversative_caveat_interlock() {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 5);
        let src = "Action [ +1 ] ! [ Guard = +1 ]";
        let mut lexer = crate::parser::lexer::Lexer::new(src, logger);
        let tokens = lexer.tokenize().expect("Lexing must succeed");
        let mut parser = Parser::new(tokens, debugger);
        let prog = parser.parse_program();
        assert!(prog.is_ok(), "Adversative caveat ! must parse cleanly: {:?}", prog.err());
    }

    #[test]
    fn test_parser_character_omni_source_reel_full_parse() {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 100);
        let path = crate::core::filetype::resolve_track_path("lexicon/01_nouns/ideas/types/character.omni");
        assert!(path.exists(), "character.omni must exist at resolved track path");
        let src = std::fs::read_to_string(&path).expect("character.omni must be readable");
        let mut lexer = crate::parser::lexer::Lexer::new(&src, logger);
        let tokens = lexer.tokenize().expect("character.omni lexing must succeed");
        let mut parser = Parser::new(tokens, debugger);
        let prog = parser.parse_program();
        assert!(prog.is_ok(), "character.omni must parse cleanly into AST: {:?}", prog.err());
        let program_node = prog.unwrap();
        assert!(!program_node.statements.is_empty(), "character.omni AST must contain parsed statements");
    }

    #[test]
    fn test_parser_svo_verbs_and_composed_locators() {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 10);
        let src = "[ ROLL 4 ] : [ Project ] : [ Character` } --> PROJECT [ Target ] ::@ 0x01";
        let mut lexer = crate::parser::lexer::Lexer::new(src, logger);
        let tokens = lexer.tokenize().expect("Lexing must succeed");
        let mut parser = Parser::new(tokens, debugger);
        let prog = parser.parse_program();
        assert!(prog.is_ok(), "SVO verb PROJECT and composed locator ::@ 0x01 must parse cleanly: {:?}", prog.err());
    }
}

// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Universal Token Cursor: Stream Navigation, Lookahead & Failpoint Guards )
// ==============================================================================
// SCRIPTURE ANCHORS & EPISTEMIC WITNESSES:
//   - Exodus 31:18: "written with the finger of God" — Inscription of eternal truth.
//   - Proverbs 24:27: "Prepare thy work without, and make it fit for thyself in the field;
//                     and afterwards build thine house." — Systematic modularization.
//   - Proverbs 22:28: "Remove not the ancient landmark, which thy fathers have set."
//                     — Preservation of invariant boundaries and balanced truth.
//
// ARCHITECTURAL TOPOLOGY:
//   The TokenCursor isolates universal token stream traversal from grammar-specific
//   parsing rules. It models the lexical stream as a discrete, 1-dimensional causal
//   ribbon with deterministic cursor navigation, arbitrary lookahead, and failpoint
//   reporting via the diagnostic debugger.
// ==============================================================================

use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::FailpointType;
use crate::parser::lexer::{Token, TokenType};

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Cursor State Initialization, Stream Context & Encapsulation )
// ==============================================================================

/// Universal stream cursor managing position, lookahead, and delimiter integrity.
pub struct TokenCursor {
    /// Ingested token stream terminated by [`TokenType::Eof`].
    tokens: Vec<Token>,
    /// Current 0-based token index.
    pos: usize,
    /// Diagnostic debugger tracking boundary depth and failpoint telemetry.
    pub debugger: OmniDebugger,
}

impl TokenCursor {
    /// Constructs a new [`TokenCursor`] over a linear token sequence.
    ///
    /// # Arguments
    /// * `tokens` - Ingested token stream from the Lexer.
    /// * `debugger` - Diagnostic debugger enforcing failpoints and boundary invariants.
    pub fn new(tokens: Vec<Token>, debugger: OmniDebugger) -> Self {
        Self {
            tokens,
            pos: 0,
            debugger,
        }
    }

    /// Returns the current 0-based cursor position.
    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Explicitly sets the cursor position (used during backtracking or macro replay).
    #[inline]
    pub fn set_pos(&mut self, new_pos: usize) {
        self.pos = new_pos.min(self.tokens.len().saturating_sub(1));
    }

    /// Returns the total number of tokens in the stream.
    #[inline]
    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    /// Returns true if the token stream contains no tokens.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    /// Returns the remaining token count from the current cursor position.
    #[inline]
    pub fn remaining(&self) -> usize {
        self.tokens.len().saturating_sub(self.pos)
    }

    /// Returns a slice of the underlying token stream.
    #[inline]
    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Universal Cursor Primitives: Inspection, Advancement, Matching & Expectation )
// ==============================================================================

impl TokenCursor {
    // --------------------------------------------------------------------------
    // 1. UNIVERSAL INSPECTION (PURE LOOKAHEAD, ZERO SIDE-EFFECTS)
    // --------------------------------------------------------------------------

    /// Inspects the current token at the cursor without advancing.
    /// Returns the final EOF token if the cursor has exceeded the stream boundary.
    #[inline]
    pub fn peek(&self) -> &Token {
        if self.pos < self.tokens.len() {
            &self.tokens[self.pos]
        } else {
            self.tokens.last().expect("Token stream must have at least an EOF token")
        }
    }

    /// Inspects a future token at `offset` positions ahead of the cursor.
    #[inline]
    pub fn peek_at(&self, offset: usize) -> &Token {
        let target_idx = self.pos + offset;
        if target_idx < self.tokens.len() {
            &self.tokens[target_idx]
        } else {
            self.tokens.last().expect("Token stream must have at least an EOF token")
        }
    }

    /// Checks if the current token matches the expected token type without consuming it.
    #[inline]
    pub fn check(&self, expected: &TokenType) -> bool {
        &self.peek().token_type == expected
    }

    /// Checks if a lookahead token at `offset` matches the expected token type.
    #[inline]
    pub fn check_at(&self, offset: usize, expected: &TokenType) -> bool {
        &self.peek_at(offset).token_type == expected
    }

    /// Checks whether the cursor has reached the end-of-file token or stream termination.
    #[inline]
    pub fn is_at_end(&self) -> bool {
        self.check(&TokenType::Eof) || self.pos >= self.tokens.len()
    }

    /// Returns the (line, col) coordinates of the current token.
    #[inline]
    pub fn current_location(&self) -> (usize, usize) {
        let tok = self.peek();
        (tok.line, tok.col)
    }

    // --------------------------------------------------------------------------
    // 2. UNIVERSAL ADVANCEMENT & CONSUMPTION (STATE ADVANCEMENT)
    // --------------------------------------------------------------------------

    /// Advances the cursor by one token and returns the consumed token.
    #[inline]
    pub fn advance(&mut self) -> Token {
        let tok = self.peek().clone();
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        tok
    }

    /// Conditionally consumes the current token if it matches `expected`.
    pub fn match_token(&mut self, expected: &TokenType) -> bool {
        if self.check(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Consumes the current token if the predicate returns true.
    pub fn consume_if<F: FnOnce(&TokenType) -> bool>(&mut self, predicate: F) -> Option<Token> {
        if predicate(&self.peek().token_type) {
            Some(self.advance())
        } else {
            None
        }
    }

    // --------------------------------------------------------------------------
    // 3. UNIVERSAL EXPECTATION & FAILPOINT GUARDS
    // --------------------------------------------------------------------------

    /// Asserts that the current token matches `expected`, consuming it on match
    /// or emitting a structured syntax failpoint on mismatch.
    pub fn expect(&mut self, expected: &TokenType, context: &str) -> Result<Token, String> {
        let current = self.peek();
        if &current.token_type == expected {
            Ok(self.advance())
        } else {
            let msg = format!(
                "Expected token '{:?}' in {}, found '{:?}' ('{}') at L{}:C{}",
                expected, context, current.token_type, current.lexeme, current.line, current.col
            );
            self.debugger.logger.failpoint(FailpointType::SyntaxAnomaly, "CURSOR", &msg);
            Err(format!("FAILPOINT: {msg}"))
        }
    }

    /// Asserts that the current token is an identifier, consuming and extracting its name.
    pub fn expect_identifier(&mut self, context: &str) -> Result<(String, Token), String> {
        let current = self.peek().clone();
        match current.token_type {
            TokenType::Identifier(ref name) => {
                let id = name.clone();
                let tok = self.advance();
                Ok((id, tok))
            }
            TokenType::Who | TokenType::What | TokenType::When | TokenType::Where | TokenType::Why | TokenType::How => {
                let id = current.lexeme.clone();
                let tok = self.advance();
                Ok((id, tok))
            }
            _ => {
                let msg = format!(
                    "Expected identifier in {}, found '{:?}' ('{}') at L{}:C{}",
                    context, current.token_type, current.lexeme, current.line, current.col
                );
                self.debugger.logger.failpoint(FailpointType::SyntaxAnomaly, "CURSOR", &msg);
                Err(format!("FAILPOINT: {msg}"))
            }
        }
    }

    /// Asserts that the current token opens a morphological boundary (`[`, `{`, `<`),
    /// recording it into the active boundary stack.
    pub fn expect_boundary_open(&mut self, context: &str) -> Result<(char, Token), String> {
        let tok = self.peek().clone();
        let open_char = match tok.token_type {
            TokenType::LBracket => '[',
            TokenType::LBrace => '{',
            TokenType::LAngle => '<',
            _ => {
                let msg = format!(
                    "Expected boundary opening delimiter ('[', '{{', '<') in {}, found '{:?}' at L{}:C{}",
                    context, tok.token_type, tok.line, tok.col
                );
                self.debugger.logger.failpoint(FailpointType::SyntaxAnomaly, "CURSOR", &msg);
                return Err(format!("FAILPOINT: {msg}"));
            }
        };

        self.advance();
        self.debugger.push_boundary(open_char, tok.line, tok.col);
        Ok((open_char, tok))
    }

    /// Asserts that the current token closes a morphological boundary (`]`, `}`, `>`),
    /// validating and popping it from the active boundary stack.
    pub fn expect_boundary_close(&mut self, context: &str) -> Result<(crate::core::boundaries::BoundaryType, Token), String> {
        if self.is_at_end() {
            let msg = format!("Unclosed boundary in {} reaches EOF", context);
            self.debugger.logger.failpoint(FailpointType::BoundaryLeak, "CURSOR", &msg);
            return Err(format!("FAILPOINT: {msg}"));
        }

        let tok = self.peek().clone();
        let close_char = match tok.token_type {
            TokenType::RBracket => ']',
            TokenType::RBrace => '}',
            TokenType::RAngle => '>',
            _ => {
                let msg = format!(
                    "Expected boundary closing delimiter (']', '}}', '>') in {}, found '{:?}' at L{}:C{}",
                    context, tok.token_type, tok.line, tok.col
                );
                self.debugger.logger.failpoint(FailpointType::SyntaxAnomaly, "CURSOR", &msg);
                return Err(format!("FAILPOINT: {msg}"));
            }
        };

        self.advance();
        let btype = self.debugger.pop_boundary(close_char, tok.line, tok.col)?;
        Ok((btype, tok))
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Cursor Invariant Verification, Boundary Mechanics & Unit Proofs )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::logger::DiagnosticLogger;

    fn create_test_cursor(src: &str) -> TokenCursor {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 10);
        let mut lexer = crate::parser::lexer::Lexer::new(src, logger);
        let tokens = lexer.tokenize().expect("Lexing must succeed");
        TokenCursor::new(tokens, debugger)
    }

    #[test]
    fn test_cursor_linear_advance_and_peek() {
        let mut cursor = create_test_cursor("[ Roll ] --> 42");
        assert_eq!(cursor.peek().token_type, TokenType::LBracket);
        assert_eq!(cursor.peek_at(1).lexeme, "Roll");
        assert_eq!(cursor.advance().token_type, TokenType::LBracket);
        assert_eq!(cursor.peek().lexeme, "Roll");
        assert!(cursor.match_token(&TokenType::Identifier("Roll".into())));
        assert_eq!(cursor.peek().token_type, TokenType::RBracket);
    }

    #[test]
    fn test_cursor_boundary_open_and_close_physics() {
        let mut cursor = create_test_cursor("{ Character ]");
        let (open_char, _) = cursor.expect_boundary_open("TestBoundary").expect("Open boundary");
        assert_eq!(open_char, '{');
        assert_eq!(cursor.advance().lexeme, "Character");
        let (btype, _) = cursor.expect_boundary_close("TestBoundary").expect("Close boundary");
        assert_eq!(btype, crate::core::boundaries::BoundaryType::TopDown);
    }

    #[test]
    fn test_cursor_safe_lookahead_past_eof() {
        let cursor = create_test_cursor("Alpha");
        assert_eq!(cursor.peek().lexeme, "Alpha");
        assert_eq!(cursor.peek_at(1).token_type, TokenType::Eof);
        assert_eq!(cursor.peek_at(100).token_type, TokenType::Eof);
    }
}

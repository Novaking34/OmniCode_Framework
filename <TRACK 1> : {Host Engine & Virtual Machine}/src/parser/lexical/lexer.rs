// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Lexer Identity, Ontological TokenType Re-exports & Morphemic Invariants )
// ==============================================================================
//! # OmniCode Lexical Tokenizer (`lexer.rs`)
//!
//! Transforms raw source text reels into structured [`Token`] streams, enforcing
//! the physical and ontological boundary invariants of OmniCode.
//!
//! ## Scriptural Grounding:
//! - **Exodus 31:18**: *"And he gave unto Moses, when he had made an end of communing
//!   with him upon mount Sinai, two tables of testimony, tables of stone, written with
//!   the finger of God."*
//!   The Lexer transcribes divine mathematical axioms from the source reel into
//!   discrete, uncorrupted tokens of stone.
//! - **Proverbs 24:27**: *"Prepare thy work without, and make it fit for thyself in
//!   the field; and afterwards build thine house."*
//!   Morphemic tokens are meticulously prepared and validated in isolation before
//!   the syntactic house is erected by the grammar engine.
//! - **Proverbs 22:28**: *"Remove not the ancient landmark, which thy fathers have set."*
//!   Preserves the 6 ranked boundary landmarks and invariant operators across all reels.
//!
//! ## Architectural Foundations:
//! 1. **Modular Sub-Engine Structure**:
//!    - Token definitions and morphemes reside in [`crate::parser::tokens`].
//!    - Universal character streaming and coordinate tracking reside in [`crate::parser::char_stream`].
//!    - Language-specific compounding, state-primes, and boundary logic reside here.
//!
//! 2. **The 6 Ranked Morphological Boundary Delimiters**:
//!    - Rank 1 `< ... >`: Invariant Axiom (Absolute Immutable Law & Ontological Truth)
//!    - Rank 2 `[ ... ]`: Denotation (Concrete State Container & Evaluation Block)
//!    - Rank 3 `{ ... }`: Connotation (Context, Environment & Payload Attributes)
//!    - Rank 4 `[ ... }`: Inside-Out Emergence (Output Projection to External Universe)
//!    - Rank 5 `{ ... ]`: Outside-In Imposition (Input Capture from External Universe)
//!    - Rank 6 `( ... )`: Intent Aside (First-Class Cognitive Teleology & Rationale)
//!
//! 3. **First-Class Machine Intent**:
//!    Asides enclosed in `( ... )` are **never** discarded as trivia or comments.
//!    They are preserved in the token stream and lowered into the AST as persistent
//!    teleological metadata nodes, recording the *why* of every machine state.
//!
//! 4. **Causal State-Primes (`Character'`, ```Character` ```)**:
//!    Discrete clock cycles advance an entity from state $S$ to state $S'$. The state-prime
//!    operator is tokenized as part of the identifier, designating causality without collision.
//!
//! 5. **Dual-Substrate Arithmetic & Hardware Locators**:
//!    Supports balanced-ternary literals (`+1:PROCEED`, `-1:ROLLBACK`, `0:AWAIT`),
//!    hexadecimal hardware addresses (`0x01`, `0x000B8000`), hardware locators (`::@`),
//!    SVO natural arithmetic (`plus`, `minus`, `times`, `div`), and agentic pronouns.

pub use crate::parser::tokens::{SourceLocation, Span, Token, TokenType};
pub use crate::parser::char_stream::{CharCheckpoint, CharCursor};

use crate::core::boundaries::BoundaryType;
use crate::core::operators::{AffixPosition, Polarity};
use crate::core::ternary::QuadState;
use crate::core::linguistics::PronounKind;
use crate::diagnostics::logger::{DiagnosticLogger, FailpointType};


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Lexer State Structure & Coordinator Initializer )
// ==============================================================================

/// State container for source text reel tokenization.
pub struct Lexer<'a> {
    /// Universal character stream cursor.
    cursor: CharCursor<'a>,
    /// Depth counter for nested square brackets `[...]`.
    pub bracket_depth: usize,
    /// Diagnostic logger recording failpoint telemetry and lexical events.
    pub logger: DiagnosticLogger,
}

impl<'a> Lexer<'a> {
    /// Constructs a new [`Lexer`] over an OmniCode source reel string.
    ///
    /// # Arguments
    /// * `source` - Complete OmniCode UTF-8 source string.
    /// * `logger` - Diagnostic logger for telemetry and failpoint recording.
    pub fn new(source: &'a str, logger: DiagnosticLogger) -> Self {
        Self {
            cursor: CharCursor::new(source),
            bracket_depth: 0,
            logger,
        }
    }

    /// Read-only reference to the internal character cursor.
    #[inline]
    pub fn cursor(&self) -> &CharCursor<'a> {
        &self.cursor
    }
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Core Tokenization Transformation Pipeline & Modular Scanning Engines )
// ==============================================================================
impl<'a> Lexer<'a> {
    /// Tokenizes the entire source reel into a complete [`Token`] stream.
    ///
    /// Evaluates sequential lexical passes:
    /// 1. Skip whitespace and `#` comments.
    /// 2. First-class Intent Asides `( ... )`.
    /// 3. Multi-character compound operators and directional vectors.
    /// 4. Prefixed polarity identifiers (`+Word`, `-Word`).
    /// 5. Single-character morphological boundaries and operators.
    /// 6. Numeric literals (Hexadecimal and Decimal).
    /// 7. Identifiers, State-Primes (`Word'`, ```Word` ```), Postfix Polarities, Pronouns, and Keywords.
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while !self.cursor.is_at_end() {
            self.cursor.skip_whitespace_and_comments('#');
            if self.cursor.is_at_end() {
                break;
            }

            let cur_line = self.cursor.line();
            let cur_col = self.cursor.col();

            // ------------------------------------------------------------------
            // 1. FIRST-CLASS INTENT ASIDES: ( ... )
            // ------------------------------------------------------------------
            if self.cursor.peek() == Some('(') {
                self.cursor.advance();
                tokens.push(self.read_aside(cur_line, cur_col)?);
                continue;
            }

            // ------------------------------------------------------------------
            // 2. MULTI-CHARACTER COMPOUND OPERATORS, VECTORS & LOCATORS
            // ------------------------------------------------------------------
            if let Some(tok) = self.match_compound_operators(cur_line, cur_col) {
                tokens.push(tok);
                continue;
            }

            // ------------------------------------------------------------------
            // 3. PREFIXED POLARITY IDENTIFIERS: +Word (Abstract), -Word (Concrete)
            // ------------------------------------------------------------------
            if (self.cursor.peek() == Some('+') || self.cursor.peek() == Some('-'))
                && self.cursor.peek_matches(1, |c| c.is_alphabetic() || c == '_')
            {
                tokens.push(self.read_prefixed_polarity(cur_line, cur_col));
                continue;
            }

            // ------------------------------------------------------------------
            // 4. BOUNDARY DELIMITERS, NUMBERS, STRINGS & IDENTIFIERS
            // ------------------------------------------------------------------
            let ch = self.cursor.peek().unwrap();
            match ch {
                // Morphological Boundaries: [ ] { } < >
                '[' | ']' | '{' | '}' | '<' | '>' => {
                    tokens.push(self.read_boundary_delimiter(ch, tokens.last(), cur_line, cur_col));
                }

                // Single-Character Operators & Connectives
                '=' | '!' | ':' | '|' | '?' | '@' | '%' | '&' | '+' | '-' | '*' => {
                    tokens.push(self.read_single_char_operator(ch, cur_line, cur_col));
                }

                // String & Character Literals ('...' or "...")
                '"' | '\'' => {
                    self.cursor.advance();
                    tokens.push(self.read_string_or_char(ch, cur_line, cur_col)?);
                }

                // Statement and Element Separators
                ';' | ',' => {
                    self.cursor.advance();
                }

                // Numeric Literals (Hexadecimal `0x...` or Decimal)
                _ if ch.is_ascii_digit() => {
                    tokens.push(self.read_number(ch, cur_line, cur_col)?);
                }

                // Identifiers, State-Primes, Pronouns, and Keywords
                _ if ch.is_alphabetic() || ch == '_' => {
                    tokens.push(self.read_identifier_or_keyword(cur_line, cur_col));
                }

                // Unrecognized Character Anomaly
                other => {
                    let msg = format!("Unexpected character '{other}' at L{cur_line}:C{cur_col}");
                    self.logger.failpoint(FailpointType::SyntaxAnomaly, "LEXER", &msg);
                    return Err(format!("FAILPOINT: {msg}"));
                }
            }
        }

        tokens.push(Token::eof(self.cursor.line(), self.cursor.col()));
        Ok(tokens)
    }

    /// Matches multi-character compound operators, directional vectors, casts, and locators
    /// using zero-allocation matching via [`CharCursor`].
    fn match_compound_operators(&mut self, cur_line: usize, cur_col: usize) -> Option<Token> {
        // Freeform Boundary Casts: [+], [-], <+>, <->, {+}, {-}
        if self.cursor.match_str("[+]") {
            return Some(Token::new(TokenType::BoundaryCast { boundary: BoundaryType::Denotation, polarity: Polarity::Abstract }, "[+]", cur_line, cur_col));
        }
        if self.cursor.match_str("[-]") {
            return Some(Token::new(TokenType::BoundaryCast { boundary: BoundaryType::Denotation, polarity: Polarity::Concrete }, "[-]", cur_line, cur_col));
        }
        if self.cursor.match_str("<+>") {
            return Some(Token::new(TokenType::BoundaryCast { boundary: BoundaryType::Invariant, polarity: Polarity::Abstract }, "<+>", cur_line, cur_col));
        }
        if self.cursor.match_str("<->") {
            return Some(Token::new(TokenType::BoundaryCast { boundary: BoundaryType::Invariant, polarity: Polarity::Concrete }, "<->", cur_line, cur_col));
        }
        if self.cursor.match_str("{+}") {
            return Some(Token::new(TokenType::BoundaryCast { boundary: BoundaryType::Connotation, polarity: Polarity::Abstract }, "{+}", cur_line, cur_col));
        }
        if self.cursor.match_str("{-}") {
            return Some(Token::new(TokenType::BoundaryCast { boundary: BoundaryType::Connotation, polarity: Polarity::Concrete }, "{-}", cur_line, cur_col));
        }

        // 3-Character Directional Vectors, Equivalence & Hardware Locator
        if self.cursor.match_str("-->") {
            return Some(Token::new(TokenType::ActUpon, "-->", cur_line, cur_col));
        }
        if self.cursor.match_str("<--") {
            return Some(Token::new(TokenType::ActAgain, "<--", cur_line, cur_col));
        }
        if self.cursor.match_str("--^") || self.cursor.match_str("^--") {
            return Some(Token::new(TokenType::BuildsTo, "^--", cur_line, cur_col));
        }
        if self.cursor.match_str("--v") || self.cursor.match_str("v--") {
            return Some(Token::new(TokenType::BreaksDownFrom, "v--", cur_line, cur_col));
        }
        if self.cursor.match_str("<=>") {
            return Some(Token::new(TokenType::InvariantEquivalence, "<=>", cur_line, cur_col));
        }
        if self.cursor.match_str("::@") {
            return Some(Token::new(TokenType::HardwareLocator, "::@", cur_line, cur_col));
        }

        // 2-Character Directional Vectors, Relations & Connectives
        if self.cursor.match_str("->") {
            return Some(Token::new(TokenType::ActUpon, "->", cur_line, cur_col));
        }
        if self.cursor.match_str("<-") {
            return Some(Token::new(TokenType::ActAgain, "<-", cur_line, cur_col));
        }
        if self.cursor.match_str("-^") {
            return Some(Token::new(TokenType::BuildsTo, "-^", cur_line, cur_col));
        }
        if self.cursor.match_str("-v") {
            return Some(Token::new(TokenType::BreaksDownFrom, "-v", cur_line, cur_col));
        }
        if self.cursor.match_str("::") {
            return Some(Token::new(TokenType::And, "::", cur_line, cur_col));
        }
        if self.cursor.match_str("!=") {
            return Some(Token::new(TokenType::IsNot, "!=", cur_line, cur_col));
        }
        if self.cursor.match_str("~=") {
            return Some(Token::new(TokenType::ToBeLike, "~=", cur_line, cur_col));
        }
        if self.cursor.match_str("~!") {
            return Some(Token::new(TokenType::But, "~!", cur_line, cur_col));
        }
        if self.cursor.match_str("<=") {
            return Some(Token::new(TokenType::LessEqual, "<=", cur_line, cur_col));
        }
        if self.cursor.match_str(">=") {
            return Some(Token::new(TokenType::GreaterEqual, ">=", cur_line, cur_col));
        }
        if self.cursor.match_str("--") {
            return Some(Token::new(TokenType::Not, "--", cur_line, cur_col));
        }

        // Diagonal Scale Modulations & 2D Quadrants
        if self.cursor.match_str("/-^") {
            return Some(Token::new(TokenType::Major, "/-^", cur_line, cur_col));
        }
        if self.cursor.match_str("\\-v") {
            return Some(Token::new(TokenType::Minor, "\\-v", cur_line, cur_col));
        }
        if self.cursor.match_str("\\-^") {
            return Some(Token::new(TokenType::Great, "\\-^", cur_line, cur_col));
        }
        if self.cursor.match_str("/-v") {
            return Some(Token::new(TokenType::Small, "/-v", cur_line, cur_col));
        }
        if self.cursor.match_str("/^") {
            return Some(Token::new(TokenType::UpperRight, "/^", cur_line, cur_col));
        }
        if self.cursor.match_str("\\v") {
            return Some(Token::new(TokenType::LowerRight, "\\v", cur_line, cur_col));
        }
        if self.cursor.match_str("^\\") {
            return Some(Token::new(TokenType::UpperLeft, "^\\", cur_line, cur_col));
        }
        if self.cursor.match_str("v/") {
            return Some(Token::new(TokenType::LowerLeft, "v/", cur_line, cur_col));
        }
        if self.cursor.match_str("..") {
            return Some(Token::new(TokenType::DotDot, "..", cur_line, cur_col));
        }

        // Signed Balanced-Ternary Literals: +1, -1
        if self.cursor.match_str("+1") {
            return Some(Token::new(TokenType::Ternary(QuadState::Proceed), "+1", cur_line, cur_col));
        }
        if self.cursor.match_str("-1") {
            return Some(Token::new(TokenType::Ternary(QuadState::Rollback), "-1", cur_line, cur_col));
        }

        None
    }

    /// Scans a first-class teleological Intent Aside enclosed in `( ... )`.
    fn read_aside(&mut self, start_line: usize, start_col: usize) -> Result<Token, String> {
        let mut depth = 1;
        let mut content = String::new();

        while let Some(ch) = self.cursor.advance() {
            match ch {
                '(' => {
                    depth += 1;
                    content.push(ch);
                }
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        let text = content.trim().to_string();
                        self.logger.log("ASIDE", format!("Captured Aside: ({text})"));
                        return Ok(Token::new(TokenType::Aside(text), format!("({content})"), start_line, start_col));
                    }
                    content.push(ch);
                }
                _ => content.push(ch),
            }
        }

        let msg = format!("Unclosed Aside '(' opened at L{start_line}:C{start_col} reaches EOF");
        self.logger.failpoint(FailpointType::BoundaryLeak, "LEXER", &msg);
        Err(format!("FAILPOINT [UnclosedAside]: {msg}"))
    }

    /// Scans a quoted string literal (`"..."`) or atomic character literal (`'...'`).
    fn read_string_or_char(&mut self, quote: char, start_line: usize, start_col: usize) -> Result<Token, String> {
        let mut s = String::new();
        while let Some(ch) = self.cursor.advance() {
            if ch == quote {
                // If single quote with single character, recognize as atomic Character glyph
                if quote == '\'' && s.chars().count() == 1 {
                    let c = s.chars().next().unwrap();
                    return Ok(Token::new(TokenType::CharLit(c), format!("'{s}'"), start_line, start_col));
                }
                return Ok(Token::new(TokenType::StringLit(s.clone()), format!("\"{s}\""), start_line, start_col));
            } else if ch == '\\' {
                if let Some(esc) = self.cursor.advance() {
                    match esc {
                        'n' => s.push('\n'),
                        'r' => s.push('\r'),
                        't' => s.push('\t'),
                        '\\' => s.push('\\'),
                        '\'' => s.push('\''),
                        '"' => s.push('"'),
                        '0' => s.push('\0'),
                        other => s.push(other),
                    }
                }
            } else {
                s.push(ch);
            }
        }
        let msg = format!("Unclosed literal opened at L{start_line}:C{start_col} reaches EOF");
        self.logger.failpoint(FailpointType::SyntaxAnomaly, "LEXER", &msg);
        Err(format!("FAILPOINT: {msg}"))
    }

    /// Scans a prefixed polarity identifier (`+Word` or `-Word`).
    fn read_prefixed_polarity(&mut self, cur_line: usize, cur_col: usize) -> Token {
        let sign = self.cursor.advance().unwrap();
        let polarity = if sign == '+' { Polarity::Abstract } else { Polarity::Concrete };
        let ident = self.cursor.consume_while(|c| c.is_alphanumeric() || c == '_' || c == '.' || c == '?');
        let lexeme = format!("{}{}", sign, ident);
        Token::new(
            TokenType::AffixedIdent {
                name: ident,
                polarity,
                position: AffixPosition::Prefix,
            },
            lexeme,
            cur_line,
            cur_col,
        )
    }

    /// Scans a single-character morphological boundary delimiter.
    fn read_boundary_delimiter(&mut self, ch: char, last_token: Option<&Token>, cur_line: usize, cur_col: usize) -> Token {
        self.cursor.advance();
        match ch {
            '[' => {
                self.bracket_depth += 1;
                Token::new(TokenType::LBracket, "[", cur_line, cur_col)
            }
            ']' => {
                self.bracket_depth = self.bracket_depth.saturating_sub(1);
                Token::new(TokenType::RBracket, "]", cur_line, cur_col)
            }
            '{' => Token::new(TokenType::LBrace, "{", cur_line, cur_col),
            '}' => Token::new(TokenType::RBrace, "}", cur_line, cur_col),
            '<' => {
                // Canon 3 Disambiguation: Casted context comparison [ { A } < { B } ]
                if self.bracket_depth > 0
                    && last_token.map_or(false, |t| t.token_type == TokenType::RBrace)
                    && self.cursor.peek_next_non_whitespace(0) == Some('{')
                {
                    Token::new(TokenType::LessThan, "<", cur_line, cur_col)
                } else {
                    Token::new(TokenType::LAngle, "<", cur_line, cur_col)
                }
            }
            '>' => {
                // Canon 3 Disambiguation: Casted context comparison [ { A } > { B } ]
                if self.bracket_depth > 0
                    && last_token.map_or(false, |t| t.token_type == TokenType::RBrace)
                    && self.cursor.peek_next_non_whitespace(0) == Some('{')
                {
                    Token::new(TokenType::GreaterThan, ">", cur_line, cur_col)
                } else {
                    Token::new(TokenType::RAngle, ">", cur_line, cur_col)
                }
            }
            _ => unreachable!(),
        }
    }

    /// Scans a single-character operator or connective.
    fn read_single_char_operator(&mut self, ch: char, cur_line: usize, cur_col: usize) -> Token {
        self.cursor.advance();
        match ch {
            '=' => Token::new(TokenType::Is, "=", cur_line, cur_col),
            '!' => Token::new(TokenType::But, "!", cur_line, cur_col),
            ':' => Token::new(TokenType::InRelationTo, ":", cur_line, cur_col),
            '|' => Token::new(TokenType::Or, "|", cur_line, cur_col),
            '?' => Token::new(TokenType::Infer, "?", cur_line, cur_col),
            '@' => Token::new(TokenType::At, "@", cur_line, cur_col),
            '%' => Token::new(TokenType::Percent, "%", cur_line, cur_col),
            '&' => Token::new(TokenType::BoundPair, "&", cur_line, cur_col),
            '+' => Token::new(TokenType::Plus, "+", cur_line, cur_col),
            '-' => Token::new(TokenType::Minus, "-", cur_line, cur_col),
            '*' => Token::new(TokenType::Times, "*", cur_line, cur_col),
            _ => unreachable!(),
        }
    }

    /// Scans numeric literals, including hexadecimal (`0x...`), standalone ternary zero `0`,
    /// and decimal integers/floats.
    fn read_number(&mut self, ch: char, cur_line: usize, cur_col: usize) -> Result<Token, String> {
        // Hexadecimal hardware address locator: 0x01, 0x000B8000
        if ch == '0' && (self.cursor.peek_at(1) == Some('x') || self.cursor.peek_at(1) == Some('X')) {
            self.cursor.advance(); // consume '0'
            self.cursor.advance(); // consume 'x' or 'X'
            let hex_str = self.cursor.consume_while(|c| c.is_ascii_hexdigit());
            let val = u64::from_str_radix(&hex_str, 16)
                .map_err(|e| format!("FAILPOINT: Invalid hex number '0x{hex_str}': {e}"))?;
            return Ok(Token::new(TokenType::Number(val as f64), format!("0x{hex_str}"), cur_line, cur_col));
        }

        // Standalone ternary zero `0` representing Await / Equilibrium
        if ch == '0' && !self.cursor.peek_matches(1, |c| c.is_alphanumeric() || c == '.') {
            self.cursor.advance();
            return Ok(Token::new(TokenType::Ternary(QuadState::Await), "0", cur_line, cur_col));
        }

        // Decimal integer or floating point number
        let num_str = self.cursor.consume_while(|c| c.is_ascii_digit() || c == '.');
        let val: f64 = num_str.parse().map_err(|e| format!("FAILPOINT: Invalid number '{num_str}': {e}"))?;
        Ok(Token::new(TokenType::Number(val), num_str, cur_line, cur_col))
    }

    /// Scans identifiers, SVO verbs, state-primes (`Name'`, ```Name` ```), postfix polarities,
    /// pronouns, and grammatical keywords.
    fn read_identifier_or_keyword(&mut self, cur_line: usize, cur_col: usize) -> Token {
        // Accumulate identifier characters, including state-prime (`) and ticks (')
        let ident = self.cursor.consume_while(|c| {
            c.is_alphanumeric() || c == '_' || c == '.' || c == '?' || c == '\'' || c == '`'
        });

        // Check for attached Postfix Polarity: Word+ (Abstract), Word- (Concrete)
        let mut postfix_polarity = None;
        if self.cursor.peek() == Some('+') {
            let next_after = self.cursor.peek_at(1);
            if next_after.map_or(true, |c| {
                c.is_whitespace() || c == ']' || c == '}' || c == '>' || c == ')' || c == ',' || c == ';' || c == '!' || c == '@' || c == ':'
            }) {
                self.cursor.advance();
                postfix_polarity = Some(Polarity::Abstract);
            }
        } else if self.cursor.peek() == Some('-') {
            let next_after = self.cursor.peek_at(1);
            if next_after.map_or(true, |c| {
                c != '>' && c != '-' && c != '^' && c != 'v' && (
                    c.is_whitespace() || c == ']' || c == '}' || c == '>' || c == ')' || c == ',' || c == ';' || c == '!' || c == '@' || c == ':'
                )
            }) {
                self.cursor.advance();
                postfix_polarity = Some(Polarity::Concrete);
            }
        }

        if let Some(polarity) = postfix_polarity {
            let lexeme = format!("{}{}", ident, polarity.symbol());
            return Token::new(
                TokenType::AffixedIdent {
                    name: ident,
                    polarity,
                    position: AffixPosition::Postfix,
                },
                lexeme,
                cur_line,
                cur_col,
            );
        }

        // Disambiguate keywords, pronouns, ternary states, and general identifiers
        match ident.to_lowercase().as_str() {
            "null" | "void" | "halt" | "stop" => Token::new(TokenType::Ternary(QuadState::Halt), ident, cur_line, cur_col),
            "yes" | "proceed" | "play" => Token::new(TokenType::Ternary(QuadState::Proceed), ident, cur_line, cur_col),
            "ok" | "await" | "pause" => Token::new(TokenType::Ternary(QuadState::Await), ident, cur_line, cur_col),
            "no" | "rollback" | "rewind" => Token::new(TokenType::Ternary(QuadState::Rollback), ident, cur_line, cur_col),
            "plus" => Token::new(TokenType::Plus, ident, cur_line, cur_col),
            "minus" => Token::new(TokenType::Minus, ident, cur_line, cur_col),
            "times" => Token::new(TokenType::Times, ident, cur_line, cur_col),
            "div" => Token::new(TokenType::Div, ident, cur_line, cur_col),
            "who" => Token::new(TokenType::Who, ident, cur_line, cur_col),
            "what" => Token::new(TokenType::What, ident, cur_line, cur_col),
            "when" => Token::new(TokenType::When, ident, cur_line, cur_col),
            "where" => Token::new(TokenType::Where, ident, cur_line, cur_col),
            "why" => Token::new(TokenType::Why, ident, cur_line, cur_col),
            "how" => Token::new(TokenType::How, ident, cur_line, cur_col),
            _ => {
                if let Some(p) = PronounKind::from_keyword(&ident) {
                    Token::new(TokenType::Pronoun(p), ident, cur_line, cur_col)
                } else {
                    Token::new(TokenType::Identifier(ident.clone()), ident, cur_line, cur_col)
                }
            }
        }
    }
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Lexical Verification Proofs, Boundary Integrity & Test Oracles )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexing_pronouns_and_reflexives() {
        let logger = DiagnosticLogger::new(false);
        let mut lexer = Lexer::new("I you it they myself itself my your _", logger);
        let tokens = lexer.tokenize().expect("Lexing pronouns");

        assert_eq!(tokens[0].token_type, TokenType::Pronoun(PronounKind::I));
        assert_eq!(tokens[1].token_type, TokenType::Pronoun(PronounKind::You));
        assert_eq!(tokens[2].token_type, TokenType::Pronoun(PronounKind::It));
        assert_eq!(tokens[3].token_type, TokenType::Pronoun(PronounKind::They));
        assert_eq!(tokens[4].token_type, TokenType::Pronoun(PronounKind::Myself));
        assert_eq!(tokens[5].token_type, TokenType::Pronoun(PronounKind::Itself));
        assert_eq!(tokens[6].token_type, TokenType::Pronoun(PronounKind::My));
        assert_eq!(tokens[7].token_type, TokenType::Pronoun(PronounKind::Your));
        assert_eq!(tokens[8].token_type, TokenType::Pronoun(PronounKind::Wildcard));
    }

    #[test]
    fn test_lexing_hexadecimal_numbers_and_locators() {
        let logger = DiagnosticLogger::new(false);
        let mut lexer = Lexer::new("0x01 ::@ 0x000B8000", logger);
        let tokens = lexer.tokenize().expect("Lexing hex numbers");

        assert_eq!(tokens[0].token_type, TokenType::Number(1.0));
        assert!(tokens[1].is_hardware_locator());
        assert_eq!(tokens[1].token_type, TokenType::HardwareLocator);
        assert_eq!(tokens[1].lexeme, "::@");
        assert_eq!(tokens[2].token_type, TokenType::Number(0xB8000 as f64));
    }

    #[test]
    fn test_lexing_state_primes_and_svo_verbs() {
        let logger = DiagnosticLogger::new(false);
        let mut lexer = Lexer::new("Character` --> INSCRIBE [ Cell ] = [ Character` ]", logger);
        let tokens = lexer.tokenize().expect("Lexing state primes and verbs");

        assert_eq!(tokens[0].lexeme, "Character`");
        assert!(tokens[0].is_state_prime());
        assert_eq!(tokens[1].token_type, TokenType::ActUpon);
        assert_eq!(tokens[2].lexeme, "INSCRIBE");
        assert_eq!(tokens[3].token_type, TokenType::LBracket);
        assert_eq!(tokens[4].lexeme, "Cell");
        assert_eq!(tokens[5].token_type, TokenType::RBracket);
        assert_eq!(tokens[6].token_type, TokenType::Is);
        assert_eq!(tokens[7].token_type, TokenType::LBracket);
        assert_eq!(tokens[8].lexeme, "Character`");
        assert_eq!(tokens[9].token_type, TokenType::RBracket);
    }

    #[test]
    fn test_lexing_relational_bounds_and_transduction() {
        let logger = DiagnosticLogger::new(false);
        let mut lexer = Lexer::new("65 <=> [ -1, +1, +1, -1, +1 ]", logger);
        let tokens = lexer.tokenize().expect("Lexing transduction and trits");

        assert_eq!(tokens[0].token_type, TokenType::Number(65.0));
        assert_eq!(tokens[1].token_type, TokenType::InvariantEquivalence);
        assert_eq!(tokens[2].token_type, TokenType::LBracket);
        assert_eq!(tokens[3].token_type, TokenType::Ternary(QuadState::Rollback));
        assert_eq!(tokens[4].token_type, TokenType::Ternary(QuadState::Proceed));
    }

    #[test]
    fn test_lexing_single_char_glyph_and_string() {
        let logger = DiagnosticLogger::new(false);
        let mut lexer = Lexer::new("'A' \"Hello\"", logger);
        let tokens = lexer.tokenize().expect("Lexing glyph and string");

        assert_eq!(tokens[0].token_type, TokenType::CharLit('A'));
        assert_eq!(tokens[1].token_type, TokenType::StringLit("Hello".into()));
    }

    #[test]
    fn test_lexing_character_omni_source_reel_token_stream() {
        let logger = DiagnosticLogger::new(false);
        let path = crate::core::filetype::resolve_track_path("lexicon/01_nouns/ideas/types/character.omni");
        assert!(path.exists(), "character.omni must exist");
        let src = std::fs::read_to_string(&path).expect("Read character.omni");
        let mut lexer = Lexer::new(&src, logger);
        let tokens = lexer.tokenize().expect("Tokenize character.omni");

        assert!(!tokens.is_empty(), "Token stream must not be empty");
        assert_eq!(tokens.last().unwrap().token_type, TokenType::Eof);
    }
}

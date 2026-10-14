// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Lexer Identity, Ontological TokenType Hierarchy & Morphemic Invariants )
// ==============================================================================
//! # OmniCode Lexical Tokenizer (`lexer.rs`)
//!
//! Transforms raw source text reels into structured [`Token`] streams, enforcing
//! the physical and ontological boundary invariants of OmniCode:
//!
//! ## Architectural Foundations:
//! 1. **Morphemic Compounding**:
//!    OmniCode constructs compound operators by fusing base characters without arbitrary
//!    jargon (e.g. `::` + `@` = `::@` "and is located at"; `~` + `=` = `~=` "to be like";
//!    `<` + `=` + `>` = `<=>` "invariant equivalence / transduction").
//!
//! 2. **The 6 Ranked Morphological Boundary Delimiters**:
//!    - Rank 1 `< ... >`: Invariant Axiom (Absolute Immutable Law & Ontological Truth)
//!    - Rank 2 `[ ... ]`: Denotation (Concrete State Container & Silicon Evaluation Block)
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
//! 4. **Causal State-Primes (``` ` ```)**:
//!    Discrete clock cycles advance an entity from state $S$ to state $S'$. The state-prime
//!    operator (``` ` ```) is tokenized as part of the identifier (e.g. `Character```, `Cell```,
//!    `Status```, `Target```, `Text```), designating causality without string collision.
//!
//! 5. **Dual-Substrate Arithmetic & Pronouns**:
//!    Supports balanced-ternary literals (`+1:PROCEED`, `-1:ROLLBACK`, `0:AWAIT`),
//!    hexadecimal hardware addresses (`0x01`, `0x000B8000`), SVO natural arithmetic
//!    (`plus`, `minus`, `times`, `div`), and agentic pronouns (`WHO`, `WHAT`, `WHEN`,
//!    `WHERE`, `WHY`, `HOW`, `I`, `you`, `it`, `they`, `myself`, `itself`, `_`).

use crate::core::boundaries::BoundaryType;
use crate::core::operators::{AffixPosition, Polarity};
use crate::core::ternary::QuadState;
use crate::core::linguistics::PronounKind;
use crate::diagnostics::logger::{DiagnosticLogger, FailpointType};

/// Complete taxonomy of grammatical tokens emitted by the Lexer.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // --------------------------------------------------------------------------
    // 1. Boundary Delimiters & First-Class Asides
    // --------------------------------------------------------------------------
    /// `[` Opening Denotation / Bottom-Up Emergence
    LBracket,
    /// `]` Closing Denotation / Top-Down Imposition
    RBracket,
    /// `{` Opening Connotation / Top-Down Imposition
    LBrace,
    /// `}` Closing Connotation / Bottom-Up Emergence
    RBrace,
    /// `<` Opening Invariant Axiom
    LAngle,
    /// `>` Closing Invariant Axiom
    RAngle,
    /// `( ... )` First-class Intent Aside capturing architectural rationale
    Aside(String),

    // --------------------------------------------------------------------------
    // 2. Grammatical Pronouns & Reflexives
    // --------------------------------------------------------------------------
    /// Agentic pronouns (`I`, `you`, `it`, `they`, `myself`, `itself`, `_`, etc.)
    Pronoun(PronounKind),

    // --------------------------------------------------------------------------
    // 3. Polarity & Boundary Cast Operators
    // --------------------------------------------------------------------------
    /// Affixed Identifiers (`+Word` Abstract, `-Word` Concrete, `Word+`, `Word-`)
    AffixedIdent {
        name: String,
        polarity: Polarity,
        position: AffixPosition,
    },
    /// Boundary Casts (`[+]`, `[-]`, `<+>`, `<->`, `{+}`, `{-}`)
    BoundaryCast {
        boundary: BoundaryType,
        polarity: Polarity,
    },

    // --------------------------------------------------------------------------
    // 4. Directional Vectors (Axial, Diagonal Scale Modulations & 2D Quadrants)
    // --------------------------------------------------------------------------
    /// `-->` / `->` Act Upon (Forward Vector / Transduction)
    ActUpon,
    /// `<--` / `<-` Act Again (Feedback / Rollback / Reverse Vector)
    ActAgain,
    /// `^--` / `--^` / `-^` Builds To (Ascension / Synthesis Vector)
    BuildsTo,
    /// `v--` / `--v` / `-v` Breaks Down From (Descension / Decomposition Vector)
    BreaksDownFrom,
    /// `/-^` Major scale up (Ascend forward)
    Major,
    /// `\-v` Minor scale down (Descend forward)
    Minor,
    /// `\-^` Great scale up (Ascend backward)
    Great,
    /// `/-v` Small scale down (Descend backward)
    Small,
    /// `/^` Quadrant I / Northeast
    UpperRight,
    /// `\v` Quadrant IV / Southeast
    LowerRight,
    /// `^\` Quadrant II / Northwest
    UpperLeft,
    /// `v/` Quadrant III / Southwest
    LowerLeft,

    // --------------------------------------------------------------------------
    // 5. Relations of Being & Invariant Transduction
    // --------------------------------------------------------------------------
    /// `=` Is (Relational Identity / Root Assignment)
    Is,
    /// `!=` Is Not (Relational Disparity)
    IsNot,
    /// `~=` To Be Like (Fuzzy Affinity / Morphological Similarity)
    ToBeLike,
    /// `<=>` Invariant Equivalence (Bijective Dimensional Transduction: Bits <=> Trits)
    InvariantEquivalence,
    /// `<` Less Than (Casted context comparison)
    LessThan,
    /// `>` Greater Than (Casted context comparison)
    GreaterThan,
    /// `<=` Less Than or Equal To (Bound comparison: `{ Character` <= [ 1 Cell ] }`)
    LessEqual,
    /// `>=` Greater Than or Equal To (Bound comparison)
    GreaterEqual,

    // --------------------------------------------------------------------------
    // 6. Connectives, Conjunctions & Bindings
    // --------------------------------------------------------------------------
    /// `!` Adversative Caveat (A BUT B / Guard Interlock)
    But,
    /// `::` Inseparable Conjunction / `::@` Composed Locator
    And,
    /// `:` In Relation To / Type Classification
    InRelationTo,
    /// `|` Disjunctive Alternative / Branching Choice
    Or,
    /// `--` Unary Negation (Not)
    Not,
    /// `&` Inseparable Twin Binding (A&B Hand-in-Hand)
    BoundPair,

    // --------------------------------------------------------------------------
    // 7. Prepositions, Locators, Ranges & Measures
    // --------------------------------------------------------------------------
    /// `@` Located At (MMIO Address / Stream Port)
    At,
    /// `..` Coordinate Range / Interval Span
    DotDot,
    /// `%` Proportion / Fullness / Fraction
    Percent,

    // --------------------------------------------------------------------------
    // 8. Cardinal Classification Clauses (The 5 W's and 1 H)
    // --------------------------------------------------------------------------
    /// `WHO` Agent / Actor / Identity
    Who,
    /// `WHAT` Substance / Entity / Vessel
    What,
    /// `WHEN` Cadence / Lifecycle / Trigger
    When,
    /// `WHERE` Spatial Site / Memory Partition / Scope
    Where,
    /// `WHY` Teleological Intent / Rationale / Axiom
    Why,
    /// `HOW` Causal Mechanism / Status / Roll Vector
    How,

    // --------------------------------------------------------------------------
    // 9. SVO Arithmetic Keywords
    // --------------------------------------------------------------------------
    /// `plus`, `add`
    Plus,
    /// `minus`, `sub`
    Minus,
    /// `times`, `mult`
    Times,
    /// `div`, `over`, `divided_by`
    Div,

    // --------------------------------------------------------------------------
    // 10. Inferential Operators & Terminals
    // --------------------------------------------------------------------------
    /// `?` Infer / Epistemic Query
    Infer,
    /// Balanced-Ternary Literals (`+1`, `-1`, `0`, `NULL`, `PROCEED`, `ROLLBACK`, `AWAIT`, `HALT`)
    Ternary(QuadState),
    /// Numeric Literals (Hexadecimal `0x...`, Decimal Integers, Floats)
    Number(f64),
    /// String Literals (`"..."`, `'...'`)
    StringLit(String),
    /// Generic Identifiers & SVO Verbs (supporting state-prime ``` ` ```)
    Identifier(String),
    /// End-of-file sentinel
    Eof,
}

/// Inscribed token unit with source reel coordinate tracking.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// Grammatical token classification.
    pub token_type: TokenType,
    /// Exact character sequence matched in the source text.
    pub lexeme: String,
    /// 1-indexed line coordinate in source reel.
    pub line: usize,
    /// 1-indexed column coordinate in source reel.
    pub col: usize,
}


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Lexer State Structure, Cursor Navigation & Scanning Primitives )
// ==============================================================================
/// State container for source text reel tokenization.
pub struct Lexer<'a> {
    /// Original borrowed source text.
    _source: &'a str,
    /// Decoded Unicode characters representing the source reel tape.
    chars: Vec<char>,
    /// Current character cursor position.
    pos: usize,
    /// Active line number (1-indexed).
    line: usize,
    /// Active column number (1-indexed).
    col: usize,
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
            _source: source,
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
            bracket_depth: 0,
            logger,
        }
    }

    /// Inspects a future character at `offset` positions ahead without advancing.
    #[inline]
    fn peek(&self, offset: usize) -> Option<char> {
        let idx = self.pos + offset;
        if idx < self.chars.len() {
            Some(self.chars[idx])
        } else {
            None
        }
    }

    /// Finds the next non-whitespace character starting at `start_offset`.
    #[inline]
    fn peek_next_non_whitespace(&self, start_offset: usize) -> Option<char> {
        let mut off = start_offset;
        while let Some(ch) = self.peek(off) {
            if !ch.is_whitespace() {
                return Some(ch);
            }
            off += 1;
        }
        None
    }

    /// Advances the cursor by one character, updating line and column coordinates.
    #[inline]
    fn advance(&mut self) -> Option<char> {
        if self.pos < self.chars.len() {
            let ch = self.chars[self.pos];
            self.pos += 1;
            if ch == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
            Some(ch)
        } else {
            None
        }
    }

    /// Matches and consumes a target string literal if it matches characters at cursor.
    fn match_str(&mut self, target: &str) -> bool {
        let target_chars: Vec<char> = target.chars().collect();
        for (i, &tc) in target_chars.iter().enumerate() {
            if self.peek(i) != Some(tc) {
                return false;
            }
        }
        for _ in 0..target_chars.len() {
            self.advance();
        }
        true
    }

    /// Skips whitespace and line comments (`# ...`).
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek(0) {
            if ch.is_whitespace() {
                self.advance();
            } else if ch == '#' {
                while let Some(c) = self.peek(0) {
                    if c == '\n' {
                        break;
                    }
                    self.advance();
                }
            } else {
                break;
            }
        }
    }

    /// Scans a first-class teleological Intent Aside enclosed in `( ... )`.
    ///
    /// Preserves nested parentheses and trims surrounding whitespace.
    fn read_aside(&mut self, start_line: usize, start_col: usize) -> Result<Token, String> {
        let mut depth = 1;
        let mut content = String::new();

        while let Some(ch) = self.advance() {
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
                        return Ok(Token {
                            token_type: TokenType::Aside(text),
                            lexeme: format!("({content})"),
                            line: start_line,
                            col: start_col,
                        });
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

    /// Scans a quoted string literal (`"..."` or `'...'`).
    fn read_string(&mut self, quote: char, start_line: usize, start_col: usize) -> Result<Token, String> {
        let mut s = String::new();
        while let Some(ch) = self.advance() {
            if ch == quote {
                return Ok(Token {
                    token_type: TokenType::StringLit(s.clone()),
                    lexeme: format!("\"{s}\""),
                    line: start_line,
                    col: start_col,
                });
            } else if ch == '\\' {
                if let Some(esc) = self.advance() {
                    s.push(esc);
                }
            } else {
                s.push(ch);
            }
        }
        let msg = format!("Unclosed string opened at L{start_line}:C{start_col} reaches EOF");
        self.logger.failpoint(FailpointType::SyntaxAnomaly, "LEXER", &msg);
        Err(format!("FAILPOINT: {msg}"))
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
    /// 7. Identifiers, State-Primes (```Word` ```), Postfix Polarities, Pronouns, and Keywords.
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while self.peek(0).is_some() {
            self.skip_whitespace();
            if self.peek(0).is_none() {
                break;
            }

            let cur_line = self.line;
            let cur_col = self.col;

            // ------------------------------------------------------------------
            // 1. FIRST-CLASS INTENT ASIDES: ( ... )
            // ------------------------------------------------------------------
            if self.peek(0) == Some('(') {
                self.advance();
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
            if (self.peek(0) == Some('+') || self.peek(0) == Some('-'))
                && self.peek(1).map_or(false, |c| c.is_alphabetic() || c == '_')
            {
                tokens.push(self.read_prefixed_polarity(cur_line, cur_col));
                continue;
            }

            // ------------------------------------------------------------------
            // 4. BOUNDARY DELIMITERS, NUMBERS, STRINGS & IDENTIFIERS
            // ------------------------------------------------------------------
            let ch = self.peek(0).unwrap();
            match ch {
                // Morphological Boundaries: [ ] { } < >
                '[' | ']' | '{' | '}' | '<' | '>' => {
                    tokens.push(self.read_boundary_delimiter(ch, tokens.last(), cur_line, cur_col));
                }

                // Single-Character Operators & Connectives
                '=' | '!' | ':' | '|' | '?' | '@' | '%' | '&' | '+' | '-' | '*' => {
                    tokens.push(self.read_single_char_operator(ch, cur_line, cur_col));
                }

                // String Literals
                '"' | '\'' => {
                    self.advance();
                    tokens.push(self.read_string(ch, cur_line, cur_col)?);
                }

                // Statement and Element Separators
                ';' | ',' => {
                    self.advance();
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

        tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".into(),
            line: self.line,
            col: self.col,
        });
        Ok(tokens)
    }

    /// Matches multi-character compound operators, directional vectors, casts, and locators.
    fn match_compound_operators(&mut self, cur_line: usize, cur_col: usize) -> Option<Token> {
        // Freeform Boundary Casts: [+], [-], <+>, <->, {+}, {-}
        if self.match_str("[+]") {
            return Some(Token { token_type: TokenType::BoundaryCast { boundary: BoundaryType::Denotation, polarity: Polarity::Abstract }, lexeme: "[+]".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("[-]") {
            return Some(Token { token_type: TokenType::BoundaryCast { boundary: BoundaryType::Denotation, polarity: Polarity::Concrete }, lexeme: "[-]".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("<+>") {
            return Some(Token { token_type: TokenType::BoundaryCast { boundary: BoundaryType::Invariant, polarity: Polarity::Abstract }, lexeme: "<+>".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("<->") {
            return Some(Token { token_type: TokenType::BoundaryCast { boundary: BoundaryType::Invariant, polarity: Polarity::Concrete }, lexeme: "<->".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("{+}") {
            return Some(Token { token_type: TokenType::BoundaryCast { boundary: BoundaryType::Connotation, polarity: Polarity::Abstract }, lexeme: "{+}".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("{-}") {
            return Some(Token { token_type: TokenType::BoundaryCast { boundary: BoundaryType::Connotation, polarity: Polarity::Concrete }, lexeme: "{-}".into(), line: cur_line, col: cur_col });
        }

        // 3-Character Directional Vectors & Equivalence
        if self.match_str("-->") {
            return Some(Token { token_type: TokenType::ActUpon, lexeme: "-->".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("<--") {
            return Some(Token { token_type: TokenType::ActAgain, lexeme: "<--".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("--^") || self.match_str("^--") {
            return Some(Token { token_type: TokenType::BuildsTo, lexeme: "^--".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("--v") || self.match_str("v--") {
            return Some(Token { token_type: TokenType::BreaksDownFrom, lexeme: "v--".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("<=>") {
            return Some(Token { token_type: TokenType::InvariantEquivalence, lexeme: "<=>".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("::@") {
            return Some(Token { token_type: TokenType::And, lexeme: "::@".into(), line: cur_line, col: cur_col });
        }

        // 2-Character Directional Vectors, Relations & Connectives
        if self.match_str("->") {
            return Some(Token { token_type: TokenType::ActUpon, lexeme: "->".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("<-") {
            return Some(Token { token_type: TokenType::ActAgain, lexeme: "<-".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("-^") {
            return Some(Token { token_type: TokenType::BuildsTo, lexeme: "-^".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("-v") {
            return Some(Token { token_type: TokenType::BreaksDownFrom, lexeme: "-v".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("::") {
            return Some(Token { token_type: TokenType::And, lexeme: "::".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("!=") {
            return Some(Token { token_type: TokenType::IsNot, lexeme: "!=".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("~=") {
            return Some(Token { token_type: TokenType::ToBeLike, lexeme: "~=".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("~!") {
            return Some(Token { token_type: TokenType::But, lexeme: "~!".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("<=") {
            return Some(Token { token_type: TokenType::LessEqual, lexeme: "<=".into(), line: cur_line, col: cur_col });
        }
        if self.match_str(">=") {
            return Some(Token { token_type: TokenType::GreaterEqual, lexeme: ">=".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("--") {
            return Some(Token { token_type: TokenType::Not, lexeme: "--".into(), line: cur_line, col: cur_col });
        }

        // Diagonal Scale Modulations & 2D Quadrants
        if self.match_str("/-^") {
            return Some(Token { token_type: TokenType::Major, lexeme: "/-^".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("\\-v") {
            return Some(Token { token_type: TokenType::Minor, lexeme: "\\-v".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("\\-^") {
            return Some(Token { token_type: TokenType::Great, lexeme: "\\-^".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("/-v") {
            return Some(Token { token_type: TokenType::Small, lexeme: "/-v".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("/^") {
            return Some(Token { token_type: TokenType::UpperRight, lexeme: "/^".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("\\v") {
            return Some(Token { token_type: TokenType::LowerRight, lexeme: "\\v".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("^\\") {
            return Some(Token { token_type: TokenType::UpperLeft, lexeme: "^\\".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("v/") {
            return Some(Token { token_type: TokenType::LowerLeft, lexeme: "v/".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("..") {
            return Some(Token { token_type: TokenType::DotDot, lexeme: "..".into(), line: cur_line, col: cur_col });
        }

        // Signed Balanced-Ternary Literals: +1, -1
        if self.match_str("+1") {
            return Some(Token { token_type: TokenType::Ternary(QuadState::Proceed), lexeme: "+1".into(), line: cur_line, col: cur_col });
        }
        if self.match_str("-1") {
            return Some(Token { token_type: TokenType::Ternary(QuadState::Rollback), lexeme: "-1".into(), line: cur_line, col: cur_col });
        }

        None
    }

    /// Scans a prefixed polarity identifier (`+Word` or `-Word`).
    fn read_prefixed_polarity(&mut self, cur_line: usize, cur_col: usize) -> Token {
        let sign = self.advance().unwrap();
        let polarity = if sign == '+' { Polarity::Abstract } else { Polarity::Concrete };
        let mut ident = String::new();
        while let Some(c) = self.peek(0) {
            if c.is_alphanumeric() || c == '_' || c == '.' || c == '?' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }
        let lexeme = format!("{}{}", sign, ident);
        Token {
            token_type: TokenType::AffixedIdent {
                name: ident,
                polarity,
                position: AffixPosition::Prefix,
            },
            lexeme,
            line: cur_line,
            col: cur_col,
        }
    }

    /// Scans a single-character morphological boundary delimiter.
    fn read_boundary_delimiter(&mut self, ch: char, last_token: Option<&Token>, cur_line: usize, cur_col: usize) -> Token {
        self.advance();
        match ch {
            '[' => {
                self.bracket_depth += 1;
                Token { token_type: TokenType::LBracket, lexeme: "[".into(), line: cur_line, col: cur_col }
            }
            ']' => {
                self.bracket_depth = self.bracket_depth.saturating_sub(1);
                Token { token_type: TokenType::RBracket, lexeme: "]".into(), line: cur_line, col: cur_col }
            }
            '{' => Token { token_type: TokenType::LBrace, lexeme: "{".into(), line: cur_line, col: cur_col },
            '}' => Token { token_type: TokenType::RBrace, lexeme: "}".into(), line: cur_line, col: cur_col },
            '<' => {
                // Canon 3 Disambiguation: Casted context comparison [ { A } < { B } ]
                if self.bracket_depth > 0
                    && last_token.map_or(false, |t| t.token_type == TokenType::RBrace)
                    && self.peek_next_non_whitespace(0) == Some('{')
                {
                    Token { token_type: TokenType::LessThan, lexeme: "<".into(), line: cur_line, col: cur_col }
                } else {
                    Token { token_type: TokenType::LAngle, lexeme: "<".into(), line: cur_line, col: cur_col }
                }
            }
            '>' => {
                // Canon 3 Disambiguation: Casted context comparison [ { A } > { B } ]
                if self.bracket_depth > 0
                    && last_token.map_or(false, |t| t.token_type == TokenType::RBrace)
                    && self.peek_next_non_whitespace(0) == Some('{')
                {
                    Token { token_type: TokenType::GreaterThan, lexeme: ">".into(), line: cur_line, col: cur_col }
                } else {
                    Token { token_type: TokenType::RAngle, lexeme: ">".into(), line: cur_line, col: cur_col }
                }
            }
            _ => unreachable!(),
        }
    }

    /// Scans a single-character operator or connective.
    fn read_single_char_operator(&mut self, ch: char, cur_line: usize, cur_col: usize) -> Token {
        self.advance();
        match ch {
            '=' => Token { token_type: TokenType::Is, lexeme: "=".into(), line: cur_line, col: cur_col },
            '!' => Token { token_type: TokenType::But, lexeme: "!".into(), line: cur_line, col: cur_col },
            ':' => Token { token_type: TokenType::InRelationTo, lexeme: ":".into(), line: cur_line, col: cur_col },
            '|' => Token { token_type: TokenType::Or, lexeme: "|".into(), line: cur_line, col: cur_col },
            '?' => Token { token_type: TokenType::Infer, lexeme: "?".into(), line: cur_line, col: cur_col },
            '@' => Token { token_type: TokenType::At, lexeme: "@".into(), line: cur_line, col: cur_col },
            '%' => Token { token_type: TokenType::Percent, lexeme: "%".into(), line: cur_line, col: cur_col },
            '&' => Token { token_type: TokenType::BoundPair, lexeme: "&".into(), line: cur_line, col: cur_col },
            '+' => Token { token_type: TokenType::Plus, lexeme: "+".into(), line: cur_line, col: cur_col },
            '-' => Token { token_type: TokenType::Minus, lexeme: "-".into(), line: cur_line, col: cur_col },
            '*' => Token { token_type: TokenType::Times, lexeme: "*".into(), line: cur_line, col: cur_col },
            _ => unreachable!(),
        }
    }

    /// Scans numeric literals, including hexadecimal (`0x...` / `0X...`),
    /// standalone balanced-ternary zero `0` (Await), and decimal floats/integers.
    fn read_number(&mut self, ch: char, cur_line: usize, cur_col: usize) -> Result<Token, String> {
        // Hexadecimal hardware address locator: 0x01, 0x000B8000
        if ch == '0' && (self.peek(1) == Some('x') || self.peek(1) == Some('X')) {
            self.advance(); // consume '0'
            self.advance(); // consume 'x' or 'X'
            let mut hex_str = String::new();
            while let Some(c) = self.peek(0) {
                if c.is_ascii_hexdigit() {
                    hex_str.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
            let val = u64::from_str_radix(&hex_str, 16)
                .map_err(|e| format!("FAILPOINT: Invalid hex number '0x{hex_str}': {e}"))?;
            return Ok(Token {
                token_type: TokenType::Number(val as f64),
                lexeme: format!("0x{hex_str}"),
                line: cur_line,
                col: cur_col,
            });
        }

        // Standalone ternary zero `0` representing Await / Equilibrium
        if ch == '0' && !self.peek(1).map_or(false, |c| c.is_alphanumeric() || c == '.') {
            self.advance();
            return Ok(Token {
                token_type: TokenType::Ternary(QuadState::Await),
                lexeme: "0".into(),
                line: cur_line,
                col: cur_col,
            });
        }

        // Decimal integer or floating point number
        let mut num_str = String::new();
        while let Some(c) = self.peek(0) {
            if c.is_ascii_digit() || c == '.' {
                num_str.push(c);
                self.advance();
            } else {
                break;
            }
        }
        let val: f64 = num_str.parse().map_err(|e| format!("FAILPOINT: Invalid number '{num_str}': {e}"))?;
        Ok(Token {
            token_type: TokenType::Number(val),
            lexeme: num_str,
            line: cur_line,
            col: cur_col,
        })
    }

    /// Scans identifiers, SVO verbs, state-primes (``` ` ```), postfix polarities,
    /// pronouns, and grammatical keywords.
    fn read_identifier_or_keyword(&mut self, cur_line: usize, cur_col: usize) -> Token {
        let mut ident = String::new();

        // Accumulate identifier characters, including state-prime (`) and ticks (')
        while let Some(c) = self.peek(0) {
            if c.is_alphanumeric() || c == '_' || c == '.' || c == '?' || c == '\'' || c == '`' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }

        // Check for attached Postfix Polarity: Word+ (Abstract), Word- (Concrete)
        let mut postfix_polarity = None;
        if self.peek(0) == Some('+') {
            let next_after = self.peek(1);
            if next_after.map_or(true, |c| {
                c.is_whitespace() || c == ']' || c == '}' || c == '>' || c == ')' || c == ',' || c == ';' || c == '!' || c == '@' || c == ':'
            }) {
                self.advance();
                postfix_polarity = Some(Polarity::Abstract);
            }
        } else if self.peek(0) == Some('-') {
            let next_after = self.peek(1);
            if next_after.map_or(true, |c| {
                c != '>' && c != '-' && c != '^' && c != 'v' && (
                    c.is_whitespace() || c == ']' || c == '}' || c == '>' || c == ')' || c == ',' || c == ';' || c == '!' || c == '@' || c == ':'
                )
            }) {
                self.advance();
                postfix_polarity = Some(Polarity::Concrete);
            }
        }

        if let Some(polarity) = postfix_polarity {
            let lexeme = format!("{}{}", ident, polarity.symbol());
            return Token {
                token_type: TokenType::AffixedIdent {
                    name: ident,
                    polarity,
                    position: AffixPosition::Postfix,
                },
                lexeme,
                line: cur_line,
                col: cur_col,
            };
        }

        // Disambiguate keywords, pronouns, ternary states, and general identifiers
        match ident.to_lowercase().as_str() {
            "null" | "void" | "halt" | "stop" => Token {
                token_type: TokenType::Ternary(QuadState::Halt),
                lexeme: ident,
                line: cur_line,
                col: cur_col,
            },
            "yes" | "proceed" | "play" => Token {
                token_type: TokenType::Ternary(QuadState::Proceed),
                lexeme: ident,
                line: cur_line,
                col: cur_col,
            },
            "ok" | "await" | "pause" => Token {
                token_type: TokenType::Ternary(QuadState::Await),
                lexeme: ident,
                line: cur_line,
                col: cur_col,
            },
            "no" | "rollback" | "rewind" => Token {
                token_type: TokenType::Ternary(QuadState::Rollback),
                lexeme: ident,
                line: cur_line,
                col: cur_col,
            },
            "plus" => Token { token_type: TokenType::Plus, lexeme: ident, line: cur_line, col: cur_col },
            "minus" => Token { token_type: TokenType::Minus, lexeme: ident, line: cur_line, col: cur_col },
            "times" => Token { token_type: TokenType::Times, lexeme: ident, line: cur_line, col: cur_col },
            "div" => Token { token_type: TokenType::Div, lexeme: ident, line: cur_line, col: cur_col },
            "who" => Token { token_type: TokenType::Who, lexeme: ident, line: cur_line, col: cur_col },
            "what" => Token { token_type: TokenType::What, lexeme: ident, line: cur_line, col: cur_col },
            "when" => Token { token_type: TokenType::When, lexeme: ident, line: cur_line, col: cur_col },
            "where" => Token { token_type: TokenType::Where, lexeme: ident, line: cur_line, col: cur_col },
            "why" => Token { token_type: TokenType::Why, lexeme: ident, line: cur_line, col: cur_col },
            "how" => Token { token_type: TokenType::How, lexeme: ident, line: cur_line, col: cur_col },
            _ => {
                if let Some(p) = PronounKind::from_keyword(&ident) {
                    Token { token_type: TokenType::Pronoun(p), lexeme: ident, line: cur_line, col: cur_col }
                } else {
                    Token { token_type: TokenType::Identifier(ident.clone()), lexeme: ident, line: cur_line, col: cur_col }
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
        assert_eq!(tokens[1].token_type, TokenType::And);
        assert_eq!(tokens[1].lexeme, "::@");
        assert_eq!(tokens[2].token_type, TokenType::Number(0xB8000 as f64));
    }

    #[test]
    fn test_lexing_state_primes_and_svo_verbs() {
        let logger = DiagnosticLogger::new(false);
        let mut lexer = Lexer::new("Character` --> INSCRIBE [ Cell ] = [ Character` ]", logger);
        let tokens = lexer.tokenize().expect("Lexing state primes and verbs");

        assert_eq!(tokens[0].lexeme, "Character`");
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

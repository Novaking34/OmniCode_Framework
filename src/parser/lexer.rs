// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Lexer Identity, TokenType Enumeration, and Token Invariants )
// ==============================================================================
//! OmniCode Lexer
//!
//! Transforms source text into tokens, capturing all 6 boundaries, directional vectors,
//! and first-class Asides `( ... )`.

use crate::core::boundaries::BoundaryType;
use crate::core::operators::{Polarity, AffixPosition};
use crate::core::ternary::QuadState;
use crate::diagnostics::logger::{DiagnosticLogger, FailpointType};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Delimiters
    LBracket,  // [
    RBracket,  // ]
    LBrace,    // {
    RBrace,    // }
    LAngle,    // <
    RAngle,    // >
    Aside(String), // ( ... )

    // Polarity & Boundary Cast Operators
    AffixedIdent {
        name: String,
        polarity: Polarity,
        position: AffixPosition,
    },
    BoundaryCast {
        boundary: BoundaryType,
        polarity: Polarity,
    },

    // Directional Vectors (Axial, Diagonal Scale Modulations & 2D Quadrants)
    ActUpon,         // -->
    ActAgain,        // <--
    BuildsTo,        // ^-- / --^
    BreaksDownFrom,  // v-- / --v
    Major,           // /-^ (Major scale up / ascend forward)
    Minor,           // \-v (Minor scale down / descend forward)
    Great,           // \-^ (Great scale up / ascend backward)
    Small,           // /-v (Small scale down / descend backward)
    UpperRight,      // /^ (Quadrant I / northeast)
    LowerRight,      // \v (Quadrant IV / southeast)
    UpperLeft,       // ^\ (Quadrant II / northwest)
    LowerLeft,       // v/ (Quadrant III / southwest)

    // Relations of Being
    Is,         // =
    IsNot,      // !=
    ToBeLike,   // ~=

    // Connectives & Bindings
    But,            // !
    And,            // ::
    InRelationTo,   // :
    Or,             // |
    Not,            // --
    BoundPair,      // & (Inseparable Hand-in-Hand atomic binding: A&B)

    // Prepositions, Spatial Locators & Ranges
    At,             // @ (Location / Memory site / Address)
    DotDot,         // .. (Coordinate range / interval span)

    // Measures & Proportions
    Percent,        // % (Proportion / Fullness / Capacity fraction)

    // Cardinal Classification Clauses (The 5 W's and 1 H)
    Who,            // WHO (Agent / Actor / Identity)
    What,           // WHAT (Substance / Entity / Vessel)
    When,           // WHEN (Cadence / Lifecycle / Trigger)
    Where,          // WHERE (Spatial site / Scope / Invariant environment)
    Why,            // WHY (Teleological intent / Axiom / Reason)
    How,            // HOW (Causal mechanism / Vector / Method)

    // Arithmetic Math Keywords (SVO Natural Language Operations)
    Plus,           // plus, add
    Minus,          // minus, sub
    Times,          // times, mult
    Div,            // div, over, divided_by

    // Infer Operator
    Infer,          // ?

    // Literals
    Ternary(QuadState),
    Number(f64),
    StringLit(String),
    Identifier(String),

    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub col: usize,
}


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Lexer State Structure and Initialization )
// ==============================================================================
pub struct Lexer<'a> {
    _source: &'a str,
    chars: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
    pub logger: DiagnosticLogger,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str, logger: DiagnosticLogger) -> Self {
        Self {
            _source: source,
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
            logger,
        }
    }

    fn peek(&self, offset: usize) -> Option<char> {
        let idx = self.pos + offset;
        if idx < self.chars.len() {
            Some(self.chars[idx])
        } else {
            None
        }
    }

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
// ( Core Tokenization Transformation Pipeline )
// ==============================================================================
impl<'a> Lexer<'a> {
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while self.peek(0).is_some() {
            self.skip_whitespace();
            if self.peek(0).is_none() {
                break;
            }

            let cur_line = self.line;
            let cur_col = self.col;

            // 1. Aside: ( ... )
            if self.peek(0) == Some('(') {
                self.advance();
                tokens.push(self.read_aside(cur_line, cur_col)?);
                continue;
            }

            // 2. Freeform Boundary Casts: [+], [-], <+>, <->, {+}, {-}
            if self.match_str("[+]") {
                tokens.push(Token { token_type: TokenType::BoundaryCast { boundary: BoundaryType::Denotation, polarity: Polarity::Abstract }, lexeme: "[+]".into(), line: cur_line, col: cur_col });
                continue;
            }
            if self.match_str("[-]") {
                tokens.push(Token { token_type: TokenType::BoundaryCast { boundary: BoundaryType::Denotation, polarity: Polarity::Concrete }, lexeme: "[-]".into(), line: cur_line, col: cur_col });
                continue;
            }
            if self.match_str("<+>") {
                tokens.push(Token { token_type: TokenType::BoundaryCast { boundary: BoundaryType::Invariant, polarity: Polarity::Abstract }, lexeme: "<+>".into(), line: cur_line, col: cur_col });
                continue;
            }
            if self.match_str("<->") {
                tokens.push(Token { token_type: TokenType::BoundaryCast { boundary: BoundaryType::Invariant, polarity: Polarity::Concrete }, lexeme: "<->".into(), line: cur_line, col: cur_col });
                continue;
            }
            if self.match_str("{+}") {
                tokens.push(Token { token_type: TokenType::BoundaryCast { boundary: BoundaryType::Connotation, polarity: Polarity::Abstract }, lexeme: "{+}".into(), line: cur_line, col: cur_col });
                continue;
            }
            if self.match_str("{-}") {
                tokens.push(Token { token_type: TokenType::BoundaryCast { boundary: BoundaryType::Connotation, polarity: Polarity::Concrete }, lexeme: "{-}".into(), line: cur_line, col: cur_col });
                continue;
            }

            // 3. Multi-char vectors (Canonical 3-char, Shorthand 2-char & Legacy)
            if self.match_str("-->") {
                tokens.push(Token { token_type: TokenType::ActUpon, lexeme: "-->".into(), line: cur_line, col: cur_col });
                continue;
            }
            if self.match_str("<--") {
                tokens.push(Token { token_type: TokenType::ActAgain, lexeme: "<--".into(), line: cur_line, col: cur_col });
                continue;
            }
            if self.match_str("--^") {
                tokens.push(Token { token_type: TokenType::BuildsTo, lexeme: "--^".into(), line: cur_line, col: cur_col });
                continue;
            }
            if self.match_str("--v") {
                tokens.push(Token { token_type: TokenType::BreaksDownFrom, lexeme: "--v".into(), line: cur_line, col: cur_col });
                continue;
            }
            if self.match_str("^--") {
                tokens.push(Token { token_type: TokenType::BuildsTo, lexeme: "^--".into(), line: cur_line, col: cur_col });
                continue;
            }
            if self.match_str("v--") {
                tokens.push(Token { token_type: TokenType::BreaksDownFrom, lexeme: "v--".into(), line: cur_line, col: cur_col });
                continue;
            }
            if self.match_str("->") {
                tokens.push(Token { token_type: TokenType::ActUpon, lexeme: "->".into(), line: cur_line, col: cur_col });
                continue;
            }
            if self.match_str("<-") {
                tokens.push(Token { token_type: TokenType::ActAgain, lexeme: "<-".into(), line: cur_line, col: cur_col });
                continue;
            }
            if self.match_str("-^") {
                tokens.push(Token { token_type: TokenType::BuildsTo, lexeme: "-^".into(), line: cur_line, col: cur_col });
                continue;
            }
            if self.match_str("-v") {
                tokens.push(Token { token_type: TokenType::BreaksDownFrom, lexeme: "-v".into(), line: cur_line, col: cur_col });
                continue;
            }
            if self.match_str("::") {
                tokens.push(Token { token_type: TokenType::And, lexeme: "::".into(), line: cur_line, col: cur_col });
                continue;
            }
            if self.match_str("!=") {
                tokens.push(Token { token_type: TokenType::IsNot, lexeme: "!=".into(), line: cur_line, col: cur_col });
                continue;
            }
            if self.match_str("~=") {
                tokens.push(Token { token_type: TokenType::ToBeLike, lexeme: "~=".into(), line: cur_line, col: cur_col });
                continue;
            }
            if self.match_str("--") {
                tokens.push(Token { token_type: TokenType::Not, lexeme: "--".into(), line: cur_line, col: cur_col });
                continue;
            }

            // Diagonal Scale Modulations & 2D Quadrants
            if self.match_str("/-^") {
                tokens.push(Token { token_type: TokenType::Major, lexeme: "/-^".into(), line: cur_line, col: cur_col });
                continue;
            }
            if self.match_str("\\-v") {
                tokens.push(Token { token_type: TokenType::Minor, lexeme: "\\-v".into(), line: cur_line, col: cur_col });
                continue;
            }
            if self.match_str("\\-^") {
                tokens.push(Token { token_type: TokenType::Great, lexeme: "\\-^".into(), line: cur_line, col: cur_col });
                continue;
            }
            if self.match_str("/-v") {
                tokens.push(Token { token_type: TokenType::Small, lexeme: "/-v".into(), line: cur_line, col: cur_col });
                continue;
            }
            if self.match_str("/^") {
                tokens.push(Token { token_type: TokenType::UpperRight, lexeme: "/^".into(), line: cur_line, col: cur_col });
                continue;
            }
            if self.match_str("\\v") {
                tokens.push(Token { token_type: TokenType::LowerRight, lexeme: "\\v".into(), line: cur_line, col: cur_col });
                continue;
            }
            if self.match_str("^\\") {
                tokens.push(Token { token_type: TokenType::UpperLeft, lexeme: "^\\".into(), line: cur_line, col: cur_col });
                continue;
            }
            if self.match_str("v/") {
                tokens.push(Token { token_type: TokenType::LowerLeft, lexeme: "v/".into(), line: cur_line, col: cur_col });
                continue;
            }
            if self.match_str("..") {
                tokens.push(Token { token_type: TokenType::DotDot, lexeme: "..".into(), line: cur_line, col: cur_col });
                continue;
            }

            // 3. Signed ternary literals: +1, -1
            if self.match_str("+1") {
                tokens.push(Token { token_type: TokenType::Ternary(QuadState::Proceed), lexeme: "+1".into(), line: cur_line, col: cur_col });
                continue;
            }
            if self.match_str("-1") {
                tokens.push(Token { token_type: TokenType::Ternary(QuadState::Rollback), lexeme: "-1".into(), line: cur_line, col: cur_col });
                continue;
            }

            // 5. Prefixed Polarity Identifiers: +Word, -Word
            if (self.peek(0) == Some('+') || self.peek(0) == Some('-'))
                && self.peek(1).map_or(false, |c| c.is_alphabetic() || c == '_')
            {
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
                tokens.push(Token {
                    token_type: TokenType::AffixedIdent {
                        name: ident,
                        polarity,
                        position: AffixPosition::Prefix,
                    },
                    lexeme,
                    line: cur_line,
                    col: cur_col,
                });
                continue;
            }

            // 6. Single-char delimiters & operators
            let ch = self.peek(0).unwrap();
            match ch {
                '[' => {
                    self.advance();
                    tokens.push(Token { token_type: TokenType::LBracket, lexeme: "[".into(), line: cur_line, col: cur_col });
                }
                ']' => {
                    self.advance();
                    tokens.push(Token { token_type: TokenType::RBracket, lexeme: "]".into(), line: cur_line, col: cur_col });
                }
                '{' => {
                    self.advance();
                    tokens.push(Token { token_type: TokenType::LBrace, lexeme: "{".into(), line: cur_line, col: cur_col });
                }
                '}' => {
                    self.advance();
                    tokens.push(Token { token_type: TokenType::RBrace, lexeme: "}".into(), line: cur_line, col: cur_col });
                }
                '<' => {
                    self.advance();
                    tokens.push(Token { token_type: TokenType::LAngle, lexeme: "<".into(), line: cur_line, col: cur_col });
                }
                '>' => {
                    self.advance();
                    tokens.push(Token { token_type: TokenType::RAngle, lexeme: ">".into(), line: cur_line, col: cur_col });
                }
                '=' => {
                    self.advance();
                    tokens.push(Token { token_type: TokenType::Is, lexeme: "=".into(), line: cur_line, col: cur_col });
                }
                '!' => {
                    self.advance();
                    tokens.push(Token { token_type: TokenType::But, lexeme: "!".into(), line: cur_line, col: cur_col });
                }
                ':' => {
                    self.advance();
                    tokens.push(Token { token_type: TokenType::InRelationTo, lexeme: ":".into(), line: cur_line, col: cur_col });
                }
                '|' => {
                    self.advance();
                    tokens.push(Token { token_type: TokenType::Or, lexeme: "|".into(), line: cur_line, col: cur_col });
                }
                '?' => {
                    self.advance();
                    tokens.push(Token { token_type: TokenType::Infer, lexeme: "?".into(), line: cur_line, col: cur_col });
                }
                '@' => {
                    self.advance();
                    tokens.push(Token { token_type: TokenType::At, lexeme: "@".into(), line: cur_line, col: cur_col });
                }
                '%' => {
                    self.advance();
                    tokens.push(Token { token_type: TokenType::Percent, lexeme: "%".into(), line: cur_line, col: cur_col });
                }
                '&' => {
                    self.advance();
                    tokens.push(Token { token_type: TokenType::BoundPair, lexeme: "&".into(), line: cur_line, col: cur_col });
                }
                '+' => {
                    self.advance();
                    tokens.push(Token { token_type: TokenType::Plus, lexeme: "+".into(), line: cur_line, col: cur_col });
                }
                '-' => {
                    self.advance();
                    tokens.push(Token { token_type: TokenType::Minus, lexeme: "-".into(), line: cur_line, col: cur_col });
                }
                '*' => {
                    self.advance();
                    tokens.push(Token { token_type: TokenType::Times, lexeme: "*".into(), line: cur_line, col: cur_col });
                }
                '"' | '\'' => {
                    self.advance();
                    tokens.push(self.read_string(ch, cur_line, cur_col)?);
                }
                ';' | ',' => {
                    self.advance();
                }
                _ if ch.is_ascii_digit() => {
                    if ch == '0' && !self.peek(1).map_or(false, |c| c.is_alphanumeric() || c == '.') {
                        self.advance();
                        tokens.push(Token { token_type: TokenType::Ternary(QuadState::Await), lexeme: "0".into(), line: cur_line, col: cur_col });
                    } else {
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
                        tokens.push(Token { token_type: TokenType::Number(val), lexeme: num_str, line: cur_line, col: cur_col });
                    }
                }
                _ if ch.is_alphabetic() || ch == '_' => {
                    let mut ident = String::new();
                    while let Some(c) = self.peek(0) {
                        if c.is_alphanumeric() || c == '_' || c == '.' || c == '?' {
                            ident.push(c);
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    let mut postfix_polarity = None;
                    if self.peek(0) == Some('+') {
                        let next_after = self.peek(1);
                        if next_after.map_or(true, |c| c.is_whitespace() || c == ']' || c == '}' || c == '>' || c == ')' || c == ',' || c == ';' || c == '!' || c == '@' || c == ':') {
                            self.advance();
                            postfix_polarity = Some(Polarity::Abstract);
                        }
                    } else if self.peek(0) == Some('-') {
                        let next_after = self.peek(1);
                        if next_after.map_or(true, |c| c != '>' && c != '-' && c != '^' && c != 'v' && (c.is_whitespace() || c == ']' || c == '}' || c == '>' || c == ')' || c == ',' || c == ';' || c == '!' || c == '@' || c == ':')) {
                            self.advance();
                            postfix_polarity = Some(Polarity::Concrete);
                        }
                    }

                    if let Some(polarity) = postfix_polarity {
                        let lexeme = format!("{}{}", ident, polarity.symbol());
                        tokens.push(Token {
                            token_type: TokenType::AffixedIdent {
                                name: ident,
                                polarity,
                                position: AffixPosition::Postfix,
                            },
                            lexeme,
                            line: cur_line,
                            col: cur_col,
                        });
                    } else {
                        match ident.to_lowercase().as_str() {
                        "null" | "void" | "halt" | "stop" => {
                            tokens.push(Token { token_type: TokenType::Ternary(QuadState::Halt), lexeme: ident, line: cur_line, col: cur_col });
                        }
                        "yes" | "proceed" | "play" => {
                            tokens.push(Token { token_type: TokenType::Ternary(QuadState::Proceed), lexeme: ident, line: cur_line, col: cur_col });
                        }
                        "ok" | "await" | "pause" => {
                            tokens.push(Token { token_type: TokenType::Ternary(QuadState::Await), lexeme: ident, line: cur_line, col: cur_col });
                        }
                        "no" | "rollback" | "rewind" => {
                            tokens.push(Token { token_type: TokenType::Ternary(QuadState::Rollback), lexeme: ident, line: cur_line, col: cur_col });
                        }
                        "plus" => {
                            tokens.push(Token { token_type: TokenType::Plus, lexeme: ident, line: cur_line, col: cur_col });
                        }
                        "minus" => {
                            tokens.push(Token { token_type: TokenType::Minus, lexeme: ident, line: cur_line, col: cur_col });
                        }
                        "times" => {
                            tokens.push(Token { token_type: TokenType::Times, lexeme: ident, line: cur_line, col: cur_col });
                        }
                        "div" => {
                            tokens.push(Token { token_type: TokenType::Div, lexeme: ident, line: cur_line, col: cur_col });
                        }
                        "who" => {
                            tokens.push(Token { token_type: TokenType::Who, lexeme: ident, line: cur_line, col: cur_col });
                        }
                        "what" => {
                            tokens.push(Token { token_type: TokenType::What, lexeme: ident, line: cur_line, col: cur_col });
                        }
                        "when" => {
                            tokens.push(Token { token_type: TokenType::When, lexeme: ident, line: cur_line, col: cur_col });
                        }
                        "where" => {
                            tokens.push(Token { token_type: TokenType::Where, lexeme: ident, line: cur_line, col: cur_col });
                        }
                        "why" => {
                            tokens.push(Token { token_type: TokenType::Why, lexeme: ident, line: cur_line, col: cur_col });
                        }
                        "how" => {
                            tokens.push(Token { token_type: TokenType::How, lexeme: ident, line: cur_line, col: cur_col });
                        }
                        _ => {
                            tokens.push(Token { token_type: TokenType::Identifier(ident.clone()), lexeme: ident, line: cur_line, col: cur_col });
                        }
                    }
                }
                }
                other => {
                    let msg = format!("Unexpected character '{other}' at L{cur_line}:C{cur_col}");
                    self.logger.failpoint(FailpointType::SyntaxAnomaly, "LEXER", &msg);
                    return Err(format!("FAILPOINT: {msg}"));
                }
            }
        }

        tokens.push(Token { token_type: TokenType::Eof, lexeme: "".into(), line: self.line, col: self.col });
        Ok(tokens)
    }
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Token Stream Verification & Teardown )
// ==============================================================================
// Lexer tokenization pipeline verified and closed.

// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Invariant Token Taxonomy, Morphemic Classifications & Source Spans )
// ==============================================================================
//! # OmniCode Token Taxonomy (`tokens.rs`)
//!
//! Defines the ontological token vocabulary, morphemic classifications, and coordinate
//! spans emitted by the OmniCode lexical analyzer.
//!
//! ## Scriptural Grounding:
//! - **Exodus 31:18**: *"And he gave unto Moses, when he had made an end of communing
//!   with him upon mount Sinai, two tables of testimony, tables of stone, written with
//!   the finger of God."*
//!   Tokens are the immutable inscribed glyphs upon the silicon reel.
//! - **Proverbs 24:27**: *"Prepare thy work without, and make it fit for thyself in
//!   the field; and afterwards build thine house."*
//!   Lexical classification prepares every token in the field before the grammar engine
//!   builds the architectural syntactic house.
//! - **Proverbs 22:28**: *"Remove not the ancient landmark, which thy fathers have set."*
//!   Preserves ancient morphemic boundaries and structural rankings without corruption.
//!
//! ## Ontological Taxonomy:
//! 1. **Morphological Boundaries**: 6 ranked delimiters (`<...>`, `[...]`, `{...}`, `[...}`, `{...]`, `(...)`).
//! 2. **Agentic Pronouns**: Identity and capability pronouns (`I`, `you`, `it`, `they`, `myself`, etc.).
//! 3. **Polarities & Casts**: Concrete (`-`) and Abstract (`+`) affixes and boundary casts.
//! 4. **Directional Vectors**: 1D axial, 2D quadrant, and scale modulations (`-->`, `<--`, `^--`, `v--`, `/^`, `\v`).
//! 5. **Relations of Being**: Root assignment (`=`), similarity (`~=`), bounds (`<=`, `>=`), and bijective transduction (`<=>`).
//! 6. **Connectives & Locators**: Inseparable conjunctions (`::`), hardware locators (`::@`), adversarial caveats (`!`).
//! 7. **SVO Arithmetic**: Natural operation keywords (`plus`, `minus`, `times`, `div`).
//! 8. **Literals**: Balanced ternary (`+1`, `-1`, `0`, `NULL`), hex addresses (`0x...`), glyphs (`'A'`), strings (`"..."`).

use crate::core::boundaries::BoundaryType;
use crate::core::operators::{AffixPosition, Polarity};
use crate::core::ternary::QuadState;
use crate::core::linguistics::PronounKind;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Coordinates, Source Spans & Inscribed Token Unit )
// ==============================================================================

/// Precise 1-indexed source reel location with byte offset tracking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SourceLocation {
    /// 1-indexed line number in source reel.
    pub line: usize,
    /// 1-indexed column number in source reel.
    pub col: usize,
    /// 0-indexed byte offset from start of source text.
    pub offset: usize,
}

impl SourceLocation {
    /// Creates a new source location coordinate.
    pub const fn new(line: usize, col: usize, offset: usize) -> Self {
        Self { line, col, offset }
    }
}

/// A contiguous source span from `start` coordinate to `end` coordinate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    /// Inclusive starting coordinate.
    pub start: SourceLocation,
    /// Inclusive ending coordinate.
    pub end: SourceLocation,
}

impl Span {
    /// Creates a new span spanning from `start` to `end`.
    pub const fn new(start: SourceLocation, end: SourceLocation) -> Self {
        Self { start, end }
    }

    /// Creates an atomic single-point span.
    pub const fn point(loc: SourceLocation) -> Self {
        Self { start: loc, end: loc }
    }
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

impl Token {
    /// Constructs a new inscribed [`Token`].
    #[inline]
    pub fn new(token_type: TokenType, lexeme: impl Into<String>, line: usize, col: usize) -> Self {
        Self {
            token_type,
            lexeme: lexeme.into(),
            line,
            col,
        }
    }

    /// Constructs an End-Of-File sentinel token.
    #[inline]
    pub fn eof(line: usize, col: usize) -> Self {
        Self {
            token_type: TokenType::Eof,
            lexeme: String::new(),
            line,
            col,
        }
    }

    /// Returns `true` if this token is an EOF sentinel.
    #[inline]
    pub fn is_eof(&self) -> bool {
        self.token_type == TokenType::Eof
    }

    /// Checks if this token is a hardware locator (`::@`), supporting both first-class
    /// [`TokenType::HardwareLocator`] and legacy compounded [`TokenType::And`] with `"::@"`.
    #[inline]
    pub fn is_hardware_locator(&self) -> bool {
        self.token_type.is_hardware_locator(&self.lexeme)
    }

    /// Returns `true` if this token is an identifier designated as a state-prime (`Name'` or ```Name` ```).
    #[inline]
    pub fn is_state_prime(&self) -> bool {
        match &self.token_type {
            TokenType::Identifier(id) => TokenType::is_state_prime_name(id),
            _ => false,
        }
    }
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Complete Ontological TokenType Hierarchy & Categorical Methods )
// ==============================================================================

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
    /// `::` Inseparable Conjunction
    And,
    /// `::@` Dedicated Hardware Locator / MMIO Port Binding
    HardwareLocator,
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
    /// Atomic Character Glyph Literal (`'A'`, `'\n'`)
    CharLit(char),
    /// String Literals (`"..."`)
    StringLit(String),
    /// Generic Identifiers & SVO Verbs (supporting state-prime ``` ` ``` and `'`)
    Identifier(String),
    /// End-of-file sentinel
    Eof,
}

impl TokenType {
    /// Returns `true` if this token marks an opening morphological boundary (`[`, `{`, `<`).
    pub fn is_open_boundary(&self) -> bool {
        matches!(self, TokenType::LBracket | TokenType::LBrace | TokenType::LAngle)
    }

    /// Returns `true` if this token marks a closing morphological boundary (`]`, `}`, `>`).
    pub fn is_close_boundary(&self) -> bool {
        matches!(self, TokenType::RBracket | TokenType::RBrace | TokenType::RAngle)
    }

    /// Returns `true` if this token is any morphological boundary delimiter.
    pub fn is_boundary(&self) -> bool {
        self.is_open_boundary() || self.is_close_boundary()
    }

    /// Returns `true` if this token represents a directional vector.
    pub fn is_directional_vector(&self) -> bool {
        matches!(
            self,
            TokenType::ActUpon
                | TokenType::ActAgain
                | TokenType::BuildsTo
                | TokenType::BreaksDownFrom
                | TokenType::Major
                | TokenType::Minor
                | TokenType::Great
                | TokenType::Small
                | TokenType::UpperRight
                | TokenType::LowerRight
                | TokenType::UpperLeft
                | TokenType::LowerLeft
        )
    }

    /// Returns `true` if this token represents a relation of being or comparison.
    pub fn is_relational(&self) -> bool {
        matches!(
            self,
            TokenType::Is
                | TokenType::IsNot
                | TokenType::ToBeLike
                | TokenType::InvariantEquivalence
                | TokenType::LessThan
                | TokenType::GreaterThan
                | TokenType::LessEqual
                | TokenType::GreaterEqual
        )
    }

    /// Returns `true` if this token represents an arithmetic operator or keyword.
    pub fn is_arithmetic(&self) -> bool {
        matches!(self, TokenType::Plus | TokenType::Minus | TokenType::Times | TokenType::Div)
    }

    /// Identifies hardware locators (`::@`), supporting both dedicated [`TokenType::HardwareLocator`]
    /// and backward-compatible [`TokenType::And`] with lexeme `"::@"`.
    pub fn is_hardware_locator(&self, lexeme: &str) -> bool {
        matches!(self, TokenType::HardwareLocator)
            || (matches!(self, TokenType::And) && lexeme == "::@")
    }

    /// Checks if a string identifier represents a causal state-prime ($S \to S'$).
    pub fn is_state_prime_name(name: &str) -> bool {
        name.ends_with('`') || name.ends_with('\'')
    }

    /// Returns the stripped canonical base name without state-prime notation.
    pub fn strip_state_prime(name: &str) -> &str {
        if let Some(stripped) = name.strip_suffix('`') {
            stripped
        } else if let Some(stripped) = name.strip_suffix('\'') {
            stripped
        } else {
            name
        }
    }
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Token Taxonomy Unit Proofs & Morphemic Invariant Tests )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_construction_and_eof() {
        let tok = Token::new(TokenType::Identifier("Character`".into()), "Character`", 1, 5);
        assert_eq!(tok.line, 1);
        assert_eq!(tok.col, 5);
        assert_eq!(tok.lexeme, "Character`");
        assert!(tok.is_state_prime());
        assert!(!tok.is_eof());

        let eof = Token::eof(10, 1);
        assert!(eof.is_eof());
    }

    #[test]
    fn test_hardware_locator_identification() {
        let dedicated = Token::new(TokenType::HardwareLocator, "::@", 1, 1);
        assert!(dedicated.is_hardware_locator());

        let legacy = Token::new(TokenType::And, "::@", 1, 1);
        assert!(legacy.is_hardware_locator());

        let regular_and = Token::new(TokenType::And, "::", 1, 1);
        assert!(!regular_and.is_hardware_locator());
    }

    #[test]
    fn test_state_prime_stripping() {
        assert_eq!(TokenType::strip_state_prime("Character`"), "Character");
        assert_eq!(TokenType::strip_state_prime("Cell'"), "Cell");
        assert_eq!(TokenType::strip_state_prime("Static"), "Static");
    }

    #[test]
    fn test_token_category_predicates() {
        assert!(TokenType::LBracket.is_open_boundary());
        assert!(TokenType::RBracket.is_close_boundary());
        assert!(TokenType::ActUpon.is_directional_vector());
        assert!(TokenType::InvariantEquivalence.is_relational());
        assert!(TokenType::Plus.is_arithmetic());
    }
}

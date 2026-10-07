//! OmniCode Core: Boundary Mechanics & Morphological Delimiters
//!
//! Delimiters are active morphological operators that govern the boundary physics
//! between an entity and its surrounding universe.
//!
//! The 6 Canonical Boundaries:
//!     1. Denotation  : [ ... ]  (Mutable Definition)
//!     2. Invariant   : < ... >  (Immutable Axiom / Constant)
//!     3. Connotation : { ... }  (In Context Of / Environment)
//!     4. TopDown     : { ... ]  (Context Defines Entity)
//!     5. BottomUp    : [ ... }  (Entity Defined by Context)
//!     6. Aside       : ( ... )  (Aside To / First-Class Machine Intent)

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoundaryType {
    /// [ ... ] Defined as (Mutable)
    Denotation,
    /// < ... > Defined as (Immutable Axiom)
    Invariant,
    /// { ... } In context of (Environment)
    Connotation,
    /// { ... ] Context defines (Outside-In causality)
    TopDown,
    /// [ ... } Defined by context (Inside-Out causality)
    BottomUp,
    /// ( ... ) Aside to (Intent / Semantic annotation)
    Aside,
}

impl BoundaryType {
    /// Returns the (open, close) character pair for this boundary type.
    pub fn delimiters(&self) -> (char, char) {
        match self {
            BoundaryType::Denotation => ('[', ']'),
            BoundaryType::Invariant => ('<', '>'),
            BoundaryType::Connotation => ('{', '}'),
            BoundaryType::TopDown => ('{', ']'),
            BoundaryType::BottomUp => ('[', '}'),
            BoundaryType::Aside => ('(', ')'),
        }
    }

    /// True if opening and closing delimiters come from different bracket families.
    pub fn is_asymmetric(&self) -> bool {
        matches!(self, BoundaryType::TopDown | BoundaryType::BottomUp)
    }

    /// True if boundary enforces unalterable immutability.
    pub fn is_immutable(&self) -> bool {
        matches!(self, BoundaryType::Invariant)
    }

    /// Resolves opening and closing characters into a valid BoundaryType.
    ///
    /// # Failpoint
    /// Returns `Err` if the pair forms an illegal boundary (e.g. `(`, `]` or `{`, `>`).
    pub fn from_delimiters(open: char, close: char) -> Result<Self, String> {
        match (open, close) {
            ('[', ']') => Ok(BoundaryType::Denotation),
            ('<', '>') => Ok(BoundaryType::Invariant),
            ('{', '}') => Ok(BoundaryType::Connotation),
            ('{', ']') => Ok(BoundaryType::TopDown),
            ('[', '}') => Ok(BoundaryType::BottomUp),
            ('(', ')') => Ok(BoundaryType::Aside),
            (o, c) => Err(format!(
                "FAILPOINT [IllegalBoundaryPair]: Delimiter pair '{o}...{c}' does not form a legal boundary. \
                 Legal pairs: [...] (Denotation), <...> (Invariant), {{...}} (Connotation), \
                 {{...] (Top-Down), [...}} (Bottom-Up), (...) (Aside)"
            )),
        }
    }
}

impl fmt::Display for BoundaryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (o, c) = self.delimiters();
        write!(f, "{:?}({o}...{c})", self)
    }
}

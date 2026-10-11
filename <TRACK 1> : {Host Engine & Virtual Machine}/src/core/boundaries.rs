// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Module Identity, Morphological Boundary Invariants & Delimiter Tables )
// ==============================================================================
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


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Context Declarations, BoundaryType Enum, and Delimiter Mapping )
// ==============================================================================
/// Morphological boundary types ranked by ontological hierarchy:
///   Rank 1: < ... > (Invariant / Axiom / Law)
///   Rank 2: [ ... ] (Denotation / Essence / Mutable State)
///   Rank 3: { ... } (Connotation / Ambient Context / Environment)
///   Rank 4: [ ... } (Bottom-Up / Inside-Out / Projection into Context)
///   Rank 5: { ... ] (Top-Down / Outside-In / Imposition by Context)
///   Rank 6: ( ... ) (Aside / First-Class Intent / Observer Rationale)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BoundaryType {
    /// Rank 1: < ... > Immutable Axiom / Law
    Invariant,
    /// Rank 2: [ ... ] Mutable Definition / Essence / State
    Denotation,
    /// Rank 3: { ... } Ambient Context / Environment
    Connotation,
    /// Rank 4: [ ... } Defined by context (Inside-Out / Emergent)
    BottomUp,
    /// Rank 5: { ... ] Context defines (Outside-In / Imposition)
    TopDown,
    /// Rank 6: ( ... ) Aside to (Intent / Observer Rationale)
    Aside,
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Core Resolution Logic: Boundary Properties, Directionality, Pair Matching )
// ==============================================================================
impl BoundaryType {
    /// Returns the ontological rank of the boundary (1 = Highest authority / ground truth).
    pub fn rank(&self) -> u8 {
        match self {
            BoundaryType::Invariant => 1,
            BoundaryType::Denotation => 2,
            BoundaryType::Connotation => 3,
            BoundaryType::BottomUp => 4,
            BoundaryType::TopDown => 5,
            BoundaryType::Aside => 6,
        }
    }

    /// Descriptive name of the ontological rank tier.
    pub fn rank_name(&self) -> &'static str {
        match self {
            BoundaryType::Invariant => "Rank 1: Axiomatic Invariant",
            BoundaryType::Denotation => "Rank 2: Denotative Essence",
            BoundaryType::Connotation => "Rank 3: Connotative Environment",
            BoundaryType::BottomUp => "Rank 4: Inside-Out Emergence",
            BoundaryType::TopDown => "Rank 5: Outside-In Imposition",
            BoundaryType::Aside => "Rank 6: Intentional Aside",
        }
    }

    /// Natural language grammatical clause template for this boundary.
    pub fn linguistic_clause(&self) -> &'static str {
        match self {
            BoundaryType::Invariant => "It is an invariant law that",
            BoundaryType::Denotation => "The entity defines and executes",
            BoundaryType::Connotation => "In the context of the ambient universe",
            BoundaryType::BottomUp => "Projecting outward from entity into context",
            BoundaryType::TopDown => "As conditioned and imposed by the context",
            BoundaryType::Aside => "With the intent and rationale that",
        }
    }

    /// Returns the (open, close) character pair for this boundary type.
    pub fn delimiters(&self) -> (char, char) {
        match self {
            BoundaryType::Invariant => ('<', '>'),
            BoundaryType::Denotation => ('[', ']'),
            BoundaryType::Connotation => ('{', '}'),
            BoundaryType::BottomUp => ('[', '}'),
            BoundaryType::TopDown => ('{', ']'),
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


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Display Trait Verification and String Formatter )
// ==============================================================================
impl fmt::Display for BoundaryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (o, c) = self.delimiters();
        write!(f, "{:?}({o}...{c})", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boundary_ranking_hierarchy() {
        assert_eq!(BoundaryType::Invariant.rank(), 1);
        assert_eq!(BoundaryType::Denotation.rank(), 2);
        assert_eq!(BoundaryType::Connotation.rank(), 3);
        assert_eq!(BoundaryType::BottomUp.rank(), 4);
        assert_eq!(BoundaryType::TopDown.rank(), 5);
        assert_eq!(BoundaryType::Aside.rank(), 6);

        // Verify ordinal comparison preserves exact hierarchy
        assert!(BoundaryType::Invariant < BoundaryType::Denotation);
        assert!(BoundaryType::Denotation < BoundaryType::Connotation);
        assert!(BoundaryType::Connotation < BoundaryType::BottomUp);
        assert!(BoundaryType::BottomUp < BoundaryType::TopDown);
        assert!(BoundaryType::TopDown < BoundaryType::Aside);
    }
}


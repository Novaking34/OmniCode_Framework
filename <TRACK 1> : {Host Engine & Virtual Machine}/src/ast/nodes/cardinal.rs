// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( AST Cardinal Nodes: Six Cardinal Interrogatives for Operator Dialogue )
// ==============================================================================
// SCRIPTURE ANCHORS & EPISTEMIC WITNESSES:
//   - Exodus 31:18: "written with the finger of God" — Inscription of eternal truth.
//   - Proverbs 24:27: "Prepare thy work without, and make it fit for thyself in the field;
//                     and afterwards build thine house." — Systematic modularization.
//   - Proverbs 22:28: "Remove not the ancient landmark, which thy fathers have set."
//                     — Preservation of invariant boundaries and balanced truth.
// ==============================================================================

use crate::ast::nodes::ASTNode;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( The 6 Cardinal Interrogatives & Cardinal Clause Definitions )
// ==============================================================================

/// The Six Cardinal Interrogatives governing OmniCode process tapes and machine-operator dialog.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardinalKind {
    /// WHO: The Agent, Subject, or Authority responsible for the transaction.
    Who,
    /// WHAT: The Direct Object, Inscription Target, or Data Payload.
    What,
    /// WHEN: The Trigger, Event, Clock Tick, or Temporal Precondition.
    When,
    /// WHERE: The Container, Spatial Domain, Memory Address, or Cell Space.
    Where,
    /// WHY: The Teleological Reason, Machine Intent, or Invariant Covenant.
    Why,
    /// HOW: The Modal Vector, Execution Status, or Velocity (+1, 0, NULL, -1).
    How,
}

impl CardinalKind {
    /// Returns the uppercase string keyword for the interrogative.
    pub fn keyword(&self) -> &'static str {
        match self {
            Self::Who => "WHO",
            Self::What => "WHAT",
            Self::When => "WHEN",
            Self::Where => "WHERE",
            Self::Why => "WHY",
            Self::How => "HOW",
        }
    }

    /// Parses a string into a [`CardinalKind`] if matching one of the 6 Interrogatives.
    pub fn from_str_keyword(s: &str) -> Option<Self> {
        match s.to_ascii_uppercase().as_str() {
            "WHO" => Some(Self::Who),
            "WHAT" => Some(Self::What),
            "WHEN" => Some(Self::When),
            "WHERE" => Some(Self::Where),
            "WHY" => Some(Self::Why),
            "HOW" => Some(Self::How),
            _ => None,
        }
    }
}

/// A Cardinal Clause node inside a process tape header or dialogue session:
/// `CARDINAL { Payload } ::` (e.g. `WHO { OmniCode Lexicon } ::`).
#[derive(Debug, Clone, PartialEq)]
pub struct CardinalClauseNode {
    pub kind: CardinalKind,
    pub payload: Box<ASTNode>,
    pub line: usize,
    pub col: usize,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Cardinal Constructors & Accessors )
// ==============================================================================

impl CardinalClauseNode {
    /// Constructs a new [`CardinalClauseNode`].
    pub fn new(kind: CardinalKind, payload: ASTNode, line: usize, col: usize) -> Self {
        Self {
            kind,
            payload: Box::new(payload),
            line,
            col,
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Invariant Guarantees & Trait Implementations )
// ==============================================================================
// Cardinal nodes satisfy Send, Sync, Clone, PartialEq across all targets.

// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( AST Boundary Nodes: Containers, Typology & Intent Asides )
// ==============================================================================
// SCRIPTURE ANCHORS & EPISTEMIC WITNESSES:
//   - Exodus 31:18: "written with the finger of God" — Inscription of eternal truth.
//   - Proverbs 24:27: "Prepare thy work without, and make it fit for thyself in the field;
//                     and afterwards build thine house." — Systematic modularization.
//   - Proverbs 22:28: "Remove not the ancient landmark, which thy fathers have set."
//                     — Preservation of invariant boundaries and balanced truth.
// ==============================================================================

use crate::ast::nodes::ASTNode;
use crate::core::boundaries::BoundaryType;
use crate::core::operators::Polarity;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( First-Class Asides, Boundary Nodes & Boundary Casts )
// ==============================================================================

/// First-class cognitive intent and machine reasoning aside `( ... )`.
#[derive(Debug, Clone, PartialEq)]
pub struct AsideNode {
    pub intent: String,
    pub line: usize,
    pub col: usize,
}

/// Boundary node enclosing child elements within a distinct topological frame:
/// `[...]` (Denotation), `{...}` (Connotation), `<...>` (Invariant),
/// `{...]` (Top-Down Imposition), `[...}` (Bottom-Up Emergence).
#[derive(Debug, Clone, PartialEq)]
pub struct BoundaryNode {
    pub boundary_type: BoundaryType,
    pub elements: Vec<ASTNode>,
    pub aside: Option<AsideNode>,
    pub line: usize,
    pub col: usize,
}

/// Boundary cast operator transmuting polarity or boundary topology.
#[derive(Debug, Clone, PartialEq)]
pub struct BoundaryCastNode {
    pub boundary: BoundaryType,
    pub polarity: Polarity,
    pub line: usize,
    pub col: usize,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Boundary Constructors, Predicates & Element Accessors )
// ==============================================================================

impl AsideNode {
    /// Constructs a new [`AsideNode`].
    pub fn new(intent: impl Into<String>, line: usize, col: usize) -> Self {
        Self {
            intent: intent.into(),
            line,
            col,
        }
    }
}

impl BoundaryNode {
    /// Constructs a new [`BoundaryNode`] with no attached aside.
    pub fn new(boundary_type: BoundaryType, elements: Vec<ASTNode>, line: usize, col: usize) -> Self {
        Self {
            boundary_type,
            elements,
            aside: None,
            line,
            col,
        }
    }

    /// Attaches a first-class cognitive intent aside to the boundary.
    pub fn with_aside(mut self, aside: AsideNode) -> Self {
        self.aside = Some(aside);
        self
    }

    /// Returns `true` if this boundary is an immutable invariant `< ... >`.
    pub fn is_invariant(&self) -> bool {
        self.boundary_type.is_immutable()
    }
}

impl BoundaryCastNode {
    /// Constructs a new [`BoundaryCastNode`].
    pub fn new(boundary: BoundaryType, polarity: Polarity, line: usize, col: usize) -> Self {
        Self {
            boundary,
            polarity,
            line,
            col,
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Invariant Guarantees & Trait Implementations )
// ==============================================================================
// Boundary nodes satisfy Send, Sync, Clone, PartialEq across all substrates.

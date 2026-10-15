// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( AST Ontological Nodes: Invariant Types, Entities, Analogies & Exemplars )
// ==============================================================================
// SCRIPTURE ANCHORS & EPISTEMIC WITNESSES:
//   - Exodus 31:18: "written with the finger of God" — Inscription of eternal truth.
//   - Proverbs 24:27: "Prepare thy work without, and make it fit for thyself in the field;
//                     and afterwards build thine house." — Systematic modularization.
//   - Proverbs 22:28: "Remove not the ancient landmark, which thy fathers have set."
//                     — Preservation of invariant boundaries and balanced truth.
// ==============================================================================

use crate::ast::nodes::boundaries::{AsideNode, BoundaryNode};
use crate::ast::nodes::ASTNode;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Ontological Type, Entity & Epistemic Ratio Node Definitions )
// ==============================================================================

/// Concrete typed entity allocated in memory:
/// `[ Name ]: < Invariant > & [ Substrate ] ( Aside ) { Context }`.
#[derive(Debug, Clone, PartialEq)]
pub struct EntityNode {
    pub name: String,
    pub definition: BoundaryNode,
    pub aside: Option<AsideNode>,
    pub context: Option<BoundaryNode>,
    pub line: usize,
    pub col: usize,
}

/// Freestanding Invariant Type definition:
/// `< Character > = < Idea { Type } > & [ Noun { Atomic Glyph } ] ( Aside ) { Invariants }`.
#[derive(Debug, Clone, PartialEq)]
pub struct InvariantTypeNode {
    pub name: String,
    pub category: Box<ASTNode>,
    pub aside: Option<AsideNode>,
    pub invariants: Vec<ASTNode>,
    pub line: usize,
    pub col: usize,
}

/// Ontological Proportion / Analogy node:
/// `[ Character ] : [ Text ] :: [ Trit ] : [ Word ]`.
#[derive(Debug, Clone, PartialEq)]
pub struct AnalogyNode {
    pub left_subject: Box<ASTNode>,
    pub left_context: Box<ASTNode>,
    pub right_subject: Box<ASTNode>,
    pub right_context: Box<ASTNode>,
    pub line: usize,
    pub col: usize,
}

/// Dual Substrate definition (Physical Silicon Bits & Virtual Balanced Trits):
/// `[ 1 Byte { 8 Bits } ] & [ 1 Cell { 5 Trits } ]`.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstrateNode {
    pub physical_bits: usize,
    pub virtual_trits: usize,
    pub line: usize,
    pub col: usize,
}

/// Canonical Exemplar mapping an integer scalar to balanced trits:
/// `65 <=> [ -1, +1, +1, -1, +1 ]` (Exemplar 'A').
#[derive(Debug, Clone, PartialEq)]
pub struct ExemplarNode {
    pub label: Option<String>,
    pub scalar: i64,
    pub trits: Vec<i8>,
    pub line: usize,
    pub col: usize,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Ontological Constructors & Accessors )
// ==============================================================================

impl EntityNode {
    /// Constructs a new [`EntityNode`].
    pub fn new(name: impl Into<String>, definition: BoundaryNode, line: usize, col: usize) -> Self {
        Self {
            name: name.into(),
            definition,
            aside: None,
            context: None,
            line,
            col,
        }
    }

    /// Attaches an intent aside to this entity.
    pub fn with_aside(mut self, aside: AsideNode) -> Self {
        self.aside = Some(aside);
        self
    }

    /// Attaches an ontological context boundary to this entity.
    pub fn with_context(mut self, context: BoundaryNode) -> Self {
        self.context = Some(context);
        self
    }
}

impl InvariantTypeNode {
    /// Constructs a new [`InvariantTypeNode`].
    pub fn new(name: impl Into<String>, category: ASTNode, line: usize, col: usize) -> Self {
        Self {
            name: name.into(),
            category: Box::new(category),
            aside: None,
            invariants: Vec::new(),
            line,
            col,
        }
    }

    /// Attaches an intent aside to this invariant type.
    pub fn with_aside(mut self, aside: AsideNode) -> Self {
        self.aside = Some(aside);
        self
    }
}

impl AnalogyNode {
    /// Constructs a new 4-term ontological analogy proportion.
    pub fn new(
        left_subject: ASTNode,
        left_context: ASTNode,
        right_subject: ASTNode,
        right_context: ASTNode,
        line: usize,
        col: usize,
    ) -> Self {
        Self {
            left_subject: Box::new(left_subject),
            left_context: Box::new(left_context),
            right_subject: Box::new(right_subject),
            right_context: Box::new(right_context),
            line,
            col,
        }
    }
}

impl SubstrateNode {
    /// Constructs a new dual-substrate node.
    pub fn new(physical_bits: usize, virtual_trits: usize, line: usize, col: usize) -> Self {
        Self {
            physical_bits,
            virtual_trits,
            line,
            col,
        }
    }
}

impl ExemplarNode {
    /// Constructs a new canonical exemplar mapping.
    pub fn new(label: Option<String>, scalar: i64, trits: Vec<i8>, line: usize, col: usize) -> Self {
        Self {
            label,
            scalar,
            trits,
            line,
            col,
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Invariant Guarantees & Trait Implementations )
// ==============================================================================
// Ontological nodes satisfy Send, Sync, Clone, PartialEq across all targets.

// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( AST Relational Nodes: Being Relations, Directional Vectors & Connectives )
// ==============================================================================
// SCRIPTURE ANCHORS & EPISTEMIC WITNESSES:
//   - Exodus 31:18: "written with the finger of God" — Inscription of eternal truth.
//   - Proverbs 24:27: "Prepare thy work without, and make it fit for thyself in the field;
//                     and afterwards build thine house." — Systematic modularization.
//   - Proverbs 22:28: "Remove not the ancient landmark, which thy fathers have set."
//                     — Preservation of invariant boundaries and balanced truth.
// ==============================================================================

use crate::ast::nodes::ASTNode;
use crate::core::operators::{BeingRelation, Connective, DirectionalVector};

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Relational, Directional & Connective Structural Node Definitions )
// ==============================================================================

/// Relational node evaluating ontological identity, equality, equivalence, or definition:
/// `left RELATION right` (`=`, `::`, `::=`, `~=`, `<=>`, `:`).
#[derive(Debug, Clone, PartialEq)]
pub struct RelationNode {
    pub left: Box<ASTNode>,
    pub relation: BeingRelation,
    pub right: Box<ASTNode>,
    pub line: usize,
    pub col: usize,
}

/// Directional causal propagation node driving energy and state across boundaries:
/// `source VECTOR target` (`-->`, `<--`, `<->`, `->>`, `<<-`).
#[derive(Debug, Clone, PartialEq)]
pub struct DirectionalNode {
    pub source: Box<ASTNode>,
    pub vector: DirectionalVector,
    pub target: Box<ASTNode>,
    pub line: usize,
    pub col: usize,
}

/// Connective node conjoining or disjoining expressions through formal logic:
/// `left CONNECTIVE right` (`&` And, `|` Or, `!` Adversative Caveat, `^` Xor).
#[derive(Debug, Clone, PartialEq)]
pub struct ConnectiveNode {
    pub left: Box<ASTNode>,
    pub connective: Connective,
    pub right: Box<ASTNode>,
    pub line: usize,
    pub col: usize,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Relational Constructors & Query Helpers )
// ==============================================================================

impl RelationNode {
    /// Constructs a new relational node.
    pub fn new(left: ASTNode, relation: BeingRelation, right: ASTNode, line: usize, col: usize) -> Self {
        Self {
            left: Box::new(left),
            relation,
            right: Box::new(right),
            line,
            col,
        }
    }
}

impl DirectionalNode {
    /// Constructs a new directional vector node.
    pub fn new(source: ASTNode, vector: DirectionalVector, target: ASTNode, line: usize, col: usize) -> Self {
        Self {
            source: Box::new(source),
            vector,
            target: Box::new(target),
            line,
            col,
        }
    }
}

impl ConnectiveNode {
    /// Constructs a new connective node.
    pub fn new(left: ASTNode, connective: Connective, right: ASTNode, line: usize, col: usize) -> Self {
        Self {
            left: Box::new(left),
            connective,
            right: Box::new(right),
            line,
            col,
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Invariant Guarantees & Trait Implementations )
// ==============================================================================
// Relational nodes satisfy Send, Sync, Clone, PartialEq across all targets.

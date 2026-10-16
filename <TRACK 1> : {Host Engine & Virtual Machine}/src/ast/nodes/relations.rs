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

/// Relational node evaluating ontological identity, state, equivalence:
/// `left RELATION right` (`=`, `!=`, `=!`, `~=`, `<=>`, `<`, `>`, `<=`, `>=`).
#[derive(Debug, Clone, PartialEq)]
pub struct RelationNode {
    pub left: Box<ASTNode>,
    pub relation: BeingRelation,
    pub right: Box<ASTNode>,
    pub line: usize,
    pub col: usize,
}

/// Directional causal propagation node driving energy and state across boundaries:
/// `source VECTOR target` (`-->`, `<--`, `--^`, `--v`, `->`, `<-`).
#[derive(Debug, Clone, PartialEq)]
pub struct DirectionalNode {
    pub source: Box<ASTNode>,
    pub vector: DirectionalVector,
    pub target: Box<ASTNode>,
    pub line: usize,
    pub col: usize,
}

/// Connective node conjoining or disjoining expressions through formal logic:
/// `left CONNECTIVE right` (`::` And, `!` But, `|` Or, `--` Not, `:` InRelationTo).
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
    pub fn new(
        left: ASTNode,
        relation: BeingRelation,
        right: ASTNode,
        line: usize,
        col: usize,
    ) -> Self {
        Self {
            left: Box::new(left),
            relation,
            right: Box::new(right),
            line,
            col,
        }
    }

    /// Returns `true` if this is an identity relation (`=`).
    pub fn is_identity(&self) -> bool {
        self.relation == BeingRelation::Is
    }

    /// Returns `true` if this is a negative relation (`!=` or `=!`).
    pub fn is_negation(&self) -> bool {
        matches!(
            self.relation,
            BeingRelation::IsNot | BeingRelation::ToBeNot
        )
    }

    /// Returns `true` if this is an analogical likeness relation (`~=`).
    pub fn is_likeness(&self) -> bool {
        self.relation == BeingRelation::ToBeLike
    }

    /// Returns `true` if this is invariant equivalence (`<=>`).
    pub fn is_equivalence(&self) -> bool {
        self.relation == BeingRelation::Equivalence
    }
}

impl DirectionalNode {
    /// Constructs a new directional vector node.
    pub fn new(
        source: ASTNode,
        vector: DirectionalVector,
        target: ASTNode,
        line: usize,
        col: usize,
    ) -> Self {
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
    pub fn new(
        left: ASTNode,
        connective: Connective,
        right: ASTNode,
        line: usize,
        col: usize,
    ) -> Self {
        Self {
            left: Box::new(left),
            connective,
            right: Box::new(right),
            line,
            col,
        }
    }

    /// Returns `true` if this is an adversative caveat (`!`).
    pub fn is_adversative(&self) -> bool {
        self.connective == Connective::But
    }

    /// Returns `true` if this is conjunction (`::`).
    pub fn is_conjunction(&self) -> bool {
        self.connective == Connective::And
    }

    /// Returns `true` if this is disjunction (`|`).
    pub fn is_disjunction(&self) -> bool {
        self.connective == Connective::Or
    }

    /// Returns `true` if this is unary inversion (`--`).
    pub fn is_inversion(&self) -> bool {
        self.connective == Connective::Not
    }

    /// Returns `true` if this is role typing / relation (`:`).
    pub fn is_role(&self) -> bool {
        self.connective == Connective::InRelationTo
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Invariant Guarantees & Trait Implementations )
// ==============================================================================
// Relational nodes satisfy Send, Sync, Clone, PartialEq across all targets.

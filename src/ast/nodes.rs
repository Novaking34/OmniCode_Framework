// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( AST Node Definitions, Boundary Typing & Ontological Hierarchy )
// ==============================================================================
//! OmniCode AST: Abstract Syntax Tree Nodes
//!
//! Preserves:
//!     1. First-Class Asides (Machine intent & AI reasoning traces)
//!     2. Boundary Typology (Denotation, Invariant, Connotation, TopDown, BottomUp)
//!     3. Directional Flow & Ontological Relations

use crate::core::boundaries::BoundaryType;
use crate::core::operators::{BeingRelation, Connective, DirectionalVector};
use crate::core::ternary::QuadState;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Scoped AST Node Structures: Aside, Boundary, Entity, Relations )
// ==============================================================================
#[derive(Debug, Clone, PartialEq)]
pub struct AsideNode {
    pub intent: String,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoundaryNode {
    pub boundary_type: BoundaryType,
    pub elements: Vec<ASTNode>,
    pub aside: Option<AsideNode>,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntityNode {
    pub name: String,
    pub definition: BoundaryNode,
    pub aside: Option<AsideNode>,
    pub context: Option<BoundaryNode>,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RelationNode {
    pub left: Box<ASTNode>,
    pub relation: BeingRelation,
    pub right: Box<ASTNode>,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectionalNode {
    pub source: Box<ASTNode>,
    pub vector: DirectionalVector,
    pub target: Box<ASTNode>,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConnectiveNode {
    pub left: Box<ASTNode>,
    pub connective: Connective,
    pub right: Box<ASTNode>,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryNotNode {
    pub operand: Box<ASTNode>,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArithmeticOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArithmeticNode {
    pub left: Box<ASTNode>,
    pub op: ArithmeticOp,
    pub right: Box<ASTNode>,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProgramNode {
    pub statements: Vec<ASTNode>,
    pub line: usize,
    pub col: usize,
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Unified ASTNode Enumeration: Complete Node Hierarchy )
// ==============================================================================
#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    Aside(AsideNode),
    Ternary(QuadState),
    Identifier(String),
    Number(f64),
    StringLit(String),
    Boundary(BoundaryNode),
    Entity(EntityNode),
    Relation(RelationNode),
    Directional(DirectionalNode),
    Connective(ConnectiveNode),
    Arithmetic(ArithmeticNode),
    UnaryNot(UnaryNotNode),
    Program(ProgramNode),
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( AST Node Invariant Verification & Trait Guarantees )
// ==============================================================================
// AST nodes satisfy Send, Sync, Clone, PartialEq invariants across all targets.

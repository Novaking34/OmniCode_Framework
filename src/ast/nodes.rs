//! OmniCode AST: Abstract Syntax Tree Nodes
//!
//! Preserves:
//!     1. First-Class Asides (Machine intent & AI reasoning traces)
//!     2. Boundary Typology (Denotation, Invariant, Connotation, TopDown, BottomUp)
//!     3. Directional Flow & Ontological Relations

use crate::core::boundaries::BoundaryType;
use crate::core::operators::{BeingRelation, Connective, DirectionalVector};
use crate::core::ternary::QuadState;

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

#[derive(Debug, Clone, PartialEq)]
pub struct ProgramNode {
    pub statements: Vec<ASTNode>,
    pub line: usize,
    pub col: usize,
}

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
    UnaryNot(UnaryNotNode),
    Program(ProgramNode),
}

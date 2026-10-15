// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( AST Primitive Nodes: Arithmetic, Polarity & Unary Inversion )
// ==============================================================================
// SCRIPTURE ANCHORS & EPISTEMIC WITNESSES:
//   - Exodus 31:18: "written with the finger of God" — Inscription of eternal truth.
//   - Proverbs 24:27: "Prepare thy work without, and make it fit for thyself in the field;
//                     and afterwards build thine house." — Systematic modularization.
//   - Proverbs 22:28: "Remove not the ancient landmark, which thy fathers have set."
//                     — Preservation of invariant boundaries and balanced truth.
// ==============================================================================

use crate::ast::nodes::ASTNode;
use crate::core::operators::{AffixPosition, Polarity};

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Binary Arithmetic Operators, Arithmetic Nodes & Polarity Modifiers )
// ==============================================================================

/// Discrete arithmetic operations supported on scalar values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArithmeticOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl ArithmeticOp {
    /// Returns the canonical mathematical operator symbol.
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
        }
    }
}

/// Binary arithmetic evaluation node: `left OP right`.
#[derive(Debug, Clone, PartialEq)]
pub struct ArithmeticNode {
    pub left: Box<ASTNode>,
    pub op: ArithmeticOp,
    pub right: Box<ASTNode>,
    pub line: usize,
    pub col: usize,
}

/// Unary logical/ternary inversion node: `! operand`.
#[derive(Debug, Clone, PartialEq)]
pub struct UnaryNotNode {
    pub operand: Box<ASTNode>,
    pub line: usize,
    pub col: usize,
}

/// Affixed polarity modifier indicating abstract or concrete bias: `+name` or `-name`.
#[derive(Debug, Clone, PartialEq)]
pub struct PolarityNode {
    pub name: String,
    pub polarity: Polarity,
    pub position: AffixPosition,
    pub line: usize,
    pub col: usize,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Primitive Node Constructors & Operational Helpers )
// ==============================================================================

impl ArithmeticNode {
    /// Constructs a new binary arithmetic node.
    pub fn new(left: ASTNode, op: ArithmeticOp, right: ASTNode, line: usize, col: usize) -> Self {
        Self {
            left: Box::new(left),
            op,
            right: Box::new(right),
            line,
            col,
        }
    }
}

impl UnaryNotNode {
    /// Constructs a new unary inversion node.
    pub fn new(operand: ASTNode, line: usize, col: usize) -> Self {
        Self {
            operand: Box::new(operand),
            line,
            col,
        }
    }
}

impl PolarityNode {
    /// Constructs a new polarity-modified entity node.
    pub fn new(name: impl Into<String>, polarity: Polarity, position: AffixPosition, line: usize, col: usize) -> Self {
        Self {
            name: name.into(),
            polarity,
            position,
            line,
            col,
        }
    }

    /// Returns the formatted symbol representation with its affix.
    pub fn formatted(&self) -> String {
        match self.position {
            AffixPosition::Prefix => format!("{}{}", self.polarity.symbol(), self.name),
            AffixPosition::Postfix => format!("{}{}", self.name, self.polarity.symbol()),
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Invariant Guarantees & Trait Implementations )
// ==============================================================================
// Primitive nodes satisfy Send, Sync, Clone, PartialEq across all targets.

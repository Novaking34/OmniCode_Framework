// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( AST Structural Nodes: Programs, Modules, Tapes, Sections, Rolls & Commit Gates )
// ==============================================================================
// SCRIPTURE ANCHORS & EPISTEMIC WITNESSES:
//   - Exodus 31:18: "written with the finger of God" — Inscription of eternal truth.
//   - Proverbs 24:27: "Prepare thy work without, and make it fit for thyself in the field;
//                     and afterwards build thine house." — Systematic modularization.
//   - Proverbs 22:28: "Remove not the ancient landmark, which thy fathers have set."
//                     — Preservation of invariant boundaries and balanced truth.
// ==============================================================================

use crate::ast::nodes::boundaries::AsideNode;
use crate::ast::nodes::ASTNode;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Structural Node Definitions & 4-Block Section Kinds )
// ==============================================================================

/// Complete source program reel or process tape containing sequential statements.
#[derive(Debug, Clone, PartialEq)]
pub struct ProgramNode {
    pub statements: Vec<ASTNode>,
    pub line: usize,
    pub col: usize,
}

/// Root Module declaration: `= [Module]: < Category > & [ Noun ] ( Aside ) { Fields }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleNode {
    pub name: String,
    pub category: Option<Box<ASTNode>>,
    pub aside: Option<AsideNode>,
    pub fields: Vec<ASTNode>,
    pub line: usize,
    pub col: usize,
}

/// Root Process Tape declaration: `= [Tape]: < Category > & [ Process ] ( Aside ) { Clauses }`.
#[derive(Debug, Clone, PartialEq)]
pub struct TapeNode {
    pub name: String,
    pub category: Option<Box<ASTNode>>,
    pub aside: Option<AsideNode>,
    pub clauses: Vec<ASTNode>,
    pub line: usize,
    pub col: usize,
}

/// The four canonical blocks governing OmniCode file architecture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SectionKind {
    /// Block 1: Metadata / Invariant Declarations
    Metadata,
    /// Block 2: Opening Block — State mounting, allocations, hardware bridges
    Opening,
    /// Block 3: Body Block — Sequential operational rolls & causal state progression
    Body,
    /// Block 4: Closing Block — Verification, sealing, tripwiring, diagnostic log & commit gates
    Closing,
}

impl SectionKind {
    /// Returns the canonical section block name.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Metadata => "Metadata",
            Self::Opening => "Opening",
            Self::Body => "Body",
            Self::Closing => "Closing",
        }
    }
}

/// A 4-Block section container holding child statements or operational rolls:
/// `Opening { ... }`, `Body [ ... ]`, `Closing ( Aside ) { ... }`.
#[derive(Debug, Clone, PartialEq)]
pub struct SectionNode {
    pub kind: SectionKind,
    pub aside: Option<AsideNode>,
    pub elements: Vec<ASTNode>,
    pub line: usize,
    pub col: usize,
}

/// An atomic operational roll within the Body block:
/// `[ ROLL N ] : [ Intent ] : Action ( Aside )`.
#[derive(Debug, Clone, PartialEq)]
pub struct RollNode {
    pub id: usize,
    pub label: String,
    pub action: Box<ASTNode>,
    pub aside: Option<AsideNode>,
    pub line: usize,
    pub col: usize,
}

/// Closing Block Fourfold Commit Status Gate:
/// `[ Commit Status ] : [ +1:PROCEED | 0:AWAIT | HALT:NULL | -1:ROLLBACK ]`.
#[derive(Debug, Clone, PartialEq)]
pub struct CommitStatusNode {
    pub cases: Vec<ASTNode>,
    pub aside: Option<AsideNode>,
    pub line: usize,
    pub col: usize,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Structural Constructors & Query Implementations )
// ==============================================================================

impl ProgramNode {
    /// Constructs a new [`ProgramNode`].
    pub fn new(statements: Vec<ASTNode>, line: usize, col: usize) -> Self {
        Self { statements, line, col }
    }
}

impl ModuleNode {
    /// Constructs a new [`ModuleNode`].
    pub fn new(name: impl Into<String>, line: usize, col: usize) -> Self {
        Self {
            name: name.into(),
            category: None,
            aside: None,
            fields: Vec::new(),
            line,
            col,
        }
    }
}

impl TapeNode {
    /// Constructs a new [`TapeNode`].
    pub fn new(name: impl Into<String>, line: usize, col: usize) -> Self {
        Self {
            name: name.into(),
            category: None,
            aside: None,
            clauses: Vec::new(),
            line,
            col,
        }
    }
}

impl SectionNode {
    /// Constructs a new [`SectionNode`].
    pub fn new(kind: SectionKind, elements: Vec<ASTNode>, line: usize, col: usize) -> Self {
        Self {
            kind,
            aside: None,
            elements,
            line,
            col,
        }
    }

    /// Attaches an intent aside to this section block.
    pub fn with_aside(mut self, aside: AsideNode) -> Self {
        self.aside = Some(aside);
        self
    }
}

impl RollNode {
    /// Constructs a new operational [`RollNode`].
    pub fn new(id: usize, label: impl Into<String>, action: ASTNode, line: usize, col: usize) -> Self {
        Self {
            id,
            label: label.into(),
            action: Box::new(action),
            aside: None,
            line,
            col,
        }
    }

    /// Attaches an intent aside to this roll.
    pub fn with_aside(mut self, aside: AsideNode) -> Self {
        self.aside = Some(aside);
        self
    }
}

impl CommitStatusNode {
    /// Constructs a new [`CommitStatusNode`].
    pub fn new(cases: Vec<ASTNode>, line: usize, col: usize) -> Self {
        Self {
            cases,
            aside: None,
            line,
            col,
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Invariant Guarantees & Trait Implementations )
// ==============================================================================
// Structural nodes satisfy Send, Sync, Clone, PartialEq across all targets.

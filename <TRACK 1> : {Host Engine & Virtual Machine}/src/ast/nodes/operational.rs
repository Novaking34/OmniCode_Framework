// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( AST Operational Nodes: SVO Verbs, Adversative Guards & Hardware Locators )
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
// ( Operational Action Node Definitions: SVO Verbs, Guards & Hardware Locators )
// ==============================================================================

/// Canonical SVO Action Verbs in the OmniCode runtime grammar.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VerbKind {
    /// INSCRIBE: Imposes an external or source value into a target cell.
    Inscribe,
    /// TRANSDUCE: Maps values bijectively across dual substrates (bits <=> trits).
    Transduce,
    /// GUARD: Asserts invariant equilibrium under adversative caveats.
    Guard,
    /// PROJECT: Projects internal state into an external target buffer or screen.
    Project,
    /// SEQUENCE: Chains discrete cells outward into a continuous text ribbon.
    Sequence,
    /// User-defined or extensible domain action verb.
    Custom(String),
}

impl VerbKind {
    /// Parses an uppercase identifier into an action verb kind.
    pub fn from_name(name: &str) -> Self {
        match name.to_ascii_uppercase().as_str() {
            "INSCRIBE" => Self::Inscribe,
            "TRANSDUCE" => Self::Transduce,
            "GUARD" => Self::Guard,
            "PROJECT" => Self::Project,
            "SEQUENCE" => Self::Sequence,
            other => Self::Custom(other.to_string()),
        }
    }

    /// Returns the uppercase verb name string.
    pub fn name(&self) -> &str {
        match self {
            Self::Inscribe => "INSCRIBE",
            Self::Transduce => "TRANSDUCE",
            Self::Guard => "GUARD",
            Self::Project => "PROJECT",
            Self::Sequence => "SEQUENCE",
            Self::Custom(s) => s.as_str(),
        }
    }
}

/// SVO Action Verb execution node:
/// `Subject --> VERB Object = Target` (e.g. `{ Glyph ] --> INSCRIBE [ Cell ] = [ Character` ]`).
#[derive(Debug, Clone, PartialEq)]
pub struct VerbNode {
    pub verb: VerbKind,
    pub subject: Option<Box<ASTNode>>,
    pub object: Box<ASTNode>,
    pub target: Option<Box<ASTNode>>,
    pub line: usize,
    pub col: usize,
}

/// Adversative Guard assertion node:
/// `Condition ! GUARD --> ProceedBranch | FallbackBranch`.
#[derive(Debug, Clone, PartialEq)]
pub struct GuardNode {
    pub condition: Box<ASTNode>,
    pub on_proceed: Box<ASTNode>,
    pub on_fallback: Option<Box<ASTNode>>,
    pub line: usize,
    pub col: usize,
}

/// MMIO Hardware Bus locator node:
/// `Target ::@ Address` (e.g. `[ Terminal Stdout ] ::@ 0x01`).
#[derive(Debug, Clone, PartialEq)]
pub struct HardwareLocatorNode {
    pub target: Box<ASTNode>,
    pub address: u64,
    pub line: usize,
    pub col: usize,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Operational Constructors & Methods )
// ==============================================================================

impl VerbNode {
    /// Constructs a new [`VerbNode`].
    pub fn new(
        verb: VerbKind,
        subject: Option<ASTNode>,
        object: ASTNode,
        target: Option<ASTNode>,
        line: usize,
        col: usize,
    ) -> Self {
        Self {
            verb,
            subject: subject.map(Box::new),
            object: Box::new(object),
            target: target.map(Box::new),
            line,
            col,
        }
    }
}

impl GuardNode {
    /// Constructs a new [`GuardNode`].
    pub fn new(
        condition: ASTNode,
        on_proceed: ASTNode,
        on_fallback: Option<ASTNode>,
        line: usize,
        col: usize,
    ) -> Self {
        Self {
            condition: Box::new(condition),
            on_proceed: Box::new(on_proceed),
            on_fallback: on_fallback.map(Box::new),
            line,
            col,
        }
    }
}

impl HardwareLocatorNode {
    /// Constructs a new [`HardwareLocatorNode`].
    pub fn new(target: ASTNode, address: u64, line: usize, col: usize) -> Self {
        Self {
            target: Box::new(target),
            address,
            line,
            col,
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Invariant Guarantees & Trait Implementations )
// ==============================================================================
// Operational nodes satisfy Send, Sync, Clone, PartialEq across all targets.

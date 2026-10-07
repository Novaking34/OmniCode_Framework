//! OmniCode Core: Relational, Directional, and Connective Operators
//!
//! Governs the 3-Dimensional Coordinate System:
//!     - Y-Axis (Being/State)      : = (IS), != (IS NOT), ~= (IS LIKE)
//!     - X-Axis (Causality/Action) : --> (Act Upon), <-- (Act Again)
//!     - Z-Axis (Scale/Layering)   : ^-- (Builds To), v-- (Breaks Down From)
//!     - Connectives               : ! (But), :: (And), : (In Relation To), | (Or), -- (Not)

use std::fmt;

/// Relations of Being along the Ontological Y-Axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BeingRelation {
    /// Exact identity / equivalence
    Is,
    /// Distinction / exclusion
    IsNot,
    /// Homomorphism / structural analogy / duck typing
    ToBeLike,
}

impl BeingRelation {
    pub fn from_symbol(sym: &str) -> Result<Self, String> {
        match sym.trim() {
            "=" => Ok(BeingRelation::Is),
            "!=" | "<>" => Ok(BeingRelation::IsNot),
            "~=" | "≈" => Ok(BeingRelation::ToBeLike),
            other => Err(format!("FAILPOINT [InvalidBeingRelation]: Unknown symbol '{other}'")),
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            BeingRelation::Is => "=",
            BeingRelation::IsNot => "!=",
            BeingRelation::ToBeLike => "~=",
        }
    }
}

/// Vectors of Action and Scale across X and Z axes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DirectionalVector {
    // X-Axis (Temporal / Dynamics)
    /// Forward causation, execution, emission
    ActUpon,
    /// Retroactive feedback, retry, rollback
    ActAgain,

    // Z-Axis (Scale / Hierarchy)
    /// Layering Up: synthesis, abstraction, emergence
    BuildsTo,
    /// Layering Down: decomposition, destructuring, compilation to IR
    BreaksDownFrom,
}

impl DirectionalVector {
    pub fn from_symbol(sym: &str) -> Result<Self, String> {
        match sym.trim() {
            "-->" | "->" => Ok(DirectionalVector::ActUpon),
            "<--" | "<-" => Ok(DirectionalVector::ActAgain),
            "^--" | "^-" | "↑" => Ok(DirectionalVector::BuildsTo),
            "v--" | "v-" | "↓" => Ok(DirectionalVector::BreaksDownFrom),
            other => Err(format!("FAILPOINT [InvalidDirectionalVector]: Unknown vector symbol '{other}'")),
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            DirectionalVector::ActUpon => "-->",
            DirectionalVector::ActAgain => "<--",
            DirectionalVector::BuildsTo => "^--",
            DirectionalVector::BreaksDownFrom => "v--",
        }
    }
}

/// Logical and Structural Connectives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Connective {
    /// Adversative caveat: A ! B (A executes, BUT must satisfy caveat B)
    But,
    /// Cumulative conjunction: A :: B (A AND B simultaneous)
    And,
    /// Relational role / typing: A : B (A in relation to B)
    InRelationTo,
    /// Disjunction / alternative: A | B (A OR B)
    Or,
    /// Ontological inversion: -- A (NOT A)
    Not,
}

impl Connective {
    pub fn from_symbol(sym: &str) -> Result<Self, String> {
        match sym.trim() {
            "!" => Ok(Connective::But),
            "::" | "&&" => Ok(Connective::And),
            ":" => Ok(Connective::InRelationTo),
            "|" | "||" => Ok(Connective::Or),
            "--" | "not" | "~" => Ok(Connective::Not),
            other => Err(format!("FAILPOINT [InvalidConnective]: Unknown connective symbol '{other}'")),
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Connective::But => "!",
            Connective::And => "::",
            Connective::InRelationTo => ":",
            Connective::Or => "|",
            Connective::Not => "--",
        }
    }
}

impl fmt::Display for BeingRelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl fmt::Display for DirectionalVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl fmt::Display for Connective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

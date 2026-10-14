// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Module Identity, 3D Coordinate Operators, Connectives & Vector Maps )
// ==============================================================================
//! OmniCode Core: Relational, Directional, and Connective Operators
//!
//! Governs the 3-Dimensional Coordinate System:
//!     - Y-Axis (Being/State)      : = (IS), != (IS NOT), ~= (IS LIKE)
//!     - X-Axis (Causality/Action) : --> (Act Upon), <-- (Act Again)
//!     - Z-Axis (Scale/Layering)   : ^-- (Builds To), v-- (Breaks Down From)
//!     - Connectives               : ! (But), :: (And), : (In Relation To), | (Or), -- (Not)

use std::fmt;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Type Declarations: BeingRelation, DirectionalVector, and Connective Enums )
// ==============================================================================
/// Relations of Being along the Ontological Y-Axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BeingRelation {
    /// Exact identity / equivalence (=)
    Is,
    /// Distinction / exclusion (!=)
    IsNot,
    /// Homomorphism / structural analogy / duck typing (~=)
    ToBeLike,
    /// Invariant Balance / Equivalence (<=>)
    Equivalence,
    /// Relational strictly less than (<)
    LessThan,
    /// Relational strictly greater than (>)
    GreaterThan,
    /// Relational less than or equal (<=)
    LessEqual,
    /// Relational greater than or equal (>=)
    GreaterEqual,
}

/// Vectors of Action, Scale, and Spatial Quadrants across X, Y, and Z axes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DirectionalVector {
    // X-Axis (Temporal / Dynamics)
    /// Forward causation, execution, emission (-->)
    ActUpon,
    /// Retroactive feedback, retry, rollback (<--)
    ActAgain,

    // Z-Axis (Scale / Hierarchy)
    /// Layering Up: synthesis, abstraction, emergence (^-- / --^)
    BuildsTo,
    /// Layering Down: decomposition, destructuring, compilation to IR (v-- / --v)
    BreaksDownFrom,

    // Diagonal Modulated Scales
    /// Major ascending forward diagonal (/-^)
    Major,
    /// Minor descending forward diagonal (\-v)
    Minor,
    /// Great ascending backward diagonal (\-^)
    Great,
    /// Small descending backward diagonal (/-v)
    Small,

    // 2D Quadrant / Compass Vectors
    /// Upper Right (Quadrant I: +X, +Y) (/^)
    UpperRight,
    /// Lower Right (Quadrant IV: +X, -Y) (\v)
    LowerRight,
    /// Upper Left (Quadrant II: -X, +Y) (^\)
    UpperLeft,
    /// Lower Left (Quadrant III: -X, -Y) (v/)
    LowerLeft,
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

/// Polarity of an entity, keyword, or value.
/// '+' represents Abstract (archetype, intension, zero-cost blueprint).
/// '-' represents Concrete (extension, physical instance, allocated substance).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Polarity {
    Abstract,
    Concrete,
}

/// Affix position of a polarity operator attached to an identifier or keyword.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AffixPosition {
    Prefix,  // +Dog, -Dog, +Cell
    Postfix, // Dog+, Dog-, PROCEED-
}

impl Polarity {
    pub fn from_symbol(sym: &str) -> Result<Self, String> {
        match sym.trim() {
            "+" => Ok(Polarity::Abstract),
            "-" => Ok(Polarity::Concrete),
            other => Err(format!("FAILPOINT [InvalidPolarity]: Unknown polarity symbol '{other}'")),
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Polarity::Abstract => "+",
            Polarity::Concrete => "-",
        }
    }
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Core Symbol Parsing & Semantic Transformations )
// ==============================================================================
impl BeingRelation {
    pub fn from_symbol(sym: &str) -> Result<Self, String> {
        match sym.trim() {
            "=" => Ok(BeingRelation::Is),
            "!=" | "<>" => Ok(BeingRelation::IsNot),
            "~=" | "≈" => Ok(BeingRelation::ToBeLike),
            "<=>" => Ok(BeingRelation::Equivalence),
            "<" => Ok(BeingRelation::LessThan),
            ">" => Ok(BeingRelation::GreaterThan),
            "<=" => Ok(BeingRelation::LessEqual),
            ">=" => Ok(BeingRelation::GreaterEqual),
            other => Err(format!("FAILPOINT [InvalidBeingRelation]: Unknown symbol '{other}'")),
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            BeingRelation::Is => "=",
            BeingRelation::IsNot => "!=",
            BeingRelation::ToBeLike => "~=",
            BeingRelation::Equivalence => "<=>",
            BeingRelation::LessThan => "<",
            BeingRelation::GreaterThan => ">",
            BeingRelation::LessEqual => "<=",
            BeingRelation::GreaterEqual => ">=",
        }
    }
}

impl DirectionalVector {
    pub fn from_symbol(sym: &str) -> Result<Self, String> {
        match sym.trim() {
            "-->" | "->" => Ok(DirectionalVector::ActUpon),
            "<--" | "<-" => Ok(DirectionalVector::ActAgain),
            "--^" | "-^" | "^--" | "^-" | "↑" => Ok(DirectionalVector::BuildsTo),
            "--v" | "-v" | "v--" | "v-" | "↓" => Ok(DirectionalVector::BreaksDownFrom),
            "/-^" => Ok(DirectionalVector::Major),
            "\\-v" => Ok(DirectionalVector::Minor),
            "\\-^" => Ok(DirectionalVector::Great),
            "/-v" => Ok(DirectionalVector::Small),
            "/^" => Ok(DirectionalVector::UpperRight),
            "\\v" => Ok(DirectionalVector::LowerRight),
            "^\\" => Ok(DirectionalVector::UpperLeft),
            "v/" => Ok(DirectionalVector::LowerLeft),
            other => Err(format!("FAILPOINT [InvalidDirectionalVector]: Unknown vector symbol '{other}'")),
        }
    }

    /// Canonical ASCII symbol for the vector.
    pub fn symbol(&self) -> &'static str {
        match self {
            DirectionalVector::ActUpon => "-->",
            DirectionalVector::ActAgain => "<--",
            DirectionalVector::BuildsTo => "--^",
            DirectionalVector::BreaksDownFrom => "--v",
            DirectionalVector::Major => "/-^",
            DirectionalVector::Minor => "\\-v",
            DirectionalVector::Great => "\\-^",
            DirectionalVector::Small => "/-v",
            DirectionalVector::UpperRight => "/^",
            DirectionalVector::LowerRight => "\\v",
            DirectionalVector::UpperLeft => "^\\",
            DirectionalVector::LowerLeft => "v/",
        }
    }

    /// Fast 2-character ASCII shorthand symbol for the vector.
    pub fn shorthand(&self) -> &'static str {
        match self {
            DirectionalVector::ActUpon => "->",
            DirectionalVector::ActAgain => "<-",
            DirectionalVector::BuildsTo => "-^",
            DirectionalVector::BreaksDownFrom => "-v",
            DirectionalVector::Major => "/-^",
            DirectionalVector::Minor => "\\-v",
            DirectionalVector::Great => "\\-^",
            DirectionalVector::Small => "/-v",
            DirectionalVector::UpperRight => "/^",
            DirectionalVector::LowerRight => "\\v",
            DirectionalVector::UpperLeft => "^\\",
            DirectionalVector::LowerLeft => "v/",
        }
    }

    /// Cardinal direction on the computational coordinate plane.
    pub fn cardinal_direction(&self) -> &'static str {
        match self {
            DirectionalVector::ActUpon => "Right (Forward Causation / Advance)",
            DirectionalVector::ActAgain => "Left (Retroaction / Feedback / Rollback)",
            DirectionalVector::BuildsTo => "Up (Synthesis / Abstraction / Ascend)",
            DirectionalVector::BreaksDownFrom => "Down (Decomposition / Analysis / Descend)",
            DirectionalVector::Major => "Major (Ascending Forward Diagonal / Major Scale)",
            DirectionalVector::Minor => "Minor (Descending Forward Diagonal / Minor Scale)",
            DirectionalVector::Great => "Great (Ascending Backward Diagonal / Great Scale)",
            DirectionalVector::Small => "Small (Descending Backward Diagonal / Small Scale)",
            DirectionalVector::UpperRight => "Upper Right (Quadrant I: +X, +Y / Northeast)",
            DirectionalVector::LowerRight => "Lower Right (Quadrant IV: +X, -Y / Southeast)",
            DirectionalVector::UpperLeft => "Upper Left (Quadrant II: -X, +Y / Northwest)",
            DirectionalVector::LowerLeft => "Lower Left (Quadrant III: -X, -Y / Southwest)",
        }
    }
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


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Display Trait Implementations and String Formatting )
// ==============================================================================
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

impl fmt::Display for Polarity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl fmt::Display for AffixPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AffixPosition::Prefix => write!(f, "Prefix"),
            AffixPosition::Postfix => write!(f, "Postfix"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polarity_and_affix_operators() {
        assert_eq!(Polarity::from_symbol("+").unwrap(), Polarity::Abstract);
        assert_eq!(Polarity::from_symbol("-").unwrap(), Polarity::Concrete);
        assert_eq!(Polarity::Abstract.symbol(), "+");
        assert_eq!(Polarity::Concrete.symbol(), "-");
        assert_eq!(format!("{}", Polarity::Abstract), "+");
        assert_eq!(format!("{}", Polarity::Concrete), "-");
    }

    #[test]
    fn test_directional_vectors_canonical_and_shorthand() {
        // Canonical 3-character symbols
        assert_eq!(DirectionalVector::from_symbol("-->").unwrap(), DirectionalVector::ActUpon);
        assert_eq!(DirectionalVector::from_symbol("<--").unwrap(), DirectionalVector::ActAgain);
        assert_eq!(DirectionalVector::from_symbol("--^").unwrap(), DirectionalVector::BuildsTo);
        assert_eq!(DirectionalVector::from_symbol("--v").unwrap(), DirectionalVector::BreaksDownFrom);

        // Fast 2-character shorthands
        assert_eq!(DirectionalVector::from_symbol("->").unwrap(), DirectionalVector::ActUpon);
        assert_eq!(DirectionalVector::from_symbol("<-").unwrap(), DirectionalVector::ActAgain);
        assert_eq!(DirectionalVector::from_symbol("-^").unwrap(), DirectionalVector::BuildsTo);
        assert_eq!(DirectionalVector::from_symbol("-v").unwrap(), DirectionalVector::BreaksDownFrom);

        // Legacy symbols
        assert_eq!(DirectionalVector::from_symbol("^--").unwrap(), DirectionalVector::BuildsTo);
        assert_eq!(DirectionalVector::from_symbol("v--").unwrap(), DirectionalVector::BreaksDownFrom);

        // Symbols emit canonical forms
        assert_eq!(DirectionalVector::BuildsTo.symbol(), "--^");
        assert_eq!(DirectionalVector::BreaksDownFrom.symbol(), "--v");

        // Scale Modulations
        assert_eq!(DirectionalVector::from_symbol("/-^").unwrap(), DirectionalVector::Major);
        assert_eq!(DirectionalVector::from_symbol("\\-v").unwrap(), DirectionalVector::Minor);
        assert_eq!(DirectionalVector::from_symbol("\\-^").unwrap(), DirectionalVector::Great);
        assert_eq!(DirectionalVector::from_symbol("/-v").unwrap(), DirectionalVector::Small);

        // 2D Quadrants
        assert_eq!(DirectionalVector::from_symbol("/^").unwrap(), DirectionalVector::UpperRight);
        assert_eq!(DirectionalVector::from_symbol("\\v").unwrap(), DirectionalVector::LowerRight);
        assert_eq!(DirectionalVector::from_symbol("^\\").unwrap(), DirectionalVector::UpperLeft);
        assert_eq!(DirectionalVector::from_symbol("v/").unwrap(), DirectionalVector::LowerLeft);
    }
}


// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Universal Runtime Value Representation, Unwrapping & Metric Invariants )
// ==============================================================================
//! OmniCode Runtime: Universal Runtime Value Substrate
//!
//! # Architectural Scope & Purpose:
//! Defines the universal `RuntimeValue` enum representing every discrete unit of
//! data flowing through the OmniCode Virtual Machine.
//!
//! Provides zero-cost scalar unwrappers, bijective character-cell transducers,
//! balanced ternary quad-state truth projections, cross-substrate relational
//! equality guards, and physical cell capacity metrics.
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**:
//!   "And he gave unto Moses, when he had made an end of communing with him upon mount Sinai,
//!    two tables of testimony, tables of stone, written with the finger of God."
//!   (The immutable inscription of atomic runes onto bounded physical tablets).
//! - **Proverbs 24:27 (KJV)**:
//!   "Prepare thy work without, and make it fit for thyself in the field;
//!    and afterwards build thine house."
//!   (The discipline of proving atomic types and substrates before constructing systems).
//! - **Proverbs 22:28 (KJV)**:
//!   "Remove not the ancient landmark, which thy fathers have set."
//!   (The preservation of historical memory boundaries, silicon addresses, and constants).
//!
//! # Why Extensive Commenting Matters for AI Agents & Developers:
//! 1. **Cross-Substrate Soundness**: `Character` unifies an 8-bit glyph byte with
//!    a 5-trit balanced ternary `Cell`. Unification methods like `is_glyph_equal`
//!    allow relational algebra to operate seamlessly across text, cells, and characters.
//! 2. **Universal Introspection**: Evaluator sub-modules and execution pipelines
//!    require standardized unwrapping (`as_state`, `as_number`, `as_text`) without
//!    resorting to repeated error-prone match blocks.
//! 3. **Bounded Capacity Invariants**: Relational constraints like `{ Character' <= [ 1 Cell ] }`
//!    require an objective metric `capacity_in_cells()` to verify memory bounds.
// ==============================================================================

use std::collections::HashMap;
use std::fmt;
use crate::core::ternary::QuadState;
use crate::core::cell::Cell;
use crate::core::vessel::Vessel;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( RuntimeValue Enum Definition & Core Constructors )
// ==============================================================================

/// Represents any discrete value flowing through the OmniCode runtime.
///
/// Implements full dual-substrate parity across balanced ternary truth states,
/// continuous text strings, dense 5-trit cells, and the atomic Character rune.
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    /// Quad-State truth value (+1: Proceed, 0: Await, -1: Rollback, NULL: Halt).
    State(QuadState),
    /// 64-bit floating-point numeric scalar or integer codepoint.
    Number(f64),
    /// Continuous text string ribbon composed of sequential characters.
    Text(String),
    /// Atomic 5-trit balanced ternary memory cell (spanning 243 states).
    Cell(Cell),
    /// Atomic Character rune enclosing both 8-bit ASCII glyph and 5-trit Cell.
    /// Fulfills invariant: `{ Character } = [ Enclosed ]: TRUE`.
    Character {
        glyph: u8,
        cell: Cell,
    },
    /// Bounded multi-cell memory vessel or storehouse partition.
    Vessel(Vessel),
    /// Structured entity record containing named fields and mutability contract.
    EntityRecord {
        name: String,
        is_immutable: bool,
        values: HashMap<String, Box<RuntimeValue>>,
    },
    /// Ordered vector of runtime values.
    Vector(Vec<RuntimeValue>),
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Constructors, Universal Unwrapping, Relational Helpers & Capacity Metrics )
// ==============================================================================

impl RuntimeValue {
    // --------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------

    /// Constructs a `RuntimeValue::State` from a balanced ternary `QuadState`.
    pub fn from_state(state: QuadState) -> Self {
        RuntimeValue::State(state)
    }

    /// Constructs a `RuntimeValue::Number` from an `f64`.
    pub fn from_number(n: f64) -> Self {
        RuntimeValue::Number(n)
    }

    /// Constructs a `RuntimeValue::Text` from any string-like type.
    pub fn from_text(t: impl Into<String>) -> Self {
        RuntimeValue::Text(t.into())
    }

    /// Constructs a `RuntimeValue::Cell` from a 5-trit `Cell`.
    pub fn from_cell(cell: Cell) -> Self {
        RuntimeValue::Cell(cell)
    }

    /// Constructs a `RuntimeValue::Character` from an 8-bit ASCII byte/glyph,
    /// bijectively transducing the byte into a 5-trit balanced ternary `Cell`.
    pub fn from_glyph(glyph: u8) -> Self {
        let cell = Cell::from_natural(glyph).unwrap_or_else(|_| Cell::from_state(QuadState::Rollback));
        RuntimeValue::Character { glyph, cell }
    }

    /// Constructs a `RuntimeValue::Character` from a standard Rust character.
    pub fn from_char(c: char) -> Self {
        let glyph = (c as u32 & 0xFF) as u8;
        Self::from_glyph(glyph)
    }

    // --------------------------------------------------------------------------
    // Universal Scalar Extractors & Unwrappers
    // --------------------------------------------------------------------------

    /// Returns the wrapped `QuadState` if this value is a `State`.
    pub fn as_state(&self) -> Option<QuadState> {
        match self {
            RuntimeValue::State(s) => Some(*s),
            _ => None,
        }
    }

    /// Returns the wrapped `f64` if this value is a `Number`.
    pub fn as_number(&self) -> Option<f64> {
        match self {
            RuntimeValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Returns a string slice if this value is `Text`.
    pub fn as_text(&self) -> Option<&str> {
        match self {
            RuntimeValue::Text(t) => Some(t.as_str()),
            _ => None,
        }
    }

    /// Returns the wrapped `Cell` if this value is a `Cell` or extracts the cell substrate of a `Character`.
    pub fn as_cell(&self) -> Option<Cell> {
        match self {
            RuntimeValue::Cell(c) => Some(*c),
            RuntimeValue::Character { cell, .. } => Some(*cell),
            _ => None,
        }
    }

    /// Returns the 8-bit glyph byte and 5-trit cell if this value is a `Character`.
    pub fn as_character(&self) -> Option<(u8, Cell)> {
        match self {
            RuntimeValue::Character { glyph, cell } => Some((*glyph, *cell)),
            _ => None,
        }
    }

    /// Returns a slice of elements if this value is a `Vector`.
    pub fn as_vector(&self) -> Option<&[RuntimeValue]> {
        match self {
            RuntimeValue::Vector(v) => Some(v.as_slice()),
            _ => None,
        }
    }

    /// Returns the 8-bit glyph byte if this value is a Character or convertible numeric/text value.
    pub fn character_glyph(&self) -> Option<u8> {
        match self {
            RuntimeValue::Character { glyph, .. } => Some(*glyph),
            RuntimeValue::Number(n) => {
                if *n >= 0.0 && *n <= 255.0 {
                    Some(*n as u8)
                } else {
                    None
                }
            }
            RuntimeValue::Text(t) => {
                let bytes = t.as_bytes();
                if bytes.len() == 1 {
                    Some(bytes[0])
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Returns the 5-trit `Cell` substrate if this value is a Character or Cell.
    pub fn character_cell(&self) -> Option<Cell> {
        match self {
            RuntimeValue::Character { cell, .. } => Some(*cell),
            RuntimeValue::Cell(c) => Some(*c),
            RuntimeValue::Number(n) => {
                if *n >= 0.0 && *n <= 242.0 {
                    Cell::from_natural(*n as u8).ok()
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    // --------------------------------------------------------------------------
    // Quad-State Truthiness Predicates
    // --------------------------------------------------------------------------

    /// Returns true if this value is strictly `+1:PROCEED`.
    pub fn is_proceed(&self) -> bool {
        matches!(self, RuntimeValue::State(QuadState::Proceed))
    }

    /// Returns true if this value is strictly `0:AWAIT`.
    pub fn is_await(&self) -> bool {
        matches!(self, RuntimeValue::State(QuadState::Await))
    }

    /// Returns true if this value is strictly `-1:ROLLBACK`.
    pub fn is_rollback(&self) -> bool {
        matches!(self, RuntimeValue::State(QuadState::Rollback))
    }

    /// Returns true if this value is strictly `NULL:HALT`.
    pub fn is_halt(&self) -> bool {
        matches!(self, RuntimeValue::State(QuadState::Halt))
    }

    /// Universally projects any runtime value to a balanced ternary `QuadState`.
    pub fn to_quad_state(&self) -> QuadState {
        match self {
            RuntimeValue::State(s) => *s,
            RuntimeValue::Number(n) => {
                if *n > 0.0 {
                    QuadState::Proceed
                } else if *n == 0.0 {
                    QuadState::Await
                } else {
                    QuadState::Rollback
                }
            }
            RuntimeValue::Text(t) => {
                if t.is_empty() {
                    QuadState::Await
                } else {
                    QuadState::Proceed
                }
            }
            RuntimeValue::Cell(c) => c.state,
            RuntimeValue::Character { cell, .. } => cell.state,
            RuntimeValue::Vessel(_) => QuadState::Proceed,
            RuntimeValue::EntityRecord { .. } => QuadState::Proceed,
            RuntimeValue::Vector(v) => {
                if v.is_empty() {
                    QuadState::Await
                } else {
                    QuadState::Proceed
                }
            }
        }
    }

    // --------------------------------------------------------------------------
    // Universal Dual-Substrate Relational Equality
    // --------------------------------------------------------------------------

    /// Compares two runtime values for semantic / glyph equivalence across representations.
    ///
    /// For example:
    /// - `Character('A')` equals `Character('A')`.
    /// - `Character('A')` equals `Text("A")`.
    /// - `Character('A')` equals `Number(65.0)`.
    /// - In general, identical types match by direct value equality.
    pub fn is_glyph_equal(&self, other: &Self) -> bool {
        if self == other {
            return true;
        }

        // Cross-compare glyph representations
        if let (Some(g1), Some(g2)) = (self.character_glyph(), other.character_glyph()) {
            if g1 == g2 {
                return true;
            }
        }

        // Cross-compare cell representations
        if let (Some(c1), Some(c2)) = (self.character_cell(), other.character_cell()) {
            if c1 == c2 {
                return true;
            }
        }

        // Numeric equivalence
        if let (Some(n1), Some(n2)) = (self.as_number(), other.as_number()) {
            return (n1 - n2).abs() < f64::EPSILON;
        }

        // String equivalence with boundary cast and quote stripping
        if let (Some(t1), Some(t2)) = (self.as_text(), other.as_text()) {
            if t1 == t2 {
                return true;
            }
            let c1 = t1.trim()
                .trim_matches(['\'', '"', '[', ']', '{', '}', '<', '>'])
                .trim();
            let c2 = t2.trim()
                .trim_matches(['\'', '"', '[', ']', '{', '}', '<', '>'])
                .trim();
            if c1 == c2 || c1.eq_ignore_ascii_case(c2) {
                return true;
            }
        }

        // Single-element vector unwrap for boundary-enclosed scalar comparisons
        if let RuntimeValue::Vector(v) = self {
            if v.len() == 1 {
                return v[0].is_glyph_equal(other);
            }
        }
        if let RuntimeValue::Vector(v) = other {
            if v.len() == 1 {
                return self.is_glyph_equal(&v[0]);
            }
        }

        // State vs Number / Text duality (Duality: words & values / states & numbers)
        // Freestanding language semantics:
        // - Literal operator "" ("0", "1", "-1") clarifies literal scalar value
        // - Boundary casts [ 0 ], { 0 }, < 0 > clarify boundary enclosure
        // - Keyword literal '' ('AWAIT', 'PROCEED') clarifies keyword as word
        match (self, other) {
            (RuntimeValue::State(s), RuntimeValue::Number(n))
            | (RuntimeValue::Number(n), RuntimeValue::State(s)) => {
                if let Some(i) = s.as_int() {
                    return (*n - i as f64).abs() < f64::EPSILON;
                }
            }
            (RuntimeValue::State(s), RuntimeValue::Text(t))
            | (RuntimeValue::Text(t), RuntimeValue::State(s)) => {
                let clean = t.trim()
                    .trim_matches(['\'', '"', '[', ']', '{', '}', '<', '>'])
                    .trim();
                if clean == "0" && *s == QuadState::Await {
                    return true;
                }
                if (clean == "1" || clean == "+1")
                    && *s == QuadState::Proceed
                {
                    return true;
                }
                if clean == "-1" && *s == QuadState::Rollback {
                    return true;
                }
                if clean.eq_ignore_ascii_case("proceed")
                    || clean.eq_ignore_ascii_case("yes")
                {
                    return *s == QuadState::Proceed;
                }
                if clean.eq_ignore_ascii_case("await")
                    || clean.eq_ignore_ascii_case("ok")
                    || clean.eq_ignore_ascii_case("pause")
                {
                    return *s == QuadState::Await;
                }
                if clean.eq_ignore_ascii_case("rollback")
                    || clean.eq_ignore_ascii_case("no")
                {
                    return *s == QuadState::Rollback;
                }
                if clean.eq_ignore_ascii_case("halt")
                    || clean.eq_ignore_ascii_case("null")
                    || clean.eq_ignore_ascii_case("void")
                {
                    return *s == QuadState::Halt;
                }
            }
            (RuntimeValue::Number(n), RuntimeValue::Text(t))
            | (RuntimeValue::Text(t), RuntimeValue::Number(n)) => {
                let clean = t.trim()
                    .trim_matches(['\'', '"', '[', ']', '{', '}', '<', '>'])
                    .trim();
                if let Ok(parsed) = clean.parse::<f64>() {
                    return (*n - parsed).abs() < f64::EPSILON;
                }
            }
            _ => {}
        }

        false
    }

    // --------------------------------------------------------------------------
    // Universal Capacity Metric
    // --------------------------------------------------------------------------

    /// Measures the physical memory capacity of a value in units of 5-trit cells.
    ///
    /// Fulfills capacity bound tests like: `{ Character' <= [ 1 Cell ] }`.
    pub fn capacity_in_cells(&self) -> usize {
        match self {
            RuntimeValue::Character { .. } => 1,
            RuntimeValue::Cell(_) => 1,
            RuntimeValue::State(_) => 1,
            RuntimeValue::Number(_) => 1,
            RuntimeValue::Text(t) => t.len(),
            RuntimeValue::Vessel(v) => v.capacity,
            RuntimeValue::EntityRecord { values, .. } => values.len(),
            RuntimeValue::Vector(v) => v.iter().map(|e| e.capacity_in_cells()).sum(),
        }
    }

    // --------------------------------------------------------------------------
    // Terminal ANSI Color Formatting
    // --------------------------------------------------------------------------

    /// Formats the runtime value with ANSI color styling for terminal output.
    pub fn format_colored(&self) -> String {
        match self {
            RuntimeValue::State(QuadState::Proceed) => "\x1b[1;32m+1:PROCEED\x1b[0m".to_string(),
            RuntimeValue::State(QuadState::Await) => "\x1b[1;33m0:AWAIT\x1b[0m".to_string(),
            RuntimeValue::State(QuadState::Rollback) => "\x1b[1;31m-1:ROLLBACK\x1b[0m".to_string(),
            RuntimeValue::State(QuadState::Halt) => "\x1b[1;35mNULL:HALT\x1b[0m".to_string(),
            RuntimeValue::Number(n) => format!("\x1b[1;36m{n}\x1b[0m"),
            RuntimeValue::Text(t) => format!("\x1b[0;32m\"{t}\"\x1b[0m"),
            RuntimeValue::Cell(c) => format!("\x1b[1;35m{c}\x1b[0m"),
            RuntimeValue::Character { glyph, cell } => {
                let ch = if glyph.is_ascii_graphic() || *glyph == b' ' {
                    (*glyph as char).to_string()
                } else {
                    format!("\\x{:02X}", glyph)
                };
                format!("\x1b[1;33m'{}' <{}>\x1b[0m", ch, cell)
            }
            RuntimeValue::Vessel(v) => format!("\x1b[1;34m{v}\x1b[0m"),
            RuntimeValue::EntityRecord { name, is_immutable, values } => {
                let kind = if *is_immutable { "Axiom" } else { "Entity" };
                format!("\x1b[1;34m{kind} < {name} > [ {} fields ]\x1b[0m", values.len())
            }
            RuntimeValue::Vector(elements) => {
                let formatted: Vec<String> = elements.iter().map(|e| e.format_colored()).collect();
                format!("[{}]", formatted.join(", "))
            }
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Display Trait Implementation & Unit Tests )
// ==============================================================================

impl fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeValue::State(s) => write!(f, "{s}"),
            RuntimeValue::Number(n) => write!(f, "{n}"),
            RuntimeValue::Text(t) => write!(f, "\"{t}\""),
            RuntimeValue::Cell(c) => write!(f, "{c}"),
            RuntimeValue::Character { glyph, .. } => {
                let ch = if glyph.is_ascii_graphic() || *glyph == b' ' {
                    (*glyph as char).to_string()
                } else {
                    format!("\\x{:02X}", glyph)
                };
                write!(f, "'{ch}'")
            }
            RuntimeValue::Vessel(v) => write!(f, "{v}"),
            RuntimeValue::EntityRecord { name, is_immutable, values } => {
                let kind = if *is_immutable { "Axiom" } else { "Entity" };
                write!(f, "{kind} < {name} > [ {} fields ]", values.len())
            }
            RuntimeValue::Vector(elements) => {
                let formatted: Vec<String> = elements.iter().map(|e| e.to_string()).collect();
                write!(f, "[{}]", formatted.join(", "))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universal_scalar_unwrappers() {
        let val_num = RuntimeValue::from_number(42.0);
        assert_eq!(val_num.as_number(), Some(42.0));
        assert_eq!(val_num.as_state(), None);

        let val_state = RuntimeValue::from_state(QuadState::Proceed);
        assert_eq!(val_state.as_state(), Some(QuadState::Proceed));
        assert!(val_state.is_proceed());
        assert!(!val_state.is_rollback());

        let val_text = RuntimeValue::from_text("Hello");
        assert_eq!(val_text.as_text(), Some("Hello"));
        assert_eq!(val_text.to_quad_state(), QuadState::Proceed);

        let empty_text = RuntimeValue::from_text("");
        assert_eq!(empty_text.to_quad_state(), QuadState::Await);
    }

    #[test]
    fn test_universal_glyph_and_cell_equality() {
        let ch = RuntimeValue::from_glyph(b'A');
        let text_a = RuntimeValue::from_text("A");
        let num_65 = RuntimeValue::from_number(65.0);

        assert!(ch.is_glyph_equal(&text_a));
        assert!(ch.is_glyph_equal(&num_65));
        assert!(text_a.is_glyph_equal(&num_65));

        let ch_b = RuntimeValue::from_glyph(b'B');
        assert!(!ch.is_glyph_equal(&ch_b));
    }

    #[test]
    fn test_universal_capacity_metric() {
        let ch = RuntimeValue::from_char('X');
        assert_eq!(ch.capacity_in_cells(), 1);

        let cell = RuntimeValue::from_cell(Cell::equilibrium());
        assert_eq!(cell.capacity_in_cells(), 1);

        let text = RuntimeValue::from_text("12345");
        assert_eq!(text.capacity_in_cells(), 5);
    }

    #[test]
    fn test_literal_operator_and_boundary_casting_duality() {
        let state_await = RuntimeValue::from_state(QuadState::Await);
        let num_zero = RuntimeValue::from_number(0.0);
        let lit_zero = RuntimeValue::from_text("\"0\"");
        let plain_zero = RuntimeValue::from_text("0");
        let cast_bracket = RuntimeValue::from_text("[ 0 ]");
        let cast_brace = RuntimeValue::from_text("{ 0 }");
        let cast_chevron = RuntimeValue::from_text("< 0 >");
        let word_await_single = RuntimeValue::from_text("'AWAIT'");
        let word_await_double = RuntimeValue::from_text("\"AWAIT\"");

        // 1. Literal operator "" clarifies zero as literal value
        assert!(num_zero.is_glyph_equal(&lit_zero));
        assert!(num_zero.is_glyph_equal(&plain_zero));
        assert!(lit_zero.is_glyph_equal(&plain_zero));

        // 2. Boundary casts [ 0 ], { 0 }, < 0 > match zero and Await state
        assert!(num_zero.is_glyph_equal(&cast_bracket));
        assert!(num_zero.is_glyph_equal(&cast_brace));
        assert!(num_zero.is_glyph_equal(&cast_chevron));
        assert!(state_await.is_glyph_equal(&cast_bracket));
        assert!(state_await.is_glyph_equal(&cast_brace));
        assert!(state_await.is_glyph_equal(&cast_chevron));

        // 3. 'AWAIT' keyword-as-word matches state Await and word "AWAIT"
        assert!(state_await.is_glyph_equal(&word_await_single));
        assert!(state_await.is_glyph_equal(&word_await_double));
        assert!(word_await_single.is_glyph_equal(&word_await_double));

        // 4. Positive and Negative Unities Duality (+1: Proceed, -1: Rollback)
        let state_proceed = RuntimeValue::from_state(QuadState::Proceed);
        let num_one = RuntimeValue::from_number(1.0);
        let cast_one = RuntimeValue::from_text("[ +1 ]");
        let word_proceed = RuntimeValue::from_text("'PROCEED'");
        assert!(num_one.is_glyph_equal(&cast_one));
        assert!(state_proceed.is_glyph_equal(&cast_one));
        assert!(state_proceed.is_glyph_equal(&word_proceed));

        let state_rollback = RuntimeValue::from_state(QuadState::Rollback);
        let num_neg_one = RuntimeValue::from_number(-1.0);
        let cast_neg_one = RuntimeValue::from_text("[ -1 ]");
        let word_rollback = RuntimeValue::from_text("'ROLLBACK'");
        assert!(num_neg_one.is_glyph_equal(&cast_neg_one));
        assert!(state_rollback.is_glyph_equal(&cast_neg_one));
        assert!(state_rollback.is_glyph_equal(&word_rollback));

        // 5. NULL / Void Duality
        let state_halt = RuntimeValue::from_state(QuadState::Halt);
        let cast_null = RuntimeValue::from_text("[ NULL ]");
        let word_null = RuntimeValue::from_text("'NULL'");
        assert!(state_halt.is_glyph_equal(&cast_null));
        assert!(state_halt.is_glyph_equal(&word_null));
    }
}

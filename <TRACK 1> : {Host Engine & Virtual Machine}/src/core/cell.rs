// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Cell Engine Identity, 8-bit to 5-Trit Codec, & Balanced Radix-3 Mapping )
// ==============================================================================
//! OmniCode Core: Cell Architecture & Byte-Trit Hardware Transducer
//!
//! Provides the physical hardware bridge between 8-bit binary silicon
//! and OmniCode's Balanced Ternary Quad-State physics:
//!     - 8 Bits (256 states) <--> 5 Trits (243 symmetric states) + 13 Control Traps
//!     - Dense packing: 1 Cell = exactly 1 physical byte (zero struct overhead)
//!     - Invariant Out-of-band Quad-States:
//!         * 243: NULL / HALT (Void / Unmapped memory)
//!         * 244: AWAIT (Superposition / Pending / Lock)
//!         * 245: ROLLBACK (Veto / Tainted / Unwound cell)
//!         * 246: PROCEED (Affirmative sealed landmark)
//!         * 247..255: Hardware Traps & System Landmarks

use std::fmt;
use crate::core::linguistics::{GrammaticalClassification, GrammaticalEntity, PartOfSpeech};
use crate::core::ternary::QuadState;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Out-of-Band Constants, Trit Definition, and Cell Structures )
// ==============================================================================

pub const BYTE_HALT: u8 = 243;
pub const BYTE_AWAIT: u8 = 244;
pub const BYTE_ROLLBACK: u8 = 245;
pub const BYTE_PROCEED: u8 = 246;
pub const BYTE_TRAP_BASE: u8 = 247;
pub const MAX_NATURAL_CELL_VALUE: u8 = 242;
pub const MIN_SIGNED_CELL_VALUE: i8 = -121;
pub const MAX_SIGNED_CELL_VALUE: i8 = 121;

/// The atomic ternary digit (Radix-3 balanced unit).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Trit {
    /// -1: Negative polarity / Contra (denoted 'T')
    Neg,
    ///  0: Neutral balance / Equilibrium (denoted '0')
    Zero,
    /// +1: Positive polarity / Affirmation (denoted '1')
    Pos,
}

impl Trit {
    pub fn as_i8(&self) -> i8 {
        match self {
            Trit::Neg => -1,
            Trit::Zero => 0,
            Trit::Pos => 1,
        }
    }

    pub fn from_i8(val: i8) -> Result<Self, String> {
        match val {
            -1 => Ok(Trit::Neg),
            0 => Ok(Trit::Zero),
            1 => Ok(Trit::Pos),
            other => Err(format!("FAILPOINT [InvalidTrit]: '{other}' is not in {{-1, 0, +1}}")),
        }
    }

    pub fn char_symbol(&self) -> char {
        match self {
            Trit::Neg => 'T',
            Trit::Zero => '0',
            Trit::Pos => '1',
        }
    }

    /// Algebraic negation of a trit: NOT(x) = -x.
    pub fn negate(&self) -> Self {
        match self {
            Trit::Neg => Trit::Pos,
            Trit::Zero => Trit::Zero,
            Trit::Pos => Trit::Neg,
        }
    }
}

impl fmt::Display for Trit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.char_symbol())
    }
}

/// The atomic storage unit of memory in OmniCode.
///
/// Holds 5 balanced trits (spanning 243 values) alongside its Quad-State resonance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell {
    pub trits: [Trit; 5],
    pub state: QuadState,
    pub is_marker: bool,
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Mathematical Conversion, Packing, Unpacking, and Word Operations )
// ==============================================================================

impl Cell {
    /// Constructs a Cell with explicit trits and Quad-State.
    pub fn new(trits: [Trit; 5], state: QuadState) -> Self {
        Self { trits, state, is_marker: false }
    }

    /// Creates an empty cell at equilibrium (0 in value, AWAIT in state).
    pub fn equilibrium() -> Self {
        Self {
            trits: [Trit::Zero; 5],
            state: QuadState::Await,
            is_marker: true,
        }
    }

    /// Creates a cell initialized with a pure out-of-band Quad-State marker.
    pub fn from_state(state: QuadState) -> Self {
        Self {
            trits: [Trit::Zero; 5],
            state,
            is_marker: true,
        }
    }

    /// Encodes a natural number in [0 ..= 242] into a 5-trit Cell with state PROCEED.
    pub fn from_natural(value: u8) -> Result<Self, String> {
        if value > MAX_NATURAL_CELL_VALUE {
            return Err(format!(
                "FAILPOINT [CellOverflow]: Value {value} exceeds single-cell capacity ({MAX_NATURAL_CELL_VALUE}). \
                 Use multi-cell Word chaining."
            ));
        }
        let signed_val = (value as i16) + (MIN_SIGNED_CELL_VALUE as i16);
        Self::from_signed(signed_val as i8)
    }

    /// Encodes a signed balanced integer in [-121 ..= +121] into 5 balanced trits.
    pub fn from_signed(value: i8) -> Result<Self, String> {
        if !(MIN_SIGNED_CELL_VALUE..=MAX_SIGNED_CELL_VALUE).contains(&value) {
            return Err(format!(
                "FAILPOINT [CellRangeError]: Signed value {value} out of range [{MIN_SIGNED_CELL_VALUE} ..= {MAX_SIGNED_CELL_VALUE}]."
            ));
        }

        let mut x = value;
        let mut trits = [Trit::Zero; 5];

        for i in 0..5 {
            let rem = ((x % 3) + 3) % 3;
            match rem {
                0 => {
                    trits[i] = Trit::Zero;
                    x /= 3;
                }
                1 => {
                    trits[i] = Trit::Pos;
                    x = (x - 1) / 3;
                }
                2 => {
                    trits[i] = Trit::Neg;
                    x = (x + 1) / 3;
                }
                _ => unreachable!(),
            }
        }

        Ok(Self {
            trits,
            state: QuadState::Proceed,
            is_marker: false,
        })
    }

    /// Decodes the 5 trits back to a signed integer in [-121 ..= +121].
    pub fn to_signed(&self) -> Option<i8> {
        if self.is_marker || self.state == QuadState::Halt {
            return None;
        }

        let mut total: i32 = 0;
        let mut multiplier: i32 = 1;

        for trit in &self.trits {
            total += (trit.as_i8() as i32) * multiplier;
            multiplier *= 3;
        }

        Some(total as i8)
    }

    /// Decodes the 5 trits back to a natural number in [0 ..= 242].
    pub fn to_natural(&self) -> Option<u8> {
        self.to_signed().map(|s| (s as i16 - (MIN_SIGNED_CELL_VALUE as i16)) as u8)
    }

    /// Packs this Cell into a single physical 8-bit byte.
    pub fn pack(&self) -> u8 {
        if self.is_marker {
            match self.state {
                QuadState::Halt => BYTE_HALT,
                QuadState::Await => BYTE_AWAIT,
                QuadState::Rollback => BYTE_ROLLBACK,
                QuadState::Proceed => BYTE_PROCEED,
            }
        } else {
            self.to_natural().unwrap_or(BYTE_HALT)
        }
    }

    /// Unpacks a single physical 8-bit byte into a Cell with full 5-trit fidelity and Quad-State.
    pub fn unpack(byte: u8) -> Self {
        match byte {
            BYTE_HALT => Self::from_state(QuadState::Halt),
            BYTE_AWAIT => Self::from_state(QuadState::Await),
            BYTE_ROLLBACK => Self::from_state(QuadState::Rollback),
            BYTE_PROCEED => Self::from_state(QuadState::Proceed),
            trap if trap >= BYTE_TRAP_BASE => {
                // Reserved system trap / hardware landmark
                Self::from_state(QuadState::Halt)
            }
            natural => {
                Self::from_natural(natural).unwrap_or_else(|_| Self::from_state(QuadState::Halt))
            }
        }
    }
}

impl GrammaticalEntity for Cell {
    fn classify(&self) -> GrammaticalClassification {
        GrammaticalClassification::new(
            PartOfSpeech::Noun,
            Some(PartOfSpeech::Preposition),
            None,
            "Atomic memory vessel; stores 5 balanced trits and Quad-State resonance",
            "Single 8-bit memory byte carrying payload and state",
        )
    }
}

impl GrammaticalEntity for Trit {
    fn classify(&self) -> GrammaticalClassification {
        GrammaticalClassification::new(
            PartOfSpeech::Noun,
            Some(PartOfSpeech::Adjective),
            None,
            "Atomic ternary digit (-1, 0, +1)",
            "Single Radix-3 unit of computational polarity",
        )
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.state == QuadState::Halt {
            write!(f, "Cell[NULL]")
        } else if let Some(nat) = self.to_natural() {
            write!(
                f,
                "Cell[Val:{nat} | {}{}{}{}{} | {:?}]",
                self.trits[4], self.trits[3], self.trits[2], self.trits[1], self.trits[0], self.state
            )
        } else {
            write!(f, "Cell[{:?}]", self.state)
        }
    }
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Bijective Round-Trip Verification & Mathematical Unit Tests )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signed_round_trip_all_243_values() {
        for s in MIN_SIGNED_CELL_VALUE..=MAX_SIGNED_CELL_VALUE {
            let cell = Cell::from_signed(s).expect("Signed conversion must succeed");
            let recovered = cell.to_signed().expect("To signed must succeed");
            assert_eq!(s, recovered, "Failed round-trip for signed value {s}");
        }
    }

    #[test]
    fn test_natural_round_trip_all_243_values() {
        for n in 0..=MAX_NATURAL_CELL_VALUE {
            let cell = Cell::from_natural(n).expect("Natural conversion must succeed");
            let recovered = cell.to_natural().expect("To natural must succeed");
            assert_eq!(n, recovered, "Failed round-trip for natural number {n}");

            // Verify physical 8-bit byte pack and unpack
            let packed_byte = cell.pack();
            assert_eq!(packed_byte, n);
            let unpacked_cell = Cell::unpack(packed_byte);
            assert_eq!(unpacked_cell.to_natural(), Some(n));
            assert_eq!(unpacked_cell.state, QuadState::Proceed);
        }
    }

    #[test]
    fn test_out_of_band_quad_state_markers() {
        // Test HALT (NULL)
        let halt_cell = Cell::from_state(QuadState::Halt);
        let packed_halt = halt_cell.pack();
        assert_eq!(packed_halt, BYTE_HALT);
        let unpacked_halt = Cell::unpack(packed_halt);
        assert_eq!(unpacked_halt.state, QuadState::Halt);

        // Test AWAIT
        let await_cell = Cell::from_state(QuadState::Await);
        let packed_await = await_cell.pack();
        assert_eq!(packed_await, BYTE_AWAIT);
        let unpacked_await = Cell::unpack(packed_await);
        assert_eq!(unpacked_await.state, QuadState::Await);

        // Test ROLLBACK
        let rollback_cell = Cell::from_state(QuadState::Rollback);
        let packed_rollback = rollback_cell.pack();
        assert_eq!(packed_rollback, BYTE_ROLLBACK);
        let unpacked_rollback = Cell::unpack(packed_rollback);
        assert_eq!(unpacked_rollback.state, QuadState::Rollback);

        // Test PROCEED marker
        let proceed_cell = Cell::from_state(QuadState::Proceed);
        assert_eq!(proceed_cell.pack(), BYTE_PROCEED);
        let unpacked_proceed = Cell::unpack(BYTE_PROCEED);
        assert_eq!(unpacked_proceed.state, QuadState::Proceed);
    }

    #[test]
    fn test_zero_and_equilibrium_properties() {
        let zero_cell = Cell::from_signed(0).unwrap();
        assert_eq!(zero_cell.trits, [Trit::Zero; 5]);
        assert_eq!(zero_cell.to_signed(), Some(0));
        assert_eq!(zero_cell.to_natural(), Some(121)); // 121 is the balanced center of [0 .. 242]
    }
}

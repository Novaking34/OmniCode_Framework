// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Physical Machine Word Architecture: 64-Bit Silicon & 32-Trit OS Substrate )
// ==============================================================================
//! OmniCode Core: Physical 64-Bit Machine Word & Logical 32-Trit Vector
//!
//! Bridges physical 64-bit binary silicon hardware with the 32-trit balanced
//! ternary operating system kernel:
//!
//! # Bitfield Layout (64 Physical Bits)
//! - **Bits 0..50 (51 bits):** 32-Trit Balanced Ternary Payload (Range: $3^{32} \approx 1.85 \times 10^{15}$)
//! - **Bits 51..52 (2 bits):** Quad-State / Transport Action (Play, Pause, Rewind, Stop)
//! - **Bit 53 (1 bit):** Polarity Affix (0 = Abstract `+`, 1 = Concrete `-`)
//! - **Bits 54..56 (3 bits):** Boundary Rank Delimiter (`()`, `[]`, `<>`, `{}`)
//! - **Bits 57..63 (7 bits):** Moral Invariant Checksum & Axiom Flags (Truth, Life, Justice, etc.)

use std::fmt;
use crate::core::boundaries::BoundaryType;
use crate::core::operators::Polarity;
use crate::core::ternary::{QuadState, TransportAction};

pub const POWERS_OF_3: [u64; 33] = {
    let mut p = [0u64; 33];
    p[0] = 1;
    let mut i = 1;
    while i <= 32 {
        p[i] = p[i - 1] * 3;
        i += 1;
    }
    p
};

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Word32Trit: 32-Trit Balanced Ternary ALU Vector )
// ==============================================================================

/// A 32-trit balanced ternary word.
/// Each trit is in `{-1, 0, +1}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Word32Trit {
    pub trits: [i8; 32],
}

impl Default for Word32Trit {
    fn default() -> Self {
        Self::zero()
    }
}

impl Word32Trit {
    /// Creates a zeroed 32-trit word (equilibrium: all 0).
    pub const fn zero() -> Self {
        Self { trits: [0; 32] }
    }

    /// Creates a 32-trit word from an explicit array of trits.
    pub fn new(trits: [i8; 32]) -> Result<Self, String> {
        for (idx, &t) in trits.iter().enumerate() {
            if t < -1 || t > 1 {
                return Err(format!("Invalid trit at index {idx}: {t}. Permitted: -1, 0, 1"));
            }
        }
        Ok(Self { trits })
    }

    /// Converts a signed 64-bit integer into a balanced ternary 32-trit word.
    pub fn from_i64(mut val: i64) -> Self {
        let mut trits = [0i8; 32];
        for i in 0..32 {
            if val == 0 {
                break;
            }
            let rem = ((val % 3) + 3) % 3;
            if rem == 0 {
                trits[i] = 0;
                val /= 3;
            } else if rem == 1 {
                trits[i] = 1;
                val = (val - 1) / 3;
            } else {
                // rem == 2 -> trit is -1, adjust carry (+1)
                trits[i] = -1;
                val = (val + 1) / 3;
            }
        }
        Self { trits }
    }

    /// Decodes the 32-trit word into a standard signed 64-bit integer.
    pub fn to_i64(&self) -> i64 {
        let mut total: i64 = 0;
        let mut power: i64 = 1;
        for i in 0..32 {
            total = total.saturating_add((self.trits[i] as i64).saturating_mul(power));
            if i < 31 {
                power = power.saturating_mul(3);
            }
        }
        total
    }

    /// Negates the 32-trit word in-place with zero sign-extension cost.
    pub fn negate(&self) -> Self {
        let mut trits = [0i8; 32];
        for i in 0..32 {
            trits[i] = -self.trits[i];
        }
        Self { trits }
    }

    /// Balanced ternary full-adder for two 32-trit words.
    pub fn add(&self, other: &Self) -> (Self, i8) {
        let mut trits = [0i8; 32];
        let mut carry: i8 = 0;

        for i in 0..32 {
            let sum = self.trits[i] + other.trits[i] + carry;
            match sum {
                3 => {
                    trits[i] = 0;
                    carry = 1;
                }
                2 => {
                    trits[i] = -1;
                    carry = 1;
                }
                1 => {
                    trits[i] = 1;
                    carry = 0;
                }
                0 => {
                    trits[i] = 0;
                    carry = 0;
                }
                -1 => {
                    trits[i] = -1;
                    carry = 0;
                }
                -2 => {
                    trits[i] = 1;
                    carry = -1;
                }
                -3 => {
                    trits[i] = 0;
                    carry = -1;
                }
                _ => unreachable!("Sum out of range: {sum}"),
            }
        }
        (Self { trits }, carry)
    }

    /// Packs the 32-trit word into an unsigned 51-bit binary representation.
    pub fn pack_51bits(&self) -> u64 {
        let mut packed: u64 = 0;
        for i in 0..32 {
            let digit = (self.trits[i] + 1) as u64; // Map -1,0,1 to 0,1,2
            packed += digit * POWERS_OF_3[i];
        }
        packed
    }

    /// Unpacks a 32-trit word from an unsigned 51-bit binary representation.
    pub fn unpack_51bits(mut val: u64) -> Self {
        let mut trits = [0i8; 32];
        for i in 0..32 {
            let rem = (val % 3) as i8;
            trits[i] = rem - 1; // Map 0,1,2 back to -1,0,1
            val /= 3;
        }
        Self { trits }
    }
}

impl fmt::Display for Word32Trit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0t")?;
        for i in (0..32).rev() {
            match self.trits[i] {
                1 => write!(f, "+")?,
                0 => write!(f, "0")?,
                -1 => write!(f, "-")?,
                _ => write!(f, "?")?,
            }
        }
        write!(f, " ({})", self.to_i64())
    }
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Word64: 64-Bit Physical Silicon Word Container )
// ==============================================================================

/// A 64-bit physical silicon machine word combining a 32-trit payload
/// with out-of-band operational metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Word64 {
    pub raw: u64,
}

impl Default for Word64 {
    fn default() -> Self {
        Self::new(Word32Trit::zero(), QuadState::Proceed, Polarity::Concrete, BoundaryType::Denotation, 0)
    }
}

impl Word64 {
    const PAYLOAD_MASK: u64   = 0x0007_FFFF_FFFF_FFFF; // Bits 0..50 (51 bits)
    const STATE_SHIFT: u32    = 51;
    const STATE_MASK: u64     = 0x3;                   // Bits 51..52 (2 bits)
    const POLARITY_SHIFT: u32 = 53;
    const POLARITY_MASK: u64  = 0x1;                   // Bit 53 (1 bit)
    const BOUNDARY_SHIFT: u32 = 54;
    const BOUNDARY_MASK: u64  = 0x7;                   // Bits 54..56 (3 bits)
    const CHECKSUM_SHIFT: u32 = 57;
    const CHECKSUM_MASK: u64  = 0x7F;                  // Bits 57..63 (7 bits)

    /// Constructs a physical 64-bit word from components.
    pub fn new(
        payload: Word32Trit,
        state: QuadState,
        polarity: Polarity,
        boundary: BoundaryType,
        checksum: u8,
    ) -> Self {
        let p_val = payload.pack_51bits() & Self::PAYLOAD_MASK;
        let s_val = match state {
            QuadState::Await => 0u64,
            QuadState::Proceed => 1u64,
            QuadState::Rollback => 2u64,
            QuadState::Halt => 3u64,
        };
        let pol_val = match polarity {
            Polarity::Abstract => 0u64,
            Polarity::Concrete => 1u64,
        };
        let b_val = match boundary {
            BoundaryType::Invariant => 0u64,
            BoundaryType::Denotation => 1u64,
            BoundaryType::Connotation => 2u64,
            BoundaryType::BottomUp => 3u64,
            BoundaryType::TopDown => 4u64,
            BoundaryType::Aside => 5u64,
        };
        let c_val = (checksum as u64) & Self::CHECKSUM_MASK;

        let raw = p_val
            | (s_val << Self::STATE_SHIFT)
            | (pol_val << Self::POLARITY_SHIFT)
            | (b_val << Self::BOUNDARY_SHIFT)
            | (c_val << Self::CHECKSUM_SHIFT);

        Self { raw }
    }

    /// Returns a zeroed physical 64-bit word with equilibrium payload and Proceed/PLAY transport.
    pub fn zero() -> Self {
        Self::from_signed_int(0)
    }

    /// Convenience constructor from a signed integer value with default Proceed/PLAY transport.
    pub fn from_signed_int(val: i64) -> Self {
        Self::new(
            Word32Trit::from_i64(val),
            QuadState::Proceed,
            Polarity::Concrete,
            BoundaryType::Denotation,
            0,
        )
    }

    /// Convenience constructor from a 32-trit word with default Proceed/PLAY transport.
    pub fn from_trit_payload(payload: Word32Trit) -> Self {
        Self::new(
            payload,
            QuadState::Proceed,
            Polarity::Concrete,
            BoundaryType::Denotation,
            0,
        )
    }

    /// Extracts the 32-trit payload.
    pub fn payload(&self) -> Word32Trit {
        let p_bits = self.raw & Self::PAYLOAD_MASK;
        Word32Trit::unpack_51bits(p_bits)
    }

    /// Extracts the QuadState / Transport Action.
    pub fn state(&self) -> QuadState {
        let s_bits = (self.raw >> Self::STATE_SHIFT) & Self::STATE_MASK;
        match s_bits {
            0 => QuadState::Await,
            1 => QuadState::Proceed,
            2 => QuadState::Rollback,
            _ => QuadState::Halt,
        }
    }

    /// Returns the cinematic transport action: Play, Pause, Rewind, Stop.
    pub fn transport(&self) -> TransportAction {
        self.state().transport()
    }

    /// Extracts the Polarity affix.
    pub fn polarity(&self) -> Polarity {
        let p_bit = (self.raw >> Self::POLARITY_SHIFT) & Self::POLARITY_MASK;
        if p_bit == 0 {
            Polarity::Abstract
        } else {
            Polarity::Concrete
        }
    }

    /// Extracts the Boundary Type.
    pub fn boundary(&self) -> BoundaryType {
        let b_bits = (self.raw >> Self::BOUNDARY_SHIFT) & Self::BOUNDARY_MASK;
        match b_bits {
            0 => BoundaryType::Invariant,
            1 => BoundaryType::Denotation,
            2 => BoundaryType::Connotation,
            3 => BoundaryType::BottomUp,
            4 => BoundaryType::TopDown,
            _ => BoundaryType::Aside,
        }
    }

    /// Extracts the moral invariant checksum byte.
    pub fn checksum(&self) -> u8 {
        ((self.raw >> Self::CHECKSUM_SHIFT) & Self::CHECKSUM_MASK) as u8
    }

    /// Formats a complete human-readable physical register inspection string.
    pub fn inspect(&self) -> String {
        format!(
            "0x{:016X} | {} | State: {:<8} (Transport: {:<6}) | Polarity: {} | Bound: {}",
            self.raw,
            self.payload(),
            self.state().macro_op(),
            self.transport().as_str(),
            self.polarity().symbol(),
            self.boundary().delimiters().0
        )
    }
}



// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests & Verification Proof )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word32_trit_integer_conversion_round_trip() {
        let test_cases = [0, 1, -1, 4, -4, 42, -42, 243, -243, 100_000, -100_000, 1_000_000_000, -1_000_000_000];
        for &val in &test_cases {
            let w = Word32Trit::from_i64(val);
            let back = w.to_i64();
            assert_eq!(val, back, "Round trip failed for value {val}");
        }
    }

    #[test]
    fn test_word32_trit_pack_unpack_round_trip() {
        let values = [0, 1, -1, 12345, -54321, 999_999_999, -999_999_999];
        for &v in &values {
            let w = Word32Trit::from_i64(v);
            let packed = w.pack_51bits();
            let unpacked = Word32Trit::unpack_51bits(packed);
            assert_eq!(w, unpacked, "Pack/unpack mismatch for value {v}");
            assert_eq!(v, unpacked.to_i64());
        }
    }

    #[test]
    fn test_word32_trit_balanced_addition() {
        let a = Word32Trit::from_i64(42);
        let b = Word32Trit::from_i64(58);
        let (sum, carry) = a.add(&b);
        assert_eq!(carry, 0);
        assert_eq!(sum.to_i64(), 100);

        let c = Word32Trit::from_i64(15);
        let d = Word32Trit::from_i64(-25);
        let (sum2, carry2) = c.add(&d);
        assert_eq!(carry2, 0);
        assert_eq!(sum2.to_i64(), -10);
    }

    #[test]
    fn test_word64_physical_container_round_trip() {
        let payload = Word32Trit::from_i64(1337);
        let w64 = Word64::new(payload, QuadState::Proceed, Polarity::Concrete, BoundaryType::Denotation, 42);

        assert_eq!(w64.payload().to_i64(), 1337);
        assert_eq!(w64.state(), QuadState::Proceed);
        assert_eq!(w64.transport(), TransportAction::Play);
        assert_eq!(w64.polarity(), Polarity::Concrete);
        assert_eq!(w64.boundary(), BoundaryType::Denotation);
        assert_eq!(w64.checksum(), 42);
    }
}

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

/// Dual-rail bit-plane masks (32 trits x 2 bits = 64 bits native)
pub const BITPLANE_POS_MASK: u64 = 0x5555_5555_5555_5555;
pub const BITPLANE_NEG_MASK: u64 = 0xAAAA_AAAA_AAAA_AAAA;

/// 2-bit dual-rail polar encodings:
/// 00 = 0 (Equilibrium)
/// 01 = +1 (Positive rail)
/// 10 = -1 (Negative rail)
/// 11 = NULL (Collision trap)
pub const TRIT_CODE_ZERO: u8 = 0;
pub const TRIT_CODE_POS: u8 = 1;
pub const TRIT_CODE_NEG: u8 = 2;
pub const TRIT_CODE_NULL: u8 = 3;

/// Kinematic bit-packing topologies across 8-bit byte windows
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackMode {
    /// 2-2-2-2: Standard dense storage (4 trits)
    Dense2222,
    /// 2-1-2-1-2: Symmetric resting equilibrium with carry channels
    Kinematic21212,
    /// 1-2-1-2-2: Prefix carry/overflow channel
    Kinematic12122,
    /// 2-2-1-2-1: Postfix carry/overflow channel
    Kinematic22121,
    /// 3-3-2: Directional vector (X:3, Y:3, Z:2)
    Directional332,
    /// 3-2-3: Directional vector (X:3, Y:2, Z:3)
    Directional323,
    /// 2-3-3: Directional vector (X:2, Y:3, Z:3)
    Directional233,
}

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

    /// Encodes a single trit into a 2-bit polar pair:
    /// 0 -> 0b00, 1 -> 0b01, -1 -> 0b10, other -> 0b11.
    pub fn trit_to_2bit(trit: i8) -> u8 {
        match trit {
            0 => TRIT_CODE_ZERO,
            1 => TRIT_CODE_POS,
            -1 => TRIT_CODE_NEG,
            _ => TRIT_CODE_NULL,
        }
    }

    /// Decodes a 2-bit polar pair into a balanced trit:
    /// 0b00 -> 0, 0b01 -> +1, 0b10 -> -1, 0b11 -> Err(Collision).
    pub fn two_bit_to_trit(code: u8) -> Result<i8, String> {
        match code & 0x03 {
            TRIT_CODE_ZERO => Ok(0),
            TRIT_CODE_POS => Ok(1),
            TRIT_CODE_NEG => Ok(-1),
            _ => Err("Dual-rail collision trap: code 11 (both rails active)".to_string()),
        }
    }

    /// Packs the 32-trit word into a 64-bit dual-rail polar bit-plane word.
    pub fn pack_dual_rail(&self) -> u64 {
        let mut packed: u64 = 0;
        for i in 0..32 {
            let code = Self::trit_to_2bit(self.trits[i]) as u64;
            packed |= (code & 0x03) << (i * 2);
        }
        packed
    }

    /// Unpacks a 64-bit dual-rail polar bit-plane word into a 32-trit word.
    /// Trips interlock (Err) if any 2-bit slot has collision code 11.
    pub fn unpack_dual_rail(val: u64) -> Result<Self, String> {
        let mut trits = [0i8; 32];
        for i in 0..32 {
            let code = ((val >> (i * 2)) & 0x03) as u8;
            trits[i] = Self::two_bit_to_trit(code)?;
        }
        Ok(Self { trits })
    }

    /// Checks if a 64-bit dual-rail word has a NULL collision (code 11).
    pub fn has_null_collision(val: u64) -> bool {
        let pos = val & BITPLANE_POS_MASK;
        let neg = (val >> 1) & BITPLANE_POS_MASK;
        (pos & neg) != 0
    }

    /// Negates a 64-bit dual-rail packed word in O(1) via SIMD rail swap.
    pub fn simd_rail_swap_negate(val: u64) -> u64 {
        let pos = val & BITPLANE_POS_MASK;
        let neg = val & BITPLANE_NEG_MASK;
        ((pos << 1) & BITPLANE_NEG_MASK) | ((neg >> 1) & BITPLANE_POS_MASK)
    }

    /// Packs a slice of trits into a single byte using the specified kinematic topology.
    pub fn pack_byte(trits: &[i8], mode: PackMode) -> u8 {
        let get = |idx: usize| -> u8 {
            if idx < trits.len() {
                Self::trit_to_2bit(trits[idx]) & 0x03
            } else {
                0
            }
        };
        let mut b: u8 = 0;
        match mode {
            PackMode::Dense2222 => {
                for i in 0..4 {
                    b |= get(i) << (i * 2);
                }
            }
            PackMode::Kinematic21212 => {
                b |= get(0) & 0x03;
                b |= (get(1) & 0x01) << 2;
                b |= (get(1) & 0x03) << 3;
                b |= ((get(1) >> 1) & 0x01) << 5;
                b |= (get(2) & 0x03) << 6;
            }
            PackMode::Kinematic12122 => {
                b |= (get(0) & 0x01) << 0;
                b |= (get(0) & 0x03) << 1;
                b |= ((get(1) >> 1) & 0x01) << 3;
                b |= (get(1) & 0x03) << 4;
                b |= (get(2) & 0x03) << 6;
            }
            PackMode::Kinematic22121 => {
                b |= (get(0) & 0x03) << 0;
                b |= (get(1) & 0x03) << 2;
                b |= (get(2) & 0x01) << 4;
                b |= (get(2) & 0x03) << 5;
                b |= ((get(2) >> 1) & 0x01) << 7;
            }
            PackMode::Directional332 => {
                b |= get(0) << 0;
                b |= get(1) << 3;
                b |= get(2) << 6;
            }
            PackMode::Directional323 => {
                b |= get(0) << 0;
                b |= get(1) << 3;
                b |= get(2) << 5;
            }
            PackMode::Directional233 => {
                b |= get(0) << 0;
                b |= get(1) << 2;
                b |= get(2) << 5;
            }
        }
        b
    }

    /// Unpacks a single byte into up to 4 trits using the specified kinematic topology.
    pub fn unpack_byte(b: u8, mode: PackMode) -> Result<[i8; 4], String> {
        let mut out = [0i8; 4];
        match mode {
            PackMode::Dense2222 => {
                for i in 0..4 {
                    out[i] = Self::two_bit_to_trit((b >> (i * 2)) & 0x03)?;
                }
            }
            PackMode::Kinematic21212 => {
                out[0] = Self::two_bit_to_trit(b & 0x03)?;
                out[1] = Self::two_bit_to_trit((b >> 3) & 0x03)?;
                out[2] = Self::two_bit_to_trit((b >> 6) & 0x03)?;
                out[3] = 0;
            }
            PackMode::Kinematic12122 => {
                out[0] = Self::two_bit_to_trit((b >> 1) & 0x03)?;
                out[1] = Self::two_bit_to_trit((b >> 4) & 0x03)?;
                out[2] = Self::two_bit_to_trit((b >> 6) & 0x03)?;
                out[3] = 0;
            }
            PackMode::Kinematic22121 => {
                out[0] = Self::two_bit_to_trit(b & 0x03)?;
                out[1] = Self::two_bit_to_trit((b >> 2) & 0x03)?;
                out[2] = Self::two_bit_to_trit((b >> 5) & 0x03)?;
                out[3] = 0;
            }
            PackMode::Directional332 => {
                out[0] = Self::two_bit_to_trit(b & 0x03)?;
                out[1] = Self::two_bit_to_trit((b >> 3) & 0x03)?;
                out[2] = Self::two_bit_to_trit((b >> 6) & 0x03)?;
                out[3] = 0;
            }
            PackMode::Directional323 => {
                out[0] = Self::two_bit_to_trit(b & 0x03)?;
                out[1] = Self::two_bit_to_trit((b >> 3) & 0x03)?;
                out[2] = Self::two_bit_to_trit((b >> 5) & 0x03)?;
                out[3] = 0;
            }
            PackMode::Directional233 => {
                out[0] = Self::two_bit_to_trit(b & 0x03)?;
                out[1] = Self::two_bit_to_trit((b >> 2) & 0x03)?;
                out[2] = Self::two_bit_to_trit((b >> 5) & 0x03)?;
                out[3] = 0;
            }
        }
        Ok(out)
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

    #[test]
    fn test_word32_dual_rail_bitplanes() {
        assert_eq!(Word32Trit::trit_to_2bit(0), TRIT_CODE_ZERO);
        assert_eq!(Word32Trit::trit_to_2bit(1), TRIT_CODE_POS);
        assert_eq!(Word32Trit::trit_to_2bit(-1), TRIT_CODE_NEG);

        assert_eq!(Word32Trit::two_bit_to_trit(0).unwrap(), 0);
        assert_eq!(Word32Trit::two_bit_to_trit(1).unwrap(), 1);
        assert_eq!(Word32Trit::two_bit_to_trit(2).unwrap(), -1);
        assert!(Word32Trit::two_bit_to_trit(3).is_err());

        let values = [0, 1, -1, 4096, -4096, 65536, -65536, 123456789, -123456789];
        for &v in &values {
            let w = Word32Trit::from_i64(v);
            let packed = w.pack_dual_rail();
            assert!(!Word32Trit::has_null_collision(packed));
            let unpacked = Word32Trit::unpack_dual_rail(packed).expect("Unpack dual rail");
            assert_eq!(w, unpacked, "Dual-rail mismatch for {v}");
            assert_eq!(v, unpacked.to_i64());
        }
    }

    #[test]
    fn test_word32_simd_rail_swap_negation() {
        let w = Word32Trit::from_i64(4096);
        let packed = w.pack_dual_rail();
        let negated_packed = Word32Trit::simd_rail_swap_negate(packed);
        let unpacked_neg = Word32Trit::unpack_dual_rail(negated_packed).expect("Unpack negated");
        assert_eq!(unpacked_neg.to_i64(), -4096);

        // Involution: -(-x) == x
        let double_neg = Word32Trit::simd_rail_swap_negate(negated_packed);
        assert_eq!(double_neg, packed);
    }

    #[test]
    fn test_word32_null_collision_trap() {
        let w = Word32Trit::from_i64(1);
        let mut packed = w.pack_dual_rail();
        assert!(!Word32Trit::has_null_collision(packed));

        // Inject 11 into slot 0
        packed |= 3;
        assert!(Word32Trit::has_null_collision(packed));
        assert!(Word32Trit::unpack_dual_rail(packed).is_err());
    }

    #[test]
    fn test_word32_dynamic_packing_topologies() {
        let modes = [
            PackMode::Dense2222,
            PackMode::Kinematic21212,
            PackMode::Kinematic12122,
            PackMode::Kinematic22121,
            PackMode::Directional332,
            PackMode::Directional323,
            PackMode::Directional233,
        ];
        let trits = [1, -1, 1, -1];
        for &mode in &modes {
            let byte_val = Word32Trit::pack_byte(&trits, mode);
            let unpacked = Word32Trit::unpack_byte(byte_val, mode).expect("Unpack byte");
            assert_eq!(unpacked[0], trits[0]);
            assert_eq!(unpacked[1], trits[1]);
            assert_eq!(unpacked[2], trits[2]);
            if mode == PackMode::Dense2222 {
                assert_eq!(unpacked[3], trits[3]);
            }
        }
    }
}

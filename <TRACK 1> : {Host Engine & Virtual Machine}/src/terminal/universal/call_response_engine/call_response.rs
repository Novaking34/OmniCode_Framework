// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Universal Call-Response Engine: Operator <-> Machine Bidirectional Ribbon )
// ==============================================================================
//! OmniCode Terminal: Universal Call & Response Protocol Engine
//!
//! # Architectural Scope & Purpose:
//! Implements the universal **Call & Response** mathematical model of the
//! OmniCode framework:
//!
//! - **The Call (Caller / Sender Input):**
//!   Rank 5 outside-in asymmetric imposition into the silicon/virtual container:
//!   `{ Call ] --> INSCRIBE [ Container ] = [ Entity' ]`.
//! - **The Response (Via Caller / Sender Output):**
//!   Rank 4 inside-out emergence projecting verified state back to the sender:
//!   `[ Entity' ] --> PROJECT [ Response ] = [ Response' ]`.
//!
//! # Reversibility & AI Cognition:
//! Because OmniCode is designed for creating an AI model (where `.omni` and `.tape`
//! are the compiled and interpreted forms of the OmniCode framework rather than
//! passive data types), the Operator and the Machine (the computer) are symmetric
//! peers taking turns as Caller and Sender:
//! - **Operator Calls -> Machine Responds:** User/Agent commands, scalar inputs, glyphs.
//! - **Machine Calls -> Operator Responds:** Machine interrupts, hardware telemetry,
//!   predictive inference requests, and future networking packets across distributed nodes.
//!
//! An "echo" is simply the special case of Call-Response that repeats what was called
//! for after proving invariant bounds and ALU equilibrium.
//!
//! # Scripture Witness:
//! - **Job 14:15 (KJV)**:
//!   "Thou shalt call, and I will answer thee: thou wilt have a desire to the
//!    work of thine hands."
//!   (The primordial covenant of Call and Response between Creator and Creation).
//! - **Isaiah 65:24 (KJV)**:
//!   "And it shall come to pass, that before they call, I will answer;
//!    and while they are yet speaking, I will hear."
//!   (Zero-latency predictive Call-Response in equilibrium).
// ==============================================================================

use crate::core::cell::{Cell, Trit};
use crate::core::ternary::QuadState;
use crate::core::word::{PackMode, Word32Trit};

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Types, Enums & Status Containers )
// ==============================================================================

/// A Call payload transmitted by the Caller / Sender (Operator or Machine).
#[derive(Debug, Clone, PartialEq)]
pub enum Call {
    /// Inscription of an atomic symbolic rune / glyph (1 Cell capacity).
    Character(char),
    /// Inscription of a scalar magnitude / quantity (1 Word capacity).
    Number(i64),
    /// Inscription of a composite stream of characters (Text ribbon).
    Stream(String),
    /// Direct injection of a 64-bit dual-rail polar bit-plane register.
    DualRail(u64),
}

/// Result of a verified Character Call-Response turn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterResponse {
    /// Echoed / verified character glyph.
    pub glyph: char,
    /// 5 balanced trits representing the character in virtual cell memory.
    pub trits: [i8; 5],
    /// Balanced ternary Quad-State (+1:PROCEED).
    pub state: QuadState,
    /// Verified cell capacity footprint (always 1 Cell).
    pub cell_footprint: usize,
}

/// Result of a verified Number Call-Response turn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberResponse {
    /// Outside-in scalar integer quantity.
    pub scalar: i64,
    /// 32 balanced trits representing the quantity in virtual word memory.
    pub trits: [i8; 32],
    /// Native 64-bit dual-rail polar bit-plane register (00=0, 01=+1, 10=-1, 11=Trap).
    pub packed_dual_rail: u64,
    /// Canonical dense 2-2-2-2 packing of least significant 4 trits (1 byte).
    pub byte_pack_2222: u8,
    /// Kinematic 2-1-2-1-2 resting equilibrium packing of least significant 3 trits (1 byte).
    pub byte_pack_21212: u8,
    /// Proof of ALU Zero-Sum Equilibrium Law: (+1) + (-1) <=> 0.
    pub equilibrium_proved: bool,
    /// Proof of SIMD rail-swap negation involution: -(-x) == x.
    pub rail_involution_proved: bool,
    /// Balanced ternary Quad-State (+1:PROCEED).
    pub state: QuadState,
    /// Verified word capacity footprint (always 1 Word).
    pub word_footprint: usize,
}

/// Result of a verified Stream Call-Response turn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamResponse {
    /// Echoed / verified text stream.
    pub echoed: String,
    /// Alias for echoed stream text.
    pub text: String,
    /// Total count of atomic character cells verified.
    pub cell_count: usize,
    /// Balanced ternary Quad-State (+1:PROCEED).
    pub state: QuadState,
}

/// Universal Response delivered to the Caller / Sender via the output channel.
#[derive(Debug, Clone, PartialEq)]
pub enum Response {
    /// Verified Character response.
    Character(CharacterResponse),
    /// Verified Number response.
    Number(NumberResponse),
    /// Verified Stream response.
    Stream(StreamResponse),
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Universal Call-Response Engine Implementation )
// ==============================================================================

/// The Universal Call & Response Engine.
#[derive(Debug, Clone, Default)]
pub struct CallResponseEngine;

impl CallResponseEngine {
    /// Executes any Call, returning the corresponding verified Response.
    pub fn execute(call: Call) -> Result<Response, String> {
        match call {
            Call::Character(c) => Self::call_char(c).map(Response::Character),
            Call::Number(n) => Self::call_number(n).map(Response::Number),
            Call::Stream(s) => Self::call_stream(&s).map(Response::Stream),
            Call::DualRail(r) => Self::call_dual_rail(r).map(Response::Number),
        }
    }

    /// Executes the full Call-Response lifecycle for an atomic Character glyph:
    /// 1. Ingestion: Reads glyph payload from Caller.
    /// 2. Inscription: Allocates single 1-Cell container.
    /// 3. Transduction: Bijectively converts 8-bit byte <=> 5 balanced trits.
    /// 4. Guard: Checks identity, single-cell capacity, and ASCII boundary.
    /// 5. Response: Returns verified CharacterResponse.
    pub fn call_char(glyph: char) -> Result<CharacterResponse, String> {
        let byte = if (glyph as u32) <= 242 {
            glyph as u8
        } else {
            return Err(format!(
                "Character '{}' (U+{:04X}) exceeds single-cell capacity of 242 values",
                glyph, glyph as u32
            ));
        };

        let cell = Cell::from_natural(byte)?;
        let mut trits = [0i8; 5];
        for (i, t) in cell.trits.iter().enumerate() {
            trits[i] = t.as_i8();
        }

        let mut cell_trits = [Trit::Zero; 5];
        for (i, &t) in trits.iter().enumerate() {
            cell_trits[i] = Trit::from_i8(t)?;
        }
        let verified_cell = Cell::new(cell_trits, QuadState::Proceed);
        let reconstructed_byte = verified_cell
            .to_natural()
            .ok_or_else(|| "Failed to decode natural byte from cell".to_string())?;

        if reconstructed_byte != byte {
            return Err(format!(
                "Transduction anomaly: byte {byte} != reconstructed {reconstructed_byte}"
            ));
        }

        let reconstructed_char = reconstructed_byte as char;
        if reconstructed_char != glyph {
            return Err(format!(
                "Identity violation: glyph '{glyph}' != reconstructed '{reconstructed_char}'"
            ));
        }

        Ok(CharacterResponse {
            glyph: reconstructed_char,
            trits,
            state: QuadState::Proceed,
            cell_footprint: 1,
        })
    }

    /// Executes the full Call-Response lifecycle for an atomic Number scalar:
    /// 1. Ingestion: Captures scalar quantity from Caller.
    /// 2. Inscription: Allocates 1-Word memory container.
    /// 3. Transduction: Bijectively converts 64-bit integer <=> 32 balanced trits.
    /// 4. Polar Rail Packing: Maps 32 trits into 64-bit dual-rail polar planes.
    /// 5. Collision Guard: Ensures no `11` collision traps exist.
    /// 6. Equilibrium Proof: Proves (+1) + (-1) <=> 0 and SIMD rail-swap involution.
    /// 7. Response: Returns verified NumberResponse.
    pub fn call_number(scalar: i64) -> Result<NumberResponse, String> {
        // Step 1 & 2: Inscribe into 32-trit Word
        let word = Word32Trit::from_i64(scalar);

        // Step 3: Bijective round-trip validation
        let reconstructed_scalar = word.to_i64();
        if reconstructed_scalar != scalar {
            return Err(format!(
                "Number transduction overflow: input {scalar} != reconstructed {reconstructed_scalar}"
            ));
        }

        // Step 4: Dual-Rail polar bit-plane packing
        let packed_dual_rail = word.pack_dual_rail();

        // Step 5: Collision trap guard
        if Word32Trit::has_null_collision(packed_dual_rail) {
            return Err(format!(
                "Dual-rail collision trap: simultaneous activation on word for scalar {scalar}"
            ));
        }

        let unpacked_word = Word32Trit::unpack_dual_rail(packed_dual_rail)?;
        if unpacked_word != word {
            return Err(format!(
                "Dual-rail unpack parity mismatch on scalar {scalar}"
            ));
        }

        // Step 6: Zero-sum equilibrium proof (+1) + (-1) == 0
        let unity_pos = Word32Trit::from_i64(1);
        let unity_neg = Word32Trit::from_i64(-1);
        let (zero_sum, carry) = unity_pos.add(&unity_neg);
        let equilibrium_proved = (zero_sum.to_i64() == 0) && (carry == 0);

        // Step 7: SIMD rail-swap negation involution proof: -(-x) == x
        let negated = Word32Trit::simd_rail_swap_negate(packed_dual_rail);
        let double_negated = Word32Trit::simd_rail_swap_negate(negated);
        let rail_involution_proved = double_negated == packed_dual_rail;

        if !equilibrium_proved || !rail_involution_proved {
            return Err("ALU Zero-Sum Equilibrium or Rail-Swap Involution violated".to_string());
        }

        // Step 8: Dynamic kinematic byte topologies (least significant trits)
        let byte_pack_2222 = Word32Trit::pack_byte(&word.trits, PackMode::Dense2222);
        let byte_pack_21212 = Word32Trit::pack_byte(&word.trits, PackMode::Kinematic21212);

        Ok(NumberResponse {
            scalar,
            trits: word.trits,
            packed_dual_rail,
            byte_pack_2222,
            byte_pack_21212,
            equilibrium_proved,
            rail_involution_proved,
            state: QuadState::Proceed,
            word_footprint: 1,
        })
    }

    /// Executes Call-Response on a raw 64-bit dual-rail register word.
    pub fn call_dual_rail(packed: u64) -> Result<NumberResponse, String> {
        if Word32Trit::has_null_collision(packed) {
            return Err("Dual-rail collision trap detected in input register (code 11)".to_string());
        }
        let word = Word32Trit::unpack_dual_rail(packed)?;
        Self::call_number(word.to_i64())
    }

    /// Executes Call-Response across an entire text stream.
    pub fn call_stream(input: &str) -> Result<StreamResponse, String> {
        let mut echoed = String::with_capacity(input.len());
        let mut cell_count = 0;
        for c in input.chars() {
            let res = Self::call_char(c)?;
            echoed.push(res.glyph);
            cell_count += 1;
        }
        Ok(StreamResponse {
            echoed: echoed.clone(),
            text: echoed,
            cell_count,
            state: QuadState::Proceed,
        })
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Comprehensive Exhaustive Unit Tests: Character, Number Range & Reversibility )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_response_character_standard_ascii() {
        for byte in 0u8..=127u8 {
            let ch = byte as char;
            let resp = CallResponseEngine::call_char(ch)
                .unwrap_or_else(|e| panic!("Failed call_char for ASCII {byte}: {e}"));
            assert_eq!(resp.glyph, ch);
            assert_eq!(resp.state, QuadState::Proceed);
            assert_eq!(resp.cell_footprint, 1);
        }
    }

    #[test]
    fn test_call_response_number_zero_and_unities() {
        let unities = [-1i64, 0, 1];
        for &u in &unities {
            let resp = CallResponseEngine::call_number(u).expect("Unity must succeed");
            assert_eq!(resp.scalar, u);
            assert_eq!(resp.state, QuadState::Proceed);
            assert!(resp.equilibrium_proved);
            assert!(resp.rail_involution_proved);
            assert_eq!(resp.word_footprint, 1);
        }
    }

    #[test]
    fn test_call_response_number_byte_range() {
        // Exhaustively verify the entire shared 8-bit byte space (-128 through +127)
        for val in -128i64..=127i64 {
            let resp = CallResponseEngine::call_number(val)
                .unwrap_or_else(|e| panic!("Failed call_number for byte value {val}: {e}"));
            assert_eq!(resp.scalar, val);
            assert_eq!(resp.state, QuadState::Proceed);
            assert!(resp.equilibrium_proved);
            assert!(resp.rail_involution_proved);
        }
    }

    #[test]
    fn test_call_response_number_single_cell_trit_range() {
        // Single cell balanced ternary range: 3^5 = 243 values (-121 through +121)
        for val in -121i64..=121i64 {
            let resp = CallResponseEngine::call_number(val)
                .unwrap_or_else(|e| panic!("Failed call_number for 5-trit cell value {val}: {e}"));
            assert_eq!(resp.scalar, val);
        }
    }

    #[test]
    fn test_call_response_number_powers_of_3() {
        // Test powers of 3 from 3^0 up to 3^31 and negative powers
        let mut p: i64 = 1;
        for exponent in 0..=31 {
            let resp_pos = CallResponseEngine::call_number(p)
                .unwrap_or_else(|e| panic!("Failed call_number for 3^{exponent} = {p}: {e}"));
            assert_eq!(resp_pos.scalar, p);

            let resp_neg = CallResponseEngine::call_number(-p)
                .unwrap_or_else(|e| panic!("Failed call_number for -(3^{exponent}) = -{p}: {e}"));
            assert_eq!(resp_neg.scalar, -p);

            if exponent < 31 {
                p *= 3;
            }
        }
    }

    #[test]
    fn test_call_response_number_powers_of_2() {
        // Test boundary powers of 2 fitting within 32 balanced trits (max (3^32-1)/2 ~= 9.26e14, so 2^49)
        for bit in 0..=49 {
            let v = 1i64 << bit;
            let resp_pos = CallResponseEngine::call_number(v)
                .unwrap_or_else(|e| panic!("Failed call_number for 2^{bit} = {v}: {e}"));
            assert_eq!(resp_pos.scalar, v);

            let resp_neg = CallResponseEngine::call_number(-v)
                .unwrap_or_else(|e| panic!("Failed call_number for -(2^{bit}) = -{v}: {e}"));
            assert_eq!(resp_neg.scalar, -v);
        }

        // Verify that 2^50 exceeds 32-trit single-word capacity and returns an error
        let v50 = 1i64 << 50;
        assert!(CallResponseEngine::call_number(v50).is_err(), "2^50 must exceed 32-trit single word");
    }

    #[test]
    fn test_call_response_number_all_256_bytes_dynamic_topologies() {
        // Test all 256 byte values across all 7 kinematic topologies
        let modes = [
            PackMode::Dense2222,
            PackMode::Kinematic21212,
            PackMode::Kinematic12122,
            PackMode::Kinematic22121,
            PackMode::Directional332,
            PackMode::Directional323,
            PackMode::Directional233,
        ];

        for byte_val in 0u8..=255u8 {
            for &mode in &modes {
                if let Ok(unpacked_trits) = Word32Trit::unpack_byte(byte_val, mode) {
                    // Repack and ensure round trip
                    let repacked = Word32Trit::pack_byte(&unpacked_trits, mode);
                    let unpacked_again = Word32Trit::unpack_byte(repacked, mode)
                        .expect("Repacked byte must unpack cleanly");
                    assert_eq!(unpacked_again[0], unpacked_trits[0]);
                    assert_eq!(unpacked_again[1], unpacked_trits[1]);
                    assert_eq!(unpacked_again[2], unpacked_trits[2]);
                    if mode == PackMode::Dense2222 {
                        assert_eq!(unpacked_again[3], unpacked_trits[3]);
                    }
                }
            }
        }
    }

    #[test]
    fn test_call_response_number_null_collision_trap() {
        // Inject collision code 11 (3) into dual rail
        let corrupted_register: u64 = 0b11;
        let res = CallResponseEngine::call_dual_rail(corrupted_register);
        assert!(res.is_err(), "Collision trap must reject code 11");
    }

    #[test]
    fn test_call_response_reversibility_and_stream() {
        // Test caller (operator) stream call
        let stream = "OMNICODE 2026";
        let resp = CallResponseEngine::call_stream(stream).expect("Stream must succeed");
        assert_eq!(resp.echoed, stream);
        assert_eq!(resp.text, stream);
        assert_eq!(resp.cell_count, 13);
        assert_eq!(resp.state, QuadState::Proceed);

        // Test universal execute dispatch
        let r1 = CallResponseEngine::execute(Call::Character('Z')).unwrap();
        assert!(matches!(r1, Response::Character(_)));

        let r2 = CallResponseEngine::execute(Call::Number(4096)).unwrap();
        assert!(matches!(r2, Response::Number(_)));
    }
}

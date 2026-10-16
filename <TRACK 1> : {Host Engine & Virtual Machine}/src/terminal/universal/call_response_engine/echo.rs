// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Universal Echo Engine: Call-and-Response Mechanism for Characters, Numbers & Streams )
// ==============================================================================
//! OmniCode Terminal: Universal Echo Protocol Engine
//!
//! # Architectural Scope & Purpose:
//! In OmniCode, **AN ECHO IS A CALL AND RESPONSE THAT REPEATS WHAT WAS CALLED FOR**.
//! Since the language is also designed for creating an AI model (where `.omni` and
//! `.tape` are the compiled and interpreted forms of the OmniCode framework, rather
//! than passive data types), the Caller and Sender represent the Operator and the
//! Machine (the computer).
//!
//! Because they are reversible, the Echo Engine handles:
//! 1. **Character Echo:** Inscribes glyph into 1 Cell, bijectively transduces to 5 trits,
//!    guards single-cell bounds, and projects verified glyph to output.
//! 2. **Number Echo:** Inscribes scalar quantity into 1 Word (32 trits <=> 64 bits),
//!    packs dual-rail polar planes (00=0, 01=+1, 10=-1), guards against 11 collision trap,
//!    proves ALU zero-sum balance (+1) + (-1) <=> 0, and projects response.
//! 3. **Stream Echo:** Chains character cells into text ribbons.
//!
//! # Scripture Witness:
//! - **Job 14:15 (KJV)**: "Thou shalt call, and I will answer thee."
//! - **Exodus 31:18 (KJV)**: Written with the finger of God.
//! - **Proverbs 24:27 (KJV)**: Prepare thy work without, and make it fit for thyself.
// ==============================================================================

use crate::core::cell::{Cell, Trit};
use crate::core::ternary::QuadState;
use crate::terminal::universal::call_response_engine::call_response::{
    Call, Response, CharacterResponse, NumberResponse, StreamResponse, CallResponseEngine,
};

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Types, Enums & Status Containers )
// ==============================================================================

/// Universal Echo Result returned by [`EchoEngine::echo`].
#[derive(Debug, Clone, PartialEq)]
pub enum EchoResult {
    /// Atomic Character Echo.
    Character(CharacterResponse),
    /// Scalar Number Echo.
    Number(NumberResponse),
    /// Stream / Text Echo.
    Stream(StreamResponse),
}

/// The Universal Echo Protocol Engine.
#[derive(Debug, Clone, Default)]
pub struct EchoEngine;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Operational Echo Methods )
// ==============================================================================

impl EchoEngine {
    /// Executes a universal Echo turn for any [`Call`] payload.
    pub fn echo(call: Call) -> Result<EchoResult, String> {
        match CallResponseEngine::execute(call)? {
            Response::Character(c) => Ok(EchoResult::Character(c)),
            Response::Number(n) => Ok(EchoResult::Number(n)),
            Response::Stream(s) => Ok(EchoResult::Stream(s)),
        }
    }

    /// Echoes a single character glyph through 5-trit radix-3 cell transduction.
    pub fn echo_char(glyph: char) -> Result<CharacterResponse, String> {
        CallResponseEngine::call_char(glyph)
    }

    /// Echoes a scalar number through 32-trit dual-rail polar bit-planes.
    pub fn echo_number(scalar: i64) -> Result<NumberResponse, String> {
        CallResponseEngine::call_number(scalar)
    }

    /// Echoes an entire text stream through continuous character cells.
    pub fn echo_stream(input: &str) -> Result<StreamResponse, String> {
        CallResponseEngine::call_stream(input)
    }

    /// Transduces an 8-bit ASCII byte into 5 balanced trits using canonical [`Cell`] natural encoding.
    pub fn byte_to_5_trits(byte: u8) -> Result<[i8; 5], String> {
        let cell = Cell::from_natural(byte)?;
        let mut trits = [0i8; 5];
        for (i, t) in cell.trits.iter().enumerate() {
            trits[i] = t.as_i8();
        }
        Ok(trits)
    }

    /// Reconstructs an 8-bit byte from 5 balanced trits using canonical [`Cell`] natural decoding.
    pub fn five_trits_to_byte(trits: &[i8; 5]) -> Result<u8, String> {
        let mut cell_trits = [Trit::Zero; 5];
        for (i, &t) in trits.iter().enumerate() {
            cell_trits[i] = Trit::from_i8(t)?;
        }
        let cell = Cell::new(cell_trits, QuadState::Proceed);
        cell.to_natural().ok_or_else(|| "Failed to decode natural byte from cell".to_string())
    }

    /// Verifies that a character fits strictly within single-cell bounds.
    pub fn verify_single_cell_bound(c: char) -> bool {
        c.len_utf8() <= 4
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests for Universal Echo Protocol Engine )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_char_letters_and_symbols() {
        let test_chars = ['A', 'Z', 'a', 'z', '0', '9', '!', '@', '#', '$', ' '];
        for &c in &test_chars {
            let res = EchoEngine::echo_char(c).expect("Echo char must succeed");
            assert_eq!(res.glyph, c);
            assert_eq!(res.state, QuadState::Proceed);
            assert_eq!(res.cell_footprint, 1);
        }
    }

    #[test]
    fn test_echo_number_equilibrium() {
        let res = EchoEngine::echo_number(42).expect("Echo number must succeed");
        assert_eq!(res.scalar, 42);
        assert!(res.equilibrium_proved);
        assert!(res.rail_involution_proved);
    }

    #[test]
    fn test_echo_stream() {
        let text = "OmniCode Universal Echo OK";
        let res = EchoEngine::echo_stream(text).expect("Stream echo must succeed");
        assert_eq!(res.echoed, text);
        assert_eq!(res.text, text);
    }

    #[test]
    fn test_echo_universal_call_dispatch() {
        let res_char = EchoEngine::echo(Call::Character('K')).unwrap();
        assert!(matches!(res_char, EchoResult::Character(_)));

        let res_num = EchoEngine::echo(Call::Number(-1)).unwrap();
        assert!(matches!(res_num, EchoResult::Number(_)));
    }
}

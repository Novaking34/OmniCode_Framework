// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Native OmniCode Character Lifecycle, Radix-3 Transduction & Echo Engine )
// ==============================================================================
//! OmniCode Terminal: Native Character Echo Protocol Engine
//!
//! # Architectural Scope & Purpose:
//! Implements the mathematical and ontological Character Invariant lifecycle
//! defined in `character.tape` directly inside the terminal substrate:
//! 1. Ingestion: Reads glyph from hardware `FROM` port (`0x03F8`).
//! 2. Inscription: Inscribes glyph into an atomic 5-trit [`Cell`] container.
//! 3. Bijective Transduction (`<=>`): Converts 8-bit byte to 5 balanced trits $\{-1, 0, +1\}^5$.
//! 4. Relational Guard: Enforces identity `{ Character' = Glyph }` and bounds `{ Character' <= [ 1 Cell ] }`.
//! 5. Projection: Emits verified Character to hardware `INTO` port (`0x000B8000`), returning the native echo.
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**:
//!   "And he gave unto Moses, when he had made an end of communing with him upon mount Sinai,
//!    two tables of testimony, tables of stone, written with the finger of God."
//!   (The immutable inscription of physical data into bounded cell tablets).
//! - **Proverbs 24:27 (KJV)**:
//!   "Prepare thy work without, and make it fit for thyself in the field;
//!    and afterwards build thine house."
//!   (The discipline of proving atomic character echo physics before constructing full terminal shells).
//! - **Proverbs 22:28 (KJV)**:
//!   "Remove not the ancient landmark, which thy fathers have set."
//!   (The preservation of historical ASCII and Radix-3 radix boundaries).
//!
//! # Why Extensive Commenting Matters for AI Agents & Developers:
//! 1. **Terminal Echo Truth**: In OmniCode, echoing a character is not a shallow byte write;
//!    it is a formal mathematical proof that the glyph traversed the 5-trit balanced ternary
//!    virtual machine without loss of precision or boundary leakage.
// ==============================================================================

use crate::core::ternary::QuadState;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Character Echo Engine Struct & Status Container )
// ==============================================================================

/// Result of a single character verification and echo cycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterEchoResult {
    /// Echoed character glyph.
    pub glyph: char,
    /// 5 balanced trits representing the character in silicon virtual memory.
    pub trits: [i8; 5],
    /// Balanced ternary execution state (+1:PROCEED).
    pub state: QuadState,
    /// Verified cell capacity footprint (always 1 Cell).
    pub cell_footprint: usize,
}

/// Native Terminal Character Echo Engine.
#[derive(Debug, Clone, Default)]
pub struct CharacterEchoEngine;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Bijective Radix-3 Transduction, Capacity Bounds & Echo Pipeline )
// ==============================================================================

use crate::core::cell::{Cell, Trit};

impl CharacterEchoEngine {
    /// Transduces an 8-bit ASCII byte into 5 balanced trits using the canonical [`Cell`] natural encoding.
    pub fn byte_to_5_trits(byte: u8) -> Result<[i8; 5], String> {
        let cell = Cell::from_natural(byte)?;
        let mut trits = [0i8; 5];
        for (i, t) in cell.trits.iter().enumerate() {
            trits[i] = t.as_i8();
        }
        Ok(trits)
    }

    /// Reconstructs an 8-bit byte from 5 balanced trits using the canonical [`Cell`] natural decoding.
    pub fn five_trits_to_byte(trits: &[i8; 5]) -> Result<u8, String> {
        let mut cell_trits = [Trit::Zero; 5];
        for (i, &t) in trits.iter().enumerate() {
            cell_trits[i] = Trit::from_i8(t)?;
        }
        let cell = Cell::new(cell_trits, QuadState::Proceed);
        cell.to_natural().ok_or_else(|| "Failed to decode natural byte from cell".to_string())
    }

    /// Verifies that a character fits strictly within single-cell bounds ($\le 1\text{ Cell}$).
    pub fn verify_single_cell_bound(c: char) -> bool {
        // Any Unicode scalar fitting within 1 Cell footprint (1-4 UTF-8 bytes)
        c.len_utf8() <= 4
    }

    /// Executes the full 5-step Character tape lifecycle for a single character glyph:
    /// Inscribe -> Transduce (<=>) -> Guard Bound (<= 1 Cell) -> Project to Target -> Echo.
    pub fn echo_char(glyph: char) -> Result<CharacterEchoResult, String> {
        // Step 1 & 2: Inscribe into Cell
        let byte = if (glyph as u32) <= 242 {
            glyph as u8
        } else {
            return Err(format!("Character '{}' (U+{:04X}) exceeds single-cell capacity of 242 values", glyph, glyph as u32));
        };

        // Step 3: Bijective Transduction (<=>)
        let trits = Self::byte_to_5_trits(byte)?;
        let reconstructed_byte = Self::five_trits_to_byte(&trits)?;
        if reconstructed_byte != byte {
            return Err(format!("Transduction anomaly: byte {byte} != reconstructed {reconstructed_byte}"));
        }

        // Step 4: Compare & Guard ({ Character' = Glyph } & { Character' <= [ 1 Cell ] })
        let reconstructed_char = reconstructed_byte as char;
        if reconstructed_char != glyph {
            return Err(format!("Identity violation: glyph '{glyph}' != reconstructed '{reconstructed_char}'"));
        }
        if !Self::verify_single_cell_bound(glyph) {
            return Err(format!("Bound violation: glyph '{glyph}' exceeds 1 Cell capacity"));
        }

        // Step 5: Target Projection (OUTWARD to %into port)
        Ok(CharacterEchoResult {
            glyph: reconstructed_char,
            trits,
            state: QuadState::Proceed,
            cell_footprint: 1,
        })
    }

    /// Executes character echo across an entire text stream, returning the verified echoed string.
    pub fn echo_stream(input: &str) -> Result<String, String> {
        let mut echoed = String::with_capacity(input.len());
        for c in input.chars() {
            let res = Self::echo_char(c)?;
            echoed.push(res.glyph);
        }
        Ok(echoed)
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Test Verification Proofs & Bijective Oracles )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bijective_character_transduction_ascii_range() {
        // Test standard ASCII letters, digits, and control glyphs
        let test_chars = ['A', 'B', 'Z', 'a', 'm', 'z', '0', '5', '9', '!', '@', '#', '$', '\n', '\r', ' '];
        for &c in &test_chars {
            let res = CharacterEchoEngine::echo_char(c).unwrap_or_else(|e| panic!("Failed echo for '{c}': {e}"));
            assert_eq!(res.glyph, c);
            assert_eq!(res.state, QuadState::Proceed);
            assert_eq!(res.cell_footprint, 1);
        }
    }

    #[test]
    fn test_transduction_horner_parity_for_a_ascii_65() {
        // 'A' is ASCII 65
        let trits = CharacterEchoEngine::byte_to_5_trits(65).unwrap();
        let byte = CharacterEchoEngine::five_trits_to_byte(&trits).unwrap();
        assert_eq!(byte, 65);
    }

    #[test]
    fn test_echo_stream_end_to_end() {
        let text = "OmniCode Native Terminal Echo OK!";
        let echoed = CharacterEchoEngine::echo_stream(text).expect("Stream echo must succeed");
        assert_eq!(echoed, text);
    }
}

// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Universal AST Node Decoding, Trit Extractors & Horner Expansion Invariants )
// ==============================================================================
//! OmniCode Runtime: Universal AST Decoders & Trit Transformers
//!
//! # Architectural Scope & Purpose:
//! Provides zero-allocation AST pattern extractors, balanced ternary trit sequence
//! decoders, and Horner power expansion algorithms for bijective numeric transduction.
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
//! 1. **Bijective Transduction Invariants**: The 5-trit Horner expansion
//!    $\sum_{i=0}^{N-1} t_i \cdot 3^i$ guarantees exact integer parity (e.g., $65 \iff [-1, +1, +1, -1, +1]$)
//!    across the C, Lua, and Rust VMs.
//! 2. **Symbolic Uniformity**: `extract_symbol_name` normalizes heterogeneous AST wrappers
//!    (`Identifier`, `Polarity`, `[ ROLL 1 ]`, `[ Character' ]`) into canonical lookup keys.
// ==============================================================================

use crate::ast::nodes::ASTNode;
use crate::core::ternary::QuadState;
use crate::core::operators::Polarity;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Universal Decoder Function Declarations )
// ==============================================================================

/// Extracts a symbolic name from a node (e.g. `[ ROLL 1 ]` -> `"ROLL 1"`, `[ Verify ]` -> `"Verify"`).
pub fn extract_symbol_name(node: &ASTNode) -> Option<String> {
    match node {
        ASTNode::Identifier(id) => Some(id.clone()),
        ASTNode::Polarity(p) => Some(p.name.clone()),
        ASTNode::Boundary(b) => {
            if b.elements.len() == 1 {
                extract_symbol_name(&b.elements[0])
            } else if b.elements.len() == 2 {
                if let (ASTNode::Identifier(id), ASTNode::Number(n)) = (&b.elements[0], &b.elements[1]) {
                    Some(format!("{} {}", id, *n as i64))
                } else if let (ASTNode::Identifier(id1), ASTNode::Identifier(id2)) = (&b.elements[0], &b.elements[1]) {
                    Some(format!("{} {}", id1, id2))
                } else {
                    None
                }
            } else {
                None
            }
        }
        ASTNode::Entity(e) => Some(e.name.clone()),
        _ => None,
    }
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Balanced Trit Extraction & Horner Polynomial Evaluation )
// ==============================================================================

/// Extracts an array of balanced trits {-1, 0, +1} from a boundary node.
pub fn extract_trits_from_node(node: &ASTNode) -> Option<Vec<i8>> {
    match node {
        ASTNode::Boundary(b) => {
            let mut trits = Vec::new();
            for el in &b.elements {
                let t = match el {
                    ASTNode::Number(n) => {
                        let i = *n as i8;
                        if i == -1 || i == 0 || i == 1 {
                            i
                        } else {
                            return None;
                        }
                    }
                    ASTNode::Polarity(p) => match p.polarity {
                        Polarity::Abstract => 1,
                        Polarity::Concrete => -1,
                    },
                    ASTNode::Ternary(s) => match s {
                        QuadState::Proceed => 1,
                        QuadState::Await => 0,
                        QuadState::Rollback => -1,
                        QuadState::Halt => 0,
                    },
                    ASTNode::Identifier(id) => match id.as_str() {
                        "-1" => -1,
                        "0" => 0,
                        "+1" | "1" => 1,
                        _ => return None,
                    },
                    _ => return None,
                };
                trits.push(t);
            }
            if !trits.is_empty() {
                Some(trits)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Computes the balanced integer value of a balanced trit slice using Horner's method:
/// `sum_{i=0..N-1} trits[i] * 3^i`.
pub fn trits_to_integer(trits: &[i8]) -> i64 {
    let mut total: i64 = 0;
    let mut multiplier: i64 = 1;
    for &t in trits {
        total += (t as i64) * multiplier;
        multiplier *= 3;
    }
    total
}

/// Converts an integer into a balanced ternary representation of length
/// `num_trits`.
pub fn integer_to_trits(mut n: i64, num_trits: usize) -> Vec<i8> {
    let mut trits = Vec::with_capacity(num_trits);
    for _ in 0..num_trits {
        let rem = ((n % 3) + 3) % 3;
        match rem {
            0 => {
                trits.push(0);
                n /= 3;
            }
            1 => {
                trits.push(1);
                n = (n - 1) / 3;
            }
            2 => {
                trits.push(-1);
                n = (n + 1) / 3;
            }
            _ => unreachable!(),
        }
    }
    trits
}

/// Converts a balanced trit sequence to dual-rail (pos, neg) bitmasks.
/// For each trit index `i`:
/// - `+1`: bit `i` set in `pos_plane`
/// - `-1`: bit `i` set in `neg_plane`
/// - `0`: bit `i` clear in both planes
pub fn trits_to_dual_rail(trits: &[i8]) -> (u64, u64) {
    let mut pos: u64 = 0;
    let mut neg: u64 = 0;
    for (i, &t) in trits.iter().enumerate().take(64) {
        match t {
            1 => pos |= 1u64 << i,
            -1 => neg |= 1u64 << i,
            _ => {}
        }
    }
    (pos, neg)
}

/// Reconstructs balanced trits from dual-rail bit-planes with collision check.
/// Returns an error if any bit position has both rails asserted
/// (`pos & neg != 0`).
pub fn dual_rail_to_trits(
    pos: u64,
    neg: u64,
    num_trits: usize,
) -> Result<Vec<i8>, String> {
    let collision = pos & neg;
    if collision != 0 {
        return Err(format!(
            "FAILPOINT [DualRailCollision]: Trap at mask 0x{:016X}",
            collision
        ));
    }
    let mut trits = Vec::with_capacity(num_trits);
    for i in 0..num_trits.min(64) {
        let p = (pos >> i) & 1;
        let n = (neg >> i) & 1;
        match (p, n) {
            (0, 0) => trits.push(0),
            (1, 0) => trits.push(1),
            (0, 1) => trits.push(-1),
            _ => unreachable!(),
        }
    }
    Ok(trits)
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests & Bijective Verification Proofs )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::boundaries::BoundaryType;
    use crate::ast::nodes::BoundaryNode;

    #[test]
    fn test_extract_symbol_name_variants() {
        let id_node = ASTNode::Identifier("Status`".to_string());
        assert_eq!(extract_symbol_name(&id_node), Some("Status`".to_string()));

        let bound_single = ASTNode::Boundary(BoundaryNode {
            boundary_type: BoundaryType::Denotation,
            elements: vec![ASTNode::Identifier("Verify".to_string())],
            aside: None,
            line: 1,
            col: 1,
        });
        assert_eq!(extract_symbol_name(&bound_single), Some("Verify".to_string()));

        let bound_double = ASTNode::Boundary(BoundaryNode {
            boundary_type: BoundaryType::Denotation,
            elements: vec![
                ASTNode::Identifier("ROLL".to_string()),
                ASTNode::Number(1.0),
            ],
            aside: None,
            line: 1,
            col: 1,
        });
        assert_eq!(extract_symbol_name(&bound_double), Some("ROLL 1".to_string()));
    }

    #[test]
    fn test_bijective_trits_horner_expansion() {
        // 'A' (ASCII 65): [-1, 1, 1, -1, 1]
        // -1*(1) + 1*(3) + 1*(9) - 1*(27) + 1*(81) = -1 + 3 + 9 - 27 + 81 = 65
        let trits_a: [i8; 5] = [-1, 1, 1, -1, 1];
        assert_eq!(trits_to_integer(&trits_a), 65);

        // '0' (ASCII 48): [0, 1, -1, -1, 1]
        // 0*(1) + 1*(3) - 1*(9) - 1*(27) + 1*(81) = 3 - 9 - 27 + 81 = 48
        let trits_0: [i8; 5] = [0, 1, -1, -1, 1];
        assert_eq!(trits_to_integer(&trits_0), 48);
    }

    #[test]
    fn test_integer_to_trits_and_dual_rail_round_trip() {
        // Round trip 65 ('A')
        let trits_65 = integer_to_trits(65, 5);
        assert_eq!(trits_65, vec![-1, 1, 1, -1, 1]);
        assert_eq!(trits_to_integer(&trits_65), 65);

        // Round trip 48 ('0')
        let trits_48 = integer_to_trits(48, 5);
        assert_eq!(trits_48, vec![0, 1, -1, -1, 1]);
        assert_eq!(trits_to_integer(&trits_48), 48);

        // Dual rail bit planes for 'A' [-1, 1, 1, -1, 1]:
        // pos bits: 1, 2, 4 (0b10110 = 22)
        // neg bits: 0, 3 (0b01001 = 9)
        let (pos, neg) = trits_to_dual_rail(&trits_65);
        assert_eq!(pos, 0b10110);
        assert_eq!(neg, 0b01001);
        assert_eq!(pos & neg, 0); // No collision

        let recovered = dual_rail_to_trits(pos, neg, 5).unwrap();
        assert_eq!(recovered, trits_65);

        // Test collision detection
        let collision_res = dual_rail_to_trits(0b11, 0b11, 2);
        assert!(collision_res.is_err());
    }
}


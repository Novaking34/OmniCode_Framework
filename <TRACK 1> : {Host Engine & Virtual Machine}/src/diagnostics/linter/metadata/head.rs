// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Process Tape (.tape) Block 1 Tape Head & Cardinal Coordinates Linter )
// ==============================================================================
//! Process Tape (.tape) Block 1 Tape Head Linter
//!
//! Evaluates and scores the Tape Head manifest for `.tape` Process Ribbons.
//! While `.omni` files represent declarative Source Reels, `.tape` files represent
//! sequential execution ribbons. Both embody the exact same ontological covenant
//! and foundation, but with process ribbon mechanics:
//! 1. Formal Tape Head Declaration (`= [Tape]: < ... > & [ ... ] {`).
//! 2. Defined Operational Space & Partitions (`WHERE { ... Cell } = [ 1 ]`).
//! 3. Covenant Witness Grounding (`< Scripture { ... } >` anchor).
//! 4. The Six Cardinal Coordinates (`WHO`, `WHAT`, `WHEN`, `WHERE`, `WHY`, `HOW`)
//!    with operational verbs (`--> INSCRIBE`) and causal directionals.
//! 5. Spatial Geometry (<= 80 columns) and bracket naming hygiene.

use crate::diagnostics::linter::syntax::SpatialLintConfig;
use super::score::MetadataScore;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( TapeHeadLinter Definition & Slice Extractor )
// ==============================================================================

pub struct TapeHeadLinter;

impl TapeHeadLinter {
    /// Extracts the text slice corresponding exclusively to the Tape Head.
    pub fn extract_slice(content: &str) -> (usize, usize, String) {
        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return (0, 0, String::new());
        }

        let mut start_idx = None;
        let mut end_idx = None;

        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if start_idx.is_none() {
                if trimmed.starts_with("= [Tape")
                    || trimmed.starts_with("[Tape.")
                    || trimmed.starts_with("Tape [")
                    || trimmed.starts_with("Tape <")
                {
                    start_idx = Some(idx);
                }
            } else {
                let upper = trimmed.to_uppercase();
                if upper.starts_with("[ ROLL")
                    || upper.starts_with("ROLL_")
                    || trimmed.starts_with("# DECLARED ROLLS")
                {
                    end_idx = Some(idx);
                    break;
                }
            }
        }

        let start = start_idx.unwrap_or(0);
        let end = end_idx.unwrap_or(lines.len());
        (start + 1, end, lines[start..end].join("\n"))
    }
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Evaluation Engine for Process Tape Head Invariants )
// ==============================================================================

impl TapeHeadLinter {
    /// Evaluates and scores Block 1 for a `.tape` Process Ribbon.
    pub fn evaluate(
        _file_path: &str,
        slice: &str,
        config: &SpatialLintConfig,
    ) -> MetadataScore {
        let mut identity = 0u32;
        let mut provenance = 0u32;
        let mut witness = 0u32;
        let mut ontology = 0u32;
        let mut spatial = 0u32;
        let mut breakdown = Vec::new();

        // ----------------------------------------------------------------------
        // 1. Identity & Formal Tape Declaration (max 20 pts)
        // ----------------------------------------------------------------------
        let has_tape_decl = slice.contains("= [Tape]:")
            || slice.contains("[Tape.")
            || slice.contains("Tape [")
            || slice.contains("Tape <");
        if has_tape_decl {
            identity += 15;
            breakdown.push("✓ Formal Process Tape Head declared (+15/15)".into());
        } else {
            breakdown.push("✗ Missing formal tape head declaration (0/15)".into());
        }

        let has_tape_context = (slice.contains('<') && slice.contains('>'))
            && (slice.contains('[') && slice.contains(']'));
        if has_tape_context {
            identity += 5;
            breakdown.push("✓ Dual-domain tape contextualization affirmed (+5/5)".into());
        } else {
            breakdown.push("✗ Missing dual domain context in tape declaration (0/5)".into());
        }

        // ----------------------------------------------------------------------
        // 2. Operational Environment & Topology (max 25 pts)
        // ----------------------------------------------------------------------
        let has_defined_space = slice.contains("WHERE")
            && (slice.contains("Cell") || slice.contains("Partition") || slice.contains('['));
        if has_defined_space {
            provenance += 10;
            breakdown.push("✓ Defined space / cell partition bounds declared (+10/10)".into());
        } else {
            breakdown.push("✗ Missing defined space or cell bounds in tape head (0/10)".into());
        }

        let has_domain_context = slice.contains("Lexicon") || slice.contains("Kernel") || slice.contains("Domain");
        if has_domain_context {
            provenance += 10;
            breakdown.push("✓ Execution domain context declared (+10/10)".into());
        } else {
            breakdown.push("✗ Missing execution domain context (0/10)".into());
        }

        let has_comments = slice.contains("TOPOLOGY")
            || slice.contains("PROCESS TAPE")
            || slice.lines().any(|l| l.trim().starts_with('#'));
        if has_comments {
            provenance += 5;
            breakdown.push("✓ Architectural topology commentary inscribed (+5/5)".into());
        } else {
            breakdown.push("✗ Missing architectural topology header commentary (0/5)".into());
        }

        // ----------------------------------------------------------------------
        // 3. Covenant & Witness Invariant Grounding (max 20 pts)
        // ----------------------------------------------------------------------
        let has_scripture = slice.contains("< Scripture") || slice.contains("Exodus");
        if has_scripture {
            witness += 15;
            breakdown.push("✓ Invariant Scripture / Covenant witness anchored (+15/15)".into());
        } else {
            breakdown.push("✗ Missing scripture/covenant witness in tape head (0/15)".into());
        }

        let has_witness_aside = slice.contains("Invariant Witness")
            || (has_scripture && slice.contains('(') && slice.contains(')'));
        if has_witness_aside {
            witness += 5;
            breakdown.push("✓ Machine-readable Witness Aside oracle attached (+5/5)".into());
        } else {
            breakdown.push("✗ Missing witness Aside oracle (0/5)".into());
        }

        // ----------------------------------------------------------------------
        // 4. Six Cardinal Coordinates & Operational Verbs (max 20 pts)
        // ----------------------------------------------------------------------
        let upper = slice.to_uppercase();
        let cardinal_clauses = ["WHO", "WHAT", "WHEN", "WHERE", "WHY", "HOW"];
        let present_clauses = cardinal_clauses.iter().filter(|&&c| upper.contains(c)).count();

        if present_clauses == 6 {
            ontology += 15;
            breakdown.push(
                "✓ All Six Cardinal Coordinates (WHO, WHAT, WHEN, WHERE, WHY, HOW) present (+15/15)".into(),
            );
        } else {
            let pts = (present_clauses as u32) * 2;
            ontology += pts;
            breakdown.push(format!(
                "⚠ Cardinal coordinates incomplete ({present_clauses}/6) (+{pts}/15)"
            ));
        }

        let has_op_verbs = upper.contains("INSCRIBE") || upper.contains("PROCEED") || upper.contains("-->");
        if has_op_verbs {
            ontology += 5;
            breakdown.push("✓ Operational SVO verb and directional vectors wired (+5/5)".into());
        } else {
            breakdown.push("✗ Missing operational verbs or directional vectors in cardinal clauses (0/5)".into());
        }

        // ----------------------------------------------------------------------
        // 5. Spatial Geometry & Syntax Hygiene (max 15 pts)
        // ----------------------------------------------------------------------
        let lines: Vec<&str> = slice.lines().collect();
        let max_cols = config.max_columns;
        let line_len_violation = lines.iter().any(|l| l.chars().count() > max_cols);

        if !line_len_violation {
            spatial += 10;
            breakdown.push(format!("✓ All Tape Head lines <= {max_cols} columns (+10/10)"));
        } else {
            breakdown.push(format!("✗ Tape Head lines exceed {max_cols}-column spatial ruler (0/10)"));
        }

        let snake_violation = lines.iter().any(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with('#') {
                return false;
            }
            if let Some(start) = line.find('[') {
                if let Some(end) = line[start..].find(']') {
                    let inside = &line[start + 1..start + end];
                    for w in inside.split_whitespace() {
                        if w.contains('_') && !w.starts_with("0x") && !w.starts_with("roll_") {
                            return true;
                        }
                    }
                }
            }
            false
        });

        if !snake_violation {
            spatial += 5;
            breakdown.push("✓ Natural spaced identifier hygiene observed (+5/5)".into());
        } else {
            breakdown.push("✗ snake_case identifiers found inside brackets (0/5)".into());
        }

        MetadataScore::new(identity, provenance, witness, ontology, spatial, breakdown)
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests for Tape Head Block 1 Linting )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tape_head_extraction_and_empty_check() {
        let (start, end, slice) = TapeHeadLinter::extract_slice("");
        assert_eq!(start, 0);
        assert_eq!(end, 0);
        assert!(slice.is_empty());
    }
}

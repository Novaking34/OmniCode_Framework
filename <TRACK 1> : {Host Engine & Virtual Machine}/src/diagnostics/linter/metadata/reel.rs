// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Source Reel (.omni) Block 1 Metadata & Invariant Type Linter )
// ==============================================================================
//! Source Reel (.omni) Block 1 Metadata Linter
//!
//! Evaluates and scores the ontological foundation of `.omni` files:
//! 1. Formal Module Identity (`= [Module]: < ... > & [ ... ]` with intent Aside).
//! 2. Invariant Provenance (`Version`, `Author`, `Created`, `Updated`, `System`).
//! 3. Covenant Witness Grounding (`< Scripture { ... } >` or `[ Witness ]:`).
//! 4. Ontological Invariants & Freestanding Type Definitions (`< Type > = ...`
//!    across the 9 dimensions: Relation, Domain, Substrate, Inscribe, Transduce,
//!    Exemplar, Compare, Sequence, Bound).
//! 5. Spatial Geometry (<= 80 columns) and bracket identifier naming hygiene.

use crate::diagnostics::linter::syntax::SpatialLintConfig;
use super::score::MetadataScore;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( ReelMetadataLinter Definition & Slice Extractor )
// ==============================================================================

pub struct ReelMetadataLinter;

impl ReelMetadataLinter {
    /// Extracts the text slice corresponding to Block 1 in a `.omni` file.
    pub fn extract_slice(content: &str) -> (usize, usize, String) {
        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return (0, 0, String::new());
        }

        let mut start_idx = None;
        let mut end_idx = None;

        for (idx, line) in lines.iter().enumerate() {
            let upper = line.to_uppercase();
            if start_idx.is_none() {
                if upper.contains("BLOCK 1") || upper.contains("METADATA BLOCK") {
                    start_idx = Some(idx);
                }
            } else if upper.contains("BLOCK 2") || upper.contains("OPENING BLOCK") {
                end_idx = Some(idx);
                break;
            }
        }

        let start = start_idx.unwrap_or(0);
        let end = end_idx.unwrap_or(lines.len());
        (start + 1, end, lines[start..end].join("\n"))
    }
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Evaluation Engine for Source Reel Metadata Invariants )
// ==============================================================================

impl ReelMetadataLinter {
    /// Evaluates and scores Block 1 for a `.omni` Source Reel.
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
        // 1. Identity & Formal Declaration (max 20 pts)
        // ----------------------------------------------------------------------
        let has_module_decl = slice.contains("= [Module]:")
            || slice.contains("Module <")
            || slice.contains("Module [")
            || slice.contains("Scroll <");
        if has_module_decl {
            identity += 15;
            breakdown.push("✓ Formal Module Identity declared (+15/15)".into());
        } else {
            breakdown.push("✗ Missing formal module identity declaration (0/15)".into());
        }

        let has_identity_aside = slice.lines().any(|l| {
            (l.contains("= [Module]:") || l.contains("Module <")) && l.contains('(')
                || (slice.contains("= [Module]:") && slice.contains("( This is the Module"))
        }) || (has_module_decl && slice.contains('(') && slice.contains(')'));

        if has_identity_aside {
            identity += 5;
            breakdown.push("✓ Module intent Aside '( ... )' attached (+5/5)".into());
        } else {
            breakdown.push("✗ Missing intent Aside on module identity (0/5)".into());
        }

        // ----------------------------------------------------------------------
        // 2. Provenance Attributes & Environment (max 25 pts)
        // ----------------------------------------------------------------------
        let has_version = slice.contains("[ Version ]") || slice.contains("version:");
        if has_version {
            provenance += 8;
            breakdown.push("✓ Invariant Version attribute specified (+8/8)".into());
        } else {
            breakdown.push("✗ Missing version attribute in metadata context (0/8)".into());
        }

        let has_author = slice.contains("[ Author ]") || slice.contains("author:");
        if has_author {
            provenance += 6;
            breakdown.push("✓ Author / Organization provenance recorded (+6/6)".into());
        } else {
            breakdown.push("✗ Missing author provenance specification (0/6)".into());
        }

        let has_timestamps = slice.contains("[ Created ]") || slice.contains("[ Updated ]");
        if has_timestamps {
            provenance += 6;
            breakdown.push("✓ Created / Updated temporal anchors inscribed (+6/6)".into());
        } else {
            breakdown.push("✗ Missing temporal created/updated timestamps (0/6)".into());
        }

        let has_system = slice.contains("[ System ]") || slice.contains("system:");
        if has_system {
            provenance += 5;
            breakdown.push("✓ Target system / runtime context declared (+5/5)".into());
        } else {
            breakdown.push("✗ Missing target system context declaration (0/5)".into());
        }

        // ----------------------------------------------------------------------
        // 3. Covenant & Witness Grounding (max 20 pts)
        // ----------------------------------------------------------------------
        let has_scripture = slice.contains("< Scripture") || slice.contains("[ Witness ]");
        if has_scripture {
            witness += 15;
            breakdown.push("✓ Invariant Covenant / Scripture witness anchored (+15/15)".into());
        } else {
            breakdown.push("✗ Missing scripture/covenant witness anchor (0/15)".into());
        }

        let has_witness_aside = slice.contains("Invariant Witness")
            || (has_scripture && slice.contains('(') && slice.contains("Witness"));
        if has_witness_aside {
            witness += 5;
            breakdown.push("✓ Machine-readable Witness Aside oracle attached (+5/5)".into());
        } else if has_scripture {
            witness += 3;
            breakdown.push("⚠ Living Aside on witness partially detected (+3/5)".into());
        } else {
            breakdown.push("✗ Missing witness Aside oracle (0/5)".into());
        }

        // ----------------------------------------------------------------------
        // 4. Ontological Dimensions & Invariant Types (max 20 pts)
        // ----------------------------------------------------------------------
        let has_freestanding_type = slice.contains("< Character > =")
            || (slice.contains("< ") && slice.contains(" > =") && slice.contains('&'));
        let has_nature_semantics = slice.contains("[ Nature ]") || slice.contains("[ Semantics ]");

        if has_freestanding_type {
            let dimensions = [
                "Relation", "Domain", "Substrate", "Inscribe",
                "Transduce", "Exemplar", "Compare", "Sequence", "Bound",
            ];
            let matched_dims = dimensions.iter().filter(|&&d| slice.contains(d)).count();
            if matched_dims >= 7 {
                ontology += 20;
                breakdown.push(format!(
                    "✓ Freestanding Invariant Type defined across {matched_dims}/9 dimensions (+20/20)"
                ));
            } else {
                let pts = (matched_dims as u32) * 2;
                ontology += pts;
                breakdown.push(format!(
                    "⚠ Freestanding Type defined with partial dimensions ({matched_dims}/9) (+{pts}/20)"
                ));
            }
        } else if has_nature_semantics {
            ontology += 20;
            breakdown.push("✓ Nature and Semantics ontology bounds defined (+20/20)".into());
        } else {
            breakdown.push("✗ Missing freestanding invariant type or nature/semantics bounds (0/20)".into());
        }

        // ----------------------------------------------------------------------
        // 5. Spatial Geometry & Syntax Hygiene within Block 1 (max 15 pts)
        // ----------------------------------------------------------------------
        let lines: Vec<&str> = slice.lines().collect();
        let max_cols = config.max_columns;
        let line_len_violation = lines.iter().any(|l| l.chars().count() > max_cols);

        if !line_len_violation {
            spatial += 10;
            breakdown.push(format!("✓ All Block 1 lines <= {max_cols} columns (+10/10)"));
        } else {
            breakdown.push(format!("✗ Block 1 lines exceed {max_cols}-column spatial ruler (0/10)"));
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
// ( Unit Tests for Source Reel Block 1 Linting )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reel_metadata_extraction_and_empty_check() {
        let (start, end, slice) = ReelMetadataLinter::extract_slice("");
        assert_eq!(start, 0);
        assert_eq!(end, 0);
        assert!(slice.is_empty());
    }
}

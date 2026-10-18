// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( OmniCode 4-Block Source Reel (.omni) Structural Linter )
// ==============================================================================
//! OmniCode 4-Block Source Reel Structural Linter
//!
//! Validates canonical 4-Block architecture for `.omni` files:
//! - BLOCK 1: METADATA BLOCK (Identity, Version, Invariant Axioms, Asides)
//! - BLOCK 2: OPENING BLOCK  (Context, Preconditions, State Initialization)
//! - BLOCK 3: BODY BLOCK     (Core Transformations, Causality Vectors, Logic)
//! - BLOCK 4: CLOSING BLOCK  (Verification, Postconditions, Teardown, Commit)
//!
//! # Scripture Witness:
//! - **Ezekiel 43:11 (KJV)**:
//!   "And if they be ashamed of all that they have done, shew them the form of the house,
//!    and the fashion thereof, and the goings out thereof, and the comings in thereof,
//!    and all the forms thereof, and all the ordinances thereof..."
//!   (The 4-Block pattern is the invariant form and fashion of the compilation house).
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( OmniReelLinter Definition & Structural Imports )
// ==============================================================================

use crate::diagnostics::linter::{BlockKind, DetectedBlock, LintReport};

pub struct OmniReelLinter;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Universal 4-Block Sequence & Semantic Scope Verification )
// ==============================================================================

impl OmniReelLinter {
    /// Lints a `.omni` source reel string against universal 4-Block invariants.
    pub fn lint(file_path: &str, content: &str, report: &mut LintReport) {
        let mut blocks = Vec::new();

        for (idx, line) in content.lines().enumerate() {
            let line_num = idx + 1;
            // Ignore indented lines (e.g. sub-reels or inner scopes)
            if line.starts_with(' ') || line.starts_with('\t') {
                continue;
            }
            let upper = line.to_uppercase();

            if upper.contains("BLOCK 1") || upper.contains("METADATA BLOCK") {
                blocks.push(DetectedBlock {
                    kind: BlockKind::Metadata,
                    line_number: line_num,
                    header_raw: line.trim().to_string(),
                });
            } else if upper.contains("BLOCK 2") || upper.contains("OPENING BLOCK") {
                blocks.push(DetectedBlock {
                    kind: BlockKind::Opening,
                    line_number: line_num,
                    header_raw: line.trim().to_string(),
                });
            } else if upper.contains("BLOCK 3") || upper.contains("BODY BLOCK") {
                blocks.push(DetectedBlock {
                    kind: BlockKind::Body,
                    line_number: line_num,
                    header_raw: line.trim().to_string(),
                });
            } else if upper.contains("BLOCK 4") || upper.contains("CLOSING BLOCK") {
                blocks.push(DetectedBlock {
                    kind: BlockKind::Closing,
                    line_number: line_num,
                    header_raw: line.trim().to_string(),
                });
            }
        }

        // Check 1: Must contain all 4 blocks
        let has_metadata = blocks.iter().any(|b| b.kind == BlockKind::Metadata);
        let has_opening = blocks.iter().any(|b| b.kind == BlockKind::Opening);
        let has_body = blocks.iter().any(|b| b.kind == BlockKind::Body);
        let has_closing = blocks.iter().any(|b| b.kind == BlockKind::Closing);

        if !has_metadata {
            report.errors.push("Missing BLOCK 1: METADATA BLOCK".into());
        }
        if !has_opening {
            report.errors.push("Missing BLOCK 2: OPENING BLOCK".into());
        }
        if !has_body {
            report.errors.push("Missing BLOCK 3: BODY BLOCK".into());
        }
        if !has_closing {
            report.errors.push("Missing BLOCK 4: CLOSING BLOCK".into());
        }

        // Check 2: Verify sequential ordering
        for i in 1..blocks.len() {
            if blocks[i].kind <= blocks[i - 1].kind {
                let err = format!(
                    "Block order violation at line {}: {} appeared after {}",
                    blocks[i].line_number,
                    blocks[i].kind.name(),
                    blocks[i - 1].kind.name()
                );
                report.errors.push(err);
            }
        }

        // Check 3: Semantic Content Analysis Across Detected Block Regions
        if has_metadata && has_opening && has_body && has_closing && report.errors.is_empty() {
            let meta_start = blocks.iter().find(|b| b.kind == BlockKind::Metadata).unwrap().line_number;
            let open_start = blocks.iter().find(|b| b.kind == BlockKind::Opening).unwrap().line_number;
            let body_start = blocks.iter().find(|b| b.kind == BlockKind::Body).unwrap().line_number;
            let close_start = blocks.iter().find(|b| b.kind == BlockKind::Closing).unwrap().line_number;

            let lines: Vec<&str> = content.lines().collect();

            let get_slice = |start: usize, end: usize| -> String {
                if start <= end && end <= lines.len() {
                    lines[(start - 1)..(end - 1)].join("\n")
                } else if start <= lines.len() {
                    lines[(start - 1)..].join("\n")
                } else {
                    String::new()
                }
            };

            let meta_slice = get_slice(meta_start, open_start);
            let open_slice = get_slice(open_start, body_start);
            let body_slice = get_slice(body_start, close_start);
            let close_slice = get_slice(close_start, lines.len() + 1);

            // Block 1 Validation
            let has_module_decl = meta_slice.contains("= [Module]:")
                || meta_slice.contains("Module <")
                || meta_slice.contains("Module [")
                || meta_slice.contains("Scroll <");
            if !has_module_decl {
                report.warnings.push(
                    "BLOCK 1: Missing formal module identity declaration (e.g. '= [Module]: < ... > & [ ... ]').".into()
                );
            }
            if !meta_slice.contains("Version") && !meta_slice.contains("version") {
                report.warnings.push("BLOCK 1: Missing version specification in metadata context.".into());
            }
            if file_path.contains("lexicon") || file_path.contains("manifest") {
                if !meta_slice.contains("Exodus") && !meta_slice.contains("Scripture") && !meta_slice.contains("Witness") {
                    report.warnings.push(
                        "BLOCK 1: Lexicon/Covenant specification should ground its invariant witness (e.g. '< Scripture { ... } >').".into()
                    );
                }
            }

            // Block 2 Validation
            if !open_slice.contains("Opening") {
                report.errors.push("BLOCK 2: Missing 'Opening' block scope declaration.".into());
            }
            if !open_slice.contains("preconditions") && !open_slice.contains("Preconditions") && !open_slice.contains("+1") {
                report.warnings.push("BLOCK 2: Opening block should explicitly verify preconditions / armed state.".into());
            }

            // Block 3 Validation
            if !body_slice.contains("Body") {
                report.errors.push("BLOCK 3: Missing 'Body' block scope declaration.".into());
            }

            // Block 4 Validation
            if !close_slice.contains("Closing") {
                report.errors.push("BLOCK 4: Missing 'Closing' block scope declaration.".into());
            }
            if !close_slice.contains("commit_status") && !close_slice.contains("Commit Status") && !close_slice.contains("Status =") {
                report.warnings.push("BLOCK 4: Closing block should affirm terminal commit status (e.g. '[ Commit Status ]').".into());
            }
        }

        report.blocks_found = blocks;
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Unit Tests & Structural Certification )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_omni_reel_linter_valid_four_blocks() {
        let code = r#"
# BLOCK 1: METADATA BLOCK
= [Module]: < Math >
# BLOCK 2: OPENING BLOCK
Opening { [ Preconditions ]: [ +1:PROCEED ] }
# BLOCK 3: BODY BLOCK
Body [ [ ROLL 1 ] : [ Inscribe ] : { 1 } --> [ Cell ] ]
# BLOCK 4: CLOSING BLOCK
Closing { [ Commit Status ]: [ +1:PROCEED ] }
"#;
        let mut report = LintReport {
            file_path: "math.omni".to_string(),
            file_type: None,
            blocks_found: Vec::new(),
            metadata_score: None,
            audit_score: None,
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        };

        OmniReelLinter::lint("math.omni", code, &mut report);
        assert!(report.errors.is_empty(), "Errors: {:?}", report.errors);
        assert_eq!(report.blocks_found.len(), 4);
    }

    #[test]
    fn test_omni_reel_linter_missing_block_fails() {
        let code = r#"
# BLOCK 1: METADATA BLOCK
= [Module]: < Incomplete >
# BLOCK 2: OPENING BLOCK
Opening { Cell [ 1 ] }
# BLOCK 3: BODY BLOCK
Body [ ROLL 1 ]
"#;
        let mut report = LintReport {
            file_path: "incomplete.omni".to_string(),
            file_type: None,
            blocks_found: Vec::new(),
            metadata_score: None,
            audit_score: None,
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        };

        OmniReelLinter::lint("incomplete.omni", code, &mut report);
        assert!(report.errors.iter().any(|e| e.contains("Missing BLOCK 4")));
    }
}

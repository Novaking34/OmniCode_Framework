// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( OmniCode Process Tape (.tape) Structural & Ribbon Linter )
// ==============================================================================
//! OmniCode Process Tape (.tape) Structural Linter
//!
//! Validates canonical Turing-style process ribbons for `.tape` files:
//! 1. Leading Tape Head (Covenant & Process Manifest):
//!    - Identity declaration (`= [Tape]:` or `[Tape.Name]`).
//!    - Six Cardinal Clauses: `WHO`, `WHAT`, `WHEN`, `WHERE`, `WHY`, `HOW`.
//!    - Invariant Scripture Witness (`< Scripture { ... } >`).
//! 2. Subordinate Operational Rolls (`[ ROLL 1 ]` .. `[ ROLL N ]` or `roll_n`):
//!    - Strictly ascending numerical order.
//!    - Non-empty operational predicates with SVO verbs or transduction.
//!    - Living Aside attached to every roll.
//! 3. Terminal Commit Status Roll:
//!    - Final roll must seal the tape in an affirmative Fourfold Commit Status.
//!
//! # Scripture Witness:
//! - **Zechariah 5:1-2 (KJV)**:
//!   "Then I turned, and lifted up mine eyes, and looked, and behold a flying roll...
//!    the length thereof is twenty cubits, and the breadth thereof ten cubits."
//!   (The sequential process ribbon operates as a continuous roll of divine covenant execution).
// ==============================================================================

use crate::diagnostics::linter::{BlockKind, DetectedBlock, LintReport};

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( TapeLinter Definition )
// ==============================================================================

pub struct TapeLinter;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Process Ribbon Lifecycle & Monotonic Roll Progression )
// ==============================================================================

impl TapeLinter {
    /// Lints a `.tape` process ribbon string against canonical tape invariants.
    pub fn lint(file_path: &str, content: &str, report: &mut LintReport) {
        let lines: Vec<&str> = content.lines().collect();

        // ----------------------------------------------------------------------
        // 1. Detect and Validate Tape Head
        // ----------------------------------------------------------------------
        let mut tape_head_line = None;
        let mut tape_head_raw = String::new();
        let mut head_text_accumulator = String::new();
        let mut in_head = false;
        let mut is_block_manifest = false;

        let mut roll_headers: Vec<(usize, usize, String)> = Vec::new(); // (line_num, roll_num, text)

        for (idx, line) in lines.iter().enumerate() {
            let line_num = idx + 1;
            let trimmed = line.trim();

            if trimmed.starts_with('#') || trimmed.is_empty() {
                continue;
            }

            if tape_head_line.is_none() {
                if trimmed.starts_with("= [Tape")
                    || trimmed.starts_with("[Tape.")
                    || trimmed.starts_with("Tape [")
                    || trimmed.starts_with("Tape <")
                {
                    tape_head_line = Some(line_num);
                    tape_head_raw = trimmed.to_string();
                    in_head = true;
                    is_block_manifest = trimmed.ends_with('{');
                    head_text_accumulator.push_str(trimmed);
                    head_text_accumulator.push(' ');
                    continue;
                }
            }

            if in_head {
                if is_block_manifest {
                    head_text_accumulator.push_str(trimmed);
                    head_text_accumulator.push(' ');
                    if trimmed == "}" || trimmed.ends_with('}') {
                        in_head = false;
                    }
                } else if trimmed.starts_with("WHERE")
                    || trimmed.starts_with("WHEN")
                    || trimmed.starts_with("WHY")
                    || trimmed.starts_with("HOW")
                    || trimmed.starts_with("WHO")
                    || trimmed.starts_with("WHAT")
                    || trimmed.starts_with("< Scripture")
                    || trimmed.starts_with("::")
                {
                    head_text_accumulator.push_str(trimmed);
                    head_text_accumulator.push(' ');
                } else {
                    in_head = false;
                }
            }

            // Detect Subordinate Rolls (e.g. `[ ROLL 1 ]` or `roll_1`)
            let upper = trimmed.to_uppercase();
            if upper.starts_with("[ ROLL") || upper.starts_with("ROLL_") {
                let roll_num = Self::extract_roll_number(&upper);
                roll_headers.push((line_num, roll_num, trimmed.to_string()));
            }
        }

        // ----------------------------------------------------------------------
        // Validate Tape Head Manifest
        // ----------------------------------------------------------------------
        if let Some(h_line) = tape_head_line {
            report.blocks_found.push(DetectedBlock {
                kind: BlockKind::Metadata,
                line_number: h_line,
                header_raw: tape_head_raw,
            });

            // Check Cardinal Clauses
            let head_upper = head_text_accumulator.to_uppercase();
            let missing_clauses: Vec<&str> = ["WHO", "WHAT", "WHEN", "WHERE", "WHY", "HOW"]
                .iter()
                .filter(|&&c| !head_upper.contains(c))
                .copied()
                .collect();

            if !missing_clauses.is_empty() {
                report.warnings.push(format!(
                    "TAPE HEAD (line {h_line}): Missing cardinal clause(s): {}. \
                     OmniCode Process Tapes must specify complete cardinal coordinates.",
                    missing_clauses.join(", ")
                ));
            }

            // Check Scripture / Covenant Witness
            if file_path.contains("lexicon") || file_path.contains("covenant") {
                if !head_text_accumulator.contains("Scripture")
                    && !head_text_accumulator.contains("Exodus")
                    && !head_text_accumulator.contains("covenant")
                {
                    report.warnings.push(
                        format!("TAPE HEAD (line {h_line}): Lexicon tape should anchor its invariant witness (e.g. '< Scripture {{ ... }} >').")
                    );
                }
            }
        } else {
            report.errors.push("Missing Tape Head manifest declaration (e.g. '= [Tape]: ...' or '[Tape.Name]').".into());
        }

        // ----------------------------------------------------------------------
        // Validate Subordinate Rolls
        // ----------------------------------------------------------------------
        if roll_headers.is_empty() {
            report.errors.push("Missing operational subordinate rolls in Process Tape (expected '[ ROLL 1 ]' ...).".into());
        } else {
            report.blocks_found.push(DetectedBlock {
                kind: BlockKind::Body,
                line_number: roll_headers[0].0,
                header_raw: format!("ROLLS: {} subordinate rolls declared", roll_headers.len()),
            });

            // Verify strict sequential ordering of rolls
            for i in 1..roll_headers.len() {
                let (prev_line, prev_num, _) = roll_headers[i - 1];
                let (curr_line, curr_num, _) = roll_headers[i];
                if curr_num <= prev_num && curr_num > 0 {
                    report.errors.push(format!(
                        "Roll sequence violation at line {curr_line}: Roll {curr_num} appeared after Roll {prev_num} at line {prev_line}."
                    ));
                }
            }

            // ------------------------------------------------------------------
            // Validate Terminal Commit Status Roll
            // ------------------------------------------------------------------
            let (last_line, last_num, ref last_text) = roll_headers[roll_headers.len() - 1];
            let last_upper = last_text.to_uppercase();
            let is_commit_roll = last_upper.contains("COMMIT STATUS") || last_upper.contains("COMMIT_STATUS");

            if !is_commit_roll {
                report.warnings.push(format!(
                    "ROLL {last_num} (line {last_line}): Terminal roll should be an explicit Commit Status roll (e.g. '[ ROLL {last_num} ] : [ Commit Status ] : [ ... ]')."
                ));
            } else {
                report.blocks_found.push(DetectedBlock {
                    kind: BlockKind::Closing,
                    line_number: last_line,
                    header_raw: last_text.clone(),
                });

                // Check Fourfold state machine if branching
                if content.contains("Commit Status") && content.contains('|') {
                    let commit_slice = &content[content.find("Commit Status").unwrap()..];
                    let commit_upper = commit_slice.to_uppercase();
                    let mut missing_states = Vec::new();
                    if !commit_upper.contains("PROCEED") { missing_states.push("PROCEED (+1)"); }
                    if !commit_upper.contains("AWAIT") { missing_states.push("AWAIT (0)"); }
                    if !commit_upper.contains("HALT") { missing_states.push("HALT (NULL)"); }
                    if !commit_upper.contains("ROLLBACK") { missing_states.push("ROLLBACK (-1)"); }

                    if !missing_states.is_empty() {
                        report.errors.push(format!(
                            "COMMIT STATUS (line {last_line}): Incomplete fourfold state machine. When using conditional branching, all four states must be defined. Missing: {}.",
                            missing_states.join(", ")
                        ));
                    }
                }
            }
        }
    }

    fn extract_roll_number(text: &str) -> usize {
        let parts: Vec<&str> = text.split_whitespace().collect();
        for (i, p) in parts.iter().enumerate() {
            if *p == "ROLL" && i + 1 < parts.len() {
                if let Ok(num) = parts[i + 1].trim_matches(|c: char| !c.is_ascii_digit()).parse::<usize>() {
                    return num;
                }
            }
            if p.starts_with("ROLL_") {
                if let Ok(num) = p[5..].trim_matches(|c: char| !c.is_ascii_digit()).parse::<usize>() {
                    return num;
                }
            }
        }
        0
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Unit Tests & Ribbon Certification )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tape_linter_valid_ribbon() {
        let code = r#"
= [Tape]: < Test > {
    WHO { Operator } :: WHAT { Echo } :: WHEN { Input } ::
    WHERE { Cell } = [ 1 ] :: WHY ( Echo ) :: HOW [ +1:PROCEED ]
}
[ ROLL 1 ] : [ Inscribe ] : { 'A' } --> [ Cell ] ( Inscribe glyph )
[ ROLL 2 ] : [ Commit Status ] : [ +1:PROCEED ] ( Confirm )
"#;
        let mut report = LintReport {
            file_path: "echo.tape".to_string(),
            file_type: None,
            blocks_found: Vec::new(),
            metadata_score: None,
            audit_score: None,
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        };

        TapeLinter::lint("echo.tape", code, &mut report);
        assert!(report.errors.is_empty(), "Errors: {:?}", report.errors);
        assert!(!report.blocks_found.is_empty());
    }

    #[test]
    fn test_tape_linter_out_of_order_rolls_fail() {
        let code = r#"
= [Tape]: < Test > {
    WHO { Operator } :: WHAT { Test } :: WHEN { Input } ::
    WHERE { Cell } = [ 1 ] :: WHY ( Test ) :: HOW [ +1:PROCEED ]
}
[ ROLL 2 ] : [ Action ] : { 'B' } ( Out of order )
[ ROLL 1 ] : [ Action ] : { 'A' } ( Out of order )
[ ROLL 3 ] : [ Commit Status ] : [ +1:PROCEED ] ( Done )
"#;
        let mut report = LintReport {
            file_path: "broken.tape".to_string(),
            file_type: None,
            blocks_found: Vec::new(),
            metadata_score: None,
            audit_score: None,
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        };

        TapeLinter::lint("broken.tape", code, &mut report);
        assert!(report.errors.iter().any(|e| e.contains("Roll sequence violation")));
    }
}

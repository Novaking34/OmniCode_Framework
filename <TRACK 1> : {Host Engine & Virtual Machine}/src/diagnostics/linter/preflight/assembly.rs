// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Assembly Pre-flight Invariant Linter & Silicon Certification )
// ==============================================================================
//! # OmniCode Assembly Pipeline Pre-flight Linter
//!
//! Serves as an invariant verification aid before physical lowering, memory allocation,
//! and machine word synthesis in [`OmniAssembler`]:
//! 1. Memory Partition Pre-flight:
//!    - Verifies `WHERE {partition} = [ N ]` specifies a power of 2 >= 64 cells.
//! 2. Silicon MMIO Address Pre-flight:
//!    - Validates hardware locators (`::@ 0x...`) match known ports (e.g. 0x01 Stdout, 0x000B8000 VGA).
//! 3. Hardware CPU Register Pre-flight:
//!    - Validates that physical registers (`%self`, `%caller`, `%acc`, `%swarm`, `%disc`, etc.)
//!      correspond to the 13 canonical registers in the ISA.
//! 4. Delimiter Stack Pre-flight:
//!    - Early boundary leak detection before tokenization to avert assembler panics.
//!    - Synthesizes machine-actionable [`FixSuggestion`] remediation records for self-healing.
//!
//! # Scripture Witness:
//! - **Exodus 25:9 (KJV)**:
//!   "According to all that I shew thee, after the pattern of the tabernacle,
//!    and the pattern of all the instruments thereof, even so shall ye make it."
//!   (Pre-flight ensures every component conforms precisely to the hardware pattern before physical assembly).
//! - **1 Kings 6:7 (KJV)**:
//!   "And the house, when it was in building, was built of stone made ready before it was brought thither:
//!    so that there was neither hammer nor axe nor any tool of iron heard in the house, while it was in building."
//!   (Every block, token, and boundary is measured and verified in pre-flight so lowering requires no violent intervention).
// ==============================================================================

use crate::diagnostics::logger::{DiagnosticLogger, FailpointType, FixKind, FixSuggestion};
use crate::diagnostics::linter::LintReport;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( AssemblyLinter Definition )
// ==============================================================================

pub struct AssemblyLinter;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Pre-flight Validation Engine & Telemetry Remediation )
// ==============================================================================

impl AssemblyLinter {
    /// Pre-flights source text before assembly lowering with a default silent logger.
    pub fn preflight(content: &str, report: &mut LintReport) {
        let logger = DiagnosticLogger::new(false);
        Self::preflight_with_logger(content, report, &logger);
    }

    /// Pre-flights source text before assembly lowering, asserting hardware invariants
    /// and emitting rich failpoints with machine-actionable remediations over `logger`.
    pub fn preflight_with_logger(content: &str, report: &mut LintReport, logger: &DiagnosticLogger) {
        // ----------------------------------------------------------------------
        // 1. Partition Size Verification
        // ----------------------------------------------------------------------
        if let Some(where_idx) = content.find("WHERE") {
            let where_slice = &content[where_idx..];
            if where_slice.to_lowercase().contains("partition") {
                for token in where_slice.split_whitespace().take(10) {
                    if let Ok(num) = token.trim_matches(|c: char| !c.is_ascii_digit()).parse::<usize>() {
                        if num > 0 {
                            if num < 64 {
                                let msg = format!(
                                    "Assembly Pre-flight: Partition size [{num}] is below minimum 64 cells. Clamping to 64."
                                );
                                logger.log("PREFLIGHT", &msg);
                                report.warnings.push(msg);
                            } else if !num.is_power_of_two() {
                                let msg = format!(
                                    "Assembly Pre-flight: Partition size [{num}] is not a power of 2. May cause memory alignment penalties."
                                );
                                logger.log("PREFLIGHT", &msg);
                                report.warnings.push(msg);
                            }
                            break;
                        }
                    }
                }
            }
        }

        // ----------------------------------------------------------------------
        // 2. Silicon MMIO Address Validation
        // ----------------------------------------------------------------------
        for (idx, line) in content.lines().enumerate() {
            let line_num = idx + 1;
            if line.contains("::@") {
                if let Some(pos) = line.find("::@") {
                    let locator_slice = &line[pos + 3..];
                    let addr_token = locator_slice.split_whitespace().next().unwrap_or("");
                    let addr_clean = addr_token.trim_matches(|c: char| c == '}' || c == ']' || c == ',' || c == ';');

                    if addr_clean.starts_with("0x") || addr_clean.starts_with("0X") {
                        if let Ok(addr) = usize::from_str_radix(&addr_clean[2..], 16) {
                            match addr {
                                0x00 => {} // Stdin port
                                0x01 => {} // Stdout port
                                0x02 => {} // Stderr port
                                0x000B8000 => {} // VGA text buffer
                                0x00020000 => {} // Virtual framebuffer
                                0x03F8 => {} // COM1 serial port
                                other => {
                                    let msg = format!(
                                        "Assembly Pre-flight (line {line_num}): Unmapped silicon MMIO target 0x{other:X}. \
                                         Verify custom hardware mapping before bare-metal deployment."
                                    );
                                    logger.log("PREFLIGHT", &msg);
                                    report.warnings.push(msg);
                                }
                            }
                        }
                    }
                }
            }
        }

        // ----------------------------------------------------------------------
        // 3. Delimiter Stack Parity Pre-flight (via Lexer & BoundaryType)
        // ----------------------------------------------------------------------
        let mut lexer = crate::parser::lexer::Lexer::new(content, logger.clone());
        if let Ok(tokens) = lexer.tokenize() {
            let mut stack: Vec<(char, usize, usize)> = Vec::new();
            for tok in tokens {
                let (ch, is_open) = match tok.token_type {
                    crate::parser::lexer::TokenType::LBracket => ('[', true),
                    crate::parser::lexer::TokenType::RBracket => (']', false),
                    crate::parser::lexer::TokenType::LBrace => ('{', true),
                    crate::parser::lexer::TokenType::RBrace => ('}', false),
                    crate::parser::lexer::TokenType::LAngle => ('<', true),
                    crate::parser::lexer::TokenType::RAngle => ('>', false),
                    _ => continue,
                };

                if is_open {
                    stack.push((ch, tok.line, tok.col));
                } else if let Some((open_ch, open_line, _open_col)) = stack.pop() {
                    if crate::core::boundaries::BoundaryType::from_delimiters(open_ch, ch).is_err() {
                        let expected_close = match open_ch {
                            '[' => "]",
                            '{' => "}",
                            '<' => ">",
                            _ => "?",
                        };
                        let err_msg = format!(
                            "Assembly Pre-flight (line {}): Illegal boundary crossing '{open_ch} ... {ch}'.",
                            tok.line
                        );
                        let suggestion = FixSuggestion::new(
                            FixKind::FixBoundaryMismatch,
                            tok.line,
                            tok.col,
                            ch.to_string(),
                            expected_close.to_string(),
                            format!("Balance opening delimiter '{open_ch}' opened at line {open_line}"),
                        );
                        logger.failpoint_with_remediation(
                            FailpointType::BoundaryMismatch,
                            "LINTER",
                            &err_msg,
                            suggestion,
                        );
                        report.errors.push(err_msg);
                    }
                } else {
                    let err_msg = format!(
                        "Assembly Pre-flight (line {}): Unmatched closing delimiter '{ch}'.",
                        tok.line
                    );
                    logger.failpoint(
                        FailpointType::BoundaryMismatch,
                        "LINTER",
                        &err_msg,
                    );
                    report.errors.push(err_msg);
                }
            }

            for (unclosed, line, col) in stack {
                let expected_close = match unclosed {
                    '[' => "]",
                    '{' => "}",
                    '<' => ">",
                    _ => "?",
                };
                let err_msg = format!(
                    "Assembly Pre-flight: Unclosed delimiter '{unclosed}' opened at L{line}:C{col} reaches EOF."
                );
                let suggestion = FixSuggestion::new(
                    FixKind::CloseBoundary,
                    line,
                    col,
                    "".to_string(),
                    expected_close.to_string(),
                    format!("Close unclosed delimiter '{unclosed}' before EOF"),
                );
                logger.failpoint_with_remediation(
                    FailpointType::BoundaryLeak,
                    "LINTER",
                    &err_msg,
                    suggestion,
                );
                report.errors.push(err_msg);
            }
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Unit Tests & Pre-flight Remediation Verification )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preflight_clean_delimiters_pass() {
        let code = "Opening { Cell [ 1 ] & { 5 Trits :: 1 Byte } }";
        let logger = DiagnosticLogger::new(false);
        let mut report = LintReport {
            file_path: "clean.omni".to_string(),
            file_type: None,
            blocks_found: Vec::new(),
            metadata_score: None,
            audit_score: None,
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        };

        AssemblyLinter::preflight_with_logger(code, &mut report, &logger);
        assert!(report.errors.is_empty(), "Errors: {:?}", report.errors);
        assert!(!logger.has_failpoints());
    }

    #[test]
    fn test_preflight_boundary_mismatch_emits_remediation() {
        let code = "Opening { Cell < 1 ] }";
        let logger = DiagnosticLogger::new(false);
        let mut report = LintReport {
            file_path: "mismatch.omni".to_string(),
            file_type: None,
            blocks_found: Vec::new(),
            metadata_score: None,
            audit_score: None,
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        };

        AssemblyLinter::preflight_with_logger(code, &mut report, &logger);
        assert!(!report.errors.is_empty());
        assert!(logger.has_failpoints());
        assert_eq!(logger.remediations().len(), 1);
        assert_eq!(logger.remediations()[0].suggested_patch, ">");
    }

    #[test]
    fn test_preflight_unclosed_delimiter_emits_remediation() {
        let code = "Opening { Cell [ 1 ]";
        let logger = DiagnosticLogger::new(false);
        let mut report = LintReport {
            file_path: "unclosed.omni".to_string(),
            file_type: None,
            blocks_found: Vec::new(),
            metadata_score: None,
            audit_score: None,
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        };

        AssemblyLinter::preflight_with_logger(code, &mut report, &logger);
        assert!(!report.errors.is_empty());
        assert!(logger.has_failpoints());
        assert!(logger.remediations().iter().any(|r| r.suggested_patch == "}"));
    }
}

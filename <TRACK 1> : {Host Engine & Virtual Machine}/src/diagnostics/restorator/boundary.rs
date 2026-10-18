// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Morphological Delimiter Auto-Balancing & Boundary Repair Physics )
// ==============================================================================
// SCRIPTURAL WITNESS:
//   "Remove not the ancient landmark, which thy fathers have set."
//   — Proverbs 22:28 (KJV)
//
// ARCHITECTURAL TOPOLOGY:
//   Pass 1 of the Restorator Autonomous Healing Pipeline.
//   Diagnoses unclosed, mismatched, or leaking delimiters across source code:
//     - Angle Invariant:       < ... >
//     - Square Context:        [ ... ]
//     - Curly Scope:           { ... }
//     - Parenthetical Stream:  ( ... )
//   Leverages the OmniDebugger's delimiter stack to synthesize surgical fixes,
//   resolving BoundaryMismatch and BoundaryLeak failpoints before compilation.
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Imports, Delimiter Matching Maps & Preconditions )
// ==============================================================================
use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::{DiagnosticLogger, FixKind};
use crate::parser::lexer::Lexer;
use crate::parser::parser::Parser;
use super::AppliedRepair;

/// Subsystem responsible for morphological delimiter and boundary healing.
pub struct BoundaryHealer;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Multi-Pass Boundary Balancing & Mismatch Remediation )
// ==============================================================================
impl BoundaryHealer {
    /// Multi-pass auto-detects and repairs mismatched and leaking delimiters.
    ///
    /// # Arguments
    /// * `source` - Raw input string that may contain boundary leaks or syntax anomalies.
    /// * `repairs` - Vector accumulating records of all applied repairs.
    ///
    /// # Returns
    /// Structurally balanced source code string.
    pub fn heal(source: &str, repairs: &mut Vec<AppliedRepair>) -> String {
        let mut text = source.to_string();
        let mut max_passes = 10;

        while max_passes > 0 {
            max_passes -= 1;
            let temp_logger = DiagnosticLogger::new(false);
            let mut lexer = Lexer::new(&text, temp_logger.clone());
            let tokens = match lexer.tokenize() {
                Ok(toks) => toks,
                Err(_) => break,
            };

            let debugger = OmniDebugger::new(temp_logger.clone(), 10);
            let mut parser = Parser::new(tokens, debugger);
            let parse_res = parser.parse_program();

            if parse_res.is_ok() {
                break; // Boundaries are completely sound
            }

            // Inspect diagnostic remediations from debugger and logger
            let mut debugger_remediations = parser.debugger.drain_remediations();
            if debugger_remediations.is_empty() {
                debugger_remediations = temp_logger.remediations();
            }

            if debugger_remediations.is_empty() {
                // If EOF unclosed boundary detected without explicit remediation, balance stack
                if !parser.debugger.boundary_stack.is_empty() {
                    let mut closers = String::new();
                    while let Some(frame) = parser.debugger.boundary_stack.pop() {
                        let c = match frame.open_char {
                            '<' => '>',
                            '[' => ']',
                            '{' => '}',
                            _ => ')',
                        };
                        closers.push(c);
                        repairs.push(AppliedRepair {
                            kind: FixKind::CloseBoundary,
                            description: format!(
                                "Appended missing closing delimiter '{c}' for boundary opened with '{}' at L{}:C{}",
                                frame.open_char, frame.line, frame.col
                            ),
                            line: frame.line,
                        });
                    }
                    text.push_str(&format!("\n{}\n", closers));
                    continue;
                }
                break;
            }

            // Apply remediation patches
            let mut applied_any = false;
            for sug in debugger_remediations {
                match sug.kind {
                    FixKind::FixBoundaryMismatch => {
                        let lines: Vec<&str> = text.lines().collect();
                        if sug.line > 0 && sug.line <= lines.len() {
                            let mut target_line = lines[sug.line - 1].to_string();
                            if let Some(pos) = target_line.find(&sug.target_token) {
                                target_line.replace_range(pos..pos + sug.target_token.len(), &sug.suggested_patch);
                                let mut new_lines = lines.clone();
                                new_lines[sug.line - 1] = &target_line;
                                text = new_lines.join("\n");
                                if !text.ends_with('\n') {
                                    text.push('\n');
                                }
                                repairs.push(AppliedRepair {
                                    kind: FixKind::FixBoundaryMismatch,
                                    description: sug.rationale,
                                    line: sug.line,
                                });
                                applied_any = true;
                                break;
                            }
                        }
                    }
                    FixKind::CloseBoundary => {
                        text.push_str(&sug.suggested_patch);
                        if !text.ends_with('\n') {
                            text.push('\n');
                        }
                        repairs.push(AppliedRepair {
                            kind: FixKind::CloseBoundary,
                            description: sug.rationale,
                            line: sug.line,
                        });
                        applied_any = true;
                        break;
                    }
                    _ => {}
                }
            }

            if !applied_any {
                break;
            }
        }

        text
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Boundary Healer Unit Verification Tests & Affirmative Invariant Sealing )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boundary_healer_mismatch_correction() {
        let broken = "action < run ]";
        let mut repairs = Vec::new();
        let healed = BoundaryHealer::heal(broken, &mut repairs);
        assert!(healed.contains('>'), "Mismatch should be corrected to angle bracket");
        assert!(!repairs.is_empty(), "Repairs must be recorded");
    }

    #[test]
    fn test_boundary_healer_unclosed_delimiter_balancing() {
        let unclosed = "Scope {\n    let a = [ 1, 2, 3\n";
        let mut repairs = Vec::new();
        let healed = BoundaryHealer::heal(unclosed, &mut repairs);
        assert!(healed.contains(']'), "Square bracket must be balanced");
        assert!(healed.contains('}'), "Curly brace must be balanced");
    }
}

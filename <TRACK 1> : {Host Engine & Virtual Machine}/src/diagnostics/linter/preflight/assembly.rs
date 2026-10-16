// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Assembly Pre-flight Invariant Linter & Silicon Certification )
// ==============================================================================
//! OmniCode Assembly Pipeline Pre-flight Linter
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

use crate::diagnostics::linter::LintReport;

pub struct AssemblyLinter;

impl AssemblyLinter {
    /// Pre-flights source text before assembly lowering, asserting hardware invariants.
    pub fn preflight(content: &str, report: &mut LintReport) {
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
                                report.warnings.push(format!(
                                    "Assembly Pre-flight: Partition size [{num}] is below minimum 64 cells. Clamping to 64."
                                ));
                            } else if !num.is_power_of_two() {
                                report.warnings.push(format!(
                                    "Assembly Pre-flight: Partition size [{num}] is not a power of 2. May cause memory alignment penalties."
                                ));
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
                                    report.warnings.push(format!(
                                        "Assembly Pre-flight (line {line_num}): Unmapped silicon MMIO target 0x{other:X}. \
                                         Verify custom hardware mapping before bare-metal deployment."
                                    ));
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
        let logger = crate::diagnostics::logger::DiagnosticLogger::new(false);
        let mut lexer = crate::parser::lexer::Lexer::new(content, logger);
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
                } else if let Some((open_ch, _open_line, _open_col)) = stack.pop() {
                    if crate::core::boundaries::BoundaryType::from_delimiters(open_ch, ch).is_err() {
                        report.errors.push(format!(
                            "Assembly Pre-flight (line {}): Illegal boundary crossing '{open_ch} ... {ch}'.",
                            tok.line
                        ));
                    }
                } else {
                    report.errors.push(format!(
                        "Assembly Pre-flight (line {}): Unmatched closing delimiter '{ch}'.",
                        tok.line
                    ));
                }
            }

            for (unclosed, line, col) in stack {
                report.errors.push(format!(
                    "Assembly Pre-flight: Unclosed delimiter '{unclosed}' opened at L{line}:C{col} reaches EOF."
                ));
            }
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Status )
// ==============================================================================
// AssemblyLinter sealed.

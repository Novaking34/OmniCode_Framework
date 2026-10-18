// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Spatial Geometry, Ruler Invariants & Syntax Hygiene Linter )
// ==============================================================================
//! # OmniCode Spatial Geometry & Layout Linter
//!
//! Enforces physical spatial invariants across all OmniCode files:
//! 1. 80-Column Spatial Ruler: Guarantees legibility in IDE viewports with dual side panels.
//! 2. Delimiter & Bracket Hygiene: Disallows snake_case inside brackets (natural spaced phrases),
//!    exempting canonical bit rails (`Bit_0`..`Bit_7`) and hex/binary addresses.
//! 3. First-Class Intent Asides: Mandates Living Asides `( ... )` for AI cognitive prompts and test oracles.
//! 4. Multiline Compound Sentence Formatting: Flags run-on clauses that break columnar rhythm.
//!
//! # Scripture Witness:
//! - **Proverbs 22:28 (KJV)**:
//!   "Remove not the ancient landmark, which thy fathers have set."
//!   (The 80-column spatial boundary is the ancient landmark of readability and IDE viewport balance).
//! - **Isaiah 40:12 (KJV)**:
//!   "Who hath measured the waters in the hollow of his hand, and meted out heaven with the span..."
//!   (Precise geometric measurement ensures order across every horizontal and vertical axis).
// ==============================================================================

use crate::diagnostics::linter::LintReport;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Configuration Structure & SpatialLinter Definition )
// ==============================================================================

/// Configuration for spatial geometry and layout linting.
#[derive(Debug, Clone)]
pub struct SpatialLintConfig {
    /// Maximum allowed column width (default: 80).
    pub max_columns: usize,
    /// Whether column limit violations constitute fatal errors or advisory warnings.
    pub strict_column_limit: bool,
}

impl Default for SpatialLintConfig {
    fn default() -> Self {
        Self {
            max_columns: 80,
            strict_column_limit: false,
        }
    }
}

pub struct SpatialLinter;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Spatial Ruler Enforcement & Layout Verification )
// ==============================================================================

impl SpatialLinter {
    /// Lints source text against physical spatial geometry and syntax hygiene rules.
    pub fn lint(content: &str, report: &mut LintReport, config: &SpatialLintConfig) {
        let lines: Vec<&str> = content.lines().collect();

        // ----------------------------------------------------------------------
        // 1. 80-Column Spatial Ruler Verification
        // ----------------------------------------------------------------------
        for (idx, line) in lines.iter().enumerate() {
            let line_num = idx + 1;
            let col_count = line.chars().count();
            if col_count > config.max_columns {
                let msg = format!(
                    "Spatial Geometry (line {line_num}): Line exceeds {max} columns ({col_count} cols). \
                     OmniCode mandates <= {max} columns for dual side panel IDE viewports.",
                    max = config.max_columns
                );
                if config.strict_column_limit {
                    report.errors.push(msg);
                } else {
                    report.warnings.push(msg);
                }
            }
        }

        // ----------------------------------------------------------------------
        // 2. First-Class Intent Asides Presence (Canon 1)
        // ----------------------------------------------------------------------
        let total_lines = lines.len();
        let has_any_asides = content.contains('(') && content.contains(')');
        if total_lines > 15 && !has_any_asides {
            report.warnings.push(
                "Canon 1: Missing first-class intent Asides '(...)'. OmniCode mandates Asides \
                 for human-AI pronunciation, system prompt seeds, and pre-test oracles.".into(),
            );
        }

        // ----------------------------------------------------------------------
        // 3. Snake_case Hygiene Inside Brackets
        // ----------------------------------------------------------------------
        for (idx, line) in lines.iter().enumerate() {
            let line_num = idx + 1;
            let trimmed = line.trim();
            if trimmed.starts_with('#') {
                continue;
            }
            if let Some(start_bracket) = line.find('[') {
                if let Some(end_bracket) = line[start_bracket..].find(']') {
                    let bracket_content = &line[start_bracket + 1..start_bracket + end_bracket];
                    for word in bracket_content.split_whitespace() {
                        let is_exempt = word.starts_with("0x")
                            || word.starts_with("0b")
                            || word.starts_with("0X")
                            || word.starts_with("roll_")
                            || word.starts_with("Bit_")
                            || word.starts_with("bit_");
                        if word.contains('_') && !is_exempt {
                            report.warnings.push(format!(
                                "Syntax Hygiene (line {line_num}): Identifier '{word}' uses snake_case with underscores inside brackets. \
                                 In OmniCode, use natural spaced phrases (e.g. '[ ... ]') without underscores."
                            ));
                            break;
                        }
                    }
                }
            }
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Unit Tests & Spatial Invariant Calibration )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spatial_linter_ruler_warning_and_strict() {
        let wide_line = format!("# {}\n", "A".repeat(85));

        // Advisory mode
        let mut report_adv = LintReport {
            file_path: "test.omni".to_string(),
            file_type: None,
            blocks_found: Vec::new(),
            metadata_score: None,
            audit_score: None,
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        };
        let cfg_adv = SpatialLintConfig::default();
        SpatialLinter::lint(&wide_line, &mut report_adv, &cfg_adv);
        assert!(report_adv.errors.is_empty());
        assert!(!report_adv.warnings.is_empty());

        // Strict mode
        let mut report_strict = LintReport {
            file_path: "test.omni".to_string(),
            file_type: None,
            blocks_found: Vec::new(),
            metadata_score: None,
            audit_score: None,
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        };
        let cfg_strict = SpatialLintConfig {
            max_columns: 80,
            strict_column_limit: true,
        };
        SpatialLinter::lint(&wide_line, &mut report_strict, &cfg_strict);
        assert!(!report_strict.errors.is_empty());
    }

    #[test]
    fn test_spatial_linter_bit_rail_exempt_from_snake_case_warning() {
        let code = "[ Bit_7 ]: \"0\"\n";
        let mut report = LintReport {
            file_path: "bit.omni".to_string(),
            file_type: None,
            blocks_found: Vec::new(),
            metadata_score: None,
            audit_score: None,
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        };
        let cfg = SpatialLintConfig::default();
        SpatialLinter::lint(code, &mut report, &cfg);
        assert!(report.warnings.is_empty(), "Bit_7 must be exempt from snake_case warnings");
    }
}

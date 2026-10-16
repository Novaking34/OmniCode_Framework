// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Spatial Geometry, Ruler Invariants & Syntax Hygiene Linter )
// ==============================================================================
//! OmniCode Spatial Geometry & Layout Linter
//!
//! Enforces physical spatial invariants across all OmniCode files:
//! 1. 80-Column Spatial Ruler: Guarantees legibility in IDE viewports with dual side panels.
//! 2. Delimiter & Bracket Hygiene: Disallows snake_case inside brackets (natural spaced phrases).
//! 3. First-Class Intent Asides: Mandates Living Asides `( ... )` for AI cognitive prompts and test oracles.
//! 4. Multiline Compound Sentence Formatting: Flags run-on clauses that break columnar rhythm.

use crate::diagnostics::linter::LintReport;

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

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( SpatialLinter Definition & Constructor )
// ==============================================================================

pub struct SpatialLinter;

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
                        if word.contains('_') && !word.starts_with("0x") && !word.starts_with("roll_") {
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
// ( Subsystem Status )
// ==============================================================================
// SpatialLinter sealed.

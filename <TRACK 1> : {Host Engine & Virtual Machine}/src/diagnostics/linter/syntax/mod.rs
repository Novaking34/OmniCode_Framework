// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Syntax, Spatial & Grammar Subsystem Coordinator )
// ==============================================================================
//! # Syntax, Spatial Geometry & Grammar Linter Subsystem
//!
//! Enforces lexical syntax, grammatical AST correctness, and spatial geometry:
//! - [`spatial`]: 80-column IDE dual side-panel ruler, Living Asides, and bracket identifier naming hygiene.
//! - [`grammar`]: Lexer token stream hygiene, AST parser verification, SVO verbs, and state-prime notation.
//!
//! # Scripture Witness:
//! - **1 Corinthians 14:40 (KJV)**:
//!   "Let all things be done decently and in order."
//!   (Every line, token, delimiter, and aside must preserve rhythmic decency and grammatical order).
//! - **Isaiah 28:10 (KJV)**:
//!   "For precept must be upon precept, precept upon precept; line upon line, line upon line; here a little, and there a little..."
//!   (Columnar alignment and rule-by-rule grammatical parsing across the source tape).
// ==============================================================================

pub mod grammar;
pub mod spatial;

pub use grammar::GrammarLinter;
pub use spatial::{SpatialLintConfig, SpatialLinter};

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Syntax Subsystem Initialization & Default Configuration )
// ==============================================================================

/// Constructs the default spatial and grammar linting configuration.
pub fn default_syntax_config() -> SpatialLintConfig {
    SpatialLintConfig::default()
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Subsystem Pipeline Coordination )
// ==============================================================================

/// Helper for running both spatial and grammar passes on source code.
pub fn run_syntax_passes(
    file_path: &str,
    content: &str,
    logger: crate::diagnostics::logger::DiagnosticLogger,
    debugger: crate::diagnostics::debugger::OmniDebugger,
    report: &mut crate::diagnostics::linter::LintReport,
    config: &SpatialLintConfig,
) {
    SpatialLinter::lint(content, report, config);
    GrammarLinter::lint(file_path, content, logger, debugger, report);
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Unit Tests & Calibration Verification )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syntax_default_config() {
        let cfg = default_syntax_config();
        assert_eq!(cfg.max_columns, 80);
        assert!(!cfg.strict_column_limit);
    }
}

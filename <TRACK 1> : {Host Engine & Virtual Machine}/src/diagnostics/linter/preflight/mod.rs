// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Assembly Pre-flight Subsystem Coordinator )
// ==============================================================================
//! # Assembly Pre-flight Linter Subsystem
//!
//! Enforces hardware invariants, memory partition sizing, MMIO mappings,
//! and delimiter stack integrity before machine lowering.
//!
//! # Scripture Witness:
//! - **Luke 14:28 (KJV)**:
//!   "For which of you, intending to build a tower, sitteth not down first, and counteth the cost,
//!    whether he have sufficient to finish it?"
//!   (Pre-flight verification audits partition boundaries and hardware resources before machine lowering).
//! - **Proverbs 24:27 (KJV)**:
//!   "Prepare thy work without, and make it fit for thyself in the field; and afterwards build thine house."
//!   (Pre-flight checks certify delimiter parity and hardware targets in advance).
// ==============================================================================

pub mod assembly;

pub use assembly::AssemblyLinter;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Preflight Pipeline Setup & Configuration )
// ==============================================================================

/// Preflight coordinator for invariant auditing prior to lowering.
pub struct PreflightCoordinator;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Pipeline Dispatch )
// ==============================================================================

impl PreflightCoordinator {
    /// Dispatches pre-flight checks on source code.
    pub fn preflight(content: &str, report: &mut crate::diagnostics::linter::LintReport) {
        AssemblyLinter::preflight(content, report);
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Unit Tests & Calibration Verification )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::linter::LintReport;

    #[test]
    fn test_preflight_coordinator_valid_input() {
        let code = "Opening { Cell [ 1 ] }";
        let mut report = LintReport {
            file_path: "test.omni".to_string(),
            file_type: None,
            blocks_found: Vec::new(),
            metadata_score: None,
            audit_score: None,
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        };
        PreflightCoordinator::preflight(code, &mut report);
        assert!(report.errors.is_empty());
    }
}

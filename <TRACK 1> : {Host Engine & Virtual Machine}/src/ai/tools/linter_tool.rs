// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( AI Linter Tool: 4-Block Architectural Auditing & Spatial Discipline )
// ==============================================================================
// SCRIPTURAL WITNESS:
//   "According to the grace of God which is given unto me, as a wise masterbuilder,
//    I have laid the foundation, and another buildeth thereon. But let every man
//    take heed how he buildeth thereupon."
//   — 1 Corinthians 3:10 (KJV)
//
//   "Let all things be done decently and in order."
//   — 1 Corinthians 14:40 (KJV)
//
// ARCHITECTURAL TOPOLOGY:
//   Tool 3 of the CPI-SI Autonomous Diagnostic Tool Belt.
//   Provides the AI Cortex with architectural geometry auditing:
//     - Verifies canonical 4-Block sequencing (Metadata -> Opening -> Body -> Closing).
//     - Audits 80-column spatial line geometry and indent symmetry.
//     - Checks covenant and scriptural witness completeness.
//     - Produces deterministic file health ratings to guide self-improvement.
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Imports, Linter Integration & Tool Struct Definition )
// ==============================================================================
use crate::diagnostics::linter::{LintReport, OmniLinter};
use crate::diagnostics::logger::DiagnosticLogger;

/// Diagnostic tool giving the AI model direct access to 4-Block architectural auditing.
#[derive(Debug, Clone)]
pub struct LinterTool {
    pub linter: OmniLinter,
}

impl LinterTool {
    /// Creates a new [`LinterTool`] bound to the provided linter instance.
    pub fn new(linter: OmniLinter) -> Self {
        Self { linter }
    }
}

/// Detailed architectural health summary returned to the AI Cortex.
#[derive(Debug, Clone)]
pub struct LinterToolReport {
    pub is_valid: bool,
    pub block_count: usize,
    pub blocks_detected: Vec<String>,
    pub error_count: usize,
    pub errors: Vec<String>,
    pub warning_count: usize,
    pub warnings: Vec<String>,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Architectural Auditing Routines & Spatial Analysis )
// ==============================================================================
impl LinterTool {
    /// Audits source code in memory against canonical 4-Block invariants.
    pub fn audit_source(&self, file_path: &str, content: &str) -> LinterToolReport {
        let rep: LintReport = self.linter.lint_source(file_path, content);

        let blocks = rep
            .blocks_found
            .iter()
            .map(|b| format!("{}: line {}", b.kind.name(), b.line_number))
            .collect();

        LinterToolReport {
            is_valid: rep.is_valid,
            block_count: rep.blocks_found.len(),
            blocks_detected: blocks,
            error_count: rep.errors.len(),
            errors: rep.errors,
            warning_count: rep.warnings.len(),
            warnings: rep.warnings,
        }
    }

    /// Audits source code in memory against canonical 4-Block invariants.
    pub fn lint_source(&self, content: &str) -> LinterToolReport {
        self.audit_source("in_memory.omni", content)
    }

    /// Quickly checks if source code satisfies 4-Block compliance.
    pub fn is_4block_compliant(&self, content: &str) -> bool {
        let rep = self.linter.lint_source("memory.omni", content);
        rep.is_valid
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Linter Tool Default Sealing & Invariant Verification )
// ==============================================================================
impl Default for LinterTool {
    fn default() -> Self {
        Self::new(OmniLinter::new(DiagnosticLogger::new(false)))
    }
}

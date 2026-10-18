// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( AI Restorator Tool: Autonomous Self-Healing & Two-Way Checksum Parity )
// ==============================================================================
// SCRIPTURAL WITNESS:
//   "He healeth the broken in heart, and bindeth up their wounds."
//   — Psalm 147:3 (KJV)
//
//   "In the mouth of two or three witnesses shall every word be established."
//   — 2 Corinthians 13:1 (KJV)
//
//   "Brethren, if a man be overtaken in a fault, ye which are spiritual,
//    restore such an one in the spirit of meekness; considering thyself,
//    lest thou also be tempted."
//   — Galatians 6:1 (KJV)
//
// ARCHITECTURAL TOPOLOGY:
//   Tool 4 of the CPI-SI Autonomous Diagnostic Tool Belt.
//   Provides the AI Cortex with autonomic self-healing capabilities:
//     - Executes the 5-pass self-healing pipeline conforming to character/number.omni.
//     - Evaluates the Two-Way Checksum (Logger <===> Debugger Parity Gate).
//     - Enforces homeostatic wait (0:AWAIT) on witness disagreement to eliminate
//       both false positives (mutating healthy code) and false negatives (unverified commit).
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Imports, Restorator Integration & Tool Struct Definition )
// ==============================================================================
use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::DiagnosticLogger;
use crate::diagnostics::restorator::{AppliedRepair, RestorationReport, Restorator, TwoWayAgreement};

/// Diagnostic tool giving the AI model direct access to autonomic self-healing and two-way checksums.
#[derive(Debug, Clone)]
pub struct RestoratorTool {
    pub restorator: Restorator,
}

impl RestoratorTool {
    /// Creates a new [`RestoratorTool`] bound to the provided restorator instance.
    pub fn new(restorator: Restorator) -> Self {
        Self { restorator }
    }
}

/// Detailed self-healing report returned to the AI Cortex.
#[derive(Debug, Clone)]
pub struct RestoratorToolReport {
    pub was_modified: bool,
    pub original_content: String,
    pub restored_content: String,
    pub repairs_applied: Vec<AppliedRepair>,
    pub verified: bool,
    pub errors: Vec<String>,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Autonomous Code Healing & Two-Way Parity Gate Execution )
// ==============================================================================
impl RestoratorTool {
    /// Restores source code in memory via the 5-pass self-healing pipeline.
    pub fn heal_source(&self, source: &str) -> RestoratorToolReport {
        let rep: RestorationReport = self.restorator.restore_source(source);

        RestoratorToolReport {
            was_modified: rep.was_modified,
            original_content: rep.original_content,
            restored_content: rep.restored_content,
            repairs_applied: rep.repairs_applied,
            verified: rep.verified,
            errors: rep.errors,
        }
    }

    /// Evaluates the 2-Way Checksum parity gate between the sensory logger and the spatial debugger.
    pub fn check_two_way_agreement(&self, debugger: &OmniDebugger, candidate: &str) -> TwoWayAgreement {
        self.restorator.evaluate_two_way_checksum(debugger, candidate)
    }

    /// Restores a file on disk in place with full round-trip verification.
    pub fn heal_file(&self, path: &str, in_place: bool) -> Result<RestoratorToolReport, String> {
        let rep = self.restorator.restore_file(path, in_place)?;
        Ok(RestoratorToolReport {
            was_modified: rep.was_modified,
            original_content: rep.original_content,
            restored_content: rep.restored_content,
            repairs_applied: rep.repairs_applied,
            verified: rep.verified,
            errors: rep.errors,
        })
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Restorator Tool Default Sealing & Invariant Verification )
// ==============================================================================
impl Default for RestoratorTool {
    fn default() -> Self {
        Self::new(Restorator::new(DiagnosticLogger::new(false)))
    }
}

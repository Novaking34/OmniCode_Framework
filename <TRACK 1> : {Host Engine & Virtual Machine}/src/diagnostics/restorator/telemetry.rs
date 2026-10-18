// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Restorator Telemetry, Cognitive Notes & Autonomous Healing Trace Logger )
// ==============================================================================
// SCRIPTURAL WITNESS:
//   "He healeth the broken in heart, and bindeth up their wounds."
//   — Psalm 147:3 (KJV)
//
// ARCHITECTURAL TOPOLOGY:
//   Telemetry, cognitive trace recording, and flight logging for the Restorator.
//   Records autonomic self-healing actions, 2-way checksum agreements, and applied
//   remediation notes directly into the DiagnosticLogger's episodic ring buffers.
//   Ensures the AI Cortex and human operators maintain complete historical custody
//   of all autonomous code transformations executed across the system.
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Imports, Telemetry Channels & Scoped Logging Helpers )
// ==============================================================================
use crate::diagnostics::logger::DiagnosticLogger;
use super::AppliedRepair;

/// Telemetry logging subsystem for the Restorator.
pub struct RestoratorTelemetry;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Cognitive Observation Recording, Healing Inscriptions & Event Logging )
// ==============================================================================
impl RestoratorTelemetry {
    /// Inscribes an episodic cognitive note documenting a completed self-healing cycle.
    pub fn record_repairs_applied(logger: &DiagnosticLogger, repairs: &[AppliedRepair]) {
        if repairs.is_empty() {
            return;
        }

        let repair_descs: Vec<String> = repairs.iter().map(|r| r.description.clone()).collect();
        logger.record_ai_observation(
            "SELF_HEAL_APPLIED",
            &format!("Applied {} automated repairs", repairs.len()),
            &format!("Repairs executed: {}", repair_descs.join("; ")),
            ["self_healing", "remediation", "restorator"].as_slice(),
            None,
        );
    }

    /// Logs an incident-driven targeted fix applied from a structured LogEvent.
    pub fn record_log_driven_fix(
        logger: &DiagnosticLogger,
        failpoint_name: &str,
        bytes_modified: usize,
    ) {
        logger.record_ai_observation(
            "LOG_DRIVEN_RESTORE",
            &format!("Applied targeted fix from log for {failpoint_name}"),
            &format!("Modified {bytes_modified} bytes in candidate buffer"),
            ["self_healing", "log_driven"].as_slice(),
            None,
        );
    }

    /// Emits a notice when restored content is committed in-place to disk.
    pub fn log_in_place_write(logger: &DiagnosticLogger, path: &str) {
        logger.log("RESTORATOR", format!("Wrote restored content in-place to '{path}'"));
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Restorator Telemetry Unit Verification Tests & Invariant Sealing )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::logger::FixKind;

    #[test]
    fn test_restorator_telemetry_records_cognitive_notes() {
        let logger = DiagnosticLogger::new(false);
        let repairs = vec![AppliedRepair {
            kind: FixKind::CloseBoundary,
            description: "Appended missing closing delimiter".into(),
            line: 1,
        }];

        RestoratorTelemetry::record_repairs_applied(&logger, &repairs);
        let notes = logger.cognitive_notes();
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].category, "SELF_HEAL_APPLIED");
    }
}

// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Debugger Telemetry, Sensory Gaze Inscription & Cognitive Baton Pass )
// ==============================================================================
// SCRIPTURAL WITNESS:
//   "The LORD is in his holy temple, the LORD'S throne is in heaven:
//    his eyes behold, his eyelids try, the children of men."
//   — Psalm 11:4 (KJV)
//
// ARCHITECTURAL TOPOLOGY:
//   The telemetry and cognitive baton pass organ of the OmniDebugger.
//   Bridges the focused scrutiny of the debugger ("the eyes") back into
//   the ambient environmental awareness of the DiagnosticLogger:
//     - Dispatches formatted gaze logs to designated telemetry channels.
//     - Inscribes episodic CognitiveNotes capturing root-cause assessments.
//     - Records inspection RollFrames into the high-speed circular flight recorder.
//     - Passes remediation suggestions to the logger's active failpoint bank.
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Imports, Monotonic Clocks, RollFrames & Cognitive Data Models )
// ==============================================================================
use crate::core::ternary::QuadState;
use crate::diagnostics::logger::{DiagnosticLogger, RollFrame};

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Telemetry Baton Pass: Focused Gaze Logging, Notes & Flight Probe Frames )
// ==============================================================================

/// Logs a focused debugger scrutiny observation to the specified diagnostic channel.
pub fn log_debugger_focus(
    logger: &DiagnosticLogger,
    channel: &'static str,
    message: impl Into<String>,
) {
    logger.log(channel, format!("Gaze Focus: {}", message.into()));
}

/// Inscribes an episodic cognitive note into the logger representing a focused debugger assessment.
pub fn record_debugger_cognitive_note(
    logger: &DiagnosticLogger,
    summary: &str,
    details: &str,
    tags: &[&str],
    code: Option<&str>,
) {
    logger.record_ai_observation(
        "REFLECTION",
        summary,
        details,
        tags,
        code,
    );
}

/// Appends a focused inspection probe frame into the DiagnosticLogger's in-memory flight recorder.
pub fn record_debugger_probe_frame(
    logger: &DiagnosticLogger,
    step: usize,
    verb: &'static str,
    state: QuadState,
) {
    let frame = RollFrame::new(
        0,
        step,
        0,
        verb,
        (0, 0),
        (0, 0),
        state,
        0,
        None,
    );
    logger.record_roll_frame(frame);
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Telemetry Unit Tests & Affirmative Invariant Sealing )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debugger_telemetry_and_cognitive_note_emission() {
        let logger = DiagnosticLogger::new(false);

        log_debugger_focus(&logger, "DEBUGGER", "Inspecting delimiter boundary stack");
        record_debugger_cognitive_note(
            &logger,
            "[Debugger] Boundary Stack Unwinding",
            "Unwound stack to recover from mismatched delimiter at L12:C4",
            ["debugger", "gaze", "boundary", "unwind"].as_slice(),
            Some("< Delimiter >"),
        );
        record_debugger_probe_frame(&logger, 1, "INSPECT", QuadState::Proceed);

        let notes = logger.cognitive_notes();
        assert!(!notes.is_empty());
        let last_note = notes.last().unwrap();
        assert!(last_note.summary.contains("[Debugger] Boundary Stack Unwinding"));
        assert!(last_note.tags.contains(&"gaze".to_string()));

        let frames = logger.recent_roll_frames(10);
        assert!(!frames.is_empty());
        assert_eq!(frames.last().unwrap().verb, "INSPECT");
    }
}

// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Diagnostic Logger Telemetry & Episodic Memory Trace for Hover Subsystem )
// ==============================================================================
// SCRIPTURAL WITNESS:
//   "The hearing ear, and the seeing eye, the LORD hath made even both of them."
//   — Proverbs 20:12 (KJV)
//
// ARCHITECTURAL TOPOLOGY:
//   Provides active sensory telemetry and episodic memory trace logging for IDE
//   hover inspections. Bridges developer focus coordinates directly into the
//   DiagnosticLogger flight recorder and AICortex episodic memory.
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Imports, Diagnostic Logger & Telemetry Types )
// ==============================================================================
use crate::core::ternary::QuadState;
use crate::diagnostics::logger::failpoint::FailpointType;
use crate::diagnostics::logger::telemetry::RollFrame;
use crate::diagnostics::logger::DiagnosticLogger;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Hover Hit Telemetry, Miss Failpoints & Episodic AI Cognitive Notes )
// ==============================================================================

/// Logs an informational telemetry event when a symbol is successfully resolved.
pub fn log_hover_resolved(
    logger: &DiagnosticLogger,
    symbol: &str,
    file_path: &str,
    line: usize,
    col: usize,
    has_tape: bool,
    has_omni: bool,
) {
    let tier_desc = match (has_tape, has_omni) {
        (true, true) => "3-Tier (Tape + Omni Blueprint + Silicon Footprint)",
        (true, false) => "2-Tier (Tape Roll + Silicon Footprint)",
        (false, true) => "2-Tier (Omni Blueprint + Silicon Footprint)",
        (false, false) => "1-Tier (Ambient Identifier)",
    };

    logger.log(
        "HOVER",
        format!("Resolved symbol [{symbol}] at {file_path}:{line}:{col} ({tier_desc})"),
    );

    // Record zero-allocation flight roll frame
    let frame = RollFrame::new(
        0,
        line,
        line,
        "HOVER_INSPECT",
        (0, 0),
        (0, 0),
        QuadState::Proceed,
        0,
        Some(format!("Hovered on [{symbol}] ({tier_desc})")),
    );
    logger.record_roll_frame(frame);
}

/// Logs a warning failpoint when a coordinate or symbol resolution yields no definition.
pub fn log_hover_miss(
    logger: &DiagnosticLogger,
    file_path: &str,
    line: usize,
    col: usize,
    reason: &str,
) {
    logger.log(
        "HOVER_MISS",
        format!("Hover lookup missed at {file_path}:{line}:{col} - {reason}"),
    );

    logger.failpoint(
        FailpointType::SyntaxAnomaly,
        "HOVER",
        format!("Hover coordinate unresolved: {file_path}:{line}:{col} ({reason})"),
    );
}

/// Records a structured episodic cognitive note so AICortex retains memory of inspected symbols.
pub fn record_hover_cognitive_trace(
    logger: &DiagnosticLogger,
    symbol: &str,
    file_path: &str,
    line: usize,
    col: usize,
    summary: &str,
) {
    logger.record_ai_observation(
        "IDE_HOVER_INSPECTION",
        &format!("Developer/Agent inspected [{symbol}] at {file_path}:{line}:{col}"),
        summary,
        &["hover", "ide", "docstring", "inspection"],
        Some(file_path),
    );
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Telemetry Unit Tests & Affirmative Invariant Sealing )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hover_telemetry_and_cognitive_note_logging() {
        let logger = DiagnosticLogger::new(false);

        log_hover_resolved(
            &logger,
            "Manifest",
            "manifest.tape",
            39,
            7,
            true,
            true,
        );

        let frames = logger.recent_roll_frames(1);
        assert!(!frames.is_empty());
        let frame = &frames[0];
        assert_eq!(frame.verb, "HOVER_INSPECT");
        assert!(frame.intent_aside.as_ref().unwrap().contains("Manifest"));

        record_hover_cognitive_trace(
            &logger,
            "Manifest",
            "manifest.tape",
            39,
            7,
            "Verified 3-Tier Hover Card for ManifestCovenant",
        );

        let notes = logger.cognitive_notes();
        assert_eq!(notes.len(), 1);
        let note = &notes[0];
        assert_eq!(note.category, "IDE_HOVER_INSPECTION");
        assert!(note.summary.contains("Manifest"));
    }

    #[test]
    fn test_hover_miss_failpoint_emission() {
        let logger = DiagnosticLogger::new(false);

        log_hover_miss(
            &logger,
            "unknown.tape",
            99,
            1,
            "No symbol found at coordinate",
        );

        assert!(logger.has_failpoints());
        let fps = logger.failpoints();
        assert!(fps.iter().any(|f| f.message.contains("Hover coordinate unresolved")));
    }
}

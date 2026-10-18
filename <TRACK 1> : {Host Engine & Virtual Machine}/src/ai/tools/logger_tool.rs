// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( AI Logger Tool: Sensory Flight Telemetry & Episodic Memory Inspection )
// ==============================================================================
// SCRIPTURAL WITNESS:
//   "The eyes of the LORD are in every place, beholding the evil and the good."
//   — Proverbs 15:3 (KJV)
//
//   "For there is nothing covered, that shall not be revealed; neither hid,
//    that shall not be known."
//   — Luke 12:2 (KJV)
//
// ARCHITECTURAL TOPOLOGY:
//   Tool 1 of the CPI-SI Autonomous Diagnostic Tool Belt.
//   Provides the AI Cortex with first-class sensory retrospection:
//     - Ingests circular flight recorder RollFrames.
//     - Queries episodic session incident logs and failpoint distributions.
//     - Tracks monotonic hardware clocks and bare-metal Genesis Inception events.
//   Allows the AI to inspect what just happened in runtime without guessing.
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Imports, Query Parameters & Response Representation )
// ==============================================================================
use std::path::Path;
use crate::diagnostics::logger::{create_query_engine_at, DiagnosticLogger, FailpointType, LogEvent, RollFrame};

/// Diagnostic tool giving the AI model direct access to sensory logging telemetry.
#[derive(Debug, Clone)]
pub struct LoggerTool {
    pub logger: DiagnosticLogger,
}

impl LoggerTool {
    /// Creates a new [`LoggerTool`] bound to the provided logger instance.
    pub fn new(logger: DiagnosticLogger) -> Self {
        Self { logger }
    }
}

/// Detailed telemetry summary returned to the AI Cortex.
#[derive(Debug, Clone)]
pub struct LoggerToolReport {
    pub total_logs: usize,
    pub failpoint_count: usize,
    pub failpoints: Vec<(FailpointType, String)>,
    pub recent_roll_frames: Vec<RollFrame>,
    pub cognitive_notes: Vec<String>,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Sensory Inspection Routines & Episodic Query Execution )
// ==============================================================================
impl LoggerTool {
    /// Inspects the current flight recorder buffer and failpoint registry.
    pub fn inspect_recent_telemetry(&self, max_frames: usize) -> LoggerToolReport {
        let events = self.logger.events();
        let failpoints = self.logger.failpoints();
        let recent_frames = self.logger.recent_roll_frames(max_frames);
        let notes = self.logger.cognitive_notes();

        let fp_summary = failpoints
            .iter()
            .filter_map(|fp| fp.failpoint_type.as_ref().map(|t| (t.clone(), fp.message.clone())))
            .collect();

        let note_summary = notes
            .iter()
            .map(|n| format!("[{}] {}: {}", n.category, n.summary, n.details))
            .collect();

        LoggerToolReport {
            total_logs: events.len(),
            failpoint_count: failpoints.len(),
            failpoints: fp_summary,
            recent_roll_frames: recent_frames,
            cognitive_notes: note_summary,
        }
    }

    /// Queries past episodic sessions stored in a scroll directory.
    pub fn query_session_history(&self, scroll_dir: &Path, session_id: &str) -> Option<LogEvent> {
        let query_engine = create_query_engine_at(scroll_dir).ok()?;
        let session = query_engine.find_by_session(session_id)?;
        session.log_path.as_ref().and_then(|lp| {
            if let Ok(content) = std::fs::read_to_string(lp) {
                Some(LogEvent {
                    elapsed_micros: 0,
                    channel: "EPISODIC",
                    message: content,
                    is_failpoint: false,
                    failpoint_type: None,
                    suggestion: None,
                    cognitive_note: None,
                })
            } else {
                None
            }
        })
    }

    /// Inscribes an AI thought or diagnostic deduction into the sensory log stream.
    pub fn log_ai_observation(&self, category: &str, headline: &str, text: &str) {
        self.logger.record_ai_observation(
            category,
            headline,
            text,
            ["cortex_tool", "logger_tool"].as_slice(),
            None,
        );
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Logger Tool Default Sealing & Unit Verification )
// ==============================================================================
impl Default for LoggerTool {
    fn default() -> Self {
        Self::new(DiagnosticLogger::new(false))
    }
}

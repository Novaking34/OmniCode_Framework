// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Diagnostic Logger Subsystem Identity, Modular Facade & Re-exports )
// ==============================================================================
//! OmniCode Diagnostics: Modular Telemetry, Failpoint & Flight Recorder System
//!
//! # Architecture & Epistemic Role
//! Decomposed into specialized machine organs:
//! - [`universal`]: Monotonic clock, nanosecond timers, and lock-free event bus.
//! - [`failpoint`]: Failpoint pathology classification and machine-applicable repairs.
//! - [`telemetry`]: Game-dev RollFrame vectors, CognitiveNotes, and FlightRecorder.
//! - [`inscriber`]: Canonical 4-Block .scroll and .log writers with rotation policies.
//! - [`query`]: Episodic query engine enabling O(1) session lookup and range scans.

pub mod universal;
pub mod failpoint;
pub mod telemetry;
pub mod inscriber;
pub mod query;

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

pub use universal::{EventBus, LogEvent, MonotonicClock, generate_timestamp_id};
pub use failpoint::{FailpointType, FixKind, FixSuggestion};
pub use telemetry::{CognitiveNote, FlightRecorder, RollFrame};
pub use inscriber::{
    RotationPolicy, format_base_name, inscribe_audit_scroll,
    inscribe_canonical_scroll, inscribe_telemetry_log, parse_file_name,
    resolve_vessel_path,
};
pub use query::{EpisodicQueryEngine, IndexedSession};

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( DiagnosticLogger Data Structure & Initialization )
// ==============================================================================
/// Thread-safe diagnostic logging, flight recording & cognitive memory engine.
#[derive(Debug, Clone)]
pub struct DiagnosticLogger {
    clock: MonotonicClock,
    bus: EventBus,
    cognitive_notes: Arc<Mutex<Vec<CognitiveNote>>>,
    flight_recorder: Arc<Mutex<FlightRecorder>>,
}

impl DiagnosticLogger {
    /// Constructs a new [`DiagnosticLogger`].
    pub fn new(print_live: bool) -> Self {
        Self {
            clock: MonotonicClock::new(),
            bus: EventBus::new(print_live),
            cognitive_notes: Arc::new(Mutex::new(Vec::new())),
            flight_recorder: Arc::new(Mutex::new(FlightRecorder::new(512))),
        }
    }
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Logging Operations, Failpoint Trapping, Flight Recording & Inscription )
// ==============================================================================
impl DiagnosticLogger {
    /// Inscribes a normal debug or trace event into the diagnostic log.
    pub fn log(&self, channel: &'static str, message: impl Into<String>) {
        let msg = message.into();
        let elapsed = self.clock.elapsed_micros();
        let event = LogEvent {
            elapsed_micros: elapsed,
            channel,
            message: msg,
            is_failpoint: false,
            failpoint_type: None,
            suggestion: None,
            cognitive_note: None,
        };
        self.bus.emit(event);
    }

    /// Intercepts and records a critical failpoint event without remediation.
    pub fn failpoint(
        &self,
        fp_type: FailpointType,
        channel: &'static str,
        message: impl Into<String>,
    ) {
        let msg = message.into();
        let elapsed = self.clock.elapsed_micros();
        let note = CognitiveNote {
            timestamp_micros: elapsed,
            category: "INCIDENT".to_string(),
            summary: msg.clone(),
            details: format!("Failpoint trapped: {}", fp_type.as_str()),
            tags: vec!["failpoint".to_string(), fp_type.as_str().to_lowercase()],
            context_snippet: None,
        };

        let event = LogEvent {
            elapsed_micros: elapsed,
            channel,
            message: msg,
            is_failpoint: true,
            failpoint_type: Some(fp_type),
            suggestion: None,
            cognitive_note: Some(note.clone()),
        };

        self.cognitive_notes.lock().unwrap().push(note);
        self.bus.emit(event);
    }

    /// Intercepts a critical failpoint event with concrete remediation telemetry.
    pub fn failpoint_with_remediation(
        &self,
        fp_type: FailpointType,
        channel: &'static str,
        message: impl Into<String>,
        suggestion: FixSuggestion,
    ) {
        let msg = message.into();
        let elapsed = self.clock.elapsed_micros();
        let note = CognitiveNote {
            timestamp_micros: elapsed,
            category: "INCIDENT".to_string(),
            summary: msg.clone(),
            details: suggestion.rationale.clone(),
            tags: vec![
                "failpoint".to_string(),
                fp_type.as_str().to_lowercase(),
                "remediation_ready".to_string(),
            ],
            context_snippet: Some(suggestion.target_token.clone()),
        };

        let event = LogEvent {
            elapsed_micros: elapsed,
            channel,
            message: msg,
            is_failpoint: true,
            failpoint_type: Some(fp_type),
            suggestion: Some(suggestion),
            cognitive_note: Some(note.clone()),
        };

        self.cognitive_notes.lock().unwrap().push(note);
        self.bus.emit(event);
    }

    /// Records an arbitrary cognitive observation into episodic memory.
    pub fn record_ai_observation(
        &self,
        category: &str,
        summary: &str,
        details: &str,
        tags: &[&str],
        context: Option<&str>,
    ) -> CognitiveNote {
        let elapsed = self.clock.elapsed_micros();
        let note = CognitiveNote {
            timestamp_micros: elapsed,
            category: category.to_string(),
            summary: summary.to_string(),
            details: details.to_string(),
            tags: tags.iter().map(|s| s.to_string()).collect(),
            context_snippet: context.map(|s| s.to_string()),
        };

        self.cognitive_notes.lock().unwrap().push(note.clone());
        self.log(
            "COGNITION",
            format!("[{category}] {summary} (tags: {:?})", note.tags),
        );
        note
    }

    /// Inscribes a pre-formed [`CognitiveNote`] directly into memory.
    pub fn log_cognitive_note(&self, note: CognitiveNote) {
        self.cognitive_notes.lock().unwrap().push(note);
    }

    /// Retrieves a cloned snapshot of all recorded cognitive memory notes.
    pub fn cognitive_notes(&self) -> Vec<CognitiveNote> {
        self.cognitive_notes.lock().unwrap().clone()
    }

    /// Exports all recorded cognitive notes into a canonical 4-Block memory scroll.
    pub fn export_ai_memory_scroll(&self) -> String {
        let notes = self.cognitive_notes();
        inscribe_audit_scroll(&notes, self.bus.all_failpoints().len())
    }

    /// Retrieves all intercepted failpoint events.
    pub fn failpoints(&self) -> Vec<LogEvent> {
        self.bus.all_failpoints()
    }

    /// Retrieves all general log events.
    pub fn events(&self) -> Vec<LogEvent> {
        self.bus.all_events()
    }

    /// Extracts all remediation suggestions currently registered.
    pub fn remediations(&self) -> Vec<FixSuggestion> {
        self.bus
            .all_failpoints()
            .iter()
            .filter_map(|e| e.suggestion.clone())
            .collect()
    }

    /// Returns `true` if any failpoints were trapped.
    pub fn has_failpoints(&self) -> bool {
        !self.bus.all_failpoints().is_empty()
    }

    /// Clears all stored events, failpoints, and cognitive memory notes.
    pub fn clear(&self) {
        self.bus.clear();
        self.cognitive_notes.lock().unwrap().clear();
        self.flight_recorder.lock().unwrap().clear();
    }

    /// Logs an internal AI Thought, indexing it in episodic memory.
    pub fn log_thought(
        &self,
        thought_id: &str,
        omnicode_tape: &str,
        premise: &str,
        quad_state_str: &str,
    ) {
        let elapsed = self.clock.elapsed_micros();
        let note = CognitiveNote {
            timestamp_micros: elapsed,
            category: "THOUGHT".to_string(),
            summary: format!("Thought [{thought_id}]: {premise}"),
            details: omnicode_tape.to_string(),
            tags: vec![
                "thought".to_string(),
                "ai_cortex".to_string(),
                quad_state_str.to_lowercase(),
            ],
            context_snippet: Some(omnicode_tape.to_string()),
        };

        self.cognitive_notes.lock().unwrap().push(note);
        self.log(
            "CORTEX",
            format!(
                "[THOUGHT: {thought_id}] (State: {quad_state_str}) Premise: \"{premise}\""
            ),
        );
    }

    /// Records an associative memory link.
    pub fn log_memory_association(&self, concept: &str, anchor: &str, details: &str) {
        let elapsed = self.clock.elapsed_micros();
        let note = CognitiveNote {
            timestamp_micros: elapsed,
            category: "ASSOCIATION".to_string(),
            summary: format!("Memory Link: {concept} <-> {anchor}"),
            details: details.to_string(),
            tags: vec!["memory".to_string(), "scripture".to_string(), concept.to_lowercase()],
            context_snippet: Some(anchor.to_string()),
        };

        self.cognitive_notes.lock().unwrap().push(note);
        self.log(
            "MEMORY",
            format!("Associated concept '{concept}' with anchor '{anchor}'"),
        );
    }

    /// Logs the physical inscription of an episodic scroll to disk.
    pub fn log_scroll_inscribed(&self, scroll_path: &Path, session_or_id: &str) {
        self.log(
            "SCROLL",
            format!("Inscribed episodic scroll '{session_or_id}' to {:?}", scroll_path),
        );
    }

    /// Inscribes a canonical 4-Block .scroll file to the given path.
    pub fn inscribe_canonical_scroll(
        &self,
        path: &Path,
        session_id: &str,
        prompt: &str,
        synthesized_omnicode: &str,
        state_str: &str,
        response: &str,
    ) -> Result<PathBuf, String> {
        let elapsed = self.clock.elapsed_micros();
        let res = inscribe_canonical_scroll(
            path,
            session_id,
            prompt,
            synthesized_omnicode,
            state_str,
            response,
            elapsed,
        )?;
        self.log_scroll_inscribed(path, session_id);
        Ok(res)
    }

    // --- GAME-DEV FLIGHT RECORDER & TELEMETRY EXTENSIONS ---

    /// Inscribes a structured execution frame into the in-memory flight recorder.
    pub fn record_roll_frame(&self, frame: RollFrame) {
        self.log(
            "FRAME",
            format!(
                "Roll {}/{}: {} => {:?}",
                frame.roll_index, frame.total_rolls, frame.verb, frame.quad_state
            ),
        );
        self.flight_recorder.lock().unwrap().record_frame(frame);
    }

    /// Retrieves the most recent execution frames from the flight recorder.
    pub fn recent_roll_frames(&self, count: usize) -> Vec<RollFrame> {
        self.flight_recorder.lock().unwrap().recent_frames(count)
    }

    /// Retrieves the most recent anomalous or failing roll frame.
    pub fn last_anomalous_frame(&self) -> Option<RollFrame> {
        self.flight_recorder.lock().unwrap().last_anomalous_frame()
    }

    /// Creates and populates an [`EpisodicQueryEngine`] pointing to `storehouse_root`.
    pub fn create_query_engine(&self, storehouse_root: &Path) -> EpisodicQueryEngine {
        let mut engine = EpisodicQueryEngine::new();
        let _ = engine.index_directory(storehouse_root);
        engine
    }

    /// Enforces the storehouse capacity rotation policy on a partition directory.
    pub fn enforce_rotation(
        &self,
        partition_dir: &Path,
        policy: RotationPolicy,
    ) -> Result<usize, String> {
        policy.enforce(partition_dir)
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Default Trait Implementation and Subsystem Sealing )
// ==============================================================================
impl Default for DiagnosticLogger {
    /// Constructs a silent [`DiagnosticLogger`] with live terminal printing disabled.
    fn default() -> Self {
        Self::new(false)
    }
}

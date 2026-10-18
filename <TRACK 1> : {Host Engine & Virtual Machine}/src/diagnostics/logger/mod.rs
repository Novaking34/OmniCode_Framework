// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Diagnostic Logger Subsystem Identity, Modular Facade & Re-exports )
// ==============================================================================
//! # OmniCode Diagnostic Logger: Unified Observability & Autonomic Memory Substrate
//!
//! # Architectural Scope & Purpose:
//! The `logger` module is the central diagnostic facade and sensory nervous system
//! for the OmniCode Host Engine & Virtual Machine (Track 1). It bridges physical
//! bare-metal silicon inception (Track 0) with living language execution (Track 2),
//! user studio interfaces (Track 3), and persistent episodic memory (Track 4).
//!
//! Decomposed into five specialized, 4-Block modular organs:
//! - [`universal`]: Monotonic microsecond clock, silicon cycle conversions,
//!   and lock-free event bus routing ([`MonotonicClock`], [`EventBus`], [`LogEvent`]).
//! - [`failpoint`]: Four-block failpoint pathology classification, structured
//!   remediation suggestions, Track 0 register dumps, and dual-rail HALT re-inception
//!   ([`FailpointType`], [`FixSuggestion`], [`HaltDumpFrame`], [`SiliconRegisters`]).
//! - [`telemetry`]: Game-dev atomic state vectors, bounded in-memory circular
//!   flight recording, neural-symbolic cognitive notes, and cold-silicon inception baselines
//!   ([`RollFrame`], [`FlightRecorder`], [`CognitiveNote`], [`GenesisBootLog`], [`GenesisBaseline`]).
//! - [`inscriber`]: Dual-stream persistence writers producing traditional `.log` telemetry
//!   and executable 4-Block `.scroll` epistemic traces, backed by strict capacity rotation
//!   ([`RotationPolicy`], [`inscribe_canonical_scroll`], [`inscribe_telemetry_log`]).
//! - [`query`]: High-performance episodic query engine delivering multi-key $O(1)$
//!   session lookups, inverted prompt keyword indexing, chronological time-range slicing,
//!   and Track 0 bare-metal silicon log discovery ([`EpisodicQueryEngine`], [`IndexedSession`], [`SessionCriteria`]).
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**:
//!   "And he gave unto Moses, when he had made an end of communing with him upon mount Sinai,
//!    two tables of testimony, tables of stone, written with the finger of God."
//!   (The permanent inscriber establishing immutable, authoritative diagnostic records).
//! - **Malachi 3:16 (KJV)**:
//!   "Then they that feared the LORD spake often one to another: and the LORD hearkened,
//!    and heard it, and a book of remembrance was written before him..."
//!   (Episodic memory notes preserved continuously for retrospective AI reflection).
//! - **Job 8:8 (KJV)**:
//!   "For enquire, I pray thee, of the former age, and prepare thyself to the search of their fathers."
//!   (Fast $O(1)$ inquiry into past execution history to diagnose failpoints and prevent regression).
//! - **1 Corinthians 14:40 (KJV)**:
//!   "Let all things be done decently and in order."
//!   (Causal monotonicity $\Delta t \ge 0$ strictly enforced across the diagnostic bus).
//! - **2 Kings 4:4 (KJV)**:
//!   "And when thou art come in, thou shalt shut the door upon thee and upon thy sons,
//!    and shalt pour out into all those vessels, and thou shalt set aside that which is full."
//!   (The container law $\text{Occupancy} \le \text{Capacity}$ enforced by circular ring buffers and rotation).
//! - **Proverbs 25:2 (KJV)**:
//!   "It is the glory of God to conceal a thing: but the honour of kings is to search out a matter."
//!   (Deep diagnostic investigation into silicon register banks and boundary states).
//!
//! # Why Extensive Commenting Matters for AI Agents & Developers:
//! 1. **Complete Architectural Grounding**: Every organ operates under explicit
//!    invariants linking high-level virtual machine execution with low-level Track 0 silicon landmarks.
//! 2. **Autonomic DAR Loop**: Connects sensory observation directly to the `Restorator`
//!    immune system and `Debugger` transactional rollback mechanics.
//! 3. **Non-Probabilistic AI Memory**: Provides the `AICortex` with deterministic,
//!    unambiguous episodic memory to eliminate stochastic hallucinations.
// ==============================================================================

pub mod failpoint;
pub mod inscriber;
pub mod query;
pub mod telemetry;
pub mod universal;

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Public Type Re-exports & DiagnosticLogger Structure Definition )
// ==============================================================================

// --- 1. Universal Clock & Event Bus Re-exports ---
pub use universal::{
    create_universal_telemetry_pair, generate_timestamp_id, parse_timestamp_id,
    DEFAULT_EVENT_BUS_CAPACITY, DEFAULT_FAILPOINT_BUS_CAPACITY, DEFAULT_MICROS_PER_TICK,
    DEFAULT_NOMINAL_CPU_MHZ, EventBus, EventBusMetrics, EventTapCallback, LogEvent,
    MICROS_PER_SEC, MonotonicClock, NANOS_PER_MICRO, SimulatedClock,
};

// --- 2. Failpoint Pathology & Dual-Rail Re-inception Re-exports ---
pub use failpoint::{
    is_dual_rail_halt, FailpointGroup, FailpointSubgroup, FailpointType, FixKind,
    FixSuggestion, HaltDumpFrame, HaltReason, ReInceptionTarget, SiliconRegisters,
};

// --- 3. Telemetry, RollFrames & Flight Recorder Re-exports ---
pub use telemetry::{
    CognitiveNote, FlightRecorder, GenesisBaseline, GenesisBootLog, RollFrame,
    DEFAULT_FLIGHT_RECORDER_CAPACITY,
};

// --- 4. Container Inscription & Rotation Policy Re-exports ---
pub use inscriber::{
    format_base_name, inscribe_audit_scroll, inscribe_canonical_scroll,
    inscribe_telemetry_log, parse_file_name, resolve_vessel_path, RotationPolicy,
};

// --- 5. Episodic Query Engine & Silicon Record Re-exports ---
pub use query::{
    create_query_engine_at, diagnose_session, query_session_by_id, EpisodicQueryEngine,
    IndexedSession, QueryResult, QuerySummary, SessionCriteria, Track0LogRecord,
};

/// Central thread-safe diagnostic logging, flight recording & cognitive memory engine.
///
/// Coordinates all five modular organs into a unified, high-performance facade:
/// - [`MonotonicClock`]: Enforces microsecond monotonic time sequencing.
/// - [`EventBus`]: Non-blocking, thread-safe broadcast router with multi-subscriber taps.
/// - [`FlightRecorder`]: Zero-allocation fixed circular ring buffer for crash analysis.
/// - [`CognitiveNote`]: Neural-symbolic episodic memory bank for the AI Cortex.
/// - [`GenesisBaseline`]: Hardware attestation state ($\Omega_0$) anchoring shutdown audits.
#[derive(Debug, Clone)]
pub struct DiagnosticLogger {
    clock: MonotonicClock,
    bus: EventBus,
    cognitive_notes: Arc<Mutex<Vec<CognitiveNote>>>,
    flight_recorder: Arc<Mutex<FlightRecorder>>,
    genesis_baseline: Arc<Mutex<Option<GenesisBaseline>>>,
}

impl DiagnosticLogger {
    /// Constructs a new [`DiagnosticLogger`] with the default flight recorder capacity (512 frames).
    pub fn new(print_live: bool) -> Self {
        Self::with_capacity(print_live, DEFAULT_FLIGHT_RECORDER_CAPACITY)
    }

    /// Constructs a new [`DiagnosticLogger`] with a customized flight recorder ring buffer capacity.
    pub fn with_capacity(print_live: bool, flight_recorder_capacity: usize) -> Self {
        Self {
            clock: MonotonicClock::new(),
            bus: EventBus::new(print_live),
            cognitive_notes: Arc::new(Mutex::new(Vec::new())),
            flight_recorder: Arc::new(Mutex::new(FlightRecorder::new(flight_recorder_capacity))),
            genesis_baseline: Arc::new(Mutex::new(None)),
        }
    }

    /// Returns a reference to the internal [`MonotonicClock`].
    pub fn clock(&self) -> &MonotonicClock {
        &self.clock
    }

    /// Returns a reference to the internal [`EventBus`].
    pub fn bus(&self) -> &EventBus {
        &self.bus
    }
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Core Logging Operations, Failpoint Trapping, Flight Recording & Query Facade )
// ==============================================================================

impl DiagnosticLogger {
    // --------------------------------------------------------------------------
    // 1. General & Failpoint Telemetry Operations
    // --------------------------------------------------------------------------

    /// Inscribes a normal debug or trace event into the diagnostic bus.
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

    /// Intercepts a critical failpoint event with concrete machine-actionable remediation telemetry.
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

    /// Intercepts an abrupt HALT event, emits a hardware crash dump over the bus,
    /// logs an episodic cognitive note tagged `#halt` and `#reinception`, and executes
    /// the safe-mode re-inception transition back to homeostasis (`0:AWAIT`).
    pub fn record_halt_dump(&self, frame: HaltDumpFrame) -> crate::core::ternary::QuadState {
        let elapsed = frame.tick_micros;
        let note = CognitiveNote {
            timestamp_micros: elapsed,
            category: "HALT_REINCEPTION".to_string(),
            summary: format!(
                "Abrupt HALT Trapped: {:?} at landmark {:#08x} -> Re-incepting to {}",
                frame.reason,
                frame.landmark_address,
                frame.re_inception_target.as_str()
            ),
            details: frame.registers.format_dump(),
            tags: vec![
                "halt".to_string(),
                "reinception".to_string(),
                "dual_rail_11".to_string(),
                "homeostasis".to_string(),
                "track0".to_string(),
            ],
            context_snippet: Some(format!("landmark: {:#08x}", frame.landmark_address)),
        };

        let event = LogEvent {
            elapsed_micros: elapsed,
            channel: "HALT",
            message: frame.format_human(),
            is_failpoint: true,
            failpoint_type: Some(frame.failpoint),
            suggestion: Some(FixSuggestion::hardware_reinception(
                frame.re_inception_target.address(),
                "Re-incept machine to safe landmark and restore homeostasis",
            )),
            cognitive_note: Some(note.clone()),
        };

        self.cognitive_notes.lock().unwrap().push(note);
        self.bus.emit(event);

        frame.execute_re_inception()
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

    /// Extracts all remediation suggestions currently registered on the bus.
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

    /// Clears all stored events, failpoints, cognitive memory notes, and genesis baseline.
    pub fn clear(&self) {
        self.bus.clear();
        self.cognitive_notes.lock().unwrap().clear();
        self.flight_recorder.lock().unwrap().clear();
        *self.genesis_baseline.lock().unwrap() = None;
    }

    // --------------------------------------------------------------------------
    // 2. Cognitive Thoughts & Associative Memory Anchors
    // --------------------------------------------------------------------------

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

    /// Records an associative memory link between an ontological concept and a biblical/manifest anchor.
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

    // --------------------------------------------------------------------------
    // 3. Inscription & Flight Recorder Extensions
    // --------------------------------------------------------------------------

    /// Logs the physical inscription of an episodic scroll to disk.
    pub fn log_scroll_inscribed(&self, scroll_path: &Path, session_or_id: &str) {
        self.log(
            "SCROLL",
            format!("Inscribed episodic scroll '{session_or_id}' to {:?}", scroll_path),
        );
    }

    /// Inscribes a canonical 4-Block `.scroll` file to the given path.
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

    /// Enforces the storehouse capacity rotation policy on a partition directory.
    pub fn enforce_rotation(
        &self,
        partition_dir: &Path,
        policy: RotationPolicy,
    ) -> Result<usize, String> {
        policy.enforce(partition_dir)
    }

    /// Filters cognitive memory notes by category.
    pub fn notes_by_category(&self, category: &str) -> Vec<CognitiveNote> {
        self.cognitive_notes
            .lock()
            .unwrap()
            .iter()
            .filter(|n| n.category == category)
            .cloned()
            .collect()
    }

    /// Filters cognitive memory notes containing a specific semantic tag.
    pub fn notes_by_tag(&self, tag: &str) -> Vec<CognitiveNote> {
        let tag_lower = tag.to_lowercase();
        self.cognitive_notes
            .lock()
            .unwrap()
            .iter()
            .filter(|n| n.tags.iter().any(|t| t.to_lowercase() == tag_lower))
            .cloned()
            .collect()
    }

    // --------------------------------------------------------------------------
    // 4. Episodic Query Subsystem Facade & Silicon Correlation
    // --------------------------------------------------------------------------

    /// Creates and populates an [`EpisodicQueryEngine`] pointing to `storehouse_root`.
    pub fn create_query_engine(&self, storehouse_root: &Path) -> EpisodicQueryEngine {
        let mut engine = EpisodicQueryEngine::new();
        let _ = engine.index_directory(storehouse_root);
        engine
    }

    /// Executes a compound query across indexed episodes using [`SessionCriteria`].
    pub fn query_episodes(
        &self,
        storehouse_root: &Path,
        criteria: &SessionCriteria,
    ) -> QueryResult {
        let engine = self.create_query_engine(storehouse_root);
        engine.query(criteria)
    }

    /// Queries an episode by ID, file stem, or alias in $O(1)$ time.
    pub fn find_session(
        &self,
        storehouse_root: &Path,
        session_id: &str,
    ) -> Option<IndexedSession> {
        let engine = self.create_query_engine(storehouse_root);
        engine.find_by_session(session_id).cloned()
    }

    /// Discovers, parses, and correlates Track 0 bare-metal silicon logs.
    pub fn correlate_track0_logs(
        &self,
        logs_dir: &Path,
    ) -> Result<Vec<Track0LogRecord>, String> {
        let mut engine = EpisodicQueryEngine::new();
        engine.index_track0_logs(logs_dir)?;
        Ok(engine.track0_records().to_vec())
    }

    /// Inspects and generates a deep diagnostic dossier for a specific session.
    pub fn diagnose_session_dossier(
        &self,
        storehouse_root: &Path,
        session_id: &str,
    ) -> Option<String> {
        let engine = self.create_query_engine(storehouse_root);
        engine.inspect_session_detail(session_id)
    }

    // --------------------------------------------------------------------------
    // 5. Genesis Inception Telemetry & Baseline Continuity (Omega_0)
    // --------------------------------------------------------------------------

    /// Ingests raw cold-boot telemetry emitted by Track 0 (BIOS MBR, Stage 2 PM, Long Mode, UEFI),
    /// parses hardware landmarks, stores primordial inception records into the baseline state Omega_0,
    /// and emits machine-readable cognitive notes for the AI Cortex.
    pub fn ingest_genesis_boot_log(&self, raw_serial_output: &str) -> Vec<GenesisBootLog> {
        let mut parsed_logs = Vec::new();

        for line in raw_serial_output.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            let elapsed = self.clock.elapsed_micros();

            // Detect memory landmark from telemetry string
            let landmark = if trimmed.contains("0x18000") || trimmed.contains("18000") {
                0x18000
            } else if trimmed.contains("0x108000") || trimmed.contains("108000") {
                0x108000
            } else if trimmed.contains("0x200000") || trimmed.contains("200000") {
                0x200000
            } else if trimmed.contains("0x7C00") || trimmed.contains("7C00") {
                0x7C00
            } else if trimmed.contains("0x0500") || trimmed.contains("0500") {
                0x0500
            } else {
                0x7C00
            };

            // Detect boot tier
            let tier = if trimmed.contains("Stage 2") || trimmed.contains("PM") || landmark == 0x18000 {
                "STAGE2_PM".to_string()
            } else if trimmed.contains("Long Mode") || trimmed.contains("64-Bit") || landmark == 0x108000 {
                "LONG_MODE".to_string()
            } else if trimmed.contains("UEFI") || trimmed.contains("GOP") {
                "UEFI".to_string()
            } else if trimmed.contains("Multiboot") || trimmed.contains("ELF") {
                "MULTIBOOT".to_string()
            } else {
                "BIOS_REAL_MODE".to_string()
            };

            // Detect invariant subsystem
            let subsystem = if trimmed.contains("ALU") || trimmed.contains("Eq") {
                "ALU_BALANCED".to_string()
            } else if trimmed.contains("REGS") || trimmed.contains("Regs") {
                "REGS_CALIBRATED".to_string()
            } else if trimmed.contains("A20") {
                "A20_GATE".to_string()
            } else if trimmed.contains("MEM") || trimmed.contains("Memory") {
                "MEMORY_MAP".to_string()
            } else if trimmed.contains("CPI-SI") || trimmed.contains("Trit") {
                "COGNITIVE_ARENA".to_string()
            } else {
                "GENESIS_BOOT".to_string()
            };

            // Verify clean invariant status
            let is_clean = !trimmed.contains("ERR")
                && !trimmed.contains("FAIL")
                && !trimmed.contains("VETO")
                && !trimmed.contains("FAULT");

            let genesis_log = GenesisBootLog::new(
                elapsed,
                landmark,
                tier,
                trimmed,
                subsystem.clone(),
                is_clean,
            );

            // Emit dual-readable event over bus
            let event = LogEvent {
                elapsed_micros: elapsed,
                channel: "BOOT",
                message: genesis_log.format_human(),
                is_failpoint: !is_clean,
                failpoint_type: if !is_clean { Some(FailpointType::HardwareFault) } else { None },
                suggestion: None,
                cognitive_note: None,
            };
            self.bus.emit(event);

            // Inscribe CognitiveNote for AI Cortex episodic memory
            let note = CognitiveNote {
                timestamp_micros: elapsed,
                category: "GENESIS_INCEPTION".to_string(),
                summary: format!("Genesis Inception [Landmark {:#08x}]: {}", landmark, subsystem),
                details: trimmed.to_string(),
                tags: vec![
                    "genesis".to_string(),
                    "silicon".to_string(),
                    "first_log".to_string(),
                    "baseline".to_string(),
                    "omega_0".to_string(),
                    subsystem.to_lowercase(),
                ],
                context_snippet: Some(format!("landmark: {:#08x}", landmark)),
            };
            self.cognitive_notes.lock().unwrap().push(note);

            parsed_logs.push(genesis_log);
        }

        // Lock in the GenesisBaseline (Omega_0)
        let elapsed = self.clock.elapsed_micros();
        let baseline = GenesisBaseline::new(elapsed, parsed_logs.clone());
        *self.genesis_baseline.lock().unwrap() = Some(baseline);

        parsed_logs
    }

    /// Records an individual pre-formed [`GenesisBootLog`] and appends it to the baseline.
    pub fn record_genesis_record(&self, log: GenesisBootLog) {
        let mut guard = self.genesis_baseline.lock().unwrap();
        if let Some(ref mut baseline) = *guard {
            if !log.is_clean {
                baseline.is_clean = false;
            }
            baseline.boot_logs.push(log);
        } else {
            let elapsed = self.clock.elapsed_micros();
            let is_clean = log.is_clean;
            *guard = Some(GenesisBaseline {
                locked_micros: elapsed,
                boot_logs: vec![log],
                is_clean,
            });
        }
    }

    /// Retrieves a cloned snapshot of the active [`GenesisBaseline`] (State Omega_0).
    pub fn genesis_baseline(&self) -> Option<GenesisBaseline> {
        self.genesis_baseline.lock().unwrap().clone()
    }

    /// Returns `true` if the genesis baseline has been ingested and verified clean.
    pub fn is_baseline_clean(&self) -> bool {
        self.genesis_baseline
            .lock()
            .unwrap()
            .as_ref()
            .map(|b| b.is_clean)
            .unwrap_or(false)
    }

    /// Audits whether the operating system state matches the homeostatic equilibrium of Omega_0.
    /// Certified before allowing ACPI shutdown, hibernation, or CPU suspension.
    pub fn audit_shutdown_equilibrium(&self) -> Result<(), String> {
        let guard = self.genesis_baseline.lock().unwrap();
        let baseline = match guard.as_ref() {
            Some(b) => b,
            None => {
                return Err("Shutdown audit veto: No Genesis Inception baseline (Omega_0) registered.".to_string());
            }
        };

        if !baseline.is_clean {
            return Err("Shutdown audit veto: Genesis baseline contains unhealed hardware inception faults.".to_string());
        }

        if self.has_failpoints() {
            let count = self.failpoints().len();
            return Err(format!(
                "Shutdown audit veto: System contains {} unhealed active failpoints; cannot return to Omega_0.",
                count
            ));
        }

        Ok(())
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Default Trait & Subsystem Verification Unit Test Suite )
// ==============================================================================

impl Default for DiagnosticLogger {
    /// Constructs a silent [`DiagnosticLogger`] with live terminal printing disabled.
    fn default() -> Self {
        Self::new(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use crate::core::ternary::QuadState;

    #[test]
    fn test_logger_lifecycle_and_monotonicity() {
        let logger = DiagnosticLogger::new(false);
        assert_eq!(logger.events().len(), 0);
        assert!(!logger.has_failpoints());

        logger.log("SYSTEM", "Initialization started");
        logger.log("SYSTEM", "Initialization completed");

        let events = logger.events();
        assert_eq!(events.len(), 2);
        assert!(events[1].elapsed_micros >= events[0].elapsed_micros);
    }

    #[test]
    fn test_logger_failpoint_trapping_and_remediations() {
        let logger = DiagnosticLogger::new(false);

        logger.failpoint(FailpointType::BoundaryMismatch, "LINTER", "Unclosed delimiter");
        assert!(logger.has_failpoints());
        assert_eq!(logger.failpoints().len(), 1);

        let suggestion = FixSuggestion::new(
            FixKind::FixBoundaryMismatch,
            10,
            15,
            "]",
            "}",
            "Balance closing morphological boundary",
        );
        logger.failpoint_with_remediation(
            FailpointType::BoundaryMismatch,
            "PARSER",
            "Delimiter mismatch corrected",
            suggestion,
        );

        assert_eq!(logger.failpoints().len(), 2);
        assert_eq!(logger.remediations().len(), 1);
        assert_eq!(logger.remediations()[0].suggested_patch, "}");
    }

    #[test]
    fn test_logger_flight_recorder_and_anomalous_frames() {
        let logger = DiagnosticLogger::with_capacity(false, 4);

        let frame1 = RollFrame::new(
            100,
            1,
            4,
            "INSCRIBE",
            (0x01, 0x00),
            (0x01, 0x00),
            QuadState::Proceed,
            0,
            None,
        );
        let frame2 = RollFrame::new(
            200,
            2,
            4,
            "TRANSPOSE",
            (0x02, 0x01),
            (0x00, 0x01),
            QuadState::Rollback,
            1,
            None,
        );

        logger.record_roll_frame(frame1);
        logger.record_roll_frame(frame2);

        let recent = logger.recent_roll_frames(2);
        assert_eq!(recent.len(), 2);

        let anomalous = logger.last_anomalous_frame().expect("Found anomaly");
        assert_eq!(anomalous.quad_state, QuadState::Rollback);
        assert_eq!(anomalous.failpoint_mask, 1);
    }

    #[test]
    fn test_logger_cognitive_notes_and_export() {
        let logger = DiagnosticLogger::new(false);

        let _note = logger.record_ai_observation(
            "SYNTAX",
            "Parsed canonical 4-block reel",
            "All delimiters balanced",
            &["syntax", "reel", "canonical"],
            Some("Opening { ... }"),
        );

        assert_eq!(logger.cognitive_notes().len(), 1);
        assert_eq!(logger.notes_by_category("SYNTAX").len(), 1);
        assert_eq!(logger.notes_by_tag("canonical").len(), 1);

        let scroll = logger.export_ai_memory_scroll();
        assert!(scroll.contains("CognitiveMemory"));
        assert!(scroll.contains("SYNTAX"));
    }

    #[test]
    fn test_logger_genesis_inception_and_shutdown_audit() {
        let logger = DiagnosticLogger::new(false);

        let raw_boot = r#"
            [BOOT] 16-Bit Genesis: Sector 0 loaded at 0x7C00
            [BOOT] Stage 2 PM: A20 Gate enabled, Flight Recorder armed at 0x18000
            [ALU ] Balanced Ternary (+1)+(-1)=0 Eq: Certified
            [REGS] 13 Regs Armed / Calibrated
        "#;

        let logs = logger.ingest_genesis_boot_log(raw_boot);
        assert_eq!(logs.len(), 4);
        assert!(logger.is_baseline_clean());
        assert!(logger.audit_shutdown_equilibrium().is_ok());

        // Introduce a failpoint; shutdown should be vetoed
        logger.failpoint(FailpointType::HardwareFault, "SILICON", "Bus voltage drift");
        assert!(logger.audit_shutdown_equilibrium().is_err());
    }

    #[test]
    fn test_logger_episodic_query_facade() {
        let temp_dir = std::env::temp_dir().join(format!(
            "omnicode_logger_facade_test_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&temp_dir).unwrap();

        let scroll_content = r#"
Metadata {
    session_id: "sess_facade_01",
    timestamp: "1792000000000000_0000_sess_facade_01",
    user_prompt: "Verify diagnostic logger facade",
}
Opening {
    Cell [ 1 ] & { 5 Trits :: 1 Byte },
}
Body {
    Cell [ 1 ] := 'X',
}
Closing {
    commit_status [ <PROCEED> ],
}
"#;
        let file_path = temp_dir.join("1792000000000000_0000_sess_facade_01.scroll");
        let mut file = fs::File::create(&file_path).unwrap();
        file.write_all(scroll_content.as_bytes()).unwrap();

        let logger = DiagnosticLogger::new(false);
        let session = logger.find_session(&temp_dir, "sess_facade_01");
        assert!(session.is_some());
        assert_eq!(session.unwrap().final_state, QuadState::Proceed);

        let criteria = SessionCriteria::new()
            .with_state(QuadState::Proceed)
            .with_keyword("facade");
        let query_res = logger.query_episodes(&temp_dir, &criteria);
        assert_eq!(query_res.sessions.len(), 1);
        assert_eq!(query_res.pass_rate(), 100.0);

        let _ = fs::remove_dir_all(&temp_dir);
    }
}

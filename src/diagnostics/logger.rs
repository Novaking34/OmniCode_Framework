// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Diagnostic Logger Identity, Failpoint Categories, Cognitive Notes & Enums )
// ==============================================================================
//! OmniCode Diagnostics: Structured Failpoint Logger & Cognitive Memory Engine
//!
//! # Purpose & Epistemic Role
//! Explicitly engineered to catch, classify, and report failpoints, boundary
//! violations, and state anomalies, while simultaneously generating structured
//! epistemic memory notes formatted for neural-symbolic AI models, autonomous
//! self-healing engines, and episodic replay verification.
//!
//! # IDE Hover & AI Agent Context
//! When inspecting or hovering over items in this module:
//! - [`FailpointType`]: Identifies the exact invariant violated (e.g. boundary leaks, null traps).
//! - [`FixKind`] & [`FixSuggestion`]: Carries concrete, machine-applicable repair telemetry.
//! - [`CognitiveNote`]: Encapsulates dual-readable episodic memory traces that an AI coding
//!   agent can ingest into its context window or persist into `.scroll` Turing reels.
//! - [`DiagnosticLogger`]: Thread-safe collector powering the self-healing feedback loop.

use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Classifications of trapped failpoints and boundary anomalies in the OmniCode runtime.
///
/// Unlike conventional exceptions or panics that abort process state, OmniCode treats
/// failpoints as first-class ontological diagnostic signals that trigger rollback
/// (`-1:ROLLBACK`), caveat vetos, or automated immune restoration (`+1:PROCEED`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FailpointType {
    /// Delimiter mismatch: opened with one delimiter class but closed with another (e.g. `< ... ]`).
    BoundaryMismatch,
    /// Unclosed boundary delimiter reaching End-of-File without a matching closing partner.
    BoundaryLeak,
    /// Execution attempted to evaluate or dereference a void/null state without a resolving caveat.
    NullTrap,
    /// An operation or caveat evaluation triggered a state rollback (`-1:ROLLBACK`).
    RollbackTriggered,
    /// An adversative caveat condition (`! [ ... ]`) vetoed an active state transition.
    CaveatVeto,
    /// Illegal attempt to mutate an immutable Rank 1 invariant axiom declared inside `< ... >`.
    ImmutableMutation,
    /// Recursive or cyclic retroaction loop exceeding the configured debugger depth threshold.
    CyclicRetroaction,
    /// Syntax, lexer, or structural grammar anomaly that violates language invariants.
    SyntaxAnomaly,
}

impl FailpointType {
    /// Returns the uppercase canonical string identifier for this failpoint category.
    ///
    /// # IDE Hover
    /// Used for telemetry, console logging headers, and `.scroll` failpoint records.
    pub fn as_str(&self) -> &'static str {
        match self {
            FailpointType::BoundaryMismatch => "FP_BOUNDARY_MISMATCH",
            FailpointType::BoundaryLeak => "FP_BOUNDARY_LEAK",
            FailpointType::NullTrap => "FP_NULL_TRAP",
            FailpointType::RollbackTriggered => "FP_ROLLBACK_TRIGGERED",
            FailpointType::CaveatVeto => "FP_CAVEAT_VETO",
            FailpointType::ImmutableMutation => "FP_IMMUTABLE_MUTATION",
            FailpointType::CyclicRetroaction => "FP_CYCLIC_RETROACTION",
            FailpointType::SyntaxAnomaly => "FP_SYNTAX_ANOMALY",
        }
    }
}

/// Categories of automated restorative repairs that the [`crate::diagnostics::restorator::Restorator`]
/// and AI agents can execute to heal code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FixKind {
    /// Append a missing closing boundary delimiter (e.g., closing an unclosed `[` with `]`).
    CloseBoundary,
    /// Substitute an erroneous closing delimiter with the delimiter matching the opening boundary.
    FixBoundaryMismatch,
    /// Inject missing canonical 4-Block architecture scaffolding headers (Metadata, Opening, Body, Closing).
    AddBlockHeader,
    /// Automatically inject missing core library imports (e.g., `Core.Arithmetic`, `Core.Logic`).
    AddImport,
    /// Synthesize and declare an inferred conceptual noun entity (e.g., `Ball < Entity? >`).
    DeclareEntityWithInfer,
    /// Modify or satisfy an adversative caveat expression preventing state commitment.
    AdjustCaveat,
    /// Convert an immutable Rank 1 invariant `< ... >` into a mutable Rank 2 denotation `[ ... ]`.
    ConvertImmutableToMutable,
}

/// Concrete remediation telemetry attached to a failpoint event.
///
/// Contains the exact coordinate, token, and replacement text required to surgically
/// heal source code without manual human intervention.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixSuggestion {
    /// Categorical classification of the automated remediation.
    pub kind: FixKind,
    /// 1-based source line number where the remediation should be applied.
    pub line: usize,
    /// 1-based source column number where the target token resides.
    pub col: usize,
    /// The erroneous or missing token sequence targeted for replacement.
    pub target_token: String,
    /// The proposed patch or substitution string that seals the invariant.
    pub suggested_patch: String,
    /// Human-readable and AI-interpretable rationale explaining *why* this fix is sound.
    pub rationale: String,
}

/// Structured epistemic memory note formatted for neural-symbolic AI models.
///
/// # Why Cognitive Notes Matter for AI Agents & IDE Tooltips
/// When an AI model or developer inspects code execution, raw text logs lack semantic
/// structure and cause context drift or hallucination. [`CognitiveNote`] structures
/// telemetry into discrete, indexable memory units:
/// - Category: The epistemic role of the observation (`OBSERVATION`, `HYPOTHESIS`, `INCIDENT`).
/// - Summary & Details: Precise descriptions of what occurred and why.
/// - Tags: Semantic taxonomy labels (`#boundary_physics`, `#failpoint`, `#self_healing`).
/// - Context Snippet: The exact line or expression that triggered the note.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CognitiveNote {
    /// Monotonic timestamp in microseconds since logger initialization.
    pub timestamp_micros: u128,
    /// Epistemic category: e.g. "OBSERVATION", "HYPOTHESIS", "INCIDENT", "REPAIR_VERIFIED".
    pub category: String,
    /// High-level single-sentence summary of the cognitive observation.
    pub summary: String,
    /// In-depth architectural details, telemetry data, or remediation rationale.
    pub details: String,
    /// Semantic taxonomy tags for associative memory indexing.
    pub tags: Vec<String>,
    /// Optional code snippet or token context under active observation.
    pub context_snippet: Option<String>,
}

impl CognitiveNote {
    /// Formats this note as an OmniCode native dual-readable syntax block.
    ///
    /// # Output Example
    /// ```omnicode
    /// Note < CognitiveTrace > {
    ///     tick: < 1240 µs >,
    ///     category: [ INCIDENT ],
    ///     summary: < "Boundary mismatch" >,
    ///     details: < "Mismatched closing delimiter" >,
    ///     tags: [ #boundary_physics :: #self_healing ]
    /// }
    /// ```
    pub fn format_as_omnicode(&self) -> String {
        let tag_str = if self.tags.is_empty() {
            "[]".to_string()
        } else {
            format!("[ {} ]", self.tags.iter().map(|t| format!("#{t}")).collect::<Vec<_>>().join(" :: "))
        };
        let ctx_str = if let Some(c) = &self.context_snippet {
            format!("\n    context: < {:?} >", c)
        } else {
            String::new()
        };
        format!(
            "Note < CognitiveTrace > {{\n    tick: < {} µs >,\n    category: [ {} ],\n    summary: < {:?} >,\n    details: < {:?} >,\n    tags: {}{}\n}}",
            self.timestamp_micros, self.category, self.summary, self.details, tag_str, ctx_str
        )
    }

    /// Formats this note as a Markdown block specifically structured for LLM context prompts
    /// and episodic memory bank ingestion.
    pub fn format_as_memory_entry(&self) -> String {
        let tag_list = if self.tags.is_empty() {
            "none".to_string()
        } else {
            self.tags.iter().map(|t| format!("#{t}")).collect::<Vec<_>>().join(", ")
        };
        let mut out = format!(
            "### [EPISODIC_MEMORY_TRACE: {}]\n- **Timestamp:** {} µs\n- **Summary:** {}\n- **Details:** {}\n- **Tags:** {}\n",
            self.category, self.timestamp_micros, self.summary, self.details, tag_list
        );
        if let Some(ctx) = &self.context_snippet {
            out.push_str(&format!("- **Context:** `{}`\n", ctx));
        }
        out
    }
}

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( LogEvent Structure and DiagnosticLogger Initialization )
// ==============================================================================
/// Individual diagnostic event recorded during compiler passes or execution ticks.
#[derive(Debug, Clone)]
pub struct LogEvent {
    /// Microseconds elapsed since the logger instance was created.
    pub elapsed_micros: u128,
    /// Channel or subsystem tag: e.g. "BOUNDARY", "DEBUGGER", "TERNARY", "IMMUTABILITY".
    pub channel: &'static str,
    /// Descriptive message explaining the operation or signal.
    pub message: String,
    /// True if this event represents an intercepted failpoint or invariant violation.
    pub is_failpoint: bool,
    /// The specific failpoint category if `is_failpoint` is true.
    pub failpoint_type: Option<FailpointType>,
    /// Concrete remediation suggestion if an automated repair is available.
    pub suggestion: Option<FixSuggestion>,
    /// Accompanying structured cognitive memory note for AI ingestion.
    pub cognitive_note: Option<CognitiveNote>,
}

/// Thread-safe diagnostic logging and cognitive memory collection engine.
///
/// Coordinates execution event tracing, failpoint interception, remediation suggestions,
/// and AI memory scroll generation across lexer, parser, evaluator, and self-healing passes.
#[derive(Debug, Clone)]
pub struct DiagnosticLogger {
    /// Reference instant used to compute microsecond elapsed offsets.
    start_time: Instant,
    /// Chronological sequence of all recorded diagnostic events.
    events: Arc<Mutex<Vec<LogEvent>>>,
    /// Filtered subset containing exclusively failpoint events.
    failpoints: Arc<Mutex<Vec<LogEvent>>>,
    /// Episodic memory notes formatted for neural-symbolic AI models.
    cognitive_notes: Arc<Mutex<Vec<CognitiveNote>>>,
    /// When true, outputs color-coded diagnostic lines to stdout/stderr in real time.
    print_live: bool,
}

impl DiagnosticLogger {
    /// Constructs a new [`DiagnosticLogger`].
    ///
    /// # Arguments
    /// * `print_live` - If true, logs are immediately mirrored to stdout/stderr.
    pub fn new(print_live: bool) -> Self {
        Self {
            start_time: Instant::now(),
            events: Arc::new(Mutex::new(Vec::new())),
            failpoints: Arc::new(Mutex::new(Vec::new())),
            cognitive_notes: Arc::new(Mutex::new(Vec::new())),
            print_live,
        }
    }
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Core Logging Operations, Cognitive Notes, Interceptions & Export )
// ==============================================================================
impl DiagnosticLogger {
    /// Inscribes a normal debug or trace event into the diagnostic log.
    ///
    /// # Arguments
    /// * `channel` - The subsystem identifier (e.g. "PARSER", "BOUNDARY", "TERNARY").
    /// * `message` - The informational message content.
    pub fn log(&self, channel: &'static str, message: impl Into<String>) {
        let msg = message.into();
        let elapsed = self.start_time.elapsed().as_micros();
        let event = LogEvent {
            elapsed_micros: elapsed,
            channel,
            message: msg.clone(),
            is_failpoint: false,
            failpoint_type: None,
            suggestion: None,
            cognitive_note: None,
        };

        if self.print_live {
            println!("[{:>6} µs] DEBUG    [{}] {}", elapsed, channel, msg);
        }

        self.events.lock().unwrap().push(event);
    }

    /// Intercepts and records a critical failpoint event without remediation telemetry.
    ///
    /// Automatically synthesizes an accompanying [`CognitiveNote`] with category `INCIDENT`.
    pub fn failpoint(&self, fp_type: FailpointType, channel: &'static str, message: impl Into<String>) {
        let msg = message.into();
        let elapsed = self.start_time.elapsed().as_micros();
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
            message: msg.clone(),
            is_failpoint: true,
            failpoint_type: Some(fp_type),
            suggestion: None,
            cognitive_note: Some(note.clone()),
        };

        if self.print_live {
            eprintln!(
                "[{:>6} µs] \x1b[91mCRITICAL\x1b[0m [{}] [FAILPOINT: {}] {}",
                elapsed,
                channel,
                fp_type.as_str(),
                msg
            );
        }

        self.cognitive_notes.lock().unwrap().push(note);
        self.failpoints.lock().unwrap().push(event.clone());
        self.events.lock().unwrap().push(event);
    }

    /// Intercepts a critical failpoint event with concrete remediation telemetry.
    ///
    /// Equips the event with a [`FixSuggestion`] and registers an episodic [`CognitiveNote`]
    /// marked `#remediation_ready` so the Restorator or AI can immediately apply the repair.
    pub fn failpoint_with_remediation(
        &self,
        fp_type: FailpointType,
        channel: &'static str,
        message: impl Into<String>,
        suggestion: FixSuggestion,
    ) {
        let msg = message.into();
        let elapsed = self.start_time.elapsed().as_micros();
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
            message: msg.clone(),
            is_failpoint: true,
            failpoint_type: Some(fp_type),
            suggestion: Some(suggestion),
            cognitive_note: Some(note.clone()),
        };

        if self.print_live {
            eprintln!(
                "[{:>6} µs] \x1b[91mCRITICAL\x1b[0m [{}] [FAILPOINT: {}] {} (Remediation: {:?})",
                elapsed,
                channel,
                fp_type.as_str(),
                msg,
                event.suggestion
            );
        }

        self.cognitive_notes.lock().unwrap().push(note);
        self.failpoints.lock().unwrap().push(event.clone());
        self.events.lock().unwrap().push(event);
    }

    /// Records an arbitrary cognitive observation or hypothesis into episodic memory.
    ///
    /// # Arguments
    /// * `category` - Epistemic label (e.g. "OBSERVATION", "HYPOTHESIS", "ORACLE_SUCCESS").
    /// * `summary` - One-line summary for rapid cognitive indexing.
    /// * `details` - Comprehensive description or reasoning trace.
    /// * `tags` - Semantic search and association tags.
    /// * `context` - Optional code snippet or token reference under study.
    pub fn record_ai_observation(
        &self,
        category: &str,
        summary: &str,
        details: &str,
        tags: &[&str],
        context: Option<&str>,
    ) -> CognitiveNote {
        let elapsed = self.start_time.elapsed().as_micros();
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

    /// Inscribes a pre-formed [`CognitiveNote`] directly into the cognitive notes buffer.
    pub fn log_cognitive_note(&self, note: CognitiveNote) {
        self.cognitive_notes.lock().unwrap().push(note);
    }

    /// Retrieves a cloned snapshot of all recorded cognitive memory notes.
    pub fn cognitive_notes(&self) -> Vec<CognitiveNote> {
        self.cognitive_notes.lock().unwrap().clone()
    }

    /// Exports all recorded cognitive notes into a canonical 4-Block OmniCode memory scroll.
    ///
    /// This scroll can be saved directly as a `.scroll` artifact or fed into the AI episodic
    /// memory learner for continuous model evolution.
    pub fn export_ai_memory_scroll(&self) -> String {
        let notes = self.cognitive_notes();
        let mut out = String::new();
        out.push_str("# ==============================================================================\n");
        out.push_str("# BLOCK 1: METADATA BLOCK\n");
        out.push_str("# ( Episodic AI Cognitive Memory Scroll & Incident Audit Log )\n");
        out.push_str("# ==============================================================================\n");
        out.push_str("Scroll < CognitiveMemory.Audit > {\n");
        out.push_str(&format!("    total_notes: < {} >,\n", notes.len()));
        out.push_str(&format!("    total_failpoints: < {} >\n", self.failpoints.lock().unwrap().len()));
        out.push_str("}\n\n");
        out.push_str("# ==============================================================================\n");
        out.push_str("# BLOCK 2: OPENING BLOCK\n");
        out.push_str("# ( Cognitive Context & Invariant Preconditions )\n");
        out.push_str("# ==============================================================================\n");
        out.push_str("Opening {\n    preconditions: [ +1 ]\n}\n\n");
        out.push_str("# ==============================================================================\n");
        out.push_str("# BLOCK 3: BODY BLOCK\n");
        out.push_str("# ( Episodic Traces & Memory Units Recorded During Execution )\n");
        out.push_str("# ==============================================================================\n");
        out.push_str("Body [\n");
        for n in &notes {
            out.push_str(&format!("    {}\n\n", n.format_as_omnicode().replace('\n', "\n    ")));
        }
        out.push_str("]\n\n");
        out.push_str("# ==============================================================================\n");
        out.push_str("# BLOCK 4: CLOSING BLOCK\n");
        out.push_str("# ( Cognitive Audit Verification Proof )\n");
        out.push_str("# ==============================================================================\n");
        out.push_str("Closing {\n    audit_verified: [ PROCEED ]\n}\n");
        out
    }

    /// Retrieves a cloned snapshot of all intercepted failpoint events.
    pub fn failpoints(&self) -> Vec<LogEvent> {
        self.failpoints.lock().unwrap().clone()
    }

    /// Retrieves a cloned snapshot of all general log events.
    pub fn events(&self) -> Vec<LogEvent> {
        self.events.lock().unwrap().clone()
    }

    /// Extracts all remediation suggestions currently registered across failpoints.
    pub fn remediations(&self) -> Vec<FixSuggestion> {
        self.failpoints
            .lock()
            .unwrap()
            .iter()
            .filter_map(|e| e.suggestion.clone())
            .collect()
    }

    /// Returns `true` if any failpoints were trapped during the execution session.
    pub fn has_failpoints(&self) -> bool {
        !self.failpoints.lock().unwrap().is_empty()
    }

    /// Clears all stored events, failpoints, and cognitive memory notes.
    pub fn clear(&self) {
        self.events.lock().unwrap().clear();
        self.failpoints.lock().unwrap().clear();
        self.cognitive_notes.lock().unwrap().clear();
    }
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Default Trait Implementation and Teardown )
// ==============================================================================
impl Default for DiagnosticLogger {
    /// Constructs a silent [`DiagnosticLogger`] with live terminal printing disabled.
    fn default() -> Self {
        Self::new(false)
    }
}

// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Diagnostic Logger Identity, Failpoint Categories, Cognitive Notes & Enums )
// ==============================================================================
//! OmniCode Diagnostics: Structured Failpoint Logger & Cognitive Memory Engine
//!
//! Explicitly engineered to catch and report failpoints, boundary violations,
//! and state anomalies, while generating structured epistemic memory notes
//! formatted for neural-symbolic AI models, autonomous self-healing engines,
//! and episodic replay verification.

use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FailpointType {
    BoundaryMismatch,
    BoundaryLeak,
    NullTrap,
    RollbackTriggered,
    CaveatVeto,
    ImmutableMutation,
    CyclicRetroaction,
    SyntaxAnomaly,
}

impl FailpointType {
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

/// Categories of automated fixes that the Restorator and AI can apply.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FixKind {
    CloseBoundary,
    FixBoundaryMismatch,
    AddBlockHeader,
    AddImport,
    DeclareEntityWithInfer,
    AdjustCaveat,
    ConvertImmutableToMutable,
}

/// Concrete remediation telemetry attached to a failpoint event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixSuggestion {
    pub kind: FixKind,
    pub line: usize,
    pub col: usize,
    pub target_token: String,
    pub suggested_patch: String,
    pub rationale: String,
}

/// Structured epistemic memory note formatted for neural-symbolic AI models,
/// allowing autonomous agents to record observations, hypotheses, and repair histories.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CognitiveNote {
    pub timestamp_micros: u128,
    pub category: String, // e.g. "OBSERVATION", "HYPOTHESIS", "INCIDENT", "REMEDIAL_ACTION"
    pub summary: String,
    pub details: String,
    pub tags: Vec<String>,
    pub context_snippet: Option<String>,
}

impl CognitiveNote {
    /// Formats this note as an OmniCode native dual-readable syntax block.
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

    /// Formats this note as a Markdown block specifically structured for LLM context prompts and memory ingestion.
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
#[derive(Debug, Clone)]
pub struct LogEvent {
    pub elapsed_micros: u128,
    pub channel: &'static str,
    pub message: String,
    pub is_failpoint: bool,
    pub failpoint_type: Option<FailpointType>,
    pub suggestion: Option<FixSuggestion>,
    pub cognitive_note: Option<CognitiveNote>,
}

#[derive(Debug, Clone)]
pub struct DiagnosticLogger {
    start_time: Instant,
    events: Arc<Mutex<Vec<LogEvent>>>,
    failpoints: Arc<Mutex<Vec<LogEvent>>>,
    cognitive_notes: Arc<Mutex<Vec<CognitiveNote>>>,
    print_live: bool,
}

impl DiagnosticLogger {
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

    /// Logs an existing cognitive note.
    pub fn log_cognitive_note(&self, note: CognitiveNote) {
        self.cognitive_notes.lock().unwrap().push(note);
    }

    /// Returns a snapshot copy of all recorded cognitive memory notes.
    pub fn cognitive_notes(&self) -> Vec<CognitiveNote> {
        self.cognitive_notes.lock().unwrap().clone()
    }

    /// Exports all cognitive notes as an OmniCode dual-readable episodic memory scroll.
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

    pub fn failpoints(&self) -> Vec<LogEvent> {
        self.failpoints.lock().unwrap().clone()
    }

    pub fn events(&self) -> Vec<LogEvent> {
        self.events.lock().unwrap().clone()
    }

    pub fn remediations(&self) -> Vec<FixSuggestion> {
        self.failpoints
            .lock()
            .unwrap()
            .iter()
            .filter_map(|e| e.suggestion.clone())
            .collect()
    }

    pub fn has_failpoints(&self) -> bool {
        !self.failpoints.lock().unwrap().is_empty()
    }

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
    fn default() -> Self {
        Self::new(false)
    }
}

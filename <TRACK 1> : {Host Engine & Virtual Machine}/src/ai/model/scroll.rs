// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Scroll Engine Identity, Dual Format Invariants & Memory Manifest )
// ==============================================================================
//! OmniCode AI Model: Episodic Scroll & Telemetry Logger
//!
//! Generates dual formal log outputs for every cognitive session:
//!     1. Traditional Code Log (`.log`): Microsecond timing, channel logs, and telemetry.
//!     2. OmniCode Scroll (`.scroll`): Native executable 4-Block Turing reel recording
//!        the complete episodic memory, causal actions, caveats, and natural responses.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use crate::core::ternary::QuadState;
use crate::diagnostics::logger::{FixSuggestion, LogEvent};
use crate::diagnostics::restorator::AppliedRepair;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Scroll Data Structures and Context Initialization )
// ==============================================================================
#[derive(Debug, Clone)]
pub struct ExecutionScroll {
    pub session_id: String,
    pub timestamp_str: String,
    pub prompt: String,
    pub synthesized_omnicode: String,
    pub execution_state: QuadState,
    pub natural_response: String,
    pub failpoints: Vec<LogEvent>,
    pub repairs_applied: Vec<AppliedRepair>,
    pub remediations_suggested: Vec<FixSuggestion>,
    pub elapsed_micros: u128,
}

#[derive(Debug, Clone)]
pub struct ScrollLogger {
    pub scroll_dir: PathBuf,
}

impl ScrollLogger {
    pub fn new(scroll_dir: impl AsRef<Path>) -> Self {
        let dir = scroll_dir.as_ref().to_path_buf();
        if !dir.exists() {
            fs::create_dir_all(&dir).ok();
        }
        Self { scroll_dir: dir }
    }
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Core Inscription Pipeline: OmniCode .scroll and Traditional .log Generation )
// ==============================================================================
impl ScrollLogger {
    /// Inscribes both a .scroll and a .log file to disk, returning their paths.
    pub fn save_dual_logs(&self, scroll: &ExecutionScroll) -> Result<(PathBuf, PathBuf), String> {
        if !self.scroll_dir.exists() {
            fs::create_dir_all(&self.scroll_dir)
                .map_err(|e| format!("Failed to create scroll directory: {e}"))?;
        }

        let base_name = format!("{}_{}", scroll.timestamp_str, scroll.session_id);
        let scroll_path = self.scroll_dir.join(format!("{base_name}.scroll"));
        let log_path = self.scroll_dir.join(format!("{base_name}.log"));

        let scroll_content = self.generate_omni_scroll(scroll);
        let log_content = self.generate_traditional_log(scroll);

        fs::write(&scroll_path, &scroll_content)
            .map_err(|e| format!("Failed to write scroll to {:?}: {e}", scroll_path))?;

        fs::write(&log_path, &log_content)
            .map_err(|e| format!("Failed to write log to {:?}: {e}", log_path))?;

        Ok((scroll_path, log_path))
    }

    /// Generates native, executable 4-Block OmniCode .scroll format.
    pub fn generate_omni_scroll(&self, s: &ExecutionScroll) -> String {
        let mut repairs_block = String::new();
        if s.repairs_applied.is_empty() {
            repairs_block.push_str("    # Immune System: Clean execution; zero Restorator repairs required.\n");
        } else {
            repairs_block.push_str("    # Immune System Interventions (Restorator):\n");
            for r in &s.repairs_applied {
                repairs_block.push_str(&format!(
                    "    repair_entry [ L{}: {:?} --> \"{}\" ]\n",
                    r.line, r.kind, r.description
                ));
            }
        }

        let mut failpoints_block = String::new();
        if s.failpoints.is_empty() {
            failpoints_block.push_str("    # Sensory Diagnostics: 0 trapped failpoints.\n");
        } else {
            failpoints_block.push_str("    # Trapped Diagnostic Failpoints:\n");
            for fp in &s.failpoints {
                let kind = fp.failpoint_type.map(|t| t.as_str()).unwrap_or("FP_GENERAL");
                failpoints_block.push_str(&format!(
                    "    failpoint_record [ {kind} : \"{}\" ]\n",
                    fp.message
                ));
            }
        }

        format!(
r#"# ==============================================================================
# BLOCK 1: METADATA BLOCK (THE COGNITIVE SCROLL)
# ( Execution Trace ID, Prompt Intent, Inferred Types, and Neural Snapshot )
# ==============================================================================
Scroll < EpistemicTrace.{session} > ( {prompt} ) {{
    version: < "1.0.0" >,
    system: < OmniCode.AI.Model >,
    timestamp: < "{timestamp}" >,
    session_id: < "{session}" >,
    execution_duration_micros: < {micros} >,
    cognitive_mode: < NeuralSymbolicTransduction >
}}

# ==============================================================================
# BLOCK 2: OPENING BLOCK (EXPERIENCE INITIALIZATION)
# ( Hardware Invariants, Preconditions, and Initial Context )
# ==============================================================================
Opening {{
    import: [ Core.Arithmetic :: Core.Logic :: Core.Vectors ],
    preconditions: [ +1 ]
}}

# Original Cognitive Discourse Prompt:
user_prompt < "{prompt}" >

# ==============================================================================
# BLOCK 3: BODY BLOCK (CHRONICLE OF EXECUTION & RESTORATION)
# ( Replay Tape: Synthesized Program, Sensory Telemetry & Immune Interventions )
# ==============================================================================
Body [
{repairs}
{failpoints}
    # Executed Turing Reel Synthesized by AI Model:
    synthesized_reel [
{reel_indented}
    ]
]

# ==============================================================================
# BLOCK 4: CLOSING BLOCK (EPISODIC COMMIT & LEARNING PROOF)
# ( Verification Proof, Final Quad-State, and Conversational Synthesis )
# ==============================================================================
Closing {{
    commit_status [ <{state}> ],
    machine_utterance ( "{response}" )
}}
"#,
            session = s.session_id,
            prompt = s.prompt,
            timestamp = s.timestamp_str,
            micros = s.elapsed_micros,
            repairs = repairs_block.trim_end(),
            failpoints = failpoints_block.trim_end(),
            reel_indented = indent_text(&s.synthesized_omnicode, "        "),
            state = s.execution_state.macro_op(),
            response = s.natural_response.replace('"', "\\\"")
        )
    }

    /// Generates traditional developer/telemetry .log format.
    pub fn generate_traditional_log(&self, s: &ExecutionScroll) -> String {
        let mut out = String::new();
        out.push_str("================================================================================\n");
        out.push_str("OMNICODE AI MODEL EXECUTION LOG (TRADITIONAL TELEMETRY)\n");
        out.push_str("================================================================================\n");
        out.push_str(&format!("Timestamp:    {}\n", s.timestamp_str));
        out.push_str(&format!("Session ID:   {}\n", s.session_id));
        out.push_str(&format!("Duration:     {} µs\n", s.elapsed_micros));
        out.push_str(&format!("Final State:  +1:{}\n", s.execution_state.macro_op()));
        out.push_str(&format!("Discourse:    \"{}\"\n", s.prompt));
        out.push_str("--------------------------------------------------------------------------------\n\n");

        out.push_str("[1. SENSORY LOG EVENTS & FAILPOINTS]\n");
        if s.failpoints.is_empty() {
            out.push_str("  • (No failpoints trapped. Sensory trace clean.)\n");
        } else {
            for (i, fp) in s.failpoints.iter().enumerate() {
                let kind = fp.failpoint_type.map(|t| t.as_str()).unwrap_or("GENERAL");
                out.push_str(&format!("  [{:>4}] [{} µs] [{}] [{}] {}\n", i + 1, fp.elapsed_micros, fp.channel, kind, fp.message));
                if let Some(ref sug) = fp.suggestion {
                    out.push_str(&format!("         -> Remediation Proposal: [{:?}] L{}: {}\n", sug.kind, sug.line, sug.rationale));
                }
            }
        }
        out.push('\n');

        out.push_str("[2. IMMUNE SYSTEM RESTORATIONS]\n");
        if s.repairs_applied.is_empty() {
            out.push_str("  • (Zero repairs required; code conformed to all boundary and block invariants.)\n");
        } else {
            for (i, r) in s.repairs_applied.iter().enumerate() {
                out.push_str(&format!("  [{:>2}] L{}: [{:?}] {}\n", i + 1, r.line, r.kind, r.description));
            }
        }
        out.push('\n');

        out.push_str("[3. SYNTHESIZED TURING REEL (OMNICODE)]\n");
        out.push_str(&s.synthesized_omnicode);
        if !s.synthesized_omnicode.ends_with('\n') {
            out.push('\n');
        }
        out.push('\n');

        out.push_str("[4. MACHINE DISCOURSE RESPONSE]\n");
        out.push_str(&format!("  \"{}\"\n", s.natural_response));
        out.push_str("================================================================================\n");

        out
    }
}

fn indent_text(s: &str, indent: &str) -> String {
    s.lines()
        .map(|line| format!("{indent}{line}"))
        .collect::<Vec<_>>()
        .join("\n")
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Helper Timestamp Utilities and Default Trait Sealing )
// ==============================================================================
pub fn generate_timestamp_id() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let count = COUNTER.fetch_add(1, Ordering::Relaxed);
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros();
    format!("{now}_{count:04}")
}

impl Default for ScrollLogger {
    fn default() -> Self {
        let dir = crate::core::filetype::resolve_scroll_storage_dir();
        Self::new(dir)
    }
}

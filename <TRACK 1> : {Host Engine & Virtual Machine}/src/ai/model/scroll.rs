// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( Scroll Engine Identity, Dual Format Invariants & Memory Manifest )
// ============================================================================
//! OmniCode AI Model: Episodic Scroll & Telemetry Logger (`scroll.rs`)
//!
//! Generates dual formal log outputs for every cognitive session:
//!     1. Traditional Code Log (`.log`): Microsecond timing, channel logs,
//!        and telemetry.
//!     2. OmniCode Scroll (`.scroll`): Native executable 4-Block Turing reel
//!        recording episodic memory, causal actions, modal domains, and
//!        scriptural anchors.
//!
//! ### Key Invariants:
//! 1. **Dual Inscription**: Every episode generates both `.scroll` and `.log`.
//! 2. **Modal Domain Grounding**: Block 1 of `.scroll` records modal domain,
//!    category, and scripture anchor when available.
//! 3. **Storehouse Capacity**: Storehouse rotation policy prevents disk leaks.
//! 4. **Strict Line Length**: Every line must be <= 80 characters.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::core::ternary::QuadState;
use crate::diagnostics::logger::{FixSuggestion, LogEvent, RotationPolicy};
use crate::diagnostics::restorator::AppliedRepair;

// ============================================================================
// BLOCK 2: TYPES BLOCK
// ( Scroll Data Structures, Audit Manifests & Context Initialization )
// ============================================================================

/// Complete episodic execution record submitted for dual disk persistence.
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
    pub modal_domain: Option<String>,
    pub domain_category: Option<String>,
    pub biblical_anchor: Option<String>,
}

impl Default for ExecutionScroll {
    fn default() -> Self {
        Self {
            session_id: String::new(),
            timestamp_str: String::new(),
            prompt: String::new(),
            synthesized_omnicode: String::new(),
            execution_state: QuadState::Proceed,
            natural_response: String::new(),
            failpoints: Vec::new(),
            repairs_applied: Vec::new(),
            remediations_suggested: Vec::new(),
            elapsed_micros: 0,
            modal_domain: None,
            domain_category: None,
            biblical_anchor: None,
        }
    }
}

/// Audit report validating storehouse health and inscription readiness.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrollSubsystemAudit {
    pub status: QuadState,
    pub scroll_dir_exists: bool,
    pub scroll_dir_writable: bool,
    pub rotation_max_files: usize,
    pub errors: Vec<String>,
}

/// Disk persistence engine for writing dual `.scroll` and `.log` archives.
#[derive(Debug, Clone)]
pub struct ScrollLogger {
    pub scroll_dir: PathBuf,
    pub rotation_policy: RotationPolicy,
}

impl ScrollLogger {
    /// Constructs a new [`ScrollLogger`] targeting the given directory.
    pub fn new(scroll_dir: impl AsRef<Path>) -> Self {
        let dir = scroll_dir.as_ref().to_path_buf();
        if !dir.exists() {
            fs::create_dir_all(&dir).ok();
        }
        Self {
            scroll_dir: dir,
            rotation_policy: RotationPolicy::default(),
        }
    }

    /// Constructs a new [`ScrollLogger`] with a custom rotation policy.
    pub fn with_rotation_policy(
        scroll_dir: impl AsRef<Path>,
        policy: RotationPolicy,
    ) -> Self {
        let dir = scroll_dir.as_ref().to_path_buf();
        if !dir.exists() {
            fs::create_dir_all(&dir).ok();
        }
        Self {
            scroll_dir: dir,
            rotation_policy: policy,
        }
    }
}

// ============================================================================
// BLOCK 3: BODY BLOCK
// ( Core Inscription Pipeline: OmniCode .scroll and Traditional .log )
// ============================================================================

impl ScrollLogger {
    /// Inscribes both a .scroll and a .log file to disk, returning their paths.
    pub fn save_dual_logs(
        &self,
        scroll: &ExecutionScroll,
    ) -> Result<(PathBuf, PathBuf), String> {
        if !self.scroll_dir.exists() {
            fs::create_dir_all(&self.scroll_dir)
                .map_err(|e| format!("Failed to create scroll dir: {e}"))?;
        }

        let (scroll_label, log_label, year, month, day) =
            generate_label_id(scroll);

        let date_rel = format!("{year:04}/{month:02}/{day:02}");
        let scroll_dest = self.scroll_dir.join("scroll").join(&date_rel);
        let log_dest = self.scroll_dir.join("log").join(&date_rel);

        if !scroll_dest.exists() {
            fs::create_dir_all(&scroll_dest).map_err(|e| {
                format!("Failed to create scroll subfolder: {e}")
            })?;
        }
        if !log_dest.exists() {
            fs::create_dir_all(&log_dest).map_err(|e| {
                format!("Failed to create log subfolder: {e}")
            })?;
        }

        let scroll_path =
            scroll_dest.join(format!("{scroll_label}.scroll"));
        let log_path = log_dest.join(format!("{log_label}.log"));

        let scroll_content = self.generate_omni_scroll(scroll);
        let log_content = self.generate_traditional_log(scroll);

        fs::write(&scroll_path, &scroll_content).map_err(|e| {
            format!("Failed to write scroll to {:?}: {e}", scroll_path)
        })?;

        fs::write(&log_path, &log_content).map_err(|e| {
            format!("Failed to write log to {:?}: {e}", log_path)
        })?;

        // Enforce storehouse vessel capacity invariant: Occupancy <= Capacity
        self.rotation_policy.enforce(&scroll_dest).ok();
        self.rotation_policy.enforce(&log_dest).ok();

        Ok((scroll_path, log_path))
    }

    /// Enforces the configured rotation policy across all partitions.
    pub fn rotate_storehouse(&self) -> Result<usize, String> {
        let mut total_rotated = 0;
        for sub in &["scroll", "log"] {
            let sub_dir = self.scroll_dir.join(sub);
            if sub_dir.exists() {
                self.enforce_recursive(&sub_dir, &mut total_rotated)?;
            } else {
                total_rotated += self.rotation_policy
                    .enforce(&self.scroll_dir)?;
            }
        }
        Ok(total_rotated)
    }

    fn enforce_recursive(
        &self,
        dir: &Path,
        count: &mut usize,
    ) -> Result<(), String> {
        if let Ok(entries) = fs::read_dir(dir) {
            let mut has_subdirs = false;
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() && !p.ends_with("archive") {
                    has_subdirs = true;
                    self.enforce_recursive(&p, count)?;
                }
            }
            if !has_subdirs {
                *count += self.rotation_policy.enforce(dir)?;
            }
        }
        Ok(())
    }

    /// Generates native, executable 4-Block OmniCode .scroll format.
    pub fn generate_omni_scroll(&self, s: &ExecutionScroll) -> String {
        let mut repairs_block = String::new();
        if s.repairs_applied.is_empty() {
            repairs_block.push_str(
                "    # Immune System: Clean; zero repairs required.\n",
            );
        } else {
            repairs_block.push_str(
                "    # Immune System Interventions (Restorator):\n",
            );
            for r in &s.repairs_applied {
                repairs_block.push_str(&format!(
                    "    repair_entry [ L{}: {:?} --> \"{}\" ]\n",
                    r.line, r.kind, r.description
                ));
            }
        }

        let mut failpoints_block = String::new();
        if s.failpoints.is_empty() {
            failpoints_block.push_str(
                "    # Sensory Diagnostics: 0 trapped failpoints.\n",
            );
        } else {
            failpoints_block.push_str("    # Trapped Diagnostic Failpoints:\n");
            for fp in &s.failpoints {
                let kind = fp
                    .failpoint_type
                    .map(|t| t.as_str())
                    .unwrap_or("FP_GENERAL");
                failpoints_block.push_str(&format!(
                    "    failpoint_record [ {kind} : \"{}\" ]\n",
                    fp.message
                ));
            }
        }

        let (scroll_label, _log_label, _, _, _) = generate_label_id(s);

        let domain_field = s
            .modal_domain
            .as_ref()
            .map(|d| format!("    domain: < \"{}\" >,\n", d))
            .unwrap_or_default();
        let cat_field = s
            .domain_category
            .as_ref()
            .map(|c| format!("    category: < \"{}\" >,\n", c))
            .unwrap_or_default();
        let anchor_field = s
            .biblical_anchor
            .as_ref()
            .map(|a| format!("    biblical_anchor: < \"{}\" >,\n", a))
            .unwrap_or_default();

        format!(
r#"# ========================================================================
# BLOCK 1: METADATA BLOCK (THE COGNITIVE SCROLL)
# ( Execution Trace ID, Prompt Intent, Inferred Types, and Neural Snapshot )
# ========================================================================
Scroll < EpistemicTrace.{session} > ( {prompt} ) {{
    label_id: < "{label_id}" >,
    version: < "1.0.0" >,
    system: < OmniCode.AI.Model >,
    timestamp: < "{timestamp}" >,
    session_id: < "{session}" >,
    execution_duration_micros: < {micros} >,
    cognitive_mode: < NeuralSymbolicTransduction >,
{domain_field}{cat_field}{anchor_field}}}

# ==============================================================================
# BLOCK 2: OPENING BLOCK (EXPERIENCE INITIALIZATION)
# ( Hardware Invariants, Preconditions, and Initial Context )
# ==============================================================================
Opening {{
    lexicon: [ Lexicon {{ Universal }} ],
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
            label_id = scroll_label,
            prompt = s.prompt,
            timestamp = s.timestamp_str,
            micros = s.elapsed_micros,
            domain_field = domain_field,
            cat_field = cat_field,
            anchor_field = anchor_field,
            repairs = repairs_block.trim_end(),
            failpoints = failpoints_block.trim_end(),
            reel_indented = indent_text(&s.synthesized_omnicode, "        "),
            state = s.execution_state.macro_op(),
            response = s.natural_response.replace('"', "\\\"")
        )
    }

    /// Generates traditional developer/telemetry .log format.
    pub fn generate_traditional_log(&self, s: &ExecutionScroll) -> String {
        let (scroll_label, log_label, _, _, _) = generate_label_id(s);
        let mut out = String::new();
        let bar = "=".repeat(78);
        let div = "-".repeat(78);

        out.push_str(&bar);
        out.push('\n');
        out.push_str(
            "OMNICODE AI MODEL EXECUTION LOG (TRADITIONAL TELEMETRY)\n",
        );
        out.push_str(&bar);
        out.push('\n');
        out.push_str(&format!("Timestamp:    {}\n", s.timestamp_str));
        out.push_str(&format!("Session ID:   {}\n", s.session_id));
        out.push_str(&format!("Label ID:     {}\n", scroll_label));
        out.push_str(&format!("Log Label:    {}\n", log_label));
        out.push_str(&format!("Duration:     {} µs\n", s.elapsed_micros));
        out.push_str(&format!(
            "Final State:  +1:{}\n",
            s.execution_state.macro_op()
        ));
        out.push_str(&format!("Discourse:    \"{}\"\n", s.prompt));
        if let Some(ref d) = s.modal_domain {
            out.push_str(&format!("Modal Domain: {}\n", d));
        }
        if let Some(ref a) = s.biblical_anchor {
            out.push_str(&format!("Scripture:    {}\n", a));
        }
        out.push_str(&div);
        out.push_str("\n\n");

        out.push_str("[1. SENSORY LOG EVENTS & FAILPOINTS]\n");
        if s.failpoints.is_empty() {
            out.push_str("  • (No failpoints trapped. Trace clean.)\n");
        } else {
            for (i, fp) in s.failpoints.iter().enumerate() {
                let kind = fp
                    .failpoint_type
                    .map(|t| t.as_str())
                    .unwrap_or("GENERAL");
                out.push_str(&format!(
                    "  [{:>4}] [{} µs] [{}] [{}] {}\n",
                    i + 1, fp.elapsed_micros, fp.channel, kind, fp.message
                ));
                if let Some(ref sug) = fp.suggestion {
                    out.push_str(&format!(
                        "         -> Remediation Proposal: [{:?}] L{}: {}\n",
                        sug.kind, sug.line, sug.rationale
                    ));
                }
            }
        }
        out.push('\n');

        out.push_str("[2. IMMUNE SYSTEM RESTORATIONS]\n");
        if s.repairs_applied.is_empty() {
            out.push_str(
                "  • (Zero repairs required; code conformed.)\n",
            );
        } else {
            for (i, r) in s.repairs_applied.iter().enumerate() {
                out.push_str(&format!(
                    "  [{:>2}] L{}: [{:?}] {}\n",
                    i + 1, r.line, r.kind, r.description
                ));
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
        out.push_str(&bar);
        out.push('\n');

        out
    }
}

fn indent_text(s: &str, indent: &str) -> String {
    s.lines()
        .map(|line| format!("{indent}{line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

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

/// Computes UTC Gregorian date parts (year, month, day) from epoch seconds.
pub fn utc_date_parts(epoch_secs: u64) -> (u32, u8, u8) {
    let days = (epoch_secs / 86400) as i64;
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = (yoe as i64) + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y as u32, m as u8, d as u8)
}

/// Derives Gregorian calendar date parts from timestamp string or system time.
pub fn derive_date_parts(timestamp_str: &str) -> (u32, u8, u8) {
    let secs = timestamp_str
        .split('_')
        .next()
        .and_then(|s| s.parse::<u64>().ok())
        .map(|val| {
            if val > 10_000_000_000 {
                val / 1_000_000
            } else {
                val
            }
        })
        .unwrap_or_else(|| {
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        });
    utc_date_parts(secs)
}

/// Generates human-and-machine-readable Label ID components:
/// `(scroll_label, log_label, year, month, day)`
pub fn generate_label_id(
    scroll: &ExecutionScroll,
) -> (String, String, u32, u8, u8) {
    let (year, month, day) = derive_date_parts(&scroll.timestamp_str);

    let seq = scroll
        .timestamp_str
        .split('_')
        .nth(1)
        .filter(|s| !s.is_empty())
        .unwrap_or("0001");

    let low_prompt = scroll.prompt.to_lowercase();
    let action = if scroll.execution_state == QuadState::Await
        || low_prompt.ends_with('?')
        || low_prompt.starts_with("what")
        || low_prompt.starts_with("who")
        || low_prompt.starts_with("how")
        || low_prompt.starts_with("why")
    {
        "EPISTEMIC"
    } else if low_prompt.contains("add ")
        || low_prompt.contains("subtract ")
        || low_prompt.contains("multiply ")
        || low_prompt.contains("divide ")
        || low_prompt.contains("arithmetic")
    {
        "ARITHMETIC"
    } else if low_prompt.contains("hello")
        || low_prompt.contains("hi")
        || low_prompt.contains("who are you")
        || low_prompt.contains("cpi-si")
        || low_prompt.contains("cpisi")
        || low_prompt.contains("covenant")
    {
        "COVENANT"
    } else if !scroll.repairs_applied.is_empty() {
        "IMMUNE"
    } else if low_prompt.contains("container")
        || low_prompt.contains("boundary")
    {
        "CONTAINER"
    } else {
        "DISCOURSE"
    };

    let state = scroll.execution_state.macro_op();

    let keyword: String = low_prompt
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| {
            !w.is_empty()
                && *w != "what"
                && *w != "is"
                && *w != "the"
                && *w != "a"
                && *w != "your"
                && *w != "favorite"
                && *w != "tell"
                && *w != "me"
                && *w != "about"
                && *w != "do"
                && *w != "you"
                && *w != "know"
        })
        .next()
        .unwrap_or("trace")
        .chars()
        .take(16)
        .collect();

    let stem = format!(
        "{year:04}{month:02}{day:02}_{seq}_{action}_{state}_{keyword}"
    );
    let scroll_label = format!("SCR_{stem}");
    let log_label = format!("LOG_{stem}");

    (scroll_label, log_label, year, month, day)
}

impl Default for ScrollLogger {
    fn default() -> Self {
        let dir = crate::core::filetype::resolve_scroll_storage_dir();
        Self::new(dir)
    }
}

// ============================================================================
// BLOCK 4: AUDIT PROCEDURES BLOCK
// ( Storehouse Verification & Inscription Capability Proving )
// ============================================================================

impl ScrollLogger {
    /// Audits the health and write-accessibility of the scroll storehouse.
    pub fn audit_scroll_subsystem(&self) -> ScrollSubsystemAudit {
        let mut errors = Vec::new();
        let scroll_dir_exists = self.scroll_dir.exists();
        let mut scroll_dir_writable = false;

        if !scroll_dir_exists {
            errors.push(format!(
                "Scroll storehouse directory does not exist: {:?}",
                self.scroll_dir
            ));
        } else {
            let probe_file = self.scroll_dir.join(".audit_probe.tmp");
            match fs::write(&probe_file, b"PROCEED") {
                Ok(_) => {
                    scroll_dir_writable = true;
                    let _ = fs::remove_file(&probe_file);
                }
                Err(e) => {
                    errors.push(format!(
                        "Scroll storehouse directory {:?} is not writable: {e}",
                        self.scroll_dir
                    ));
                }
            }
        }

        let status = if errors.is_empty() {
            QuadState::Proceed
        } else {
            QuadState::Rollback
        };

        ScrollSubsystemAudit {
            status,
            scroll_dir_exists,
            scroll_dir_writable,
            rotation_max_files: self
                .rotation_policy
                .max_files_per_partition,
            errors,
        }
    }
}

// ============================================================================
// BLOCK 5: UNIT TESTS & INVARIANT VERIFICATION SUITE
// ( Dual Log Generation, Modal Metadata Inscription & Audit Proofs )
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scroll_generation_with_modal_domain() {
        let logger = ScrollLogger::new("target/tmp_test_scrolls");
        let scroll = ExecutionScroll {
            session_id: "sess_modal_01".into(),
            timestamp_str: "123456_0001".into(),
            prompt: "what is truth?".into(),
            synthesized_omnicode: "roll_truth [ Law_Truth = PROCEED ]".into(),
            execution_state: QuadState::Proceed,
            natural_response: "Truth is covenant alignment.".into(),
            failpoints: Vec::new(),
            repairs_applied: Vec::new(),
            remediations_suggested: Vec::new(),
            elapsed_micros: 250,
            modal_domain: Some("Pistic".into()),
            domain_category: Some("TheologyReligion".into()),
            biblical_anchor: Some("John 14:6".into()),
        };

        let omni = logger.generate_omni_scroll(&scroll);
        assert!(omni.contains("domain: < \"Pistic\" >"));
        assert!(omni.contains("category: < \"TheologyReligion\" >"));
        assert!(omni.contains("biblical_anchor: < \"John 14:6\" >"));
        assert!(omni.contains("commit_status [ <PROCEED> ]"));

        let log = logger.generate_traditional_log(&scroll);
        assert!(log.contains("Modal Domain: Pistic"));
        assert!(log.contains("Scripture:    John 14:6"));
    }

    #[test]
    fn test_audit_scroll_subsystem() {
        let logger = ScrollLogger::new("target/tmp_test_scrolls_audit");
        let audit = logger.audit_scroll_subsystem();
        assert_eq!(audit.status, QuadState::Proceed);
        assert!(audit.scroll_dir_exists);
        assert!(audit.scroll_dir_writable);
        assert!(audit.errors.is_empty());
    }
}

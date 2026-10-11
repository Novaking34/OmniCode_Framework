// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( AI Learner Identity, Episodic Experience Engine & Memory Manifest )
// ==============================================================================
//! OmniCode AI Model: Episodic Learner & Memory Ingestion Engine
//!
//! Processes recorded execution scrolls (.scroll) and traditional logs (.log)
//! to extract supervised training pairs:
//!     Discourse Prompt -> Synthesized OmniCode -> Trapped Failpoint -> Restorator Patch -> +1:PROCEED
//!
//! Enables the internal AI model to accumulate experiential knowledge, detect
//! recurrent failure modes, and expand its cognitive vocabulary over time.

use std::fs;
use std::path::{Path, PathBuf};
use crate::core::ternary::QuadState;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Learning Structures, Episodic Memory Representation and Invariants )
// ==============================================================================
#[derive(Debug, Clone)]
pub struct EpisodicExperience {
    pub session_id: String,
    pub prompt: String,
    pub final_state: QuadState,
    pub repair_count: usize,
    pub failpoint_count: usize,
    pub scroll_path: PathBuf,
    pub log_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Default)]
pub struct EpisodicSummary {
    pub total_episodes: usize,
    pub proceed_count: usize,
    pub rollback_count: usize,
    pub await_count: usize,
    pub halt_count: usize,
    pub total_repairs_learned: usize,
    pub experiences: Vec<EpisodicExperience>,
}

pub struct AILearner {
    pub scroll_dir: PathBuf,
}

impl AILearner {
    pub fn new(scroll_dir: impl AsRef<Path>) -> Self {
        Self {
            scroll_dir: scroll_dir.as_ref().to_path_buf(),
        }
    }
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Episodic Learning Scanner: Parsing Scrolls & Building Training Memory )
// ==============================================================================
impl AILearner {
    /// Ingests all recorded scrolls and logs from the scroll directory,
    /// summarizing experience patterns and self-healing success rates.
    pub fn digest_experiences(&self) -> Result<EpisodicSummary, String> {
        if !self.scroll_dir.exists() {
            return Ok(EpisodicSummary::default());
        }

        let entries = fs::read_dir(&self.scroll_dir)
            .map_err(|e| format!("Failed to read scroll directory {:?}: {e}", self.scroll_dir))?;

        let mut summary = EpisodicSummary::default();

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("scroll") {
                if let Ok(content) = fs::read_to_string(&path) {
                    let exp = self.parse_scroll_content(&path, &content);
                    summary.total_episodes += 1;
                    match exp.final_state {
                        QuadState::Proceed => summary.proceed_count += 1,
                        QuadState::Rollback => summary.rollback_count += 1,
                        QuadState::Await => summary.await_count += 1,
                        QuadState::Halt => summary.halt_count += 1,
                    }
                    summary.total_repairs_learned += exp.repair_count;
                    summary.experiences.push(exp);
                }
            }
        }

        Ok(summary)
    }

    /// Extracts semantic metadata from a native .scroll file.
    fn parse_scroll_content(&self, path: &Path, content: &str) -> EpisodicExperience {
        let mut session_id = "unknown_sess".to_string();
        let mut prompt = String::new();
        let mut final_state = QuadState::Proceed;
        let mut repair_count = 0;
        let mut failpoint_count = 0;

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("session_id: < \"") && trimmed.ends_with("\" >,") {
                session_id = trimmed
                    .trim_start_matches("session_id: < \"")
                    .trim_end_matches("\" >,")
                    .to_string();
            } else if trimmed.starts_with("user_prompt < \"") && trimmed.ends_with("\" >") {
                prompt = trimmed
                    .trim_start_matches("user_prompt < \"")
                    .trim_end_matches("\" >")
                    .to_string();
            } else if trimmed.starts_with("commit_status [") {
                if trimmed.contains("<PROCEED>") {
                    final_state = QuadState::Proceed;
                } else if trimmed.contains("<ROLLBACK>") {
                    final_state = QuadState::Rollback;
                } else if trimmed.contains("<AWAIT>") {
                    final_state = QuadState::Await;
                } else if trimmed.contains("<HALT>") {
                    final_state = QuadState::Halt;
                }
            } else if trimmed.starts_with("repair_entry [") {
                repair_count += 1;
            } else if trimmed.starts_with("failpoint_record [") {
                failpoint_count += 1;
            }
        }

        let mut log_path = path.to_path_buf();
        log_path.set_extension("log");
        let log_exists = if log_path.exists() { Some(log_path) } else { None };

        EpisodicExperience {
            session_id,
            prompt,
            final_state,
            repair_count,
            failpoint_count,
            scroll_path: path.to_path_buf(),
            log_path: log_exists,
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Default Trait Implementation and Sealing )
// ==============================================================================
impl Default for AILearner {
    fn default() -> Self {
        let dir = if std::path::Path::new("runtime/scrolls").exists() {
            "runtime/scrolls"
        } else {
            "scrolls"
        };
        Self::new(dir)
    }
}

// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( AI Learner Identity, Episodic Experience Engine & Memory Manifest )
// ============================================================================
//! OmniCode AI Model: Episodic Learner & Memory Ingestion Engine (`learner.rs`)
//!
//! Processes recorded execution scrolls (`.scroll`) and logs (`.log`)
//! to extract supervised training pairs:
//!     Discourse Prompt -> OmniCode -> Failpoint -> Patch -> State
//!
//! Enables the internal AI model to accumulate experiential knowledge, detect
//! recurrent failure modes, and expand its cognitive vocabulary over time.
//!
//! ### Key Invariants:
//! 1. **Episodic Digestibility**: Parses native 4-Block `.scroll` Turing reels.
//! 2. **Modal Domain Tracking**: Categorizes experiences across modal spheres.
//! 3. **Synaptic Plasticity**: Reinforces neural weights for verified outcomes.
//! 4. **Strict Line Length**: Every line must be <= 80 characters.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::ai::neural::network::NeuralBrain;
use crate::core::ternary::QuadState;

// ============================================================================
// BLOCK 2: TYPES BLOCK
// ( Learning Structures, Episodic Memory Representation & Audits )
// ============================================================================

/// Parsed episodic experience extracted from an inscribed `.scroll` reel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpisodicExperience {
    pub session_id: String,
    pub prompt: String,
    pub final_state: QuadState,
    pub repair_count: usize,
    pub failpoint_count: usize,
    pub scroll_path: PathBuf,
    pub log_path: Option<PathBuf>,
    pub modal_domain: Option<String>,
    pub domain_category: Option<String>,
}

/// Comprehensive summary of all digested episodic experiences.
#[derive(Debug, Clone, Default)]
pub struct EpisodicSummary {
    pub total_episodes: usize,
    pub proceed_count: usize,
    pub rollback_count: usize,
    pub await_count: usize,
    pub halt_count: usize,
    pub total_repairs_learned: usize,
    pub domain_distribution: HashMap<String, usize>,
    pub experiences: Vec<EpisodicExperience>,
}

/// Diagnostic report verifying storehouse accessibility and learning status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnerSubsystemAudit {
    pub status: QuadState,
    pub storehouse_accessible: bool,
    pub total_scrolls_found: usize,
    pub total_episodes_digested: usize,
    pub errors: Vec<String>,
}

/// Ingestion engine transforming past scrolls into neuroplastic adaptations.
pub struct AILearner {
    pub scroll_dir: PathBuf,
}

impl AILearner {
    /// Constructs a new [`AILearner`] targeting the given scroll storehouse.
    pub fn new(scroll_dir: impl AsRef<Path>) -> Self {
        Self {
            scroll_dir: scroll_dir.as_ref().to_path_buf(),
        }
    }
}

impl Default for AILearner {
    fn default() -> Self {
        let dir = if Path::new("storage/scrolls").exists() {
            "storage/scrolls"
        } else if Path::new("runtime/scrolls").exists() {
            "runtime/scrolls"
        } else {
            "scrolls"
        };
        Self::new(dir)
    }
}

// ============================================================================
// BLOCK 3: BODY BLOCK
// ( Episodic Learning Scanner: Parsing Scrolls & Building Training Memory )
// ============================================================================

impl AILearner {
    /// Ingests all recorded scrolls and logs from the scroll directory,
    /// summarizing experience patterns and self-healing success rates.
    pub fn digest_experiences(&self) -> Result<EpisodicSummary, String> {
        if !self.scroll_dir.exists() {
            return Ok(EpisodicSummary::default());
        }

        let scroll_paths = collect_scroll_files(&self.scroll_dir);
        let mut summary = EpisodicSummary::default();

        for path in scroll_paths {
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
                if let Some(ref d) = exp.modal_domain {
                    let entry = summary
                        .domain_distribution
                        .entry(d.clone())
                        .or_insert(0);
                    *entry += 1;
                }
                summary.experiences.push(exp);
            }
        }

        Ok(summary)
    }

    /// Feeds digested episodic experiences directly into the NeuralBrain,
    /// adapting balanced-ternary synaptic weights based on state outcomes.
    pub fn train_brain(
        &self,
        brain: &mut NeuralBrain,
    ) -> Result<usize, String> {
        let summary = self.digest_experiences()?;
        let mut adapted = 0;

        for (ep_idx, exp) in summary.experiences.iter().enumerate() {
            if brain.layers.is_empty() {
                break;
            }
            let l0_len = brain.layers[0].neurons.len();
            if l0_len > 0 {
                let neuron_idx = ep_idx % l0_len;
                let w_len = brain.layers[0].neurons[neuron_idx].weights.len();
                if w_len > 0 {
                    let weight_idx =
                        (exp.repair_count + exp.failpoint_count) % w_len;
                    brain.adapt_synapse(
                        0,
                        neuron_idx,
                        weight_idx,
                        exp.final_state,
                    );
                    adapted += 1;
                }
            }

            // Output layer reinforcement for verified execution
            if brain.layers.len() > 1 {
                let last_layer = brain.layers.len() - 1;
                let out_len = brain.layers[last_layer].neurons.len();
                if out_len > 0 {
                    let out_neuron = 0;
                    let out_w_len = brain.layers[last_layer].neurons[out_neuron]
                        .weights
                        .len();
                    if out_w_len > 0 {
                        let out_weight = ep_idx % out_w_len;
                        brain.adapt_synapse(
                            last_layer,
                            out_neuron,
                            out_weight,
                            exp.final_state,
                        );
                    }
                }
            }
        }

        Ok(adapted)
    }

    /// Extracts semantic metadata from a native .scroll file.
    pub fn parse_scroll_content(
        &self,
        path: &Path,
        content: &str,
    ) -> EpisodicExperience {
        let mut session_id = "unknown_sess".to_string();
        let mut prompt = String::new();
        let mut final_state = QuadState::Proceed;
        let mut repair_count = 0;
        let mut failpoint_count = 0;
        let mut modal_domain = None;
        let mut domain_category = None;

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("session_id: < \"")
                && trimmed.ends_with("\" >,")
            {
                session_id = trimmed
                    .trim_start_matches("session_id: < \"")
                    .trim_end_matches("\" >,")
                    .to_string();
            } else if trimmed.starts_with("user_prompt < \"")
                && trimmed.ends_with("\" >")
            {
                prompt = trimmed
                    .trim_start_matches("user_prompt < \"")
                    .trim_end_matches("\" >")
                    .to_string();
            } else if trimmed.starts_with("domain: < \"")
                && (trimmed.ends_with("\" >,") || trimmed.ends_with("\" >"))
            {
                modal_domain = Some(
                    trimmed
                        .trim_start_matches("domain: < \"")
                        .trim_end_matches("\" >,")
                        .trim_end_matches("\" >")
                        .to_string(),
                );
            } else if trimmed.starts_with("category: < \"")
                && (trimmed.ends_with("\" >,") || trimmed.ends_with("\" >"))
            {
                domain_category = Some(
                    trimmed
                        .trim_start_matches("category: < \"")
                        .trim_end_matches("\" >,")
                        .trim_end_matches("\" >")
                        .to_string(),
                );
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
        let log_exists = if log_path.exists() {
            Some(log_path)
        } else {
            let path_str = path.to_string_lossy();
            if path_str.contains("/scroll/") {
                let mut sibling =
                    PathBuf::from(path_str.replace("/scroll/", "/log/"));
                sibling.set_extension("log");
                if sibling.exists() {
                    Some(sibling)
                } else if let Some(fname) =
                    sibling.file_name().and_then(|f| f.to_str())
                {
                    if fname.starts_with("SCR_") {
                        let replaced = fname.replacen("SCR_", "LOG_", 1);
                        let alt = sibling.with_file_name(replaced);
                        if alt.exists() {
                            Some(alt)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        };

        EpisodicExperience {
            session_id,
            prompt,
            final_state,
            repair_count,
            failpoint_count,
            scroll_path: path.to_path_buf(),
            log_path: log_exists,
            modal_domain,
            domain_category,
        }
    }
}

/// Recursively collects all .scroll files located within `dir`.
fn collect_scroll_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                files.extend(collect_scroll_files(&p));
            } else if p.extension().and_then(|e| e.to_str()) == Some("scroll") {
                files.push(p);
            }
        }
    }
    files
}

// ============================================================================
// BLOCK 4: AUDIT PROCEDURES BLOCK
// ( Storehouse Diagnostic Invariants & Audit Proving )
// ============================================================================

impl AILearner {
    /// Audits the learner subsystem by probing storehouse accessibility.
    pub fn audit_learner_subsystem(&self) -> LearnerSubsystemAudit {
        let mut errors = Vec::new();
        let accessible = self.scroll_dir.exists();

        if !accessible {
            errors.push(format!(
                "Scroll storehouse path {:?} is inaccessible",
                self.scroll_dir
            ));
        }

        let scrolls = if accessible {
            collect_scroll_files(&self.scroll_dir)
        } else {
            Vec::new()
        };

        let status = if errors.is_empty() {
            QuadState::Proceed
        } else {
            QuadState::Rollback
        };

        LearnerSubsystemAudit {
            status,
            storehouse_accessible: accessible,
            total_scrolls_found: scrolls.len(),
            total_episodes_digested: scrolls.len(),
            errors,
        }
    }
}

// ============================================================================
// BLOCK 5: UNIT TESTS & INVARIANT VERIFICATION SUITE
// ( Parsing, Ingestion, Modal Distribution & Audit Proofs )
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_learner_digest_empty() {
        let learner = AILearner::new("target/nonexistent_scroll_storehouse");
        let summary = learner
            .digest_experiences()
            .expect("Empty storehouse must not error");
        assert_eq!(summary.total_episodes, 0);
        assert_eq!(summary.experiences.len(), 0);
    }

    #[test]
    fn test_ai_learner_parse_modal_scroll() {
        let learner = AILearner::new("target/tmp_learner");
        let sample_scroll = r#"
Scroll < EpistemicTrace.sess_test > ( what is fruit? ) {
    version: < "1.0.0" >,
    session_id: < "sess_test" >,
    domain: < "BioticOrganic" >,
    category: < "EcologyStewardship" >,
}
user_prompt < "what is fruit?" >
Body [
    synthesized_reel [
        roll_fruit [ Fruit = PROCEED ]
    ]
]
Closing {
    commit_status [ <PROCEED> ]
}
"#;
        let exp = learner
            .parse_scroll_content(Path::new("test.scroll"), sample_scroll);
        assert_eq!(exp.session_id, "sess_test");
        assert_eq!(exp.prompt, "what is fruit?");
        assert_eq!(exp.final_state, QuadState::Proceed);
        assert_eq!(exp.modal_domain.as_deref(), Some("BioticOrganic"));
        assert_eq!(exp.domain_category.as_deref(), Some("EcologyStewardship"));
    }

    #[test]
    fn test_ai_learner_audit_subsystem() {
        let learner = AILearner::new("target");
        let audit = learner.audit_learner_subsystem();
        assert_eq!(audit.status, QuadState::Proceed);
        assert!(audit.storehouse_accessible);
        assert!(audit.errors.is_empty());
    }
}

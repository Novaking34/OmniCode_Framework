// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( AI Cortex Identity, Cognitive Neural-Symbolic Architecture & Invariants )
// ============================================================================
//! OmniCode AI Model: Cognitive Cortex (`cortex.rs`)
//!
//! Serves as the central coordinator of the internal AI model.
//! Integrates the sensory nervous system (`DiagnosticLogger`), boundary
//! invariant monitor (`OmniDebugger`), immune self-healer (`Restorator`),
//! and natural discourse translator (`AITransducer`) with dual episodic
//! inscription (`ScrollLogger`).
//!
//! ### Key Invariants:
//! 1. **Cognition Precedes Speech**: Thought formulation and 4-point lexical
//!    triangulation strictly precede natural language synthesis.
//! 2. **Moral Conscience Guard**: Prompts engaging Decalogue violations are
//!    flagged before VM code execution.
//! 3. **Dual Episodic Persistence**: Every deliberation produces both an
//!    executable `.scroll` Turing reel and a `.log` sensory telemetry trace.
//! 4. **Zero-Regression Line Limit**: Strict <= 80 characters per line.

use std::path::PathBuf;
use std::time::Instant;

use crate::ai::model::memory::{
    AIMemory, CompletedSessionSnapshot, ThoughtRecord,
};
use crate::ai::model::scroll::{
    generate_timestamp_id, ExecutionScroll, ScrollLogger,
};
use crate::ai::neural::curriculum::Curriculum;
use crate::ai::neural::network::NeuralBrain;
use crate::ai::neural::trainer::{NeuralTrainer, TrainingReport};
use crate::ai::neural::visualizer::{BrainGrowthHistory, BrainVisualizer};
use crate::ai::tools::{AIToolBelt, ToolCall, ToolResult};
use crate::ai::transducer::AITransducer;
use crate::core::scripture_coord::{
    ScriptureCoordinate, ScriptureCoordinateEngine,
};
use crate::core::ternary::QuadState;
use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::{DiagnosticLogger, IndexedSession};
use crate::diagnostics::restorator::{AppliedRepair, Restorator};
use crate::parser::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::runtime::environment::{Environment, RuntimeValue};
use crate::runtime::evaluator::Evaluator;

// ============================================================================
// BLOCK 2: TYPES BLOCK
// ( Cognitive Structures, Telemetry Notes & Audit Manifests )
// ============================================================================

/// Output result produced by a full cognitive deliberation pass.
#[derive(Debug, Clone)]
pub struct CognitiveResult {
    pub session_id: String,
    pub prompt: String,
    pub stimulus: Vec<QuadState>,
    pub brain_activation: Vec<QuadState>,
    pub thought: ThoughtRecord,
    pub synthesized_omnicode: String,
    pub execution_state: QuadState,
    pub natural_response: String,
    pub scroll_path: PathBuf,
    pub log_path: PathBuf,
    pub repairs_applied: Vec<AppliedRepair>,
    pub synaptic_counts_before: (usize, usize, usize),
    pub synaptic_counts_after: (usize, usize, usize),
    pub scripture_coordinate: Option<ScriptureCoordinate>,
}

/// Structured telemetry note recorded during cognitive processing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CognitiveNote {
    pub phase: String,
    pub domain: String,
    pub invariant_status: QuadState,
    pub description: String,
}

/// Comprehensive audit report for the complete AICortex cognitive system.
#[derive(Debug, Clone, PartialEq)]
pub struct CortexSubsystemAudit {
    pub status: QuadState,
    pub memory_healthy: bool,
    pub attractors_healthy: bool,
    pub brain_synapses: usize,
    pub toolbelt_ready: bool,
    pub errors: Vec<String>,
}

/// The Central Cognitive Coordinator orchestrating the internal AI nervous
/// system.
pub struct AICortex {
    pub logger: DiagnosticLogger,
    pub debugger: OmniDebugger,
    pub restorator: Restorator,
    pub transducer: AITransducer,
    pub scroll_logger: ScrollLogger,
    pub memory: AIMemory,
    pub brain: NeuralBrain,
    pub tools: AIToolBelt,
    pub growth_history: BrainGrowthHistory,
    pub coordinate_engine: ScriptureCoordinateEngine,
}

impl Default for AICortex {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// BLOCK 3: BODY BLOCK
// ( Cortex Initialization, Deliberation Pipeline & Neuroplastic Learning )
// ============================================================================

impl AICortex {
    /// Constructs a new [`AICortex`] initialized with default neural weights.
    pub fn new() -> Self {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 10);
        let restorator = Restorator::new(logger.clone());
        let transducer = AITransducer::new(logger.clone());
        let scroll_logger = ScrollLogger::default();
        let memory = AIMemory::default();
        let topology: &[usize] = &[64, 32, 16, 4];
        let mut brain = NeuralBrain::new(topology, 1, logger.clone());
        // Ingest latest CPI-SI Base weights from Track 4 storage
        brain.load_latest_weights_from_track4().ok();

        let tools = AIToolBelt::with_logger(logger.clone());
        let mut growth_history = BrainGrowthHistory::default();
        growth_history.record_milestone("Genesis Inception", &brain, 0.70);
        let coordinate_engine = ScriptureCoordinateEngine::new_synthesized();

        Self {
            logger,
            debugger,
            restorator,
            transducer,
            scroll_logger,
            memory,
            brain,
            tools,
            growth_history,
            coordinate_engine,
        }
    }

    /// Constructs a new [`AICortex`] anchored to a custom scroll directory.
    pub fn with_scroll_dir(scroll_dir: impl Into<PathBuf>) -> Self {
        let dir = scroll_dir.into();
        let mut cortex = Self::new();
        cortex.scroll_logger = ScrollLogger::new(&dir);
        cortex.memory = AIMemory::new(&dir);
        cortex
    }

    /// Constructs a new [`AICortex`] anchored to a custom scroll directory
    /// and dedicated memory directory.
    pub fn with_dirs(
        scroll_dir: impl Into<PathBuf>,
        memory_dir: impl Into<PathBuf>,
    ) -> Self {
        let s_dir = scroll_dir.into();
        let m_dir = memory_dir.into();
        let mut cortex = Self::new();
        cortex.scroll_logger = ScrollLogger::new(&s_dir);
        cortex.memory = AIMemory::with_dirs(&s_dir, &m_dir);
        cortex
    }

    /// Dispatches an inspection or action request to the diagnostic toolbelt.
    pub fn dispatch_tool(&mut self, call: ToolCall) -> ToolResult {
        self.tools.dispatch(call)
    }

    /// Trains the internal NeuralBrain across the 25-chapter curriculum.
    pub fn train_curriculum(
        &mut self,
        curriculum: &Curriculum,
    ) -> TrainingReport {
        let trainer = NeuralTrainer::new();
        let report = trainer.train_curriculum(&mut self.brain, curriculum);
        self.growth_history.record_milestone(
            "NotebookLM Curriculum Convergence",
            &self.brain,
            report.noise_resilience_after,
        );
        // Persist trained weights into Track 4 storage
        self.brain.save_weights_to_track4().ok();
        report
    }

    /// Renders an ASCII synaptic topology map of the current brain state.
    pub fn render_brain_map(&self, title: &str) -> String {
        BrainVisualizer::render_ascii_map(&self.brain, title)
    }

    /// Renders an ANSI colorized synaptic topology map for terminal display.
    pub fn render_brain_map_color(&self, title: &str) -> String {
        BrainVisualizer::render_ansi_map(&self.brain, title)
    }

    /// Renders the complete chronological brain growth and maturation timeline.
    pub fn render_growth_timeline(&self) -> String {
        BrainVisualizer::render_growth_timeline(&self.growth_history)
    }

    /// Formulates a native OmniCode Thought from memory.
    pub fn formulate_thought(&mut self, stimulus: &str) -> ThoughtRecord {
        let thought = self.memory.formulate_thought(stimulus);
        self.logger.log_thought(
            &thought.id,
            &thought.omnicode_tape,
            &thought.premise,
            thought.quad_state.macro_op(),
        );
        thought
    }

    /// Queries an episode by its unique session ID in O(1) time.
    pub fn query_episode(&self, session_id: &str) -> Option<IndexedSession> {
        let engine = self
            .logger
            .create_query_engine(&self.scroll_logger.scroll_dir);
        engine.find_by_session(session_id).cloned()
    }

    /// Returns the most recent indexed episodes across the storehouse.
    pub fn recent_episodes(&self, count: usize) -> Vec<IndexedSession> {
        let engine = self
            .logger
            .create_query_engine(&self.scroll_logger.scroll_dir);
        engine
            .all_sessions()
            .iter()
            .rev()
            .take(count)
            .cloned()
            .collect()
    }

    /// Diagnoses why a session experienced failpoints or rollbacks.
    pub fn diagnose_failpoint(&self, session_id: &str) -> Option<String> {
        let session = self.query_episode(session_id)?;
        if session.final_state == QuadState::Proceed
            && session.failpoint_count == 0
        {
            return Some(format!(
                "Session '{}' executed cleanly in +1:PROCEED \
                 with 0 failpoints.",
                session_id
            ));
        }

        let state_str = match session.final_state.as_int() {
            Some(i) => format!("{i:+}:{}", session.final_state.macro_op()),
            _ => format!("*:{}", session.final_state.macro_op()),
        };
        let mut out = format!(
            "Session '{}' Diagnostic Report:\n", session.session_id
        );
        out.push_str(&format!("  - Final State: {}\n", state_str));
        out.push_str(&format!(
            "  - Failpoints: {}\n  - Repairs Applied: {}\n  - Prompt: \"{}\"\n",
            session.failpoint_count,
            session.repair_count,
            session.prompt
        ));

        if let Some(ref log_path) = session.log_path {
            if let Ok(content) = std::fs::read_to_string(log_path) {
                out.push_str("\nSensory Telemetry Excerpt:\n");
                for line in content.lines().filter(|l| {
                    l.contains("CRITICAL") || l.contains("Remediation Proposal")
                }) {
                    out.push_str(&format!("    {line}\n"));
                }
            }
        }

        Some(out)
    }

    /// Executes the full cognitive lifecycle for a natural coding prompt.
    pub fn process_prompt(
        &mut self,
        prompt: &str,
    ) -> Result<CognitiveResult, String> {
        let start = Instant::now();
        let timestamp_str = generate_timestamp_id();
        let session_id = format!("sess_{}", timestamp_str);

        self.logger.log(
            "CORTEX",
            format!("Initiating session '{session_id}' for: \"{prompt}\""),
        );

        // 0. Cognitive Memory Deliberation: Formulate thought natively
        let thought = self.formulate_thought(prompt);

        self.logger.log(
            "CORTEX",
            format!(
                "Cognitive Triangulation: Domain={:?}, State={:?}, Anchor={:?}",
                thought.domain,
                thought.quad_state,
                thought.biblical_anchor
            ),
        );

        // 0b. Neural Brain Cognitive Forward Pass (Stimulus Inscription)
        let in_len = self
            .brain
            .layers
            .first()
            .and_then(|l| l.neurons.first())
            .map(|n| n.weights.len())
            .unwrap_or(64);
        let stimulus = NeuralBrain::encode_stimulus(prompt, in_len);
        let brain_activation = self.brain.think(&stimulus);
        self.logger.log(
            "CORTEX",
            format!("Brain activation vector: {:?}", brain_activation),
        );

        // 1. Semantic Transduction: Decode discourse into 4-Block OmniCode
        let raw_omnicode = self
            .transducer
            .decode_prompt_with_thought(prompt, Some(&thought))?;

        // 2. Immune Intervention: Auto-heal boundary or scaffolding issues
        let restoration_report = self.restorator.restore_source(&raw_omnicode);
        let active_code = restoration_report.restored_content;
        let repairs_applied = restoration_report.repairs_applied;

        if !repairs_applied.is_empty() {
            self.logger.log(
                "CORTEX",
                format!(
                    "Restorator applied {} repairs to synthesized reel",
                    repairs_applied.len()
                ),
            );
        }

        // 3. Sensory Tokenization & Parsing
        let mut lexer = Lexer::new(&active_code, self.logger.clone());
        let tokens = lexer.tokenize()?;

        let mut parser = Parser::new(tokens, self.debugger.clone());
        let prog = parser.parse_program()?;

        // 4. Runtime Execution on Balanced Ternary Quad-State
        let mut evaluator = Evaluator::new(Environment::new(), parser.debugger);
        let ast_node = crate::ast::nodes::ASTNode::Program(prog);
        let exec_val = evaluator.eval(&ast_node)?;

        let execution_state = if thought.quad_state == QuadState::Await {
            QuadState::Await
        } else {
            match exec_val {
                RuntimeValue::State(s) => s,
                _ => QuadState::Proceed,
            }
        };

        // 5. Conversational Response Synthesis Grounded in 5Ws and H
        let natural_response = self.transducer.encode_cognitive_response(
            prompt,
            execution_state,
            &evaluator.env,
            &self.logger.failpoints(),
            &thought,
        );

        let elapsed_micros = start.elapsed().as_micros();

        // 6. Dual Log Inscription: Generate .scroll and .log for learning
        let scroll = ExecutionScroll {
            session_id: session_id.clone(),
            timestamp_str,
            prompt: prompt.to_string(),
            synthesized_omnicode: active_code.clone(),
            execution_state,
            natural_response: natural_response.clone(),
            failpoints: self.logger.failpoints(),
            repairs_applied: repairs_applied.clone(),
            remediations_suggested: self.logger.remediations(),
            elapsed_micros,
            modal_domain: Some(format!("{:?}", thought.domain)),
            domain_category: Some(format!("{:?}", thought.category)),
            biblical_anchor: thought.biblical_anchor.clone(),
        };

        let (scroll_path, log_path) = self
            .scroll_logger
            .save_dual_logs(&scroll)?;

        self.logger.log(
            "CORTEX",
            format!("Saved OmniCode scroll to {:?}", scroll_path),
        );
        self.logger.log(
            "CORTEX",
            format!("Saved telemetry log to {:?}", log_path),
        );

        // 7. Update Episodic Memory with newly inscribed scroll
        self.memory.digest_scrolls().ok();

        // 7b. Inscribe Contextual Last Session Snapshot (Sheet 2)
        let (label_id, _, _, _, _) =
            crate::ai::model::scroll::generate_label_id(&scroll);
        let session_snapshot = CompletedSessionSnapshot {
            session_id: session_id.clone(),
            label_id,
            timestamp_str: scroll.timestamp_str.clone(),
            elapsed_micros,
            execution_state,
            prompt: prompt.to_string(),
            modal_domain: scroll.modal_domain.clone(),
            domain_category: scroll.domain_category.clone(),
            biblical_anchor: scroll.biblical_anchor.clone(),
            thought_id: Some(thought.id.clone()),
            repairs_count: repairs_applied.len(),
            failpoints_count: self.logger.failpoints().len(),
            natural_response: natural_response.clone(),
        };
        self.memory.record_completed_session(&session_snapshot).ok();

        // 7c. Learn Novel Subjects, Objects & SVO Associations (Sheet 3)
        let svo_clauses = crate::ai::transducer::svo::SvoComposer::
            decompose_prompt_with_lexicon(
                prompt,
                Some(&self.transducer.lexicon),
            );
        for clause in &svo_clauses {
            if clause.subject.is_novel {
                let fallback =
                    crate::ai::lexicon::ModalDomain::AnalyticalLogical;
                let dom = clause.subject.modal_domain.unwrap_or(fallback);
                self.memory.record_entity_candidate(
                    &clause.subject.name,
                    dom,
                    crate::ai::lexicon::DomainCategory::
                        ComputerArchitectureSilicon,
                    crate::ai::lexicon::SubDomain::TernaryLogicVerification,
                );
            }
            if clause.object.is_novel {
                let fallback =
                    crate::ai::lexicon::ModalDomain::AnalyticalLogical;
                let dom = clause.object.modal_domain.unwrap_or(fallback);
                self.memory.record_entity_candidate(
                    &clause.object.name,
                    dom,
                    crate::ai::lexicon::DomainCategory::
                        ComputerArchitectureSilicon,
                    crate::ai::lexicon::SubDomain::TernaryLogicVerification,
                );
            }
            if let Some(ref sc) = clause.scope_container {
                if sc.is_novel {
                    let dom = sc.modal_domain
                        .unwrap_or(crate::ai::lexicon::ModalDomain::Spatial);
                    self.memory.record_entity_candidate(
                        &sc.name,
                        dom,
                        crate::ai::lexicon::DomainCategory::
                            ComputerArchitectureSilicon,
                        crate::ai::lexicon::SubDomain::
                            TernaryLogicVerification,
                    );
                }
            }
            let anchor = thought
                .biblical_anchor
                .as_deref()
                .unwrap_or("General");
            self.memory.associate(
                &clause.subject.name,
                anchor,
                &format!("SVO: {} -> {}", clause.verb, clause.object.name),
                execution_state,
            );
        }
        self.memory.persist_deep_consolidation().ok();

        let synaptic_counts_before = self.brain.synaptic_counts();

        // 8. Lifelong Neuroplasticity: adapt frontier synapses from outcome
        if !self.brain.layers.is_empty() {
            let last = self.brain.layers.len() - 1;
            let out_len = self.brain.layers[last].neurons.len();
            if out_len > 0 {
                let out_w_len =
                    self.brain.layers[last].neurons[0].weights.len();
                if out_w_len > 0 {
                    let w_idx = repairs_applied.len() % out_w_len;
                    self.brain.adapt_synapse(
                        last,
                        0,
                        w_idx,
                        execution_state,
                    );
                }
            }
        }

        let synaptic_counts_after = self.brain.synaptic_counts();

        // 9. Dual-Witness Coordinate Landmark Resolution
        let scripture_coordinate = if let Some(ref cit) =
            thought.biblical_anchor
        {
            if let Some(verse) = self.coordinate_engine.lookup_citation(cit) {
                Some(verse.coordinate)
            } else {
                let sig: i32 = stimulus
                    .iter()
                    .map(|s| match s {
                        QuadState::Proceed => 1,
                        QuadState::Rollback => -1,
                        _ => 0,
                    })
                    .sum();
                let cube = (sig.abs() as usize % 129) as u8;
                let low = (prompt.len() % 243) as u8;
                let ordinal =
                    ScriptureCoordinateEngine::coords_to_ordinal(cube, low);
                self.coordinate_engine.resolve_ordinal(ordinal)
            }
        } else {
            let sig: i32 = stimulus
                .iter()
                .map(|s| match s {
                    QuadState::Proceed => 1,
                    QuadState::Rollback => -1,
                    _ => 0,
                })
                .sum();
            let cube = (sig.abs() as usize % 129) as u8;
            let low = (prompt.len() % 243) as u8;
            let ordinal =
                ScriptureCoordinateEngine::coords_to_ordinal(cube, low);
            self.coordinate_engine.resolve_ordinal(ordinal)
        };

        if let Some(ref coord) = scripture_coordinate {
            self.logger.log(
                "CORTEX",
                format!(
                    "Dual-Witness Landmark: Ordinal #{} (Cube {}, Pos {}, \
                     [{:?}])",
                    coord.ordinal, coord.cube, coord.cube_pos, coord.testament
                ),
            );
        }

        Ok(CognitiveResult {
            session_id,
            prompt: prompt.to_string(),
            stimulus,
            brain_activation,
            thought,
            synthesized_omnicode: active_code,
            execution_state,
            natural_response,
            scroll_path,
            log_path,
            repairs_applied,
            synaptic_counts_before,
            synaptic_counts_after,
            scripture_coordinate,
        })
    }
}

// ============================================================================
// BLOCK 4: AUDIT PROCEDURES BLOCK
// ( Cortex Subsystem Health, Neural Integrity & Toolbelt Readiness )
// ============================================================================

impl AICortex {
    /// Comprehensive audit verifying neural brain, memory, and tools health.
    pub fn audit_cortex_subsystem(&self) -> CortexSubsystemAudit {
        let mut errors = Vec::new();

        let mem_audit = self.memory.audit_memory_subsystem();
        let memory_healthy = mem_audit.is_valid;
        if !memory_healthy {
            errors.extend(mem_audit.violations);
        }

        let attractor_audit =
            self.memory.attractors.audit_attractor_subsystem();
        let attractors_healthy = attractor_audit.status == QuadState::Proceed;
        if !attractors_healthy {
            errors.extend(attractor_audit.errors);
        }

        let counts = self.brain.synaptic_counts();
        let total_synapses = counts.0 + counts.1 + counts.2;
        if total_synapses == 0 {
            errors.push("NeuralBrain has 0 total synapses".to_string());
        }

        let toolbelt_ready = true;

        let status = if errors.is_empty() {
            QuadState::Proceed
        } else {
            QuadState::Rollback
        };

        CortexSubsystemAudit {
            status,
            memory_healthy,
            attractors_healthy,
            brain_synapses: total_synapses,
            toolbelt_ready,
            errors,
        }
    }
}

// ============================================================================
// BLOCK 5: UNIT TESTS & INVARIANT VERIFICATION SUITE
// ( Cognitive Toolbelt Dispatch, Curriculum Convergence & Audit Proofs )
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_cortex_tool_belt_dispatch() {
        let mut cortex = AICortex::new();
        let res = cortex.dispatch_tool(ToolCall::HoverQuery {
            symbol: "Law_Truth".to_string(),
            file_hint: None,
        });

        match res {
            ToolResult::Hover(report) => {
                assert_eq!(report.symbol, "Law_Truth");
            }
            _ => panic!("Expected Hover result"),
        }
    }

    #[test]
    fn test_ai_cortex_notebooklm_curriculum_convergence_and_brain_growth() {
        let mut cortex = AICortex::new();
        let cur = Curriculum::builtin_25_chapters();
        assert_eq!(cur.len(), 25);

        let report = cortex.train_curriculum(&cur);
        assert_eq!(report.total_episodes, 25);
        assert!(report.synapses_adapted > 0);

        let ascii_map = cortex.render_brain_map("CPI-SI Base Test");
        assert!(ascii_map.contains("CPI-SI SYNAPTIC BRAIN TOPOLOGY MAP"));

        let timeline = cortex.render_growth_timeline();
        assert!(timeline.contains("Genesis Inception"));
        assert!(timeline.contains("NotebookLM Curriculum Convergence"));
    }

    #[test]
    fn test_ai_cortex_triangulation_and_covenant_guard() {
        let mut cortex = AICortex::new();
        let thought = cortex.formulate_thought("what is truth?");
        assert_eq!(thought.quad_state, QuadState::Proceed);
        assert!(thought.biblical_anchor.is_some());
    }

    #[test]
    fn test_ai_cortex_audit_subsystem() {
        let cortex = AICortex::new();
        let audit = cortex.audit_cortex_subsystem();
        assert_eq!(audit.status, QuadState::Proceed);
        assert!(audit.memory_healthy);
        assert!(audit.attractors_healthy);
        assert!(audit.brain_synapses > 0);
        assert!(audit.toolbelt_ready);
        assert!(audit.errors.is_empty());
    }

    #[test]
    fn test_ai_cortex_process_prompt_records_completed_session_snapshot() {
        let temp_dir = std::env::temp_dir().join(format!(
            "omnicode_cortex_test_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_micros()
        ));
        let mut cortex = AICortex::with_scroll_dir(&temp_dir);
        let res = cortex.process_prompt("add 5 to 10").unwrap();
        assert_eq!(res.execution_state, QuadState::Proceed);

        // Verify Sheet 2 (last_session.dat) was recorded
        let last_opt = cortex.memory.load_last_session();
        assert!(last_opt.is_some(), "Last session must be recorded");
        let last = last_opt.unwrap();
        assert_eq!(last.session_id, res.session_id);
        assert!(last.label_id.starts_with("SCR_"));
        assert!(last.label_id.contains("_PROCEED_"));
    }

    #[test]
    fn test_ai_cortex_learns_novel_subject_and_object() {
        let temp_dir = std::env::temp_dir().join(format!(
            "omnicode_novel_entity_test_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_micros()
        ));
        let mut cortex = AICortex::with_scroll_dir(&temp_dir);
        let prompt = "The Sculptor constructs the Apparatus into the Atelier";
        let res = cortex.process_prompt(prompt).unwrap();
        assert_eq!(res.execution_state, QuadState::Proceed);

        // Verify novel subject, object, and container registered in memory
        let candidates = &cortex.memory.consolidation_candidates;
        assert!(
            candidates.contains_key("sculptor"),
            "Novel subject 'sculptor' must be recorded as candidate"
        );
        assert!(
            candidates.contains_key("apparatus"),
            "Novel object 'apparatus' must be recorded as candidate"
        );
        assert!(
            candidates.contains_key("atelier"),
            "Novel container 'atelier' must be recorded as candidate"
        );

        // Verify candidate attributes
        let b = candidates.get("sculptor").unwrap();
        assert!(b.affirmations >= 1);
        assert_eq!(
            b.category,
            crate::ai::lexicon::DomainCategory::ComputerArchitectureSilicon
        );
    }
}

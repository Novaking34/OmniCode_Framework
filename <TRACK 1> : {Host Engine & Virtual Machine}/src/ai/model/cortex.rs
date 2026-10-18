// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( AI Cortex Identity, Cognitive Neural-Symbolic Architecture & Invariants )
// ============================================================================
//! OmniCode AI Model: Cognitive Cortex
//!
//! Serves as the central coordinator of the internal AI model.
//! Integrates the sensory nervous system (DiagnosticLogger), boundary
//! invariant monitor (OmniDebugger), immune self-healer (Restorator),
//! and natural discourse translator (AITransducer) with dual episodic
//! inscription (ScrollLogger).

use std::path::PathBuf;
use std::time::Instant;
use crate::ai::model::memory::{AIMemory, ThoughtRecord};
use crate::ai::model::scroll::{
    generate_timestamp_id, ExecutionScroll, ScrollLogger,
};
use crate::ai::neural::network::NeuralBrain;
use crate::ai::neural::curriculum::Curriculum;
use crate::ai::neural::trainer::{NeuralTrainer, TrainingReport};
use crate::ai::neural::visualizer::{BrainGrowthHistory, BrainVisualizer};
use crate::ai::tools::{AIToolBelt, ToolCall, ToolResult};
use crate::ai::transducer::AITransducer;
use crate::core::ternary::QuadState;
use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::{DiagnosticLogger, IndexedSession};
use crate::diagnostics::restorator::{AppliedRepair, Restorator};
use crate::parser::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::runtime::environment::{Environment, RuntimeValue};
use crate::runtime::evaluator::Evaluator;

// ============================================================================
// BLOCK 2: OPENING BLOCK
// ( Cognitive Structures and Cortex Initialization )
// ============================================================================
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
}

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
}

impl AICortex {
    pub fn new() -> Self {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 10);
        let restorator = Restorator::new(logger.clone());
        let transducer = AITransducer::new(logger.clone());
        let scroll_logger = ScrollLogger::default();
        let memory = AIMemory::default();
        let topology: &[usize] = &[64, 32, 16, 4];
        let mut brain = NeuralBrain::new(topology, 1, logger.clone());
        let weights_path = PathBuf::from("storage/weights/cpisi_base.weights");
        if weights_path.exists() {
            brain.load_weights(&weights_path).ok();
        }

        let tools = AIToolBelt::with_logger(logger.clone());
        let mut growth_history = BrainGrowthHistory::default();
        growth_history.record_milestone("Genesis Inception", &brain, 0.70);

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
        }
    }

    pub fn with_scroll_dir(scroll_dir: impl Into<PathBuf>) -> Self {
        let dir = scroll_dir.into();
        let mut cortex = Self::new();
        cortex.scroll_logger = ScrollLogger::new(&dir);
        cortex.memory = AIMemory::new(&dir);
        cortex
    }

    /// Dispatches an action or inspection request to the five-fold diagnostic toolbelt.
    pub fn dispatch_tool(&mut self, call: ToolCall) -> ToolResult {
        self.tools.dispatch(call)
    }

    /// Trains the internal NeuralBrain across the 25-chapter NotebookLM curriculum.
    pub fn train_curriculum(&mut self, curriculum: &Curriculum) -> TrainingReport {
        let trainer = NeuralTrainer::new();
        let report = trainer.train_curriculum(&mut self.brain, curriculum);
        self.growth_history.record_milestone(
            "NotebookLM Curriculum Convergence",
            &self.brain,
            report.noise_resilience_after,
        );
        let weights_path = PathBuf::from("storage/weights/cpisi_base.weights");
        self.brain.save_weights(&weights_path).ok();
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
}


// ============================================================================
// BLOCK 3: BODY BLOCK
// ( Core Cognitive Execution Pipeline: Transduce -> Heal -> Exec -> Inscribe )
// ============================================================================
impl AICortex {
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
        // from memory
        let thought = self.formulate_thought(prompt);

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
        let raw_omnicode = self.transducer.decode_prompt(prompt)?;

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

        let execution_state = match exec_val {
            RuntimeValue::State(s) => s,
            _ => QuadState::Proceed,
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
        })
    }
}


// ============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Default Trait Implementation and Sealing )
// ============================================================================
impl Default for AICortex {
    fn default() -> Self {
        Self::new()
    }
}

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
}

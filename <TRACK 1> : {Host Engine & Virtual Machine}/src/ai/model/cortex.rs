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
}

impl AICortex {
    pub fn new() -> Self {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 10);
        let restorator = Restorator::new(logger.clone());
        let transducer = AITransducer::new(logger.clone());
        let scroll_logger = ScrollLogger::default();
        let memory = AIMemory::default();
        let topology: &[usize] = &[8, 16, 4];
        let mut brain = NeuralBrain::new(topology, 1, logger.clone());
        let weights_path = PathBuf::from("storage/weights/cpisi_base.weights");
        if weights_path.exists() {
            brain.load_weights(&weights_path).ok();
        }

        Self {
            logger,
            debugger,
            restorator,
            transducer,
            scroll_logger,
            memory,
            brain,
        }
    }

    pub fn with_scroll_dir(scroll_dir: impl Into<PathBuf>) -> Self {
        let dir = scroll_dir.into();
        let mut cortex = Self::new();
        cortex.scroll_logger = ScrollLogger::new(&dir);
        cortex.memory = AIMemory::new(&dir);
        cortex
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
        let stimulus = NeuralBrain::encode_stimulus(prompt, 8);
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

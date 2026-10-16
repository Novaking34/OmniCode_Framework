// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( AI Transducer Subsystem Identity, Module Manifest & Exports )
// ============================================================================
//! OmniCode AI: Semantic Transducer & Natural Language Discourse Interface
//!
//! Bridges the Human Layer (natural language SVO discourse) with the Machine
//! Layer (balanced ternary quad-state physics and Rank 1 invariant opcodes).

pub mod decoder;
pub mod encoder;
pub mod executor;
pub mod patterns;
pub mod session;
pub mod templates;

use crate::ai::lexicon::BiblicalLexicon;
use crate::core::ternary::QuadState;
use crate::diagnostics::logger::DiagnosticLogger;
use crate::runtime::environment::Environment;

pub use session::TransductionSession;

/// The primary coordinator for human discourse $\leftrightarrow$ OmniCode reel.
pub struct AITransducer {
    pub logger: DiagnosticLogger,
    pub lexicon: BiblicalLexicon,
}

impl AITransducer {
    /// Constructs a new [`AITransducer`].
    pub fn new(logger: DiagnosticLogger) -> Self {
        Self {
            logger,
            lexicon: BiblicalLexicon::new(),
        }
    }

    /// Full cognitive round-trip: Prompt -> OmniCode -> Execution -> Response.
    pub fn transduce_and_execute(
        &self,
        prompt: &str,
    ) -> Result<TransductionSession, String> {
        self.logger.log(
            "AI",
            format!("Received natural prompt: \"{prompt}\""),
        );

        // 1. Decode natural prompt into canonical 4-Block OmniCode Reel
        let omnicode = self.decode_prompt(prompt)?;
        self.logger.log("AI", "Synthesized canonical OmniCode Turing reel");

        // 2. Execute synthesized reel through VM
        let (execution_state, bindings_summary, env) =
            executor::execute_reel(&omnicode, self.logger.clone())?;

        // 3. Encode machine outcome into natural sentence response
        let natural_response = self.encode_response(
            prompt,
            execution_state,
            &env,
            &self.logger.failpoints(),
        );

        Ok(TransductionSession {
            prompt: prompt.to_string(),
            synthesized_omnicode: omnicode,
            execution_state,
            natural_response,
            bindings_summary,
        })
    }

    /// Decodes a natural English sentence into a canonical 4-Block Reel.
    pub fn decode_prompt(&self, prompt: &str) -> Result<String, String> {
        decoder::decode_prompt(&self.lexicon, prompt)
    }

    /// Encodes machine execution state back into natural language discourse.
    pub fn encode_response(
        &self,
        prompt: &str,
        state: QuadState,
        env: &Environment,
        failpoints: &[crate::diagnostics::logger::LogEvent],
    ) -> String {
        encoder::encode_response(
            prompt,
            state,
            env,
            failpoints,
            &self.lexicon,
        )
    }

    /// Encodes machine execution state and formulated thought into discourse.
    pub fn encode_cognitive_response(
        &self,
        prompt: &str,
        state: QuadState,
        env: &Environment,
        failpoints: &[crate::diagnostics::logger::LogEvent],
        thought: &crate::ai::model::memory::ThoughtRecord,
    ) -> String {
        encoder::encode_response_with_thought(
            prompt,
            state,
            env,
            failpoints,
            &self.lexicon,
            thought,
        )
    }
}

// ============================================================================
// BLOCK 4: CLOSING BLOCK
// ( AI Transducer Default Implementation and Sealing )
// ============================================================================
impl Default for AITransducer {
    fn default() -> Self {
        Self::new(DiagnosticLogger::new(false))
    }
}

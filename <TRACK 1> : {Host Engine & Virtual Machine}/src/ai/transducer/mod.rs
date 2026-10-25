// =============================================================================
// BLOCK 1: METADATA BLOCK
// ( AI Transducer Subsystem Identity, Module Manifest & Exports )
// =============================================================================
//! # OmniCode AI Transducer Subsystem (`mod.rs`)
//!
//! Bridges the Human Layer (natural language discourse) with the Machine Layer
//! (balanced ternary quad-state physics and 4-Block Turing reels).
//!
//! ### Architecture:
//! - [`decoder`]: Natural discourse $\longrightarrow$ 4-Block Turing reels.
//! - [`encoder`]: Thoughts & execution $\longrightarrow$ Natural discourse.
//! - [`executor`]: VM evaluation pipeline and environment binding extraction.
//! - [`patterns`]: Syntactic and morphological sentence parsing.
//! - [`session`]: Transduction session lifecycle and state manifest.
//! - [`svo`]: Continuous SVO composition across 8 parts of speech.
//! - [`templates`]: Dynamic 4-Block AST reel generation engine.

pub mod decoder;
pub mod encoder;
pub mod executor;
pub mod patterns;
pub mod session;
pub mod svo;
pub mod templates;

use crate::ai::lexicon::BiblicalLexicon;
use crate::ai::model::memory::ThoughtRecord;
use crate::core::ternary::QuadState;
use crate::diagnostics::logger::DiagnosticLogger;
use crate::runtime::environment::Environment;

pub use session::TransductionSession;
pub use svo::{SvoClause, SvoComposer, SvoCompositionResult, SvoOperator};

// =============================================================================
// BLOCK 2: OPENING BLOCK
// ( Transducer Coordinator Structure & Default Scaffolding )
// =============================================================================

/// Primary coordinator for human discourse $\longleftrightarrow$ OmniCode reel.
pub struct AITransducer {
    /// Diagnostic sensory logger.
    pub logger: DiagnosticLogger,
    /// Reference to the World Lexicon and Covenant anchor.
    pub lexicon: BiblicalLexicon,
}

impl Default for AITransducer {
    fn default() -> Self {
        Self::new(DiagnosticLogger::new(false))
    }
}

// =============================================================================
// BLOCK 3: BODY BLOCK
// ( Full Cognitive Round-Trip, Decoding, SVO Composition & Encoding )
// =============================================================================

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
        let output = executor::execute_reel(&omnicode, self.logger.clone())?;

        // 3. Encode machine outcome into natural sentence response
        let natural_response = self.encode_response(
            prompt,
            output.execution_state,
            &output.environment,
            &self.logger.failpoints(),
        );

        Ok(TransductionSession::from_output(
            prompt,
            omnicode,
            output,
            natural_response,
        ))
    }

    /// Decodes a natural English sentence into a canonical 4-Block Reel.
    pub fn decode_prompt(&self, prompt: &str) -> Result<String, String> {
        decoder::decode_prompt(&self.lexicon, prompt)
    }

    /// Decodes a prompt with contextual cortical thought deliberation.
    pub fn decode_prompt_with_thought(
        &self,
        prompt: &str,
        thought: Option<&ThoughtRecord>,
    ) -> Result<String, String> {
        decoder::decode_prompt_with_thought(&self.lexicon, prompt, thought)
    }

    /// Composes a conversational prompt algebraically into nested SVO
    /// directional expressions verified through the Two-Way Checksum gate.
    pub fn compose_svo_with_parity_gate(
        &self,
        prompt: &str,
        debugger: &crate::diagnostics::debugger::OmniDebugger,
        restorator: &crate::diagnostics::restorator::Restorator,
    ) -> SvoCompositionResult {
        SvoComposer::compose_and_verify(
            prompt,
            &self.logger,
            debugger,
            restorator,
        )
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
        thought: &ThoughtRecord,
    ) -> String {
        encoder::encode_response_with_thought(
            prompt,
            state,
            env,
            failpoints,
            &self.lexicon,
            Some(thought),
        )
    }
}

// =============================================================================
// BLOCK 4: AUDIT PROCEDURES BLOCK
// ( Master Self-Diagnostic Verification for Entire Transducer Subsystem )
// =============================================================================

/// Executes master self-diagnostic audit across all Transducer modules.
pub fn audit_transducer_subsystem() -> Result<(), String> {
    session::audit_session_subsystem()?;
    patterns::audit_patterns_subsystem()?;
    executor::audit_executor_subsystem()?;
    svo::audit_svo_subsystem()?;
    templates::audit_templates_subsystem()?;
    decoder::audit_decoder_subsystem()?;
    encoder::audit_encoder_subsystem()?;
    Ok(())
}

// =============================================================================
// BLOCK 5: UNIT TESTS BLOCK
// ( 100% Hermetic Unit Testing )
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_entire_transducer_subsystem() {
        assert!(audit_transducer_subsystem().is_ok());
    }

    #[test]
    fn test_transducer_round_trip_full() {
        let transducer = AITransducer::default();
        let prompt = "Add 2 balls to the group A which contains 3 balls";
        let session = transducer
            .transduce_and_execute(prompt)
            .expect("Transduction should succeed");

        assert_eq!(session.execution_state, QuadState::Proceed);
        assert!(session.synthesized_omnicode.contains("roll_1 = 5"));
        assert!(session.natural_response.contains("5 balls in total"));
    }
}

// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( AI Model Subsystem Architecture, Subsystem Manifest & Re-exports )
// ============================================================================
//! OmniCode Internal AI Model Subsystem (`model/mod.rs`)
//!
//! Organizes the core cognitive neural-symbolic engine:
//!   - `cortex`: Central coordinator orchestrating the nervous system.
//!   - `attractor`: Sparse associative memory with balanced-ternary resonance.
//!   - `memory`: 3-tier cognitive memory with modal triangulation.
//!   - `discourse`: Multi-turn conversational vocal chords.
//!   - `scroll`: Dual-format episodic logging (.scroll and .log).
//!   - `learner`: Supervised training pair extraction from past reels.
//!
//! ### Key Invariants:
//! 1. **Modular 5-Block Structure**: All child modules adhere to 5 blocks.
//! 2. **Cognitive Decoupling**: Thought formulation strictly precedes speech.
//! 3. **Unified Auditability**: Central `ModelSubsystemAudit` aggregates
//!    subsystem health across all components.
//! 4. **Strict Line Length Invariant**: Every line must be <= 80 characters.

pub mod attractor;
pub mod cortex;
pub mod discourse;
pub mod learner;
pub mod memory;
pub mod scroll;

// ============================================================================
// BLOCK 2: TYPES BLOCK
// ( AI Model Re-exports & Subsystem Audit Manifest )
// ============================================================================

pub use attractor::{
    AttractorBasin, AttractorMatch, AttractorSubsystemAudit,
    EpisodicAttractorNetwork,
};
pub use cortex::{
    AICortex, CognitiveNote, CognitiveResult, CortexSubsystemAudit,
};
pub use discourse::{
    ConversationTurn, DiscourseEngine, DiscourseSession,
    DiscourseSubsystemAudit, DiscourseTopic,
};
pub use learner::{
    AILearner, EpisodicExperience, EpisodicSummary, LearnerSubsystemAudit,
};
pub use memory::{
    AIMemory, CompletedSessionSnapshot, ConsolidationCandidate,
    ConsolidationStatus, MemoryAssociation, MemoryAuditReport, MemoryTier,
    ThoughtRecord, TriangulationOutcome,
};
pub use scroll::{ExecutionScroll, ScrollLogger, ScrollSubsystemAudit};

use crate::core::ternary::QuadState;

/// Comprehensive composite audit covering all six model components.
#[derive(Debug, Clone, PartialEq)]
pub struct ModelSubsystemAudit {
    pub status: QuadState,
    pub cortex_audit: CortexSubsystemAudit,
    pub attractor_audit: AttractorSubsystemAudit,
    pub discourse_audit: DiscourseSubsystemAudit,
    pub learner_audit: LearnerSubsystemAudit,
    pub scroll_audit: ScrollSubsystemAudit,
    pub errors: Vec<String>,
}

// ============================================================================
// BLOCK 3: BODY BLOCK
// ( Subsystem Audit Execution & Coordinator Procedures )
// ============================================================================

/// Runs a comprehensive health audit across all components of the AI model.
pub fn audit_entire_model_subsystem(cortex: &AICortex) -> ModelSubsystemAudit {
    let mut errors = Vec::new();

    let cortex_audit = cortex.audit_cortex_subsystem();
    if cortex_audit.status != QuadState::Proceed {
        errors.push("Cortex audit reported non-proceed state".to_string());
    }

    let attractor_audit = cortex.memory.attractors.audit_attractor_subsystem();
    if attractor_audit.status != QuadState::Proceed {
        errors.push("Attractor audit reported non-proceed state".to_string());
    }

    let discourse_audit = DiscourseEngine::audit_discourse_subsystem();
    if discourse_audit.status != QuadState::Proceed {
        errors.push("Discourse audit reported non-proceed state".to_string());
    }

    let learner = AILearner::new(&cortex.scroll_logger.scroll_dir);
    let learner_audit = learner.audit_learner_subsystem();
    if learner_audit.status != QuadState::Proceed {
        errors.push("Learner audit reported non-proceed state".to_string());
    }

    let scroll_audit = cortex.scroll_logger.audit_scroll_subsystem();
    if scroll_audit.status != QuadState::Proceed {
        errors.push("Scroll audit reported non-proceed state".to_string());
    }

    let status = if errors.is_empty() {
        QuadState::Proceed
    } else {
        QuadState::Rollback
    };

    ModelSubsystemAudit {
        status,
        cortex_audit,
        attractor_audit,
        discourse_audit,
        learner_audit,
        scroll_audit,
        errors,
    }
}

// ============================================================================
// BLOCK 4: AUDIT PROCEDURES BLOCK
// ( Invariant Proving and Subsystem Integrity Verification )
// ============================================================================

impl ModelSubsystemAudit {
    /// Returns true if all six sub-audits succeeded in +1:PROCEED.
    pub fn is_healthy(&self) -> bool {
        self.status == QuadState::Proceed && self.errors.is_empty()
    }
}

// ============================================================================
// BLOCK 5: UNIT TESTS & INVARIANT VERIFICATION SUITE
// ( Composite Subsystem Health & Re-export Integrity )
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_entire_model_subsystem() {
        let cortex = AICortex::new();
        let audit = audit_entire_model_subsystem(&cortex);
        assert!(
            audit.is_healthy(),
            "Complete model subsystem audit must pass: {:?}",
            audit.errors
        );
        assert_eq!(audit.status, QuadState::Proceed);
        assert_eq!(audit.cortex_audit.status, QuadState::Proceed);
        assert_eq!(audit.attractor_audit.status, QuadState::Proceed);
        assert_eq!(audit.discourse_audit.status, QuadState::Proceed);
        assert_eq!(audit.scroll_audit.status, QuadState::Proceed);
    }
}

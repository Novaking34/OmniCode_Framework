// =============================================================================
// BLOCK 1: METADATA BLOCK
// ( Multi-Turn Conversation Session, Persistent VM State & Learning Container )
// =============================================================================
//
// @module      :: omnicode::ai::transducer::session
// @file        :: src/ai/transducer/session.rs
// @layer       :: Track 1 / AI Subsystem / Transducer / Session Engine
// @version     :: 2.0.0
// @epoch       :: Radix-3 Balanced Ternary Genesis
// @author      :: OmniCode Systems Architecture Group
// @system      :: CPI-SI Autonomic AI & VM Pipeline
// @repository  :: OmniCode_Framework
// @role        :: Living container for continuous human-machine conversation.
//                 Preserves VM environment across dialogue turns, tracks
//                 spatial proof sheets, records tool invocations, and
//                 consolidates learned entities for Track 4 episodic memory.
//
// @invariants  ::
//   1. Multi-Turn Causal Monotonicity: Turns advance sequentially (dt >= 0)
//      without losing historical context or environment vessels.
//   2. Environment Continuity: VM vessels and bindings accumulate across turns.
//   3. Dynamic Entity Learning: Discovered entities persist in session scope.
//   4. Strict 80-Column Geometry: Every source line remains <= 80 characters.
//   5. Radix-3 Balanced Ternary: Conversational state commits to
//      {+1: PROCEED, -1: ROLLBACK, 0: AWAIT, NULL: HALT}.
//
// @witness     ::
//   - Malachi 3:16 (KJV): "Then they that feared the LORD spake often one to
//     another: and the LORD hearkened, and heard it, and a book of remembrance
//     was written before him..." (The permanent conversational session record)
//   - Job 8:8 (KJV): "For enquire, I pray thee, of the former age, and prepare
//     thyself to the search of their fathers." (Fast state recall across
//     conversational turns)
//   - 1 Corinthians 14:40 (KJV): "Let all things be done decently and in
//     order." (Immutable invariant of structural order and discipline)
//
// =============================================================================

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::ai::transducer::executor::ReelExecutionOutput;
use crate::ai::transducer::svo::{SvoClause, SvoEntity};
use crate::core::ternary::QuadState;
use crate::diagnostics::restorator::AppliedRepair;
use crate::runtime::environment::Environment;

// =============================================================================
// BLOCK 2: OPENING BLOCK
// ( Core Structures, Multi-Turn State & Construction Invariants )
// =============================================================================

/// An individual dialogue turn within a multi-turn conversation session.
#[derive(Debug, Clone, PartialEq)]
pub struct SessionTurn {
    /// Zero-indexed turn number.
    pub turn_index: usize,
    /// Timestamp (microseconds since UNIX epoch) when this turn executed.
    pub timestamp_micros: u128,
    /// Human prompt for this turn.
    pub prompt: String,
    /// Synthesized 4-Block OmniCode proof sheet for this turn.
    pub synthesized_omnicode: String,
    /// Execution state from the VM evaluation.
    pub execution_state: QuadState,
    /// Natural response delivered to the operator.
    pub natural_response: String,
    /// Bindings summary at the end of this turn.
    pub bindings_summary: HashMap<String, String>,
    /// Deliberated SVO clauses extracted from the prompt.
    pub svo_clauses: Vec<SvoClause>,
    /// Repairs applied by the Restorator during this turn.
    pub repairs_applied: Vec<AppliedRepair>,
}

/// Living container for continuous human-machine conversation and state.
#[derive(Debug, Clone)]
pub struct TransductionSession {
    /// Unique identifier for this session (e.g., "sess_1729641600000").
    pub session_id: String,
    /// Latest human prompt.
    pub prompt: String,
    /// Latest synthesized 4-Block OmniCode Turing Reel proof sheet.
    pub synthesized_omnicode: String,
    /// Current verified balanced-ternary execution state.
    pub execution_state: QuadState,
    /// Latest natural language response formulated for the operator.
    pub natural_response: String,
    /// Environment variable bindings after latest VM execution.
    pub bindings_summary: HashMap<String, String>,
    /// Persistent runtime environment accumulated across turns.
    pub environment: Environment,
    /// Chronological multi-turn dialogue history.
    pub turns: Vec<SessionTurn>,
    /// Novel entities learned during this session.
    pub learned_entities: Vec<SvoEntity>,
    /// Timestamp (microseconds) when the session was initiated.
    pub start_time_micros: u128,
}

impl PartialEq for TransductionSession {
    fn eq(&self, other: &Self) -> bool {
        self.session_id == other.session_id
            && self.prompt == other.prompt
            && self.synthesized_omnicode == other.synthesized_omnicode
            && self.execution_state == other.execution_state
            && self.natural_response == other.natural_response
            && self.bindings_summary == other.bindings_summary
            && self.turns == other.turns
            && self.learned_entities == other.learned_entities
    }
}

// =============================================================================
// BLOCK 3: BODY BLOCK
// ( Session Lifecycle, Multi-Turn Advancement & Memory Continuity )
// =============================================================================

impl TransductionSession {
    /// Creates a new transduction session record with default environment.
    pub fn new(
        prompt: impl Into<String>,
        synthesized_omnicode: impl Into<String>,
        execution_state: QuadState,
        natural_response: impl Into<String>,
        bindings_summary: HashMap<String, String>,
    ) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_micros())
            .unwrap_or(0);
        let session_id = format!("sess_{now}");
        let p = prompt.into();
        let s = synthesized_omnicode.into();
        let n = natural_response.into();

        let initial_turn = SessionTurn {
            turn_index: 0,
            timestamp_micros: now,
            prompt: p.clone(),
            synthesized_omnicode: s.clone(),
            execution_state,
            natural_response: n.clone(),
            bindings_summary: bindings_summary.clone(),
            svo_clauses: Vec::new(),
            repairs_applied: Vec::new(),
        };

        Self {
            session_id,
            prompt: p,
            synthesized_omnicode: s,
            execution_state,
            natural_response: n,
            bindings_summary,
            environment: Environment::new(),
            turns: vec![initial_turn],
            learned_entities: Vec::new(),
            start_time_micros: now,
        }
    }

    /// Constructs a session directly from a verified [`ReelExecutionOutput`].
    pub fn from_output(
        prompt: impl Into<String>,
        synthesized_omnicode: impl Into<String>,
        output: ReelExecutionOutput,
        natural_response: impl Into<String>,
    ) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_micros())
            .unwrap_or(0);
        let session_id = format!("sess_{now}");
        let p = prompt.into();
        let s = synthesized_omnicode.into();
        let n = natural_response.into();

        let initial_turn = SessionTurn {
            turn_index: 0,
            timestamp_micros: now,
            prompt: p.clone(),
            synthesized_omnicode: s.clone(),
            execution_state: output.execution_state,
            natural_response: n.clone(),
            bindings_summary: output.bindings_summary.clone(),
            svo_clauses: Vec::new(),
            repairs_applied: output.repairs_applied,
        };

        Self {
            session_id,
            prompt: p,
            synthesized_omnicode: s,
            execution_state: output.execution_state,
            natural_response: n,
            bindings_summary: output.bindings_summary,
            environment: output.environment,
            turns: vec![initial_turn],
            learned_entities: Vec::new(),
            start_time_micros: now,
        }
    }

    /// Advances the conversation by adding a new turn to the dialogue tape.
    ///
    /// Preserves existing environment vessels, merges new bindings, and
    /// updates latest session state.
    pub fn advance_turn(
        &mut self,
        prompt: &str,
        synthesized_omnicode: &str,
        output: ReelExecutionOutput,
        natural_response: &str,
        svo_clauses: Vec<SvoClause>,
    ) -> &SessionTurn {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_micros())
            .unwrap_or(0);

        let turn_index = self.turns.len();
        self.prompt = prompt.to_string();
        self.synthesized_omnicode = synthesized_omnicode.to_string();
        self.execution_state = output.execution_state;
        self.natural_response = natural_response.to_string();
        self.bindings_summary = output.bindings_summary.clone();
        self.environment = output.environment;

        // Record any novel entities discovered in this turn
        for clause in &svo_clauses {
            if clause.subject.is_novel {
                self.record_learned_entity(clause.subject.clone());
            }
            if clause.object.is_novel {
                self.record_learned_entity(clause.object.clone());
            }
        }

        let turn = SessionTurn {
            turn_index,
            timestamp_micros: now,
            prompt: prompt.to_string(),
            synthesized_omnicode: synthesized_omnicode.to_string(),
            execution_state: output.execution_state,
            natural_response: natural_response.to_string(),
            bindings_summary: output.bindings_summary,
            svo_clauses,
            repairs_applied: output.repairs_applied,
        };

        self.turns.push(turn);
        self.turns.last().unwrap()
    }

    /// Records a novel learned entity into the session's active vocabulary.
    pub fn record_learned_entity(&mut self, entity: SvoEntity) {
        if !self.learned_entities.iter().any(|e| e.name == entity.name) {
            self.learned_entities.push(entity);
        }
    }

    /// Returns the names of all entities currently active in session scope.
    pub fn active_entities(&self) -> Vec<String> {
        let mut set = Vec::new();
        for e in &self.learned_entities {
            if !set.contains(&e.name) {
                set.push(e.name.clone());
            }
        }
        for k in self.bindings_summary.keys() {
            if !set.contains(k) {
                set.push(k.clone());
            }
        }
        set
    }

    /// Resolves an active vessel binding by name.
    pub fn resolve_vessel(&self, name: &str) -> Option<&String> {
        self.bindings_summary.get(name)
    }

    /// Returns the total number of dialogue turns in this session.
    pub fn turn_count(&self) -> usize {
        self.turns.len()
    }

    /// Returns the total count of repairs applied by the Restorator.
    pub fn total_repairs(&self) -> usize {
        self.turns.iter().map(|t| t.repairs_applied.len()).sum()
    }

    /// Returns `true` if the latest turn completed in equilibrium
    /// (+1: Proceed).
    pub fn is_proceed(&self) -> bool {
        self.execution_state == QuadState::Proceed
    }

    /// Returns `true` if the session is awaiting epistemic grounding
    /// (0: Await).
    pub fn is_await(&self) -> bool {
        self.execution_state == QuadState::Await
    }

    /// Returns `true` if the session rolled back due to a caveat
    /// (-1: Rollback).
    pub fn is_rollback(&self) -> bool {
        self.execution_state == QuadState::Rollback
    }

    /// Formats a multi-turn summary of the conversation trajectory.
    pub fn summary(&self) -> String {
        format!(
            "Session '{}': turns={}, state={:?}, entities={}, \
             repairs={}, bindings={}",
            self.session_id,
            self.turns.len(),
            self.execution_state,
            self.learned_entities.len(),
            self.total_repairs(),
            self.bindings_summary.len()
        )
    }
}

// =============================================================================
// BLOCK 4: AUDIT PROCEDURES BLOCK
// ( Self-Diagnostic Verification for Multi-Turn Session Continuity )
// =============================================================================

/// Executes automated self-diagnostic audit for the session subsystem.
pub fn audit_session_subsystem() -> Result<(), String> {
    let mut bindings = HashMap::new();
    bindings.insert("basket".to_string(), "5".to_string());

    let mut session = TransductionSession::new(
        "There are 5 loaves in the basket",
        "Module < Test.Turn1 > () {}",
        QuadState::Proceed,
        "Confirmed 5 loaves in the basket.",
        bindings,
    );

    if !session.is_proceed() {
        return Err("Audit failed: Session did not report Proceed".into());
    }
    if session.is_await() || session.is_rollback() {
        return Err("Audit failed: Mutually exclusive state collided".into());
    }
    if session.resolve_vessel("basket").map(|s| s.as_str()) != Some("5") {
        return Err("Audit failed: Vessel 'basket' missing or corrupted".into());
    }
    if session.turn_count() != 1 {
        return Err("Audit failed: Expected turn_count=1".into());
    }

    // Advance to Turn 2
    let mut turn2_bindings = HashMap::new();
    turn2_bindings.insert("basket".to_string(), "8".to_string());

    let turn2_output = ReelExecutionOutput {
        execution_state: QuadState::Proceed,
        bindings_summary: turn2_bindings,
        environment: Environment::new(),
        repairs_applied: Vec::new(),
        audit_passed: true,
        telemetry_frames: 2,
    };

    session.advance_turn(
        "Add 3 more loaves",
        "Module < Test.Turn2 > () {}",
        turn2_output,
        "Added 3 loaves; basket now contains 8.",
        Vec::new(),
    );

    if session.turn_count() != 2 {
        return Err("Audit failed: Turn count did not increment to 2".into());
    }
    if session.resolve_vessel("basket").map(|s| s.as_str()) != Some("8") {
        return Err("Audit failed: Vessel 'basket' did not update to 8".into());
    }

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
    fn test_transduction_session_lifecycle() {
        let mut bindings = HashMap::new();
        bindings.insert("res".into(), "42".into());

        let s = TransductionSession::new(
            "What is 42?",
            "# 4-Block Reel",
            QuadState::Proceed,
            "The answer is 42.",
            bindings,
        );

        assert!(s.is_proceed());
        assert!(!s.is_await());
        assert!(!s.is_rollback());
        assert_eq!(s.prompt, "What is 42?");
        assert!(s.summary().contains("Proceed"));
        assert_eq!(s.turn_count(), 1);
    }

    #[test]
    fn test_session_audit_procedure() {
        assert!(audit_session_subsystem().is_ok());
    }

    #[test]
    fn test_session_multi_turn_accumulation() {
        let mut s = TransductionSession::new(
            "Turn 1 prompt",
            "Turn 1 reel",
            QuadState::Proceed,
            "Turn 1 response",
            HashMap::new(),
        );

        assert_eq!(s.turn_count(), 1);

        let mut turn2_bindings = HashMap::new();
        turn2_bindings.insert("count".into(), "10".into());
        let turn2_output = ReelExecutionOutput {
            execution_state: QuadState::Proceed,
            bindings_summary: turn2_bindings,
            environment: Environment::new(),
            repairs_applied: Vec::new(),
            audit_passed: true,
            telemetry_frames: 1,
        };

        s.advance_turn(
            "Turn 2 prompt",
            "Turn 2 reel",
            turn2_output,
            "Turn 2 response",
            Vec::new(),
        );

        assert_eq!(s.turn_count(), 2);
        assert_eq!(s.prompt, "Turn 2 prompt");
        assert_eq!(s.resolve_vessel("count"), Some(&"10".to_string()));
    }

    #[test]
    fn test_session_learned_entity_persistence() {
        let mut s = TransductionSession::new(
            "Turn 1",
            "Reel 1",
            QuadState::Proceed,
            "Response 1",
            HashMap::new(),
        );

        let entity = SvoEntity {
            name: "Apparatus".into(),
            boundary: crate::core::boundaries::BoundaryType::Denotation,
            type_descriptor: "Vessel".into(),
            modal_domain: None,
            is_operator: false,
            is_novel: true,
        };

        s.record_learned_entity(entity);
        assert_eq!(s.learned_entities.len(), 1);
        assert!(s.active_entities().contains(&"Apparatus".to_string()));
    }
}

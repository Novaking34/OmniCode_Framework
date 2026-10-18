// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( OmniDebugger Facade: The Focused Eyes of the DAR Cybernetic Loop )
// ==============================================================================
// SCRIPTURAL WITNESS:
//   "Neither is there any creature that is not manifest in his sight: but all
//    things are naked and opened unto the eyes of him with whom we have to do."
//   — Hebrews 4:13 (KJV)
//
//   "The eyes of the LORD are in every place, beholding the evil and the good."
//   — Proverbs 15:3 (KJV)
//
// ARCHITECTURAL TOPOLOGY:
//   The focused inspection, invariant assessment, and test oracle organ of the
//   OmniCode Host Engine (Track 1).
//
//   While the DiagnosticLogger functions as ambient sensory perception—recording
//   the environment as a whole (timestamps, serial logs, hardware landmarks,
//   ring buffers)—the OmniDebugger serves as the focused pair of eyes:
//     - Gazing at boundary stacks: trapping leaks and illegal crossings (`boundary.rs`).
//     - Gazing at immutable laws: guarding Rank 1 invariants `<...>` against mutation (`immutable.rs`).
//     - Gazing at causal momentum: tracking rollback loops to halt retroaction traps (`retroaction.rs`).
//     - Gazing at execution truth: inspecting QuadState (+1, 0, -1, NULL) transitions (`state.rs`).
//     - Gazing at candidate repairs: certifying patches with empirical test oracles (`oracle.rs`).
//     - Passing observations: inscribing cognitive notes and probe frames into the logger (`telemetry.rs`).
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Submodule Declarations, Imports & OmniDebugger Context Definition )
// ==============================================================================
pub mod boundary;
pub mod immutable;
pub mod oracle;
pub mod retroaction;
pub mod state;
pub mod telemetry;

use std::collections::{HashMap, HashSet};

use crate::core::boundaries::BoundaryType;
use crate::core::ternary::QuadState;
use crate::diagnostics::logger::{DiagnosticLogger, FailpointType, FixKind, FixSuggestion, LogEvent};

pub use boundary::{BoundaryFrame, BoundaryTracker};
pub use immutable::ImmutableRegistry;
pub use oracle::TestOracleSandbox;
pub use retroaction::RetroactionTracker;
pub use state::StateInspector;

/// Runtime invariant inspector, boundary stack tracker, and test verification oracle.
/// Functions as the focused pair of eyes within the DAR (Detect, Assess, Restore) cybernetic loop.
#[derive(Debug, Clone)]
pub struct OmniDebugger {
    /// Shared diagnostic logger for emitting debug events, failpoints, and cognitive traces.
    pub logger: DiagnosticLogger,
    /// Active LIFO stack of unclosed boundary frames.
    pub boundary_stack: Vec<BoundaryFrame>,
    /// Set of entity names declared inside `< ... >` sealed against mutation.
    pub immutable_registry: HashSet<String>,
    /// Execution rollback counters keyed by target label to detect infinite retroaction loops.
    pub retroaction_counters: HashMap<String, usize>,
    /// Staged queue of automated remediation suggestions awaiting application by the Restorator.
    pub remediation_queue: Vec<FixSuggestion>,
    /// Maximum permitted rollback operations per target before triggering cyclic retroaction failure.
    pub max_retroaction_depth: usize,
    /// Total operational steps and state evaluations inspected by this debugger.
    pub step_count: usize,
}

impl OmniDebugger {
    /// Constructs a new [`OmniDebugger`].
    ///
    /// # Arguments
    /// * `logger` - The shared [`DiagnosticLogger`] instance for ambient telemetry recording.
    /// * `max_retroaction_depth` - Threshold ceiling for retroactions before trapping cyclic loops.
    pub fn new(logger: DiagnosticLogger, max_retroaction_depth: usize) -> Self {
        telemetry::log_debugger_focus(
            &logger,
            "DEBUGGER",
            format!("Armed focused eye with retroaction depth ceiling {max_retroaction_depth}"),
        );

        Self {
            logger,
            boundary_stack: Vec::new(),
            immutable_registry: HashSet::new(),
            retroaction_counters: HashMap::new(),
            remediation_queue: Vec::new(),
            max_retroaction_depth,
            step_count: 0,
        }
    }
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Core Invariant Enforcement: Boundary Push/Pop, Immutability, Retroaction, Oracles )
// ==============================================================================
impl OmniDebugger {
    /// Pushes an opening boundary delimiter onto the active stack.
    ///
    /// # Arguments
    /// * `open` - Delimiter character (`<`, `[`, `{`, `(`).
    /// * `line` - 1-based line number.
    /// * `col` - 1-based column number.
    pub fn push_boundary(&mut self, open: char, line: usize, col: usize) {
        let frame = BoundaryFrame::new(open, line, col);
        let depth = self.boundary_stack.len() + 1;
        self.logger.log(
            "BOUNDARY",
            format!("Gaze Focus: Pushing delimiter '{open}' at L{line}:C{col} (depth: {depth})"),
        );
        self.boundary_stack.push(frame);
        telemetry::record_debugger_probe_frame(&self.logger, self.step_count, "PUSH_BOUNDARY", QuadState::Proceed);
    }

    /// Pops a closing boundary delimiter and validates the pair against the stack top.
    ///
    /// # Failpoints Intercepted
    /// - **Empty Stack**: Traps unmatched closing delimiter as [`FailpointType::BoundaryMismatch`].
    /// - **Mismatched Delimiter**: Traps illegal pair crossing (e.g. `< ... ]`), logs a
    ///   [`FixSuggestion`] pointing to the correct partner, and queues remediation.
    ///
    /// # Returns
    /// `Ok(BoundaryType)` on valid resolution, or `Err(String)` on failpoint mismatch.
    pub fn pop_boundary(&mut self, close: char, line: usize, col: usize) -> Result<BoundaryType, String> {
        if self.boundary_stack.is_empty() {
            let msg = format!("Unmatched closing delimiter '{close}' at L{line}:C{col} (stack is empty)");
            self.logger.failpoint(FailpointType::BoundaryMismatch, "DEBUGGER", &msg);
            telemetry::record_debugger_cognitive_note(
                &self.logger,
                "[Debugger] Unmatched Delimiter Error",
                &msg,
                &["debugger", "failpoint", "boundary", "empty_stack"],
                Some(&close.to_string()),
            );
            return Err(format!("FAILPOINT: {msg}"));
        }

        let frame = self.boundary_stack.pop().unwrap();
        match BoundaryType::from_delimiters(frame.open_char, close) {
            Ok(btype) => {
                self.logger.log(
                    "BOUNDARY",
                    format!(
                        "Gaze Focus: Successfully resolved boundary {} '{}{}' (remaining depth: {})",
                        btype, frame.open_char, close, self.boundary_stack.len()
                    ),
                );
                telemetry::record_debugger_probe_frame(&self.logger, self.step_count, "POP_BOUNDARY", QuadState::Proceed);
                Ok(btype)
            }
            Err(err) => {
                let msg = format!(
                    "Boundary mismatch: opened with '{}' (L{}:C{}) but closed with '{}' at L{}:C{}",
                    frame.open_char, frame.line, frame.col, close, line, col
                );
                let matching = frame.matching_closing_char();
                let suggestion = FixSuggestion {
                    kind: FixKind::FixBoundaryMismatch,
                    line,
                    col,
                    target_token: close.to_string(),
                    suggested_patch: matching.to_string(),
                    rationale: format!(
                        "Replace mismatched closing delimiter '{close}' with matching '{matching}' to seal boundary opened with '{}' at L{}:C{}",
                        frame.open_char, frame.line, frame.col
                    ),
                };

                self.logger.failpoint_with_remediation(
                    FailpointType::BoundaryMismatch,
                    "DEBUGGER",
                    &msg,
                    suggestion.clone(),
                );

                telemetry::record_debugger_cognitive_note(
                    &self.logger,
                    "[Debugger] Boundary Delimiter Mismatch",
                    &msg,
                    &["debugger", "failpoint", "boundary", "mismatch"],
                    Some(&format!("{}{}", frame.open_char, close)),
                );

                self.remediation_queue.push(suggestion);
                Err(err)
            }
        }
    }

    /// Asserts that no boundaries remain unclosed at EOF.
    ///
    /// If unclosed delimiters remain on the stack, logs [`FailpointType::BoundaryLeak`]
    /// with a concrete [`FixSuggestion`] indicating the required closing tokens.
    pub fn assert_all_boundaries_closed(&mut self) -> Result<(), String> {
        if let Some(frame) = self.boundary_stack.last() {
            let msg = format!(
                "Boundary leak: delimiter '{}' opened at L{}:C{} was never closed",
                frame.open_char, frame.line, frame.col
            );
            let matching = frame.matching_closing_char();
            let suggestion = FixSuggestion {
                kind: FixKind::CloseBoundary,
                line: frame.line,
                col: frame.col,
                target_token: frame.open_char.to_string(),
                suggested_patch: format!("\n{matching}"),
                rationale: format!(
                    "Append missing closing delimiter '{matching}' for boundary opened with '{}' at L{}:C{}",
                    frame.open_char, frame.line, frame.col
                ),
            };

            self.logger.failpoint_with_remediation(
                FailpointType::BoundaryLeak,
                "DEBUGGER",
                &msg,
                suggestion.clone(),
            );

            telemetry::record_debugger_cognitive_note(
                &self.logger,
                "[Debugger] Boundary Delimiter Leak",
                &msg,
                &["debugger", "failpoint", "boundary", "leak"],
                Some(&frame.open_char.to_string()),
            );

            self.remediation_queue.push(suggestion);
            return Err(format!("FAILPOINT: {msg}"));
        }
        Ok(())
    }

    /// Registers an entity identifier defined inside `< ... >` as globally immutable Rank 1 axiom.
    pub fn register_immutable(&mut self, name: &str) {
        self.immutable_registry.insert(name.to_string());
        self.logger.log(
            "IMMUTABILITY",
            format!("Gaze Focus: Registered invariant <{name}>"),
        );
        telemetry::record_debugger_cognitive_note(
            &self.logger,
            &format!("[Debugger] Registered Invariant <{name}>"),
            &format!("Sealed entity '{name}' as a Rank 1 immutable axiom against mutation"),
            &["debugger", "immutability", "rank_1", "axiom"],
            Some(name),
        );
    }

    /// Verifies that mutation is permitted for the given entity identifier.
    ///
    /// # Failpoints Intercepted
    /// If the entity was registered as immutable, traps [`FailpointType::ImmutableMutation`],
    /// and suggests converting the declaration to a mutable Rank 2 denotation `[ ... ]`.
    pub fn assert_mutation_permitted(&mut self, name: &str) -> Result<(), String> {
        if self.immutable_registry.contains(name) {
            let msg = format!(
                "Illegal mutation attempted on immutable invariant <{name}>! Entity cannot be changed."
            );
            let suggestion = FixSuggestion {
                kind: FixKind::ConvertImmutableToMutable,
                line: 0,
                col: 0,
                target_token: name.to_string(),
                suggested_patch: format!("[ {name} ]"),
                rationale: format!(
                    "Entity <{name}> was sealed as a Rank 1 immutable invariant; declare it with '[ {name} ]' if updates are required"
                ),
            };

            self.logger.failpoint_with_remediation(
                FailpointType::ImmutableMutation,
                "DEBUGGER",
                &msg,
                suggestion.clone(),
            );

            telemetry::record_debugger_cognitive_note(
                &self.logger,
                &format!("[Debugger] Illegal Mutation Veto <{name}>"),
                &msg,
                &["debugger", "failpoint", "immutability", "veto"],
                Some(name),
            );

            self.remediation_queue.push(suggestion);
            return Err(format!("FAILPOINT: {msg}"));
        }
        Ok(())
    }

    /// Retrieves and empties the active remediation queue.
    pub fn drain_remediations(&mut self) -> Vec<FixSuggestion> {
        std::mem::take(&mut self.remediation_queue)
    }

    /// Monitors retroaction steps on a named target to prevent infinite rollback loops.
    ///
    /// # Failpoints Intercepted
    /// If rollback iterations exceed `max_retroaction_depth`, traps [`FailpointType::CyclicRetroaction`].
    pub fn track_retroaction(&mut self, target: &str) -> Result<usize, String> {
        let count = self.retroaction_counters.entry(target.to_string()).or_insert(0);
        *count += 1;
        let cur = *count;

        self.logger.log(
            "VECTOR",
            format!("Gaze Focus: Retroaction step #{cur} on '{target}' (ceiling: {})", self.max_retroaction_depth),
        );

        if cur > self.max_retroaction_depth {
            let msg = format!(
                "Cyclic retroaction loop detected! Target '{target}' rolled back {cur} times, \
                 exceeding threshold ({})",
                self.max_retroaction_depth
            );
            self.logger.failpoint(FailpointType::CyclicRetroaction, "DEBUGGER", &msg);
            telemetry::record_debugger_cognitive_note(
                &self.logger,
                &format!("[Debugger] Cyclic Retroaction Loop on '{target}'"),
                &msg,
                &["debugger", "failpoint", "retroaction", "loop"],
                Some(target),
            );
            return Err(format!("FAILPOINT: {msg}"));
        }

        Ok(cur)
    }

    /// Inspects QuadState and raises appropriate signals or failpoints.
    ///
    /// - `QuadState::Halt`: Traps void / NULL halt trap (`FailpointType::NullTrap`).
    /// - `QuadState::Rollback`: Traps rollback trigger (`FailpointType::RollbackTriggered`).
    /// - `QuadState::Await`: Standby state logged.
    /// - `QuadState::Proceed`: Commit state logged.
    pub fn inspect_state(&mut self, state: QuadState, context: &str) {
        self.step_count += 1;
        let step = self.step_count;
        telemetry::record_debugger_probe_frame(&self.logger, step, "INSPECT_STATE", state);

        match state {
            QuadState::Halt => {
                self.logger.failpoint(
                    FailpointType::NullTrap,
                    "DEBUGGER",
                    format!("Gaze Focus [Step #{step}]: NULL / Void halt trap encountered in context: {context}"),
                );
            }
            QuadState::Rollback => {
                self.logger.failpoint(
                    FailpointType::RollbackTriggered,
                    "DEBUGGER",
                    format!("Gaze Focus [Step #{step}]: Rollback (-1) triggered in context: {context}"),
                );
            }
            QuadState::Await => {
                self.logger.log("TERNARY", format!("Gaze Focus [Step #{step}]: Await (0) standby in context: {context}"));
            }
            QuadState::Proceed => {
                self.logger.log("TERNARY", format!("Gaze Focus [Step #{step}]: Proceed (+1) committed in context: {context}"));
            }
        }
    }

    /// Executes a verification test against an isolated child debugger context.
    ///
    /// # Arguments
    /// * `test_name` - Human and AI readable identifier for the verification assertion.
    /// * `test_runner` - Closure executing the test logic against a sandboxed child debugger.
    pub fn execute_verification_test<F>(&mut self, test_name: &str, test_runner: F) -> Result<(), String>
    where
        F: FnOnce(&mut OmniDebugger) -> Result<(), String>,
    {
        self.logger.log("TEST_ORACLE", format!("Gaze Focus: Spawning verification test: {test_name}"));
        let mut test_debugger = OmniDebugger::new(self.logger.clone(), self.max_retroaction_depth);
        match test_runner(&mut test_debugger) {
            Ok(()) => {
                self.logger.log("TEST_ORACLE", format!("Verification test '{test_name}' passed (+1:PROCEED)"));
                self.logger.record_ai_observation(
                    "ORACLE_SUCCESS",
                    &format!("Test '{test_name}' passed"),
                    "Candidate state passed invariant verification without failpoints",
                    ["test_oracle", "verified", "proceed"].as_slice(),
                    Some(test_name),
                );
                telemetry::record_debugger_probe_frame(&self.logger, self.step_count, "ORACLE_PASS", QuadState::Proceed);
                Ok(())
            }
            Err(err) => {
                self.logger.failpoint(
                    FailpointType::RollbackTriggered,
                    "TEST_ORACLE",
                    format!("Verification test '{test_name}' failed: {err}"),
                );
                self.logger.record_ai_observation(
                    "ORACLE_FAILURE",
                    &format!("Test '{test_name}' rejected candidate"),
                    &err,
                    ["test_oracle", "rejected", "rollback"].as_slice(),
                    Some(test_name),
                );
                telemetry::record_debugger_probe_frame(&self.logger, self.step_count, "ORACLE_FAIL", QuadState::Rollback);
                Err(err)
            }
        }
    }

    /// Verifies candidate source code with a provided test oracle function.
    ///
    /// # Arguments
    /// * `candidate_source` - The restored source string under evaluation.
    /// * `test_oracle` - Test function verifying candidate execution invariants.
    pub fn verify_candidate_repair<F>(
        &mut self,
        candidate_source: &str,
        test_oracle: F,
    ) -> Result<String, String>
    where
        F: Fn(&str) -> Result<(), String>,
    {
        oracle::TestOracleSandbox::verify_candidate_repair(candidate_source, test_oracle, &self.logger)
    }

    /// Ingests a structured [`LogEvent`] containing a [`FixSuggestion`], applies the patch,
    /// and invokes the verification oracle before committing the restored code.
    pub fn execute_restoration_from_log<F>(
        &mut self,
        source: &str,
        log_event: &LogEvent,
        test_oracle: F,
    ) -> Result<String, String>
    where
        F: Fn(&str) -> Result<(), String>,
    {
        oracle::TestOracleSandbox::execute_restoration_from_log(source, log_event, test_oracle, &self.logger)
    }

    /// Triggers an intentional or intrusion-provoked dead-man switch cascade into watchdog cryosleep.
    ///
    /// Preserves the flight-recorder state, emits high-priority cognitive telemetry,
    /// zeroes ephemeral state counters, and parks the debugger in `0:AWAIT` sentinel homeostasis.
    pub fn trigger_deadman_cascade(&mut self, reason: &str) -> QuadState {
        self.step_count += 1;
        let step = self.step_count;
        let msg = format!("Dead-Man Switch Triggered [Step #{step}]: Entering Watchdog Cryosleep ({reason})");

        self.logger.failpoint(
            FailpointType::DeadManTrigger,
            "DEADMAN",
            &msg,
        );

        telemetry::record_debugger_cognitive_note(
            &self.logger,
            "[Debugger] Dead-Man Cryosleep Engaged",
            &msg,
            &["debugger", "deadman", "cryosleep", "sentinel", "failpoint"],
            Some("DEADMAN_CASCADE"),
        );

        telemetry::record_debugger_probe_frame(&self.logger, step, "DEADMAN_CASCADE", QuadState::Await);

        // Zero ephemeral boundary stacks & retroaction counters to prevent data leakage
        self.boundary_stack.clear();
        self.retroaction_counters.clear();

        QuadState::Await
    }

    /// Evaluates whether the environment has certified all four ground truths to resurrect from cryosleep:
    ///
    /// 1. **Mathematical Rectitude:** `alu_proof == 0` (Balanced ternary `(+1) + (-1) = 0`).
    /// 2. **Topological Purity:** `unauthorized_hooks == false` (RAM decontaminated, no rogue DMA/MMIO).
    /// 3. **Scriptural Dual-Witness Parity:** `dual_witness_parity == true` (KJV & WEB checksum intact).
    /// 4. **Biological Quiescence:** `alien_noise_detected == false` (ambient input velocity within normal human bounds).
    ///
    /// # The Strict Certification Barrier (Failing Test Invariant)
    /// If ANY of the four ground truths is uncertified, resurrection is strictly VETOED, keeping
    /// the machine safely asleep in cryosleep. When all four are proven, returns `Ok(QuadState::Proceed)`.
    pub fn certify_deadman_resurrection(
        &mut self,
        alu_sum: i32,
        unauthorized_hooks: bool,
        dual_witness_parity: bool,
        alien_noise_detected: bool,
    ) -> Result<QuadState, FailpointType> {
        self.step_count += 1;
        let step = self.step_count;

        // Condition 1: ALU Sanity
        if alu_sum != 0 {
            let msg = format!("Resurrection Veto [Step #{step}]: Balanced ternary ALU imbalance ({alu_sum} != 0)");
            self.logger.failpoint(FailpointType::AluEquilibriumBreach, "RESURRECTION", &msg);
            telemetry::record_debugger_cognitive_note(
                &self.logger,
                "[Resurrection Veto] ALU Drift",
                &msg,
                &["debugger", "resurrection", "veto", "alu"],
                Some("ALU_DRIFT"),
            );
            return Err(FailpointType::AluEquilibriumBreach);
        }

        // Condition 2: Memory Hygiene
        if unauthorized_hooks {
            let msg = format!("Resurrection Veto [Step #{step}]: Topological memory contamination / rogue hooks detected");
            self.logger.failpoint(FailpointType::MemoryLandmarkBreach, "RESURRECTION", &msg);
            telemetry::record_debugger_cognitive_note(
                &self.logger,
                "[Resurrection Veto] Memory Contamination",
                &msg,
                &["debugger", "resurrection", "veto", "memory"],
                Some("MEMORY_POLLUTION"),
            );
            return Err(FailpointType::MemoryLandmarkBreach);
        }

        // Condition 3: Scriptural Dual-Witness Parity
        if !dual_witness_parity {
            let msg = format!("Resurrection Veto [Step #{step}]: Scriptural dual-witness parity (KJV/WEB) broken");
            self.logger.failpoint(FailpointType::ImmutableMutation, "RESURRECTION", &msg);
            telemetry::record_debugger_cognitive_note(
                &self.logger,
                "[Resurrection Veto] Invariant Checksum Broken",
                &msg,
                &["debugger", "resurrection", "veto", "scripture"],
                Some("PARITY_BROKEN"),
            );
            return Err(FailpointType::ImmutableMutation);
        }

        // Condition 4: Biological Quiescence
        if alien_noise_detected {
            let msg = format!("Resurrection Veto [Step #{step}]: Environmental entropy / high-velocity alien noise still active");
            self.logger.failpoint(FailpointType::EquilibriumDriftVeto, "RESURRECTION", &msg);
            telemetry::record_debugger_cognitive_note(
                &self.logger,
                "[Resurrection Veto] Alien Noise Active",
                &msg,
                &["debugger", "resurrection", "veto", "noise"],
                Some("ALIEN_NOISE"),
            );
            return Err(FailpointType::EquilibriumDriftVeto);
        }

        // All 4 Ground Truths Certified! The Machine Wakes Up!
        let msg = format!("Dead-Man Resurrection Certified [Step #{step}]: All 4 ground truths verified (+1:PROCEED)");
        self.logger.log("RESURRECTION", &msg);
        self.logger.record_ai_observation(
            "RESURRECTION_CERTIFIED",
            &msg,
            "Machine resurrected from cryosleep: ALU balanced, memory pure, dual-witness intact, noise quiescent",
            ["resurrection", "certified", "proceed", "cryosleep_exit"].as_slice(),
            Some("DEADMAN_RESURRECTION"),
        );
        telemetry::record_debugger_probe_frame(&self.logger, step, "RESURRECTION_PASS", QuadState::Proceed);

        Ok(QuadState::Proceed)
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( OmniDebugger Subsystem Verification Tests & Affirmative Invariant Sealing )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debugger_boundary_and_immutability_lifecycle() {
        let logger = DiagnosticLogger::new(false);
        let mut debugger = OmniDebugger::new(logger, 10);

        // Boundary Push & Pop
        debugger.push_boundary('<', 1, 1);
        assert_eq!(debugger.boundary_stack.len(), 1);
        let btype = debugger.pop_boundary('>', 1, 15).unwrap();
        assert_eq!(btype, BoundaryType::Invariant);
        assert!(debugger.assert_all_boundaries_closed().is_ok());

        // Immutability Seal & Veto
        debugger.register_immutable("Covenant");
        assert!(debugger.immutable_registry.contains("Covenant"));
        assert!(debugger.assert_mutation_permitted("Covenant").is_err());
        assert!(debugger.assert_mutation_permitted("TempVar").is_ok());

        // Staged remediations
        let remediations = debugger.drain_remediations();
        assert_eq!(remediations.len(), 1);
        assert_eq!(remediations[0].kind, FixKind::ConvertImmutableToMutable);
    }

    #[test]
    fn test_debugger_retroaction_depth_tripwire() {
        let logger = DiagnosticLogger::new(false);
        let mut debugger = OmniDebugger::new(logger, 2);

        assert_eq!(debugger.track_retroaction("Snapshot").unwrap(), 1);
        assert_eq!(debugger.track_retroaction("Snapshot").unwrap(), 2);
        assert!(debugger.track_retroaction("Snapshot").is_err());
    }

    #[test]
    fn test_debugger_verification_oracle_execution() {
        let logger = DiagnosticLogger::new(false);
        let mut debugger = OmniDebugger::new(logger, 5);

        // Passing oracle test
        let pass_res = debugger.execute_verification_test("verify_truth", |_child| Ok(()));
        assert!(pass_res.is_ok());

        // Failing oracle test
        let fail_res = debugger.execute_verification_test("veto_falsehood", |_child| {
            Err("Moral axiom breached".to_string())
        });
        assert!(fail_res.is_err());
    }

    #[test]
    fn test_debugger_deadman_cascade_and_resurrection_certification() {
        let logger = DiagnosticLogger::new(false);
        let mut debugger = OmniDebugger::new(logger.clone(), 5);

        // 1. Trigger Dead-Man Switch Cascade
        let cryosleep_state = debugger.trigger_deadman_cascade("Simulated Physical Threat Collision");
        assert_eq!(cryosleep_state, QuadState::Await, "Dead-man cascade must park machine in 0:AWAIT cryosleep");

        // Verify failpoint registered in logger
        assert!(logger.has_failpoints());
        let fps = logger.failpoints();
        assert_eq!(fps.len(), 1);
        assert_eq!(fps[0].failpoint_type, Some(FailpointType::DeadManTrigger));

        // 2. Strict Negative Vetoes (Failing test invariants when environment is not certain)
        // Veto A: ALU Imbalance (+1) + (-1) != 0
        let veto_alu = debugger.certify_deadman_resurrection(1, false, true, false);
        assert_eq!(veto_alu, Err(FailpointType::AluEquilibriumBreach));

        // Veto B: Memory contamination / illegal hooks
        let veto_mem = debugger.certify_deadman_resurrection(0, true, true, false);
        assert_eq!(veto_mem, Err(FailpointType::MemoryLandmarkBreach));

        // Veto C: Invariant scriptural dual-witness parity broken
        let veto_scripture = debugger.certify_deadman_resurrection(0, false, false, false);
        assert_eq!(veto_scripture, Err(FailpointType::ImmutableMutation));

        // Veto D: Environmental entropy / high-velocity alien noise active
        let veto_noise = debugger.certify_deadman_resurrection(0, false, true, true);
        assert_eq!(veto_noise, Err(FailpointType::EquilibriumDriftVeto));

        // 3. Positive Resurrection (All 4 Ground Truths Certified!)
        let resurrect_res = debugger.certify_deadman_resurrection(0, false, true, false);
        assert_eq!(resurrect_res, Ok(QuadState::Proceed), "Resurrection must commit +1:PROCEED when all truths certified");

        // Verify cognitive notes reflect resurrection
        let notes = logger.cognitive_notes();
        assert!(notes.iter().any(|n| n.category == "RESURRECTION_CERTIFIED"));
    }
}

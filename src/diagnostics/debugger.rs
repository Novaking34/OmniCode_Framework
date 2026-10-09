// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( OmniDebugger Identity, Invariant Verification Engine & Safety Constraints )
// ==============================================================================
//! OmniCode Diagnostics: Debugger & Invariant Inspector
//!
//! Tracks boundary stack, detects boundary leaks/mismatches, monitors immutability,
//! checks caveat fulfillment, and detects cyclic retroaction.

use std::collections::{HashMap, HashSet};
use crate::core::boundaries::BoundaryType;
use crate::core::ternary::QuadState;
use crate::diagnostics::logger::{DiagnosticLogger, FailpointType, FixKind, FixSuggestion, LogEvent};


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( BoundaryFrame Definition and OmniDebugger Context Setup )
// ==============================================================================
#[derive(Debug, Clone)]
pub struct BoundaryFrame {
    pub open_char: char,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone)]
pub struct OmniDebugger {
    pub logger: DiagnosticLogger,
    pub boundary_stack: Vec<BoundaryFrame>,
    pub immutable_registry: HashSet<String>,
    pub retroaction_counters: HashMap<String, usize>,
    pub remediation_queue: Vec<FixSuggestion>,
    pub max_retroaction_depth: usize,
    pub step_count: usize,
}

impl OmniDebugger {
    pub fn new(logger: DiagnosticLogger, max_retroaction_depth: usize) -> Self {
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
// ( Core Invariant Enforcement: Boundary Push/Pop, Immutability, Retroaction )
// ==============================================================================
impl OmniDebugger {
    /// Pushes an opening boundary delimiter onto the stack.
    pub fn push_boundary(&mut self, open: char, line: usize, col: usize) {
        self.logger.log(
            "BOUNDARY",
            format!("Pushing delimiter '{open}' at L{line}:C{col} (depth: {})", self.boundary_stack.len() + 1),
        );
        self.boundary_stack.push(BoundaryFrame { open_char: open, line, col });
    }

    /// Pops a closing boundary delimiter and validates the pair against the stack.
    ///
    /// # Failpoints
    /// - Stack empty: Unmatched closing bracket.
    /// - Illegal pair: Boundary mismatch with suggested repair patch.
    pub fn pop_boundary(&mut self, close: char, line: usize, col: usize) -> Result<BoundaryType, String> {
        if self.boundary_stack.is_empty() {
            let msg = format!("Unmatched closing delimiter '{close}' at L{line}:C{col} (stack is empty)");
            self.logger.failpoint(FailpointType::BoundaryMismatch, "DEBUGGER", &msg);
            return Err(format!("FAILPOINT: {msg}"));
        }

        let frame = self.boundary_stack.pop().unwrap();
        match BoundaryType::from_delimiters(frame.open_char, close) {
            Ok(btype) => {
                self.logger.log(
                    "BOUNDARY",
                    format!(
                        "Successfully resolved boundary {} '{}{}' (remaining depth: {})",
                        btype, frame.open_char, close, self.boundary_stack.len()
                    ),
                );
                Ok(btype)
            }
            Err(err) => {
                let msg = format!(
                    "Boundary mismatch: opened with '{}' (L{}:C{}) but closed with '{}' at L{}:C{}",
                    frame.open_char, frame.line, frame.col, close, line, col
                );
                let matching = match frame.open_char {
                    '<' => '>',
                    '[' => ']',
                    '{' => '}',
                    _ => ')',
                };
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
                self.remediation_queue.push(suggestion);
                Err(err)
            }
        }
    }

    /// Asserts that no boundaries remain unclosed at EOF, providing a remediation hint.
    pub fn assert_all_boundaries_closed(&mut self) -> Result<(), String> {
        if let Some(frame) = self.boundary_stack.last() {
            let msg = format!(
                "Boundary leak: delimiter '{}' opened at L{}:C{} was never closed",
                frame.open_char, frame.line, frame.col
            );
            let matching = match frame.open_char {
                '<' => '>',
                '[' => ']',
                '{' => '}',
                _ => ')',
            };
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
            self.remediation_queue.push(suggestion);
            return Err(format!("FAILPOINT: {msg}"));
        }
        Ok(())
    }

    /// Registers an entity identifier defined inside `<...>` as globally immutable.
    pub fn register_immutable(&mut self, name: &str) {
        self.immutable_registry.insert(name.to_string());
        self.logger.log("IMMUTABILITY", format!("Registered invariant <{name}>"));
    }

    /// Verifies that mutation is permitted for the given entity.
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
            self.remediation_queue.push(suggestion);
            return Err(format!("FAILPOINT: {msg}"));
        }
        Ok(())
    }

    /// Retrieves and empties the active remediation queue.
    pub fn drain_remediations(&mut self) -> Vec<FixSuggestion> {
        std::mem::take(&mut self.remediation_queue)
    }

    /// Monitors retroaction steps to prevent infinite rollback loops.
    pub fn track_retroaction(&mut self, target: &str) -> Result<usize, String> {
        let count = self.retroaction_counters.entry(target.to_string()).or_insert(0);
        *count += 1;
        let cur = *count;

        self.logger.log(
            "VECTOR",
            format!("Retroaction step #{cur} on '{target}'"),
        );

        if cur > self.max_retroaction_depth {
            let msg = format!(
                "Cyclic retroaction loop detected! Target '{target}' rolled back {cur} times, \
                 exceeding threshold ({})",
                self.max_retroaction_depth
            );
            self.logger.failpoint(FailpointType::CyclicRetroaction, "DEBUGGER", &msg);
            return Err(format!("FAILPOINT: {msg}"));
        }

        Ok(cur)
    }

    /// Inspects QuadState and raises appropriate signals.
    pub fn inspect_state(&mut self, state: QuadState, context: &str) {
        self.step_count += 1;
        match state {
            QuadState::Halt => {
                self.logger.failpoint(
                    FailpointType::NullTrap,
                    "DEBUGGER",
                    format!("NULL / Void halt trap encountered in context: {context}"),
                );
            }
            QuadState::Rollback => {
                self.logger.failpoint(
                    FailpointType::RollbackTriggered,
                    "DEBUGGER",
                    format!("Rollback (-1) triggered in context: {context}"),
                );
            }
            QuadState::Await => {
                self.logger.log("TERNARY", format!("Await (0) standby in context: {context}"));
            }
            QuadState::Proceed => {
                self.logger.log("TERNARY", format!("Proceed (+1) committed in context: {context}"));
            }
        }
    }

    /// Executes a verification test against an isolated child debugger context.
    /// Acts as an execution oracle for hypothesis verification before state commit.
    pub fn execute_verification_test<F>(&mut self, test_name: &str, test_runner: F) -> Result<(), String>
    where
        F: FnOnce(&mut OmniDebugger) -> Result<(), String>,
    {
        self.logger.log("TEST_ORACLE", format!("Spawning verification test: {test_name}"));
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
                Err(err)
            }
        }
    }

    /// Verifies candidate source code with a provided test oracle function.
    /// If the oracle passes, returns Ok(candidate).
    /// If the oracle fails, issues a rollback (-1:ROLLBACK) and returns Err.
    pub fn verify_candidate_repair<F>(
        &mut self,
        candidate_source: &str,
        test_oracle: F,
    ) -> Result<String, String>
    where
        F: Fn(&str) -> Result<(), String>,
    {
        self.logger.log("SELF_HEAL", "Verifying candidate repair via test oracle...");
        match test_oracle(candidate_source) {
            Ok(()) => {
                self.logger.log("SELF_HEAL", "Candidate repair verified (+1:PROCEED)!");
                self.inspect_state(QuadState::Proceed, "candidate_repair_verified");
                self.logger.record_ai_observation(
                    "REPAIR_VERIFIED",
                    "Candidate source passed verification oracle",
                    "The proposed restorative patch executed cleanly without failpoints",
                    ["self_healing", "oracle_passed", "verified"].as_slice(),
                    None,
                );
                Ok(candidate_source.to_string())
            }
            Err(e) => {
                self.inspect_state(QuadState::Rollback, "candidate_repair_rejected");
                let msg = format!("Candidate repair failed test verification: {e}");
                self.logger.failpoint(FailpointType::RollbackTriggered, "SELF_HEAL", &msg);
                self.logger.record_ai_observation(
                    "REPAIR_REJECTED",
                    "Candidate source failed verification oracle",
                    &msg,
                    ["self_healing", "oracle_failed", "rollback"].as_slice(),
                    None,
                );
                Err(msg)
            }
        }
    }

    /// Ingests a structured LogEvent containing a FixSuggestion, applies the candidate patch,
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
        let suggestion = log_event.suggestion.as_ref().ok_or_else(|| {
            "LogEvent does not contain remediation suggestion for automated restoration".to_string()
        })?;

        self.logger.log(
            "SELF_HEAL",
            format!("Applying remediation suggestion from log: {:?}", suggestion.kind),
        );

        let mut candidate = source.to_string();
        match suggestion.kind {
            FixKind::FixBoundaryMismatch => {
                let lines: Vec<&str> = candidate.lines().collect();
                if suggestion.line > 0 && suggestion.line <= lines.len() {
                    let mut target_line = lines[suggestion.line - 1].to_string();
                    if let Some(pos) = target_line.find(&suggestion.target_token) {
                        target_line.replace_range(pos..pos + suggestion.target_token.len(), &suggestion.suggested_patch);
                        let mut new_lines = lines.clone();
                        new_lines[suggestion.line - 1] = &target_line;
                        candidate = new_lines.join("\n");
                        if !candidate.ends_with('\n') {
                            candidate.push('\n');
                        }
                    }
                }
            }
            FixKind::CloseBoundary => {
                candidate.push_str(&suggestion.suggested_patch);
                if !candidate.ends_with('\n') {
                    candidate.push('\n');
                }
            }
            FixKind::ConvertImmutableToMutable => {
                candidate = candidate.replace(
                    &format!("< {} >", suggestion.target_token),
                    &suggestion.suggested_patch,
                );
            }
            _ => {
                candidate = candidate.replace(&suggestion.target_token, &suggestion.suggested_patch);
            }
        }

        // Call the test oracle to certify candidate
        self.verify_candidate_repair(&candidate, test_oracle)
    }
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Postcondition Verification and Debugger Status )
// ==============================================================================
// OmniDebugger safety invariants verified and closed.

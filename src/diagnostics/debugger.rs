//! OmniCode Diagnostics: Debugger & Invariant Inspector
//!
//! Tracks boundary stack, detects boundary leaks/mismatches, monitors immutability,
//! checks caveat fulfillment, and detects cyclic retroaction.

use std::collections::{HashMap, HashSet};
use crate::core::boundaries::BoundaryType;
use crate::core::ternary::QuadState;
use crate::diagnostics::logger::{DiagnosticLogger, FailpointType};

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
            max_retroaction_depth,
            step_count: 0,
        }
    }

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
    /// - Illegal pair: Boundary mismatch.
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
                self.logger.failpoint(FailpointType::BoundaryMismatch, "DEBUGGER", &msg);
                Err(err)
            }
        }
    }

    /// Asserts that no boundaries remain unclosed at EOF.
    pub fn assert_all_boundaries_closed(&mut self) -> Result<(), String> {
        if let Some(frame) = self.boundary_stack.last() {
            let msg = format!(
                "Boundary leak: delimiter '{}' opened at L{}:C{} was never closed",
                frame.open_char, frame.line, frame.col
            );
            self.logger.failpoint(FailpointType::BoundaryLeak, "DEBUGGER", &msg);
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
            self.logger.failpoint(FailpointType::ImmutableMutation, "DEBUGGER", &msg);
            return Err(format!("FAILPOINT: {msg}"));
        }
        Ok(())
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
}

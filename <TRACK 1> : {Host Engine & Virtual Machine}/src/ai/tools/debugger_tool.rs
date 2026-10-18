// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( AI Debugger Tool: Focused Spatial Inspection & Test Oracle Sandboxes )
// ==============================================================================
// SCRIPTURAL WITNESS:
//   "Prove all things; hold fast that which is good."
//   — 1 Thessalonians 5:21 (KJV)
//
//   "His eyes behold, his eyelids try, the children of men."
//   — Psalm 11:4 (KJV)
//
// ARCHITECTURAL TOPOLOGY:
//   Tool 2 of the CPI-SI Autonomous Diagnostic Tool Belt.
//   Provides the AI Cortex with focused structural scrutiny:
//     - Audits boundary delimiter stacks (<>, [], {}, (), {], [}) on source text.
//     - Verifies Rank 1 Immutable Laws (< ... >) to prevent unlawful mutations.
//     - Executes candidate code inside isolated empirical test oracles.
//     - Evaluates balanced ternary truth states (+1, 0, -1, NULL) and traps.
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Imports, Verification Types & Tool Struct Definition )
// ==============================================================================
use crate::ast::nodes::ASTNode;
use crate::core::ternary::QuadState;
use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::DiagnosticLogger;
use crate::parser::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::runtime::environment::{Environment, RuntimeValue};
use crate::runtime::evaluator::Evaluator;

/// Diagnostic tool giving the AI model direct access to spatial debugging and verification oracles.
#[derive(Debug, Clone)]
pub struct DebuggerTool {
    pub debugger: OmniDebugger,
}

impl DebuggerTool {
    /// Creates a new [`DebuggerTool`] bound to the provided debugger instance.
    pub fn new(debugger: OmniDebugger) -> Self {
        Self { debugger }
    }
}

/// Outcome report of a debugger structural audit.
#[derive(Debug, Clone)]
pub struct DebuggerAuditReport {
    pub boundaries_balanced: bool,
    pub unclosed_frames_count: usize,
    pub pending_remediations_count: usize,
    pub parse_succeeded: bool,
    pub error: Option<String>,
}

/// Comprehensive report returned by DebuggerTool::evaluate_source.
#[derive(Debug, Clone)]
pub struct DebuggerToolReport {
    pub audit: DebuggerAuditReport,
    pub sandbox_result: Option<Result<QuadState, String>>,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Structural Boundary Auditing, Immutability Guards & Oracle Sandboxing )
// ==============================================================================
impl DebuggerTool {
    /// Evaluates source code, checking boundaries and running sandbox evaluation.
    pub fn evaluate_source(&self, code: &str, _step_by_step: bool) -> DebuggerToolReport {
        let audit = self.audit_source_boundaries(code);
        let sandbox_result = if audit.boundaries_balanced && audit.parse_succeeded {
            Some(self.test_in_sandbox(code))
        } else {
            None
        };
        DebuggerToolReport {
            audit,
            sandbox_result,
        }
    }

    /// Audits a source string to verify whether its morphological boundaries are sound.
    pub fn audit_source_boundaries(&self, source: &str) -> DebuggerAuditReport {
        let temp_logger = DiagnosticLogger::new(false);
        let mut lexer = Lexer::new(source, temp_logger.clone());
        let tokens = match lexer.tokenize() {
            Ok(t) => t,
            Err(e) => {
                return DebuggerAuditReport {
                    boundaries_balanced: false,
                    unclosed_frames_count: 0,
                    pending_remediations_count: 0,
                    parse_succeeded: false,
                    error: Some(format!("Lexer error: {e}")),
                };
            }
        };

        let temp_debugger = OmniDebugger::new(temp_logger.clone(), 10);
        let mut parser = Parser::new(tokens, temp_debugger);
        let parse_res = parser.parse_program();

        let unclosed = parser.debugger.boundary_stack.len();
        let remediations = parser.debugger.remediation_queue.len();
        let parse_ok = parse_res.is_ok();

        DebuggerAuditReport {
            boundaries_balanced: unclosed == 0 && remediations == 0,
            unclosed_frames_count: unclosed,
            pending_remediations_count: remediations,
            parse_succeeded: parse_ok,
            error: parse_res.err(),
        }
    }

    /// Evaluates candidate OmniCode code inside a sandboxed test oracle.
    ///
    /// Runs Lint -> Lex -> Parse -> VM Eval in complete isolation.
    /// Returns the resulting [`QuadState`] if execution is successful.
    pub fn test_in_sandbox(&self, candidate: &str) -> Result<QuadState, String> {
        let temp_logger = DiagnosticLogger::new(false);
        let mut lexer = Lexer::new(candidate, temp_logger.clone());
        let tokens = lexer.tokenize().map_err(|e| format!("Lexer sandbox error: {e}"))?;

        let temp_debugger = OmniDebugger::new(temp_logger.clone(), 10);
        let mut parser = Parser::new(tokens, temp_debugger);
        let prog = parser.parse_program().map_err(|e| format!("Parser sandbox error: {e}"))?;

        let mut evaluator = Evaluator::new(Environment::new(), parser.debugger);
        let val = evaluator
            .eval(&ASTNode::Program(prog))
            .map_err(|e| format!("VM sandbox evaluation error: {e}"))?;

        match val {
            RuntimeValue::State(s) => Ok(s),
            _ => Ok(QuadState::Proceed),
        }
    }

    /// Checks whether an entity name is registered as an immutable Rank 1 invariant.
    pub fn is_immutable(&self, name: &str) -> bool {
        self.debugger.immutable_registry.contains(name)
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Debugger Tool Default Sealing & Invariant Verification )
// ==============================================================================
impl Default for DebuggerTool {
    fn default() -> Self {
        Self::new(OmniDebugger::new(DiagnosticLogger::new(false), 10))
    }
}

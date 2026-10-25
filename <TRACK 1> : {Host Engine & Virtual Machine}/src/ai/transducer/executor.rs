// =============================================================================
// BLOCK 1: METADATA BLOCK
// ( OmniCode Turing Reel Execution, Tool Dispatch & Dynamic VM Engine )
// =============================================================================
//
// @module      :: omnicode::ai::transducer::executor
// @file        :: src/ai/transducer/executor.rs
// @layer       :: Track 1 / AI Subsystem / Transducer / Execution Engine
// @version     :: 2.0.0
// @epoch       :: Radix-3 Balanced Ternary Genesis
// @author      :: OmniCode Systems Architecture Group
// @system      :: CPI-SI Autonomic AI & VM Pipeline
// @repository  :: OmniCode_Framework
// @role        :: Orchestrates Turing reel execution on the VM, performs
//                 preflight diagnostic linting, autonomic self-healing via the
//                 Restorator, and dispatches terminal-like tool commands.
//
// @invariants  ::
//   1. Radix-3 Balanced Ternary Truth States: Every reel evaluates into
//      QuadState in {+1: PROCEED, -1: ROLLBACK, 0: AWAIT, NULL: HALT}.
//   2. Autonomic Preflight: Malformed synthesized reels undergo automated
//      boundary audit and restorative healing before VM evaluation.
//   3. Closed-Loop Tool Execution: Tool calls mirror Linux terminal commands,
//      providing empirical ground-truth observations back to the AI.
//   4. Strict 80-Column Geometry: Every source line must remain <= 80 chars.
//
// @witness     ::
//   - Ecclesiastes 10:10 (KJV): "If the iron be blunt, and he do not whet the
//     edge, then must he put to more strength: but wisdom is profitable to
//     direct." (Sharpening tools for efficient, directed machine execution)
//   - Proverbs 24:3-4 (KJV): "Through wisdom is an house builded; and by
//     understanding it is established: And by knowledge shall the chambers
//     be filled with all precious and pleasant riches." (Building the runtime
//     house through understanding and well-ordered tool dispatch)
//   - 1 Corinthians 14:40 (KJV): "Let all things be done decently and in
//     order." (Immutable invariant of structural order and discipline)
//
// =============================================================================

use std::collections::HashMap;
use crate::ai::tools::{AIToolBelt, ToolCall, ToolResult};
use crate::core::ternary::QuadState;
use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::DiagnosticLogger;
use crate::diagnostics::restorator::AppliedRepair;
use crate::parser::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::runtime::environment::{Environment, RuntimeValue};
use crate::runtime::evaluator::Evaluator;

// =============================================================================
// BLOCK 2: OPENING BLOCK
// ( Execution Result Structure, Tool Interfaces & Configuration )
// =============================================================================

/// Comprehensive outcome returned from evaluating an OmniCode Turing reel.
#[derive(Debug, Clone)]
pub struct ReelExecutionOutput {
    /// Resulting QuadState from the reel.
    pub execution_state: QuadState,
    /// String-formatted map of runtime environment bindings.
    pub bindings_summary: HashMap<String, String>,
    /// Underlying runtime environment containing evaluated symbols.
    pub environment: Environment,
    /// Diagnostic repairs applied by the Restorator during preflight.
    pub repairs_applied: Vec<AppliedRepair>,
    /// Whether the reel passed boundary and structural verification.
    pub audit_passed: bool,
    /// Total number of telemetry frames recorded during execution.
    pub telemetry_frames: usize,
}

/// The unified Reel Executor equipped with the Five-Fold Diagnostic Tool Belt.
#[derive(Debug, Clone)]
pub struct ReelExecutor {
    /// Diagnostic logger for recording sensory telemetry.
    pub logger: DiagnosticLogger,
    /// The Five-Fold Tool Belt for preflight, healing, and tool dispatch.
    pub tools: AIToolBelt,
}

impl ReelExecutor {
    /// Constructs a new [`ReelExecutor`] with fresh diagnostic tools.
    pub fn new(logger: DiagnosticLogger) -> Self {
        let tools = AIToolBelt::with_logger(logger.clone());
        Self { logger, tools }
    }
}

impl Default for ReelExecutor {
    fn default() -> Self {
        Self::new(DiagnosticLogger::new(false))
    }
}

// =============================================================================
// BLOCK 3: BODY BLOCK
// ( VM Evaluation Pipeline, Autonomic Healing & Tool Dispatch )
// =============================================================================

impl ReelExecutor {
    /// Dispatches a terminal-like tool call to the diagnostic tool belt.
    ///
    /// Mirrors invoking a terminal command in Linux: queries empirical
    /// machine state (telemetry, symbols) or effects system changes
    /// (restoration, sandboxed evaluation).
    pub fn dispatch_tool(&mut self, call: ToolCall) -> ToolResult {
        self.logger.log(
            "EXECUTOR",
            format!("Dispatching tool call: {:?}", call),
        );
        self.tools.dispatch(call)
    }

    /// Evaluates a synthesized OmniCode reel through the complete VM pipeline.
    ///
    /// Steps:
    /// 1. Preflight boundary audit via [`DebuggerTool`].
    /// 2. Autonomic restoration via [`RestoratorTool`] if boundary errors
    ///    exist.
    /// 3. Tokenization via [`Lexer`].
    /// 4. Parsing via [`Parser`].
    /// 5. Execution on the balanced ternary [`Evaluator`].
    /// 6. Extraction of runtime bindings and telemetry summary.
    pub fn execute(
        &mut self,
        omnicode: &str,
    ) -> Result<ReelExecutionOutput, String> {
        let mut active_code = omnicode.to_string();
        let mut repairs_applied = Vec::new();

        // 1. Preflight boundary audit
        let audit = self.tools.debugger.audit_source_boundaries(&active_code);
        let mut audit_passed =
            audit.boundaries_balanced && audit.parse_succeeded;

        // 2. Autonomic immune healing if preflight failed
        if !audit_passed {
            self.logger.log(
                "EXECUTOR",
                "Preflight boundary check failed; engaging Restorator tool",
            );
            let restoration = self.tools.restorator.heal_source(&active_code);
            if restoration.was_modified {
                repairs_applied = restoration.repairs_applied.clone();
                active_code = restoration.restored_content;
                let re_audit =
                    self.tools.debugger.audit_source_boundaries(&active_code);
                audit_passed =
                    re_audit.boundaries_balanced && re_audit.parse_succeeded;
            }
        }

        // 3. Sensory tokenization
        let mut lexer = Lexer::new(&active_code, self.logger.clone());
        let tokens = lexer.tokenize()?;

        // 4. AST construction
        let debugger = OmniDebugger::new(self.logger.clone(), 10);
        let mut parser = Parser::new(tokens, debugger);
        let prog = parser.parse_program()?;

        // 5. Balanced ternary quad-state evaluation
        let mut evaluator =
            Evaluator::new(Environment::new(), parser.debugger);
        let res_val =
            evaluator.eval(&crate::ast::nodes::ASTNode::Program(prog))?;

        let execution_state = match res_val {
            RuntimeValue::State(s) => s,
            _ => QuadState::Proceed,
        };

        // 6. Bindings summary extraction
        let mut bindings_summary = HashMap::new();
        for (k, v) in evaluator.env.bindings() {
            bindings_summary.insert(k.clone(), format!("{:?}", v));
        }

        let telemetry_frames = self.logger.events().len();

        Ok(ReelExecutionOutput {
            execution_state,
            bindings_summary,
            environment: evaluator.env,
            repairs_applied,
            audit_passed,
            telemetry_frames,
        })
    }
}

/// Convenience entry point to execute an OmniCode reel with a dedicated
/// executor.
pub fn execute_reel(
    omnicode: &str,
    logger: DiagnosticLogger,
) -> Result<ReelExecutionOutput, String> {
    let mut executor = ReelExecutor::new(logger);
    executor.execute(omnicode)
}

// =============================================================================
// BLOCK 4: AUDIT PROCEDURES BLOCK
// ( Self-Diagnostic Verification for VM Reel & Tool Execution )
// =============================================================================

/// Executes automated self-diagnostic audit for the executor subsystem.
pub fn audit_executor_subsystem() -> Result<(), String> {
    let test_reel = r#"
Module < Test.Reel > () {
    version: < "1.0.0" >
}
Opening {
    preconditions: [ +1 ]
}
vessel [ 10 ]
Body [
    roll_1 [ vessel = 10 ]
]
Closing {
    verify: [ roll_1 = PROCEED ],
    commit_status [ PROCEED ]
}
"#;

    let logger = DiagnosticLogger::new(false);
    let mut executor = ReelExecutor::new(logger);

    // 1. Audit reel execution
    let output = executor.execute(test_reel)?;

    if output.execution_state != QuadState::Proceed {
        return Err(format!(
            "Audit failed: Reel state was {:?}, expected Proceed",
            output.execution_state
        ));
    }

    if !output.bindings_summary.contains_key("vessel") {
        return Err("Audit failed: Environment missing 'vessel' binding".into());
    }

    // 2. Audit tool call dispatch (akin to terminal inspection)
    let tool_res = executor.dispatch_tool(ToolCall::InspectTelemetry {
        max_frames: 5,
    });

    match tool_res {
        ToolResult::Telemetry(report) => {
            if report.total_logs == 0 {
                return Err(
                    "Audit failed: Telemetry tool returned 0 logs".into()
                );
            }
        }
        _ => return Err("Audit failed: Unexpected tool call result".into()),
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
    fn test_executor_subsystem_audit() {
        assert!(audit_executor_subsystem().is_ok());
    }

    #[test]
    fn test_executor_simple_addition_reel() {
        let reel = r#"
Module < Test.Addition > () {
    version: < "1.0.0" >
}
Opening {
    preconditions: [ +1 ]
}
x [ 5 ]
y [ 7 ]
Body [
    roll_add [ x --> x plus y ]
]
Closing {
    verify: [ roll_add = 12 ],
    commit_status [ PROCEED ]
}
"#;
        let logger = DiagnosticLogger::new(false);
        let output =
            execute_reel(reel, logger).expect("Reel should execute");

        assert_eq!(output.execution_state, QuadState::Proceed);
        assert_eq!(
            output.environment.get("x"),
            Some(&RuntimeValue::Number(12.0))
        );
        assert!(output.bindings_summary.contains_key("x"));
        assert!(output.audit_passed);
    }

    #[test]
    fn test_executor_tool_dispatch_like_terminal() {
        let logger = DiagnosticLogger::new(false);
        let mut executor = ReelExecutor::new(logger);

        // Terminal-style lint command
        let lint_call = ToolCall::LintSource {
            source: "let x = 1".to_string(),
        };
        let lint_result = executor.dispatch_tool(lint_call);

        match lint_result {
            ToolResult::Lint(rep) => {
                assert!(!rep.is_valid);
                assert!(rep.error_count > 0);
            }
            _ => panic!("Expected Lint report"),
        }

        // Terminal-style telemetry query command
        let telem_call = ToolCall::InspectTelemetry { max_frames: 3 };
        let telem_result = executor.dispatch_tool(telem_call);

        match telem_result {
            ToolResult::Telemetry(rep) => {
                assert!(rep.total_logs > 0);
            }
            _ => panic!("Expected Telemetry report"),
        }
    }

    #[test]
    fn test_executor_autonomic_healing_on_malformed_input() {
        // Reel with unclosed brackets and missing closing block
        let malformed = "Module < Broken > () {\n vessel [ 10\n";
        let logger = DiagnosticLogger::new(false);
        let mut executor = ReelExecutor::new(logger);

        // The executor preflight encounters boundary failure and engages
        // the Restorator tool.
        let _ = executor.execute(malformed);
        // The logger recorded the preflight restoration attempt
        assert!(executor.logger.events().iter().any(|e| {
            e.message.contains("Restorator") || e.message.contains("Preflight")
        }));
    }
}

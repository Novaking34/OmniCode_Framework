// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( OmniCode Turing Reel Execution & Dynamic Environment Extraction )
// ============================================================================
//! # Reel Executor (`executor.rs`)
//!
//! Lexes, parses, and evaluates synthesized 4-Block OmniCode reels, returning
//! final execution QuadState, environment bindings, and runtime telemetry.

use std::collections::HashMap;
use crate::core::ternary::QuadState;
use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::DiagnosticLogger;
use crate::parser::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::runtime::environment::{Environment, RuntimeValue};
use crate::runtime::evaluator::Evaluator;

/// Executes a synthesized OmniCode reel through the VM pipeline.
pub fn execute_reel(
    omnicode: &str,
    logger: DiagnosticLogger,
) -> Result<(QuadState, HashMap<String, String>, Environment), String> {
    let mut lexer = Lexer::new(omnicode, logger.clone());
    let tokens = lexer.tokenize()?;

    let debugger = OmniDebugger::new(logger, 10);
    let mut parser = Parser::new(tokens, debugger);
    let prog = parser.parse_program()?;

    let mut evaluator = Evaluator::new(Environment::new(), parser.debugger);
    let res_val = evaluator.eval(&crate::ast::nodes::ASTNode::Program(prog))?;

    let execution_state = match res_val {
        RuntimeValue::State(s) => s,
        _ => QuadState::Proceed,
    };

    let mut bindings_summary = HashMap::new();
    for (k, v) in evaluator.env.bindings() {
        bindings_summary.insert(k.clone(), format!("{:?}", v));
    }

    Ok((execution_state, bindings_summary, evaluator.env))
}

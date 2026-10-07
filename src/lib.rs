// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Library Root Identity, Universal Subsystem Modules & Core Re-exports )
// ==============================================================================
//! OmniCode Language Engine
//! High-performance, AI-native language engine based on formal ontology,
//! set theory, and balanced ternary quad-state execution.

pub mod core;
pub mod diagnostics;
pub mod ast;
pub mod parser;
pub mod runtime;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Public Re-export Declarations & High-Level API Surface )
// ==============================================================================
pub use core::ternary::{QuadState, ROLLBACK, AWAIT, PROCEED, HALT};
pub use core::boundaries::BoundaryType;
pub use core::operators::{BeingRelation, DirectionalVector, Connective};
pub use diagnostics::logger::{DiagnosticLogger, FailpointType};
pub use diagnostics::debugger::OmniDebugger;
pub use diagnostics::linter::{OmniLinter, LintReport, BlockKind};
pub use parser::lexer::Lexer;
pub use parser::parser::Parser;
pub use runtime::environment::Environment;
pub use runtime::evaluator::Evaluator;


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Core Convenience Execution Pipeline )
// ==============================================================================
/// Convenience execution helper that tokenizes, parses, and evaluates an OmniCode string.
pub fn execute(source: &str, print_logs: bool) -> Result<runtime::RuntimeValue, String> {
    let logger = DiagnosticLogger::new(print_logs);
    let mut lexer = Lexer::new(source, logger.clone());
    let tokens = lexer.tokenize()?;

    let debugger = OmniDebugger::new(logger, 10);
    let mut parser = Parser::new(tokens, debugger);
    let program = parser.parse_program()?;

    let mut evaluator = Evaluator::new(Environment::new(), parser.debugger);
    evaluator.eval(&ast::nodes::ASTNode::Program(program))
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Engine Status & Library Invariants )
// ==============================================================================
// OmniCode library surface verified and sealed.

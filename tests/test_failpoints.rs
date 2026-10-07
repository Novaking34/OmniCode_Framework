//! OmniCode Failpoint Test Suite
//!
//! Explicitly engineered to verify that failpoints are caught, logged,
//! and intercepted by the OmniDebugger and DiagnosticLogger.

use omnicode::core::boundaries::BoundaryType;
use omnicode::core::ternary::{QuadState, ROLLBACK, AWAIT, PROCEED, HALT};
use omnicode::diagnostics::logger::{DiagnosticLogger, FailpointType};
use omnicode::diagnostics::debugger::OmniDebugger;
use omnicode::parser::lexer::Lexer;
use omnicode::parser::parser::Parser;
use omnicode::runtime::environment::{Environment, RuntimeValue};
use omnicode::runtime::evaluator::Evaluator;

#[test]
fn test_failpoint_boundary_mismatch_illegal_crossing() {
    let source = "bad_boundary < 42 ]";
    let logger = DiagnosticLogger::new(false);
    let mut lexer = Lexer::new(source, logger.clone());
    let tokens = lexer.tokenize().expect("Tokenization should succeed");

    let debugger = OmniDebugger::new(logger.clone(), 5);
    let mut parser = Parser::new(tokens, debugger);

    let result = parser.parse_program();
    assert!(result.is_err(), "Parser must reject illegal boundary '< ... ]'");

    let failpoints = logger.failpoints();
    assert!(!failpoints.is_empty(), "Failpoint must be recorded in logger");
    assert_eq!(
        failpoints[0].failpoint_type,
        Some(FailpointType::BoundaryMismatch),
        "Expected FP_BOUNDARY_MISMATCH"
    );
}

#[test]
fn test_failpoint_boundary_leak_unclosed_delimiter() {
    let source = "unclosed_block [ 100 200";
    let logger = DiagnosticLogger::new(false);
    let mut lexer = Lexer::new(source, logger.clone());
    let tokens = lexer.tokenize().expect("Tokenization should succeed");

    let debugger = OmniDebugger::new(logger.clone(), 5);
    let mut parser = Parser::new(tokens, debugger);

    let result = parser.parse_program();
    assert!(result.is_err(), "Parser must reject unclosed boundary reaching EOF");

    let failpoints = logger.failpoints();
    assert!(
        failpoints.iter().any(|fp| fp.failpoint_type == Some(FailpointType::BoundaryLeak)),
        "Expected FP_BOUNDARY_LEAK"
    );
}

#[test]
fn test_failpoint_immutable_mutation_attempt() {
    let source = r#"
        CONSTANT < 100 >
        CONSTANT [ 200 ]
    "#;
    let logger = DiagnosticLogger::new(false);
    let mut lexer = Lexer::new(source, logger.clone());
    let tokens = lexer.tokenize().expect("Tokens");

    let debugger = OmniDebugger::new(logger.clone(), 5);
    let mut parser = Parser::new(tokens, debugger);
    let prog = parser.parse_program().expect("Parse");

    let mut evaluator = Evaluator::new(Environment::new(), parser.debugger);
    let result = evaluator.eval(&omnicode::ast::nodes::ASTNode::Program(prog));

    assert!(result.is_err(), "Must reject mutation of invariant <...>");
    let failpoints = logger.failpoints();
    assert!(
        failpoints.iter().any(|fp| fp.failpoint_type == Some(FailpointType::ImmutableMutation)),
        "Expected FP_IMMUTABLE_MUTATION"
    );
}

#[test]
fn test_failpoint_caveat_veto_and_rollback() {
    // Action is PROCEED (+1), BUT Caveat is ROLLBACK (-1)
    let source = "Transaction [ +1 ] ! [ -1 ]";
    let logger = DiagnosticLogger::new(false);
    let mut lexer = Lexer::new(source, logger.clone());
    let tokens = lexer.tokenize().expect("Tokens");

    let debugger = OmniDebugger::new(logger.clone(), 5);
    let mut parser = Parser::new(tokens, debugger);
    let prog = parser.parse_program().expect("Parse");

    let mut evaluator = Evaluator::new(Environment::new(), parser.debugger);
    let result = evaluator.eval(&omnicode::ast::nodes::ASTNode::Program(prog)).expect("Eval");

    // The caveat should veto and resolve to Rollback
    assert_eq!(result, RuntimeValue::State(QuadState::Rollback));

    let failpoints = logger.failpoints();
    assert!(
        failpoints.iter().any(|fp| fp.failpoint_type == Some(FailpointType::CaveatVeto)),
        "Expected FP_CAVEAT_VETO"
    );
}

#[test]
fn test_failpoint_null_trap_absorption() {
    let source = "risky_operation [ +1 ] ! [ NULL ]";
    let logger = DiagnosticLogger::new(false);
    let mut lexer = Lexer::new(source, logger.clone());
    let tokens = lexer.tokenize().expect("Tokens");

    let debugger = OmniDebugger::new(logger.clone(), 5);
    let mut parser = Parser::new(tokens, debugger);
    let prog = parser.parse_program().expect("Parse");

    let mut evaluator = Evaluator::new(Environment::new(), parser.debugger);
    let result = evaluator.eval(&omnicode::ast::nodes::ASTNode::Program(prog)).expect("Eval");

    assert_eq!(result, RuntimeValue::State(QuadState::Halt));

    let failpoints = logger.failpoints();
    assert!(
        failpoints.iter().any(|fp| fp.failpoint_type == Some(FailpointType::NullTrap)),
        "Expected FP_NULL_TRAP"
    );
}

#[test]
fn test_failpoint_cyclic_retroaction_threshold() {
    let logger = DiagnosticLogger::new(false);
    let mut debugger = OmniDebugger::new(logger.clone(), 3); // threshold of 3

    // Step 1, 2, 3 ok
    assert!(debugger.track_retroaction("loop").is_ok());
    assert!(debugger.track_retroaction("loop").is_ok());
    assert!(debugger.track_retroaction("loop").is_ok());

    // Step 4 must trigger CyclicRetroaction failpoint
    let over_threshold = debugger.track_retroaction("loop");
    assert!(over_threshold.is_err(), "Must trip cyclic retroaction detector");

    let failpoints = logger.failpoints();
    assert!(
        failpoints.iter().any(|fp| fp.failpoint_type == Some(FailpointType::CyclicRetroaction)),
        "Expected FP_CYCLIC_RETROACTION"
    );
}

#[test]
fn test_balanced_ternary_algebraic_symmetry() {
    // Negation symmetry (-- NOT)
    assert_eq!(PROCEED.invert(), ROLLBACK);
    assert_eq!(ROLLBACK.invert(), PROCEED);
    assert_eq!(AWAIT.invert(), AWAIT);
    assert_eq!(HALT.invert(), HALT);

    // Conjunction (:: AND)
    assert_eq!(PROCEED.and(AWAIT), AWAIT);
    assert_eq!(PROCEED.and(ROLLBACK), ROLLBACK);
    assert_eq!(AWAIT.and(ROLLBACK), ROLLBACK);
    assert_eq!(PROCEED.and(HALT), HALT);

    // Disjunction (| OR)
    assert_eq!(ROLLBACK.or(PROCEED), PROCEED);
    assert_eq!(ROLLBACK.or(AWAIT), AWAIT);
    assert_eq!(AWAIT.or(PROCEED), PROCEED);
    assert_eq!(AWAIT.or(HALT), HALT);
}

#[test]
fn test_asymmetric_causality_boundaries() {
    // Top-Down: { 42 ] (Context defines)
    assert_eq!(BoundaryType::from_delimiters('{', ']').unwrap(), BoundaryType::TopDown);
    // Bottom-Up: [ 42 } (Defined by context)
    assert_eq!(BoundaryType::from_delimiters('[', '}').unwrap(), BoundaryType::BottomUp);
    // Standard symmetric pairs
    assert_eq!(BoundaryType::from_delimiters('[', ']').unwrap(), BoundaryType::Denotation);
    assert_eq!(BoundaryType::from_delimiters('<', '>').unwrap(), BoundaryType::Invariant);
    assert_eq!(BoundaryType::from_delimiters('{', '}').unwrap(), BoundaryType::Connotation);
    assert_eq!(BoundaryType::from_delimiters('(', ')').unwrap(), BoundaryType::Aside);
}

#[test]
fn test_canonical_notation_file() {
    let source = std::fs::read_to_string("examples/canonical_notation.omni")
        .expect("Failed to read canonical_notation.omni");
    
    let result = omnicode::execute(&source, false);
    assert!(result.is_ok(), "Canonical notation file must parse and evaluate successfully: {:?}", result.err());
}

// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Test Suite Identity, Failpoint Test Manifest, and Invariant Imports )
// ==============================================================================
//! OmniCode Failpoint Test Suite
//!
//! Explicitly engineered to verify that failpoints are caught, logged,
//! and intercepted by the OmniDebugger, DiagnosticLogger, and OmniLinter.

use omnicode::core::boundaries::BoundaryType;
use omnicode::core::ternary::{QuadState, ROLLBACK, AWAIT, PROCEED, HALT};
use omnicode::diagnostics::logger::{DiagnosticLogger, FailpointType};
use omnicode::diagnostics::debugger::OmniDebugger;
use omnicode::diagnostics::linter::OmniLinter;
use omnicode::parser::lexer::Lexer;
use omnicode::parser::parser::Parser;
use omnicode::runtime::environment::{Environment, RuntimeValue};
use omnicode::runtime::evaluator::Evaluator;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Test Harness Preconditions & Setup Helpers )
// ==============================================================================
fn create_test_debugger(threshold: usize) -> (DiagnosticLogger, OmniDebugger) {
    let logger = DiagnosticLogger::new(false);
    let debugger = OmniDebugger::new(logger.clone(), threshold);
    (logger, debugger)
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Failpoint Test Execution Cases: Boundary, Immutability, Caveats, Loops, Linter )
// ==============================================================================
#[test]
fn test_failpoint_boundary_mismatch_illegal_crossing() {
    let source = "bad_boundary < 42 ]";
    let (logger, debugger) = create_test_debugger(5);
    let mut lexer = Lexer::new(source, logger.clone());
    let tokens = lexer.tokenize().expect("Tokenization should succeed");

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
    let (logger, debugger) = create_test_debugger(5);
    let mut lexer = Lexer::new(source, logger.clone());
    let tokens = lexer.tokenize().expect("Tokenization should succeed");

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
    let (logger, debugger) = create_test_debugger(5);
    let mut lexer = Lexer::new(source, logger.clone());
    let tokens = lexer.tokenize().expect("Tokens");

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
    let source = "Transaction [ +1 ] ! [ -1 ]";
    let (logger, debugger) = create_test_debugger(5);
    let mut lexer = Lexer::new(source, logger.clone());
    let tokens = lexer.tokenize().expect("Tokens");

    let mut parser = Parser::new(tokens, debugger);
    let prog = parser.parse_program().expect("Parse");

    let mut evaluator = Evaluator::new(Environment::new(), parser.debugger);
    let result = evaluator.eval(&omnicode::ast::nodes::ASTNode::Program(prog)).expect("Eval");

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
    let (logger, debugger) = create_test_debugger(5);
    let mut lexer = Lexer::new(source, logger.clone());
    let tokens = lexer.tokenize().expect("Tokens");

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
    let (logger, mut debugger) = create_test_debugger(3);

    assert!(debugger.track_retroaction("loop").is_ok());
    assert!(debugger.track_retroaction("loop").is_ok());
    assert!(debugger.track_retroaction("loop").is_ok());

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
    assert_eq!(PROCEED.invert(), ROLLBACK);
    assert_eq!(ROLLBACK.invert(), PROCEED);
    assert_eq!(AWAIT.invert(), AWAIT);
    assert_eq!(HALT.invert(), HALT);

    assert_eq!(PROCEED.and(AWAIT), AWAIT);
    assert_eq!(PROCEED.and(ROLLBACK), ROLLBACK);
    assert_eq!(AWAIT.and(ROLLBACK), ROLLBACK);
    assert_eq!(PROCEED.and(HALT), HALT);

    assert_eq!(ROLLBACK.or(PROCEED), PROCEED);
    assert_eq!(ROLLBACK.or(AWAIT), AWAIT);
    assert_eq!(AWAIT.or(PROCEED), PROCEED);
    assert_eq!(AWAIT.or(HALT), HALT);
}

#[test]
fn test_asymmetric_causality_boundaries() {
    assert_eq!(BoundaryType::from_delimiters('{', ']').unwrap(), BoundaryType::TopDown);
    assert_eq!(BoundaryType::from_delimiters('[', '}').unwrap(), BoundaryType::BottomUp);
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

#[test]
fn test_four_block_structural_linter_validation() {
    let logger = DiagnosticLogger::new(false);
    let linter = OmniLinter::new(logger);

    // Test compliant 4-block source
    let compliant_source = r#"
# BLOCK 1: METADATA BLOCK
Module < Test > () {}
# BLOCK 2: OPENING BLOCK
Opening {}
# BLOCK 3: BODY BLOCK
Body []
# BLOCK 4: CLOSING BLOCK
Closing {}
"#;
    let report = linter.lint_source("test_compliant", compliant_source);
    assert!(report.is_valid, "Compliant 4-block source must pass linter");
    assert_eq!(report.blocks_found.len(), 4);

    // Test non-compliant source (missing body block)
    let non_compliant_source = r#"
# BLOCK 1: METADATA BLOCK
Module < Broken > () {}
# BLOCK 2: OPENING BLOCK
Opening {}
# BLOCK 4: CLOSING BLOCK
Closing {}
"#;
    let fail_report = linter.lint_source("test_broken", non_compliant_source);
    assert!(!fail_report.is_valid, "Source missing Body block must fail lint");
    assert!(fail_report.errors.iter().any(|e| e.contains("Missing BLOCK 3")));
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Test Suite Verification and Status Sealing )
// ==============================================================================
// Failpoint test harness sealed and verified.

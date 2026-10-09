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

#[test]
fn test_repl_eval_and_environment_persistence() {
    let logger = DiagnosticLogger::new(false);
    let mut env = Environment::new();

    // 1. Define axiom PI
    let source1 = "PI < 3.14159 >";
    let mut lexer1 = Lexer::new(source1, logger.clone());
    let tokens1 = lexer1.tokenize().unwrap();
    let debugger1 = OmniDebugger::new(logger.clone(), 5);
    let mut parser1 = Parser::new(tokens1, debugger1);
    let prog1 = parser1.parse_program().unwrap();
    let mut evaluator1 = Evaluator::new(env.clone(), parser1.debugger);
    let res1 = evaluator1.eval(&omnicode::ast::nodes::ASTNode::Program(prog1)).unwrap();
    assert_eq!(res1, RuntimeValue::Number(3.14159));
    env = evaluator1.env;

    // 2. Query environment
    assert_eq!(env.get("PI"), Some(&RuntimeValue::Number(3.14159)));

    // 3. Evaluate identity check in same environment
    let source2 = "status [ +1 = +1 ]";
    let mut lexer2 = Lexer::new(source2, logger.clone());
    let tokens2 = lexer2.tokenize().unwrap();
    let debugger2 = OmniDebugger::new(logger.clone(), 5);
    let mut parser2 = Parser::new(tokens2, debugger2);
    let prog2 = parser2.parse_program().unwrap();
    let mut evaluator2 = Evaluator::new(env.clone(), parser2.debugger);
    let res2 = evaluator2.eval(&omnicode::ast::nodes::ASTNode::Program(prog2)).unwrap();
    assert_eq!(res2, RuntimeValue::State(QuadState::Proceed));
    env = evaluator2.env;

    assert_eq!(env.bindings().len(), 2);
}

#[test]
fn test_cardinal_directions_and_micro_macro_terms() {
    let source = r#"
        step1 [ YES = PROCEED ]
        step2 [ NO = ROLLBACK ]
        step3 [ OK = AWAIT ]
        step4 [ VOID = HALT ]
        forward [ 10 --> 20 ]
        shorthand_right [ 10 -> 20 ]
        upward [ 10 --^ 20 ]
        shorthand_up [ 10 -^ 20 ]
        downward [ 20 --v 10 ]
        shorthand_down [ 20 -v 10 ]
    "#;
    let logger = DiagnosticLogger::new(false);
    let mut lexer = Lexer::new(source, logger.clone());
    let tokens = lexer.tokenize().expect("Tokenize cardinal directions");
    let debugger = OmniDebugger::new(logger, 10);
    let mut parser = Parser::new(tokens, debugger);
    let prog = parser.parse_program().expect("Parse cardinal directions");
    let mut evaluator = Evaluator::new(Environment::new(), parser.debugger);
    let res = evaluator.eval(&omnicode::ast::nodes::ASTNode::Program(prog)).expect("Eval");
    assert_eq!(res, RuntimeValue::Number(10.0));
}

#[test]
fn test_svo_arithmetic_keywords_and_infer_operator() {
    let source = r#"
        Ball < Entity? >
        Group < Set? >
        A [ 4 ]
        new_ball [ 1 ]
        roll_1 [ A --> A plus new_ball ]
        roll_2 [ A = 5 ]
    "#;
    let logger = DiagnosticLogger::new(false);
    let mut lexer = Lexer::new(source, logger.clone());
    let tokens = lexer.tokenize().expect("Tokenize SVO keywords");
    let debugger = OmniDebugger::new(logger, 10);
    let mut parser = Parser::new(tokens, debugger);
    let prog = parser.parse_program().expect("Parse SVO keywords");
    let mut evaluator = Evaluator::new(Environment::new(), parser.debugger);
    let res = evaluator.eval(&omnicode::ast::nodes::ASTNode::Program(prog)).expect("Eval");
    assert_eq!(res, RuntimeValue::State(QuadState::Proceed));
    assert_eq!(evaluator.env.get("A"), Some(&RuntimeValue::Number(5.0)));
}


#[test]
fn test_lib_core_vectors() {
    let source = std::fs::read_to_string("lib/core/vectors.omni").unwrap();
    let logger = DiagnosticLogger::new(false);
    let mut lexer = Lexer::new(&source, logger.clone());
    let tokens = lexer.tokenize().expect("Tokenize");
    let debugger = OmniDebugger::new(logger.clone(), 10);
    let mut parser = Parser::new(tokens, debugger);
    let prog = parser.parse_program().expect("Parse");
    let mut evaluator = Evaluator::new(Environment::new(), parser.debugger);
    let res = evaluator.eval(&omnicode::ast::nodes::ASTNode::Program(prog)).expect("Eval");
    assert_eq!(res, RuntimeValue::State(QuadState::Proceed));
    assert!(!logger.has_failpoints(), "Failpoints: {:?}", logger.failpoints());
}

#[test]
fn test_module_import_and_library_resolution() {
    let source = r#"
        Opening {
            import: [ Core.Arithmetic :: Core.Logic ],
            preconditions: [ +1 ]
        }
        test_val [ 50 ]
        delta [ 10 ]
        res [ test_val --> test_val plus delta ]
        Closing {
            verify: [
                res = 60 ::
                ADD = InvariantMacro
            ],
            commit_status [ PROCEED ]
        }
    "#;
    let logger = DiagnosticLogger::new(false);
    let mut lexer = Lexer::new(source, logger.clone());
    let tokens = lexer.tokenize().expect("Tokenize module import");
    let debugger = OmniDebugger::new(logger, 10);
    let mut parser = Parser::new(tokens, debugger);
    let prog = parser.parse_program().expect("Parse module import");
    let mut evaluator = Evaluator::new(Environment::new(), parser.debugger);
    let res = evaluator.eval(&omnicode::ast::nodes::ASTNode::Program(prog)).expect("Eval");
    assert_eq!(res, RuntimeValue::State(QuadState::Proceed));

    // Verify imported modules tracked
    assert!(evaluator.loaded_modules.contains("Core.Arithmetic"));
    assert!(evaluator.loaded_modules.contains("Core.Logic"));

    // Verify symbols from imported libraries populated in environment
    assert!(evaluator.env.contains("ADD"), "ADD macro must be present from Core.Arithmetic");
    assert!(evaluator.env.contains("SUB"), "SUB macro must be present from Core.Arithmetic");
    assert!(evaluator.env.contains("PROCEED_MACRO"), "PROCEED_MACRO must be present from Core.Logic");

    // Verify immutability of imported Rank 1 invariant macro
    assert!(evaluator.debugger.immutable_registry.contains("ADD"));
    assert_eq!(evaluator.env.get("res"), Some(&RuntimeValue::Number(60.0)));
}

#[test]
fn test_restorator_boundary_and_structural_auto_heal() {
    let broken_source = r#"
        bad_boundary < 42 ]
        unclosed_block [ 10 20
        step [ 5 --> 10 ]
    "#;
    let logger = DiagnosticLogger::new(false);
    let restorator = omnicode::Restorator::new(logger);
    let report = restorator.restore_source(broken_source);

    assert!(report.was_modified, "Broken source must be modified by restorator");
    assert!(!report.repairs_applied.is_empty(), "Restorator must log applied repairs");
    assert!(report.verified, "Restored code must pass verification: {:?}", report.errors);
    assert_eq!(report.final_state, Some(QuadState::Proceed));
    assert!(report.restored_content.contains("BLOCK 1: METADATA"));
    assert!(report.restored_content.contains("BLOCK 2: OPENING"));
    assert!(report.restored_content.contains("BLOCK 3: BODY"));
    assert!(report.restored_content.contains("BLOCK 4: CLOSING"));
}

#[test]
fn test_ai_semantic_transducer_round_trip() {
    let prompt = "Add 1 ball to the group A which contains 4 balls";
    let logger = DiagnosticLogger::new(false);
    let transducer = omnicode::AITransducer::new(logger);
    let session = transducer.transduce_and_execute(prompt).expect("Transduction must succeed");

    assert_eq!(session.execution_state, QuadState::Proceed);
    assert!(session.synthesized_omnicode.contains("Ball < Entity? >"));
    assert!(session.synthesized_omnicode.contains("Group < Set? >"));
    assert!(session.synthesized_omnicode.contains("A [ 4 : Ball? ]"));
    assert!(session.synthesized_omnicode.contains("A --> A plus new_Ball"));
    assert!(session.synthesized_omnicode.contains("roll_1 = 5"));
    assert!(session.natural_response.contains("I added 1 ball to Group A"));
    assert!(session.natural_response.contains("5 balls in total"));
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Test Suite Verification and Status Sealing )
// ==============================================================================
// Failpoint test harness sealed and verified.

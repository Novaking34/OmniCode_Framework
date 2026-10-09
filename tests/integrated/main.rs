// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Integration Test Suite Identity, Subsystem Interactions & Failpoint Invariants )
// ==============================================================================
//! OmniCode Subsystem Integration Test Suite
//!
//! Verifies interactions between compiler components, runtime engines,
//! and the autonomous self-healing loop:
//!   - Boundary physics and asymmetric causality delimiters.
//!   - Failpoint traps (mismatches, leaks, immutable mutations, caveat vetoes).
//!   - Cyclic retroaction depth bounding (infinite rollback protection).
//!   - 4-Block structural linter validation.
//!   - Module import and core standard library resolution.
//!   - Connected Self-Healing Feedback Loop:
//!     Logger -> Structured Cognitive Trace -> Debugger/Restorator -> Test Oracle.

use omnicode::core::boundaries::BoundaryType;
use omnicode::core::ternary::QuadState;
use omnicode::diagnostics::logger::{DiagnosticLogger, FailpointType};
use omnicode::diagnostics::debugger::OmniDebugger;
use omnicode::diagnostics::linter::OmniLinter;
use omnicode::parser::lexer::Lexer;
use omnicode::parser::parser::Parser;
use omnicode::runtime::environment::{Environment, RuntimeValue};
use omnicode::runtime::evaluator::Evaluator;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Integration Test Harness Setup & Invariant Preconditions )
// ==============================================================================
/// Constructs an integration test fixture containing a silent [`DiagnosticLogger`]
/// and an [`OmniDebugger`] with a custom retroaction limit threshold.
///
/// # Arguments
/// * `threshold` - Maximum retroactions permitted before triggering [`FailpointType::CyclicRetroaction`].
fn create_test_debugger(threshold: usize) -> (DiagnosticLogger, OmniDebugger) {
    let logger = DiagnosticLogger::new(false);
    let debugger = OmniDebugger::new(logger.clone(), threshold);
    (logger, debugger)
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Subsystem Integration Cases: Failpoints, Imports, Self-Healing Loop )
// ==============================================================================

/// Verifies asymmetric causality delimiters that govern directional influence:
/// - `{ ... ]`: Top-Down / Outside-In imposition by ambient context.
/// - `[ ... }`: Bottom-Up / Inside-Out emergence defined by internal denotation.
/// - Standard symmetric pairs: `[ ... ]` (Denotation), `< ... >` (Invariant), `{ ... }` (Connotation), `( ... )` (Aside).
#[test]
fn test_asymmetric_causality_boundaries() {
    assert_eq!(BoundaryType::from_delimiters('{', ']').unwrap(), BoundaryType::TopDown);
    assert_eq!(BoundaryType::from_delimiters('[', '}').unwrap(), BoundaryType::BottomUp);
    assert_eq!(BoundaryType::from_delimiters('[', ']').unwrap(), BoundaryType::Denotation);
    assert_eq!(BoundaryType::from_delimiters('<', '>').unwrap(), BoundaryType::Invariant);
    assert_eq!(BoundaryType::from_delimiters('{', '}').unwrap(), BoundaryType::Connotation);
    assert_eq!(BoundaryType::from_delimiters('(', ')').unwrap(), BoundaryType::Aside);
}

/// Verifies that closing a boundary with an illegal partner (e.g. `< ... ]`) triggers
/// a [`FailpointType::BoundaryMismatch`], records remediation telemetry, and creates
/// an episodic [`crate::diagnostics::logger::CognitiveNote`] for AI memory.
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

    // Verify cognitive notes captured the incident for AI episodic memory
    let notes = logger.cognitive_notes();
    assert!(!notes.is_empty(), "Incident note must be recorded");
    assert_eq!(notes[0].category, "INCIDENT");
    assert!(notes[0].tags.contains(&"remediation_ready".to_string()));
}

/// Verifies that reaching EOF while delimiters remain open on the stack triggers
/// [`FailpointType::BoundaryLeak`] with a remediation patch proposing the missing closers.
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

/// Verifies that attempting to rebind or mutate an entity declared inside `< ... >`
/// triggers [`FailpointType::ImmutableMutation`] and suggests converting it to `[ ... ]`.
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

/// Verifies that an unsatisfied adversative caveat expression (`! [ ... ]`) vetoes
/// the active state transition, triggers [`FailpointType::CaveatVeto`], and returns [`QuadState::Rollback`].
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

/// Verifies that evaluating into a void state without a resolving caveat triggers
/// [`FailpointType::NullTrap`] and halts execution into [`QuadState::Halt`].
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

/// Verifies that repeated retroactions on the same label exceeding the threshold limit
/// are caught and aborted as [`FailpointType::CyclicRetroaction`].
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

/// Verifies structural linter enforcement of the universal 4-Block architecture:
/// Confirms that compliant code passes, and non-compliant code missing blocks is flagged with errors.
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

/// Verifies standard library imports (`Core.Arithmetic`, `Core.Logic`) inside the `Opening` block,
/// confirming that library macros (`ADD`, `SUB`, `PROCEED_MACRO`) are resolved and registered.
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

    assert!(evaluator.loaded_modules.contains("Core.Arithmetic"));
    assert!(evaluator.loaded_modules.contains("Core.Logic"));
    assert!(evaluator.env.contains("ADD"), "ADD macro must be present from Core.Arithmetic");
    assert!(evaluator.env.contains("SUB"), "SUB macro must be present from Core.Arithmetic");
    assert!(evaluator.env.contains("PROCEED_MACRO"), "PROCEED_MACRO must be present from Core.Logic");
    assert!(evaluator.debugger.immutable_registry.contains("ADD"));
    assert_eq!(evaluator.env.get("res"), Some(&RuntimeValue::Number(60.0)));
}

/// Verifies that the standard vector library `lib/core/vectors.omni` parses and executes cleanly.
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

/// Verifies multi-pass automatic healing of syntax errors and unclosed delimiters
/// via [`crate::diagnostics::restorator::Restorator`].
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

/// Verifies end-to-end prompt transduction into executable AST and natural response generation
/// via [`crate::ai::SemanticTransducer`].
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

/// The Cybernetic Self-Healing Feedback Loop Test:
///
/// Demonstrates the full feedback loop between sensory logging, debugger intervention,
/// restorative patch synthesis, and test oracle verification:
/// 1. **Phase 1 (Anomaly Detection)**: Execution hits an illegal delimiter crossing (`< 99 ]`).
/// 2. **Phase 2 (Structured Telemetry)**: Logger records a [`FailpointType::BoundaryMismatch`]
///    equipped with a [`crate::diagnostics::logger::FixSuggestion`].
/// 3. **Phase 3 (Oracle-Verified Restoration)**: The Debugger consumes the structured log,
///    applies the patch, and runs an isolated test oracle to certify that the fix is sound.
/// 4. **Phase 4 (Immune System Restoration)**: Restorator executes round-trip verification with an oracle.
/// 5. **Phase 5 (Episodic Memory Audit)**: Verifies that cognitive memory notes reflect the certified repair.
#[test]
fn test_self_healing_feedback_loop_logger_debugger_oracle() {
    let broken_input = "rank1_invariant < 99 ]";
    let logger = DiagnosticLogger::new(false);

    // Phase 1: Anomaly Encounter & Structured Telemetry Inscription
    let mut lexer = Lexer::new(broken_input, logger.clone());
    let tokens = lexer.tokenize().expect("Lexing succeeds");
    let mut debugger = OmniDebugger::new(logger.clone(), 5);
    let mut parser = Parser::new(tokens, debugger.clone());
    let parse_err = parser.parse_program();
    assert!(parse_err.is_err(), "Must trap boundary mismatch");

    // Phase 2: Inspect Structured LogEvent and Remediation Suggestion
    let failpoints = logger.failpoints();
    assert_eq!(failpoints.len(), 1);
    let incident = &failpoints[0];
    assert_eq!(incident.failpoint_type, Some(FailpointType::BoundaryMismatch));
    assert!(incident.suggestion.is_some(), "Structured remediation required");

    // Phase 3: The Debugger applies the restoration from the structured log and invokes the test oracle
    let oracle = |candidate_code: &str| -> Result<(), String> {
        let test_logger = DiagnosticLogger::new(false);
        let mut test_lexer = Lexer::new(candidate_code, test_logger.clone());
        let test_tokens = test_lexer.tokenize().map_err(|e| format!("Lex error: {e}"))?;
        let test_debugger = OmniDebugger::new(test_logger.clone(), 5);
        let mut test_parser = Parser::new(test_tokens, test_debugger);
        test_parser.parse_program().map_err(|e| format!("Parse error: {e}"))?;
        if test_logger.has_failpoints() {
            return Err("Candidate caused failpoints".to_string());
        }
        Ok(())
    };

    let healed = debugger
        .execute_restoration_from_log(broken_input, incident, oracle)
        .expect("Self-healing restoration must be verified by oracle");

    assert_eq!(healed, "rank1_invariant < 99 >\n");

    // Phase 4: Restorator round-trip oracle certification
    let restorator = omnicode::Restorator::new(logger.clone());
    let report = restorator.restore_and_verify_with_oracle(broken_input, |code| {
        if code.contains("bad_boundary") {
            Err("Forbidden pattern".to_string())
        } else {
            Ok(())
        }
    });

    assert!(report.verified, "Restorator oracle certification succeeded");
    assert!(report.was_modified);

    // Phase 5: Confirm cognitive notes audit trace contains the verified repair
    let notes = logger.cognitive_notes();
    assert!(notes.iter().any(|n| n.category == "REPAIR_VERIFIED" || n.category == "ORACLE_CERTIFIED"));
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Integration Verification Proof & Postconditions )
// ==============================================================================
// Integration test harness sealed and verified.

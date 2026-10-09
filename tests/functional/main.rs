// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Functional Test Suite Identity, E2E Cognitive Workflows & Lifecycle Invariants )
// ==============================================================================
//! OmniCode Functional Test Suite
//!
//! Verifies end-to-end user journeys, live file lifecycle operations, and the
//! complete neural-symbolic cognitive loop:
//!   - REPL interactive session simulation with persistent environment state.
//!   - Canonical syntax example file parsing and execution from disk.
//!   - AI Cortex end-to-end prompt-to-evaluation pipeline producing dual files
//!     (.scroll native Turing reels & .log traditional execution traces).
//!   - AILearner episodic memory digestion of execution scrolls.
//!   - End-to-end file healing with in-place disk modification and re-execution.

use std::fs;
use std::path::PathBuf;
use omnicode::core::ternary::QuadState;
use omnicode::ai::{AICortex, AILearner};
use omnicode::diagnostics::logger::DiagnosticLogger;
use omnicode::diagnostics::debugger::OmniDebugger;
use omnicode::diagnostics::restorator::Restorator;
use omnicode::parser::lexer::Lexer;
use omnicode::parser::parser::Parser;
use omnicode::runtime::environment::{Environment, RuntimeValue};
use omnicode::runtime::evaluator::Evaluator;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Test Artifact Isolation & Fixture Initialization )
// ==============================================================================
/// Prepares an isolated test workspace directory within `target/test_artifacts/`.
///
/// # Invariants & Isolation
/// - If the target directory already exists from a previous run, it is purged.
/// - A fresh, empty directory is created to prevent test cross-contamination.
///
/// # Arguments
/// * `name` - Subdirectory identifier uniquely distinguishing the functional test run.
///
/// # Returns
/// A [`PathBuf`] pointing to the clean artifact directory on disk.
fn get_test_dir(name: &str) -> PathBuf {
    let dir = PathBuf::from(format!("target/test_artifacts/functional_{}", name));
    if dir.exists() {
        fs::remove_dir_all(&dir).ok();
    }
    fs::create_dir_all(&dir).ok();
    dir
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( End-to-End Functional Test Cases: REPL, Scripts, AI Cortex, Digestion, Repair )
// ==============================================================================

/// Validates REPL interactive simulation and multi-turn environment state persistence.
///
/// # Cognitive & IDE Invariant
/// In an interactive Read-Eval-Print Loop (REPL), variable bindings and invariant definitions
/// established in step $N$ must remain accessible and immutable in step $N+1$.
/// This test simulates multiple discrete parse-eval turns over a shared [`Environment`]:
/// 1. Defining a mathematical invariant axiom (`PI < 3.14159 >`).
/// 2. Verifying symbol presence in the environment across evaluate boundaries.
/// 3. Evaluating a dynamic comparison expression (`status [ +1 = +1 ]`) using the same context.
#[test]
fn test_repl_eval_and_environment_persistence() {
    let logger = DiagnosticLogger::new(false);
    let mut env = Environment::new();

    // 1. Step 1: Define invariant axiom PI in REPL session
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

    // 2. Step 2: Query stored variable across REPL iterations
    assert_eq!(env.get("PI"), Some(&RuntimeValue::Number(3.14159)));

    // 3. Step 3: Evaluate comparison using previous binding
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

/// Verifies that the canonical OmniCode notation specification file executes successfully.
///
/// # Test Rationale & Regression Guard
/// Reads `examples/canonical_notation.omni` directly from disk and runs it through the
/// high-level [`omnicode::execute`] pipeline. This guarantees that real-world sample files
/// shipped with the repository remain syntax-compliant and semantically valid as the
/// parser, runtime, and diagnostics evolve.
#[test]
fn test_canonical_notation_file() {
    let source = fs::read_to_string("examples/canonical_notation.omni")
        .expect("Failed to read examples/canonical_notation.omni");

    let result = omnicode::execute(&source, false);
    assert!(
        result.is_ok(),
        "Canonical notation file must parse and evaluate successfully: {:?}",
        result.err()
    );
}

/// Validates the full neural-symbolic cognitive pipeline of [`AICortex`].
///
/// # Cognitive Workflow Under Test
/// 1. Initializes an [`AICortex`] configured with an isolated scratch directory.
/// 2. Submits an arithmetic prompt requiring symbolic reasoning ("Add 2 balls to group A...").
/// 3. Confirms that the execution state reaches [`QuadState::Proceed`].
/// 4. Confirms that the natural language response synthesizes the correct computed value ("7 balls").
/// 5. Verifies dual file emission on disk:
///    - Native `.scroll` containing full 4-Block cognitive structure, epistemic traces, and commit status.
///    - Traditional `.log` containing human-readable session metadata and execution state.
#[test]
fn test_ai_cortex_end_to_end_pipeline() {
    let test_dir = get_test_dir("cortex_pipeline");
    let mut cortex = AICortex::with_scroll_dir(&test_dir);

    let prompt = "Add 2 balls to the group A which contains 5 balls";
    let res = cortex.process_prompt(prompt).expect("Cortex processing should succeed");

    assert_eq!(res.execution_state, QuadState::Proceed, "Execution must reach +1:PROCEED");
    assert!(res.natural_response.contains("7 balls"), "Response must contain evaluated sum 7 balls");

    // Verify dual log generation on disk
    assert!(res.scroll_path.exists(), "OmniCode .scroll must exist on disk");
    assert!(res.log_path.exists(), "Traditional .log must exist on disk");

    // Verify .scroll adheres to 4-Block architecture
    let scroll_content = fs::read_to_string(&res.scroll_path).expect("Must read .scroll");
    assert!(scroll_content.contains("BLOCK 1: METADATA BLOCK"), "Scroll must contain Metadata Block");
    assert!(scroll_content.contains("Scroll < EpistemicTrace."), "Scroll must have EpistemicTrace header");
    assert!(scroll_content.contains("BLOCK 2: OPENING BLOCK"), "Scroll must contain Opening Block");
    assert!(scroll_content.contains("BLOCK 3: BODY BLOCK"), "Scroll must contain Body Block");
    assert!(scroll_content.contains("BLOCK 4: CLOSING BLOCK"), "Scroll must contain Closing Block");
    assert!(scroll_content.contains("commit_status [ <PROCEED> ]"), "Scroll must record final commit state");

    // Verify traditional .log format
    let log_content = fs::read_to_string(&res.log_path).expect("Must read .log");
    assert!(log_content.contains("OMNICODE AI MODEL EXECUTION LOG"), "Log header required");
    assert!(log_content.contains(&format!("Session ID:   {}", res.session_id)), "Session ID must match");
    assert!(log_content.contains("Final State:  +1:PROCEED"), "Final state must match");
}

/// Validates experiential memory aggregation via [`AILearner`].
///
/// # Learning Loop Under Test
/// 1. Executes two independent cognitive sessions through [`AICortex`], creating two `.scroll` files.
/// 2. Initializes an [`AILearner`] pointing to the session directory.
/// 3. Invokes [`AILearner::digest_experiences`] to parse, validate, and summarize episodic logs.
/// 4. Asserts that all episodes were ingested, state metrics reflect success, and log paths are verified.
#[test]
fn test_ai_learner_digests_dual_logs() {
    let test_dir = get_test_dir("learner_digestion");
    let mut cortex = AICortex::with_scroll_dir(&test_dir);

    // Run two distinct cognitive sessions
    cortex.process_prompt("Add 3 apples to the basket B which contains 4 apples").expect("Session 1 succeeds");
    cortex.process_prompt("Subtract 2 coins from the bag C which contains 10 coins").expect("Session 2 succeeds");

    // Digest experiences using AILearner
    let learner = AILearner::new(&test_dir);
    let summary = learner.digest_experiences().expect("Experience digestion succeeds");

    assert_eq!(summary.total_episodes, 2, "Learner must ingest exactly 2 episodes");
    assert_eq!(summary.proceed_count, 2, "Both sessions must have achieved +1:PROCEED");
    assert_eq!(summary.experiences.len(), 2, "Summary must contain 2 experience records");

    for exp in &summary.experiences {
        assert!(exp.scroll_path.exists(), "Experience scroll path must exist");
        assert!(exp.log_path.is_some(), "Experience traditional log must exist");
        assert_eq!(exp.final_state, QuadState::Proceed);
    }
}

/// Validates closed-loop autonomous file self-healing on physical disk files.
///
/// # End-to-End Self-Healing Lifecycle
/// 1. Writes syntactically malformed OmniCode code with boundary mismatches and missing blocks to disk.
/// 2. Invokes [`Restorator::restore_file`] with in-place overwriting enabled (`overwrite = true`).
/// 3. Verifies that the resulting file:
///    - Was modified and marked as verified by the internal test oracle.
///    - Contains complete 4-Block architecture envelopes.
///    - Can be read back from disk and successfully executed via [`omnicode::execute`].
#[test]
fn test_e2e_file_self_healing_lifecycle() {
    let test_dir = get_test_dir("file_healing");
    let broken_file = test_dir.join("damaged_script.omni");

    let damaged_code = r#"
broken_val < 77 ]
incomplete_block [ 10 20
calc [ 10 --> 10 plus 5 ]
"#;
    fs::write(&broken_file, damaged_code).expect("Write damaged file");

    let logger = DiagnosticLogger::new(false);
    let restorator = Restorator::new(logger.clone());

    // Execute in-place file restoration
    let report = restorator
        .restore_file(broken_file.to_str().unwrap(), true)
        .expect("File restoration succeeds");

    assert!(report.was_modified, "File should have been healed");
    assert!(report.verified, "Healed code must verify");

    // Read back healed file from disk and evaluate end-to-end
    let healed_content = fs::read_to_string(&broken_file).expect("Read healed file");
    assert!(healed_content.contains("BLOCK 1: METADATA BLOCK"));
    assert!(healed_content.contains("BLOCK 4: CLOSING BLOCK"));

    let eval_res = omnicode::execute(&healed_content, false);
    assert!(eval_res.is_ok(), "Healed file must execute successfully from disk");
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Functional Test Suite Verification Proof & Postconditions )
// ==============================================================================
// Functional test suite verified and sealed.

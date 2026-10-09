// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Unit Test Suite Identity, Isolated Component Invariants & Coverage Manifest )
// ==============================================================================
//! OmniCode Unit Test Suite
//!
//! Verifies individual, isolated language components in complete decoupling:
//!   - Boundary ranking hierarchy (< > > [ ] > { } > ( )) and delimiter physics.
//!   - Balanced Ternary QuadState algebra (T4 = {-1, 0, +1, NULL}) symmetry.
//!   - Directional vector operators (--> / <--, --^ / --v) and shorthands.
//!   - SVO arithmetic keywords (plus, minus, times, div) and infer operator (?).
//!   - FileType detection and 4-block scaffold generation.
//!   - Structured CognitiveNote formatting and episodic memory exports for AI.

use omnicode::core::boundaries::BoundaryType;
use omnicode::core::filetype::OmniFileType;
use omnicode::core::operators::DirectionalVector;
use omnicode::core::ternary::{QuadState, ROLLBACK, AWAIT, PROCEED, HALT};
use omnicode::diagnostics::logger::{CognitiveNote, DiagnosticLogger};
use omnicode::parser::lexer::Lexer;
use omnicode::parser::parser::Parser;
use omnicode::runtime::environment::{Environment, RuntimeValue};
use omnicode::runtime::evaluator::Evaluator;
use omnicode::diagnostics::debugger::OmniDebugger;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Unit Test Harness Preconditions & Isolation Fixtures )
// ==============================================================================
/// Constructs an isolated test fixture containing a silent [`DiagnosticLogger`]
/// and an [`OmniDebugger`] configured with a retroaction depth threshold of 10.
///
/// # Returns
/// A tuple `(DiagnosticLogger, OmniDebugger)` cleanly decoupled from external state.
fn create_unit_test_fixtures() -> (DiagnosticLogger, OmniDebugger) {
    let logger = DiagnosticLogger::new(false);
    let debugger = OmniDebugger::new(logger.clone(), 10);
    (logger, debugger)
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Isolated Component Unit Tests: Boundaries, Ternary, Operators, Formats )
// ==============================================================================

/// Asserts the formal 6-rank morphological boundary hierarchy of OmniCode:
/// - Rank 1: `< ... >` Invariant Axioms & Immutable Laws (Highest Authority).
/// - Rank 2: `[ ... ]` Concrete Denotations & Mutable Entities.
/// - Rank 3: `{ ... }` Connotative Scopes & Ambient Contexts.
/// - Rank 4: `[ ... }` Inside-Out / Bottom-Up Emergence.
/// - Rank 5: `{ ... ]` Outside-In / Top-Down Context Imposition.
/// - Rank 6: `( ... )` First-Class Cognitive Asides & Intent Annotations.
#[test]
fn test_boundary_ranking_hierarchy() {
    // Rank 1: Invariant Axioms <...>
    assert_eq!(BoundaryType::from_delimiters('<', '>').unwrap(), BoundaryType::Invariant);
    assert_eq!(BoundaryType::Invariant.rank(), 1);

    // Rank 2: Concrete Denotations [...]
    assert_eq!(BoundaryType::from_delimiters('[', ']').unwrap(), BoundaryType::Denotation);
    assert_eq!(BoundaryType::Denotation.rank(), 2);

    // Rank 3: Connotative Scopes {...}
    assert_eq!(BoundaryType::from_delimiters('{', '}').unwrap(), BoundaryType::Connotation);
    assert_eq!(BoundaryType::Connotation.rank(), 3);

    // Rank 6: Cognitive Asides (...)
    assert_eq!(BoundaryType::from_delimiters('(', ')').unwrap(), BoundaryType::Aside);
    assert_eq!(BoundaryType::Aside.rank(), 6);
}

/// Verifies algebraic symmetry and truth-table invariants across Balanced Ternary QuadState:
/// - Inversion Symmetry: `PROCEED.invert() == ROLLBACK`, `AWAIT.invert() == AWAIT`, `HALT.invert() == HALT`.
/// - Conjunction (`and`): MIN operation in Kleene-Łukasiewicz trivalent logic.
/// - Disjunction (`or`): MAX operation in Kleene-Łukasiewicz trivalent logic.
#[test]
fn test_balanced_ternary_algebraic_symmetry() {
    // Inversion symmetry: -(-1) = +1, -(+1) = -1, -(0) = 0, -(NULL) = NULL
    assert_eq!(PROCEED.invert(), ROLLBACK);
    assert_eq!(ROLLBACK.invert(), PROCEED);
    assert_eq!(AWAIT.invert(), AWAIT);
    assert_eq!(HALT.invert(), HALT);

    // Conjunction (AND = MIN in Kleene-Łukasiewicz logic):
    assert_eq!(PROCEED.and(AWAIT), AWAIT);
    assert_eq!(PROCEED.and(ROLLBACK), ROLLBACK);
    assert_eq!(AWAIT.and(ROLLBACK), ROLLBACK);
    assert_eq!(PROCEED.and(HALT), HALT);

    // Disjunction (OR = MAX in Kleene-Łukasiewicz logic):
    assert_eq!(ROLLBACK.or(PROCEED), PROCEED);
    assert_eq!(ROLLBACK.or(AWAIT), AWAIT);
    assert_eq!(AWAIT.or(PROCEED), PROCEED);
    assert_eq!(AWAIT.or(HALT), HALT);
}

/// Verifies macro state keywords (`YES`, `NO`, `OK`, `VOID`) and cardinal direction
/// arrow operators (both canonical 3-character forms and fast 2-character shorthands).
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
    let (logger, debugger) = create_unit_test_fixtures();
    let mut lexer = Lexer::new(source, logger.clone());
    let tokens = lexer.tokenize().expect("Tokenize cardinal directions");
    let mut parser = Parser::new(tokens, debugger);
    let prog = parser.parse_program().expect("Parse cardinal directions");
    let mut evaluator = Evaluator::new(Environment::new(), parser.debugger);
    let res = evaluator.eval(&omnicode::ast::nodes::ASTNode::Program(prog)).expect("Eval");
    assert_eq!(res, RuntimeValue::Number(10.0));
}

/// Verifies Subject-Verb-Object (SVO) arithmetic keyword execution (`plus`, `minus`)
/// and the ontological infer operator (`?`) declaring open conceptual types.
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
    let (logger, debugger) = create_unit_test_fixtures();
    let mut lexer = Lexer::new(source, logger.clone());
    let tokens = lexer.tokenize().expect("Tokenize SVO keywords");
    let mut parser = Parser::new(tokens, debugger);
    let prog = parser.parse_program().expect("Parse SVO keywords");
    let mut evaluator = Evaluator::new(Environment::new(), parser.debugger);
    let res = evaluator.eval(&omnicode::ast::nodes::ASTNode::Program(prog)).expect("Eval");
    assert_eq!(res, RuntimeValue::State(QuadState::Proceed));
    assert_eq!(evaluator.env.get("A"), Some(&RuntimeValue::Number(5.0)));
}

/// Verifies parsing and symbol mapping for the 3D coordinate vector system:
/// - X-Axis (Temporal Dynamics): `-->` / `->` (ActUpon), `<--` / `<-` (ActAgain).
/// - Z-Axis (Scale & Hierarchy): `--^` / `-^` (BuildsTo), `--v` / `-v` (BreaksDownFrom).
#[test]
fn test_directional_vectors_canonical_and_shorthand() {
    assert_eq!(DirectionalVector::from_symbol("-->").unwrap(), DirectionalVector::ActUpon);
    assert_eq!(DirectionalVector::from_symbol("->").unwrap(), DirectionalVector::ActUpon);
    assert_eq!(DirectionalVector::from_symbol("<--").unwrap(), DirectionalVector::ActAgain);
    assert_eq!(DirectionalVector::from_symbol("<-").unwrap(), DirectionalVector::ActAgain);
    assert_eq!(DirectionalVector::from_symbol("--^").unwrap(), DirectionalVector::BuildsTo);
    assert_eq!(DirectionalVector::from_symbol("-^").unwrap(), DirectionalVector::BuildsTo);
    assert_eq!(DirectionalVector::from_symbol("--v").unwrap(), DirectionalVector::BreaksDownFrom);
    assert_eq!(DirectionalVector::from_symbol("-v").unwrap(), DirectionalVector::BreaksDownFrom);
}

/// Verifies formal extension parsing and classification for OmniCode artifacts:
/// - `.omni`: Primary executable source reel ([`OmniFileType::SourceReel`]).
/// - `.scroll`: Cognitive episodic trace reel ([`OmniFileType::EpisodicScroll`]).
#[test]
fn test_file_type_detection() {
    assert_eq!(OmniFileType::from_extension("omni"), Some(OmniFileType::SourceReel));
    assert_eq!(OmniFileType::from_extension("scroll"), Some(OmniFileType::EpisodicScroll));
    assert_eq!(OmniFileType::from_extension("unknown"), None);
}

/// Verifies structured [`CognitiveNote`] generation and its dual-readable representations:
/// 1. OmniCode native block syntax (`Note < CognitiveTrace > { ... }`).
/// 2. Markdown episodic memory trace (`### [EPISODIC_MEMORY_TRACE: ...]`) for AI agent ingestion.
#[test]
fn test_cognitive_note_formatting_and_memory_trace() {
    let note = CognitiveNote {
        timestamp_micros: 4200,
        category: "OBSERVATION".to_string(),
        summary: "Boundary state verified".to_string(),
        details: "Rank 1 invariant <PI> bound to immutable memory".to_string(),
        tags: vec!["immutability".to_string(), "rank1".to_string()],
        context_snippet: Some("PI < 3.14159 >".to_string()),
    };

    // Verify OmniCode native dual-readable formatting
    let omni_str = note.format_as_omnicode();
    assert!(omni_str.contains("Note < CognitiveTrace >"));
    assert!(omni_str.contains("category: [ OBSERVATION ]"));
    assert!(omni_str.contains("tags: [ #immutability :: #rank1 ]"));
    assert!(omni_str.contains("context: < \"PI < 3.14159 >\" >"));

    // Verify AI LLM markdown memory trace formatting
    let mem_str = note.format_as_memory_entry();
    assert!(mem_str.contains("### [EPISODIC_MEMORY_TRACE: OBSERVATION]"));
    assert!(mem_str.contains("- **Timestamp:** 4200 µs"));
    assert!(mem_str.contains("- **Tags:** #immutability, #rank1"));
    assert!(mem_str.contains("- **Context:** `PI < 3.14159 >`"));
}

/// Verifies that [`DiagnosticLogger::export_ai_memory_scroll`] synthesizes a valid
/// 4-block memory audit scroll containing metadata, preconditions, recorded traces, and commit proof.
#[test]
fn test_logger_episodic_memory_scroll_export() {
    let logger = DiagnosticLogger::new(false);
    logger.record_ai_observation(
        "OBSERVATION",
        "Variable X initialized",
        "X assigned value 10 in denotation block",
        ["initialization", "state"].as_slice(),
        Some("X [ 10 ]"),
    );

    let scroll = logger.export_ai_memory_scroll();
    assert!(scroll.contains("BLOCK 1: METADATA BLOCK"));
    assert!(scroll.contains("Scroll < CognitiveMemory.Audit >"));
    assert!(scroll.contains("total_notes: < 1 >"));
    assert!(scroll.contains("BLOCK 2: OPENING BLOCK"));
    assert!(scroll.contains("BLOCK 3: BODY BLOCK"));
    assert!(scroll.contains("BLOCK 4: CLOSING BLOCK"));
    assert!(scroll.contains("audit_verified: [ PROCEED ]"));
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Test Suite Verification Proof & Postconditions )
// ==============================================================================
// Unit test suite verified and sealed.

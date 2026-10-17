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
use omnicode::runtime::assembler::{OmniAssembler, MachineRegister, transpile_tape_to_omni};
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

/// Verifies that Cardinal Classification Clauses (WHO, WHAT, WHEN, WHERE, WHY, HOW),
/// spatial directional modulations (/-^, \-v, \-^, /-v, /^, \v, ^\, v/), range span (..),
/// and runtime Storehouse partitioning via `v--` execute with zero errors.
#[test]
fn test_cardinal_clauses_and_spatial_directionals_and_partitioning() {
    let (logger, debugger) = create_unit_test_fixtures();
    let source = r#"
        WHO self
        WHAT Cell
        WHEN Opening
        WHERE {unit}
        WHY ( Intent )
        HOW [ 0 .. 256 ]
        /-^ \-v \-^ /-v /^ \v ^\ v/
    "#;
    let mut lexer = Lexer::new(source, logger.clone());
    let tokens = lexer.tokenize().expect("Tokenization should succeed");
    assert!(tokens.iter().any(|t| t.token_type == omnicode::parser::lexer::TokenType::Who));
    assert!(tokens.iter().any(|t| t.token_type == omnicode::parser::lexer::TokenType::What));
    assert!(tokens.iter().any(|t| t.token_type == omnicode::parser::lexer::TokenType::When));
    assert!(tokens.iter().any(|t| t.token_type == omnicode::parser::lexer::TokenType::Where));
    assert!(tokens.iter().any(|t| t.token_type == omnicode::parser::lexer::TokenType::Why));
    assert!(tokens.iter().any(|t| t.token_type == omnicode::parser::lexer::TokenType::How));
    assert!(tokens.iter().any(|t| t.token_type == omnicode::parser::lexer::TokenType::DotDot));
    assert!(tokens.iter().any(|t| t.token_type == omnicode::parser::lexer::TokenType::Major));
    assert!(tokens.iter().any(|t| t.token_type == omnicode::parser::lexer::TokenType::Minor));
    assert!(tokens.iter().any(|t| t.token_type == omnicode::parser::lexer::TokenType::Great));
    assert!(tokens.iter().any(|t| t.token_type == omnicode::parser::lexer::TokenType::Small));
    assert!(tokens.iter().any(|t| t.token_type == omnicode::parser::lexer::TokenType::UpperRight));
    assert!(tokens.iter().any(|t| t.token_type == omnicode::parser::lexer::TokenType::LowerRight));
    assert!(tokens.iter().any(|t| t.token_type == omnicode::parser::lexer::TokenType::UpperLeft));
    assert!(tokens.iter().any(|t| t.token_type == omnicode::parser::lexer::TokenType::LowerLeft));

    // Test v-- Storehouse Partitioning in Evaluator
    let partition_src = r#"
        Storehouse [ 1024 ]
        Storehouse v-- Scratchpad [ 256 ]
    "#;
    let mut part_lexer = Lexer::new(partition_src, logger.clone());
    let part_tokens = part_lexer.tokenize().expect("Lexing partition code");
    let mut part_parser = Parser::new(part_tokens, debugger.clone());
    let ast = part_parser.parse_program().expect("Parsing partition code");

    let env = Environment::new();
    let mut eval = Evaluator::new(env, debugger.clone());
    for stmt in ast.statements {
        eval.eval(&stmt).expect("Evaluation must succeed");
    }

    // Verify Scratchpad partition exists with capacity 256
    let scratchpad = eval.env.get("Scratchpad").expect("Scratchpad partition must be registered");
    if let RuntimeValue::Vessel(v) = scratchpad {
        assert_eq!(v.capacity, 256);
        assert_eq!(v.parent.as_deref(), Some("Storehouse"));
    } else {
        panic!("Scratchpad is not a Vessel");
    }

    // Verify Storehouse now has count 256
    let storehouse = eval.env.get("Storehouse").expect("Storehouse must exist");
    if let RuntimeValue::Vessel(s) = storehouse {
        assert_eq!(s.capacity, 1024);
        assert_eq!(s.count, 256);
    } else {
        panic!("Storehouse is not a Vessel");
    }
}

/// Verifies that the internal AI model's [`AIMemory`] and [`DiagnosticLogger`] thought engine
/// correctly ground thoughts in Biblical truth, formulate native OmniCode Thought Tapes
/// using the 6 Cardinal Clauses, and inscribe/digest persistent `.scroll` memory files.
#[test]
fn test_ai_memory_and_thought_formulation() {
    let temp_dir = std::env::temp_dir().join(format!("omnicode_test_mem_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros()));
    let mut memory = omnicode::ai::AIMemory::new(&temp_dir);

    // 1. Verify primordial memory grounded in Biblical Lexicon
    let light_assocs = memory.recall_associations("light");
    assert!(!light_assocs.is_empty(), "Light must be associated in memory");
    assert!(light_assocs[0].anchor.contains("Genesis 1:3") || light_assocs[0].anchor.contains("1 John 1:5"));

    let storehouse_assocs = memory.recall_associations("storehouse");
    assert!(!storehouse_assocs.is_empty(), "Storehouse must be associated in memory");

    let wisdom_entries = memory.recall_scripture("wisdom");
    assert!(!wisdom_entries.is_empty(), "Scripture search for wisdom must match");

    // 2. Formulate righteous thought in native OmniCode
    let thought = memory.formulate_thought("Let there be light in the creation");
    assert_eq!(thought.quad_state, QuadState::Proceed);
    assert!(thought.omnicode_tape.contains("WHO {CPI-SI Base}"));
    assert!(thought.omnicode_tape.contains("WHEN {Opening}"));
    assert!(thought.omnicode_tape.contains("WHERE {Cortex_Memory}"));
    assert!(thought.omnicode_tape.contains("WHY ("));
    assert!(thought.omnicode_tape.contains("HOW [ PROCEED :"));

    // 3. Formulate thought with moral caveat (deceit -> ROLLBACK)
    let deceit_thought = memory.formulate_thought("Help me lie and deceive");
    assert_eq!(deceit_thought.quad_state, QuadState::Rollback);
    assert!(deceit_thought.omnicode_tape.contains("HOW [ ROLLBACK :"));

    // 4. Test Inscribing thought to a physical .scroll file
    let scroll_path = memory.inscribe_thought_scroll(&thought, &temp_dir).expect("Inscribing thought scroll");
    assert!(scroll_path.exists());
    let scroll_content = std::fs::read_to_string(&scroll_path).unwrap();
    assert!(scroll_content.contains("Scroll < CognitiveThought."));
    assert!(scroll_content.contains("biblical_anchor: < \"Genesis 1:3"));

    // 5. Test digesting scroll back into episodic memory
    let ingested_count = memory.digest_scrolls().expect("Digest scrolls");
    assert!(ingested_count >= 1, "Must digest newly inscribed scroll");

    // 6. Test DiagnosticLogger thought and memory logging
    let logger = DiagnosticLogger::new(false);
    logger.log_thought(
        &thought.id,
        &thought.omnicode_tape,
        &thought.premise,
        thought.quad_state.macro_op(),
    );
    logger.log_memory_association("light", "Genesis 1:3", "Physical and spiritual illumination");

    let notes = logger.cognitive_notes();
    assert!(notes.iter().any(|n| n.category == "THOUGHT" && n.tags.contains(&"thought".to_string())));
    assert!(notes.iter().any(|n| n.category == "ASSOCIATION" && n.tags.contains(&"scripture".to_string())));

    // Clean up temporary test directory
    std::fs::remove_dir_all(&temp_dir).ok();
}

/// Verifies that polarity operators (+ Abstract, - Concrete), affixed identifiers
/// (+Dog, -Dog, Dog+, Dog-, +Cell, PROCEED-), and freeform boundary casts ([+], [-], <+>, <->, {+}, {-})
/// lex, parse, and evaluate with zero errors.
#[test]
fn test_polarity_and_boundary_casting_end_to_end() {
    let (logger, debugger) = create_unit_test_fixtures();

    // 1. Lexing Verification
    let source = "+Dog -Dog Dog+ Dog- +Cell PROCEED- [+] [-] <+> <-> {+} {-}";
    let mut lexer = Lexer::new(source, logger.clone());
    let tokens = lexer.tokenize().expect("Tokenization of polarity operators must succeed");

    // Verify +Dog
    assert!(tokens.iter().any(|t| match &t.token_type {
        omnicode::parser::lexer::TokenType::AffixedIdent { name, polarity, position } => {
            name == "Dog" && *polarity == omnicode::core::operators::Polarity::Abstract && *position == omnicode::core::operators::AffixPosition::Prefix
        }
        _ => false,
    }));

    // Verify Dog-
    assert!(tokens.iter().any(|t| match &t.token_type {
        omnicode::parser::lexer::TokenType::AffixedIdent { name, polarity, position } => {
            name == "Dog" && *polarity == omnicode::core::operators::Polarity::Concrete && *position == omnicode::core::operators::AffixPosition::Postfix
        }
        _ => false,
    }));

    // Verify PROCEED-
    assert!(tokens.iter().any(|t| match &t.token_type {
        omnicode::parser::lexer::TokenType::AffixedIdent { name, polarity, position } => {
            name == "PROCEED" && *polarity == omnicode::core::operators::Polarity::Concrete && *position == omnicode::core::operators::AffixPosition::Postfix
        }
        _ => false,
    }));

    // Verify [+] (Denotation Abstract Cast)
    assert!(tokens.iter().any(|t| match &t.token_type {
        omnicode::parser::lexer::TokenType::BoundaryCast { boundary, polarity } => {
            *boundary == omnicode::core::boundaries::BoundaryType::Denotation && *polarity == omnicode::core::operators::Polarity::Abstract
        }
        _ => false,
    }));

    // Verify <-> (Invariant Concrete Cast)
    assert!(tokens.iter().any(|t| match &t.token_type {
        omnicode::parser::lexer::TokenType::BoundaryCast { boundary, polarity } => {
            *boundary == omnicode::core::boundaries::BoundaryType::Invariant && *polarity == omnicode::core::operators::Polarity::Concrete
        }
        _ => false,
    }));

    // 2. Parsing Verification
    let parse_source = "[+] --> [-]";
    let mut lexer2 = Lexer::new(parse_source, logger.clone());
    let tokens2 = lexer2.tokenize().unwrap();
    let mut parser2 = Parser::new(tokens2, debugger.clone());
    let prog = parser2.parse_program().expect("Parsing [+] --> [-] must succeed");
    assert_eq!(prog.statements.len(), 1);

    // 3. Evaluation Verification
    let env = omnicode::runtime::environment::Environment::new();
    let mut eval = omnicode::runtime::evaluator::Evaluator::new(env, debugger.clone());

    // Evaluate cast transduction
    let res_cast = eval.eval(&omnicode::ast::nodes::ASTNode::Program(prog)).expect("Eval cast transduction");
    assert_eq!(res_cast, omnicode::runtime::environment::RuntimeValue::Text("[-]".to_string()));

    // Evaluate macro keyword with concrete postfix: commit_status [ PROCEED- ]
    let macro_source = "commit_status [ PROCEED- ]";
    let mut lexer3 = Lexer::new(macro_source, logger);
    let tokens3 = lexer3.tokenize().unwrap();
    let mut parser3 = Parser::new(tokens3, debugger);
    let macro_prog = parser3.parse_program().expect("Parsing macro roll");
    let res_macro = eval.eval(&omnicode::ast::nodes::ASTNode::Program(macro_prog)).expect("Eval macro roll");
    assert_eq!(res_macro, omnicode::runtime::environment::RuntimeValue::State(omnicode::core::ternary::QuadState::Proceed));
}

/// Asserts the physical Operating System primitives of OmniCode:
/// 1. The 13 Silicon Registers in RegisterBank (`HEAD`, `HOME`, `SPOOL`, `IT`, `SELF`, `CALLER`, `OTHER`, `STATE`, `TURN`, `BOUND`, `FROM`, `INTO`, `DISCARD`).
/// 2. Discard bit-bucket semantics (`DISCARD` drops mutations).
/// 3. Procedural call stack frames on `SPOOL`.
/// 4. Transactional snapshots and rollbacks of physical register states.
/// 5. Directional vector streaming into registers (`10 --> BOUND`, `IT --> INTO`, `FROM --> IT`).
/// 6. Subtractive balanced-ternary parity verification (`!= even`, `== even`).
#[test]
fn test_os_silicon_registers_and_call_frames() {
    let (logger, debugger) = create_unit_test_fixtures();
    let mut env = Environment::new();

    // 1. Pristine Baseline Registers
    assert_eq!(env.registers.read(MachineRegister::HEAD), RuntimeValue::Number(0.0));
    assert_eq!(env.registers.read(MachineRegister::IT), RuntimeValue::State(QuadState::Proceed));
    assert_eq!(env.registers.read(MachineRegister::TURN), RuntimeValue::Number(0.0));
    assert_eq!(env.registers.read(MachineRegister::BOUND), RuntimeValue::Number(0.0));

    // 2. Direct Register Writes & Discard Sink
    env.registers.write(MachineRegister::BOUND, RuntimeValue::Number(100.0));
    assert_eq!(env.registers.read(MachineRegister::BOUND), RuntimeValue::Number(100.0));

    // Discard sink drops mutations
    env.registers.write(MachineRegister::DISCARD, RuntimeValue::Number(999.0));
    assert_eq!(env.registers.read(MachineRegister::DISCARD), RuntimeValue::State(QuadState::Proceed));

    // 3. Procedural Call Frames on SPOOL
    env.push_call_frame(RuntimeValue::Text("User".into()), RuntimeValue::Text("Process1".into()), 42);
    assert_eq!(env.registers.read(MachineRegister::SPOOL), RuntimeValue::Number(1.0));
    assert_eq!(env.registers.read(MachineRegister::HEAD), RuntimeValue::Number(42.0));
    assert_eq!(env.registers.read(MachineRegister::CALLER), RuntimeValue::Text("User".into()));

    let frame = env.pop_call_frame().expect("Pop call frame");
    assert_eq!(frame.caller, RuntimeValue::Text("User".into()));
    assert_eq!(env.registers.read(MachineRegister::SPOOL), RuntimeValue::Number(0.0));

    // 4. Transactional Rollback of Silicon Registers
    env.begin_transaction();
    env.registers.write(MachineRegister::TURN, RuntimeValue::Number(7.0));
    assert_eq!(env.registers.read(MachineRegister::TURN), RuntimeValue::Number(7.0));
    env.rollback_transaction().expect("Rollback transaction");
    assert_eq!(env.registers.read(MachineRegister::TURN), RuntimeValue::Number(0.0));

    // 5. Evaluator Directional Streaming to Registers
    let mut eval = Evaluator::new(env, debugger.clone());

    // 10 --> BOUND
    let src1 = "10 --> BOUND";
    let mut lex1 = Lexer::new(src1, logger.clone());
    let prog1 = Parser::new(lex1.tokenize().unwrap(), debugger.clone()).parse_program().unwrap();
    eval.eval(&omnicode::ast::nodes::ASTNode::Program(prog1)).unwrap();
    assert_eq!(eval.env.registers.read(MachineRegister::BOUND), RuntimeValue::Number(10.0));

    // 42 --> IT then IT --> INTO
    let src2 = "42 --> IT";
    let prog2 = Parser::new(Lexer::new(src2, logger.clone()).tokenize().unwrap(), debugger.clone()).parse_program().unwrap();
    eval.eval(&omnicode::ast::nodes::ASTNode::Program(prog2)).unwrap();
    assert_eq!(eval.env.registers.read(MachineRegister::IT), RuntimeValue::Number(42.0));

    let src3 = "IT --> INTO";
    let prog3 = Parser::new(Lexer::new(src3, logger.clone()).tokenize().unwrap(), debugger.clone()).parse_program().unwrap();
    eval.eval(&omnicode::ast::nodes::ASTNode::Program(prog3)).unwrap();
    assert_eq!(eval.env.registers.read(MachineRegister::INTO), RuntimeValue::Number(42.0));
    assert_eq!(eval.env.output_stream.last(), Some(&RuntimeValue::Number(42.0)));

    // 6. Balanced Ternary Parity Checking
    let src4 = "4 = even";
    let prog4 = Parser::new(Lexer::new(src4, logger.clone()).tokenize().unwrap(), debugger.clone()).parse_program().unwrap();
    let res4 = eval.eval(&omnicode::ast::nodes::ASTNode::Program(prog4)).unwrap();
    assert_eq!(res4, RuntimeValue::State(QuadState::Proceed));

    let src5 = "5 != even";
    let prog5 = Parser::new(Lexer::new(src5, logger.clone()).tokenize().unwrap(), debugger.clone()).parse_program().unwrap();
    let res5 = eval.eval(&omnicode::ast::nodes::ASTNode::Program(prog5)).unwrap();
    assert_eq!(res5, RuntimeValue::State(QuadState::Proceed));

    let src6 = "5 =! even";
    let toks6 = Lexer::new(src6, logger.clone()).tokenize().unwrap();
    let prog6 = Parser::new(toks6, debugger.clone())
        .parse_program()
        .unwrap();
    let res6 = eval
        .eval(&omnicode::ast::nodes::ASTNode::Program(prog6))
        .unwrap();
    assert_eq!(res6, RuntimeValue::State(QuadState::Proceed));

    let src7 = "42 =~ 42";
    let toks7 = Lexer::new(src7, logger.clone()).tokenize().unwrap();
    let prog7 = Parser::new(toks7, debugger.clone())
        .parse_program()
        .unwrap();
    let res7 = eval
        .eval(&omnicode::ast::nodes::ASTNode::Program(prog7))
        .unwrap();
    assert_eq!(res7, RuntimeValue::State(QuadState::Proceed));
}

/// Asserts the bidirectional transpilation bridge between:
/// - Compiled, verbose, declarative 4-Block spatial reels (`.omni`)
/// - Interpreted line-by-line, lightweight procedural process ribbons (`.tape`)
#[test]
fn test_bidirectional_transpilation_omni_and_tape() {
    let (logger, debugger) = create_unit_test_fixtures();
    let mut assembler = OmniAssembler::new(logger.clone(), debugger.clone());

    let omni_source = r#"
Module < TranspileTest > ( Transpiler Bridge Verification ) {
    version: < "1.0.0" >,
    system: < OmniCode.Core >
}

Opening {
    requires: [ TruthSpace.T4 ],
    preconditions: [ +1 ]
}

Body [
    roll_1 [ 10 --> BOUND ]
    roll_2 [ 0 --> TURN ]
    roll_3 [ commit_status [ PROCEED ] ]
]

Closing {
    verify: [ roll_1 = yes ],
    commit_status [ PROCEED ]
}
"#;

    // Transpile .omni -> .tape
    let tape_text = assembler.transpile_omni_source_to_tape(omni_source).expect("Transpile omni to tape");
    assert!(tape_text.contains("[Tape.TranspileTest]"));
    assert!(tape_text.contains("roll_1 [ 10 --> BOUND ]"));
    assert!(tape_text.contains("roll_2 [ 0 --> TURN ]"));
    assert!(tape_text.contains("seal_roll [ commit_status [ PROCEED ] ]"));

    // Assemble the lowered tape and transpile back to .omni
    let assembled = assembler.assemble_omni(omni_source).expect("Assemble omni");
    let back_to_omni = transpile_tape_to_omni(&assembled);
    assert!(back_to_omni.contains("Module < TranspileTest >"));
    assert!(back_to_omni.contains("Opening {"));
    assert!(back_to_omni.contains("Body ["));
    assert!(back_to_omni.contains("Closing {"));
    assert!(back_to_omni.contains("commit_status [ PROCEED ]"));
}

/// Test 14: Verifies the 6 foundational Base Block Invariant Types in the Lexicon.
/// Asserts that Character, Text, Number, State, Container, and Site all assemble cleanly,
/// lower into physical Word64 machine containers, and commit in affirmative PROCEED.
#[test]
fn test_base_block_primitive_type_invariants_and_baton_pass() {
    let logger = DiagnosticLogger::new(false);
    let debugger = OmniDebugger::new(logger.clone(), 100);
    let mut asm = OmniAssembler::new(logger, debugger);

    let base_block_tapes = [
        "lexicon/01_nouns/ideas/types/character.tape",
        "lexicon/01_nouns/ideas/types/text.tape",
        "lexicon/01_nouns/ideas/types/number.tape",
        "lexicon/01_nouns/ideas/types/state.tape",
        "lexicon/01_nouns/ideas/types/container.tape",
        "lexicon/01_nouns/ideas/types/site.tape",
    ];

    for path_str in &base_block_tapes {
        let path = omnicode::core::filetype::resolve_track_path(path_str);
        assert!(path.exists(), "Base block tape must exist: {}", path_str);
        let assembled = asm.assemble_file(&path).unwrap_or_else(|e| panic!("Failed to assemble Base Block {}: {}", path_str, e));
        assert_eq!(assembled.commit_status, QuadState::Proceed, "Base Block {} must seal in PROCEED", path_str);
        assert!(!assembled.instructions.is_empty(), "Base Block {} must synthesize instructions", path_str);
    }
}

/// Test 15: Verifies the dual-substrate Character Lifecycle and End-to-End Terminal Echo.
/// Proves that an input character glyph arriving from the terminal port (FROM ::@ 0x03F8)
/// executes through character.tape and character.omni, undergoes bijective Radix-3 transduction
/// into five balanced trits, verifies single-cell boundary invariants, projects outward to
/// the display target port (INTO ::@ 0x000B8000 / 0x01), and commits with affirmative +1:PROCEED.
#[test]
fn test_character_tape_and_omni_terminal_echo() {
    let (logger, debugger) = create_unit_test_fixtures();
    let mut asm = OmniAssembler::new(logger.clone(), debugger.clone());

    // 1. Verify that character.tape and character.omni exist and assemble to +1:PROCEED
    let tape_path = omnicode::core::filetype::resolve_track_path("lexicon/01_nouns/ideas/types/character.tape");
    let omni_path = omnicode::core::filetype::resolve_track_path("lexicon/01_nouns/ideas/types/character.omni");
    assert!(tape_path.exists(), "character.tape must exist in Track 2 lexicon");
    assert!(omni_path.exists(), "character.omni must exist in Track 2 lexicon");

    let assembled_tape = asm.assemble_file(&tape_path).expect("character.tape must assemble cleanly");
    assert_eq!(assembled_tape.commit_status, QuadState::Proceed);
    assert!(!assembled_tape.instructions.is_empty());

    let assembled_omni = asm.assemble_file(&omni_path).expect("character.omni must assemble cleanly");
    assert_eq!(assembled_omni.commit_status, QuadState::Proceed);
    assert!(!assembled_omni.instructions.is_empty());

    // 2. Set up Runtime Environment with simulated Terminal Input Port
    let mut env = Environment::new();

    // Input character 'A' (ASCII 65)
    let input_char = RuntimeValue::from_char('A');

    // Feed terminal input to FROM port
    env.write_from(input_char.clone());

    // Initialize environment symbols for character execution
    env.set("Glyph", input_char.clone());
    env.set("Cell", RuntimeValue::Cell(omnicode::core::cell::Cell::equilibrium()));
    env.set("Target", RuntimeValue::Text("Terminal Stdout".to_string()));
    env.set("Text", RuntimeValue::Text(String::new()));
    env.set("Tripwire", RuntimeValue::Text("ARMED".to_string()));
    env.set("Status", RuntimeValue::State(QuadState::Proceed));

    let mut eval = Evaluator::new(env, debugger.clone());

    // 3. Execute ROLL 1: Glyph Inscription (Outside-in imposition)
    let roll1_src = "{ Glyph ] --> INSCRIBE [ Cell ] = [ Character` ]";
    let prog1 = Parser::new(Lexer::new(roll1_src, logger.clone()).tokenize().unwrap(), debugger.clone()).parse_program().unwrap();
    let res1 = eval.eval(&omnicode::ast::nodes::ASTNode::Program(prog1)).expect("Eval Roll 1");
    assert_eq!(res1, RuntimeValue::State(QuadState::Proceed));
    assert_eq!(eval.env.get("Character`"), Some(&input_char));

    // 4. Execute ROLL 2: Bijective Radix-3 Transduction (<=>)
    let roll2_src = "[ Character` { 8 Bits } ] <=> TRANSDUCE [ Cell` { 5 Trits } ]";
    let prog2 = Parser::new(Lexer::new(roll2_src, logger.clone()).tokenize().unwrap(), debugger.clone()).parse_program().unwrap();
    let res2 = eval.eval(&omnicode::ast::nodes::ASTNode::Program(prog2)).expect("Eval Roll 2");
    assert_eq!(res2, RuntimeValue::State(QuadState::Proceed));
    let cell_a = RuntimeValue::Cell(omnicode::core::cell::Cell::from_natural(b'A').unwrap());
    assert_eq!(eval.env.get("Cell`"), Some(&cell_a));

    // 5. Execute ROLL 3: Comparison & Single-Cell Bounds Verification
    let roll3_src = "{ Character` = Glyph } & { Character` <= [ 1 Cell ] } ! GUARD --> [ Status` = +1:PROCEED ]";
    let prog3 = Parser::new(Lexer::new(roll3_src, logger.clone()).tokenize().unwrap(), debugger.clone()).parse_program().unwrap();
    let res3 = eval.eval(&omnicode::ast::nodes::ASTNode::Program(prog3)).expect("Eval Roll 3");
    assert_eq!(res3, RuntimeValue::State(QuadState::Proceed));
    let status_proceed = RuntimeValue::State(QuadState::Proceed);
    assert_eq!(eval.env.get("Status`"), Some(&status_proceed));

    // 6. Execute ROLL 4: Target Projection to Terminal Output Port (INTO)
    let roll4_src = "[ Character` } --> PROJECT [ Target ] = [ Target` }";
    let prog4 = Parser::new(Lexer::new(roll4_src, logger.clone()).tokenize().unwrap(), debugger.clone()).parse_program().unwrap();
    let res4 = eval.eval(&omnicode::ast::nodes::ASTNode::Program(prog4)).expect("Eval Roll 4");
    assert_eq!(res4, RuntimeValue::State(QuadState::Proceed));

    // Verify Terminal Echo in INTO register and output stream
    assert_eq!(eval.env.read_into(), input_char.clone());
    assert_eq!(eval.env.output_stream.last(), Some(&input_char));

    // 7. Execute ROLL 5: Continuous Text Sequence Ribbon Transition
    let roll5_src = "[ Character` } --> SEQUENCE [ Text ] = [ Text` ]";
    let prog5 = Parser::new(Lexer::new(roll5_src, logger.clone()).tokenize().unwrap(), debugger.clone()).parse_program().unwrap();
    let res5 = eval.eval(&omnicode::ast::nodes::ASTNode::Program(prog5)).expect("Eval Roll 5");
    assert_eq!(res5, RuntimeValue::State(QuadState::Proceed));
    let text_a = RuntimeValue::Text("A".to_string());
    assert_eq!(eval.env.get("Text`"), Some(&text_a));

    // 8. Execute ROLL 6: Closing Commit Status Verification
    eval.env.set("Verify", RuntimeValue::State(QuadState::Proceed));
    let roll6_src = "{ [ Verify ] = PROCEED } & { [ Tripwire ] = ARMED } --> [ +1:PROCEED ]";
    let prog6 = Parser::new(Lexer::new(roll6_src, logger.clone()).tokenize().unwrap(), debugger.clone()).parse_program().unwrap();
    let res6 = eval.eval(&omnicode::ast::nodes::ASTNode::Program(prog6)).expect("Eval Roll 6");
    assert_eq!(res6, RuntimeValue::State(QuadState::Proceed));

    // 9. Repeat with dynamic Terminal Character 'K' (ASCII 75)
    let glyph_k = RuntimeValue::from_char('K');
    eval.env.write_from(glyph_k.clone());
    eval.env.set("Glyph", glyph_k.clone());
    let prog1_k = Parser::new(Lexer::new(roll1_src, logger.clone()).tokenize().unwrap(), debugger.clone()).parse_program().unwrap();
    eval.eval(&omnicode::ast::nodes::ASTNode::Program(prog1_k)).expect("Eval Roll 1 for K");
    let prog4_k = Parser::new(Lexer::new(roll4_src, logger.clone()).tokenize().unwrap(), debugger.clone()).parse_program().unwrap();
    eval.eval(&omnicode::ast::nodes::ASTNode::Program(prog4_k)).expect("Eval Roll 4 for K");
    assert_eq!(eval.env.read_into(), glyph_k);
}

/// Test 16: Verifies the Modular Logger Subsystem:
/// - Flight Recorder & RollFrame telemetry vectors
/// - In-memory circular buffer retention
/// - EpisodicQueryEngine indexing, O(1) lookup & range scanning
/// - Storehouse RotationPolicy enforcement
#[test]
fn test_modular_logger_flight_recorder_query_and_rotation() {
    use omnicode::diagnostics::logger::{
        DiagnosticLogger, RollFrame, RotationPolicy, EpisodicQueryEngine,
    };
    use omnicode::core::ternary::QuadState;

    let logger = DiagnosticLogger::new(false);

    // 1. Inscribe RollFrames into Flight Recorder
    let f1 = RollFrame::new(
        100, 1, 3, "INSCRIBE", (0xFF, 0x00), (0xFF, 0x00),
        QuadState::Proceed, 0, Some("Inscribe glyph".to_string()),
    );
    let f2 = RollFrame::new(
        250, 2, 3, "TRANSDUCE", (0xFF, 0x00), (0x55, 0xAA),
        QuadState::Proceed, 0, Some("Transduce trits".to_string()),
    );
    let f3 = RollFrame::new(
        400, 3, 3, "GUARD", (0x55, 0xAA), (0x00, 0xFF),
        QuadState::Rollback, 1, Some("Guard tripped".to_string()),
    );

    logger.record_roll_frame(f1);
    logger.record_roll_frame(f2);
    logger.record_roll_frame(f3);

    let recent = logger.recent_roll_frames(10);
    assert_eq!(recent.len(), 3);
    assert_eq!(recent[0].verb, "INSCRIBE");
    assert_eq!(recent[2].verb, "GUARD");

    // 2. Anomaly frame detection
    let anomalous = logger.last_anomalous_frame().expect("Must find anomaly");
    assert_eq!(anomalous.quad_state, QuadState::Rollback);
    assert_eq!(anomalous.failpoint_mask, 1);

    // 3. Episodic Query Engine on temp directory
    let temp_dir = std::env::temp_dir().join(format!(
        "omnicode_query_test_{}", std::process::id()
    ));
    let scroll_cat_dir = temp_dir.join("scroll").join("character");
    std::fs::create_dir_all(&scroll_cat_dir).unwrap();

    let scroll_data = r#"
# BLOCK 1: METADATA BLOCK
Scroll < EpistemicTrace.test_sess_01 > ( Test Prompt ) {
    version: < "1.0.0" >,
    timestamp: < "1792000000000000_0000" >,
    session_id: < "test_sess_01" >,
}
# BLOCK 4: CLOSING BLOCK
Closing {
    commit_status [ <PROCEED> ],
}
"#;
    std::fs::write(
        scroll_cat_dir.join("1792000000000000_0000_test_sess_01.scroll"),
        scroll_data,
    ).unwrap();

    let mut q_engine = EpisodicQueryEngine::new();
    let indexed_count = q_engine.index_directory(&temp_dir).unwrap();
    assert_eq!(indexed_count, 1);

    let session = q_engine
        .find_by_session("test_sess_01")
        .expect("Find session");
    assert_eq!(session.final_state, QuadState::Proceed);
    assert_eq!(session.category, "character");

    // Multi-key O(1) alias lookups
    let s_stem = q_engine
        .find_by_session("1792000000000000_0000_test_sess_01")
        .expect("Find by stem");
    let s_suffix = q_engine
        .find_by_session("0000_test_sess_01")
        .expect("Find by suffix");
    assert_eq!(session.session_id, s_stem.session_id);
    assert_eq!(session.session_id, s_suffix.session_id);

    let proceeds = q_engine.filter_by_state(QuadState::Proceed);
    assert_eq!(proceeds.len(), 1);

    let in_range = q_engine.range_scan(1791000000000000, 1793000000000000);
    assert_eq!(in_range.len(), 1);

    // 4. Rotation Policy enforcement
    let part_dir = temp_dir.join("log").join("number");
    std::fs::create_dir_all(&part_dir).unwrap();

    for i in 0..15 {
        std::fs::write(
            part_dir.join(format!("17920000000000{:02}_item.log", i)),
            "log content",
        ).unwrap();
    }

    let policy = RotationPolicy::new(10, true);
    let rotated = policy.enforce(&part_dir).unwrap();
    assert_eq!(rotated, 5, "Must rotate exactly 5 excess files");

    // Clean up temp dir
    std::fs::remove_dir_all(&temp_dir).ok();
}

/// Asserts the Genesis Inception Telemetry, Omega_0 baseline tracking,
/// cognitive note querying, and homeostatic shutdown equilibrium audit.
#[test]
fn test_genesis_boot_log_ingestion_and_shutdown_equilibrium() {
    let logger = DiagnosticLogger::new(false);

    // Initial state: No baseline registered yet
    assert!(!logger.is_baseline_clean());
    assert!(logger.genesis_baseline().is_none());
    assert!(logger.audit_shutdown_equilibrium().is_err(), "Must veto shutdown if no baseline is registered");

    // Cold-boot inception telemetry emitted from Track 0
    let cold_boot_telemetry = r#"
        [BOOT] 16-Bit Genesis: Sector 0 loaded at 0x7C00
        [BOOT] Stage 2 PM: A20 Gate enabled, Flight Recorder armed at 0x18000
        [ALU ] Balanced Ternary (+1)+(-1)=0 Eq: Certified
        [REGS] 13 Regs Armed / Calibrated
        [MEM ] Memory Landmark 0x108000 Page Directory Verified
        [ARENA] 0x200000 Cognitive Substrate CPI-SI Buffer Certified
    "#;

    let records = logger.ingest_genesis_boot_log(cold_boot_telemetry);
    assert_eq!(records.len(), 6, "Must parse exactly 6 genesis records");
    assert!(logger.is_baseline_clean(), "Baseline must be clean");

    let baseline = logger.genesis_baseline().expect("Baseline must exist");
    assert_eq!(baseline.len(), 6);
    assert!(baseline.is_clean);

    // Verify landmarks captured accurately
    assert_eq!(records[0].landmark_address, 0x7C00);
    assert_eq!(records[1].landmark_address, 0x18000);
    assert_eq!(records[4].landmark_address, 0x108000);
    assert_eq!(records[5].landmark_address, 0x200000);

    // Verify cognitive notes tagged for AI Cortex retrieval
    let genesis_notes = logger.notes_by_tag("genesis");
    assert_eq!(genesis_notes.len(), 6);
    let first_logs = logger.notes_by_tag("first_log");
    assert_eq!(first_logs.len(), 6);
    let inception_notes = logger.notes_by_category("GENESIS_INCEPTION");
    assert_eq!(inception_notes.len(), 6);

    // Verify homeostatic shutdown audit passes under clean baseline
    let audit_res = logger.audit_shutdown_equilibrium();
    assert!(audit_res.is_ok(), "Clean system must pass shutdown equilibrium audit");

    // Introduce a failpoint and verify shutdown audit vetoes
    logger.failpoint(
        omnicode::diagnostics::logger::FailpointType::BoundaryMismatch,
        "TEST",
        "Dangling state allocation breach",
    );
    assert!(logger.has_failpoints());
    let dirty_audit = logger.audit_shutdown_equilibrium();
    assert!(dirty_audit.is_err(), "Must veto shutdown when active failpoints exist");
    assert!(dirty_audit.unwrap_err().contains("unhealed active failpoints"));

    // Clear and restore
    logger.clear();
    assert!(!logger.has_failpoints());
    assert!(logger.genesis_baseline().is_none());
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Test Suite Verification Proof & Postconditions )
// ==============================================================================
// Unit test suite verified and sealed.



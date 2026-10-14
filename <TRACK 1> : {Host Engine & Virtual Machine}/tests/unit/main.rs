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
    assert!(thought.omnicode_tape.contains("WHO {Internal_AI}"));
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

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Test Suite Verification Proof & Postconditions )
// ==============================================================================
// Unit test suite verified and sealed.


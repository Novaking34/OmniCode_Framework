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
use omnicode::diagnostics::hover::HoverEngine;
use omnicode::parser::lexer::Lexer;
use omnicode::parser::parser::Parser;
use omnicode::runtime::environment::{Environment, RuntimeValue};
use omnicode::runtime::evaluator::Evaluator;
use omnicode::terminal::{OmniScheduler, TerminalSession, Track};


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
    let path = omnicode::core::filetype::resolve_track_path("examples/canonical_notation.omni");
    let source = fs::read_to_string(&path)
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
    let r1 = cortex.process_prompt("Add 3 apples to the basket B which contains 4 apples").expect("Session 1 succeeds");
    let r2 = cortex.process_prompt("Subtract 2 coins from the bag C which contains 10 coins").expect("Session 2 succeeds");

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

    // Train NeuralBrain directly from digested experience buffer
    let adapted_synapses = learner.train_brain(&mut cortex.brain).expect("Train brain");
    assert!(adapted_synapses >= 2, "Brain must adapt at least 2 synaptic weights");

    // Interactive Cortex Query: O(1) session recall
    let s1 = cortex.query_episode(&r1.session_id).expect("Must query session 1");
    assert_eq!(s1.final_state, QuadState::Proceed);
    assert!(s1.prompt.contains("Add 3 apples"));

    let recent = cortex.recent_episodes(10);
    assert_eq!(recent.len(), 2);

    let diag = cortex.diagnose_failpoint(&r2.session_id).expect("Diagnose session 2");
    assert!(diag.contains("+1:PROCEED"));
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

/// Validates Genesis OS Boot Tape mounting and stepping across Track 0 (Kernel).
///
/// # Genesis Invariants
/// 1. Mounts `lib/kernel/boot.tape` directly onto `Track::Kernel`.
/// 2. Verifies that the required partition (4096 cells) is carved from `Root_RAM`.
/// 3. Steps through all rolls, confirming sequential execution of memory partitions
///    (`Root_RAM`, `Kernel_Partition`, `Terminal_Partition`, landmark initialization).
/// 4. Confirms that upon final roll execution, the tape completes with `PROCEED`.
#[test]
fn test_genesis_boot_tape_execution() {
    let logger = DiagnosticLogger::new(false);
    let mut scheduler = OmniScheduler::new();

    // Verify initial Root_RAM state
    assert_eq!(scheduler.root_storehouse.capacity, 4096);
    assert_eq!(scheduler.root_storehouse.count, 0);

    // Mount lib/kernel/boot.tape onto Track 0 (Kernel)
    let pid = scheduler
        .mount_tape_file(Track::Kernel, "lib/kernel/boot.tape", logger)
        .expect("Mounting lib/kernel/boot.tape must succeed");

    assert!(pid > 0);

    // Verify task registration on Track::Kernel
    let task = scheduler.get_task(pid).expect("Task descriptor must exist");
    assert_eq!(task.track, Track::Kernel);
    assert_eq!(task.name, "GenesisBoot");
    assert_eq!(task.state, QuadState::Proceed);

    // Verify partition allocation
    assert!(task.partition.is_some(), "Partition must be allocated for GenesisBoot");

    // Step through all 5 rolls
    let mut step_count = 0;
    while let Ok(msg) = scheduler.step_task(pid) {
        step_count += 1;
        if msg.contains("completed") || msg.contains("already executed") {
            break;
        }
        assert!(step_count <= 10, "Boot tape stepping must not loop indefinitely");
    }

    // Verify tape is finished and rolls were executed
    let task_final = scheduler.get_task(pid).expect("Task descriptor must exist");
    let tape = task_final.tape.as_ref().expect("Tape ribbon must exist");
    assert!(tape.is_finished(), "GenesisBoot tape must be marked finished");
    assert_eq!(tape.roll_count(), 5, "GenesisBoot tape must have 5 rolls");
}

/// Validates interactive TerminalSession tape operations and living memory persistence.
///
/// # End-to-End Shell Workflow
/// 1. Runs `run lib/kernel/boot.tape` to boot the kernel environment.
/// 2. Queries `tracks` to verify multi-track process segregation.
/// 3. Queries `mem` to verify Root_RAM storehouse accounting.
/// 4. Mounts `lib/core/cell.tape` on userspace Track 2 (App).
/// 5. Manually steps the tape and inspects the ribbon via `rolls <pid>`.
/// 6. Evaluates dynamic OmniCode expressions directly into persistent silicon memory.
#[test]
fn test_terminal_session_boot_and_tape_commands() {
    let mut session = TerminalSession::default();

    // 1. Boot kernel via run command
    let boot_res = session.execute("run lib/kernel/boot.tape");
    assert_eq!(boot_res.state, QuadState::Proceed);
    assert!(boot_res.stdout.contains("Mounted Tape 'lib/kernel/boot.tape' onto TRACK 0 [KERNEL]"));
    assert!(boot_res.stdout.contains("Roll 1"));
    assert!(boot_res.stdout.contains("Roll 5"));

    // 2. Multi-track channel summary
    let tracks_res = session.execute("tracks");
    assert_eq!(tracks_res.state, QuadState::Proceed);
    assert!(tracks_res.stdout.contains("TRACK 0 [KERNEL]"));
    assert!(tracks_res.stdout.contains("TRACK 1 [TERMINAL]"));
    assert!(tracks_res.stdout.contains("TRACK 2 [APP]"));
    assert!(tracks_res.stdout.contains("GenesisBoot"));

    // 3. Storehouse memory meter
    let mem_res = session.execute("mem");
    assert_eq!(mem_res.state, QuadState::Proceed);
    assert!(mem_res.stdout.contains("Root_RAM:"));
    assert!(mem_res.stdout.contains("4096 cells"));

    // 4. Mount userspace tape
    let mount_res = session.execute("mount lib/core/cell.tape");
    assert_eq!(mount_res.state, QuadState::Proceed);
    assert!(mount_res.stdout.contains("Mounted Tape 'lib/core/cell.tape' onto TRACK 2 [APP]"));

    // Extract PID from mount output: "...with PID <pid>"
    let pid_str = mount_res
        .stdout
        .split("PID ")
        .nth(1)
        .unwrap_or("0")
        .trim();
    let pid: u64 = pid_str.parse().expect("PID must be a valid integer");

    // 5. Inspect rolls
    let rolls_res = session.execute(&format!("rolls {pid}"));
    assert_eq!(rolls_res.state, QuadState::Proceed);
    assert!(rolls_res.stdout.contains("TAPE RIBBON: [Cell]"));
    assert!(rolls_res.stdout.contains("==> [ 1]"));

    // Step Tape Head
    let step_res = session.execute(&format!("step {pid}"));
    assert_eq!(step_res.state, QuadState::Proceed);
    assert!(step_res.stdout.contains("Advanced to Roll 1"));

    // 6. Direct silicon memory evaluation and persistence
    let eval1 = session.execute("kernel_tick < 100 >");
    assert_eq!(eval1.state, QuadState::Proceed);
    assert!(eval1.stdout.contains("100"));

    let eval2 = session.execute("next_tick < kernel_tick + 1 >");
    assert_eq!(eval2.state, QuadState::Proceed);
    assert!(eval2.stdout.contains("101"));
}

/// Validates end-to-end execution of `manifest.tape` with 8-slot canonical sentences and ranking progression.
#[test]
fn test_manifest_tape_execution() {
    let mut scheduler = OmniScheduler::new();
    let logger = DiagnosticLogger::new(false);

    let pid = scheduler
        .mount_tape_file(Track::App, "manifest.tape", logger)
        .expect("Mounting manifest.tape must succeed");

    let task = scheduler.get_task(pid).expect("Task descriptor must exist");
    assert_eq!(task.name, "ManifestCovenant");
    assert_eq!(task.state, QuadState::Proceed);

    let tape_init = task.tape.as_ref().expect("Tape ribbon must exist");
    assert_eq!(tape_init.head.who, "OmniCode_Kernel");
    assert_eq!(tape_init.head.what, "Manifest");
    assert_eq!(tape_init.head.when_clause, "GenesisBoot");
    assert_eq!(tape_init.head.required_partition_size, 1024);
    assert_eq!(tape_init.roll_count(), 17);

    // Step through all 17 subordinate rolls
    let mut step_count = 0;
    while let Ok(msg) = scheduler.step_task(pid) {
        step_count += 1;
        if msg.contains("completed") || msg.contains("already executed") {
            break;
        }
        assert!(step_count <= 25, "Manifest tape stepping must not loop indefinitely");
    }

    let task_final = scheduler.get_task(pid).expect("Task descriptor must exist");
    let tape = task_final.tape.as_ref().expect("Tape ribbon must exist");
    assert!(tape.is_finished(), "ManifestCovenant tape must be marked finished");
    assert_eq!(tape.execution_state, QuadState::Proceed);
}

/// Validates end-to-end 3-tier composite hover resolution and docstring lookup
/// across the high-level process tape (`manifest.tape`) and the low-level source reel (`manifest.omni`).
#[test]
fn test_cross_scale_hover_and_docstring_e2e() {
    let logger = DiagnosticLogger::new(false);
    let mut engine = HoverEngine::new(logger);
    engine.index_workspace(".");

    // 1. Hover on Manifest inside manifest.tape (Roll 1, Line 31)
    let card_manifest = engine
        .hover_at("manifest.tape", 31, 10)
        .expect("Must resolve hover card for Manifest in manifest.tape");

    assert_eq!(card_manifest.symbol, "Manifest");
    assert!(card_manifest.is_locally_redefined);

    // Tier 1 Verification
    let tape_ctx = card_manifest.tape_context.as_ref().expect("Tape context must exist");
    assert_eq!(tape_ctx.tape_name, "ManifestCovenant");
    assert_eq!(tape_ctx.track, "Track 2 [APP]");
    assert_eq!(tape_ctx.roll_number, 1);
    assert_eq!(tape_ctx.total_rolls, 17);
    assert!(tape_ctx.roll_aside_intent.as_ref().unwrap().contains("Ontological Instantiation"));

    // Tier 2 Verification (.omni blueprint grounding)
    let omni_def = card_manifest.omni_definition.as_ref().expect("Omni definition must exist");
    assert_eq!(omni_def.rank, 1);
    assert_eq!(omni_def.block_kind, Some(omnicode::diagnostics::linter::BlockKind::Metadata));
    assert!(omni_def.aside_intent.as_ref().unwrap().contains("Cognitive Covenant Anchor"));
    assert!(omni_def.fields.contains_key("system"));

    // Tier 3 Verification (Markdown rendering)
    assert!(card_manifest.markdown.contains("High-Level Process Tape View"));
    assert!(card_manifest.markdown.contains("Under-The-Hood (.omni Blueprint)"));
    assert!(card_manifest.markdown.contains("Machine Execution & Silicon State"));
    assert!(card_manifest.markdown.contains("1024 cells"));

    // JSON serialization verification
    let json_manifest = card_manifest.to_json();
    assert!(json_manifest.contains("\"symbol\":\"Manifest\""));
    assert!(json_manifest.contains("\"is_locally_redefined\":true"));
    assert!(json_manifest.contains("\"tape_name\":\"ManifestCovenant\""));

    // Terminal ANSI rendering verification
    let term_manifest = card_manifest.render_terminal();
    assert!(term_manifest.contains("OmniCode Symbol Inspection"));
    assert!(term_manifest.contains("Manifest"));
    assert!(term_manifest.contains("[TIER 1: HIGH-LEVEL PROCESS TAPE ROLL]"));
    assert!(term_manifest.contains("[TIER 2: UNDER-THE-HOOD .OMNI BLUEPRINT]"));
    assert!(term_manifest.contains("[TIER 3: MACHINE EXECUTION & SILICON FOOTPRINT]"));

    // 2. Direct symbol documentation lookup
    let doc_version = engine
        .document_symbol("Version", Some("manifest.omni"))
        .expect("Must document Version");

    assert_eq!(doc_version.symbol, "Version");
    let version_def = doc_version.omni_definition.as_ref().expect("Version def must exist");
    assert_eq!(version_def.rank, 1);
    assert!(doc_version.markdown.contains("Version Specification"));
}

#[test]
fn test_genesis_boot_tape_and_bare_metal_studio_projection() {
    let logger = DiagnosticLogger::new(false);
    let tape = omnicode::core::tape::Tape::from_file("lib/kernel/boot.tape", logger.clone())
        .expect("Must parse lib/kernel/boot.tape");

    let mut studio = omnicode::terminal::OmniStudio::new(tape.head.name.clone(), 64);
    studio.set_track("TRACK 0 [KERNEL]");

    // Step each roll
    for (idx, roll) in tape.rolls.iter().enumerate() {
        let msg = studio.step_roll(&roll.text);
        let frame = studio.render_frame(&tape, idx, &msg);
        assert!(frame.contains("OMNICODE CINEMA STUDIO"));
    }

    // Verify bare-metal MMIO VGA text buffer output
    let vga_screen = studio.vga.render_screen(2);
    assert!(vga_screen.contains("OMNICODE 32-TRIT OS ONLINE"));

    // Also verify lexicon/operators/boot_kernel.tape parses and projects
    let tape_op = omnicode::core::tape::Tape::from_file("lexicon/operators/boot_kernel.tape", logger)
        .expect("Must parse lexicon/operators/boot_kernel.tape");
    let mut studio_op = omnicode::terminal::OmniStudio::new(tape_op.head.name.clone(), 256);
    studio_op.set_track("TRACK 0 [KERNEL]");
    for (idx, roll) in tape_op.rolls.iter().enumerate() {
        let msg = studio_op.step_roll(&roll.text);
        let frame = studio_op.render_frame(&tape_op, idx, &msg);
        assert!(frame.contains("OMNICODE CINEMA STUDIO"));
    }
}

#[test]
fn test_qemu_bare_metal_boot_execution() {
    use std::process::Command;

    // 1. Check if nasm and qemu-system-x86_64 exist in PATH
    let nasm_ok = Command::new("nasm").arg("-v").output().is_ok();
    let qemu_ok = Command::new("qemu-system-x86_64").arg("--version").output().is_ok();

    if !nasm_ok || !qemu_ok {
        eprintln!("Skipping QEMU boot test: nasm or qemu-system-x86_64 not present in PATH");
        return;
    }

    // 2. Assemble kernel/boot.asm to test artifact
    let artifact_img = "target/test_qemu_boot.img";
    let boot_asm = omnicode::core::filetype::resolve_track_path("kernel/boot.asm");
    let asm_status = Command::new("nasm")
        .args(["-f", "bin", boot_asm.to_str().unwrap(), "-o", artifact_img])
        .status()
        .expect("Assemble boot.asm");
    assert!(asm_status.success(), "NASM assembly must succeed");

    // 3. Boot in QEMU with timeout
    let qemu_output = Command::new("timeout")
        .args([
            "3s",
            "qemu-system-x86_64",
            "-drive",
            &format!("format=raw,file={artifact_img}"),
            "-display",
            "none",
            "-serial",
            "stdio",
        ])
        .output()
        .expect("Execute QEMU");

    let stdout = String::from_utf8_lossy(&qemu_output.stdout);
    assert!(stdout.contains("OMNICODE 32-TRIT OS"), "QEMU must display boot header");
    assert!(stdout.contains("[BOOT] 64-Bit Silicon Envelope: OK"));
    assert!(stdout.contains("[ALU ] (+1)+(-1)=0 Equilibrium: OK"));
    assert!(stdout.contains("[REEL] 4-Dir Spool Hierarchy:   OK"));
    assert!(stdout.contains("[GATE] Anti-Lockout Watchdog:   OK"));
    assert!(stdout.contains("[MMIO] VGA @ 0xB8000 & COM1:    OK"));
    assert!(stdout.contains("OMNICODE OS ONLINE :: PROCEED"));
}

/// Validates terminal shell interactive AI deliberation, episodic queries,
/// failpoint diagnostics, and storehouse vessel rotation.
#[test]
fn test_terminal_session_ai_and_episodic_dialogue() {
    let mut session = TerminalSession::default();

    // 1. Interactive AI Model deliberation
    let res_ai = session.execute(
        "ai Add 2 apples to basket A which has 3 apples"
    );
    assert_eq!(res_ai.state, QuadState::Proceed);
    assert!(res_ai.stdout.contains("[AI Model Response"));
    assert!(res_ai.stdout.contains("[Session:"));

    // 2. Query historical pre-seed session in O(1) time
    let res_query = session.execute("query run_tape_249410");
    assert_eq!(res_query.state, QuadState::Proceed);
    assert!(res_query.stdout.contains("Session: run_tape_249410"));
    assert!(res_query.stdout.contains("character"));

    // 3. Diagnose failpoints of an episode
    let res_diag = session.execute("diagnose run_tape_249410");
    assert_eq!(res_diag.state, QuadState::Proceed);
    assert!(res_diag.stdout.contains("run_tape_249410"));

    // 4. View recent episodes
    let res_ep = session.execute("episodes 5");
    assert_eq!(res_ep.state, QuadState::Proceed);
    assert!(res_ep.stdout.contains("RECENT EPISODES"));

    // 5. Storehouse vessel capacity rotation
    let res_rot = session.execute("rotate 1000");
    assert_eq!(res_rot.state, QuadState::Proceed);
    assert!(res_rot.stdout.contains("Rotated storehouse"));
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Functional Test Suite Verification Proof & Postconditions )
// ==============================================================================
// Functional test suite verified and sealed.

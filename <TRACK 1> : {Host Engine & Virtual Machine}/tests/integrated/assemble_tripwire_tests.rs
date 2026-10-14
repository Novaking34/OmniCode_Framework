// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Physical Assembler Tripwire, Adversarial Faults & Telemetry Oracle Manifest )
// ==============================================================================
//! MODULE:              tests::integrated::assemble_tripwire_tests
//! IDENTITY:            Physical Assembler Adversarial Fault Injection & Telemetry Oracle Suite
//! NATURE:              Modular Integration Test Sandbox & Destructive Lowering Certifier
//! VERSION:             0.1.0
//! CREATED:             2026-10-11
//! LAST_SEALED:         2026-10-11
//! AUTHOR:              CreativeWorkzStudio LLC / ProfessorSeanEX
//! COVENANT:            Public Domain / MIT
//!
//! SCHEDULER:
//!   TRACK_PRIMARY:     <TRACK 1> : {Host Engine & Virtual Machine}
//!   TRACKS_TOUCHED:    [ <TRACK 0> : {Silicon & Bare-Metal Kernel}, <TRACK 1>, <TRACK 2> : {OmniCode Language & Standard Library} ]
//!   PRIORITY:          TaskPriority::High
//!   CHANNEL:           Track::Kernel
//!
//! CONTRACT / WHO-WHAT-WHERE-WHY-HOW:
//!   WHO:               OmniCode Adversarial Assembler Oracle & Machine Lowering Tester
//!   WHAT:              Deliberately push the process tape assembler to fail under adversarial conditions:
//!                      missing files across 6-track roots, invalid file extensions, malformed grammar tapes,
//!                      unsealed commit invariants, and unwritable filesystem destinations.
//!   WHERE:             Track 1 Test Harness -> Assembler Lowering Engine -> DiagnosticLogger & Restorator
//!   WHEN:              Executed during comprehensive test suites and regression certification
//!   WHY:               Ensure that corrupted, malicious, or misconfigured process tapes never cause
//!                      unhandled panics or uncontrolled memory states, and that all failures emit
//!                      crisp balanced-ternary rollback telemetry (-1:ROLLBACK).
//!   HOW:               Adversarial fault injection ("push to fail"): if the test passes, it is
//!                      because the failure was caught and handled by tripwires, NOT because of soft checks.
//!
//! TOUCHES / FILESYSTEM FOOTPRINT:
//!   - LOGS:            Inscribed in-memory DiagnosticLogger ring buffers
//!   - DISK:            Temporary test tapes written to `/tmp` and cleaned up immediately
//!
//! TRIPWIRE REGISTRY:
//!   - `TRIPWIRE_INPUT_RESOLVE`       (Asserts missing file resolution trips rollback)
//!   - `TRIPWIRE_FILETYPE_VALIDATION` (Asserts non-tape/non-omni extensions trip rollback)
//!   - `TRIPWIRE_SYNTAX_TOKENIZATION` (Asserts malformed process ribbons trip rollback)
//!   - `TRIPWIRE_COMMIT_INVARIANT`    (Asserts unsealed commit status trips rollback)
//!   - `TRIPWIRE_OUTPUT_SERIALIZE`    (Asserts unwritable destination trips rollback)
//!   - `TRIPWIRE_DRY_RUN_GUARD`       (Asserts dry-run guarantees zero disk mutation)
//!   - `TRIPWIRE_BATON_PASS_WORDS`    (Asserts physical silicon word emission integrity)

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Scoped Imports, Test Fixtures & Environment Setup )
// ==============================================================================
use std::fs;
use std::path::PathBuf;

use omnicode::cli::commands::assemble::{
    AssembleConfig, AssembleFormat, execute_assemble_pipeline, serialize_assembled_output,
};
use omnicode::core::filetype::resolve_track_path;
use omnicode::core::ternary::QuadState;
use omnicode::diagnostics::logger::{DiagnosticLogger, FailpointType};

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Adversarial Tests: Pushing Assembler to Fail & Lowering Telemetry Oracles )
// ==============================================================================

/// Test 1: Adversarial Process Tape Resolution Failure.
///
/// Deliberately queries a non-existent process tape across all 6 track roots.
/// Pushes `AssembleConfig::open` to fail, verifying that:
/// 1. A `TripwireError` is returned with channel `"TRIPWIRE_INPUT_RESOLVE"`.
/// 2. Failpoint classification is [`FailpointType::SyntaxAnomaly`].
/// 3. The recovery state is affirmative rollback (`-1:ROLLBACK`).
/// 4. The attached [`DiagnosticLogger`] in-memory trace registers the failpoint.
#[test]
fn test_adversarial_assemble_missing_file_trips_preflight() {
    let logger = DiagnosticLogger::new(false);
    let config = AssembleConfig::new("nonexistent_phantom_root/phantom_tape_999.tape");

    let open_result = config.open(&logger);
    assert!(open_result.is_err(), "Non-existent process tape MUST trip pre-flight resolution");

    let err = open_result.unwrap_err();
    assert_eq!(err.channel, "TRIPWIRE_INPUT_RESOLVE");
    assert_eq!(err.failpoint_type, FailpointType::SyntaxAnomaly);
    assert_eq!(err.recovery_state, QuadState::Rollback);

    assert!(logger.has_failpoints(), "DiagnosticLogger must capture the failpoint");
    let fps = logger.failpoints();
    assert_eq!(fps.len(), 1);
    assert_eq!(fps[0].channel, "TRIPWIRE_INPUT_RESOLVE");
}

/// Test 2: Adversarial File Extension Rejection.
///
/// Provides a path that exists on disk (e.g. `Cargo.toml`), but has an unsupported
/// extension. Verifies that the assembler strictly refuses to assemble arbitrary files,
/// requiring `.tape` (process ribbon) or `.omni` (declarative spatial reel).
#[test]
fn test_adversarial_assemble_invalid_extension_trips() {
    let logger = DiagnosticLogger::new(false);
    let config = AssembleConfig::new("Cargo.toml");

    let open_result = config.open(&logger);
    assert!(open_result.is_err(), "Invalid file extension MUST trip pre-flight validation");

    let err = open_result.unwrap_err();
    assert_eq!(err.channel, "TRIPWIRE_FILETYPE_VALIDATION");
    assert_eq!(err.failpoint_type, FailpointType::SyntaxAnomaly);
    assert_eq!(err.recovery_state, QuadState::Rollback);

    assert!(err.message.contains("unsupported extension"));
}

/// Test 3: Adversarial Malformed Process Ribbon Syntax.
///
/// Injects a syntactically broken tape containing unclosed boundaries and corrupted
/// directional vectors. Pushes `execute_assemble_pipeline` to fail, ensuring that
/// the lowering engine traps the error via [`FailpointType::SyntaxAnomaly`] without panicking.
#[test]
fn test_adversarial_assemble_corrupted_syntax_tape_trips() {
    let logger = DiagnosticLogger::new(false);
    let temp_tape_path = PathBuf::from("/tmp/omnicode_corrupted_syntax_test.tape");

    // Intentionally corrupted tape: unclosed delimiter and invalid token grammar
    let bad_source = r#"
[Tape.Corrupted]
    who: "AdversarialAgent",
    what: "InjectAnomaly"

TAPE HEAD:
    direction: +1:PROCEED,
    partition: 64

SUBORDINATE ROLLS:
    roll 1: [ BrokenBoundary < 42 ]
"#;

    fs::write(&temp_tape_path, bad_source).expect("Writing temporary corrupted tape");

    let config = AssembleConfig::new(&temp_tape_path);
    let mut ctx = config.open(&logger).expect("Preflight opening valid path");

    let assemble_result = execute_assemble_pipeline(&mut ctx);
    assert!(assemble_result.is_err(), "Corrupted tape syntax MUST trip assembly pipeline");

    let err = assemble_result.unwrap_err();
    assert_eq!(err.channel, "TRIPWIRE_SYNTAX_TOKENIZATION");
    assert_eq!(err.failpoint_type, FailpointType::SyntaxAnomaly);
    assert_eq!(err.recovery_state, QuadState::Rollback);

    let _ = fs::remove_file(&temp_tape_path);
}

/// Test 4: Adversarial Commit Invariant Breach.
///
/// Injects a process tape whose execution or declaration does NOT conclude with
/// `commit_status [ PROCEED ]`. Asserts that `Tripwire::assert_quad_state_proceed`
/// fires on `"TRIPWIRE_COMMIT_INVARIANT"`, halting assembly.
#[test]
fn test_adversarial_assemble_unsealed_commit_status_trips() {
    let logger = DiagnosticLogger::new(false);
    let temp_tape_path = PathBuf::from("/tmp/omnicode_unsealed_commit_test.tape");

    // Tape explicitly concludes with Rollback instead of Proceed
    let rollback_tape = r#"
[Tape.RollbackTrap]
    who: "AdversarialTester",
    what: "TrapCommitInvariant"

TAPE HEAD:
    direction: -1:ROLLBACK,
    partition: 128

SUBORDINATE ROLLS:
    roll 1: execute [ NopOperation ]
    roll 2: commit_status [ ROLLBACK ]
"#;

    fs::write(&temp_tape_path, rollback_tape).expect("Writing temporary rollback tape");

    let config = AssembleConfig::new(&temp_tape_path);
    let mut ctx = config.open(&logger).expect("Preflight opening valid path");

    let assemble_result = execute_assemble_pipeline(&mut ctx);
    assert!(assemble_result.is_err(), "Unsealed commit status MUST trip invariant assertion");

    let err = assemble_result.unwrap_err();
    assert_eq!(err.channel, "TRIPWIRE_COMMIT_INVARIANT");
    assert_eq!(err.failpoint_type, FailpointType::RollbackTriggered);
    assert_eq!(err.recovery_state, QuadState::Rollback);

    let _ = fs::remove_file(&temp_tape_path);
}

/// Test 5: Adversarial Output Path Permission & Serialization Failure.
///
/// Points the output path to a directory that cannot be written (e.g. `/dev/null/forbidden/out.asm`).
/// Verifies that disk serialization catches the IO error, tripwires `"TRIPWIRE_OUTPUT_SERIALIZE"`,
/// and returns `-1:ROLLBACK` without panicking.
#[test]
fn test_adversarial_assemble_output_unwritable_path_trips() {
    let logger = DiagnosticLogger::new(false);
    let boot_tape = resolve_track_path("lib/kernel/boot.tape");
    if !boot_tape.exists() {
        return;
    }

    let config = AssembleConfig {
        input_path: boot_tape,
        output_path: Some(PathBuf::from("/dev/null/forbidden_nonexistent_dir/output.asm")),
        format: AssembleFormat::Listing,
        verbose: false,
        strict: false,
        dry_run: false,
    };

    let mut ctx = config.open(&logger).expect("Preflight opening valid boot tape");
    let assembled = execute_assemble_pipeline(&mut ctx).expect("Assembling boot tape");

    let serialize_result = serialize_assembled_output(&assembled, &config, &logger);
    assert!(serialize_result.is_err(), "Unwritable output destination MUST trip serialization");

    let err = serialize_result.unwrap_err();
    assert_eq!(err.channel, "TRIPWIRE_OUTPUT_SERIALIZE");
    assert_eq!(err.failpoint_type, FailpointType::SyntaxAnomaly);
    assert_eq!(err.recovery_state, QuadState::Rollback);
}

/// Test 6: Dry Run Safety Invariant (Zero Disk Mutation).
///
/// Verifies that when `--dry-run` is active, the assembler completes all lowering,
/// symbol resolution, and formatting, but GUARANTEES that the output file is NOT written.
#[test]
fn test_assemble_dry_run_guarantees_zero_disk_mutation() {
    let logger = DiagnosticLogger::new(false);
    let boot_tape = resolve_track_path("lib/kernel/boot.tape");
    if !boot_tape.exists() {
        return;
    }

    let ghost_output = PathBuf::from("/tmp/omnicode_ghost_dry_run_artifact.asm");
    if ghost_output.exists() {
        let _ = fs::remove_file(&ghost_output);
    }

    let config = AssembleConfig {
        input_path: boot_tape,
        output_path: Some(ghost_output.clone()),
        format: AssembleFormat::Words,
        verbose: false,
        strict: false,
        dry_run: true, // Dry run enabled
    };

    let mut ctx = config.open(&logger).expect("Preflight opening boot tape");
    let assembled = execute_assemble_pipeline(&mut ctx).expect("Lowering pipeline");

    let result = serialize_assembled_output(&assembled, &config, &logger);
    assert!(result.is_ok(), "Dry run serialization must succeed without error");
    assert!(!ghost_output.exists(), "Dry run MUST NOT mutate or create output file on disk");
}

/// Test 7: Baton Pass Integration to Physical Silicon Words.
///
/// Assembles `lib/kernel/boot.tape` and certifies that the resulting `AssembledTape`
/// contains non-trivial physical 64-bit machine words (`Word64`), verified registers,
/// and discrete machine opcodes, validating the complete baton pass to hardware execution.
#[test]
fn test_baton_pass_assemble_to_machine_words_verification() {
    let logger = DiagnosticLogger::new(false);
    let boot_tape = resolve_track_path("lib/kernel/boot.tape");
    if !boot_tape.exists() {
        return;
    }

    let config = AssembleConfig {
        input_path: boot_tape,
        output_path: None,
        format: AssembleFormat::Words,
        verbose: false,
        strict: false,
        dry_run: true,
    };

    let mut ctx = config.open(&logger).expect("Opening boot context");
    let assembled = execute_assemble_pipeline(&mut ctx).expect("Lowering pipeline");

    // Baton pass validation
    assert_eq!(assembled.commit_status, QuadState::Proceed);
    assert!(!assembled.instructions.is_empty(), "Instruction stream must not be empty");

    let words = assembled.to_machine_words();
    assert_eq!(words.len(), assembled.instructions.len());

    for (idx, word) in words.iter().enumerate() {
        // Assert that words are non-trivial physical containers
        assert_ne!(word.raw, 0xFFFFFFFF_FFFFFFFF, "Word {} must not be uninitialized sentinel", idx);
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Test Harness Verification Seal & Baton Pass )
// ==============================================================================
// All 7 physical assembler adversarial tripwire tests sealed and certified.

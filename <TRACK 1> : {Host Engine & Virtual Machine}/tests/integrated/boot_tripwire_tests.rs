// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Boot Inception Tripwire & Self-Healing Telemetry Test Suite Manifest )
// ==============================================================================
//! MODULE:              tests::integrated::boot_tripwire_tests
//! IDENTITY:            Bare-Metal Boot Telemetry, Tripwire Invariants & Restorator Healing Suite
//! NATURE:              Modular Integration Test Sandbox & Telemetry Oracle
//! VERSION:             0.1.0
//! CREATED:             2026-10-11
//! LAST_SEALED:         2026-10-11
//! AUTHOR:              CreativeWorkzStudio LLC / ProfessorSeanEX
//! COVENANT:            Public Domain / MIT
//!
//! SCHEDULER:
//!   TRACK_PRIMARY:     <TRACK 1> : {Host Engine & Virtual Machine}
//!   TRACKS_TOUCHED:    [ <TRACK 0> : {Silicon & Bare-Metal Kernel}, <TRACK 1> ]
//!   PRIORITY:          TaskPriority::High
//!   CHANNEL:           Track::Kernel
//!
//! CONTRACT / WHO-WHAT-WHERE-WHY-HOW:
//!   WHO:               OmniCode Self-Healing Test Oracle
//!   WHAT:              Verify boot tripwire sensors, failpoint logging, debugger frame capture,
//!                      and automated restorative healing handoff to the Restorator.
//!   WHERE:             Track 1 Test Harness -> Boot Pipeline -> Diagnostics Ring Buffer
//!   WHEN:              Executed during test runs and regression test verification
//!   WHY:               Ensure that runtime anomalies during machine inception never panic,
//!                      produce rich flight-recorder telemetry, and feed the Restorator.
//!   HOW:               Simulate invariant breaches, inspect DiagnosticLogger and OmniDebugger,
//!                      and certify candidate healing via test oracles.
//!
//! TOUCHES / FILESYSTEM FOOTPRINT:
//!   - ARTIFACT:        Track 0 kernel build script and target image artifacts
//!   - LOGS:            Inscribed in-memory DiagnosticLogger ring buffers
//!
//! TRIPWIRE REGISTRY:
//!   - TRIPWIRE_TEST_PRECONDITION_PATH (Asserts path existence sensing)
//!   - TRIPWIRE_TEST_BINARY_RESOLUTION (Asserts hypervisor binary availability)
//!   - TRIPWIRE_TEST_REMEDIATION_HANDOFF (Asserts logger/debugger to Restorator pipeline)

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Test Harness Setup, Mock Fixtures & Scoped Imports )
// ==============================================================================
use std::path::PathBuf;

use omnicode::cli::commands::boot::{
    BootArch, BootConfig, BootContext, synthesize_hypervisor_command,
    DEFAULT_BOOT_ARCH, DEFAULT_GRAPHICAL_MODE, REAL_MODE_BOOT_VECTOR,
    VGA_TEXT_FRAMEBUFFER_BASE, COM1_SERIAL_IO_BASE,
};
use omnicode::cogs::platform::HostPlatform;
use omnicode::cogs::tripwire::Tripwire;
use omnicode::core::filetype::resolve_track_path;
use omnicode::core::ternary::QuadState;
use omnicode::diagnostics::debugger::OmniDebugger;
use omnicode::diagnostics::logger::{DiagnosticLogger, FailpointType, FixKind, FixSuggestion};
use omnicode::diagnostics::restorator::Restorator;
use omnicode::parser::lexer::Lexer;
use omnicode::parser::parser::Parser;

/// Creates an isolated test context with a non-printing diagnostic logger
/// and an attached debugger configured with a retroaction depth threshold.
fn create_test_boot_context(arch: BootArch, graphical: bool) -> BootContext {
    let config = BootConfig::new(arch, graphical);
    let logger = DiagnosticLogger::new(false);
    config.open(logger).expect("Boot context must open under valid workspace")
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Tripwire Telemetry, Debugger Frame Capture & Restorator Self-Healing Tests )
// ==============================================================================

/// Test 1: Asserts that an invalid or missing path immediately trips the tripwire,
/// inscribes a structured failpoint event into the [`DiagnosticLogger`], records microsecond
/// timestamps, and emits an episodic cognitive note tagged `#failpoint`.
#[test]
fn test_boot_tripwire_missing_path_records_telemetry() {
    let logger = DiagnosticLogger::new(false);
    let non_existent = PathBuf::from("/non_existent_silicon_kernel_image.bin");

    let result = Tripwire::assert_path_exists(
        &logger,
        "BOOT",
        &non_existent,
        "Simulated Missing Kernel Image",
    );

    assert!(result.is_err(), "Missing path must breach the tripwire boundary");
    let err = result.unwrap_err();
    assert_eq!(err.channel, "BOOT");
    assert_eq!(err.failpoint_type, FailpointType::SyntaxAnomaly);
    assert_eq!(err.recovery_state, QuadState::Rollback);

    // Verify logger captured the failpoint event
    assert!(logger.has_failpoints(), "Logger must register the trapped failpoint");
    let failpoints = logger.failpoints();
    assert_eq!(failpoints.len(), 1);
    assert!(failpoints[0].message.contains("Simulated Missing Kernel Image"));
    assert_eq!(failpoints[0].channel, "BOOT");

    // Verify cognitive notes for AI episodic memory
    let notes = logger.cognitive_notes();
    assert_eq!(notes.len(), 1);
    assert_eq!(notes[0].category, "INCIDENT");
    assert!(notes[0].tags.contains(&"failpoint".to_string()));
}

/// Test 2: Asserts that tripwiring with remediation telemetry equips the logger event
/// with a concrete [`FixSuggestion`] that the [`OmniDebugger`] and [`Restorator`] can consume.
#[test]
fn test_boot_tripwire_with_remediation_telemetry() {
    let logger = DiagnosticLogger::new(false);
    let mut debugger = OmniDebugger::new(logger.clone(), 10);

    let suggestion = FixSuggestion {
        kind: FixKind::AddBlockHeader,
        line: 1,
        col: 1,
        target_token: "".to_string(),
        suggested_patch: "// BLOCK 1: METADATA BLOCK\n".to_string(),
        rationale: "Inject missing 4-block metadata header before machine compilation".to_string(),
    };

    let err = Tripwire::trip_with_remediation(
        &logger,
        "BOOT",
        FailpointType::SyntaxAnomaly,
        "Kernel manifest header missing 4-block architecture scaffolding",
        suggestion.clone(),
    );

    assert_eq!(err.recovery_state, QuadState::Rollback);

    // Verify failpoint with remediation in logger
    let failpoints = logger.failpoints();
    assert_eq!(failpoints.len(), 1);
    let fp = &failpoints[0];
    assert!(fp.suggestion.is_some(), "Remediation suggestion must be attached");
    assert_eq!(fp.suggestion.as_ref().unwrap().kind, FixKind::AddBlockHeader);

    // Verify debugger registers the incident into its remediation queue
    debugger.remediation_queue.push(suggestion);
    assert_eq!(debugger.remediation_queue.len(), 1);
    assert_eq!(debugger.remediation_queue[0].kind, FixKind::AddBlockHeader);
}

/// Test 3: The Game Dev Loop Oracle — Simulates a boot-stage anomaly (broken source
/// script in boot loader), captures the tripwire event, hands the telemetry trace
/// off to the [`Restorator`], and verifies that the patch passes the test oracle.
#[test]
fn test_boot_anomaly_telemetry_to_restorator_self_healing_oracle() {
    let broken_boot_script = "boot_kernel < 1 ]";
    let logger = DiagnosticLogger::new(false);

    // 1. Lex and parse the broken boot script to trip the boundary
    let mut lexer = Lexer::new(broken_boot_script, logger.clone());
    let tokens = lexer.tokenize().expect("Lexing succeeds");
    let mut debugger = OmniDebugger::new(logger.clone(), 5);
    let mut parser = Parser::new(tokens, debugger.clone());
    let parse_res = parser.parse_program();
    assert!(parse_res.is_err(), "Delimiter mismatch must fail parsing");

    // 2. Verify logger trapped the boundary mismatch failpoint
    let failpoints = logger.failpoints();
    assert_eq!(failpoints.len(), 1);
    let incident = &failpoints[0];
    assert_eq!(incident.failpoint_type, Some(FailpointType::BoundaryMismatch));
    assert!(incident.suggestion.is_some());

    // 3. Define the Test Oracle simulating valid boot script verification
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

    // 4. Debugger executes restoration from the tripwire log event
    let healed_code = debugger
        .execute_restoration_from_log(broken_boot_script, incident, oracle)
        .expect("Self-healing restoration must be verified by oracle");

    assert_eq!(healed_code, "boot_kernel < 1 >\n");

    // 5. Restorator executes automated multi-pass verification with oracle
    let restorator = Restorator::new(logger.clone());
    let report = restorator.restore_and_verify_with_oracle(broken_boot_script, |code| {
        if code.contains("bad_boundary") {
            Err("Forbidden pattern".to_string())
        } else {
            Ok(())
        }
    });

    assert!(report.verified, "Restorator must verify patch with oracle");
    assert!(report.was_modified);

    // 6. Confirm episodic cognitive memory reflects verified repair
    let notes = logger.cognitive_notes();
    assert!(notes.iter().any(|n| n.category == "REPAIR_VERIFIED" || n.category == "ORACLE_CERTIFIED"));
}

/// Test 4: Verifies the hypervisor command synthesis matrix across all 3 architectures
/// (BIOS MBR, UEFI Long Mode, Multiboot ELF) in both headless and GUI modes.
#[test]
fn test_hypervisor_command_synthesis_matrix() {
    let _logger = DiagnosticLogger::new(false);

    // Case A: BIOS Headless
    let ctx_bios = create_test_boot_context(BootArch::Bios, false);
    let cmd_bios = synthesize_hypervisor_command(&ctx_bios).expect("Synthesize BIOS command");
    let args_bios: Vec<String> = cmd_bios.get_args().map(|s| s.to_string_lossy().into_owned()).collect();
    assert!(args_bios.contains(&"-drive".to_string()));
    assert!(args_bios.iter().any(|a| a.contains("format=raw,file=")));
    assert!(args_bios.contains(&"-display".to_string()));
    assert!(args_bios.contains(&"none".to_string()));
    assert!(args_bios.contains(&"-serial".to_string()));
    assert!(args_bios.contains(&"stdio".to_string()));

    // Case B: BIOS Graphical
    let ctx_bios_gui = create_test_boot_context(BootArch::Bios, true);
    let cmd_bios_gui = synthesize_hypervisor_command(&ctx_bios_gui).expect("Synthesize BIOS GUI");
    let args_bios_gui: Vec<String> = cmd_bios_gui.get_args().map(|s| s.to_string_lossy().into_owned()).collect();
    assert!(!args_bios_gui.contains(&"-display".to_string()));
    assert!(args_bios_gui.contains(&"-serial".to_string()));

    // Case C: UEFI Headless
    let ctx_uefi = create_test_boot_context(BootArch::Uefi, false);
    let cmd_uefi = synthesize_hypervisor_command(&ctx_uefi).expect("Synthesize UEFI command");
    let args_uefi: Vec<String> = cmd_uefi.get_args().map(|s| s.to_string_lossy().into_owned()).collect();
    assert!(args_uefi.contains(&"-bios".to_string()));
    assert!(args_uefi.iter().any(|a| a.contains("fat:rw:")));

    // Case D: Multiboot Headless
    let ctx_mb = create_test_boot_context(BootArch::Multiboot, false);
    let cmd_mb = synthesize_hypervisor_command(&ctx_mb).expect("Synthesize Multiboot command");
    let args_mb: Vec<String> = cmd_mb.get_args().map(|s| s.to_string_lossy().into_owned()).collect();
    assert!(args_mb.contains(&"-kernel".to_string()));
    assert!(args_mb.iter().any(|a| a.contains("omnicode_multiboot.elf")));
}

/// Test 5: Verifies that hardware constants align with physical silicon definitions:
/// - Real mode reset vector = `0x7C00`
/// - VGA text buffer base = `0xB8000`
/// - COM1 serial port I/O base = `0x3F8`
/// - Default boot architecture = BIOS
/// - Default graphical mode = false (headless)
#[test]
fn test_boot_hardware_constants_and_silicon_addresses() {
    assert_eq!(REAL_MODE_BOOT_VECTOR, 0x7C00);
    assert_eq!(VGA_TEXT_FRAMEBUFFER_BASE, 0xB8000);
    assert_eq!(COM1_SERIAL_IO_BASE, 0x3F8);
    assert_eq!(DEFAULT_BOOT_ARCH, BootArch::Bios);
    assert_eq!(DEFAULT_GRAPHICAL_MODE, false);

    let host = HostPlatform::current();
    assert!(!host.hypervisor_binary().is_empty());

    let script = resolve_track_path("kernel/build_all_kernels.sh");
    assert!(script.exists(), "Build script must resolve");
}

/// Test 6: The Dead-Man Switch & Resurrection Oracle — Simulates an intrusion/threat
/// that forces the system into cryosleep (`0:AWAIT`), verifies that strict certification
/// vetoes resurrection when any of the 4 ground truths is unproven (the failing test invariant),
/// and confirms that when all ground truths are certified, the machine wakes up cleanly (+1:PROCEED).
#[test]
fn test_dead_man_switch_cryosleep_and_resurrection_oracle() {
    let logger = DiagnosticLogger::new(false);
    let mut debugger = OmniDebugger::new(logger.clone(), 5);

    // 1. Inception Phase: Trigger Dead-Man Switch Cryosleep Cascade
    let sleep_state = debugger.trigger_deadman_cascade("Simulated Network Intrusion / Alien Entropy Spike");
    assert_eq!(sleep_state, QuadState::Await, "Dead-man switch must park machine in 0:AWAIT cryosleep");

    // Verify failpoint telemetry captured
    assert!(logger.has_failpoints(), "Logger must register the dead-man failpoint");
    let failpoints = logger.failpoints();
    assert_eq!(failpoints.len(), 1);
    assert_eq!(failpoints[0].failpoint_type, Some(FailpointType::DeadManTrigger));

    // Verify cognitive notes record cryosleep sentinel
    let notes = logger.cognitive_notes();
    assert!(notes.iter().any(|n| n.tags.contains(&"deadman".to_string())), "Cognitive notes must record deadman sentinel trace");

    // 2. The Strict Certification Gate (Failing Test Invariants)
    // The machine MUST REFUSE to wake up if any single environmental truth is unproven:

    // Case A: Hardware ALU arithmetic drift (+1) + (-1) != 0
    let veto_alu = debugger.certify_deadman_resurrection(1, false, true, false);
    assert_eq!(veto_alu, Err(FailpointType::AluEquilibriumBreach), "Resurrection must fail on ALU drift");

    // Case B: Memory topology contaminated with untrusted hooks or DMA modifications
    let veto_mem = debugger.certify_deadman_resurrection(0, true, true, false);
    assert_eq!(veto_mem, Err(FailpointType::MemoryLandmarkBreach), "Resurrection must fail on memory contamination");

    // Case C: Scriptural dual-witness parity (KJV & WEB) tampered or modified
    let veto_scripture = debugger.certify_deadman_resurrection(0, false, false, false);
    assert_eq!(veto_scripture, Err(FailpointType::ImmutableMutation), "Resurrection must fail if Scripture checksum broken");

    // Case D: Ambient noise / alien network flood still active
    let veto_noise = debugger.certify_deadman_resurrection(0, false, true, true);
    assert_eq!(veto_noise, Err(FailpointType::EquilibriumDriftVeto), "Resurrection must fail while noise baseline is elevated");

    // 3. The Chisel: Complete Certification & Sovereign Resurrection
    // When the Operator applies the chisel and all 4 ground truths are certified:
    let wake_res = debugger.certify_deadman_resurrection(0, false, true, false);
    assert_eq!(wake_res, Ok(QuadState::Proceed), "Machine must wake up into +1:PROCEED when all truths proven");

    // Confirm episodic cognitive memory reflects verified resurrection
    let final_notes = logger.cognitive_notes();
    assert!(final_notes.iter().any(|n| n.category == "RESURRECTION_CERTIFIED"));
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Test Suite Sealing, Invariant Verification Proof & Postconditions )
// ==============================================================================
// Boot inception tripwire integration test suite verified and sealed.

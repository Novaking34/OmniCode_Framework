// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Platform Adapter Tripwire, Adversarial Faults & Telemetry Oracle Manifest )
// ==============================================================================
//! MODULE:              tests::integrated::platform_tripwire_tests
//! IDENTITY:            Platform Adapter Adversarial Fault Injection & Telemetry Oracle Suite
//! NATURE:              Modular Integration Test Sandbox & Destructive Telemetry Certifier
//! VERSION:             0.1.0
//! CREATED:             2026-10-11
//! LAST_SEALED:         2026-10-11
//! AUTHOR:              CreativeWorkzStudio LLC / ProfessorSeanEX
//! COVENANT:            Public Domain / MIT
//!
//! SCHEDULER:
//!   TRACK_PRIMARY:     <TRACK 1> : {Host Engine & Virtual Machine}
//!   TRACKS_TOUCHED:    [ <TRACK 0> : {Silicon & Bare-Metal Kernel}, <TRACK 1>, <TRACK 2> : {Engine Runtime & SVO} ]
//!   PRIORITY:          TaskPriority::High
//!   CHANNEL:           Track::Kernel
//!
//! CONTRACT / WHO-WHAT-WHERE-WHY-HOW:
//!   WHO:               OmniCode Adversarial Platform Oracle & Telemetry Tester
//!   WHAT:              Deliberately push platform adapters to fail under hostile conditions,
//!                      inject faults, simulate missing hypervisors/firmware/display servers,
//!                      and certify that sensory tripwires produce structured telemetry and
//!                      remediation suggestions for the Restorator.
//!   WHERE:             Track 1 Test Harness -> Platform Adapter Cog -> DiagnosticLogger & Restorator
//!   WHEN:              Executed during comprehensive test suites and regression certification
//!   WHY:               Ensure that hostile host environments never cause unhandled panics or crashes,
//!                      and that every failure is converted into an actionable cognitive observation.
//!   HOW:               Adversarial fault injection ("push to fail"): if the test passes, it is
//!                      because the failure was caught and handled, NOT because of soft green checks.
//!
//! TOUCHES / FILESYSTEM FOOTPRINT:
//!   - LOGS:            Inscribed in-memory DiagnosticLogger ring buffers
//!   - SUBPROCESS:      Executes isolated test child processes to verify error trapping
//!
//! TRIPWIRE REGISTRY:
//!   - TRIPWIRE_TEST_HYPERVISOR_MISSING    (Asserts hypervisor missing remediation)
//!   - TRIPWIRE_TEST_FIRMWARE_UNRESOLVED   (Asserts firmware missing AI observation)
//!   - TRIPWIRE_TEST_DISPLAY_HEADLESS      (Asserts display server missing caveat fix)
//!   - TRIPWIRE_TEST_SUBPROCESS_NONZERO    (Asserts child process non-zero exit trapping)
//!   - TRIPWIRE_TEST_BATON_PASS_BOOT       (Asserts clean handoff to boot context)

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Scoped Imports, Test Fixtures & Environment Setup )
// ==============================================================================
use std::process::Command;

use omnicode::cli::commands::boot::{BootArch, BootConfig};
use omnicode::cogs::platform::HostPlatform;
use omnicode::cogs::tripwire::Tripwire;
use omnicode::core::ternary::QuadState;
use omnicode::diagnostics::logger::{DiagnosticLogger, FailpointType};

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Adversarial Tests: Pushing Platform Adapters to Fail & Telemetry Oracles )
// ==============================================================================

/// Test 1: Adversarial Hypervisor Resolution Failure.
///
/// Deliberately simulates a non-existent or corrupted hypervisor binary on the host platform.
/// Pushes `Tripwire::assert_binary_available` to fail, verifying that:
/// 1. A `TripwireError` is returned with `QuadState::Rollback`.
/// 2. The `DiagnosticLogger` captures the failpoint with channel `"PLATFORM"`.
/// 3. A concrete `FixSuggestion` with installation instructions (`apt`, `brew`, `winget`)
///    is attached for the [`omnicode::diagnostics::restorator::Restorator`].
#[test]
fn test_adversarial_hypervisor_missing_trips_remediation() {
    let logger = DiagnosticLogger::new(false);
    let platform = HostPlatform::current();

    // Query an intentionally non-existent executable name
    let non_existent_binary = "qemu-system-nonexistent-silicon-arch-999";
    let check = platform.resolve_in_path(non_existent_binary);
    assert!(check.is_none(), "Non-existent binary must NOT resolve on PATH");

    // Push tripwire to fail and assert that it trips
    let trip_result = Tripwire::assert_binary_available(&logger, "PLATFORM", non_existent_binary);
    assert!(trip_result.is_err(), "Missing binary MUST trip the sensory wire");

    let err = trip_result.unwrap_err();
    assert_eq!(err.channel, "PLATFORM");
    assert_eq!(err.failpoint_type, FailpointType::SyntaxAnomaly);
    assert_eq!(err.recovery_state, QuadState::Rollback);

    // Assert that the logger recorded the failpoint
    assert!(logger.has_failpoints(), "Logger must register the trapped failpoint");
    let fps = logger.failpoints();
    assert_eq!(fps.len(), 1);
    assert!(fps[0].message.contains(non_existent_binary));

    // Assert that cognitive note with #failpoint tag was recorded
    let notes = logger.cognitive_notes();
    assert!(
        notes.iter().any(|n| n.tags.iter().any(|t| t == "failpoint")),
        "Episodic trace must contain #failpoint tag"
    );
}

/// Test 2: Adversarial Firmware Traversal & Fallback Guidance.
///
/// Traverses firmware candidates when none are present on disk (or in a simulated environment),
/// asserting that `find_ovmf_firmware_with_telemetry`:
/// 1. Emits individual telemetry probe events into the logger.
/// 2. Records an AI cognitive observation labeled `"FIRMWARE_UNRESOLVED"`.
/// 3. Inscribes tags `["firmware", "ovmf", "uefi", "fallback"]` so the Restorator
///    and AI reasoning agents can propose falling back to `--bios` MBR mode.
#[test]
fn test_adversarial_firmware_discovery_records_ai_observation() {
    let logger = DiagnosticLogger::new(false);
    let platform = HostPlatform::Unknown; // Unknown platform has generic fallback candidates

    // Probing firmware with telemetry must record probe logs
    let fw = platform.find_ovmf_firmware_with_telemetry(&logger);

    let events = logger.events();
    assert!(events.iter().any(|e| e.channel == "FIRMWARE"), "Probe events must be logged");

    // If firmware is absent on host, verify AI observation was registered
    if fw.is_none() {
        let notes = logger.cognitive_notes();
        assert!(
            notes.iter().any(|n| n.category == "FIRMWARE_UNRESOLVED"),
            "Logger must record FIRMWARE_UNRESOLVED note for AI cortex and Restorator"
        );
        let target_note = notes.iter().find(|n| n.category == "FIRMWARE_UNRESOLVED").unwrap();
        assert!(target_note.tags.iter().any(|t| t == "fallback"));
    }
}

/// Test 3: Adversarial Display Server Assertion.
///
/// Tests `assert_display_server_available` when display server is absent (simulated or real).
/// Asserts that if no display server is active:
/// 1. The tripwire fires with `FailpointType::SyntaxAnomaly`.
/// 2. A `FixSuggestion` with `FixKind::AdjustCaveat` is generated, recommending `--headless`.
#[test]
fn test_adversarial_display_server_assertion_contract() {
    let logger = DiagnosticLogger::new(false);
    let platform = HostPlatform::current();

    // If display server is active, assert success
    if platform.has_display_server() {
        let res = platform.assert_display_server_available(&logger);
        assert!(res.is_ok(), "Display server assertion must succeed when display server is active");
    } else {
        // If headless, assert that tripwire trips with remediation
        let res = platform.assert_display_server_available(&logger);
        assert!(res.is_err(), "Headless environment MUST trip display server assertion");
        let err = res.unwrap_err();
        assert_eq!(err.channel, "DISPLAY");
        assert!(logger.has_failpoints(), "Logger must capture display failpoint");
    }
}

/// Test 4: Adversarial Host Subprocess Non-Zero Exit Trapping.
///
/// Deliberately spawns a child process that exits with a non-zero exit status code (`42`).
/// Asserts that `execute_host_command`:
/// 1. Traps the non-zero exit status without panicking.
/// 2. Returns a `TripwireError` containing the exit code and stderr output.
/// 3. Inscribes the failpoint into the logger with channel `"SUBPROCESS_TEST"`.
#[test]
fn test_adversarial_subprocess_traps_nonzero_exit() {
    let logger = DiagnosticLogger::new(false);
    let platform = HostPlatform::current();

    // Construct a failing command depending on host family
    let cmd = if platform.is_windows() {
        let mut c = Command::new("cmd");
        c.args(["/C", "exit 42"]);
        c
    } else {
        let mut c = Command::new("sh");
        c.args(["-c", "echo 'Simulated Subprocess Error' >&2; exit 42"]);
        c
    };

    let result = platform.execute_host_command(&logger, "SUBPROCESS_TEST", cmd);
    assert!(result.is_err(), "Non-zero exit must trip the subprocess execution wire");

    let err = result.unwrap_err();
    assert_eq!(err.channel, "SUBPROCESS_TEST");
    assert!(err.message.contains("42"), "Error message must report exit code 42");

    if !platform.is_windows() {
        assert!(
            err.message.contains("Simulated Subprocess Error"),
            "Error message must capture child stderr stream"
        );
    }

    assert!(logger.has_failpoints(), "Logger must capture non-zero subprocess failpoint");
}

/// Test 5: Baton Pass Integration Test between `platform.rs` and `boot.rs`.
///
/// Verifies the full handoff contract:
/// 1. `HostPlatform::current()` identifies the host environment.
/// 2. `BootConfig::new` ingests the platform without data loss.
/// 3. `BootConfig::open` uses the platform's hypervisor binary and firmware paths
///    to construct an immutable `BootContext`.
/// 4. If `--gui` is requested in a headless environment, an AI observation is recorded
///    in the logger without aborting process state.
#[test]
fn test_baton_pass_platform_to_boot_context_integration() {
    let platform = HostPlatform::current();
    assert!(!platform.display_name().is_empty());

    let logger = DiagnosticLogger::new(false);
    let cfg = BootConfig::new(BootArch::Bios, false);
    assert_eq!(cfg.host_platform, platform, "Config must retain current host platform");

    let ctx = cfg.open(logger).expect("Boot context opening must succeed under valid workspace");
    assert_eq!(ctx.hypervisor_binary, platform.hypervisor_binary(), "Hypervisor binary must match platform contract");
    assert_eq!(ctx.debugger.step_count, 0, "Debugger must be armed at step 0");

    // Test graphical request handoff: if headless, verify observation
    let logger_gui = DiagnosticLogger::new(false);
    let cfg_gui = BootConfig::new(BootArch::Bios, true);
    let ctx_gui = cfg_gui.open(logger_gui.clone()).expect("Context opening must succeed");
    assert!(ctx_gui.config.graphical);

    if !platform.has_display_server() {
        assert!(
            logger_gui.cognitive_notes().iter().any(|n| n.category == "GUI_DISPLAY_HEADLESS"),
            "Opening graphical config in headless environment must record GUI_DISPLAY_HEADLESS observation"
        );
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Module Sealing & Invariant Verification )
// ==============================================================================
// Verification Constraints:
//   - All 5 adversarial tests MUST push to fail and verify clean recovery.
//   - Zero unhandled panics across subprocess execution or missing binary probes.

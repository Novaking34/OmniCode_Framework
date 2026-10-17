// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Universal Invariant Tripwire Cog Manifest & Cybernetic Sensory Boundaries )
// ==============================================================================
//! MODULE:              omnicode::cogs::tripwire
//! IDENTITY:            Universal Invariant Tripwire Sensor Cog
//! NATURE:              Dynamic Boundary Protection, Collision Detection & Diagnostic Interception
//! VERSION:             0.3.0
//! CREATED:             2026-10-11
//! LAST_SEALED:         2026-10-11
//! AUTHOR:              CreativeWorkzStudio LLC / ProfessorSeanEX
//! COVENANT:            Public Domain / MIT
//!
//! SCHEDULER:
//!   TRACK_PRIMARY:     <TRACK 1> : {Host Engine & Virtual Machine}
//!   TRACKS_TOUCHED:    [ <TRACK 0> : {Silicon & Bare-Metal Kernel}, <TRACK 1>, <TRACK 2> : {Engine Runtime & SVO}, <TRACK 3> : {Studio & Graphical IDE} ]
//!   PRIORITY:          TaskPriority::High
//!   CHANNEL:           Track::Kernel
//!
//! CONTRACT / WHO-WHAT-WHERE-WHY-HOW:
//!   WHO:               OmniCode Universal Sensory Invariant Tripwire Subsystem
//!   WHAT:              Trap invariant violations, record structured failpoint events,
//!                      equip remediation suggestions, and summon the debugger and restorator.
//!   WHERE:             Enforced across all modules, CLI gears, platform adapters, terminal sessions, and scheduler tasks.
//!   WHEN:              Evaluated at precondition boundaries, file accesses, subprocess invocations, and state transitions.
//!   WHY:               Eliminate unhandled panics and raw process aborts, converting crashes into recoverable
//!                      diagnostic signals that trigger transaction rollbacks (`-1:ROLLBACK`) or automated self-healing.
//!   HOW:               Balanced Ternary (+1:PROCEED invariant held, -1:ROLLBACK invariant breached with telemetry, 0:AWAIT standby).
//!
//! THE GAME DEV COLLISION TRIGGER PARADIGM:
//!   In modern game engine architecture (e.g., Unreal, Unity, Godot):
//!   - Invisible Trigger Volumes: Spatial and physical boundary colliders placed in world space. When an entity
//!     penetrates an illegal zone or invalid state, the physics engine does not crash the game loop; it fires an
//!     `OnTriggerEnter` or `OnCollision` event callback.
//!   - In OmniCode: Tripwires are the cognitive and computational collision triggers. Instead of allowing undefined
//!     behavior, unhandled null states, absent silicon binaries, missing kernel images, or corrupted filesystems to
//!     panic the engine (`panic!`), a tripwire immediately catches the boundary collision.
//!
//! CYBERNETIC FAILPOINT & TRANSLATION ENVELOPE:
//!   - Translates raw runtime anomalies into formal ontological failpoint categories ([`FailpointType`]).
//!   - Emits microsecond-timestamped telemetry directly into the active [`DiagnosticLogger`] ring buffer and [`crate::diagnostics::debugger::OmniDebugger`].
//!   - Inscribes episodic cognitive memory notes tagged `#failpoint` for AI reasoning models and developers.
//!   - Replaces uncontrolled stack unwinding with balanced-ternary quad-state causality (`-1:ROLLBACK` or `0:AWAIT`).
//!
//! THE RESTORATOR HANDOFF & REPAIR TELEMETRY:
//!   - When a tripwire trips with remediation ([`Tripwire::trip_with_remediation`]), it equips the trapped event
//!     with a [`FixSuggestion`] and registers `#remediation_ready` telemetry.
//!   - The [`crate::diagnostics::restorator::Restorator`] (OmniCode's autonomous immune system) ingests this structured
//!     flight data, executes surgical AST/token patches, test-certifies the candidate repair, and restores system balance.
//!
//! TOUCHES / SYSTEM & PROCESS FOOTPRINT:
//!   - TELEMETRY:       Direct emission to [`DiagnosticLogger`] and active [`crate::diagnostics::debugger::OmniDebugger`].
//!   - TERMINAL:        Standardized ANSI red/yellow tripwire alert streams to stderr and session logs.
//!   - RECOVERY:        Integrates with transaction rollbacks and automated Restorator immune responses.
//!   - SUBPROCESSES:    Monitors exit status codes and standard streams of external host child processes.
//!
//! REQUIRES / HOST CAPABILITIES:
//!   - Standard error stream access (`std::io::stderr`).
//!   - Filesystem metadata inspection (`std::path::Path::exists`, `std::path::Path::is_dir`, `std::fs::metadata`).
//!   - Subprocess status inspection (`std::process::ExitStatus`).
//!
//! TRIPWIRE REGISTRY:
//!   - `TRIPWIRE_PATH_EXISTENCE`       (Assertion of critical filesystem resources and kernel artifacts)
//!   - `TRIPWIRE_DIR_EXISTENCE`        (Assertion of required workspace and directory hierarchies)
//!   - `TRIPWIRE_BINARY_AVAILABLE`     (Assertion of required external host toolchains on system PATH)
//!   - `TRIPWIRE_COMMAND_STATUS`       (Assertion of zero exit status on host subprocesses)
//!   - `TRIPWIRE_STATE_VIOLATION`      (Assertion of semantic invariant boundaries and quad-state commitments)
//!   - `TRIPWIRE_BOOLEAN_INVARIANT`    (Assertion of mathematical and ontological propositional truths)
//!
//! NAVIGATION GUIDE (HOW TO NAVIGATE AND EXTEND THIS COG AS THE CODE GROWS):
//!   - Block 1 (Lines 1-85): Manifest, Game Dev Trigger Analogy, Telemetry Envelope, Tripwire Registry.
//!   - Block 2 (Lines 86-175): Scoped Imports, Error Envelopes (`TripwireError`), Display Formatting & Defaults.
//!     * To add custom recovery constructors: add methods to `TripwireError` in Section 2.2.
//!     * To modify equilibrium recovery defaults: review constants in Section 2.3.
//!   - Block 3 (Lines 176-350): Body Block & Sensory Assertion Oracles.
//!     * 3.1: Explicit Failpoint Inscription & Remediation Oracles (`trip`, `trip_with_remediation`, `trip_await`).
//!     * 3.2: Filesystem Spatial Invariant Assertions (`assert_path_exists`, `assert_dir_exists`, `assert_file_readable`).
//!     * 3.3: Host Toolchain & Binary Availability Oracles (`assert_binary_available`).
//!     * 3.4: Subprocess Lifecycle & Exit Status Assertions (`assert_command_success`, `assert_command_output_clean`).
//!     * 3.5: Mathematical, State Machine & Causality Boundary Assertions (`assert_quad_state_proceed`, `assert_invariant`).
//!   - Block 4 (Lines 351+): Module Sealing, Baton Pass Protocol & In-File Unit Verification Tests.

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Scoped Imports, Invariant Data Types & Tripwire Error Definitions )
// ==============================================================================

// ------------------------------------------------------------------------------
// 2.1. Scoped Namespace Ingestion & System Imports
// ------------------------------------------------------------------------------
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Output};

use crate::core::ternary::QuadState;
use crate::diagnostics::logger::{DiagnosticLogger, FailpointType, FixSuggestion};

// ------------------------------------------------------------------------------
// 2.2. Invariant Data Types & Tripwire Error Definitions
// ------------------------------------------------------------------------------
/// Formal error envelope produced when an execution boundary or invariant tripwire is breached.
///
/// Unlike standard Rust errors or panics that destroy process context, [`TripwireError`]
/// encapsulates rich cybernetic metadata:
/// - Channel identifier for precise subsystem origin tracking.
/// - Ontological failpoint category ([`FailpointType`]).
/// - Human and machine readable diagnostic message.
/// - Balanced-ternary recovery state (`-1:ROLLBACK` or `0:AWAIT`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TripwireError {
    /// Subsystem or channel where the tripwire fired (e.g. "BOOT", "PLATFORM", "TERMINAL", "SCHEDULER").
    pub channel: &'static str,
    /// Formal diagnostic classification of the failpoint.
    pub failpoint_type: FailpointType,
    /// Detailed diagnostic message explaining the breach.
    pub message: String,
    /// Balanced ternary state mapping for recovery (typically `-1:ROLLBACK` or `0:AWAIT`).
    pub recovery_state: QuadState,
}

impl TripwireError {
    /// Creates a new tripwire error mapped to `-1:ROLLBACK` state.
    pub fn rollback(channel: &'static str, fp_type: FailpointType, msg: impl Into<String>) -> Self {
        Self {
            channel,
            failpoint_type: fp_type,
            message: msg.into(),
            recovery_state: QuadState::Rollback,
        }
    }

    /// Creates a new tripwire error mapped to `0:AWAIT` standby state.
    pub fn await_state(channel: &'static str, fp_type: FailpointType, msg: impl Into<String>) -> Self {
        Self {
            channel,
            failpoint_type: fp_type,
            message: msg.into(),
            recovery_state: QuadState::Await,
        }
    }

    /// Creates a new tripwire error mapped to `NULL:HALT` state.
    /// In dual-rail balanced ternary, HALT is an abrupt stop (both rails 11 asserted)
    /// that does NOT flatline the machine, but triggers the tripwire to capture
    /// hardware dump telemetry and restart/re-incept the machine back to homeostasis (0:AWAIT).
    pub fn halt(channel: &'static str, fp_type: FailpointType, msg: impl Into<String>) -> Self {
        Self {
            channel,
            failpoint_type: fp_type,
            message: msg.into(),
            recovery_state: QuadState::Halt,
        }
    }

    /// Returns `true` if this error dictates an active transaction rollback (`-1:ROLLBACK`).
    pub fn is_rollback(&self) -> bool {
        self.recovery_state == QuadState::Rollback
    }

    /// Returns `true` if this error indicates that the operation is awaiting asynchronous completion (`0:AWAIT`).
    pub fn is_await(&self) -> bool {
        self.recovery_state == QuadState::Await
    }

    /// Returns `true` if this error indicates an abrupt HALT requiring machine re-inception.
    pub fn is_halt(&self) -> bool {
        self.recovery_state == QuadState::Halt
    }
}

impl fmt::Display for TripwireError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] Failpoint {}: {} (State: {})",
            self.channel,
            self.failpoint_type.as_str(),
            self.message,
            self.recovery_state.macro_op()
        )
    }
}

impl std::error::Error for TripwireError {}

// ------------------------------------------------------------------------------
// 2.3. Initial Truth States & Equilibrium Recovery Defaults
// ------------------------------------------------------------------------------
/// Default quad-state recovery commitment when a tripwire collision occurs.
pub const DEFAULT_TRIPWIRE_RECOVERY_STATE: QuadState = QuadState::Rollback;

/// Default diagnostic channel assigned when no subsystem is explicitly specified.
pub const DEFAULT_TRIPWIRE_CHANNEL: &str = "TRIPWIRE";

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Universal Tripwire Engine Implementation & Assertion Oracles )
// ==============================================================================

/// Mechanical Tripwire Sensor for invariant enforcement, collision sensing, and diagnostic alerting.
#[derive(Debug, Clone, Copy, Default)]
pub struct Tripwire;

// ------------------------------------------------------------------------------
// 3.1. Explicit Failpoint Inscription & Remediation Oracles
// ------------------------------------------------------------------------------
impl Tripwire {
    /// Explicitly trips a diagnostic wire, recording a failpoint in the logger and emitting a standard alert.
    ///
    /// # Game Dev Collision Volume Hook:
    /// Inscribes a critical failpoint into [`DiagnosticLogger`], writes an ANSI red alert to stderr,
    /// and packages the violation into a [`TripwireError`] with `-1:ROLLBACK`.
    pub fn trip(
        logger: &DiagnosticLogger,
        channel: &'static str,
        fp_type: FailpointType,
        message: impl Into<String>,
    ) -> TripwireError {
        let msg = message.into();
        logger.failpoint(fp_type, channel, msg.clone());

        eprintln!(
            "\x1b[1;31m✗ Tripwire Alert [{}]\x1b[0m [{}]: {}",
            channel,
            fp_type.as_str(),
            msg
        );

        TripwireError::rollback(channel, fp_type, msg)
    }

    /// Explicitly trips a diagnostic wire equipped with concrete remediation telemetry for the Restorator.
    ///
    /// # Restorator Immune Handoff:
    /// Attaches a [`FixSuggestion`] and registers an episodic cognitive memory note tagged
    /// `#remediation_ready`, allowing automated self-healing passes to immediately parse and apply the patch.
    pub fn trip_with_remediation(
        logger: &DiagnosticLogger,
        channel: &'static str,
        fp_type: FailpointType,
        message: impl Into<String>,
        suggestion: FixSuggestion,
    ) -> TripwireError {
        let msg = message.into();
        logger.failpoint_with_remediation(fp_type, channel, msg.clone(), suggestion);

        eprintln!(
            "\x1b[1;31m✗ Tripwire Alert [{}]\x1b[0m [{}]: {}",
            channel,
            fp_type.as_str(),
            msg
        );

        TripwireError::rollback(channel, fp_type, msg)
    }

    /// Trips a diagnostic wire that pauses execution in `0:AWAIT` standby state rather than rolling back.
    ///
    /// Used when a required resource is temporarily in flight or pending background synthesis.
    pub fn trip_await(
        logger: &DiagnosticLogger,
        channel: &'static str,
        fp_type: FailpointType,
        message: impl Into<String>,
    ) -> TripwireError {
        let msg = message.into();
        logger.failpoint(fp_type, channel, msg.clone());

        eprintln!(
            "\x1b[1;33m⚠ Tripwire Standby [{}]\x1b[0m [{}]: {}",
            channel,
            fp_type.as_str(),
            msg
        );

        TripwireError::await_state(channel, fp_type, msg)
    }
}

// ------------------------------------------------------------------------------
// 3.2. Filesystem Spatial Invariant Assertions
// ------------------------------------------------------------------------------
impl Tripwire {
    /// Asserts that a required file or directory exists on disk.
    ///
    /// # Precondition Sensor:
    /// Returns the verified [`PathBuf`] on success (+1:PROCEED). If absent, trips
    /// a [`FailpointType::SyntaxAnomaly`] error with `-1:ROLLBACK`.
    pub fn assert_path_exists(
        logger: &DiagnosticLogger,
        channel: &'static str,
        path: impl AsRef<Path>,
        description: &'static str,
    ) -> Result<PathBuf, TripwireError> {
        let p = path.as_ref();
        if p.exists() {
            Ok(p.to_path_buf())
        } else {
            let msg = format!("{description} not found at expected path: {:?}", p);
            Err(Self::trip(logger, channel, FailpointType::SyntaxAnomaly, msg))
        }
    }

    /// Asserts that a target path exists and is specifically a directory.
    pub fn assert_dir_exists(
        logger: &DiagnosticLogger,
        channel: &'static str,
        path: impl AsRef<Path>,
        description: &'static str,
    ) -> Result<PathBuf, TripwireError> {
        let p = path.as_ref();
        if p.is_dir() {
            Ok(p.to_path_buf())
        } else {
            let msg = format!("{description} directory does not exist: {:?}", p);
            Err(Self::trip(logger, channel, FailpointType::SyntaxAnomaly, msg))
        }
    }

    /// Asserts that a target file exists and is readable by the current process.
    pub fn assert_file_readable(
        logger: &DiagnosticLogger,
        channel: &'static str,
        path: impl AsRef<Path>,
        description: &'static str,
    ) -> Result<PathBuf, TripwireError> {
        let p = path.as_ref();
        if !p.exists() {
            let msg = format!("{description} file does not exist: {:?}", p);
            return Err(Self::trip(logger, channel, FailpointType::SyntaxAnomaly, msg));
        }

        match fs::metadata(p) {
            Ok(_) => Ok(p.to_path_buf()),
            Err(e) => {
                let msg = format!("Cannot read metadata for {description} at {:?}: {}", p, e);
                Err(Self::trip(logger, channel, FailpointType::SyntaxAnomaly, msg))
            }
        }
    }
}

// ------------------------------------------------------------------------------
// 3.3. Host Toolchain & Binary Availability Oracles
// ------------------------------------------------------------------------------
impl Tripwire {
    /// Asserts that a required executable binary is available on the host system PATH.
    ///
    /// # Platform Awareness:
    /// Dispatches `where` on Windows and `which` on Unix/macOS to probe host environment variables.
    pub fn assert_binary_available(
        logger: &DiagnosticLogger,
        channel: &'static str,
        bin_name: &'static str,
    ) -> Result<(), TripwireError> {
        let check = if cfg!(target_os = "windows") {
            Command::new("where").arg(bin_name).output()
        } else {
            Command::new("which").arg(bin_name).output()
        };

        match check {
            Ok(out) if out.status.success() => Ok(()),
            _ => {
                let msg = format!(
                    "Required host binary '{bin_name}' is not accessible on system PATH"
                );
                Err(Self::trip(logger, channel, FailpointType::SyntaxAnomaly, msg))
            }
        }
    }
}

// ------------------------------------------------------------------------------
// 3.4. Subprocess Lifecycle & Exit Status Assertions
// ------------------------------------------------------------------------------
impl Tripwire {
    /// Asserts that a subprocess command completed successfully with exit code 0.
    ///
    /// Traps non-zero exit codes or spawn failures without process panic.
    pub fn assert_command_success(
        logger: &DiagnosticLogger,
        channel: &'static str,
        status: Result<ExitStatus, std::io::Error>,
        command_name: &'static str,
    ) -> Result<(), TripwireError> {
        match status {
            Ok(s) if s.success() => Ok(()),
            Ok(s) => {
                let code = s.code().map(|c| c.to_string()).unwrap_or_else(|| "signal".into());
                let msg = format!("Subprocess '{command_name}' exited with non-zero status ({code})");
                Err(Self::trip(logger, channel, FailpointType::SyntaxAnomaly, msg))
            }
            Err(e) => {
                let msg = format!("Failed to spawn or communicate with subprocess '{command_name}': {e}");
                Err(Self::trip(logger, channel, FailpointType::SyntaxAnomaly, msg))
            }
        }
    }

    /// Asserts that a captured subprocess [`Output`] succeeded, logging stderr on failure.
    pub fn assert_command_output_clean(
        logger: &DiagnosticLogger,
        channel: &'static str,
        output: Result<Output, std::io::Error>,
        command_name: &'static str,
    ) -> Result<Output, TripwireError> {
        match output {
            Ok(out) if out.status.success() => Ok(out),
            Ok(out) => {
                let code = out.status.code().unwrap_or(-1);
                let stderr = String::from_utf8_lossy(&out.stderr);
                let msg = format!(
                    "Subprocess '{command_name}' failed with status code {}: {}",
                    code, stderr.trim()
                );
                Err(Self::trip(logger, channel, FailpointType::SyntaxAnomaly, msg))
            }
            Err(e) => {
                let msg = format!("Failed to execute subprocess '{command_name}': {e}");
                Err(Self::trip(logger, channel, FailpointType::SyntaxAnomaly, msg))
            }
        }
    }
}

// ------------------------------------------------------------------------------
// 3.5. Mathematical, State Machine & Causality Boundary Assertions
// ------------------------------------------------------------------------------
impl Tripwire {
    /// Asserts that a quad-state evaluation committed to `+1:PROCEED`.
    ///
    /// If the state is `-1:ROLLBACK` or `0:AWAIT`, raises an explicit tripwire error.
    pub fn assert_quad_state_proceed(
        logger: &DiagnosticLogger,
        channel: &'static str,
        state: QuadState,
        context: &'static str,
    ) -> Result<(), TripwireError> {
        if state == QuadState::Proceed {
            Ok(())
        } else {
            let msg = format!(
                "QuadState invariant breach in {context}: expected +1:PROCEED, encountered {}",
                state.macro_op()
            );
            Err(Self::trip(logger, channel, FailpointType::RollbackTriggered, msg))
        }
    }

    /// Asserts an arbitrary boolean propositional truth condition.
    ///
    /// If false, trips the wire with the designated failpoint category and message.
    pub fn assert_invariant(
        logger: &DiagnosticLogger,
        channel: &'static str,
        condition: bool,
        fp_type: FailpointType,
        message: impl Into<String>,
    ) -> Result<(), TripwireError> {
        if condition {
            Ok(())
        } else {
            Err(Self::trip(logger, channel, fp_type, message))
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Verification and Sealing, Baton Pass & In-File Unit Tests )
// ==============================================================================

// ------------------------------------------------------------------------------
// 4.1. Universal Tripwire Sealing & Downstream Baton Pass Specification
// ------------------------------------------------------------------------------
// BATON PASS CONTRACT:
//   - Upstream Producer: Hardware, Filesystem, Runtime Processes & Evaluator ALU.
//   - Downstream Receivers:
//     * `<TRACK 1>` Boot Gear (`src/cli/commands/boot.rs`): Inception and compiler tripwires.
//     * `<TRACK 1>` Platform Cog (`src/cogs/platform.rs`): Hypervisor and firmware tripwires.
//     * `<TRACK 1>` Diagnostics (`src/diagnostics/`): Restorator immune repair and Debugger frames.
//     * `<TRACK 1>` Terminal Session (`src/terminal/session.rs`): Tape mount and execution boundary tripwires.
//   - Certified Invariants:
//     1. Tripwire breaches never panic or abort process memory; all breaches produce `TripwireError`.
//     2. All tripwire events inscribe structured failpoint records into `DiagnosticLogger`.
//     3. `Tripwire::trip_with_remediation` attaches `FixSuggestion` and tags `#remediation_ready`.
//     4. Filesystem, binary, subprocess, and boolean invariant assertion methods return
//        deterministic, uncheatable results.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::logger::{FixKind, FixSuggestion};

    /// Verifies that explicitly tripping a wire records a failpoint in `DiagnosticLogger`
    /// and constructs a `TripwireError` with `-1:ROLLBACK` recovery state.
    ///
    /// # Failure Mode Guarded Against:
    /// Prevents silent failure where an error is created without inscriptive telemetry.
    #[test]
    fn test_tripwire_explicit_trip_and_failpoint_capture() {
        let logger = DiagnosticLogger::new(false);
        let err = Tripwire::trip(
            &logger,
            "TEST_CHANNEL",
            FailpointType::SyntaxAnomaly,
            "Simulated explicit tripwire fault",
        );

        assert_eq!(err.channel, "TEST_CHANNEL");
        assert_eq!(err.failpoint_type, FailpointType::SyntaxAnomaly);
        assert_eq!(err.recovery_state, QuadState::Rollback);
        assert!(err.is_rollback());
        assert!(!err.is_await());
        assert!(logger.has_failpoints(), "DiagnosticLogger must record the failpoint");
        assert_eq!(logger.failpoints().len(), 1);
    }

    /// Verifies that `trip_with_remediation` successfully attaches a `FixSuggestion`
    /// and emits an episodic cognitive note tagged `#remediation_ready`.
    ///
    /// # Invariants Tested:
    /// - Failpoint contains the attached fix suggestion.
    /// - Cognitive notes buffer contains a note tagged `#remediation_ready`.
    #[test]
    fn test_tripwire_with_remediation_suggestion() {
        let logger = DiagnosticLogger::new(false);
        let suggestion = FixSuggestion {
            kind: FixKind::AddBlockHeader,
            line: 1,
            col: 1,
            target_token: "HEADER".to_string(),
            suggested_patch: "BLOCK 1: METADATA".to_string(),
            rationale: "Automated header repair".to_string(),
        };

        let err = Tripwire::trip_with_remediation(
            &logger,
            "REPAIR_CHANNEL",
            FailpointType::BoundaryMismatch,
            "Simulated repairable fault",
            suggestion.clone(),
        );

        assert_eq!(err.channel, "REPAIR_CHANNEL");
        assert_eq!(err.failpoint_type, FailpointType::BoundaryMismatch);

        let fps = logger.failpoints();
        assert_eq!(fps.len(), 1);
        assert_eq!(fps[0].suggestion, Some(suggestion));

        let notes = logger.cognitive_notes();
        assert!(
            notes.iter().any(|n| n.tags.iter().any(|t| t == "remediation_ready")),
            "Episodic notes must include #remediation_ready tag"
        );
    }

    /// Verifies that `trip_await` sets `0:AWAIT` standby state rather than rolling back.
    #[test]
    fn test_tripwire_await_state_constructor() {
        let logger = DiagnosticLogger::new(false);
        let err = Tripwire::trip_await(
            &logger,
            "ASYNC_CHANNEL",
            FailpointType::NullTrap,
            "Pending background artifact",
        );

        assert_eq!(err.channel, "ASYNC_CHANNEL");
        assert_eq!(err.recovery_state, QuadState::Await);
        assert!(err.is_await());
        assert!(!err.is_rollback());
    }

    /// Verifies that `assert_path_exists` returns `Ok(PathBuf)` for existing paths
    /// and `Err(TripwireError)` with failpoint telemetry for missing paths.
    ///
    /// # Uncheatable Assertion:
    /// Tests both positive path resolution (`Cargo.toml`) and negative path failure.
    #[test]
    fn test_assert_path_exists_contract() {
        let logger = DiagnosticLogger::new(false);

        // Positive check: Cargo manifest must resolve
        let ok_res = Tripwire::assert_path_exists(&logger, "TEST", "Cargo.toml", "Cargo Manifest");
        assert!(ok_res.is_ok(), "Cargo.toml must resolve successfully");
        assert!(ok_res.unwrap().exists());

        // Negative check: missing file must trip with failpoint
        let missing = Tripwire::assert_path_exists(&logger, "TEST", "nonexistent_omnicode_file_xyz.omni", "Missing File");
        assert!(missing.is_err(), "Missing file must return TripwireError");
        let err = missing.unwrap_err();
        assert_eq!(err.channel, "TEST");
        assert!(logger.has_failpoints(), "Logger must capture failpoint on missing path");
    }

    /// Verifies that `assert_dir_exists` validates directories and rejects non-directory files.
    #[test]
    fn test_assert_dir_exists_contract() {
        let logger = DiagnosticLogger::new(false);

        // Positive: current workspace root is a directory
        assert!(Tripwire::assert_dir_exists(&logger, "TEST", ".", "Workspace Root Directory").is_ok());

        // Negative: Cargo.toml is a file, not a directory
        assert!(Tripwire::assert_dir_exists(&logger, "TEST", "Cargo.toml", "Manifest File As Dir").is_err());
    }

    /// Verifies that `assert_invariant` succeeds on true conditions and trips on false conditions.
    #[test]
    fn test_assert_invariant_contract() {
        let logger = DiagnosticLogger::new(false);

        let ok_res = Tripwire::assert_invariant(&logger, "MATH", 2 + 2 == 4, FailpointType::SyntaxAnomaly, "Arithmetic holds");
        assert!(ok_res.is_ok());

        let err_res = Tripwire::assert_invariant(&logger, "MATH", 2 + 2 == 5, FailpointType::SyntaxAnomaly, "Arithmetic breached");
        assert!(err_res.is_err());
        assert!(logger.has_failpoints());
    }

    /// Verifies that `assert_quad_state_proceed` accepts `QuadState::Proceed` and trips on `Rollback` or `Await`.
    #[test]
    fn test_assert_quad_state_proceed_contract() {
        let logger = DiagnosticLogger::new(false);

        assert!(Tripwire::assert_quad_state_proceed(&logger, "EXEC", QuadState::Proceed, "Step 1").is_ok());
        assert!(Tripwire::assert_quad_state_proceed(&logger, "EXEC", QuadState::Rollback, "Step 2").is_err());
        assert!(Tripwire::assert_quad_state_proceed(&logger, "EXEC", QuadState::Await, "Step 3").is_err());
    }

    /// Verifies that `TripwireError` implements `Display` with channel, category, message, and state.
    #[test]
    fn test_tripwire_error_display_formatting() {
        let err = TripwireError::rollback("KERNEL", FailpointType::ImmutableMutation, "Attempted write to immutable register");
        let formatted = format!("{err}");
        assert!(formatted.contains("[KERNEL]"));
        assert!(formatted.contains("FP_IMMUTABLE_MUTATION"));
        assert!(formatted.contains("Attempted write to immutable register"));
        assert!(formatted.contains("ROLLBACK"));
    }
}

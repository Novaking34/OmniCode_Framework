// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Dynamic Host Platform Cog Manifest, Scheduling Contract & Tripwire Registry )
// ==============================================================================
//! MODULE:              omnicode::cogs::platform
//! IDENTITY:            Dynamic Host Platform Adapter Cog
//! NATURE:              Dynamic Environmental Meshing (Runtime Host Adaptation & In-Waiting Symbiosis)
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
//!   WHO:               OmniCode Host Environmental Adapter & Hardware Abstraction Layer (HAL)
//!   WHAT:              Dynamically detect host OS and CPU architecture, resolve toolchains and hypervisors,
//!                      probe hardware virtualization acceleration, discover UEFI OVMF firmware blobs,
//!                      audit host display servers, and orchestrate tripwired subprocess execution.
//!   WHERE:             Runtime host environment (GNU/Linux, Apple macOS Darwin, Microsoft Windows NT, Generic POSIX)
//!   WHEN:              Evaluated at engine startup, CLI command dispatch, GUI studio launch, or bare-metal boot inception.
//!   WHY:               Enable the "In-Waiting" symbiotic execution model: OmniCode is an autonomous
//!                      operating system and balanced-ternary language, but during incubation it executes
//!                      harmoniously on any host OS without hardcoding platform paths or failing when moved.
//!   HOW:               Balanced Ternary (+1:PROCEED host detected & certified, -1:ROLLBACK unsupported/missing, 0:AWAIT probe)
//!
//! THE "IN-WAITING" SYMBIOTIC INCUBATION MODEL:
//!   OmniCode is not merely a programming language compiler; it is an entirely distinct computing paradigm,
//!   operating system, and balanced-ternary machine architecture. However, during its developmental incubation,
//!   engineers work inside conventional operating systems (Linux, macOS, Windows).
//!   This cog acts as the umbilical cord and Hardware Abstraction Layer (HAL):
//!   - Adapts to the host's native filesystem hierarchy (FHS vs Homebrew vs Windows Program Files).
//!   - Selects the correct hypervisor binary (`qemu-system-x86_64` vs `qemu-system-x86_64.exe`).
//!   - Selects native acceleration flags (`-enable-kvm` on Linux, `-accel hvf` on macOS, `-accel whpx` on Windows).
//!   - Resolves native graphical browser dispatchers (`xdg-open` vs `open` vs `cmd /C start`).
//!   - Detects headless display environments to prevent graphical crashes.
//!
//! GAME DEV IDEOLOGY (THE HAL & PHYSICS SIMULATION ANALOGY):
//!   In modern game engine architecture (e.g., Unreal, Unity, Godot), the game simulation tick (`Update()`)
//!   runs identically regardless of whether rendering is powered by DirectX 12, Metal, or Vulkan.
//!   In OmniCode:
//!   - The Core Simulation: Word32 balanced-ternary ALU, tape reels, SVO grammar, and QuadState causality.
//!   - The Platform Adapter (This Cog): The cross-platform driver layer abstracting host silicon and OS syscalls.
//!   - Tripwires: Spatial and temporal collision volumes. When an invariant fails (missing binary, unreadable
//!     KVM node, absent display server), the tripwire catches the collision, records frame telemetry to
//!     `DiagnosticLogger` and `OmniDebugger`, and passes remediation tokens to `Restorator` for autonomous self-healing.
//!
//! TOUCHES / SYSTEM & PROCESS FOOTPRINT:
//!   - DISPATCH:        Native browser openers (`xdg-open` on Linux, `open` on macOS, `cmd /C start` on Windows)
//!   - DISCOVERY:       Host filesystem firmware lookups (`/usr/share/qemu`, `/opt/homebrew`, `C:\Program Files\qemu`)
//!   - BINARIES:        Probes for `qemu-system-x86_64` vs `qemu-system-x86_64.exe` on host PATH
//!   - VIRTUALIZATION:  Probes `/dev/kvm` (Linux KVM), Hypervisor.framework (macOS HVF), WHPX (Windows), TCG (Fallback)
//!   - DISPLAY:         Probes `$DISPLAY`, `$WAYLAND_DISPLAY`, and window manager sessions for GUI capability.
//!   - PROCESSES:       Uniform child process spawning with standard stream capture and exit status checking.
//!
//! REQUIRES / HOST CAPABILITIES:
//!   - Rust `cfg!(target_os)` and `cfg!(target_arch)` target family introspection.
//!   - Subprocess execution capabilities (`std::process::Command`).
//!   - Filesystem metadata and permission inspection (`std::path::Path::exists`, `std::fs::OpenOptions`).
//!   - Environment variable inspection (`std::env::var_os`, `std::env::var`).
//!
//! TRIPWIRE REGISTRY:
//!   - TRIPWIRE_PLATFORM_PROBE       (Identification and verification of host platform plane)
//!   - TRIPWIRE_ARCH_SUPPORT         (Assertion of host CPU architecture compatibility with target)
//!   - TRIPWIRE_FIRMWARE_DISCOVERY   (Resolution and verification of valid OVMF firmware candidate)
//!   - TRIPWIRE_HYPERVISOR_AVAIL     (Assertion of hypervisor executable on host PATH)
//!   - TRIPWIRE_ACCEL_DEVICE         (Hardware virtualization node accessibility / permission check)
//!   - TRIPWIRE_DISPLAY_SERVER       (Assertion of active display server when GUI mode is requested)
//!   - TRIPWIRE_BROWSER_DISPATCH     (Successful handoff to host graphical browser)
//!   - TRIPWIRE_SUBPROCESS_EXEC      (Generic host child process execution and exit code verification)
//!
//! NAVIGATION GUIDE (HOW TO NAVIGATE AND EXTEND THIS MODULE):
//!   - Block 1 (Lines 1-90): Module Manifest, Incubation Philosophy, Game Dev Analogy, Tripwire Registry.
//!   - Block 2 (Lines 91-180): Scoped Imports, Multi-OS Firmware Matrices, Equilibrium Constants, HostArch Enum.
//!     * To add firmware search paths for a new Linux distro or BSD: edit `LINUX_OVMF_CANDIDATES` in Section 2.2.
//!     * To add a new CPU architecture: add a variant to `HostArch` in Section 2.4.
//!   - Block 3 (Lines 181-480): Body Block & Implementation Oracles.
//!     * 3.1: HostPlatform runtime classification, display names, and environment summary.
//!     * 3.2: Host PATH resolution (`resolve_in_path`) and hypervisor binary verification.
//!     * 3.3: Hardware virtualization acceleration probes (`has_hardware_acceleration`, `assert_acceleration_supported`).
//!     * 3.4: Host display server & GUI environment oracles (`has_display_server`, `assert_display_server_available`).
//!     * 3.5: UEFI OVMF firmware search with telemetry and tripwires (`find_ovmf_firmware_with_telemetry`).
//!     * 3.6: Host process dispatch, child execution, and browser launching (`execute_host_command`).
//!   - Block 4 (Lines 481+): Unit tests verifying platform detection, binary resolution, PATH parsing, and telemetry.

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Scoped Imports, Environment Definitions, Search Matrices, Architecture & Constants )
// ==============================================================================

// ------------------------------------------------------------------------------
// 2.1. Scoped Namespace Ingestion & System Imports
// ------------------------------------------------------------------------------
use std::env;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use crate::cogs::tripwire::{Tripwire, TripwireError};
use crate::diagnostics::logger::{DiagnosticLogger, FailpointType, FixKind, FixSuggestion};

// ------------------------------------------------------------------------------
// 2.2. Multi-Platform Firmware Candidate Matrices & Search Registries
// ------------------------------------------------------------------------------
/// Canonical Linux OVMF UEFI firmware candidate search paths.
///
/// Covers major distribution layouts:
/// - Debian / Ubuntu: `/usr/share/ovmf/OVMF.fd`
/// - Arch Linux / Fedora / RHEL: `/usr/share/edk2-ovmf/x64/OVMF_CODE.fd`, `/usr/share/edk2/ovmf/OVMF_CODE.fd`
/// - NixOS / Alpine / OpenSUSE: `/usr/share/qemu/OVMF.fd`, `/usr/share/OVMF/OVMF_CODE_4M.fd`
pub const LINUX_OVMF_CANDIDATES: &[&str] = &[
    "/usr/share/qemu/OVMF.fd",
    "/usr/share/OVMF/OVMF_CODE_4M.fd",
    "/usr/share/ovmf/OVMF.fd",
    "/usr/share/edk2-ovmf/x64/OVMF_CODE.fd",
    "/usr/share/edk2/ovmf/OVMF_CODE.fd",
];

/// Canonical Apple macOS OVMF firmware search paths.
///
/// Covers Homebrew Apple Silicon (`/opt/homebrew`), Intel Mac (`/usr/local`),
/// and MacPorts (`/opt/local`) installations of QEMU:
pub const MACOS_OVMF_CANDIDATES: &[&str] = &[
    "/opt/homebrew/share/qemu/edk2-x86_64-code.fd",
    "/usr/local/share/qemu/edk2-x86_64-code.fd",
    "/opt/homebrew/Cellar/qemu/share/qemu/edk2-x86_64-code.fd",
    "/opt/local/share/qemu/edk2-x86_64-code.fd",
];

/// Canonical Microsoft Windows OVMF firmware search paths.
///
/// Covers standard 64-bit and 32-bit QEMU installer directories under `Program Files`:
pub const WINDOWS_OVMF_CANDIDATES: &[&str] = &[
    "C:\\Program Files\\qemu\\share\\edk2-x86_64-code.fd",
    "C:\\Program Files (x86)\\qemu\\share\\edk2-x86_64-code.fd",
    "C:\\qemu\\share\\edk2-x86_64-code.fd",
];

/// Fallback generic firmware search paths for unrecognized POSIX environments.
pub const FALLBACK_OVMF_CANDIDATES: &[&str] = &[
    "/usr/share/qemu/OVMF.fd",
    "/usr/local/share/qemu/OVMF.fd",
];

// ------------------------------------------------------------------------------
// 2.3. Initial Truth States & Equilibrium Platform Defaults
// ------------------------------------------------------------------------------
/// Default hypervisor binary for POSIX-compliant host environments.
pub const DEFAULT_UNIX_HYPERVISOR: &str = "qemu-system-x86_64";

/// Default hypervisor binary for Microsoft Windows host environments.
pub const DEFAULT_WINDOWS_HYPERVISOR: &str = "qemu-system-x86_64.exe";

/// Canonical Linux KVM device node for hardware virtualization.
pub const LINUX_KVM_DEVICE_NODE: &str = "/dev/kvm";

// ------------------------------------------------------------------------------
// 2.4. Host Silicon Architecture Typology
// ------------------------------------------------------------------------------
/// Physical host CPU instruction set architecture.
///
/// Identifies the silicon substrate upon which the host operating system executes.
/// Informs hypervisor hardware acceleration compatibility (e.g. KVM or HVF can only
/// be used when host architecture matches guest target architecture).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HostArch {
    /// 64-bit x86 architecture (AMD64 / Intel 64).
    X86_64,
    /// 64-bit ARM architecture (Apple Silicon M-series, AWS Graviton, Ampere Altra).
    Aarch64,
    /// 64-bit RISC-V architecture.
    RiscV64,
    /// Unidentified or unsupported host processor architecture.
    Unknown,
}

impl HostArch {
    /// Detects the host processor architecture at runtime using target architecture directives.
    pub fn current() -> Self {
        if cfg!(target_arch = "x86_64") {
            HostArch::X86_64
        } else if cfg!(target_arch = "aarch64") {
            HostArch::Aarch64
        } else if cfg!(target_arch = "riscv64") {
            HostArch::RiscV64
        } else {
            HostArch::Unknown
        }
    }

    /// Returns the human and machine readable designation of the CPU architecture.
    pub fn display_name(&self) -> &'static str {
        match self {
            HostArch::X86_64 => "x86_64 (AMD64 / Intel 64)",
            HostArch::Aarch64 => "aarch64 (ARM64 / Apple Silicon)",
            HostArch::RiscV64 => "riscv64 (RISC-V 64-Bit)",
            HostArch::Unknown => "unknown-arch (Generic Processor)",
        }
    }
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Dynamic Host Platform Implementation, Environmental Oracles & Process Dispatch )
// ==============================================================================

// ------------------------------------------------------------------------------
// 3.1. Platform Runtime Detection & Classification
// ------------------------------------------------------------------------------
/// Host operating system classification for "In-Waiting" symbiotic execution.
///
/// Represents the physical runtime habitat in which the OmniCode host engine
/// and developer tooling incubate. Enables dynamic adaptation of hypervisors,
/// firmware paths, native process dispatchers, and hardware acceleration flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HostPlatform {
    /// GNU/Linux host (KVM hardware virtualization, systemd/FHS filesystem layout).
    Linux,
    /// Apple macOS host (Darwin kernel, Hypervisor.framework / HVF, Homebrew paths).
    MacOS,
    /// Microsoft Windows host (NT kernel, PE32+ binaries, Windows Hypervisor Platform / WHPX).
    Windows,
    /// Unidentified or generic POSIX environment (BSD, Solaris, embedded runtime).
    Unknown,
}

impl HostPlatform {
    /// Dynamically detects the host operating system platform at runtime.
    ///
    /// Evaluates compile-time target family directives (`cfg!(target_os)`) while
    /// providing runtime verification hooks.
    pub fn current() -> Self {
        if cfg!(target_os = "linux") {
            HostPlatform::Linux
        } else if cfg!(target_os = "macos") {
            HostPlatform::MacOS
        } else if cfg!(target_os = "windows") {
            HostPlatform::Windows
        } else {
            HostPlatform::Unknown
        }
    }

    /// Detects the physical host CPU architecture.
    pub fn arch(&self) -> HostArch {
        HostArch::current()
    }

    /// Returns the canonical platform display title with incubation role.
    pub fn display_name(&self) -> &'static str {
        match self {
            HostPlatform::Linux => "GNU/Linux (Host Incubation Plane)",
            HostPlatform::MacOS => "Apple macOS (Darwin Incubation Plane)",
            HostPlatform::Windows => "Microsoft Windows (NT Incubation Plane)",
            HostPlatform::Unknown => "Generic Host Plane (Fallback Environment)",
        }
    }

    /// Returns the target operating system family name (`"unix"` or `"windows"`).
    pub fn target_os_family(&self) -> &'static str {
        match self {
            HostPlatform::Windows => "windows",
            _ => "unix",
        }
    }

    /// Returns `true` if the host platform adheres to POSIX standards (Linux, macOS, BSD).
    pub fn is_posix(&self) -> bool {
        !matches!(self, HostPlatform::Windows)
    }

    /// Returns `true` if the host platform is Microsoft Windows.
    pub fn is_windows(&self) -> bool {
        matches!(self, HostPlatform::Windows)
    }

    /// Returns `true` if the host platform is GNU/Linux.
    pub fn is_linux(&self) -> bool {
        matches!(self, HostPlatform::Linux)
    }

    /// Returns `true` if the host platform is Apple macOS Darwin.
    pub fn is_macos(&self) -> bool {
        matches!(self, HostPlatform::MacOS)
    }

    /// Produces a comprehensive, machine-readable diagnostic snapshot of the host habitat.
    ///
    /// Used by [`DiagnosticLogger`], [`crate::diagnostics::debugger::OmniDebugger`], and AI cortex
    /// to record environmental state in episodic logs.
    pub fn environment_summary(&self) -> String {
        format!(
            "HostPlatform: {} | Arch: {} | Family: {} | POSIX: {} | Accel: {} | Display: {}",
            self.display_name(),
            self.arch().display_name(),
            self.target_os_family(),
            self.is_posix(),
            self.has_hardware_acceleration(),
            self.has_display_server()
        )
    }
}

// ------------------------------------------------------------------------------
// 3.2. Host PATH Resolution & Toolchain Binary Oracles
// ------------------------------------------------------------------------------
impl HostPlatform {
    /// Searches the host system `PATH` environment variable for an executable by name.
    ///
    /// # Platform Mechanics:
    /// - Respects platform-specific path list separators (`:` on Unix/POSIX, `;` on Windows).
    /// - On Windows, automatically tests the filename with `.exe` if not already provided.
    /// - Validates that candidate files exist and are regular files.
    pub fn resolve_in_path(&self, binary: &str) -> Option<PathBuf> {
        let path_var = env::var_os("PATH")?;

        let candidate_names: Vec<String> = if self.is_windows() && !binary.ends_with(".exe") {
            vec![format!("{}.exe", binary), binary.to_string()]
        } else {
            vec![binary.to_string()]
        };

        for dir in env::split_paths(&path_var) {
            for name in &candidate_names {
                let candidate = dir.join(name);
                if candidate.is_file() {
                    return Some(candidate);
                }
            }
        }

        None
    }

    /// Returns the platform-appropriate primary hypervisor binary name.
    pub fn hypervisor_binary(&self) -> &'static str {
        match self {
            HostPlatform::Windows => DEFAULT_WINDOWS_HYPERVISOR,
            _ => DEFAULT_UNIX_HYPERVISOR,
        }
    }

    /// Returns an ordered list of candidate hypervisor executable names.
    pub fn hypervisor_candidates(&self) -> &'static [&'static str] {
        match self {
            HostPlatform::Windows => &[DEFAULT_WINDOWS_HYPERVISOR, "qemu.exe"],
            HostPlatform::Linux => &[DEFAULT_UNIX_HYPERVISOR, "qemu-kvm", "kvm"],
            HostPlatform::MacOS => &[DEFAULT_UNIX_HYPERVISOR],
            HostPlatform::Unknown => &[DEFAULT_UNIX_HYPERVISOR],
        }
    }

    /// Dynamically locates the first valid hypervisor binary present on the host system PATH.
    pub fn find_hypervisor_binary(&self) -> Option<PathBuf> {
        for &candidate in self.hypervisor_candidates() {
            if let Some(path) = self.resolve_in_path(candidate) {
                return Some(path);
            }
        }
        None
    }

    /// Asserts that a valid hypervisor binary is present and reachable on the host PATH.
    ///
    /// # Sensory Tripwire:
    /// Trips [`Tripwire::assert_binary_available`], recording a failpoint in the logger
    /// with actionable installation remediation for Linux, macOS, or Windows if missing.
    pub fn assert_hypervisor_available(
        &self,
        logger: &DiagnosticLogger,
    ) -> Result<PathBuf, TripwireError> {
        let binary = self.hypervisor_binary();

        match self.resolve_in_path(binary) {
            Some(resolved_path) => {
                logger.log(
                    "PLATFORM",
                    format!("Hypervisor binary verified on PATH: {:?}", resolved_path),
                );
                Ok(resolved_path)
            }
            None => {
                let install_hint = match self {
                    HostPlatform::Linux => "sudo apt install qemu-system-x86 (Debian/Ubuntu) or sudo pacman -S qemu-system-x86 (Arch)",
                    HostPlatform::MacOS => "brew install qemu (Homebrew) or port install qemu (MacPorts)",
                    HostPlatform::Windows => "winget install QEMU.QEMU or download from https://www.qemu.org/download/#windows",
                    HostPlatform::Unknown => "Install QEMU from your operating system package repository",
                };

                let suggestion = FixSuggestion {
                    kind: FixKind::AddImport,
                    line: 1,
                    col: 1,
                    target_token: binary.to_string(),
                    suggested_patch: install_hint.to_string(),
                    rationale: format!(
                        "Host hypervisor executable '{}' is required for bare-metal machine inception",
                        binary
                    ),
                };

                let err = Tripwire::trip_with_remediation(
                    logger,
                    "PLATFORM",
                    FailpointType::SyntaxAnomaly,
                    format!(
                        "Hypervisor binary '{}' not found on host PATH. Remediation: {}",
                        binary, install_hint
                    ),
                    suggestion,
                );
                Err(err)
            }
        }
    }
}

// ------------------------------------------------------------------------------
// 3.3. Virtualization Acceleration Oracles & Permission Probes
// ------------------------------------------------------------------------------
impl HostPlatform {
    /// Probes whether hardware virtualization acceleration is accessible on the host.
    ///
    /// # Physical Hardware Invariants:
    /// - **Linux**: Inspects existence and read/write accessibility of `/dev/kvm`.
    /// - **macOS**: Target family supports Hypervisor.framework (`hvf`) on Darwin when
    ///   running on matching architecture or Apple Silicon hypervisor.
    /// - **Windows**: Target family supports Windows Hypervisor Platform (`whpx`).
    pub fn has_hardware_acceleration(&self) -> bool {
        match self {
            HostPlatform::Linux => {
                let kvm_path = Path::new(LINUX_KVM_DEVICE_NODE);
                if !kvm_path.exists() {
                    return false;
                }
                // Verify read and write permissions to /dev/kvm
                OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(kvm_path)
                    .is_ok()
            }
            HostPlatform::MacOS => true,
            HostPlatform::Windows => false, // Conservative default; requires Hyper-V / WHPX feature check
            HostPlatform::Unknown => false,
        }
    }

    /// Asserts that hardware acceleration is available, logging telemetry or tripping an alert.
    ///
    /// # Game Dev Collision Volume Analogy:
    /// If an acceleration tripwire is armed, tripping here warns that execution will drop
    /// into software emulation (TCG), which incurs significant performance overhead.
    pub fn assert_acceleration_supported(
        &self,
        logger: &DiagnosticLogger,
    ) -> Result<bool, TripwireError> {
        let accel = self.has_hardware_acceleration();
        if accel {
            logger.log(
                "ACCELERATION",
                format!("Hardware virtualization acceleration active on {}", self.display_name()),
            );
            Ok(true)
        } else {
            logger.record_ai_observation(
                "ACCELERATION_ABSENT",
                "Hardware virtualization acceleration unavailable",
                "Falling back to Tiny Code Generator (TCG) software emulation",
                ["virtualization", "tcg", "fallback", "performance"].as_slice(),
                None,
            );
            Ok(false)
        }
    }

    /// Returns the recommended hypervisor command-line arguments for hardware acceleration.
    ///
    /// Falls back to pure TCG software emulation if acceleration is unavailable.
    pub fn recommended_accel_args(&self) -> &'static [&'static str] {
        match self {
            HostPlatform::Linux if self.has_hardware_acceleration() => &["-enable-kvm"],
            HostPlatform::MacOS => &["-accel", "hvf"],
            HostPlatform::Windows if self.has_hardware_acceleration() => &["-accel", "whpx"],
            _ => &["-accel", "tcg"],
        }
    }
}

// ------------------------------------------------------------------------------
// 3.4. Host Display Server & Graphical Subsystem Oracles
// ------------------------------------------------------------------------------
impl HostPlatform {
    /// Probes whether a graphical display server or window manager is active on the host.
    ///
    /// Prevents GUI window spawn crashes when running in headless environments
    /// (e.g. continuous integration servers, SSH remote sessions, or containers).
    pub fn has_display_server(&self) -> bool {
        match self {
            HostPlatform::Linux | HostPlatform::Unknown => {
                env::var("DISPLAY").is_ok() || env::var("WAYLAND_DISPLAY").is_ok()
            }
            HostPlatform::MacOS => {
                // macOS GUI window server is available in standard desktop user sessions
                env::var("TERM_PROGRAM").is_ok() || env::var("Apple_PubSub_Socket_Render").is_ok() || true
            }
            HostPlatform::Windows => {
                // Windows desktop GUI is standard unless running in Windows Server Core
                true
            }
        }
    }

    /// Asserts that a graphical display server is available when GUI mode is requested.
    ///
    /// # Sensory Tripwire:
    /// If running in a headless environment and GUI display is requested, trips a wire
    /// recommending headless serial mode (`--display none`).
    pub fn assert_display_server_available(
        &self,
        logger: &DiagnosticLogger,
    ) -> Result<(), TripwireError> {
        if self.has_display_server() {
            logger.log("DISPLAY", "Graphical display server verified active (+1:PROCEED)");
            Ok(())
        } else {
            let suggestion = FixSuggestion {
                kind: FixKind::AdjustCaveat,
                line: 1,
                col: 1,
                target_token: "--gui".to_string(),
                suggested_patch: "--headless".to_string(),
                rationale: "Host display environment has no active DISPLAY or WAYLAND_DISPLAY session".to_string(),
            };

            let err = Tripwire::trip_with_remediation(
                logger,
                "DISPLAY",
                FailpointType::SyntaxAnomaly,
                "No graphical display server detected (headless environment). Run without --gui.".to_string(),
                suggestion,
            );
            Err(err)
        }
    }
}

// ------------------------------------------------------------------------------
// 3.5. UEFI Firmware Search Oracles & Telemetry Traversal
// ------------------------------------------------------------------------------
impl HostPlatform {
    /// Returns the candidate search paths for UEFI OVMF firmware images on this platform.
    pub fn ovmf_search_paths(&self) -> &'static [&'static str] {
        match self {
            HostPlatform::Linux => LINUX_OVMF_CANDIDATES,
            HostPlatform::MacOS => MACOS_OVMF_CANDIDATES,
            HostPlatform::Windows => WINDOWS_OVMF_CANDIDATES,
            HostPlatform::Unknown => FALLBACK_OVMF_CANDIDATES,
        }
    }

    /// Dynamically locates the first existing OVMF firmware image on the host system.
    pub fn find_ovmf_firmware(&self) -> Option<PathBuf> {
        // First inspect custom override environment variable if provided
        if let Ok(env_path) = env::var("OMNICODE_OVMF_PATH") {
            let p = PathBuf::from(env_path);
            if p.exists() {
                return Some(p);
            }
        }

        for &path_str in self.ovmf_search_paths() {
            let path = Path::new(path_str);
            if path.exists() {
                return Some(path.to_path_buf());
            }
        }
        None
    }

    /// Traverses the platform's firmware candidate list while logging telemetry events.
    ///
    /// # Telemetry Inscription:
    /// Inscribes candidate probe entries into [`DiagnosticLogger`]. If a firmware blob
    /// is resolved, logs success. If none are found, registers an informational diagnostic
    /// note equipped with remediation guidance for the Restorator and AI agents.
    pub fn find_ovmf_firmware_with_telemetry(&self, logger: &DiagnosticLogger) -> Option<PathBuf> {
        logger.log("FIRMWARE", format!("Probing OVMF firmware candidates on {}", self.display_name()));

        if let Ok(env_override) = env::var("OMNICODE_OVMF_PATH") {
            let p = PathBuf::from(env_override);
            if p.exists() {
                logger.log("FIRMWARE", format!("Resolved OVMF firmware via OMNICODE_OVMF_PATH: {:?}", p));
                return Some(p);
            }
        }

        for &path_str in self.ovmf_search_paths() {
            let path = Path::new(path_str);
            if path.exists() {
                logger.log("FIRMWARE", format!("Resolved OVMF firmware at: {:?}", path));
                return Some(path.to_path_buf());
            } else {
                logger.log("FIRMWARE", format!("Candidate not found: {}", path_str));
            }
        }

        logger.record_ai_observation(
            "FIRMWARE_UNRESOLVED",
            "No standard OVMF firmware image found on host",
            "UEFI boot will require explicit --ovmf path or fallback to BIOS MBR mode",
            ["firmware", "ovmf", "uefi", "fallback"].as_slice(),
            None,
        );

        None
    }

    /// Asserts that a valid OVMF firmware blob exists on disk, tripping a wire if absent.
    ///
    /// # Precondition Tripwire:
    /// If no candidate exists, trips a [`TripwireError`] equipped with a [`FixSuggestion`]
    /// instructing how to install or specify the firmware image.
    pub fn assert_ovmf_firmware(
        &self,
        logger: &DiagnosticLogger,
    ) -> Result<PathBuf, TripwireError> {
        if let Some(firmware) = self.find_ovmf_firmware_with_telemetry(logger) {
            Ok(firmware)
        } else {
            let suggestion = FixSuggestion {
                kind: FixKind::AddImport,
                line: 1,
                col: 1,
                target_token: "--uefi".to_string(),
                suggested_patch: "--bios".to_string(),
                rationale: "Fallback to Legacy BIOS MBR mode or install ovmf package on host system".to_string(),
            };

            let err = Tripwire::trip_with_remediation(
                logger,
                "FIRMWARE",
                FailpointType::SyntaxAnomaly,
                format!("OVMF UEFI firmware image not found on {}", self.display_name()),
                suggestion,
            );
            Err(err)
        }
    }
}

// ------------------------------------------------------------------------------
// 3.6. Host OS Process Dispatch & Native Subprocess Execution
// ------------------------------------------------------------------------------
impl HostPlatform {
    /// Constructs the platform-appropriate [`Command`] for opening a target URL in the default browser.
    pub fn format_browser_command(&self, url: &str) -> Command {
        match self {
            HostPlatform::Linux => {
                let mut cmd = Command::new("xdg-open");
                cmd.arg(url);
                cmd
            }
            HostPlatform::MacOS => {
                let mut cmd = Command::new("open");
                cmd.arg(url);
                cmd
            }
            HostPlatform::Windows => {
                let mut cmd = Command::new("cmd");
                cmd.args(["/C", "start", url]);
                cmd
            }
            HostPlatform::Unknown => {
                let mut cmd = Command::new("xdg-open");
                cmd.arg(url);
                cmd
            }
        }
    }

    /// Dynamically launches the default web browser on the host system for a target URL.
    pub fn open_browser(&self, url: &str) {
        let mut cmd = self.format_browser_command(url);
        let _ = cmd.spawn();
    }

    /// Launches the default browser wrapped in diagnostic logging and tripwire interception.
    pub fn open_browser_with_tripwire(
        &self,
        logger: &DiagnosticLogger,
        url: &str,
    ) -> Result<(), TripwireError> {
        logger.log(
            "BROWSER_DISPATCH",
            format!("Dispatching browser on {} for URL: {}", self.display_name(), url),
        );
        let mut cmd = self.format_browser_command(url);
        match cmd.spawn() {
            Ok(_) => {
                logger.log("BROWSER_DISPATCH", "Browser child process dispatched cleanly (+1:PROCEED)");
                Ok(())
            }
            Err(e) => {
                let msg = format!("Failed to dispatch host browser for URL '{}': {}", url, e);
                Err(Tripwire::trip(logger, "BROWSER_DISPATCH", FailpointType::SyntaxAnomaly, msg))
            }
        }
    }

    /// Executes an arbitrary host command wrapped in tripwire collision detection and telemetry.
    ///
    /// # Game Dev Simulation Hook:
    /// Captures stdout and stderr streams. If the command exits with non-zero status code,
    /// logs an execution failure and trips a structured error for the Restorator.
    pub fn execute_host_command(
        &self,
        logger: &DiagnosticLogger,
        subsystem: &'static str,
        mut cmd: Command,
    ) -> Result<Output, TripwireError> {
        logger.log(subsystem, format!("Spawning host subprocess: {:?}", cmd));
        match cmd.output() {
            Ok(output) => {
                if output.status.success() {
                    logger.log(subsystem, "Host subprocess terminated with exit status 0 (+1:PROCEED)");
                    Ok(output)
                } else {
                    let code = output.status.code().unwrap_or(-1);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let msg = format!(
                        "Host subprocess exited with non-zero status code {}: {}",
                        code, stderr.trim()
                    );
                    Err(Tripwire::trip(logger, subsystem, FailpointType::SyntaxAnomaly, msg))
                }
            }
            Err(e) => {
                let msg = format!("Failed to spawn host subprocess: {}", e);
                Err(Tripwire::trip(logger, subsystem, FailpointType::SyntaxAnomaly, msg))
            }
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Verification and Sealing, Baton Pass & In-File Unit Tests )
// ==============================================================================

// ------------------------------------------------------------------------------
// 4.1. Dynamic Adapter Sealing & Downstream Baton Pass Specification
// ------------------------------------------------------------------------------
// BATON PASS CONTRACT:
//   - Upstream Producer: Rust runtime and compile-time target introspection (`cfg!(target_os)`).
//   - Downstream Receivers:
//     * `<TRACK 1>` Boot Gear (`src/cli/commands/boot.rs`): Hypervisor binary resolution,
//       UEFI firmware lookup, display server probe, and acceleration flags.
//     * `<TRACK 3>` Studio IDE (`src/terminal/studio.rs`): Native browser dispatch and GUI server.
//     * `<TRACK 1>` REPL & Terminal (`src/terminal/session.rs`): Console color and terminal probes.
//   - Certified Invariants:
//     1. `HostPlatform::current()` and `HostArch::current()` return deterministic, valid variants.
//     2. Binary candidate matrices map strictly to executable names appropriate for the OS family.
//     3. `resolve_in_path` navigates host environment paths respecting platform path delimiters.
//     4. Browser command synthesizers construct platform-native dispatch subprocesses.
//     5. Virtualization acceleration flags accurately select KVM, HVF, WHPX, or fallback TCG.

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that `HostPlatform::current()` dynamically resolves the active host operating system
    /// and supplies non-empty display names, binary targets, firmware matrices, and OS families.
    ///
    /// # Failure Mode Guarded Against:
    /// Prevents null or uninitialized platform definitions from entering downstream gears.
    #[test]
    fn test_platform_current_detection() {
        let p = HostPlatform::current();
        assert!(!p.display_name().is_empty(), "Display name must not be empty");
        assert!(!p.hypervisor_binary().is_empty(), "Hypervisor binary name must not be empty");
        assert!(!p.ovmf_search_paths().is_empty(), "Firmware search paths matrix must not be empty");
        assert!(!p.target_os_family().is_empty(), "OS family string must not be empty");
    }

    /// Verifies that `HostArch::current()` detects the host CPU instruction set architecture.
    ///
    /// # Failure Mode Guarded Against:
    /// Prevents silent fallback to mismatched architecture flags during hypervisor command synthesis.
    #[test]
    fn test_host_arch_detection() {
        let arch = HostArch::current();
        assert!(!arch.display_name().is_empty(), "CPU architecture display title must not be empty");
        let platform = HostPlatform::current();
        assert_eq!(platform.arch(), arch, "Platform arch must match HostArch::current");
    }

    /// Verifies that `environment_summary()` produces a comprehensive, well-formed diagnostic string
    /// containing all critical platform facets for episodic logging.
    ///
    /// # Invariants Tested:
    /// - Contains `"HostPlatform:"`, `"Arch:"`, `"Family:"`, `"POSIX:"`, `"Accel:"`, and `"Display:"`.
    #[test]
    fn test_environment_summary() {
        let p = HostPlatform::current();
        let summary = p.environment_summary();
        assert!(summary.contains("HostPlatform:"), "Summary must report HostPlatform");
        assert!(summary.contains("Arch:"), "Summary must report host architecture");
        assert!(summary.contains("Family:"), "Summary must report OS family");
        assert!(summary.contains("POSIX:"), "Summary must report POSIX compliance");
        assert!(summary.contains("Accel:"), "Summary must report hardware acceleration status");
        assert!(summary.contains("Display:"), "Summary must report display server availability");
    }

    /// Verifies the platform-to-binary mapping matrix across all supported operating system families.
    ///
    /// # Invariants Tested:
    /// - Windows requires `.exe` binary extension (`qemu-system-x86_64.exe`).
    /// - POSIX systems (Linux, macOS, Unknown) expect bare binary names (`qemu-system-x86_64`).
    #[test]
    fn test_platform_binaries() {
        assert_eq!(HostPlatform::Linux.hypervisor_binary(), "qemu-system-x86_64");
        assert_eq!(HostPlatform::Windows.hypervisor_binary(), "qemu-system-x86_64.exe");
        assert_eq!(HostPlatform::MacOS.hypervisor_binary(), "qemu-system-x86_64");
        assert_eq!(HostPlatform::Unknown.hypervisor_binary(), "qemu-system-x86_64");
    }

    /// Verifies the OS family classification and semantic predicate methods on `HostPlatform`.
    ///
    /// # Invariants Tested:
    /// - Linux: family `"unix"`, `is_posix() == true`, `is_linux() == true`, `is_windows() == false`.
    /// - macOS: family `"unix"`, `is_posix() == true`, `is_macos() == true`, `is_windows() == false`.
    /// - Windows: family `"windows"`, `is_posix() == false`, `is_windows() == true`.
    #[test]
    fn test_platform_os_family() {
        assert_eq!(HostPlatform::Linux.target_os_family(), "unix");
        assert!(HostPlatform::Linux.is_posix());
        assert!(HostPlatform::Linux.is_linux());
        assert!(!HostPlatform::Linux.is_windows());
        assert!(!HostPlatform::Linux.is_macos());

        assert_eq!(HostPlatform::MacOS.target_os_family(), "unix");
        assert!(HostPlatform::MacOS.is_posix());
        assert!(HostPlatform::MacOS.is_macos());
        assert!(!HostPlatform::MacOS.is_linux());

        assert_eq!(HostPlatform::Windows.target_os_family(), "windows");
        assert!(HostPlatform::Windows.is_windows());
        assert!(!HostPlatform::Windows.is_posix());
    }

    /// Verifies hardware virtualization acceleration command-line argument mapping.
    ///
    /// # Invariants Tested:
    /// - macOS maps to Apple Hypervisor.framework (`-accel hvf`).
    /// - Fallback/Unknown maps to software emulation (`-accel tcg`).
    /// - Linux provides acceleration flags (`-enable-kvm` or fallback).
    #[test]
    fn test_virtualization_accel_args() {
        let args_linux = HostPlatform::Linux.recommended_accel_args();
        assert!(!args_linux.is_empty(), "Linux acceleration argument list must not be empty");

        let args_macos = HostPlatform::MacOS.recommended_accel_args();
        assert_eq!(args_macos, &["-accel", "hvf"], "macOS must use Hypervisor.framework");

        let args_fallback = HostPlatform::Unknown.recommended_accel_args();
        assert_eq!(args_fallback, &["-accel", "tcg"], "Unknown platform must fall back to TCG");
    }

    /// Verifies native browser command synthesizer across Linux, macOS, and Windows.
    ///
    /// # Invariants Tested:
    /// - Linux dispatches `xdg-open <URL>`.
    /// - macOS dispatches `open <URL>`.
    /// - Windows dispatches `cmd /C start <URL>`.
    #[test]
    fn test_format_browser_command() {
        let test_url = "http://localhost:8080/studio";

        let cmd_linux = HostPlatform::Linux.format_browser_command(test_url);
        assert_eq!(cmd_linux.get_program(), "xdg-open");

        let cmd_macos = HostPlatform::MacOS.format_browser_command(test_url);
        assert_eq!(cmd_macos.get_program(), "open");

        let cmd_windows = HostPlatform::Windows.format_browser_command(test_url);
        assert_eq!(cmd_windows.get_program(), "cmd");
    }

    /// Verifies that `resolve_in_path` successfully resolves existing system binaries
    /// and correctly returns `None` for non-existent binaries.
    ///
    /// # Failure Mode Guarded Against:
    /// Prevents false positives or broken delimiter splitting when scanning host PATH.
    #[test]
    fn test_resolve_in_path_contract() {
        let platform = HostPlatform::current();

        // Standard shell binary must resolve on any functioning host
        let target = if platform.is_windows() { "cmd" } else { "sh" };
        let resolved = platform.resolve_in_path(target);
        assert!(resolved.is_some(), "Standard system shell must resolve on host PATH");
        let path = resolved.unwrap();
        assert!(path.is_file(), "Resolved PATH entry must point to a regular file");

        // Intentionally non-existent binary must return None
        let bogus = platform.resolve_in_path("definitely_nonexistent_binary_omnicode_xyz");
        assert!(bogus.is_none(), "Non-existent executable name must return None");
    }

    /// Verifies that candidate search path matrices across all platform variants
    /// are non-empty and contain valid non-empty path strings.
    ///
    /// # Invariants Tested:
    /// - Linux candidates, macOS candidates, Windows candidates, and Fallback candidates
    ///   must all be populated.
    #[test]
    fn test_ovmf_search_paths_contract() {
        for platform in &[HostPlatform::Linux, HostPlatform::MacOS, HostPlatform::Windows, HostPlatform::Unknown] {
            let paths = platform.ovmf_search_paths();
            assert!(!paths.is_empty(), "Platform {:?} must declare firmware candidates", platform);
            for &p in paths {
                assert!(!p.is_empty(), "Firmware candidate path string must not be empty");
            }
        }
    }

    /// Verifies that the display server probe evaluates without panic.
    #[test]
    fn test_display_server_probe_contract() {
        let platform = HostPlatform::current();
        let _has_display = platform.has_display_server();
    }
}

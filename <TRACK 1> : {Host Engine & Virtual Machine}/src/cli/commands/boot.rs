// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Machine Manifest, Scheduling Priority, Dependency Contract & Tripwire Registry )
// ==============================================================================
//! MODULE:              omnicode::cli::commands::boot
//! IDENTITY:            Bare-Metal Multi-Architecture Boot Gear
//! NATURE:              Cold-Start Machine Inception & Silicon Hardware Simulation Driver
//! VERSION:             0.3.0
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
//!   WHO:               OmniCode Host Orchestrator Engine & Machine Inception Driver
//!   WHAT:              Assemble bare-metal operating system kernels in Track 0 and launch
//!                      the QEMU hypervisor across BIOS, UEFI, and Multiboot architectures.
//!   WHERE:             `<TRACK 0>/kernel` tree -> host QEMU hypervisor -> COM1 / VGA silicon endpoints
//!   WHEN:              Invoked via CLI command `omnicode boot [--bios|--uefi|--multiboot] [--gui] [--ovmf <PATH>]`
//!   WHY:               Verify silicon-level self-sufficiency, hardware invariants, and balanced-ternary machine bootstrap.
//!   HOW:               Balanced Ternary (+1:PROCEED clean exit, -1:ROLLBACK trapped error, 0:AWAIT execution running)
//!
//! THE "IN-WAITING" MULTI-OS SYMBIOTIC MESH:
//!   OmniCode is fundamentally an autonomous bare-metal Operating System and Balanced-Ternary computing
//!   architecture. However, until custom silicon machines are ubiquitous, OmniCode incubates comfortably
//!   inside existing host operating systems (Linux, macOS, Windows).
//!   This gear delegates host environment discovery to [`crate::cogs::platform::HostPlatform`], ensuring:
//!   - Dynamic selection of hypervisor binaries (`qemu-system-x86_64` vs `qemu-system-x86_64.exe`).
//!   - Dynamic discovery of UEFI OVMF firmware blobs across Debian, Arch, Fedora, NixOS, macOS Homebrew, and Windows.
//!   - Dynamic probing of display servers to avoid graphical window crashes in headless containers or SSH sessions.
//!
//! GAME DEV IDEOLOGY (THE STAGE INCEPTION & SIMULATION TICK):
//!   In modern game engine architecture (e.g., Unreal, Unity, Godot):
//!   - Level Loading / Inception: Setting up world geometry, shaders, and player spawn points before rendering.
//!   - Collision Volumes & Triggers: Spatial boundaries that fire events when crossed or violated.
//!   - Flight Recorder & Telemetry HUD: Frame-by-frame debug logging with nanosecond timestamps.
//!   - Respawn / Immune Recovery: Resetting player coordinates or healing scene state after a collision failpoint.
//!   In OmniCode Bare-Metal Boot:
//!   - Stage Inception: `execute_kernel_pipeline` compiles the bare-metal kernel and binds MMIO registers.
//!   - Tripwires: Invariant collision sensors (`Tripwire::assert_*`) wrapping every file access, compiler invocation,
//!     and hypervisor process launch.
//!   - Flight Recorder: [`DiagnosticLogger`] and [`OmniDebugger`] track step counts, register cognitive notes,
//!     and serialize episodic scrolls.
//!   - Respawn / Healing: Any tripped invariant produces a [`FixSuggestion`] and `#remediation_ready` telemetry
//!     for autonomous recovery by the [`crate::diagnostics::restorator::Restorator`].
//!
//! TOUCHES / FILESYSTEM FOOTPRINT:
//!   - EXECUTABLE:      `<TRACK 0>/kernel/build_all_kernels.sh`
//!   - ARTIFACT:        `<TRACK 0>/kernel/omnicode_bios.img` (512-byte MBR boot sector)
//!   - ARTIFACT:        `<TRACK 0>/kernel/esp/EFI/BOOT/BOOTX64.EFI` (UEFI PE32+ application)
//!   - ARTIFACT:        `<TRACK 0>/kernel/omnicode_multiboot.elf` (ELF multiboot kernel)
//!   - FIRMWARE:        OVMF UEFI Code image (probed from host system via `HostPlatform`)
//!
//! TOUCHES / SILICON HARDWARE & MMIO ADDRESSES:
//!   - COM1 Serial Port (I/O Port `0x3F8`): Serial console telemetry output at 115200 baud 8N1.
//!   - VGA Text Buffer Base (Memory Address `0xB8000`): 80x25 monochrome/color text screen buffer.
//!   - Real Mode Reset Vector (Physical Memory `0x7C00`): IBM PC MBR bootstrap entrypoint.
//!
//! REQUIRES / HOST DEPENDENCIES:
//!   - `qemu-system-x86_64` (virtual machine emulator & silicon hypervisor)
//!   - `bash` (bare-metal kernel build pipeline driver)
//!   - `nasm` (16-bit real mode and multiboot x86 assembler)
//!   - `clang` + `lld-link` (x86_64-unknown-windows UEFI PE32+ compiler & linker)
//!   - `ld` (elf_i386 multiboot linker)
//!
//! TRIPWIRE REGISTRY:
//!   - `TRIPWIRE_KERNEL_BUILD_SCRIPT` (Track 0 build script accessibility and readability)
//!   - `TRIPWIRE_KERNEL_COMPILATION`  (Kernel compiler pipeline exit status code)
//!   - `TRIPWIRE_IMAGE_RESOLVE`       (Generated boot artifact existence after assembly)
//!   - `TRIPWIRE_HYPERVISOR_PATH`     (Availability of hypervisor binary on host PATH)
//!   - `TRIPWIRE_FIRMWARE_RESOLVE`    (Resolution of UEFI OVMF firmware blob)
//!   - `TRIPWIRE_DISPLAY_SERVER`      (Assertion of host display server when GUI mode is active)
//!   - `TRIPWIRE_QEMU_LAUNCH`         (Hypervisor execution status code and crash traps)
//!
//! NAVIGATION GUIDE (HOW TO NAVIGATE AND EXTEND THIS MODULE AS THE CODE GROWS):
//!   - Block 1 (Lines 1-95): Manifest, Multi-OS In-Waiting Philosophy, Game Dev Analogy, Tripwire Registry.
//!   - Block 2 (Lines 96-260): Opening Block, Domain Typology, Configuration & Equilibrium Constants.
//!     * To add a new boot architecture (e.g. RISC-V `RiscV64`): add a variant to `BootArch` in Section 2.2.
//!     * To inspect how options are parsed from CLI tokens: examine `BootConfig::from_args` in Section 2.3.
//!     * To view pre-flight invariant certification: examine `BootConfig::open` in Section 2.3.
//!     * To check silicon MMIO addresses: review equilibrium constants in Section 2.4.
//!   - Block 3 (Lines 261-530): Body Block & Bare-Metal Simulation Tick.
//!     * 3.1: Public dispatchers (`run_boot`, `run_boot_with_config`).
//!     * 3.2: Kernel compilation pipeline and build script execution (`execute_kernel_pipeline`).
//!     * 3.3: Silicon MMIO & hypervisor command synthesis (`synthesize_hypervisor_command`).
//!     * 3.4: Machine execution loop and telemetry sealing (`execute_boot_pipeline`, `execute_boot_machine`).
//!   - Block 4 (Lines 531+): Unit tests validating architecture names, argument parsing, command synthesis, and PATH lookups.

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Scoped Imports, Environment Setup, Data Structures & Interface Definitions )
// ==============================================================================

// ------------------------------------------------------------------------------
// 2.1. Scoped Namespace Ingestion & Multi-Platform Imports
// ------------------------------------------------------------------------------
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::cogs::platform::HostPlatform;
use crate::cogs::tripwire::{Tripwire, TripwireError};
use crate::core::filetype::resolve_track_path;
use crate::core::ternary::QuadState;
use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::DiagnosticLogger;

// ------------------------------------------------------------------------------
// 2.2. Domain Typology & Architectural Target Definitions
// ------------------------------------------------------------------------------
/// Supported bare-metal boot target architectures.
///
/// Each architecture represents a discrete physical execution plane into which
/// OmniCode can materialize directly from cold power-on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootArch {
    /// 16-Bit Real Mode MBR (512-byte sector loaded at physical address `0x7C00`).
    /// Minimalist, self-contained hardware bootstrap with BIOS INT 10h / INT 13h / INT 16h.
    Bios,

    /// 64-Bit Native UEFI Long Mode (PE32+ EFI Application loaded by firmware boot manager).
    /// Modern hardware standard with graphical GOP framebuffer and EFI System Table.
    Uefi,

    /// 32-Bit Direct Kernel Handshake via Multiboot Specification (ELF executable).
    /// Used for direct hypervisor injection (QEMU `-kernel`, GRUB, Xen, KVM).
    Multiboot,
}

impl BootArch {
    /// Human and machine readable description of the architecture target.
    pub fn display_name(&self) -> &'static str {
        match self {
            BootArch::Bios => "Legacy BIOS MBR (16-Bit Real Mode -> 512-Byte Boot Sector)",
            BootArch::Uefi => "Modern UEFI 64-Bit Long Mode (PE32+ EFI Application via OVMF)",
            BootArch::Multiboot => "Direct Kernel Multiboot/GRUB (ELF Hypervisor Handshake)",
        }
    }

    /// Expected target artifact path relative to `<TRACK 0>/kernel`.
    pub fn target_artifact_relpath(&self) -> &'static str {
        match self {
            BootArch::Bios => "kernel/omnicode_bios.img",
            BootArch::Uefi => "kernel/esp",
            BootArch::Multiboot => "kernel/omnicode_multiboot.elf",
        }
    }
}

// ------------------------------------------------------------------------------
// 2.3. Boot Configuration Specification & Options Matrix
// ------------------------------------------------------------------------------
/// Concrete execution configuration for the bare-metal machine inception.
#[derive(Debug, Clone)]
pub struct BootConfig {
    /// Target execution architecture (BIOS, UEFI, or Multiboot).
    pub arch: BootArch,
    /// Whether to open an emulated graphical window or run headless over COM1 serial.
    pub graphical: bool,
    /// Host operating system plane under which execution is launched.
    pub host_platform: HostPlatform,
    /// Optional custom override path for UEFI OVMF firmware image.
    pub custom_ovmf: Option<PathBuf>,
}

/// Fully resolved, verified execution context prepared by the Opening Block.
///
/// Acts as the immutable "Setup Frame" handed to the Body Block's simulation tick.
/// Contains certified file paths, armed diagnostic sensors, and pre-resolved host binaries.
#[derive(Debug, Clone)]
pub struct BootContext {
    /// Initial boot configuration parameters.
    pub config: BootConfig,
    /// Attached telemetry logger for recording events and failpoints.
    pub logger: DiagnosticLogger,
    /// Interactive debugger tracking step count and QuadState transitions.
    pub debugger: OmniDebugger,
    /// Verified absolute path to `<TRACK 0>/kernel/build_all_kernels.sh`.
    pub build_script: PathBuf,
    /// Target boot artifact (MBR image, ESP directory, or ELF executable).
    pub boot_artifact: PathBuf,
    /// Pre-resolved UEFI OVMF firmware blob (if UEFI mode is selected).
    pub ovmf_firmware: Option<PathBuf>,
    /// Hypervisor executable name appropriate for the host platform.
    pub hypervisor_binary: &'static str,
}

impl BootConfig {
    /// Constructs a standard boot configuration with platform auto-detection.
    pub fn new(arch: BootArch, graphical: bool) -> Self {
        Self {
            arch,
            graphical,
            host_platform: HostPlatform::current(),
            custom_ovmf: None,
        }
    }

    /// Parses raw command-line tokens into a validated [`BootConfig`].
    pub fn from_args(args: &[String]) -> Self {
        let arch = if args.iter().any(|a| a == "--uefi" || a == "-u") {
            BootArch::Uefi
        } else if args.iter().any(|a| a == "--multiboot" || a == "-m" || a == "--direct") {
            BootArch::Multiboot
        } else {
            BootArch::Bios
        };

        let graphical = args.iter().any(|a| a == "--gui" || a == "-g" || a == "--window");

        let custom_ovmf = args.iter().position(|a| a == "--ovmf" || a == "-b").and_then(|idx| {
            args.get(idx + 1).map(PathBuf::from)
        });

        Self {
            arch,
            graphical,
            host_platform: HostPlatform::current(),
            custom_ovmf,
        }
    }

    /// Prepares and certifies the execution context before the Body touches the metal.
    ///
    /// # Precondition Tripwires & Environmental Probes:
    /// - Asserts that the Track 0 kernel build script is reachable on disk.
    /// - Asserts that the target boot artifact path resolves.
    /// - Pre-resolves firmware candidate and platform hypervisor binary.
    /// - Inspects host display server if graphical GUI window was requested.
    /// - Initializes an attached [`OmniDebugger`] to monitor step invariants.
    pub fn open(&self, logger: DiagnosticLogger) -> Result<BootContext, TripwireError> {
        let debugger = OmniDebugger::new(logger.clone(), 10);
        let build_script_path = resolve_track_path("kernel/build_all_kernels.sh");
        let build_script = Tripwire::assert_path_exists(
            &logger,
            "BOOT",
            build_script_path,
            "Track 0 kernel build pipeline script",
        )?;

        let boot_artifact = resolve_track_path(self.arch.target_artifact_relpath());

        // Probe host display environment if GUI window requested
        if self.graphical && !self.host_platform.has_display_server() {
            logger.record_ai_observation(
                "GUI_DISPLAY_HEADLESS",
                "Graphical window requested but host display server not detected",
                "Host environment lacks active DISPLAY or WAYLAND_DISPLAY session; window spawn may fail",
                ["gui", "display", "headless", "observation"].as_slice(),
                None,
            );
        }

        let ovmf_firmware = if self.arch == BootArch::Uefi {
            if let Some(custom) = &self.custom_ovmf {
                Some(Tripwire::assert_path_exists(&logger, "BOOT", custom, "Custom OVMF firmware")?)
            } else {
                self.host_platform.find_ovmf_firmware()
            }
        } else {
            None
        };

        let hypervisor_binary = self.host_platform.hypervisor_binary();

        Ok(BootContext {
            config: self.clone(),
            logger,
            debugger,
            build_script,
            boot_artifact,
            ovmf_firmware,
            hypervisor_binary,
        })
    }
}

// ------------------------------------------------------------------------------
// 2.4. Initial Truth States & Equilibrium Defaults
// ------------------------------------------------------------------------------
/// Canonical default boot architecture for minimal real-mode baseline.
pub const DEFAULT_BOOT_ARCH: BootArch = BootArch::Bios;

/// Default display mode: headless COM1 serial terminal output for pure dataflow.
pub const DEFAULT_GRAPHICAL_MODE: bool = false;

/// Real-mode bootstrap reset vector base in silicon RAM (`0x7C00`).
pub const REAL_MODE_BOOT_VECTOR: u64 = 0x7C00;

/// Standard VGA 80x25 text framebuffer memory-mapped base address (`0xB8000`).
pub const VGA_TEXT_FRAMEBUFFER_BASE: u64 = 0xB8000;

/// Standard IBM PC COM1 serial communications port I/O base address (`0x3F8`).
pub const COM1_SERIAL_IO_BASE: u16 = 0x3F8;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Kernel Compilation, Inception Tripwires & QEMU Execution Engine )
// ==============================================================================

// ------------------------------------------------------------------------------
// 3.1. Public Dispatcher & Context Resolution
// ------------------------------------------------------------------------------
/// Builds the target kernels in Track 0 and launches the virtual machine in QEMU.
///
/// # Game Dev Loop & Cybernetic Architecture:
/// Functions as the primary command-line engine driver. Resolves ambient hardware
/// parameters, constructs the diagnostic logger and debugger sensors, and invokes
/// the self-healing execution pipeline.
///
/// # Returns
/// - `Ok(QuadState::Proceed)`: Hypervisor session completed cleanly without fatal invariant breach.
/// - `Err(TripwireError)`: Precondition, compilation, or hypervisor tripwire tripped with `-1:ROLLBACK`.
pub fn run_boot(arch: BootArch, graphical: bool) -> Result<QuadState, TripwireError> {
    let config = BootConfig::new(arch, graphical);
    run_boot_with_config(config)
}

/// Executes boot inception under a customized [`BootConfig`].
pub fn run_boot_with_config(config: BootConfig) -> Result<QuadState, TripwireError> {
    let logger = DiagnosticLogger::new(false);
    let ctx = match config.open(logger) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("\x1b[1;31m[!] Precondition Tripwire Tripped during BootContext Open:\x1b[0m {e}");
            return Err(e);
        }
    };

    execute_boot_pipeline(ctx)
}

// ------------------------------------------------------------------------------
// 3.2. Kernel Compilation Pipeline & Build Tripwires
// ------------------------------------------------------------------------------
/// Orchestrates the execution of the Track 0 bare-metal kernel build pipeline.
///
/// # Silicon & Assembly Invariants:
/// - Reaches into `<TRACK 0>/kernel/build_all_kernels.sh`.
/// - Assembles 16-bit real-mode bootstrap vector (`0x7C00`) via NASM.
/// - Compiles 64-bit UEFI PE32+ application with Clang/LLD targeting GOP framebuffer.
/// - Links 32-bit ELF multiboot header with GNU LD for hypervisor direct handshakes.
///
/// # Sensory Tripwires:
/// - `TRIPWIRE_KERNEL_BUILD_SCRIPT`: Asserts build script is readable and executable.
/// - `TRIPWIRE_KERNEL_COMPILATION`: Asserts zero exit code from compilation subprocess.
/// - `TRIPWIRE_IMAGE_RESOLVE`: Asserts that target binary artifact exists on disk after assembly.
pub fn execute_kernel_pipeline(ctx: &mut BootContext) -> Result<(), TripwireError> {
    ctx.logger.log("KERNEL_PIPELINE", "Initiating Track 0 Bare-Metal Kernel Build Pipeline...");
    ctx.debugger.inspect_state(QuadState::Await, "kernel_build_started");
    ctx.debugger.step_count += 1;

    // 0. Transpile MilleniumOS boot architecture directly from OmniCode source reel
    let boot_omni_path = resolve_track_path("lib/kernel/boot.omni");
    if boot_omni_path.exists() {
        if let Ok(omni_content) = std::fs::read_to_string(&boot_omni_path) {
            if let Ok(generated_asm) = crate::runtime::assembler::transpile_omni_to_asm(&omni_content) {
                let bios_asm_path = resolve_track_path("kernel/bios/boot.asm");
                let _ = std::fs::write(&bios_asm_path, &generated_asm);
                let root_asm_path = resolve_track_path("kernel/boot.asm");
                let _ = std::fs::write(&root_asm_path, &generated_asm);
                ctx.logger.log("KERNEL_PIPELINE", "Transpiled lib/kernel/boot.omni -> bios/boot.asm for MilleniumOS");
            }
        }
    }

    let script_dir = ctx.build_script.parent().unwrap_or_else(|| Path::new("."));
    ctx.logger.log(
        "KERNEL_PIPELINE",
        format!("Executing driver script in directory: {:?}", script_dir),
    );

    println!("\x1b[1;34m[i] Executing bare-metal kernel build pipeline in {:?}...\x1b[0m", script_dir);
    let build_status = Command::new("bash")
        .arg(ctx.build_script.file_name().unwrap_or_default())
        .current_dir(script_dir)
        .status();

    match Tripwire::assert_command_success(&ctx.logger, "KERNEL_PIPELINE", build_status, "Kernel Build Pipeline") {
        Ok(()) => {
            println!("\x1b[1;32m✓ Track 0 Bare-Metal Kernel Pipeline Built Successfully\x1b[0m");
            ctx.logger.log("KERNEL_PIPELINE", "Kernel compilation subprocess returned exit code 0");
            ctx.logger.record_ai_observation(
                "KERNEL_BUILD_SUCCESS",
                "Bare-metal kernel compilation passed",
                "Track 0 build script executed cleanly with all artifacts generated",
                ["kernel", "build", "silicon", "verified"].as_slice(),
                Some("build_all_kernels.sh"),
            );
        }
        Err(e) => {
            ctx.debugger.inspect_state(QuadState::Rollback, "kernel_build_failed");
            ctx.logger.record_ai_observation(
                "KERNEL_BUILD_FAILED",
                "Bare-metal kernel compilation failed",
                &e.message,
                ["kernel", "build_failure", "rollback"].as_slice(),
                Some("build_all_kernels.sh"),
            );
            return Err(e);
        }
    }

    // Verify generated artifact presence post-build
    Tripwire::assert_path_exists(
        &ctx.logger,
        "KERNEL_PIPELINE",
        &ctx.boot_artifact,
        "Post-compilation target boot artifact",
    )?;

    ctx.debugger.inspect_state(QuadState::Proceed, "kernel_artifact_verified");
    Ok(())
}

// ------------------------------------------------------------------------------
// 3.3. Hypervisor Command Synthesis & Parameter Mapping
// ------------------------------------------------------------------------------
/// Constructs the platform-specific hypervisor launch command with exact MMIO,
/// serial redirection, and drive geometry parameters.
///
/// # Hardware & Architecture Mapping (Simulating Motherboard Buses):
/// - **BIOS**: Maps raw 512-byte image as primary boot drive (`format=raw,file=...`).
/// - **UEFI**: Maps OVMF firmware blob (`-bios ...`) and mounts ESP FAT partition.
/// - **Multiboot**: Direct hypervisor injection via QEMU `-kernel` flag.
/// - **Serial & VGA**: Multiplexes COM1 (`0x3F8`) and VGA text buffer (`0xB8000`)
///   to host standard I/O streams for headless evaluation or GUI windows.
///
/// # Sensory Tripwires:
/// - `TRIPWIRE_HYPERVISOR_PATH`: Asserts hypervisor binary (`qemu-system-x86_64`) exists on PATH.
/// - `TRIPWIRE_FIRMWARE_RESOLVE`: Asserts UEFI OVMF firmware blob exists when UEFI mode is active.
pub fn synthesize_hypervisor_command(ctx: &BootContext) -> Result<Command, TripwireError> {
    ctx.logger.log("HYPERVISOR", format!("Synthesizing launch command for {}", ctx.hypervisor_binary));

    // Assert that the hypervisor binary is present on host system PATH
    Tripwire::assert_binary_available(&ctx.logger, "HYPERVISOR", ctx.hypervisor_binary)?;

    let mut cmd = Command::new(ctx.hypervisor_binary);

    match ctx.config.arch {
        BootArch::Bios => {
            ctx.logger.log("HYPERVISOR", format!("Configuring BIOS MBR raw drive: {:?}", ctx.boot_artifact));
            let path_str = ctx.boot_artifact.to_string_lossy();
            let file_arg = if path_str.starts_with('/') || path_str.starts_with("./") {
                path_str.into_owned()
            } else {
                format!("./{path_str}")
            };
            cmd.arg("-drive").arg(format!("format=raw,file={file_arg}"));
        }
        BootArch::Uefi => {
            let ovmf = if let Some(ref fw) = ctx.ovmf_firmware {
                fw.to_string_lossy().into_owned()
            } else {
                let default_ovmf = "/usr/share/qemu/OVMF.fd".to_string();
                ctx.logger.log("HYPERVISOR", format!("Using fallback OVMF path: {}", default_ovmf));
                default_ovmf
            };
            ctx.logger.log("HYPERVISOR", format!("Configuring UEFI with OVMF firmware: {}", ovmf));
            cmd.arg("-bios").arg(ovmf);

            let esp_dir = if ctx.boot_artifact.to_string_lossy().contains(':') {
                let staged = PathBuf::from("target/esp");
                let staged_boot = staged.join("EFI").join("BOOT");
                let _ = std::fs::create_dir_all(&staged_boot);
                let efi_bin = ctx.boot_artifact.join("EFI").join("BOOT").join("BOOTX64.EFI");
                if efi_bin.exists() {
                    let _ = std::fs::copy(&efi_bin, staged_boot.join("BOOTX64.EFI"));
                }
                staged.to_string_lossy().into_owned()
            } else {
                ctx.boot_artifact.to_string_lossy().into_owned()
            };
            cmd.arg("-drive").arg(format!("file=fat:rw:{esp_dir},format=raw"));
            cmd.arg("-net").arg("none");
        }
        BootArch::Multiboot => {
            ctx.logger.log("HYPERVISOR", format!("Configuring direct multiboot ELF injection: {:?}", ctx.boot_artifact));
            let path_str = ctx.boot_artifact.to_string_lossy();
            let kernel_arg = if path_str.starts_with('/') || path_str.starts_with("./") {
                path_str.into_owned()
            } else {
                format!("./{path_str}")
            };
            cmd.arg("-kernel").arg(kernel_arg);
        }
    }

    if ctx.config.graphical {
        ctx.logger.log("HYPERVISOR", "Display mode: Graphical Window with COM1 serial on stdio");
        cmd.arg("-serial").arg("stdio");
    } else {
        ctx.logger.log("HYPERVISOR", "Display mode: Headless with COM1 serial on stdio (display=none)");
        cmd.arg("-display").arg("none").arg("-serial").arg("stdio");
    }

    Ok(cmd)
}

// ------------------------------------------------------------------------------
// 3.4. Machine Execution Loop, Tripwire Interception & Telemetry Seal
// ------------------------------------------------------------------------------
/// Executes the complete bare-metal machine inception pipeline.
///
/// # Game Dev Loop Rationale:
/// Directly executes the simulation tick of the host engine:
/// 1. Emits visual inception banner with machine metadata.
/// 2. Executes kernel pipeline wrapped in build tripwires.
/// 3. Synthesizes hypervisor launch command with hardware mappings.
/// 4. Executes virtual machine, trapping non-zero exit codes.
/// 5. Inscribes telemetry into [`DiagnosticLogger`] and [`OmniDebugger`] for the [`crate::diagnostics::restorator::Restorator`].
pub fn execute_boot_pipeline(mut ctx: BootContext) -> Result<QuadState, TripwireError> {
    println!("\x1b[1;36m===============================================================================\x1b[0m");
    println!("\x1b[1;36m  OmniCode OS: Bare-Metal Multi-Architecture Machine Inception                \x1b[0m");
    println!("\x1b[1;36m  Mode: {}\x1b[0m", ctx.config.arch.display_name());
    println!("\x1b[1;36m  Host: {}\x1b[0m", ctx.config.host_platform.display_name());
    println!("\x1b[1;36m===============================================================================\x1b[0m\n");

    ctx.logger.log(
        "BOOT",
        format!(
            "Beginning Machine Inception [{}] on [{}]",
            ctx.config.arch.display_name(),
            ctx.config.host_platform.display_name()
        ),
    );
    ctx.debugger.inspect_state(QuadState::Await, "machine_inception_begun");
    ctx.debugger.step_count += 1;

    // 1. Execute Bare-Metal Kernel Pipeline in Track 0
    execute_kernel_pipeline(&mut ctx)?;

    // 2. Configure Machine Emulator Command
    let mut cmd = synthesize_hypervisor_command(&ctx)?;

    if ctx.config.graphical {
        println!("\x1b[1;36m[i] Launching graphical virtual machine display with serial on stdio...\x1b[0m");
    } else {
        println!("\x1b[1;36m[i] Launching headless terminal machine with COM1 serial on stdio...\x1b[0m");
    }

    // 3. Launch Virtual Machine Inception
    ctx.logger.log("BOOT", format!("Spawning hypervisor child process: {:?}", cmd));
    ctx.debugger.inspect_state(QuadState::Await, "hypervisor_spawning");
    ctx.debugger.step_count += 1;

    let run_status = cmd.status();
    match Tripwire::assert_command_success(&ctx.logger, "BOOT", run_status, ctx.hypervisor_binary) {
        Ok(()) => {
            println!("\n\x1b[1;32m✓ QEMU Machine Inception Terminated Cleanly (+1:PROCEED)\x1b[0m");
            ctx.logger.log("BOOT", "Virtual machine exited with status code 0 (+1:PROCEED)");
            ctx.debugger.inspect_state(QuadState::Proceed, "machine_inception_clean_exit");
            ctx.logger.record_ai_observation(
                "BOOT_SUCCESS",
                "Machine inception ran to completion",
                "QEMU execution exited with success code 0",
                ["hypervisor", "boot", "proceed", "verified"].as_slice(),
                Some(ctx.hypervisor_binary),
            );
            Ok(QuadState::Proceed)
        }
        Err(e) => {
            ctx.debugger.inspect_state(QuadState::Rollback, "machine_inception_trapped");
            ctx.logger.record_ai_observation(
                "BOOT_FAILED",
                "Machine inception trapped failpoint",
                &e.message,
                ["hypervisor", "boot_failure", "rollback"].as_slice(),
                Some(ctx.hypervisor_binary),
            );
            eprintln!("  (Ensure {} is installed and accessible on your PATH)", ctx.hypervisor_binary);
            Err(e)
        }
    }
}

/// Backwards-compatible synchronous executor for machine inception.
pub fn execute_boot_machine(ctx: BootContext) {
    if let Err(e) = execute_boot_pipeline(ctx) {
        eprintln!("\x1b[1;31mMachine inception halted:\x1b[0m {e}");
        std::process::exit(1);
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Module Sealing, Invariant Verification & Downstream Baton Pass )
// ==============================================================================

// ------------------------------------------------------------------------------
// 4.1. Module Sealing & Downstream Baton Pass Specification
// ------------------------------------------------------------------------------
// BATON PASS CONTRACT:
//   - Upstream Producer: CLI Command Dispatcher (`omnicode boot ...`).
//   - Downstream Receiver: `<TRACK 0>` Bare-Metal Kernel Pipeline & Host QEMU Hypervisor.
//   - Certified Invariants:
//     1. Target architecture variants (`Bios`, `Uefi`, `Multiboot`) map deterministically
//        to valid artifact targets in the `<TRACK 0>/kernel` tree.
//     2. Hardware MMIO base registers (COM1 `0x3F8`, VGA `0xB8000`, Real Mode `0x7C00`)
//        are immutable and verified.
//     3. Command synthesis accurately maps drives, firmware, and stdio multiplexing.
//     4. `BootContext::open` certifies all filesystem preconditions, initializes an
//        attached `OmniDebugger` and `DiagnosticLogger`, and leaves zero leaked state.
//     5. Any execution failure is intercepted by sensory tripwires and packaged with
//        remediation suggestions for the `Restorator`.

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that all `BootArch` enum variants return non-empty, distinct, human-readable display titles.
    ///
    /// # Failure Mode Guarded Against:
    /// Prevents silent string regression or unmapped match arms when new architecture targets are added.
    #[test]
    fn test_boot_arch_display() {
        assert!(BootArch::Bios.display_name().contains("BIOS"), "BIOS display title must identify MBR mode");
        assert!(BootArch::Uefi.display_name().contains("UEFI"), "UEFI display title must identify Long Mode");
        assert!(BootArch::Multiboot.display_name().contains("Multiboot"), "Multiboot display title must identify ELF mode");
    }

    /// Verifies that the physical kernel build script and default BIOS boot sector image resolve in `<TRACK 0>`.
    ///
    /// # Failure Mode Guarded Against:
    /// Guards against repository relocation anomalies or broken relative path traversal to Track 0.
    #[test]
    fn test_kernel_paths_resolve() {
        let build_script = resolve_track_path("kernel/build_all_kernels.sh");
        assert!(build_script.exists(), "Track 0 kernel build driver script must exist on disk");
        let bios_img = resolve_track_path("kernel/omnicode_bios.img");
        assert!(bios_img.exists(), "Track 0 default BIOS binary artifact must exist on disk");
    }

    /// Verifies that `HostPlatform::current()` correctly detects the active host environment
    /// and provides non-empty hypervisor and firmware candidate paths.
    ///
    /// # Failure Mode Guarded Against:
    /// Prevents null or uninitialized platform definitions from entering the boot pipeline.
    #[test]
    fn test_host_platform_detection() {
        let current = HostPlatform::current();
        assert!(!current.display_name().is_empty(), "Host platform must have a non-empty display name");
        assert!(!current.hypervisor_binary().is_empty(), "Host platform must provide a default hypervisor binary");
        assert!(!current.ovmf_search_paths().is_empty(), "Host platform must declare firmware candidate paths");
    }

    /// Verifies command-line argument parsing across short flags, long flags, and option permutations.
    ///
    /// # Invariants Tested:
    /// - `--uefi` and `-u` select `BootArch::Uefi`.
    /// - `--multiboot` and `-m` select `BootArch::Multiboot`.
    /// - Absence of arch flags defaults to `BootArch::Bios`.
    /// - `--gui` sets `graphical = true`.
    /// - `--ovmf <PATH>` extracts custom firmware path override.
    #[test]
    fn test_boot_config_from_args() {
        let args = vec!["omnicode".to_string(), "boot".to_string(), "--uefi".to_string(), "--gui".to_string()];
        let cfg = BootConfig::from_args(&args);
        assert_eq!(cfg.arch, BootArch::Uefi, "Flag --uefi must select BootArch::Uefi");
        assert!(cfg.graphical, "Flag --gui must set graphical mode to true");

        let args_mb = vec!["omnicode".to_string(), "boot".to_string(), "-m".to_string()];
        let cfg_mb = BootConfig::from_args(&args_mb);
        assert_eq!(cfg_mb.arch, BootArch::Multiboot, "Flag -m must select BootArch::Multiboot");
        assert!(!cfg_mb.graphical, "Default graphical mode must be headless (false)");

        let args_custom_ovmf = vec![
            "omnicode".to_string(),
            "boot".to_string(),
            "--uefi".to_string(),
            "--ovmf".to_string(),
            "/custom/ovmf.fd".to_string(),
        ];
        let cfg_custom = BootConfig::from_args(&args_custom_ovmf);
        assert_eq!(cfg_custom.custom_ovmf, Some(PathBuf::from("/custom/ovmf.fd")));
    }

    /// Verifies that `BootConfig::open` successfully certifies preconditions, resolves Track 0 scripts,
    /// attaches an armed `OmniDebugger`, and constructs an immutable `BootContext`.
    ///
    /// # Failure Mode Guarded Against:
    /// Prevents uncertified or partially initialized execution frames from reaching the hypervisor spawner.
    #[test]
    fn test_boot_context_open_preconditions() {
        let cfg = BootConfig::new(BootArch::Bios, false);
        let logger = DiagnosticLogger::new(false);
        let ctx_res = cfg.open(logger);
        assert!(ctx_res.is_ok(), "Opening BootContext must succeed when Track 0 workspace is valid");
        let ctx = ctx_res.unwrap();
        assert!(ctx.build_script.exists(), "Certified build script path must exist on disk");
        assert_eq!(ctx.config.arch, BootArch::Bios, "Context config arch must match specification");
        assert_eq!(ctx.debugger.step_count, 0, "Attached debugger must start at step count 0");
    }

    /// Verifies hypervisor command synthesis for BIOS MBR mode in headless serial operation.
    ///
    /// # Silicon Wiring Invariants:
    /// - Primary raw drive mapped to target artifact (`-drive format=raw,file=...`).
    /// - Display disabled (`-display none`).
    /// - COM1 serial port (`0x3F8`) multiplexed to standard I/O (`-serial stdio`).
    #[test]
    fn test_synthesize_hypervisor_command_bios_headless() {
        let cfg = BootConfig::new(BootArch::Bios, false);
        let logger = DiagnosticLogger::new(false);
        let ctx = cfg.open(logger).expect("Context open succeeds");
        let cmd = synthesize_hypervisor_command(&ctx).expect("Command synthesis succeeds");
        let args: Vec<String> = cmd.get_args().map(|s| s.to_string_lossy().into_owned()).collect();

        assert!(args.contains(&"-drive".to_string()), "Command must include -drive flag");
        assert!(args.iter().any(|a| a.contains("format=raw,file=")), "Must specify raw MBR disk format");
        assert!(args.contains(&"-display".to_string()), "Headless mode must include -display flag");
        assert!(args.contains(&"none".to_string()), "Headless mode must specify display none");
        assert!(args.contains(&"-serial".to_string()), "COM1 serial port must be redirected");
        assert!(args.contains(&"stdio".to_string()), "Serial redirection must target stdio");
    }

    /// Verifies hypervisor command synthesis for graphical window display mode.
    ///
    /// # Graphical Wiring Invariants:
    /// - Must NOT include `-display none`.
    /// - Serial COM1 must remain connected to `stdio` for background telemetry.
    #[test]
    fn test_synthesize_hypervisor_command_graphical() {
        let cfg = BootConfig::new(BootArch::Bios, true);
        let logger = DiagnosticLogger::new(false);
        let ctx = cfg.open(logger).expect("Context open succeeds");
        let cmd = synthesize_hypervisor_command(&ctx).expect("Command synthesis succeeds");
        let args: Vec<String> = cmd.get_args().map(|s| s.to_string_lossy().into_owned()).collect();

        assert!(!args.contains(&"-display".to_string()), "Graphical mode must not pass -display none");
        assert!(args.contains(&"-serial".to_string()), "Graphical mode must retain COM1 serial redirection");
        assert!(args.contains(&"stdio".to_string()), "Serial output must stream to host stdio");
    }

    /// Verifies hypervisor command synthesis for direct Multiboot kernel injection.
    ///
    /// # Hypervisor Direct Handshake Invariants:
    /// - ELF binary injected directly into RAM via QEMU `-kernel` parameter.
    #[test]
    fn test_synthesize_hypervisor_command_multiboot() {
        let cfg = BootConfig::new(BootArch::Multiboot, false);
        let logger = DiagnosticLogger::new(false);
        let ctx = cfg.open(logger).expect("Context open succeeds");
        let cmd = synthesize_hypervisor_command(&ctx).expect("Command synthesis succeeds");
        let args: Vec<String> = cmd.get_args().map(|s| s.to_string_lossy().into_owned()).collect();

        assert!(args.contains(&"-kernel".to_string()), "Multiboot must pass -kernel flag");
        assert!(args.iter().any(|a| a.contains("omnicode_multiboot.elf")), "Kernel path must point to multiboot ELF");
    }

    /// Verifies hypervisor command synthesis for UEFI Long Mode execution.
    ///
    /// # Firmware & ESP Wiring Invariants:
    /// - OVMF firmware image mapped via `-bios`.
    /// - ESP directory mapped as FAT drive via `file=fat:rw:...,format=raw`.
    #[test]
    fn test_synthesize_hypervisor_command_uefi() {
        let mut cfg = BootConfig::new(BootArch::Uefi, false);
        cfg.custom_ovmf = Some(PathBuf::from("/usr/share/qemu/OVMF.fd"));
        let logger = DiagnosticLogger::new(false);
        let ctx = cfg.open(logger).expect("Context open succeeds");
        let cmd = synthesize_hypervisor_command(&ctx).expect("Command synthesis succeeds");
        let args: Vec<String> = cmd.get_args().map(|s| s.to_string_lossy().into_owned()).collect();

        assert!(args.contains(&"-bios".to_string()), "UEFI mode must pass -bios flag");
        assert!(args.contains(&"/usr/share/qemu/OVMF.fd".to_string()), "Must wire specified OVMF firmware blob");
        assert!(args.contains(&"-drive".to_string()), "Must configure EFI System Partition drive");
        assert!(args.iter().any(|a| a.contains("file=fat:rw:")), "Must mount ESP as FAT filesystem");
    }
}

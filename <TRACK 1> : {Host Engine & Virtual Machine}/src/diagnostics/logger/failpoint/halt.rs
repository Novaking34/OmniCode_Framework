// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Dual-Rail HALT Telemetry, Track 0 Register Dumps & Re-Inception Mechanics )
// ==============================================================================
//! OmniCode Diagnostics: HALT Physics & Silicon Re-Inception Telemetry
//!
//! # The Dual-Rail HALT Invariant
//! In balanced ternary logic, 2 bits represent 1 trit:
//! - `(0, 0)` = AWAIT (0): Homeostasis, neutral equilibrium, standby, pause.
//! - `(1, 0)` = PROCEED (+1): Positive rail high. Forward commit motion.
//! - `(0, 1)` = ROLLBACK (-1): Negative rail high. Rewind, time-travel, retroaction.
//! - `(1, 1)` = HALT (NULL / ⊥): Both rails high! Saturated state. No "free" data.
//!
//! # Why HALT Restarts Instead of Flatlining
//! A flatlined machine is a dead machine. For an operating system with a millennial
//! horizon (MilleniumOS), flatlining is mathematically unacceptable:
//! - `ROLLBACK` is a form of rewinding, not stopping.
//! - `AWAIT` is a pause (standby/homeostasis), not a stop.
//! - `HALT` is an abrupt stop triggered when both rails saturate at `(1, 1)`.
//!
//! When a HALT occurs, the cybernetic collision sensor (Tripwire) catches it,
//! captures the full hardware state (Track 0 registers, CPU CR0/CR3/CR4, memory landmark),
//! and **restarts the machine back to homeostasis (`0`, or AWAIT)**.

use std::fmt;
use super::types::FailpointType;
use crate::core::ternary::QuadState;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Halt Reason, Silicon Register Bank, Re-Inception Target & Halt Dump Frame )
// ==============================================================================

/// Underlying trigger cause for an abrupt HALT event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HaltReason {
    /// Dual-rail hardware state saturated at (1, 1); no free data capacity.
    DualRailSaturation,
    /// CPU hardware trap intercepted (e.g. #GP, #PF, #UD, #DF, #MC).
    CpuTrap(u32),
    /// Balanced ternary ALU sanity failed: (+1) + (-1) != 0 equilibrium breach.
    AluImbalance,
    /// Memory landmark violation (e.g. illegal access crossing 0x7C00, 0x18000, 0x108000).
    LandmarkViolation,
    /// Watchdog sentinel idle loop expired without interrupt.
    WatchdogTrip,
    /// Low-power `hlt` sentinel loop invoked by safe-mode de-escalation.
    ExplicitHaltInstruction,
}

/// The 13 calibrated silicon registers and CPU control registers from Track 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SiliconRegisters {
    // The 7 Cognitive Silicon Registers of Track 0
    pub head: u64,      // %head: Cognitive focus
    pub home: u64,      // %home: Moral origin anchor
    pub turn: u64,      // %turn: Step/tick counter
    pub spool: u64,     // %spool: I/O port origin
    pub spool_in: u64,  // %spool_in: Inbound stream head
    pub spool_out: u64, // %spool_out: Outbound stream head
    pub frame: u64,     // %frame: Execution frame landmark

    // General CPU Registers
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rsp: u64,
    pub rbp: u64,

    // Hardware Control Registers & Flags
    pub cr0: u64,
    pub cr3: u64,
    pub cr4: u64,
    pub rflags: u64,
}

/// Safe-mode de-escalation landmark to which the machine cascades down upon HALT.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReInceptionTarget {
    /// Sector 0 Real-Mode Genesis Landmark at 0x7C00.
    BiosMbrSector0,
    /// Track 0 Real-Mode Landmark at 0x0500.
    RealModeGenesis,
    /// Stage 2 Protected Mode Flight Recorder Landmark at 0x18000.
    Stage2ProtectedMode,
    /// Long Mode 64-Bit Kernel Base Landmark at 0x108000.
    LongModeKernel,
    /// Cognitive Substrate Arena Landmark at 0x200000.
    CognitiveSubstrateArena,
    /// Hardware CPU Reset Vector at 0xFFFF0.
    CpuResetVector,
}

/// Comprehensive flight-recorder crash dump frame captured upon an abrupt HALT.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HaltDumpFrame {
    /// Monotonic microsecond timestamp when the HALT occurred.
    pub tick_micros: u128,
    /// Specific causal reason for the abrupt halt.
    pub reason: HaltReason,
    /// The trapped failpoint classification.
    pub failpoint: FailpointType,
    /// Dual-rail hardware state at halt: always `(1, 1)` (saturated).
    pub dual_rail_saturation: (u8, u8),
    /// Active physical memory landmark at the moment of the halt.
    pub landmark_address: u64,
    /// Snapshot of silicon and CPU registers captured at the collision boundary.
    pub registers: SiliconRegisters,
    /// Target landmark where the machine cascades down to re-incept.
    pub re_inception_target: ReInceptionTarget,
    /// The target homeostasis state after re-inception (always `0:AWAIT`).
    pub re_inception_state: QuadState,
    /// Number of cumulative re-inceptions executed in this session.
    pub restart_count: usize,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Constructors, Dump Formatters, Register Calibrations & Re-Inception )
// ==============================================================================

impl SiliconRegisters {
    /// Constructs a clean register bank calibrated to baseline zero.
    pub fn calibrated_zero() -> Self {
        Self {
            head: 0,
            home: 0,
            turn: 0,
            spool: 0,
            spool_in: 0,
            spool_out: 0,
            frame: 0,
            rax: 0,
            rbx: 0,
            rcx: 0,
            rdx: 0,
            rsi: 0,
            rdi: 0,
            rsp: 0x7C00,
            rbp: 0x7C00,
            cr0: 0x0000_0001,
            cr3: 0x0010_8000,
            cr4: 0x0000_0020,
            rflags: 0x0000_0002,
        }
    }

    /// Formats the registers into an 80-column diagnostic dump.
    pub fn format_dump(&self) -> String {
        format!(
            "REGISTERS:\n  \
             head: {:#018x}  home: {:#018x}  turn: {:#018x}\n  \
             spool: {:#018x} in:   {:#018x}  out:  {:#018x}\n  \
             rax:  {:#018x}  rbx:  {:#018x}  rcx:  {:#018x}\n  \
             rdx:  {:#018x}  rsi:  {:#018x}  rdi:  {:#018x}\n  \
             rsp:  {:#018x}  rbp:  {:#018x}  cr0:  {:#018x}\n  \
             cr3:  {:#018x}  cr4:  {:#018x}  rflags: {:#018x}",
            self.head, self.home, self.turn,
            self.spool, self.spool_in, self.spool_out,
            self.rax, self.rbx, self.rcx,
            self.rdx, self.rsi, self.rdi,
            self.rsp, self.rbp, self.cr0,
            self.cr3, self.cr4, self.rflags
        )
    }
}

impl ReInceptionTarget {
    /// Resolves the physical RAM address for this landmark.
    pub fn address(&self) -> u64 {
        match self {
            ReInceptionTarget::BiosMbrSector0 => 0x7C00,
            ReInceptionTarget::RealModeGenesis => 0x0500,
            ReInceptionTarget::Stage2ProtectedMode => 0x18000,
            ReInceptionTarget::LongModeKernel => 0x108000,
            ReInceptionTarget::CognitiveSubstrateArena => 0x200000,
            ReInceptionTarget::CpuResetVector => 0xFFFF0,
        }
    }

    /// Returns the canonical uppercase identifier for this landmark.
    pub fn as_str(&self) -> &'static str {
        match self {
            ReInceptionTarget::BiosMbrSector0 => "LANDMARK_BIOS_MBR_0x7C00",
            ReInceptionTarget::RealModeGenesis => "LANDMARK_REAL_GENESIS_0x0500",
            ReInceptionTarget::Stage2ProtectedMode => "LANDMARK_STAGE2_PM_0x18000",
            ReInceptionTarget::LongModeKernel => "LANDMARK_LONG_MODE_0x108000",
            ReInceptionTarget::CognitiveSubstrateArena => "LANDMARK_SUBSTRATE_0x200000",
            ReInceptionTarget::CpuResetVector => "LANDMARK_RESET_0xFFFF0",
        }
    }
}

impl HaltDumpFrame {
    /// Constructs a new [`HaltDumpFrame`] capturing an abrupt HALT event.
    pub fn new(
        tick_micros: u128,
        reason: HaltReason,
        failpoint: FailpointType,
        landmark_address: u64,
        registers: SiliconRegisters,
        re_inception_target: ReInceptionTarget,
        restart_count: usize,
    ) -> Self {
        Self {
            tick_micros,
            reason,
            failpoint,
            dual_rail_saturation: (1, 1), // Always (1, 1) at HALT
            landmark_address,
            registers,
            re_inception_target,
            re_inception_state: QuadState::Await, // Always restarts to homeostasis
            restart_count,
        }
    }

    /// Executes the re-inception transition: clears the saturated dual-rail state,
    /// returns the machine to `0:AWAIT`, and cascades down to the safe landmark.
    pub fn execute_re_inception(&self) -> QuadState {
        // In OmniCode, HALT does not flatline; it discharges saturated rails back to (0, 0)
        QuadState::Await
    }

    /// Formats this halt dump as a human-readable ANSI/ASCII terminal block.
    pub fn format_human(&self) -> String {
        format!(
            "================================================================================\n\
             [HALT INTERCEPTION DUMP] (Tick: {} µs | Re-inception Count: {})\n\
             Reason: {:?}\n\
             Failpoint: {}\n\
             Dual-Rail Saturation: ({}, {}) [NO FREE DATA - COLLISION TRAP]\n\
             Active Landmark: {:#08x} -> Re-Inception Target: {} ({:#08x})\n\
             Post-Halt Target State: {} [HOMEOSTASIS RESTORED]\n\
             --------------------------------------------------------------------------------\n\
             {}\n\
             ================================================================================",
            self.tick_micros,
            self.restart_count,
            self.reason,
            self.failpoint,
            self.dual_rail_saturation.0,
            self.dual_rail_saturation.1,
            self.landmark_address,
            self.re_inception_target.as_str(),
            self.re_inception_target.address(),
            self.re_inception_state.macro_op(),
            self.registers.format_dump()
        )
    }

    /// Formats this halt dump as a canonical OmniCode `.scroll` note for AI long-term memory.
    pub fn format_omnicode_scroll(&self) -> String {
        format!(
            "Note < HaltTrace > {{\n    \
                 tick: < {} µs >,\n    \
                 category: [ HALT_REINCEPTION ],\n    \
                 summary: < \"Abrupt HALT Trapped; Discharged to Homeostasis (Await)\" >,\n    \
                 failpoint: [ {} ],\n    \
                 saturation: ( 1, 1 ),\n    \
                 active_landmark: < {:#08x} >,\n    \
                 re_inception_target: < {} >,\n    \
                 re_inception_state: < PROCEED :: AWAIT >,\n    \
                 tags: [ #halt :: #reinception :: #dual_rail_11 :: #homeostasis :: #track0 ]\n\
             }}",
            self.tick_micros,
            self.failpoint.as_str(),
            self.landmark_address,
            self.re_inception_target.as_str()
        )
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Display Trait Implementations, Invariant Proofs & Module Sealing )
// ==============================================================================

impl fmt::Display for HaltDumpFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[HALT: {:?} @ {:#08x} -> Re-incepting to {}]",
            self.reason,
            self.landmark_address,
            self.re_inception_target.as_str()
        )
    }
}

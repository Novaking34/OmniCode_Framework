// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Genesis Inception Telemetry & Silicon Baseline Tracking Structures )
// ==============================================================================
//! Genesis Inception Telemetry
//!
//! Encapsulates cold-silicon inception logs emitted by Track 0 (BIOS MBR,
//! Stage 2 PM, Long Mode, and UEFI) over physical RAM landmarks and COM1 UART.
//! Provides the primordial ground truth (State Omega_0) required by the AI Cortex
//! upon awakening.

/// Single bare-metal inception log frame captured during machine boot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenesisBootLog {
    /// Monotonic timestamp in microseconds when this inception log was ingested.
    pub timestamp_micros: u128,
    /// Physical RAM address or landmark (e.g. 0x0500, 0x7C00, 0x18000, 0x108000, 0x200000).
    pub landmark_address: u64,
    /// Boot tier or architecture (e.g. "BIOS_REAL_MODE", "STAGE2_PM", "LONG_MODE", "UEFI").
    pub tier: String,
    /// Raw telemetry string emitted over COM1 serial or memory buffer.
    pub raw_telemetry: String,
    /// Invariant subsystem verified (e.g. "ALU_BALANCED", "REGS_CALIBRATED", "A20_GATE", "MEMORY_MAP").
    pub invariant_subsystem: String,
    /// Whether the baseline invariant check passed cleanly.
    pub is_clean: bool,
}

impl GenesisBootLog {
    /// Constructs a new [`GenesisBootLog`].
    pub fn new(
        timestamp_micros: u128,
        landmark_address: u64,
        tier: impl Into<String>,
        raw_telemetry: impl Into<String>,
        invariant_subsystem: impl Into<String>,
        is_clean: bool,
    ) -> Self {
        Self {
            timestamp_micros,
            landmark_address,
            tier: tier.into(),
            raw_telemetry: raw_telemetry.into(),
            invariant_subsystem: invariant_subsystem.into(),
            is_clean,
        }
    }

    /// Formats the genesis log as a human-readable ANSI/ASCII diagnostic line.
    pub fn format_human(&self) -> String {
        let status = if self.is_clean { "OK" } else { "FAIL" };
        format!(
            "[{:>6} µs] [GENESIS: {:#08x}] [{:<12}] {:<20} => {}",
            self.timestamp_micros,
            self.landmark_address,
            self.tier,
            self.invariant_subsystem,
            status
        )
    }
}

/// Primordial Baseline State (Omega_0) captured at cold boot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenesisBaseline {
    /// Inception timestamp in microseconds when baseline was locked.
    pub locked_micros: u128,
    /// Chronological series of genesis boot logs from cold metal.
    pub boot_logs: Vec<GenesisBootLog>,
    /// Whether all genesis checks passed without error.
    pub is_clean: bool,
}

impl GenesisBaseline {
    /// Constructs a new [`GenesisBaseline`].
    pub fn new(locked_micros: u128, boot_logs: Vec<GenesisBootLog>) -> Self {
        let is_clean = boot_logs.iter().all(|l| l.is_clean);
        Self {
            locked_micros,
            boot_logs,
            is_clean,
        }
    }

    /// Returns the number of genesis inception records.
    pub fn len(&self) -> usize {
        self.boot_logs.len()
    }

    /// Returns `true` if no inception records exist.
    pub fn is_empty(&self) -> bool {
        self.boot_logs.is_empty()
    }
}

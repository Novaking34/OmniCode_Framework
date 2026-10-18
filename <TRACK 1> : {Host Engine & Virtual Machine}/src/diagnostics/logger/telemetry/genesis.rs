// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Genesis Inception Telemetry & Silicon Baseline Tracking Structures )
// ==============================================================================
//! OmniCode Telemetry: Genesis Inception Telemetry & Silicon Baseline ($\Omega_0$)
//!
//! # Architectural Scope & Purpose:
//! Encapsulates cold-silicon inception logs emitted by Track 0 (BIOS MBR,
//! Stage 2 PM, Long Mode, and UEFI) over physical RAM landmarks and COM1 UART serial.
//! Provides the primordial ground truth (State $\Omega_0$) required by the AI Cortex
//! upon machine awakening:
//! - Physical RAM Landmark Constants (`0x0500`, `0x7C00`, `0x18000`, `0x108000`, `0x200000`).
//! - [`GenesisBootLog`]: Single bare-metal inception frame capturing landmark, tier, and status.
//! - [`GenesisBaseline`]: Immutable snapshot of primordial hardware equilibrium ($\Omega_0$).
//!
//! # Scripture Witness:
//! - **Genesis 1:1 (KJV)**:
//!   "In the beginning God created the heaven and the earth."
//!   (The foundational truth that system state begins at a definite primordial inception point).
//! - **Proverbs 22:28 (KJV)**:
//!   "Remove not the ancient landmark, which thy fathers have set."
//!   (The inviolability of physical silicon RAM landmarks: 0x7C00, 0x18000, 0x108000, 0x200000).
//! - **Deuteronomy 29:29 (KJV)**:
//!   "The secret things belong unto the LORD our God: but those things which are revealed
//!    belong unto us and to our children for ever, that we may do all the words of this law."
//!   (The mandate to fully reveal and record hardware inception ground truth for AI reasoning).
//!
//! # Why Extensive Commenting Matters for AI Agents & Developers:
//! 1. **Machine Awakening Problem**: When the AI Cortex wakes up in Track 1, it has missed the first
//!    several milliseconds of bare-metal execution. The `GenesisBaseline` bridges this temporal gap.
//! 2. **Homeostatic Equilibrium Law**: The machine cannot safely power down or hibernate unless
//!    the state has returned to the pure, uncorrupted equilibrium certified at $\Omega_0$.
//! 3. **Ancient Landmarks**: Physical memory addresses are hardware contracts that guarantee
//!    isomorphic execution across QEMU emulation, hypervisors, and physical bare-metal silicon.
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Physical Silicon Landmark Constants, Boot Log & Baseline Definitions )
// ==============================================================================

/// Hardware RAM landmark for Real Mode Interrupt Vector Table (`0x00000500`).
pub const LANDMARK_BIOS_IVT: u64 = 0x00000500;

/// Hardware RAM landmark for Sector 0 BIOS MBR bootstrap reset vector (`0x00007C00`).
pub const LANDMARK_BIOS_MBR: u64 = 0x00007C00;

/// Hardware RAM landmark for Stage 2 Protected Mode Flight Recorder buffer (`0x00018000`).
pub const LANDMARK_STAGE2_PM: u64 = 0x00018000;

/// Hardware RAM landmark for Long Mode 64-bit 4-level page table directory (`0x00108000`).
pub const LANDMARK_PAGE_TABLE: u64 = 0x00108000;

/// Hardware RAM landmark for Cognitive Substrate CPI-SI execution arena (`0x00200000`).
pub const LANDMARK_COGNITIVE_SUBSTRATE: u64 = 0x00200000;

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

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Boot Log Methods, Serial Parsers & Baseline Homeostasis Audits )
// ==============================================================================

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

    /// Attempts to parse a raw COM1 serial output line into a [`GenesisBootLog`].
    ///
    /// Recognizes patterns emitted by Track 0:
    /// `[BOOT] 16-Bit Genesis: Sector 0 loaded at 0x7C00`
    /// `[ALU ] Balanced Ternary (+1)+(-1)=0 Eq: Certified`
    pub fn parse_serial_line(timestamp_micros: u128, line: &str) -> Option<Self> {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return None;
        }

        let is_clean = !trimmed.to_uppercase().contains("FAIL")
            && !trimmed.to_uppercase().contains("FAULT")
            && !trimmed.to_uppercase().contains("ERROR");

        let (landmark, tier, subsystem) = if trimmed.contains("0x7C00") || trimmed.contains("0x7c00") {
            (LANDMARK_BIOS_MBR, "BIOS_REAL_MODE", "SECTOR_ZERO")
        } else if trimmed.contains("0x18000") || trimmed.contains("Stage 2 PM") {
            (LANDMARK_STAGE2_PM, "STAGE2_PM", "A20_GATE")
        } else if trimmed.contains("0x108000") || trimmed.contains("Page Directory") {
            (LANDMARK_PAGE_TABLE, "LONG_MODE", "PAGE_TABLES")
        } else if trimmed.contains("0x200000") || trimmed.contains("Cognitive") {
            (LANDMARK_COGNITIVE_SUBSTRATE, "ARENA", "CPI_SI_SUBSTRATE")
        } else if trimmed.contains("ALU") {
            (LANDMARK_BIOS_MBR, "BIOS_REAL_MODE", "ALU_BALANCED")
        } else if trimmed.contains("REGS") {
            (LANDMARK_STAGE2_PM, "STAGE2_PM", "REGS_CALIBRATED")
        } else {
            (LANDMARK_BIOS_IVT, "BOOT", "GENERIC_INCEPTION")
        };

        Some(Self::new(
            timestamp_micros,
            landmark,
            tier,
            trimmed,
            subsystem,
            is_clean,
        ))
    }
}

impl GenesisBaseline {
    /// Constructs a new [`GenesisBaseline`].
    pub fn new(locked_micros: u128, boot_logs: Vec<GenesisBootLog>) -> Self {
        let is_clean = !boot_logs.is_empty() && boot_logs.iter().all(|l| l.is_clean);
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

    /// Finds all genesis logs matching a specific physical RAM landmark address.
    pub fn find_by_landmark(&self, landmark: u64) -> Vec<&GenesisBootLog> {
        self.boot_logs
            .iter()
            .filter(|l| l.landmark_address == landmark)
            .collect()
    }

    /// Finds the first genesis log matching a specific subsystem name.
    pub fn find_by_subsystem(&self, subsystem: &str) -> Option<&GenesisBootLog> {
        self.boot_logs
            .iter()
            .find(|l| l.invariant_subsystem.eq_ignore_ascii_case(subsystem))
    }

    /// Audits whether the baseline represents a pure, uncompromised homeostatic equilibrium ($\Omega_0$).
    pub fn audit_equilibrium(&self) -> Result<(), String> {
        if self.is_empty() {
            return Err("Genesis audit failed: No inception logs registered.".to_string());
        }

        if !self.is_clean {
            let failed: Vec<&str> = self
                .boot_logs
                .iter()
                .filter(|l| !l.is_clean)
                .map(|l| l.invariant_subsystem.as_str())
                .collect();
            return Err(format!(
                "Genesis audit failed: Baseline contains corrupted subsystems: {:?}",
                failed
            ));
        }

        Ok(())
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Test Verification Proofs & Landmark Invariant Tests )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ancient_landmarks_constants() {
        assert_eq!(LANDMARK_BIOS_IVT, 0x0500);
        assert_eq!(LANDMARK_BIOS_MBR, 0x7C00);
        assert_eq!(LANDMARK_STAGE2_PM, 0x18000);
        assert_eq!(LANDMARK_PAGE_TABLE, 0x108000);
        assert_eq!(LANDMARK_COGNITIVE_SUBSTRATE, 0x200000);
    }

    #[test]
    fn test_genesis_boot_log_formatting() {
        let log = GenesisBootLog::new(
            1405,
            LANDMARK_BIOS_MBR,
            "BIOS_REAL_MODE",
            "ALU_BALANCED Eq Certified",
            "ALU_BALANCED",
            true,
        );

        assert!(log.is_clean);
        let human = log.format_human();
        assert!(human.contains("[  1405 µs] [GENESIS: 0x007c00] [BIOS_REAL_MODE] ALU_BALANCED"));
        assert!(human.contains("=> OK"));
    }

    #[test]
    fn test_genesis_serial_line_parsing() {
        let log = GenesisBootLog::parse_serial_line(
            500,
            "[BOOT] 16-Bit Genesis: Sector 0 loaded at 0x7C00",
        );
        assert!(log.is_some());
        let l = log.unwrap();
        assert_eq!(l.landmark_address, LANDMARK_BIOS_MBR);
        assert_eq!(l.tier, "BIOS_REAL_MODE");
        assert!(l.is_clean);

        let failed_log = GenesisBootLog::parse_serial_line(
            600,
            "[ALU] Hardware Fault: Parity error detected",
        );
        assert!(failed_log.is_some());
        assert!(!failed_log.unwrap().is_clean);
    }

    #[test]
    fn test_genesis_baseline_and_equilibrium_audit() {
        let log1 = GenesisBootLog::new(
            100,
            LANDMARK_BIOS_MBR,
            "BIOS",
            "Reset OK",
            "SECTOR_ZERO",
            true,
        );
        let log2 = GenesisBootLog::new(
            200,
            LANDMARK_STAGE2_PM,
            "STAGE2_PM",
            "A20 Gate Active",
            "A20_GATE",
            true,
        );

        let baseline = GenesisBaseline::new(250, vec![log1, log2]);
        assert_eq!(baseline.len(), 2);
        assert!(!baseline.is_empty());
        assert!(baseline.is_clean);
        assert!(baseline.audit_equilibrium().is_ok());

        let mbr_logs = baseline.find_by_landmark(LANDMARK_BIOS_MBR);
        assert_eq!(mbr_logs.len(), 1);

        let a20 = baseline.find_by_subsystem("A20_GATE");
        assert!(a20.is_some());
        assert_eq!(a20.unwrap().landmark_address, LANDMARK_STAGE2_PM);

        // Corrupted baseline fails audit
        let corrupted_log = GenesisBootLog::new(
            300,
            LANDMARK_PAGE_TABLE,
            "PAGE",
            "Paging Error",
            "PAGING",
            false,
        );
        let dirty_baseline = GenesisBaseline::new(350, vec![corrupted_log]);
        assert!(!dirty_baseline.is_clean);
        assert!(dirty_baseline.audit_equilibrium().is_err());
    }
}

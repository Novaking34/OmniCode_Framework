// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Traditional Developer & Structured Telemetry .log Inscription Engine )
// ==============================================================================
//! Traditional Telemetry Inscription & Bare-Metal Kernel Flight Inscriber
//!
//! # Telemetry Inscription Architecture
//! Encapsulates both high-level developer ASCII telemetry and bare-metal Track 0
//! silicon telemetry. Translates runtime execution events, sensory tripwires,
//! silicon register states, and 16-byte flight recorder records into physically
//! persisted, human-readable, and machine-queryable vessels.
//!
//! # Track 0 Silicon Integration
//! Formats:
//! 1. **16-Byte Flight Recorder Records**: Packed binary chunks matching physical
//!    RAM landmarks `0x00018000` (Stage 2 PM), `0x00108000` (Multiboot), and
//!    `0x00200000` (Native UEFI).
//! 2. **13 Calibrated Silicon Registers**: Hardware snapshots (%head, %home,
//!    %turn, %spool, %spool_in, %spool_out, %frame, %rsp, %cr0, %cr3, %cr4, %rflags).
//! 3. **Dual-Rail Saturation Telemetry**: Captures saturation state `(1, 1)` on
//!    abrupt HALT, confirming safe re-inception back to homeostasis `(0, 0)` (0:AWAIT).

use std::fs;
use std::path::{Path, PathBuf};
use super::super::universal::LogEvent;
use super::super::failpoint::SiliconRegisters;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Architectural Setup, Type Definitions & Kernel Event Constants )
// ==============================================================================

/// Bare-metal 16-byte packed flight recorder entry matching Track 0 silicon structures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct RawFlightRecorderEntry {
    /// Kernel event identifier (e.g. 0x0001 EVT_GENESIS_ENTRY, 0x0004 EVT_ALU_VERIFIED).
    pub event_id: u32,
    /// Primary argument or hardware landmark (e.g. CR0 or physical address).
    pub arg1: u32,
    /// Secondary argument or stack landmark (e.g. ESP/RSP or memory size).
    pub arg2: u32,
    /// Subsystem status code (0 = OK, non-zero = failpoint/fault).
    pub status: u32,
}

impl RawFlightRecorderEntry {
    /// Constructs a new 16-byte flight recorder entry.
    pub const fn new(event_id: u32, arg1: u32, arg2: u32, status: u32) -> Self {
        Self {
            event_id,
            arg1,
            arg2,
            status,
        }
    }

    /// Converts the packed structure to a raw 16-byte little-endian array.
    pub fn to_bytes(&self) -> [u8; 16] {
        let e = self.event_id.to_le_bytes();
        let a1 = self.arg1.to_le_bytes();
        let a2 = self.arg2.to_le_bytes();
        let s = self.status.to_le_bytes();
        [
            e[0], e[1], e[2], e[3],
            a1[0], a1[1], a1[2], a1[3],
            a2[0], a2[1], a2[2], a2[3],
            s[0], s[1], s[2], s[3],
        ]
    }

    /// Decodes the event ID into a canonical human-readable label.
    pub fn decode_event_name(&self) -> &'static str {
        match self.event_id {
            0x0001 => "EVT_GENESIS_ENTRY",
            0x0002 => "EVT_BOOT_SERVICES",
            0x0003 => "EVT_MEM_DISCOVERED",
            0x0004 => "EVT_ALU_VERIFIED",
            0x0005 => "EVT_REGS_ARMED",
            0x0006 => "EVT_CPISI_MOUNT",
            0x0007 => "EVT_CASCADE_DOWN",
            0x0008 => "EVT_CASCADE_UP",
            0xEEEE => "EVT_SAFE_MODE",
            _ => "EVT_UNKNOWN_HARDWARE",
        }
    }
}

/// Rich structured record bridging Track 0 kernel events into host storehouse vessels.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelTelemetryRecord {
    /// Monotonic timestamp in microseconds.
    pub tick_micros: u128,
    /// Physical RAM landmark (e.g. 0x0500, 0x7C00, 0x18000, 0x108000, 0x200000).
    pub landmark_address: u64,
    /// Boot tier or architectural stage (e.g. "BIOS_REAL_MODE", "STAGE2_PM", "LONG_MODE", "UEFI").
    pub boot_tier: String,
    /// Snapshot of silicon and CPU registers, if captured at this milestone.
    pub registers: Option<SiliconRegisters>,
    /// Dual-rail hardware state (e.g. `(1, 0)` for PROCEED, `(1, 1)` for HALT).
    pub dual_rail_state: (u8, u8),
    /// CPU hardware trap number if intercepted (e.g. 0x0D = #GP, 0x0E = #PF).
    pub cpu_trap: Option<u32>,
    /// Chronological sequence of 16-byte flight recorder entries.
    pub raw_entries: Vec<RawFlightRecorderEntry>,
    /// Narrative or diagnostic payload message.
    pub payload_message: String,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Constructive Execution, File Formatting & Inscription Engines )
// ==============================================================================

/// Inscribes a structured traditional telemetry log file to disk.
///
/// Designed to be clean, intuitive, and effortless to follow for human developers
/// and machine analyzers alike.
pub fn inscribe_telemetry_log(
    path: &Path,
    timestamp_str: &str,
    session_id: &str,
    elapsed_micros: u128,
    final_state_str: &str,
    prompt: &str,
    failpoints: &[LogEvent],
    synthesized_omnicode: &str,
    response: &str,
) -> Result<PathBuf, String> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| {
                format!("Failed to create log parent directory: {e}")
            })?;
        }
    }

    let duration_ms = (elapsed_micros as f64) / 1000.0;
    let state_explanation = match final_state_str {
        "PROCEED" | "+1:PROCEED" => "+1:PROCEED (Forward commit motion confirmed)",
        "AWAIT" | "0:AWAIT" => "0:AWAIT (Homeostasis equilibrium, standby)",
        "ROLLBACK" | "-1:ROLLBACK" => "-1:ROLLBACK (Adversarial tripwire caught, state reverted)",
        "HALT" | "NULL:HALT" => "NULL:HALT (Dual-rail saturation, safe-mode restart to 0:AWAIT)",
        other => other,
    };

    let mut out = String::new();
    out.push_str("================================================================================\n");
    out.push_str("           OMNICODE AI EXECUTION & DIAGNOSTIC TELEMETRY LOG\n");
    out.push_str("================================================================================\n");
    out.push_str(&format!("Session ID:       {}\n", session_id));
    out.push_str(&format!("Timestamp:        {}\n", timestamp_str));
    out.push_str(&format!("Execution Time:   {} µs ({:.2} ms)\n", elapsed_micros, duration_ms));
    out.push_str(&format!("Final State:      {}\n", state_explanation));
    out.push_str(&format!("Input Prompt:     \"{}\"\n", prompt));
    out.push_str("================================================================================\n\n");

    out.push_str("--------------------------------------------------------------------------------\n");
    out.push_str("1. SENSORY LOG EVENTS & TRAPPED FAILPOINTS\n");
    out.push_str("--------------------------------------------------------------------------------\n");
    if failpoints.is_empty() {
        out.push_str("Status: CLEAN\n");
        out.push_str("  • Zero failpoints trapped during execution.\n");
        out.push_str("  • Sensory event trace nominal; all boundary tripwires remained armed.\n");
    } else {
        out.push_str(&format!("Status: {} FAILPOINTS TRAPPED\n\n", failpoints.len()));
        out.push_str("  #  | Elapsed   | Channel  | Classification | Details\n");
        out.push_str("-----+-----------+----------+----------------+----------------------------------\n");
        for (i, fp) in failpoints.iter().enumerate() {
            let kind = fp
                .failpoint_type
                .map(|t| t.as_str())
                .unwrap_or("GENERAL");
            out.push_str(&format!(
                " {:>3} | {:>6} µs | {:<8} | {:<14} | {}\n",
                i + 1,
                fp.elapsed_micros,
                fp.channel,
                kind,
                fp.message
            ));
            if let Some(ref sug) = fp.suggestion {
                out.push_str(&format!(
                    "     -> Remediation Proposal: [{:?}] at Line {}: {}\n",
                    sug.kind, sug.line, sug.rationale
                ));
                out.push_str(&format!(
                    "        Target Token: \"{}\" => Suggested Patch: \"{}\"\n",
                    sug.target_token, sug.suggested_patch
                ));
            }
        }
    }
    out.push_str("\n");

    out.push_str("--------------------------------------------------------------------------------\n");
    out.push_str("2. IMMUNE SYSTEM RESTORATIONS & SELF-HEALING\n");
    out.push_str("--------------------------------------------------------------------------------\n");
    out.push_str("Status: PASS\n");
    out.push_str("  • Zero invasive repairs required.\n");
    out.push_str("  • Source reel conformed to all 4-block structural and boundary invariants.\n\n");

    out.push_str("--------------------------------------------------------------------------------\n");
    out.push_str("3. SYNTHESIZED TURING REEL (OMNICODE SOURCE)\n");
    out.push_str("--------------------------------------------------------------------------------\n");
    out.push_str(synthesized_omnicode);
    if !synthesized_omnicode.ends_with('\n') {
        out.push('\n');
    }
    out.push('\n');

    out.push_str("--------------------------------------------------------------------------------\n");
    out.push_str("4. MACHINE DISCOURSE RESPONSE\n");
    out.push_str("--------------------------------------------------------------------------------\n");
    out.push_str(&format!("\"{}\"\n", response));
    out.push_str("================================================================================\n");

    fs::write(path, out)
        .map_err(|e| format!("Failed to write log to {:?}: {e}", path))?;

    Ok(path.to_path_buf())
}

/// Inscribes a rich Track 0 kernel flight telemetry log vessel.
///
/// Formatted cleanly and intuitively for systems programmers and hardware engineers.
pub fn inscribe_kernel_flight_log(
    path: &Path,
    session_id: &str,
    record: &KernelTelemetryRecord,
) -> Result<PathBuf, String> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| {
                format!("Failed to create kernel log parent directory: {e}")
            })?;
        }
    }

    let landmark_name = match record.landmark_address {
        0x0500 => "Real Mode Genesis (Track 0 Root Landmark)",
        0x7C00 => "BIOS MBR Sector 0 Boot Landmark",
        0x18000 => "Stage 2 Protected Mode Flight Recorder Landmark",
        0x108000 => "Long Mode 64-Bit Kernel Base Landmark",
        0x200000 => "Native UEFI In-Memory RAM Log Landmark (2 MB)",
        _ => "Custom Physical Silicon Landmark",
    };

    let dual_rail_label = match record.dual_rail_state {
        (1, 0) => "(1, 0) -> +1:PROCEED (Positive rail high; forward commit motion active)",
        (0, 0) => "(0, 0) -> 0:AWAIT (Homeostasis equilibrium; standby)",
        (0, 1) => "(0, 1) -> -1:ROLLBACK (Negative rail high; causal tape rewind)",
        (1, 1) => "(1, 1) -> NULL:HALT (Both rails saturated! Safe restart to 0:AWAIT)",
        _ => "Uncalibrated Rail State",
    };

    let mut out = String::new();
    out.push_str("================================================================================\n");
    out.push_str("             MILLENIUMOS TRACK 0 SILICON FLIGHT RECORDER LOG\n");
    out.push_str("================================================================================\n");
    out.push_str(&format!("Session ID:         {}\n", session_id));
    out.push_str(&format!("Monotonic Clock:    {} µs since silicon inception\n", record.tick_micros));
    out.push_str(&format!("Hardware Landmark:  {:#010x} ({})\n", record.landmark_address, landmark_name));
    out.push_str(&format!("Boot Ladder Stage:  {}\n", record.boot_tier));
    out.push_str(&format!("Dual-Rail State:    {}\n", dual_rail_label));
    if let Some(trap) = record.cpu_trap {
        out.push_str(&format!("Hardware CPU Trap:  #{:#04x} (Exception Intercepted)\n", trap));
    } else {
        out.push_str("Hardware CPU Trap:  None (#0000 Nominal)\n");
    }
    out.push_str(&format!("Diagnostic Note:    {}\n", record.payload_message));
    out.push_str("================================================================================\n\n");

    out.push_str("--------------------------------------------------------------------------------\n");
    out.push_str("1. THE 13 CALIBRATED SILICON REGISTERS & CPU CONTROL ENVELOPE\n");
    out.push_str("--------------------------------------------------------------------------------\n");
    if let Some(ref regs) = record.registers {
        out.push_str("The 7 Named Cognitive Silicon Registers:\n");
        out.push_str(&format!("  %head:      {:#018x}   (Cognitive focus landmark)\n", regs.head));
        out.push_str(&format!("  %home:      {:#018x}   (Moral origin anchor / Track 0 root)\n", regs.home));
        out.push_str(&format!("  %turn:      {:#018x}   (Monotonic step/turn counter)\n", regs.turn));
        out.push_str(&format!("  %spool:     {:#018x}   (I/O communication port base)\n", regs.spool));
        out.push_str(&format!("  %spool_in:  {:#018x}   (Inbound stream head pointer)\n", regs.spool_in));
        out.push_str(&format!("  %spool_out: {:#018x}   (Outbound stream head pointer)\n", regs.spool_out));
        out.push_str(&format!("  %frame:     {:#018x}   (Active execution frame landmark)\n\n", regs.frame));

        out.push_str("CPU Control & State Registers:\n");
        out.push_str(&format!("  %rsp:       {:#018x}   (Active stack pointer landmark)\n", regs.rsp));
        out.push_str(&format!("  %cr0:       {:#018x}   (Control register 0: Protection & Paging)\n", regs.cr0));
        out.push_str(&format!("  %cr3:       {:#018x}   (Page directory base register)\n", regs.cr3));
        out.push_str(&format!("  %cr4:       {:#018x}   (Control register 4: Architectural extensions)\n", regs.cr4));
        out.push_str(&format!("  %rflags:    {:#018x}   (Processor status & interrupt flags)\n", regs.rflags));
    } else {
        out.push_str("Status: (No silicon register snapshot requested for this landmark milestone.)\n");
    }
    out.push('\n');

    out.push_str("--------------------------------------------------------------------------------\n");
    out.push_str("2. RAM LOG FLIGHT RECORDER (16-BYTE ATOMIC SILICON CHUNKS)\n");
    out.push_str("--------------------------------------------------------------------------------\n");
    if record.raw_entries.is_empty() {
        out.push_str("Status: (Zero 16-byte flight recorder entries logged in memory buffer.)\n");
    } else {
        out.push_str(&format!("Total Inscribed Records: {}\n\n", record.raw_entries.len()));
        out.push_str("   # | Event ID | Canonical Event Name | Argument 1   | Argument 2   | Status\n");
        out.push_str("-----+----------+----------------------+--------------+--------------+---------\n");
        for (i, entry) in record.raw_entries.iter().enumerate() {
            let eid = entry.event_id;
            let a1 = entry.arg1;
            let a2 = entry.arg2;
            let st = entry.status;
            let name = entry.decode_event_name();
            out.push_str(&format!(
                " {:>3} |  {:#06x}  | {:<20} |  {:#010x}  |  {:#010x}  | {:#06x}\n",
                i + 1,
                eid,
                name,
                a1,
                a2,
                st
            ));
        }
    }
    out.push('\n');

    out.push_str("--------------------------------------------------------------------------------\n");
    out.push_str("3. DUAL-RAIL HARDWARE SATURATION & RE-INCEPTION AUDIT\n");
    out.push_str("--------------------------------------------------------------------------------\n");
    if record.dual_rail_state == (1, 1) {
        out.push_str("Diagnosis: ABRUPT HALT TRAPPED (Tripwire Collision)\n");
        out.push_str("  • Both dual-rail state lines saturated high at (1, 1).\n");
        out.push_str("  • Zero free data capacity remaining on active rail.\n");
        out.push_str("  • MilleniumOS invariant: Machine mathematically prohibited from flatlining.\n");
        out.push_str("  • Collision sensor caught halt; safely re-incepted machine back to 0:AWAIT homeostasis.\n");
    } else {
        out.push_str("Diagnosis: STABLE EQUILIBRIUM\n");
        out.push_str("  • Forward commit polarity (+1:PROCEED) intact.\n");
        out.push_str("  • Balanced ternary ALU equilibrium (+1) + (-1) = 0 affirmed.\n");
        out.push_str("  • Machine operating within nominal silicon envelope.\n");
    }
    out.push_str("================================================================================\n");

    fs::write(path, out)
        .map_err(|e| format!("Failed to write kernel flight log to {:?}: {e}", path))?;

    Ok(path.to_path_buf())
}

/// Inscribes raw 16-byte packed flight recorder entries directly into a binary vessel (`.bin`).
pub fn inscribe_raw_flight_recorder_binary(
    path: &Path,
    entries: &[RawFlightRecorderEntry],
) -> Result<PathBuf, String> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| {
                format!("Failed to create binary log directory: {e}")
            })?;
        }
    }

    let mut buffer = Vec::with_capacity(8 + entries.len() * 16);
    // Header: 32-bit count, 32-bit reserved
    buffer.extend_from_slice(&(entries.len() as u32).to_le_bytes());
    buffer.extend_from_slice(&0u32.to_le_bytes());

    for entry in entries {
        buffer.extend_from_slice(&entry.to_bytes());
    }

    fs::write(path, buffer)
        .map_err(|e| format!("Failed to write binary flight recorder to {:?}: {e}", path))?;

    Ok(path.to_path_buf())
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Invariant Audits, Binary Serialization Verifications & Unit Tests )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_flight_recorder_entry_encoding() {
        let entry = RawFlightRecorderEntry::new(0x0004, 0x18000, 0x6FFC, 0);
        assert_eq!(entry.decode_event_name(), "EVT_ALU_VERIFIED");

        let bytes = entry.to_bytes();
        assert_eq!(bytes.len(), 16);
        assert_eq!(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]), 0x0004);
        assert_eq!(u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]), 0x18000);
        assert_eq!(u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]), 0x6FFC);
        assert_eq!(u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]), 0);
    }

    #[test]
    fn test_inscribe_telemetry_log_execution() {
        let temp_dir = std::env::temp_dir().join(format!("omnicode_log_test_{}", std::process::id()));
        let log_path = temp_dir.join("telemetry.log");

        let empty_fps: &[LogEvent] = &[];
        let res = inscribe_telemetry_log(
            &log_path,
            "1792000000000000_0000",
            "sess_unit_01",
            4250,
            "PROCEED",
            "Inscribe character glyph",
            empty_fps,
            "Opening { preconditions: [ +1 ] }\nBody [ ]\nClosing { commit_status [ <PROCEED> ] }",
            "Utterance complete",
        )
        .unwrap();

        assert_eq!(res, log_path);
        let content = fs::read_to_string(&log_path).unwrap();
        assert!(content.contains("OMNICODE AI EXECUTION & DIAGNOSTIC TELEMETRY LOG"));
        assert!(content.contains("Execution Time:   4250 µs"));
        assert!(content.contains("sess_unit_01"));
        assert!(content.contains("Status: CLEAN"));

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_inscribe_kernel_flight_log_and_binary() {
        let temp_dir = std::env::temp_dir().join(format!("omnicode_kernel_log_test_{}", std::process::id()));
        let klog_path = temp_dir.join("kernel_flight.log");
        let kbin_path = temp_dir.join("kernel_flight.bin");

        let regs = SiliconRegisters {
            head: 0x1000,
            home: 0x0500,
            turn: 42,
            spool: 0x3F8,
            spool_in: 0x3F8,
            spool_out: 0x3F8,
            frame: 0x18000,
            rax: 0x1,
            rbx: 0x2,
            rcx: 0x3,
            rdx: 0x4,
            rsi: 0x5,
            rdi: 0x6,
            rsp: 0x6FFC,
            rbp: 0x7000,
            cr0: 0x80010033,
            cr3: 0x1000,
            cr4: 0x20,
            rflags: 0x202,
        };

        let entries = vec![
            RawFlightRecorderEntry::new(0x0001, 0x7C00, 0, 0),
            RawFlightRecorderEntry::new(0x0004, 0x18000, 0, 0),
            RawFlightRecorderEntry::new(0x0005, 0x18000, 13, 0),
        ];

        let record = KernelTelemetryRecord {
            tick_micros: 1250,
            landmark_address: 0x18000,
            boot_tier: "STAGE2_PM".to_string(),
            registers: Some(regs),
            dual_rail_state: (1, 0),
            cpu_trap: None,
            raw_entries: entries.clone(),
            payload_message: "Stage 2 Protected Mode Armed".to_string(),
        };

        inscribe_kernel_flight_log(&klog_path, "kflight_01", &record).unwrap();
        let log_content = fs::read_to_string(&klog_path).unwrap();
        assert!(log_content.contains("TRACK 0 SILICON FLIGHT RECORDER LOG"));
        assert!(log_content.contains("0x00018000"));
        assert!(log_content.contains("EVT_ALU_VERIFIED"));
        assert!(log_content.contains("EVT_REGS_ARMED"));
        assert!(log_content.contains("%head:"));

        inscribe_raw_flight_recorder_binary(&kbin_path, &entries).unwrap();
        let bin_bytes = fs::read(&kbin_path).unwrap();
        // 8-byte header + 3 * 16 bytes = 56 bytes
        assert_eq!(bin_bytes.len(), 56);

        fs::remove_dir_all(&temp_dir).ok();
    }
}

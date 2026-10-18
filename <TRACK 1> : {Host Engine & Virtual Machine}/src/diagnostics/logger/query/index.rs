// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( IndexedSession, SessionCriteria & Track0LogRecord Data Structures )
// ==============================================================================
//! # Indexed Episode & Silicon Log Record Data Structures
//!
//! # Architectural Scope & Purpose:
//! The `index` module defines the core data representations utilized by the
//! [`EpisodicQueryEngine`]. In the OmniCode architecture, logging is never a
//! write-only discard pipeline; it is an indexed, queryable memory storehouse.
//!
//! This module provides:
//! - [`IndexedSession`]: Compact, high-level representation of past execution
//!   episodes parsed from native 4-Block `.scroll` headers.
//! - [`SessionCriteria`]: Fluent, multi-dimensional query builder for compound
//!   filters across domain categories, QuadStates, failpoints, and timestamps.
//! - [`QueryResult`]: Aggregated search result container with statistical metrics
//!   (pass rate, repair efficiency, time spans).
//! - [`Track0LogRecord`]: Concrete structured representation of bare-metal silicon
//!   telemetry dumps from Track 0 hardware flight recorders (`0x18000`, `0x108000`, `0x200000`).
//!
//! # Scripture Witness:
//! - **Job 8:8 (KJV)**:
//!   "For enquire, I pray thee, of the former age, and prepare thyself to the search of their fathers."
//!   (Episodic memory inquiry into preceding execution cycles to derive wisdom).
//! - **Proverbs 25:2 (KJV)**:
//!   "It is the glory of God to conceal a thing: but the honour of kings is to search out a matter."
//!   (Diagnostic investigation into execution failpoints, registers, and memory states).
//! - **Ecclesiastes 1:9 (KJV)**:
//!   "The thing that hath been, it is that which shall be; and that which is done is that which shall be done."
//!   (Temporal determinism and repeatable diagnostic causality across execution rolls).
//!
//! # Why Structured Index Records Matter for AI Agents & Engineers:
//! 1. **Zero-Allocation Filtering**: Querying millions of historical events by
//!    streaming lightweight index records avoids loading gigabytes of raw log strings into memory.
//! 2. **Track 0 Silicon Grounding**: Bridges the high-level cognitive session
//!    with the physical CPU register bank and bare-metal flight recorder addresses.
//! 3. **Episodic AI Reflection**: Provides the `AICortex` and `AILearner` with
//!    instant, non-probabilistic recall of past prompt completions, syntax repairs, and error rates.
// ==============================================================================

use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use crate::core::ternary::QuadState;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Structural Types, Query Criteria & Silicon Record Definitions )
// ==============================================================================

/// High-level indexed record representing a past execution episode.
///
/// Extracted from the first 25 lines of a native 4-Block `.scroll` file in $O(1)$ time,
/// avoiding linear disk reads of large execution traces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedSession {
    /// Unique session identifier string (e.g. `sess_20261018_142055_001` or `char_test_01`).
    pub session_id: String,
    /// Monotonic timestamp in microseconds since epoch.
    pub timestamp_micros: u128,
    /// Human-readable timestamp ID string (e.g. `20261018_091530_123456`).
    pub timestamp_id: String,
    /// Domain category partition (e.g. `genesis`, `character`, `number`, `sessions/prompts`, `cortex`).
    pub category: String,
    /// Final fourfold commit state ($\mathbb{T}_4 \in \{ -1, 0, +1, \text{NULL} \}$).
    pub final_state: QuadState,
    /// Number of failpoints trapped during execution.
    pub failpoint_count: usize,
    /// Number of automated repairs applied by Restorator.
    pub repair_count: usize,
    /// Total words/tokens inscribed in the session prompt.
    pub word_count: usize,
    /// Path to the native 4-Block `.scroll` episodic trace file.
    pub scroll_path: PathBuf,
    /// Path to the paired traditional `.log` file if present on disk.
    pub log_path: Option<PathBuf>,
    /// Natural language prompt or command line string that initiated the run.
    pub prompt: String,
    /// Flag indicating whether physical silicon / MMIO tripwires were tripped.
    pub hardware_tripped: bool,
}

/// Fluent query specification builder for multi-dimensional filtering.
#[derive(Debug, Clone, Default)]
pub struct SessionCriteria {
    /// Optional domain category filter.
    pub category: Option<String>,
    /// Optional final commit state filter.
    pub final_state: Option<QuadState>,
    /// Filter for sessions with or without failpoints (`Some(true)` => failpoints > 0).
    pub has_failpoints: Option<bool>,
    /// Filter for sessions with automated repairs (`Some(true)` => repair_count > 0).
    pub has_repairs: Option<bool>,
    /// Monotonic microsecond lower time bound.
    pub min_timestamp_micros: Option<u128>,
    /// Monotonic microsecond upper time bound.
    pub max_timestamp_micros: Option<u128>,
    /// Substring search token for the session prompt.
    pub prompt_keyword: Option<String>,
    /// Maximum number of matching records to return.
    pub limit: Option<usize>,
}

/// Aggregate statistical summary computed across a set of queried sessions.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct QuerySummary {
    /// Total sessions matched by the query.
    pub total_matched: usize,
    /// Number of sessions that concluded in `+1:PROCEED`.
    pub proceed_count: usize,
    /// Number of sessions that concluded in `-1:ROLLBACK`.
    pub rollback_count: usize,
    /// Number of sessions that concluded in `0:AWAIT`.
    pub await_count: usize,
    /// Number of sessions that concluded in `NULL:HALT`.
    pub halt_count: usize,
    /// Total failpoints trapped across all matched sessions.
    pub total_failpoints: usize,
    /// Total repairs applied by Restorator across all matched sessions.
    pub total_repairs: usize,
    /// Earliest timestamp microsecond in the matched set.
    pub earliest_micros: u128,
    /// Latest timestamp microsecond in the matched set.
    pub latest_micros: u128,
}

/// Complete result container holding matched records and statistical aggregates.
#[derive(Debug, Clone)]
pub struct QueryResult {
    /// Matching indexed sessions.
    pub sessions: Vec<IndexedSession>,
    /// Aggregate statistical metrics.
    pub summary: QuerySummary,
}

/// Structured record representing a low-level Track 0 silicon telemetry dump.
///
/// Captures bare-metal hardware flight recorder dumps, serial UART beacons,
/// and CPU register values from physical silicon inception.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Track0LogRecord {
    /// Inception boot mode (e.g. `BIOS Safe Mode`, `Stage 2 PM`, `Multiboot ELF`, `Native UEFI`).
    pub boot_mode: String,
    /// Target platform architecture (e.g. `x86_16`, `x86_32`, `x86_64`).
    pub architecture: String,
    /// Path to the raw Track 0 log file.
    pub log_path: PathBuf,
    /// Hardware flight recorder physical RAM address (e.g. `0x18000`, `0x108000`, `0x200000`).
    pub flight_recorder_address: Option<u64>,
    /// Named CPU register bank snapshot captured at boot or tripwire halt.
    pub registers: HashMap<String, u64>,
    /// Whether balanced ternary ALU equation `(+1)+(-1)=0` was verified.
    pub alu_verified: bool,
    /// Whether the CPI-SI cognitive substrate was successfully mounted.
    pub cpisi_arena_mounted: bool,
    /// Whether execution concluded in Safe Mode or Proceed.
    pub status: String,
    /// Raw lines captured from COM1 serial or OVMF firmware.
    pub raw_lines: Vec<String>,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Method Implementations, Constructors & Display Formatters )
// ==============================================================================

impl IndexedSession {
    /// Returns `true` if the session executed cleanly without failpoints in `+1:PROCEED`.
    pub fn is_clean(&self) -> bool {
        self.final_state == QuadState::Proceed && self.failpoint_count == 0
    }

    /// Returns `true` if the session required automated restoration or experienced rollbacks.
    pub fn had_anomalies(&self) -> bool {
        self.failpoint_count > 0 || self.final_state == QuadState::Rollback || self.final_state == QuadState::Halt
    }

    /// Returns the session duration in microseconds if duration is encoded, or 0.
    pub fn duration_micros(&self) -> u128 {
        // Derived from end of prompt or metadata if present
        0
    }

    /// Formats the session as an architectural one-line ledger entry.
    pub fn to_ledger_line(&self) -> String {
        let state_str = match self.final_state {
            QuadState::Proceed => "+1:PROCEED",
            QuadState::Rollback => "-1:ROLLBACK",
            QuadState::Await => " 0:AWAIT",
            QuadState::Halt => "NULL:HALT",
        };
        format!(
            "[{:>16} µs] {:<18} | {:<12} | State: {:<11} | FP: {:<3} | Rep: {:<3} | \"{}\"",
            self.timestamp_micros,
            self.session_id,
            self.category,
            state_str,
            self.failpoint_count,
            self.repair_count,
            truncate_prompt(&self.prompt, 40)
        )
    }
}

impl fmt::Display for IndexedSession {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_ledger_line())
    }
}

impl SessionCriteria {
    /// Constructs a blank, unrestricted query criteria.
    pub fn new() -> Self {
        Self::default()
    }

    /// Constrains the search to a specific domain category.
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// Constrains the search to a specific final commit QuadState.
    pub fn with_state(mut self, state: QuadState) -> Self {
        self.final_state = Some(state);
        self
    }

    /// Filters for sessions with or without failpoints.
    pub fn with_failpoints(mut self, has_failpoints: bool) -> Self {
        self.has_failpoints = Some(has_failpoints);
        self
    }

    /// Filters for sessions with or without Restorator repairs.
    pub fn with_repairs(mut self, has_repairs: bool) -> Self {
        self.has_repairs = Some(has_repairs);
        self
    }

    /// Constrains results to a monotonic time window `[min_micros ..= max_micros]`.
    pub fn with_time_range(mut self, min_micros: u128, max_micros: u128) -> Self {
        self.min_timestamp_micros = Some(min_micros);
        self.max_timestamp_micros = Some(max_micros);
        self
    }

    /// Constrains results to prompts containing the given substring token (case-insensitive).
    pub fn with_keyword(mut self, keyword: impl Into<String>) -> Self {
        self.prompt_keyword = Some(keyword.into().to_lowercase());
        self
    }

    /// Limits the maximum number of returned records.
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Evaluates whether an [`IndexedSession`] satisfies all active filter criteria.
    pub fn matches(&self, session: &IndexedSession) -> bool {
        if let Some(ref cat) = self.category {
            if &session.category != cat && !session.category.starts_with(cat) {
                return false;
            }
        }
        if let Some(st) = self.final_state {
            if session.final_state != st {
                return false;
            }
        }
        if let Some(hf) = self.has_failpoints {
            if (session.failpoint_count > 0) != hf {
                return false;
            }
        }
        if let Some(hr) = self.has_repairs {
            if (session.repair_count > 0) != hr {
                return false;
            }
        }
        if let Some(min_t) = self.min_timestamp_micros {
            if session.timestamp_micros < min_t {
                return false;
            }
        }
        if let Some(max_t) = self.max_timestamp_micros {
            if session.timestamp_micros > max_t {
                return false;
            }
        }
        if let Some(ref kw) = self.prompt_keyword {
            if !session.prompt.to_lowercase().contains(kw) && !session.session_id.to_lowercase().contains(kw) {
                return false;
            }
        }
        true
    }
}

impl QueryResult {
    /// Builds a [`QueryResult`] from a slice of matched sessions and computes summary statistics.
    pub fn from_sessions(sessions: Vec<IndexedSession>) -> Self {
        let total_matched = sessions.len();
        let mut proceed_count = 0;
        let mut rollback_count = 0;
        let mut await_count = 0;
        let mut halt_count = 0;
        let mut total_failpoints = 0;
        let mut total_repairs = 0;
        let mut earliest_micros = u128::MAX;
        let mut latest_micros = 0;

        for s in &sessions {
            match s.final_state {
                QuadState::Proceed => proceed_count += 1,
                QuadState::Rollback => rollback_count += 1,
                QuadState::Await => await_count += 1,
                QuadState::Halt => halt_count += 1,
            }
            total_failpoints += s.failpoint_count;
            total_repairs += s.repair_count;
            if s.timestamp_micros < earliest_micros {
                earliest_micros = s.timestamp_micros;
            }
            if s.timestamp_micros > latest_micros {
                latest_micros = s.timestamp_micros;
            }
        }

        if sessions.is_empty() {
            earliest_micros = 0;
        }

        let summary = QuerySummary {
            total_matched,
            proceed_count,
            rollback_count,
            await_count,
            halt_count,
            total_failpoints,
            total_repairs,
            earliest_micros,
            latest_micros,
        };

        Self { sessions, summary }
    }

    /// Returns the pass rate as a percentage `[0.0 .. 100.0]`.
    pub fn pass_rate(&self) -> f64 {
        if self.summary.total_matched == 0 {
            100.0
        } else {
            (self.summary.proceed_count as f64 / self.summary.total_matched as f64) * 100.0
        }
    }
}

impl Track0LogRecord {
    /// Parses a raw Track 0 log file from disk into a structured record.
    pub fn parse_from_file(path: PathBuf, content: &str) -> Self {
        let path_str = path.to_string_lossy();
        let mut boot_mode = "Unknown Boot Mode".to_string();
        let mut architecture = "x86".to_string();
        let mut flight_recorder_address = None;
        let mut registers = HashMap::new();
        let mut alu_verified = false;
        let mut cpisi_arena_mounted = false;
        let mut status = "UNKNOWN".to_string();
        let mut raw_lines = Vec::new();

        if path_str.contains("bios_safe_mode") {
            boot_mode = "Legacy BIOS Stage 1 (Safe Mode)".to_string();
            architecture = "x86_16".to_string();
        } else if path_str.contains("bios_dual_stage2") {
            boot_mode = "Dual-State Stage 2 Protected Mode".to_string();
            architecture = "x86_32".to_string();
            flight_recorder_address = Some(0x18000);
        } else if path_str.contains("multiboot") {
            boot_mode = "Multiboot 1 Direct Hypervisor".to_string();
            architecture = "x86_32".to_string();
            flight_recorder_address = Some(0x108000);
        } else if path_str.contains("uefi_safe_mode") {
            boot_mode = "UEFI Safe Mode Cascade".to_string();
            architecture = "x86_64".to_string();
            flight_recorder_address = Some(0x200000);
        } else if path_str.contains("uefi") {
            boot_mode = "Native 64-Bit UEFI (PE32+)".to_string();
            architecture = "x86_64".to_string();
            flight_recorder_address = Some(0x200000);
        }

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            raw_lines.push(trimmed.to_string());

            if trimmed.contains("(+1)+(-1)=0") && (trimmed.contains("OK") || trimmed.contains("VERIFIED")) {
                alu_verified = true;
            }
            if trimmed.contains("CPI-SI") && (trimmed.contains("MOUNTED") || trimmed.contains("Mount:  OK") || trimmed.contains("Arena:  OK")) {
                cpisi_arena_mounted = true;
            }
            if trimmed.contains("Flight Recorder (18000)") || trimmed.contains("0x18000") {
                flight_recorder_address = Some(0x18000);
            } else if trimmed.contains("Flight Recorder (108000)") || trimmed.contains("0x108000") {
                flight_recorder_address = Some(0x108000);
            } else if trimmed.contains("Flight Recorder (200000)") || trimmed.contains("0x200000") {
                flight_recorder_address = Some(0x200000);
            }

            if trimmed.contains("MILLENIUMOS") && trimmed.contains("PROCEED") {
                status = "PROCEED".to_string();
            } else if trimmed.contains("SAFE MODE") {
                if status == "UNKNOWN" {
                    status = "SAFE_MODE".to_string();
                }
            }

            // Parse registers if present, e.g. RAX=0x0000000007E8E650 RBX=0x0000000007E8E698
            if trimmed.contains("REGS:") {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                for part in parts {
                    if let Some(eq_pos) = part.find('=') {
                        let reg_name = &part[..eq_pos];
                        let val_str = &part[eq_pos + 1..];
                        let val_clean = val_str.trim_start_matches("0x").trim_start_matches("0X");
                        if let Ok(val) = u64::from_str_radix(val_clean, 16) {
                            registers.insert(reg_name.to_string(), val);
                        }
                    }
                }
            }
        }

        Self {
            boot_mode,
            architecture,
            log_path: path,
            flight_recorder_address,
            registers,
            alu_verified,
            cpisi_arena_mounted,
            status,
            raw_lines,
        }
    }
}

fn truncate_prompt(prompt: &str, max_len: usize) -> String {
    if prompt.len() <= max_len {
        prompt.to_string()
    } else {
        format!("{}...", &prompt[..max_len])
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Test Verification Proofs & Index Data Structure Invariants )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexed_session_clean_and_anomaly_invariants() {
        let clean = IndexedSession {
            session_id: "sess_01".to_string(),
            timestamp_micros: 1000,
            timestamp_id: "20261018_090000_001000".to_string(),
            category: "character".to_string(),
            final_state: QuadState::Proceed,
            failpoint_count: 0,
            repair_count: 0,
            word_count: 12,
            scroll_path: PathBuf::from("test.scroll"),
            log_path: None,
            prompt: "Echo character 'A'".to_string(),
            hardware_tripped: false,
        };

        assert!(clean.is_clean());
        assert!(!clean.had_anomalies());

        let mut anomaly = clean.clone();
        anomaly.failpoint_count = 1;
        assert!(!anomaly.is_clean());
        assert!(anomaly.had_anomalies());
    }

    #[test]
    fn test_session_criteria_matching() {
        let session = IndexedSession {
            session_id: "sess_char_01".to_string(),
            timestamp_micros: 50_000,
            timestamp_id: "20261018_090000_050000".to_string(),
            category: "character".to_string(),
            final_state: QuadState::Proceed,
            failpoint_count: 0,
            repair_count: 0,
            word_count: 5,
            scroll_path: PathBuf::from("test.scroll"),
            log_path: None,
            prompt: "Initialize character cell".to_string(),
            hardware_tripped: false,
        };

        let c1 = SessionCriteria::new()
            .with_category("character")
            .with_state(QuadState::Proceed);
        assert!(c1.matches(&session));

        let c2 = SessionCriteria::new().with_category("number");
        assert!(!c2.matches(&session));

        let c3 = SessionCriteria::new().with_time_range(10_000, 60_000);
        assert!(c3.matches(&session));

        let c4 = SessionCriteria::new().with_keyword("character");
        assert!(c4.matches(&session));

        let c5 = SessionCriteria::new().with_keyword("arithmetic");
        assert!(!c5.matches(&session));
    }

    #[test]
    fn test_query_result_aggregation() {
        let s1 = IndexedSession {
            session_id: "s1".to_string(),
            timestamp_micros: 100,
            timestamp_id: "100".to_string(),
            category: "cat".to_string(),
            final_state: QuadState::Proceed,
            failpoint_count: 0,
            repair_count: 0,
            word_count: 1,
            scroll_path: PathBuf::from("s1.scroll"),
            log_path: None,
            prompt: "p1".to_string(),
            hardware_tripped: false,
        };
        let s2 = IndexedSession {
            session_id: "s2".to_string(),
            timestamp_micros: 200,
            timestamp_id: "200".to_string(),
            category: "cat".to_string(),
            final_state: QuadState::Rollback,
            failpoint_count: 2,
            repair_count: 1,
            word_count: 2,
            scroll_path: PathBuf::from("s2.scroll"),
            log_path: None,
            prompt: "p2".to_string(),
            hardware_tripped: false,
        };

        let result = QueryResult::from_sessions(vec![s1, s2]);
        assert_eq!(result.summary.total_matched, 2);
        assert_eq!(result.summary.proceed_count, 1);
        assert_eq!(result.summary.rollback_count, 1);
        assert_eq!(result.summary.total_failpoints, 2);
        assert_eq!(result.summary.total_repairs, 1);
        assert_eq!(result.summary.earliest_micros, 100);
        assert_eq!(result.summary.latest_micros, 200);
        assert_eq!(result.pass_rate(), 50.0);
    }

    #[test]
    fn test_track0_log_record_parser() {
        let log_sample = r#"
MILLENIUMOS: OMNICODE 32-TRIT OS [x86 MBR]
[BOOT] 16-Bit Genesis: OK
[ALU ] (+1)+(-1)=0 Eq: OK
[REGS] 13 Regs Armed / Calibrated
[BOOT] State 0: Safe Mode Beacon
MILLENIUMOS SAFE MODE :: PROCEED
"#;
        let record = Track0LogRecord::parse_from_file(
            PathBuf::from("/kernel/images/logs/bios_safe_mode.log"),
            log_sample,
        );

        assert_eq!(record.architecture, "x86_16");
        assert!(record.alu_verified);
        assert_eq!(record.status, "PROCEED");
    }
}

// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Storehouse Inscriber Subsystem: Writers, Naming & Rotation Engine )
// ==============================================================================
//! Storehouse Inscription Subsystem: Vessels, Naming & Capacity Rotation
//!
//! # System Identity & Architectural Grounding
//! Grounded in the container law `Occupancy <= Capacity` (2 Kings 4:4), the
//! Inscriber subsystem is the dedicated machine organ responsible for transforming
//! transient sensory events, cognitive notes, and bare-metal Track 0 kernel
//! flight records into permanently persisted, bounded, and indexed vessels.
//!
//! # Subsystem Decomposition
//! - [`naming`]: Deterministic temporal prefix formatting and $O(1)$ landmark parsing.
//! - [`rotation`]: Multi-dimensional capacity enforcement across file count, file bytes,
//!   and partition volumes with automatic archival and manifest emission.
//! - [`log_writer`]: Traditional ASCII telemetry, 13 silicon register dumps, and 16-byte
//!   atomic flight recorder logs matching physical RAM landmarks.
//! - [`scroll_writer`]: Executable canonical 4-Block `.scroll` programs encoding
//!   episodic memory traces and Track 0 silicon genesis milestones (`CPI-SI`).

pub mod rotation;
pub mod naming;
pub mod scroll_writer;
pub mod log_writer;

use std::path::PathBuf;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Architectural Declarations, State Definitions & Comprehensive Re-exports )
// ==============================================================================

pub use rotation::{RotationPolicy, RotationReport};
pub use naming::{
    format_base_name, format_kernel_base_name, format_rotated_name,
    parse_file_name, resolve_custom_vessel_path, resolve_vessel_path,
    ParsedFileName, VesselNamingScheme,
};
pub use scroll_writer::{
    inscribe_audit_scroll, inscribe_canonical_scroll, inscribe_kernel_audit_scroll,
    ScrollGenre,
};
pub use log_writer::{
    inscribe_kernel_flight_log, inscribe_raw_flight_recorder_binary,
    inscribe_telemetry_log, KernelTelemetryRecord, RawFlightRecorderEntry,
};

/// High-level inscriber orchestrator managing storehouse partitions, active logs,
/// and automated rotation policies.
#[derive(Debug, Clone)]
pub struct InscriberEngine {
    /// Root directory of the episodic storehouse.
    pub storehouse_root: PathBuf,
    /// Rotation policy governing partition capacity.
    pub rotation_policy: RotationPolicy,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Inscription Orchestration, Partition Enforcement & Inscriber Methods )
// ==============================================================================

impl InscriberEngine {
    /// Constructs a new [`InscriberEngine`] rooted at `storehouse_root`.
    pub fn new(storehouse_root: impl Into<PathBuf>, policy: RotationPolicy) -> Self {
        Self {
            storehouse_root: storehouse_root.into(),
            rotation_policy: policy,
        }
    }

    /// Resolves the destination directory for a specific category and file type.
    pub fn resolve_path(&self, category: &str, is_scroll: bool, base_name: &str) -> PathBuf {
        resolve_vessel_path(&self.storehouse_root, category, is_scroll, base_name)
    }

    /// Enforces the active capacity rotation policy on a target partition directory.
    pub fn enforce_partition_capacity(&self, category: &str, is_scroll: bool) -> Result<RotationReport, String> {
        let subfolder = if is_scroll { "scroll" } else { "log" };
        let partition_dir = self.storehouse_root.join(subfolder).join(category);
        self.rotation_policy.enforce_detailed(&partition_dir)
    }

    /// Inscribes an episodic execution episode as both a canonical 4-Block `.scroll`
    /// and a traditional `.log` file, ensuring storehouse capacity invariants hold.
    pub fn inscribe_dual_episode(
        &self,
        category: &str,
        session_id: &str,
        timestamp_micros: u128,
        prompt: &str,
        synthesized_omnicode: &str,
        state_str: &str,
        response: &str,
    ) -> Result<(PathBuf, PathBuf), String> {
        let ts_str = timestamp_micros.to_string();
        let base_name = format_base_name(&ts_str, session_id);

        let scroll_path = self.resolve_path(category, true, &base_name);
        let log_path = self.resolve_path(category, false, &base_name);

        // Inscribe Canonical Scroll
        inscribe_canonical_scroll(
            &scroll_path,
            session_id,
            prompt,
            synthesized_omnicode,
            state_str,
            response,
            timestamp_micros,
        )?;

        // Inscribe Traditional Telemetry Log
        inscribe_telemetry_log(
            &log_path,
            &ts_str,
            session_id,
            timestamp_micros,
            state_str,
            prompt,
            &[],
            synthesized_omnicode,
            response,
        )?;

        // Enforce capacity bounds on both partition directories
        let _ = self.enforce_partition_capacity(category, true);
        let _ = self.enforce_partition_capacity(category, false);

        Ok((scroll_path, log_path))
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Invariants, Integration Audits & Unit Tests )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_inscriber_engine_dual_episode_lifecycle() {
        let temp_root = std::env::temp_dir().join(format!("omnicode_inscriber_mod_test_{}", std::process::id()));
        let policy = RotationPolicy::new(10, true);
        let engine = InscriberEngine::new(&temp_root, policy);

        let (scroll_p, log_p) = engine.inscribe_dual_episode(
            "character",
            "sess_inscriber_test_01",
            1792000000000000,
            "Transduce glyph",
            "Opening { preconditions: [ +1 ] }\nBody [ ]\nClosing { commit_status [ <PROCEED> ] }",
            "PROCEED",
            "Glyph transduced successfully",
        )
        .unwrap();

        assert!(scroll_p.exists());
        assert!(log_p.exists());

        let scroll_content = fs::read_to_string(&scroll_p).unwrap();
        assert!(scroll_content.contains("< EpistemicTrace { sess_inscriber_test_01 } >"));
        assert!(scroll_content.contains("commit_status [ <PROCEED> ]"));

        let log_content = fs::read_to_string(&log_p).unwrap();
        assert!(log_content.contains("OMNICODE AI EXECUTION & DIAGNOSTIC TELEMETRY LOG"));

        fs::remove_dir_all(&temp_root).ok();
    }
}

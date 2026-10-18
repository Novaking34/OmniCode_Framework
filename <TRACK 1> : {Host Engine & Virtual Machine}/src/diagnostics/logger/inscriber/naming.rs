// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Structured Naming Conventions for Deterministic Telemetry Indexing )
// ==============================================================================
//! Naming Conventions & Parsing Machinery for Storehouse Vessels
//!
//! # Epistemic Role & Container Invariants
//! Grounded in the container law `Occupancy <= Capacity` (2 Kings 4:4), every
//! telemetry and memory artifact in the OmniCode storehouse is inscribed into a
//! deterministic vessel.
//!
//! # Deterministic Vessel Indexing
//! Vessel file names are formatted with monotonic timestamp prefixes and domain
//! coordinates, enabling:
//! 1. $O(1)$ prefix matching and temporal range scans directly on the filesystem.
//! 2. Clean single-dot extensions (`.log`, `.scroll`, `.bin`, `.json`) without
//!    ambiguous multi-dot segments.
//! 3. Deterministic extraction of session IDs, Track 0 memory landmarks, boot tiers,
//!    and rotation generation indices.

use std::path::{Path, PathBuf};

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Architectural Setup, Type Definitions & Vessel Naming Schemas )
// ==============================================================================

/// High-level naming scheme categorization for storehouse artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VesselNamingScheme {
    /// Standard execution episode: `<timestamp>_<session_id>.<ext>`
    StandardSession,
    /// Track 0 bare-metal kernel flight telemetry: `<timestamp>_0x<landmark:08x>_<tier>_<session_id>.<ext>`
    KernelLandmark,
    /// Rotated partition file segment: `<timestamp>_<session_id>_part<part_num>.<ext>`
    RotatedPart,
    /// Archived snapshot: `archive/<timestamp>_<session_id>.<ext>`
    ArchivedSnapshot,
}

/// Comprehensive metadata extracted directly from a vessel file name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedFileName {
    /// Monotonic timestamp in microseconds since epoch or machine inception.
    pub timestamp_micros: u128,
    /// Unique session or component identifier.
    pub session_id: String,
    /// Vessel file extension without dot (e.g., "log", "scroll", "bin", "json").
    pub extension: String,
    /// Physical RAM landmark if this is a Track 0 kernel vessel (e.g., 0x18000, 0x108000, 0x200000).
    pub landmark_address: Option<u64>,
    /// Boot tier or architectural stage if identified (e.g., "STAGE2-PM", "UEFI", "LONGMODE").
    pub tier: Option<String>,
    /// Rotation generation / segment index if this is a rotated part.
    pub part_index: Option<usize>,
    /// Whether the file is located within an `archive/` subfolder.
    pub is_archived: bool,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Vessel Path Resolution, Formatting Constructors & Parsing Engines )
// ==============================================================================

/// Formats a standardized base name for telemetry and scroll files.
///
/// Pattern: `<timestamp>_<session_id>`
#[inline]
pub fn format_base_name(timestamp: &str, session_id: &str) -> String {
    format!("{timestamp}_{session_id}")
}

/// Formats a Track 0 bare-metal kernel base name encoding hardware landmarks.
///
/// Pattern: `<timestamp>_0x<landmark:08x>_<tier>_<session_id>`
/// Note: Hyphenates any multi-word tier (e.g. `STAGE2-PM`) to preserve clean single-underscore tokens.
pub fn format_kernel_base_name(
    timestamp: &str,
    landmark_address: u64,
    tier: &str,
    session_id: &str,
) -> String {
    let sanitized_tier = tier.replace([' ', '_', '/'], "-").to_uppercase();
    format!("{timestamp}_0x{landmark_address:08x}_{sanitized_tier}_{session_id}")
}

/// Formats a rotated part filename with an explicit segment index.
///
/// Pattern: `<base_name>_part<part_index>.<ext>`
/// Avoids ambiguous middle dots, ensuring only one extension delimiter.
pub fn format_rotated_name(base_name: &str, part_index: usize, ext: &str) -> String {
    format!("{base_name}_part{part_index}.{ext}")
}

/// Resolves the canonical destination directory for a specific category and file type.
///
/// If `root/scroll` or `root/log` subfolders exist, nests the output within:
/// `root/{subfolder}/{category}/{base_name}.{ext}`
/// Otherwise writes directly to `root/{base_name}.{ext}`.
pub fn resolve_vessel_path(
    root: &Path,
    category: &str,
    is_scroll: bool,
    base_name: &str,
) -> PathBuf {
    let subfolder = if is_scroll { "scroll" } else { "log" };
    let ext = if is_scroll { "scroll" } else { "log" };

    let dest_dir = if root.join(subfolder).exists() {
        root.join(subfolder).join(category)
    } else {
        root.to_path_buf()
    };

    dest_dir.join(format!("{base_name}.{ext}"))
}

/// Resolves a specialized vessel destination with a custom extension and directory structure.
pub fn resolve_custom_vessel_path(
    root: &Path,
    subfolder: &str,
    category: &str,
    base_name: &str,
    ext: &str,
) -> PathBuf {
    let dest_dir = if root.join(subfolder).exists() {
        root.join(subfolder).join(category)
    } else if !category.is_empty() {
        root.join(category)
    } else {
        root.to_path_buf()
    };

    dest_dir.join(format!("{base_name}.{ext}"))
}

/// Extracts rich metadata from a standardized telemetry vessel file path.
///
/// Supports standard `<timestamp>_<session_id>.<ext>`, kernel landmark names,
/// and rotated `_part<N>` logs.
pub fn parse_file_name(path: &Path) -> Option<ParsedFileName> {
    let ext = path.extension()?.to_string_lossy().to_string();

    // Check if path is inside an archive directory
    let is_archived = path
        .parent()
        .and_then(|p| p.file_name())
        .map(|name| name.to_string_lossy() == "archive")
        .unwrap_or(false);

    let file_stem = path.file_stem()?.to_str()?;

    // Detect if there is a `_part<N>` or `.part<N>` segment in the stem
    let mut part_index = None;
    let main_stem = if let Some(part_pos) = file_stem.find("_part") {
        let after = &file_stem[part_pos + 5..];
        if let Ok(num) = after.parse::<usize>() {
            part_index = Some(num);
        }
        &file_stem[..part_pos]
    } else if let Some(part_pos) = file_stem.find(".part") {
        let after = &file_stem[part_pos + 5..];
        if let Ok(num) = after.parse::<usize>() {
            part_index = Some(num);
        }
        &file_stem[..part_pos]
    } else {
        file_stem
    };

    let parts: Vec<&str> = main_stem.split('_').collect();
    if parts.is_empty() {
        return None;
    }

    let micros = parts[0].parse::<u128>().ok()?;

    // Check for Kernel Landmark: <micros>_0x<addr>_<tier>_<session...>
    let mut landmark_address = None;
    let mut tier = None;
    let session_id = if parts.len() >= 4 && parts[1].starts_with("0x") {
        if let Ok(addr) = u64::from_str_radix(&parts[1][2..], 16) {
            landmark_address = Some(addr);
            tier = Some(parts[2].to_string());
            parts[3..].join("_")
        } else if parts.len() > 1 {
            parts[1..].join("_")
        } else {
            "unknown".to_string()
        }
    } else if parts.len() > 1 {
        parts[1..].join("_")
    } else {
        "unknown".to_string()
    };

    Some(ParsedFileName {
        timestamp_micros: micros,
        session_id,
        extension: ext,
        landmark_address,
        tier,
        part_index,
        is_archived,
    })
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Invariant Audits, Round-Trip Verifications & Unit Tests )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_and_parse_standard_base_name() {
        let base = format_base_name("1792000000000000", "sess_alpha_99");
        assert_eq!(base, "1792000000000000_sess_alpha_99");

        let p = Path::new("/var/omnicode/scroll/character/1792000000000000_sess_alpha_99.scroll");
        let parsed = parse_file_name(p).expect("Must parse standard scroll");
        assert_eq!(parsed.timestamp_micros, 1792000000000000);
        assert_eq!(parsed.session_id, "sess_alpha_99");
        assert_eq!(parsed.extension, "scroll");
        assert_eq!(parsed.landmark_address, None);
        assert_eq!(parsed.tier, None);
        assert_eq!(parsed.part_index, None);
        assert!(!parsed.is_archived);
    }

    #[test]
    fn test_format_and_parse_kernel_landmark_vessel() {
        let k_base = format_kernel_base_name(
            "1792000000100000",
            0x00018000,
            "STAGE2_PM",
            "kernel_flight",
        );
        assert_eq!(k_base, "1792000000100000_0x00018000_STAGE2-PM_kernel_flight");

        let p = Path::new("/omnicode/log/kernel/1792000000100000_0x00018000_STAGE2-PM_kernel_flight.log");
        let parsed = parse_file_name(p).expect("Must parse kernel vessel");
        assert_eq!(parsed.timestamp_micros, 1792000000100000);
        assert_eq!(parsed.landmark_address, Some(0x18000));
        assert_eq!(parsed.tier.as_deref(), Some("STAGE2-PM"));
        assert_eq!(parsed.session_id, "kernel_flight");
        assert_eq!(parsed.extension, "log");
        assert!(!parsed.is_archived);
    }

    #[test]
    fn test_format_and_parse_rotated_part_and_archive() {
        let rotated = format_rotated_name("1792000000200000_sess_omega", 3, "log");
        assert_eq!(rotated, "1792000000200000_sess_omega_part3.log");

        let p = Path::new("/storehouse/archive/1792000000200000_sess_omega_part3.log");
        let parsed = parse_file_name(p).expect("Must parse rotated archived file");
        assert_eq!(parsed.timestamp_micros, 1792000000200000);
        assert_eq!(parsed.session_id, "sess_omega");
        assert_eq!(parsed.extension, "log");
        assert_eq!(parsed.part_index, Some(3));
        assert!(parsed.is_archived);
    }

    #[test]
    fn test_resolve_vessel_paths() {
        let root = Path::new("/var/storehouse");
        let path_scroll = resolve_vessel_path(root, "number", true, "1000_test");
        assert_eq!(
            path_scroll,
            PathBuf::from("/var/storehouse/1000_test.scroll")
        );

        let custom = resolve_custom_vessel_path(root, "flight", "hardware", "flight_01", "bin");
        assert_eq!(
            custom,
            PathBuf::from("/var/storehouse/hardware/flight_01.bin")
        );
    }
}

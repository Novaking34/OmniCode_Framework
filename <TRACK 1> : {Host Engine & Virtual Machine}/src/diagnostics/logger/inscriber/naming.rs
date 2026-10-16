// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Structured Naming Conventions for Deterministic Telemetry Indexing )
// ==============================================================================
//! Naming Conventions & Parsing Machinery for Storehouse Vessels
//!
//! Formats deterministic file names allowing $O(1)$ prefix matching and
//! temporal range queries directly from filesystem paths.

use std::path::{Path, PathBuf};

/// Formats a standardized base name for telemetry and scroll files.
///
/// Pattern: `<timestamp>_<session_id>`
pub fn format_base_name(timestamp: &str, session_id: &str) -> String {
    format!("{timestamp}_{session_id}")
}

/// Resolves the destination directory for a specific category and file type.
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

/// Metadata extracted directly from a vessel file name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedFileName {
    pub timestamp_micros: u128,
    pub session_id: String,
    pub extension: String,
}

/// Extracts metadata from a standardized telemetry file name.
pub fn parse_file_name(path: &Path) -> Option<ParsedFileName> {
    let file_stem = path.file_stem()?.to_string_lossy();
    let ext = path.extension()?.to_string_lossy().to_string();

    let parts: Vec<&str> = file_stem.split('_').collect();
    if parts.is_empty() {
        return None;
    }

    let micros = parts[0].parse::<u128>().ok()?;
    let session = if parts.len() > 1 {
        parts[1..].join("_")
    } else {
        "unknown".to_string()
    };

    Some(ParsedFileName {
        timestamp_micros: micros,
        session_id: session,
        extension: ext,
    })
}

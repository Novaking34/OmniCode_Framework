// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( RotationPolicy: Enforcing Vessel Capacity Bounds on Episodic Storehouses )
// ==============================================================================
//! Storehouse Vessel Rotation Engine
//!
//! Grounded in container law `Occupancy <= Capacity` (2 Kings 4:4), this module
//! prevents unbounded file proliferation by rotating and archiving old files.

use std::fs;
use std::path::Path;

/// Policy governing storehouse partition retention and rotation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RotationPolicy {
    /// Maximum number of active files allowed in a single domain partition.
    pub max_files_per_partition: usize,
    /// When true, older files are moved to an `archive/` subfolder.
    pub archive_old_files: bool,
}

impl RotationPolicy {
    /// Constructs a standard rotation policy with the given partition capacity.
    pub fn new(max_files_per_partition: usize, archive_old_files: bool) -> Self {
        Self {
            max_files_per_partition: max_files_per_partition.max(10),
            archive_old_files,
        }
    }

    /// Enforces the policy on the specified partition directory.
    pub fn enforce(&self, partition_dir: &Path) -> Result<usize, String> {
        if !partition_dir.exists() {
            return Ok(0);
        }

        let mut entries = Vec::new();
        let read_dir = fs::read_dir(partition_dir)
            .map_err(|e| format!("Failed to read partition dir {:?}: {e}", partition_dir))?;

        for entry in read_dir.flatten() {
            let p = entry.path();
            if p.is_file() && !p.file_name().unwrap_or_default().to_string_lossy().starts_with('.') {
                entries.push(p);
            }
        }

        if entries.len() <= self.max_files_per_partition {
            return Ok(0);
        }

        // Sort chronologically by filename (timestamp is in prefix)
        entries.sort();

        let excess_count = entries.len() - self.max_files_per_partition;
        let mut rotated = 0;

        let archive_dir = partition_dir.join("archive");
        if self.archive_old_files && !archive_dir.exists() {
            fs::create_dir_all(&archive_dir)
                .map_err(|e| format!("Failed to create archive dir {:?}: {e}", archive_dir))?;
        }

        for file_path in entries.iter().take(excess_count) {
            if self.archive_old_files {
                let file_name = file_path.file_name().unwrap();
                let dest = archive_dir.join(file_name);
                if fs::rename(file_path, &dest).is_ok() {
                    rotated += 1;
                }
            } else if fs::remove_file(file_path).is_ok() {
                rotated += 1;
            }
        }

        Ok(rotated)
    }
}

impl Default for RotationPolicy {
    /// Default storehouse policy: 1000 files per partition with archival.
    fn default() -> Self {
        Self::new(1000, true)
    }
}

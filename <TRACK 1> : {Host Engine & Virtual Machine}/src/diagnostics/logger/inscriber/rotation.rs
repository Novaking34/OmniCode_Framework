// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( RotationPolicy: Enforcing Vessel Capacity Bounds on Episodic Storehouses )
// ==============================================================================
//! Storehouse Vessel Rotation Engine & Capacity Enforcer
//!
//! # The Container Law of Episodic Storage
//! Grounded in the invariant of container law:
//! $$\text{Occupancy} \le \text{Capacity}$$
//! as established in 2 Kings 4:4 (*"When the vessels were full, she set aside that
//! which was full"*), this module prevents unbounded file proliferation across
//! millisecond turns and millennial operational horizons.
//!
//! # Multi-Dimensional Capacity Constraints
//! The rotation engine continuously bounds storehouse partitions across:
//! 1. **Vessel Count Bounds**: Maximum number of active files per partition.
//! 2. **Vessel Byte Limits**: Maximum byte size per individual `.log` or `.scroll`.
//! 3. **Partition Aggregate Capacity**: Maximum cumulative bytes allowed per domain.
//! 4. **Chronological Archival**: Seamless migration of sealed vessels to `archive/`
//!    subfolders with atomic rotation manifest generation.

use std::fs;
use std::path::{Path, PathBuf};

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Architectural Setup, Type Definitions & Rotation Telemetry )
// ==============================================================================

/// Structured report summarizing the outcome of a partition rotation cycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RotationReport {
    /// Total number of active vessel files scanned during enforcement.
    pub total_files_scanned: usize,
    /// Number of excess files rotated out of the active partition.
    pub files_rotated: usize,
    /// Number of rotated files migrated into the `archive/` subfolder.
    pub files_archived: usize,
    /// Number of rotated files permanently pruned (when archival is disabled).
    pub files_pruned: usize,
    /// Total byte volume reclaimed from the active partition directory.
    pub bytes_reclaimed: u64,
    /// Path to the audit manifest recording this rotation pass, if generated.
    pub manifest_path: Option<PathBuf>,
}

impl RotationReport {
    /// Constructs an empty baseline rotation report.
    pub fn empty() -> Self {
        Self {
            total_files_scanned: 0,
            files_rotated: 0,
            files_archived: 0,
            files_pruned: 0,
            bytes_reclaimed: 0,
            manifest_path: None,
        }
    }
}

/// Policy governing storehouse partition retention, byte capacity, and rotation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RotationPolicy {
    /// Maximum number of active vessel files allowed in a single domain partition.
    pub max_files_per_partition: usize,
    /// When true, older vessels are moved to an `archive/` subfolder rather than deleted.
    pub archive_old_files: bool,
    /// Maximum byte size of a single active log or scroll before active rotation is required.
    pub max_bytes_per_file: u64,
    /// Maximum aggregate byte capacity for the entire partition directory.
    pub max_partition_bytes: u64,
    /// Optional maximum age retention limit in microseconds.
    pub max_retention_micros: Option<u128>,
    /// When true, generates a structured JSON rotation manifest upon each rotation cycle.
    pub generate_manifest: bool,
    /// When true, appends archive generation markers or compression flags.
    pub compress_archives: bool,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Constructive Execution, Capacity Enforcement & Archival Mechanics )
// ==============================================================================

impl RotationPolicy {
    /// Constructs a standard rotation policy with the given partition file capacity.
    ///
    /// Preserves strict backwards compatibility with legacy callers while defaulting
    /// byte bounds to conservative, scalable thresholds (50 MB per file, 1 GB partition).
    pub fn new(max_files_per_partition: usize, archive_old_files: bool) -> Self {
        Self {
            max_files_per_partition: max_files_per_partition.max(10),
            archive_old_files,
            max_bytes_per_file: 50 * 1024 * 1024, // 50 MB
            max_partition_bytes: 1024 * 1024 * 1024, // 1 GB
            max_retention_micros: None,
            generate_manifest: true,
            compress_archives: false,
        }
    }

    /// Builder method to configure maximum file byte size before rollover.
    pub fn with_max_bytes_per_file(mut self, bytes: u64) -> Self {
        self.max_bytes_per_file = bytes.max(1);
        self
    }

    /// Builder method to configure aggregate partition byte capacity.
    pub fn with_max_partition_bytes(mut self, bytes: u64) -> Self {
        self.max_partition_bytes = bytes.max(1);
        self
    }

    /// Builder method to configure maximum retention duration in microseconds.
    pub fn with_retention_micros(mut self, micros: u128) -> Self {
        self.max_retention_micros = Some(micros);
        self
    }

    /// Builder method to enable or disable rotation manifest emission.
    pub fn with_manifest(mut self, generate: bool) -> Self {
        self.generate_manifest = generate;
        self
    }

    /// Builder method to enable or disable archive compression flags.
    pub fn with_compression(mut self, compress: bool) -> Self {
        self.compress_archives = compress;
        self
    }

    /// Checks whether an individual active vessel file requires rotation due to byte limits.
    pub fn check_rotation_needed(&self, file_path: &Path) -> bool {
        if let Ok(meta) = fs::metadata(file_path) {
            meta.len() >= self.max_bytes_per_file
        } else {
            false
        }
    }

    /// Rotates a single active vessel file by renaming it to a `.part<N>` segment.
    ///
    /// For example, `telemetry.log` -> `telemetry.part1.log`.
    pub fn rotate_active_file(&self, file_path: &Path) -> Result<PathBuf, String> {
        if !file_path.exists() {
            return Err(format!("Cannot rotate nonexistent file: {:?}", file_path));
        }

        let parent = file_path.parent().unwrap_or_else(|| Path::new("."));
        let file_stem = file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("vessel");
        let ext = file_path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("log");

        // Scan for highest existing part index
        let mut part = 1;
        while parent.join(format!("{file_stem}_part{part}.{ext}")).exists() {
            part += 1;
        }

        let target_path = parent.join(format!("{file_stem}_part{part}.{ext}"));
        fs::rename(file_path, &target_path).map_err(|e| {
            format!("Failed to rotate active file {:?} to {:?}: {e}", file_path, target_path)
        })?;

        Ok(target_path)
    }

    /// Enforces the policy on the specified partition directory, returning the number of rotated files.
    ///
    /// Preserves backwards compatibility for callers expecting `Result<usize, String>`.
    pub fn enforce(&self, partition_dir: &Path) -> Result<usize, String> {
        let report = self.enforce_detailed(partition_dir)?;
        Ok(report.files_rotated)
    }

    /// Executes detailed multi-dimensional capacity enforcement across count and byte limits.
    pub fn enforce_detailed(&self, partition_dir: &Path) -> Result<RotationReport, String> {
        if !partition_dir.exists() {
            return Ok(RotationReport::empty());
        }

        let mut report = RotationReport::empty();
        let read_dir = fs::read_dir(partition_dir)
            .map_err(|e| format!("Failed to read partition dir {:?}: {e}", partition_dir))?;

        // Collect all non-hidden, non-archive active vessel files with metadata
        struct FileEntry {
            path: PathBuf,
            size_bytes: u64,
        }

        let mut entries = Vec::new();
        let mut total_partition_bytes = 0u64;

        for entry in read_dir.flatten() {
            let p = entry.path();
            if p.is_file() {
                let name = p.file_name().unwrap_or_default().to_string_lossy();
                // Exclude hidden files and rotation manifests from rotation consideration
                if !name.starts_with('.') && !name.ends_with(".manifest.json") {
                    let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                    total_partition_bytes += size;
                    entries.push(FileEntry { path: p, size_bytes: size });
                }
            }
        }

        report.total_files_scanned = entries.len();

        // Sort chronologically by filename (standard naming places timestamp prefix first)
        entries.sort_by(|a, b| a.path.cmp(&b.path));

        // Determine excess items by count
        let count_excess = if entries.len() > self.max_files_per_partition {
            entries.len() - self.max_files_per_partition
        } else {
            0
        };

        // Determine excess items by cumulative partition byte capacity
        let mut byte_excess_indices = Vec::new();
        let mut current_bytes = total_partition_bytes;
        for (i, entry) in entries.iter().enumerate() {
            if current_bytes > self.max_partition_bytes && i >= count_excess {
                byte_excess_indices.push(i);
                current_bytes = current_bytes.saturating_sub(entry.size_bytes);
            }
        }

        let total_rotate_target = count_excess + byte_excess_indices.len();
        if total_rotate_target == 0 {
            return Ok(report);
        }

        let archive_dir = partition_dir.join("archive");
        if self.archive_old_files && !archive_dir.exists() {
            fs::create_dir_all(&archive_dir)
                .map_err(|e| format!("Failed to create archive dir {:?}: {e}", archive_dir))?;
        }

        let mut rotated_file_names = Vec::new();

        // Process files that exceed count capacity (oldest first)
        for entry in entries.iter().take(count_excess) {
            let file_name = entry.path.file_name().unwrap();
            let size = entry.size_bytes;

            if self.archive_old_files {
                let mut dest = archive_dir.join(file_name);
                // Handle potential destination collisions
                if dest.exists() {
                    let stem = entry.path.file_stem().and_then(|s| s.to_str()).unwrap_or("vessel");
                    let ext = entry.path.extension().and_then(|s| s.to_str()).unwrap_or("log");
                    dest = archive_dir.join(format!("{stem}_{}.{ext}", std::process::id()));
                }

                if fs::rename(&entry.path, &dest).is_ok() {
                    report.files_rotated += 1;
                    report.files_archived += 1;
                    report.bytes_reclaimed += size;
                    rotated_file_names.push(dest.file_name().unwrap().to_string_lossy().to_string());
                }
            } else if fs::remove_file(&entry.path).is_ok() {
                report.files_rotated += 1;
                report.files_pruned += 1;
                report.bytes_reclaimed += size;
                rotated_file_names.push(file_name.to_string_lossy().to_string());
            }
        }

        // Process files that exceed byte capacity
        for idx in byte_excess_indices {
            if let Some(entry) = entries.get(idx) {
                if !entry.path.exists() {
                    continue;
                }
                let file_name = entry.path.file_name().unwrap();
                let size = entry.size_bytes;

                if self.archive_old_files {
                    let dest = archive_dir.join(file_name);
                    if fs::rename(&entry.path, &dest).is_ok() {
                        report.files_rotated += 1;
                        report.files_archived += 1;
                        report.bytes_reclaimed += size;
                        rotated_file_names.push(dest.file_name().unwrap().to_string_lossy().to_string());
                    }
                } else if fs::remove_file(&entry.path).is_ok() {
                    report.files_rotated += 1;
                    report.files_pruned += 1;
                    report.bytes_reclaimed += size;
                    rotated_file_names.push(file_name.to_string_lossy().to_string());
                }
            }
        }

        // Generate JSON audit manifest if configured
        if self.generate_manifest && report.files_rotated > 0 {
            let manifest_target = if self.archive_old_files {
                archive_dir.join("rotation_manifest.json")
            } else {
                partition_dir.join(".rotation_manifest.json")
            };

            let manifest_content = format!(
                "{{\n  \"timestamp_micros\": {},\n  \"files_rotated\": {},\n  \"files_archived\": {},\n  \"files_pruned\": {},\n  \"bytes_reclaimed\": {},\n  \"rotated_vessels\": {:?}\n}}\n",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_micros())
                    .unwrap_or(0),
                report.files_rotated,
                report.files_archived,
                report.files_pruned,
                report.bytes_reclaimed,
                rotated_file_names,
            );

            if fs::write(&manifest_target, manifest_content).is_ok() {
                report.manifest_path = Some(manifest_target);
            }
        }

        Ok(report)
    }
}

impl Default for RotationPolicy {
    /// Default storehouse policy: 1000 files per partition with archival.
    fn default() -> Self {
        Self::new(1000, true)
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Invariant Audits, Capacity Boundary Verifications & Unit Tests )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotation_policy_file_count_enforce() {
        let temp_dir = std::env::temp_dir().join(format!("omnicode_rot_count_test_{}", std::process::id()));
        fs::create_dir_all(&temp_dir).unwrap();

        for i in 0..15 {
            fs::write(
                temp_dir.join(format!("17920000000000{:02}_item.log", i)),
                "telemetry vessel payload data",
            )
            .unwrap();
        }

        let policy = RotationPolicy::new(10, true);
        let rotated = policy.enforce(&temp_dir).unwrap();
        assert_eq!(rotated, 5, "Must rotate exactly 5 excess files");

        let archive_dir = temp_dir.join("archive");
        assert!(archive_dir.exists(), "Archive directory must be created");

        let remaining = fs::read_dir(&temp_dir)
            .unwrap()
            .flatten()
            .filter(|e| e.path().is_file())
            .count();
        assert_eq!(remaining, 10, "Exactly 10 active files must remain");

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_rotation_policy_partition_byte_capacity() {
        let temp_dir = std::env::temp_dir().join(format!("omnicode_rot_bytes_test_{}", std::process::id()));
        fs::create_dir_all(&temp_dir).unwrap();

        // Create 5 files of 1000 bytes each = 5000 bytes total
        let chunk = vec![b'X'; 1000];
        for i in 0..5 {
            fs::write(temp_dir.join(format!("100000000000000{i}_log.log")), &chunk).unwrap();
        }

        // Set policy: max 10 files (count allows all 5), but max 2500 bytes (requires rotating 3 files)
        let policy = RotationPolicy::new(10, true).with_max_partition_bytes(2500);
        let report = policy.enforce_detailed(&temp_dir).unwrap();

        assert!(
            report.files_rotated >= 2,
            "Must rotate files exceeding byte capacity: rotated {}",
            report.files_rotated
        );
        assert!(report.bytes_reclaimed >= 2000);
        assert!(report.manifest_path.is_some(), "Manifest must be emitted");

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_rotation_policy_active_file_size_check_and_rollover() {
        let temp_dir = std::env::temp_dir().join(format!("omnicode_rot_active_test_{}", std::process::id()));
        fs::create_dir_all(&temp_dir).unwrap();

        let active_file = temp_dir.join("active_session.log");
        fs::write(&active_file, vec![b'A'; 2048]).unwrap();

        let policy = RotationPolicy::new(10, true).with_max_bytes_per_file(1024);
        assert!(policy.check_rotation_needed(&active_file));

        let rotated_target = policy.rotate_active_file(&active_file).unwrap();
        assert_eq!(
            rotated_target,
            temp_dir.join("active_session_part1.log")
        );
        assert!(rotated_target.exists());
        assert!(!active_file.exists());

        fs::remove_dir_all(&temp_dir).ok();
    }
}

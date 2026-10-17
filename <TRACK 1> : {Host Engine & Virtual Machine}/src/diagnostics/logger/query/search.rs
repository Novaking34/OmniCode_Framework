// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( EpisodicQueryEngine: O(1) Lookup & Partitioned Range Search )
// ============================================================================
//! Episodic Query Engine
//!
//! Indexes and queries past execution scrolls and telemetry without linear
//! file scans, supporting filtering by session, category, QuadState, and time.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use super::index::IndexedSession;
use crate::core::ternary::QuadState;

/// High-performance query engine for episodic memory storehouses.
#[derive(Debug, Clone, Default)]
pub struct EpisodicQueryEngine {
    sessions: Vec<IndexedSession>,
    by_session_id: HashMap<String, usize>,
    by_category: HashMap<String, Vec<usize>>,
}

impl EpisodicQueryEngine {
    /// Constructs an empty [`EpisodicQueryEngine`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Indexes all `.scroll` files located under `root_dir`.
    pub fn index_directory(
        &mut self,
        root_dir: &Path,
    ) -> Result<usize, String> {
        self.sessions.clear();
        self.by_session_id.clear();
        self.by_category.clear();

        if !root_dir.exists() {
            return Ok(0);
        }

        let scroll_paths = collect_scroll_files(root_dir);
        let mut count = 0;

        for path in scroll_paths {
            if let Some(session) = parse_indexed_session(&path) {
                let idx = self.sessions.len();
                self.by_session_id
                    .insert(session.session_id.clone(), idx);
                self.by_category
                    .entry(session.category.clone())
                    .or_default()
                    .push(idx);
                self.sessions.push(session);
                count += 1;
            }
        }

        // Sort chronologically
        self.sessions.sort_by_key(|s| s.timestamp_micros);
        // Rebuild session ID map after sort with multi-key aliases
        self.by_session_id.clear();
        self.by_category.clear();
        for (idx, s) in self.sessions.iter().enumerate() {
            self.by_session_id.insert(s.session_id.clone(), idx);
            if let Some(stem) = s
                .scroll_path
                .file_stem()
                .and_then(|t| t.to_str())
            {
                self.by_session_id.insert(stem.to_string(), idx);
                let parts: Vec<&str> = stem.split('_').collect();
                if parts.len() >= 3 {
                    self.by_session_id.insert(parts[1..].join("_"), idx);
                    self.by_session_id.insert(parts[2..].join("_"), idx);
                }
            }
            self.by_category
                .entry(s.category.clone())
                .or_default()
                .push(idx);
        }

        Ok(count)
    }

    /// Finds an episode by its exact session ID in O(1) time.
    pub fn find_by_session(&self, session_id: &str) -> Option<&IndexedSession> {
        let idx = *self.by_session_id.get(session_id)?;
        self.sessions.get(idx)
    }

    /// Filters all episodes belonging to a specific domain category.
    pub fn filter_by_category(&self, category: &str) -> Vec<&IndexedSession> {
        if let Some(indices) = self.by_category.get(category) {
            indices.iter().filter_map(|&i| self.sessions.get(i)).collect()
        } else {
            Vec::new()
        }
    }

    /// Filters episodes by their commit QuadState.
    pub fn filter_by_state(&self, state: QuadState) -> Vec<&IndexedSession> {
        self.sessions
            .iter()
            .filter(|s| s.final_state == state)
            .collect()
    }

    /// Filters episodes that experienced failpoints.
    pub fn filter_by_failpoint(
        &self,
        has_failpoints: bool,
    ) -> Vec<&IndexedSession> {
        self.sessions
            .iter()
            .filter(|s| (s.failpoint_count > 0) == has_failpoints)
            .collect()
    }

    /// Range scans episodes executed between `start_micros` and `end_micros`.
    pub fn range_scan(
        &self,
        start_micros: u128,
        end_micros: u128,
    ) -> Vec<&IndexedSession> {
        self.sessions
            .iter()
            .filter(|s| {
                s.timestamp_micros >= start_micros
                    && s.timestamp_micros <= end_micros
            })
            .collect()
    }

    /// Returns a slice of all indexed sessions.
    pub fn all_sessions(&self) -> &[IndexedSession] {
        &self.sessions
    }
}

/// Recursively collects all `.scroll` files within `dir`.
fn collect_scroll_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                files.extend(collect_scroll_files(&p));
            } else if p.extension().and_then(|e| e.to_str()) == Some("scroll") {
                files.push(p);
            }
        }
    }
    files
}

/// Parses an `IndexedSession` from a single `.scroll` file.
fn parse_indexed_session(path: &Path) -> Option<IndexedSession> {
    let content = fs::read_to_string(path).ok()?;
    let mut session_id = "unknown".to_string();
    let mut prompt = String::new();
    let mut final_state = QuadState::Proceed;
    let mut failpoint_count = 0;
    let mut repair_count = 0;
    let mut timestamp_micros = 0u128;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("session_id:") {
            if let Some(sq) = trimmed.find('"') {
                if let Some(eq) = trimmed.rfind('"') {
                    if sq < eq {
                        session_id = trimmed[sq + 1..eq].to_string();
                    }
                }
            }
        } else if trimmed.starts_with("timestamp:") {
            if let Some(sq) = trimmed.find('"') {
                if let Some(eq) = trimmed.rfind('"') {
                    if sq < eq {
                        let ts_str = &trimmed[sq + 1..eq];
                        if let Some(first) = ts_str.split('_').next() {
                            timestamp_micros = first.parse().unwrap_or(0);
                        }
                    }
                }
            }
        } else if trimmed.starts_with("user_prompt") {
            if let Some(sq) = trimmed.find('"') {
                if let Some(eq) = trimmed.rfind('"') {
                    if sq < eq {
                        prompt = trimmed[sq + 1..eq].to_string();
                    }
                }
            }
        } else if prompt.is_empty()
            && trimmed.starts_with("Scroll < EpistemicTrace")
        {
            if let Some(open_p) = trimmed.find('(') {
                if let Some(close_p) = trimmed.rfind(')') {
                    if open_p < close_p {
                        prompt = trimmed[open_p + 1..close_p]
                            .trim()
                            .to_string();
                    }
                }
            }
        } else if trimmed.starts_with("commit_status [") {
            if trimmed.contains("<PROCEED>") {
                final_state = QuadState::Proceed;
            } else if trimmed.contains("<ROLLBACK>") {
                final_state = QuadState::Rollback;
            } else if trimmed.contains("<AWAIT>") {
                final_state = QuadState::Await;
            } else if trimmed.contains("<HALT>") {
                final_state = QuadState::Halt;
            }
        } else if trimmed.starts_with("failpoint_record [") {
            failpoint_count += 1;
        } else if trimmed.starts_with("repair_entry [") {
            repair_count += 1;
        }
    }

    // Determine category from path or prompt
    let path_str = path.to_string_lossy();
    let category = if path_str.contains("/genesis/")
        || prompt.contains("genesis")
        || prompt.contains("first_log")
        || prompt.contains("boot")
    {
        "genesis".to_string()
    } else if path_str.contains("/character/")
        || prompt.contains("character")
    {
        "character".to_string()
    } else if path_str.contains("/number/") || prompt.contains("number") {
        "number".to_string()
    } else if path_str.contains("/sessions/prompts")
        || (!prompt.trim().is_empty() && path_str.contains("/sessions/"))
    {
        "sessions/prompts".to_string()
    } else if path_str.contains("/sessions/")
        || session_id.starts_with("sess_")
    {
        "sessions/system".to_string()
    } else {
        "cortex".to_string()
    };

    // Locate paired log file
    let mut log_path = path.to_path_buf();
    log_path.set_extension("log");
    let paired_log = if log_path.exists() {
        Some(log_path)
    } else if path_str.contains("/scroll/") {
        let alt = PathBuf::from(path_str.replace("/scroll/", "/log/"));
        let mut alt_log = alt;
        alt_log.set_extension("log");
        if alt_log.exists() { Some(alt_log) } else { None }
    } else {
        None
    };

    Some(IndexedSession {
        session_id,
        timestamp_micros,
        category,
        final_state,
        failpoint_count,
        repair_count,
        scroll_path: path.to_path_buf(),
        log_path: paired_log,
        prompt,
    })
}

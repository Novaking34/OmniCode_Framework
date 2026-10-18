// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( EpisodicQueryEngine: O(1) Indexing, Compound Search & Silicon Correlation )
// ==============================================================================
//! # Episodic Query Engine Implementation
//!
//! # Architectural Scope & Purpose:
//! The `search` module implements [`EpisodicQueryEngine`]. Traditional logging
//! architectures treat telemetry as an append-only disposal dump requiring linear
//! file scans (`grep`) that degrade as system uptime and log volume scale ($O(N)$).
//!
//! In contrast, OmniCode's [`EpisodicQueryEngine`] provides:
//! 1. **Multi-Key $O(1)$ Hash Indexing**: Maps session IDs, timestamp prefixes,
//!    file stems, and short suffixes to internal session indices.
//! 2. **Inverted Keyword Index**: Tokenizes user prompts and system tasks, mapping
//!    terms to execution episodes without disk access.
//! 3. **Chronological Monotonicity**: Preserves strictly sorted microsecond
//!    sequences, enabling instant binary range-slicing ($\Delta t = t_1 - t_0$).
//! 4. **Track 0 Silicon Correlation**: Discovers and parses bare-metal hardware
//!    dumps directly from `kernel/images/logs/`, correlating hardware register
//!    dumps (e.g. `0x18000`, `0x108000`, `0x200000`) with high-level software episodes.
//! 5. **Dual-Stream Pairing**: Resolves between the concise 4-Block `.scroll`
//!    cognitive memory file and its paired microsecond-by-microsecond `.log` file.
//!
//! # Scripture Witness:
//! - **Jeremiah 29:13 (KJV)**:
//!   "And ye shall seek me, and find me, when ye shall search for me with all your heart."
//!   (Thorough, exhaustive search yielding complete diagnostic understanding).
//! - **Matthew 7:7 (KJV)**:
//!   "Ask, and it shall be given you; seek, and ye shall find; knock, and it shall be opened unto you."
//!   (The query interface as the open door to system memory and self-healing).
//! - **1 Chronicles 28:9 (KJV)**:
//!   "for the LORD searcheth all hearts, and understandeth all the imaginations of the thoughts."
//!   (Full inspection of internal system state, cognitive deliberations, and failpoints).
//!
//! # Why Fast Querying Matters for AI Agents & Autonomic Healing:
//! - **Autonomous Reflection**: When the `AICortex` fails or encounters an edge case,
//!   it queries past episodes to discover whether the same prompt previously succeeded,
//!   what syntax repairs were applied by `Restorator`, and what QuadState resulted.
//! - **DAR Immune Feedback**: The self-healing loop cannot wait seconds for linear disk
//!   scans. It requires sub-millisecond retrieval of failpoint frequencies and patterns.
// ==============================================================================

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use super::index::{IndexedSession, QueryResult, SessionCriteria, Track0LogRecord};
use crate::core::ternary::QuadState;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Engine Struct Definition & Index Invariant Declarations )
// ==============================================================================

/// High-performance query engine for episodic memory storehouses.
///
/// Maintains multi-key hash maps, category buckets, QuadState indices,
/// and an inverted keyword index over in-memory session records.
#[derive(Debug, Clone, Default)]
pub struct EpisodicQueryEngine {
    /// Chronologically ordered list of all indexed execution episodes.
    sessions: Vec<IndexedSession>,
    /// Multi-key alias index mapping session IDs, stems, and suffixes to vector positions.
    by_session_id: HashMap<String, usize>,
    /// Domain category partition index mapping category names to session positions.
    by_category: HashMap<String, Vec<usize>>,
    /// State partition index mapping QuadStates to session positions.
    by_state: HashMap<QuadState, Vec<usize>>,
    /// Inverted keyword index mapping tokenized prompt words to session positions.
    by_keyword: HashMap<String, Vec<usize>>,
    /// Parsed low-level Track 0 bare-metal silicon logs.
    track0_records: Vec<Track0LogRecord>,
    /// Index mapping Track 0 boot modes (e.g. "x86_16", "uefi") to silicon records.
    by_track0_mode: HashMap<String, Vec<usize>>,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Directory Indexing, Query Implementations & Track 0 Parsers )
// ==============================================================================

impl EpisodicQueryEngine {
    /// Constructs an empty [`EpisodicQueryEngine`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Indexes all `.scroll` files located under `root_dir`.
    ///
    /// Clears existing indices, scans `.scroll` files, extracts 4-block header
    /// metadata in $O(1)$ time per file, sorts chronologically, and populates
    /// multi-key hash maps and the inverted keyword index.
    pub fn index_directory(&mut self, root_dir: &Path) -> Result<usize, String> {
        self.sessions.clear();
        self.by_session_id.clear();
        self.by_category.clear();
        self.by_state.clear();
        self.by_keyword.clear();

        if !root_dir.exists() {
            return Ok(0);
        }

        let scroll_paths = collect_scroll_files(root_dir);
        let mut count = 0;

        for path in scroll_paths {
            if let Some(session) = parse_indexed_session(&path) {
                self.sessions.push(session);
                count += 1;
            }
        }

        // Sort chronologically by monotonic microsecond timestamp
        self.sessions.sort_by_key(|s| s.timestamp_micros);

        // Populate indices over sorted array
        for (idx, s) in self.sessions.iter().enumerate() {
            // 1. Primary session ID
            self.by_session_id.insert(s.session_id.clone(), idx);

            // 2. Multi-key aliases (stem, date prefix, suffixes)
            if let Some(stem) = s.scroll_path.file_stem().and_then(|t| t.to_str()) {
                self.by_session_id.insert(stem.to_string(), idx);
                let parts: Vec<&str> = stem.split('_').collect();
                if parts.len() >= 3 {
                    self.by_session_id.insert(parts[1..].join("_"), idx);
                    self.by_session_id.insert(parts[2..].join("_"), idx);
                }
            }

            // 3. Category index
            self.by_category
                .entry(s.category.clone())
                .or_default()
                .push(idx);

            // 4. QuadState index
            self.by_state.entry(s.final_state).or_default().push(idx);

            // 5. Inverted keyword index over prompt tokens
            let tokens = tokenize_prompt(&s.prompt);
            for token in tokens {
                self.by_keyword.entry(token).or_default().push(idx);
            }
        }

        Ok(count)
    }

    /// Discovers and indexes Track 0 bare-metal silicon logs from a logs directory.
    pub fn index_track0_logs(&mut self, logs_dir: &Path) -> Result<usize, String> {
        self.track0_records.clear();
        self.by_track0_mode.clear();

        if !logs_dir.exists() {
            return Ok(0);
        }

        let mut count = 0;
        if let Ok(entries) = fs::read_dir(logs_dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.extension().and_then(|e| e.to_str()) == Some("log") {
                    if let Ok(content) = fs::read_to_string(&p) {
                        let record = Track0LogRecord::parse_from_file(p, &content);
                        let idx = self.track0_records.len();
                        self.by_track0_mode
                            .entry(record.architecture.clone())
                            .or_default()
                            .push(idx);
                        self.by_track0_mode
                            .entry(record.boot_mode.clone())
                            .or_default()
                            .push(idx);
                        self.track0_records.push(record);
                        count += 1;
                    }
                }
            }
        }

        Ok(count)
    }

    /// Finds an episode by its exact session ID or alias in $O(1)$ time.
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

    /// Filters episodes by their final commit QuadState.
    pub fn filter_by_state(&self, state: QuadState) -> Vec<&IndexedSession> {
        if let Some(indices) = self.by_state.get(&state) {
            indices.iter().filter_map(|&i| self.sessions.get(i)).collect()
        } else {
            self.sessions.iter().filter(|s| s.final_state == state).collect()
        }
    }

    /// Filters episodes that experienced failpoints.
    pub fn filter_by_failpoint(&self, has_failpoints: bool) -> Vec<&IndexedSession> {
        self.sessions
            .iter()
            .filter(|s| (s.failpoint_count > 0) == has_failpoints)
            .collect()
    }

    /// Range scans episodes executed between `start_micros` and `end_micros` inclusive.
    pub fn range_scan(&self, start_micros: u128, end_micros: u128) -> Vec<&IndexedSession> {
        self.sessions
            .iter()
            .filter(|s| s.timestamp_micros >= start_micros && s.timestamp_micros <= end_micros)
            .collect()
    }

    /// Searches episodes by prompt keyword via the inverted index.
    pub fn search_keyword(&self, keyword: &str) -> Vec<&IndexedSession> {
        let token = keyword.trim().to_lowercase();
        if let Some(indices) = self.by_keyword.get(&token) {
            indices.iter().filter_map(|&i| self.sessions.get(i)).collect()
        } else {
            // Fallback substring search
            self.sessions
                .iter()
                .filter(|s| s.prompt.to_lowercase().contains(&token))
                .collect()
        }
    }

    /// Executes a compound search matching all constraints defined in [`SessionCriteria`].
    pub fn query(&self, criteria: &SessionCriteria) -> QueryResult {
        let mut matched = Vec::new();

        for session in &self.sessions {
            if criteria.matches(session) {
                matched.push(session.clone());
                if let Some(limit) = criteria.limit {
                    if matched.len() >= limit {
                        break;
                    }
                }
            }
        }

        QueryResult::from_sessions(matched)
    }

    /// Returns a slice of all indexed sessions.
    pub fn all_sessions(&self) -> &[IndexedSession] {
        &self.sessions
    }

    /// Returns a slice of all parsed Track 0 silicon log records.
    pub fn track0_records(&self) -> &[Track0LogRecord] {
        &self.track0_records
    }

    /// Filters Track 0 records by boot mode or architecture string.
    pub fn find_track0_by_mode(&self, mode: &str) -> Vec<&Track0LogRecord> {
        if let Some(indices) = self.by_track0_mode.get(mode) {
            indices.iter().filter_map(|&i| self.track0_records.get(i)).collect()
        } else {
            self.track0_records
                .iter()
                .filter(|r| r.boot_mode.contains(mode) || r.architecture.contains(mode))
                .collect()
        }
    }

    /// Inspects and formats deep diagnostic details for a specific session.
    pub fn inspect_session_detail(&self, session_id: &str) -> Option<String> {
        let session = self.find_by_session(session_id)?;
        let mut out = format!("===============================================================================\n");
        out.push_str(&format!("  OMNICODE EPISODIC DIAGNOSTIC DOSSIER: {}\n", session.session_id));
        out.push_str(&format!("===============================================================================\n"));
        out.push_str(&format!("• Timestamp:     {} µs ({})\n", session.timestamp_micros, session.timestamp_id));
        out.push_str(&format!("• Category:      {}\n", session.category));
        out.push_str(&format!("• Final State:   {:?}\n", session.final_state));
        out.push_str(&format!("• Failpoints:    {}\n", session.failpoint_count));
        out.push_str(&format!("• Repairs:       {}\n", session.repair_count));
        out.push_str(&format!("• Scroll Path:   {}\n", session.scroll_path.display()));
        if let Some(ref lp) = session.log_path {
            out.push_str(&format!("• Log Path:      {}\n", lp.display()));
        }
        out.push_str(&format!("• User Prompt:   \"{}\"\n", session.prompt));

        // Read paired log telemetry if available
        if let Some(ref lp) = session.log_path {
            if let Ok(content) = fs::read_to_string(lp) {
                out.push_str(&format!("\n--- SENSORY LOG EXCERPT (Critical & Remediation Lines) ---\n"));
                let mut matches = 0;
                for line in content.lines() {
                    if line.contains("CRITICAL") || line.contains("Remediation") || line.contains("FAILPOINT") {
                        out.push_str(&format!("  {}\n", line));
                        matches += 1;
                        if matches > 20 {
                            out.push_str("  ... [Remaining entries truncated]\n");
                            break;
                        }
                    }
                }
                if matches == 0 {
                    out.push_str("  (No critical failpoints or tripwires recorded in sensory log)\n");
                }
            }
        }

        out.push_str(&format!("===============================================================================\n"));
        Some(out)
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

/// Tokenizes a prompt string into clean, lowercase keyword tokens.
fn tokenize_prompt(prompt: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    for word in prompt.split_whitespace() {
        let clean: String = word
            .chars()
            .filter(|c| c.is_alphanumeric())
            .flat_map(|c| c.to_lowercase())
            .collect();
        if clean.len() >= 3 {
            tokens.push(clean);
        }
    }
    tokens
}

/// Parses an `IndexedSession` from the first 30 lines of a `.scroll` file.
fn parse_indexed_session(path: &Path) -> Option<IndexedSession> {
    let content = fs::read_to_string(path).ok()?;
    let mut session_id = "unknown".to_string();
    let mut prompt = String::new();
    let mut final_state = QuadState::Proceed;
    let mut failpoint_count = 0;
    let mut repair_count = 0;
    let mut timestamp_micros = 0u128;
    let mut timestamp_id = String::new();
    let mut hardware_tripped = false;

    for line in content.lines().take(40) {
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
                        timestamp_id = ts_str.to_string();
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
        } else if prompt.is_empty() && trimmed.starts_with("Scroll < EpistemicTrace") {
            if let Some(open_p) = trimmed.find('(') {
                if let Some(close_p) = trimmed.rfind(')') {
                    if open_p < close_p {
                        prompt = trimmed[open_p + 1..close_p].trim().to_string();
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
            if trimmed.contains("SILICON") || trimmed.contains("MMIO") || trimmed.contains("REGISTER") {
                hardware_tripped = true;
            }
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
    } else if path_str.contains("/character/") || prompt.contains("character") {
        "character".to_string()
    } else if path_str.contains("/number/") || prompt.contains("number") {
        "number".to_string()
    } else if path_str.contains("/sessions/prompts")
        || (!prompt.trim().is_empty() && path_str.contains("/sessions/"))
    {
        "sessions/prompts".to_string()
    } else if path_str.contains("/sessions/") || session_id.starts_with("sess_") {
        "sessions/system".to_string()
    } else {
        "cortex".to_string()
    };

    let word_count = prompt.split_whitespace().count();

    // Locate paired log file
    let mut log_path = path.to_path_buf();
    log_path.set_extension("log");
    let paired_log = if log_path.exists() {
        Some(log_path)
    } else if path_str.contains("/scroll/") {
        let alt = PathBuf::from(path_str.replace("/scroll/", "/log/"));
        let mut alt_log = alt;
        alt_log.set_extension("log");
        if alt_log.exists() {
            Some(alt_log)
        } else {
            None
        }
    } else {
        None
    };

    Some(IndexedSession {
        session_id,
        timestamp_micros,
        timestamp_id,
        category,
        final_state,
        failpoint_count,
        repair_count,
        word_count,
        scroll_path: path.to_path_buf(),
        log_path: paired_log,
        prompt,
        hardware_tripped,
    })
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Test Verification Proofs & Query Invariant Seals )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_indexing_directory_and_multikey_aliases() {
        let temp_dir = std::env::temp_dir().join(format!(
            "omnicode_query_test_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let scroll_dir = temp_dir.join("character");
        fs::create_dir_all(&scroll_dir).unwrap();

        let scroll_content = r#"
Metadata {
    session_id: "test_sess_alpha",
    timestamp: "1792000000000000_0000_test_sess_alpha",
    user_prompt: "Initialize character memory vessel",
}
Opening {
    Cell [ 1 ] & { 5 Trits :: 1 Byte },
}
Body {
    Cell [ 1 ] := 'A',
}
Closing {
    commit_status [ <PROCEED> ],
}
"#;
        let file_path = scroll_dir.join("1792000000000000_0000_test_sess_alpha.scroll");
        let mut file = fs::File::create(&file_path).unwrap();
        file.write_all(scroll_content.as_bytes()).unwrap();

        let mut engine = EpisodicQueryEngine::new();
        let count = engine.index_directory(&temp_dir).unwrap();
        assert_eq!(count, 1);

        // 1. Direct session ID lookup
        let s1 = engine.find_by_session("test_sess_alpha").expect("Found by ID");
        assert_eq!(s1.final_state, QuadState::Proceed);
        assert_eq!(s1.category, "character");

        // 2. Full stem lookup
        let s2 = engine
            .find_by_session("1792000000000000_0000_test_sess_alpha")
            .expect("Found by stem");
        assert_eq!(s2.session_id, "test_sess_alpha");

        // 3. Suffix lookup
        let s3 = engine
            .find_by_session("0000_test_sess_alpha")
            .expect("Found by suffix");
        assert_eq!(s3.session_id, "test_sess_alpha");

        // 4. Inverted keyword search
        let kw_results = engine.search_keyword("vessel");
        assert_eq!(kw_results.len(), 1);
        assert_eq!(kw_results[0].session_id, "test_sess_alpha");

        // 5. Compound criteria query
        let criteria = SessionCriteria::new()
            .with_category("character")
            .with_state(QuadState::Proceed)
            .with_keyword("memory");
        let q_res = engine.query(&criteria);
        assert_eq!(q_res.sessions.len(), 1);
        assert_eq!(q_res.pass_rate(), 100.0);

        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_range_scan_and_state_filtering() {
        let mut engine = EpisodicQueryEngine::new();
        let s1 = IndexedSession {
            session_id: "s1".to_string(),
            timestamp_micros: 10_000,
            timestamp_id: "10000".to_string(),
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
            timestamp_micros: 20_000,
            timestamp_id: "20000".to_string(),
            category: "cat".to_string(),
            final_state: QuadState::Rollback,
            failpoint_count: 1,
            repair_count: 0,
            word_count: 1,
            scroll_path: PathBuf::from("s2.scroll"),
            log_path: None,
            prompt: "p2".to_string(),
            hardware_tripped: false,
        };
        let s3 = IndexedSession {
            session_id: "s3".to_string(),
            timestamp_micros: 30_000,
            timestamp_id: "30000".to_string(),
            category: "cat".to_string(),
            final_state: QuadState::Proceed,
            failpoint_count: 0,
            repair_count: 0,
            word_count: 1,
            scroll_path: PathBuf::from("s3.scroll"),
            log_path: None,
            prompt: "p3".to_string(),
            hardware_tripped: false,
        };

        engine.sessions = vec![s1, s2, s3];

        let ranged = engine.range_scan(15_000, 25_000);
        assert_eq!(ranged.len(), 1);
        assert_eq!(ranged[0].session_id, "s2");

        let rollbacks = engine.filter_by_state(QuadState::Rollback);
        assert_eq!(rollbacks.len(), 1);
        assert_eq!(rollbacks[0].session_id, "s2");

        let clean = engine.filter_by_failpoint(false);
        assert_eq!(clean.len(), 2);
    }
}

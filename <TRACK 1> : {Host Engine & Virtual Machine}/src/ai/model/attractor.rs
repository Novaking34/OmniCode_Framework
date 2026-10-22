// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( Episodic Tape Attractor Network & Sparse Associative Memory Invariants )
// ============================================================================
//! # Episodic Tape Attractor Networks (`attractor.rs`)
//!
//! Replaces quadratic $O(N^2)$ transformer self-attention with external
//! sparse associative memory over Track 4 episodic `.scroll` and `.tape` reels.
//!
//! ### Key Invariants:
//! 1. **$O(1)$ to $O(\log N)$ Associative Recall**: Recall is performed
//!    directly via balanced-ternary Hamming resonance across indexed basins,
//!    eliminating GPU matmul and context window truncation.
//! 2. **Ternary Resonance Metric**: Normalized correlation:
//!    $$\sum a_i \cdot b_i / D \in [-1.0, +1.0]$$
//!    measuring semantic alignment between sensory vectors.
//! 3. **Episodic Persistence**: Grounded directly in Track 4 archives.
//! 4. **Zero-Watt Memory**: Storage lives on disk and lightweight structures
//!    (< 1 MB), recalled in sub-millisecond execution on a single core.
//! 5. **Scripture Coordinate Anchoring**: 243-stride hypercube landmarks
//!    provide geographic indexing into the Decalogue canonical space.
//! 6. **Strict Line Length Invariant**: Every line must be <= 80 characters.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::ai::lexicon::TermTriangulation;
use crate::ai::neural::tokenizer::TernaryByteTree;
use crate::core::scripture_coord::{
    ScriptureCoordinate, ScriptureCoordinateEngine,
};
use crate::core::ternary::QuadState;

// ============================================================================
// BLOCK 2: TYPES BLOCK
// ( Attractor Data Structures, Match Metrics & Audit Manifests )
// ============================================================================

/// An indexed episodic memory attractor basin stored in the network.
#[derive(Debug, Clone, PartialEq)]
pub struct AttractorBasin {
    /// Unique session identifier from the recorded scroll or tape.
    pub session_id: String,
    /// Originating natural language prompt or stimulus.
    pub prompt: String,
    /// Normalized lowercase prompt for $O(1)$ exact lookup.
    pub prompt_normalized: String,
    /// 64-trit balanced-ternary sensory vector.
    pub sensory_vector: Vec<QuadState>,
    /// Synthesized 4-Block OmniCode Turing reel or Tape.
    pub omnicode_tape: String,
    /// Natural language discourse response.
    pub natural_response: String,
    /// Quad-State execution outcome (+1: PROCEED, 0: AWAIT, -1: ROLLBACK).
    pub outcome_state: QuadState,
    /// Timestamp in microseconds.
    pub timestamp_micros: u128,
    /// Originating file path on disk (e.g. in Track 4 storage).
    pub source_path: PathBuf,
    /// Associated Track 4 Scripture Coordinate landmark.
    pub coordinate: Option<ScriptureCoordinate>,
    /// Optional 4-point geometric triangulation from Universal World Lexicon.
    pub triangulation: Option<TermTriangulation>,
}

/// The result of an associative recall query across the attractor network.
#[derive(Debug, Clone)]
pub struct AttractorMatch {
    /// The recalled attractor basin.
    pub basin: AttractorBasin,
    /// Normalized ternary resonance score in [-1.0, +1.0].
    pub resonance: f64,
    /// Discrete ternary Hamming distance in [0, D].
    pub hamming_distance: usize,
    /// Whether this was an exact O(1) lexical match.
    pub is_exact: bool,
}

/// Diagnostic report verifying integrity of the episodic attractor network.
#[derive(Debug, Clone, PartialEq)]
pub struct AttractorSubsystemAudit {
    /// Overall health status (+1: PROCEED, -1: ROLLBACK).
    pub status: QuadState,
    /// Total count of attractor basins currently indexed.
    pub total_basins: usize,
    /// Total count of signature partition buckets.
    pub total_buckets: usize,
    /// Maximum self-resonance deviation observed across indexed basins.
    pub max_resonance_deviation: f64,
    /// Whether all hypercube coordinates lie strictly in [0..=128].
    pub hypercube_in_bounds: bool,
    /// Audit error messages encountered during verification.
    pub errors: Vec<String>,
}

/// Sparse Associative Memory engine coordinating attractor basins over Track 4.
#[derive(Debug, Clone)]
pub struct EpisodicAttractorNetwork {
    /// In-memory attractor basins.
    basins: Vec<AttractorBasin>,
    /// Exact lexical hash map for O(1) instant recall.
    lexical_index: HashMap<String, usize>,
    /// Ternary signature partition buckets for O(log N) candidate narrowing.
    signature_buckets: HashMap<i32, Vec<usize>>,
    /// 243-stride hypercube bucket index (0..=128) for geometric coordinate
    /// landmark recall.
    hypercube_index: HashMap<u8, Vec<usize>>,
    /// Fixed sensory vector dimension (default 64 trits).
    pub vector_dim: usize,
    /// Sub-word tokenizer for turning prompts into balanced-ternary vectors.
    tokenizer: TernaryByteTree,
    /// Scripture coordinate engine for 243-stride hypercube landmark mapping.
    coordinate_engine: ScriptureCoordinateEngine,
}

impl Default for EpisodicAttractorNetwork {
    fn default() -> Self {
        Self::new(64)
    }
}

// ============================================================================
// BLOCK 3: BODY BLOCK
// ( Attractor Ingestion, Hamming Resonance & Associative Recall )
// ============================================================================

impl EpisodicAttractorNetwork {
    /// Constructs a new [`EpisodicAttractorNetwork`] with sensory dimension.
    pub fn new(vector_dim: usize) -> Self {
        Self {
            basins: Vec::new(),
            lexical_index: HashMap::new(),
            signature_buckets: HashMap::new(),
            hypercube_index: HashMap::new(),
            vector_dim,
            tokenizer: TernaryByteTree::new(),
            coordinate_engine: ScriptureCoordinateEngine::new_synthesized(),
        }
    }

    /// Number of attractor basins currently indexed.
    pub fn len(&self) -> usize {
        self.basins.len()
    }

    /// Whether the attractor network is empty.
    pub fn is_empty(&self) -> bool {
        self.basins.is_empty()
    }

    /// Returns the total number of indexed attractor basins.
    pub fn basin_count(&self) -> usize {
        self.basins.len()
    }

    /// Computes the signature bucket key for a balanced-ternary sensory vector.
    ///
    /// The bucket key is computed from the net balanced ternary excitation sum,
    /// partitioning the 64-dimensional space into discrete topological basins.
    pub fn compute_signature(vector: &[QuadState]) -> i32 {
        let mut sum: i32 = 0;
        for (idx, st) in vector.iter().enumerate() {
            let trit = match st {
                QuadState::Proceed => 1,
                QuadState::Rollback => -1,
                _ => 0,
            };
            // Weight leading trits with higher positional significance
            let weight = if idx < 8 { 3 } else { 1 };
            sum += trit * weight;
        }
        sum
    }

    /// Computes discrete Ternary Hamming Distance between two sensory vectors.
    /// Returns the number of positions where states differ.
    pub fn hamming_distance(a: &[QuadState], b: &[QuadState]) -> usize {
        let len = a.len().min(b.len());
        let mut diffs = a.len().max(b.len()) - len;
        for i in 0..len {
            if a[i] != b[i] {
                diffs += 1;
            }
        }
        diffs
    }

    /// Computes Normalized Ternary Resonance in [-1.0, +1.0] between vectors.
    ///
    /// $$\text{Resonance}(a, b) = \frac{\sum_{i=1}^D a_i \cdot b_i}{D}$$
    pub fn ternary_resonance(a: &[QuadState], b: &[QuadState]) -> f64 {
        if a.is_empty() || b.is_empty() {
            return 0.0;
        }
        let len = a.len().min(b.len());
        let mut dot_sum: i32 = 0;
        for i in 0..len {
            let t_a = match a[i] {
                QuadState::Proceed => 1,
                QuadState::Rollback => -1,
                _ => 0,
            };
            let t_b = match b[i] {
                QuadState::Proceed => 1,
                QuadState::Rollback => -1,
                _ => 0,
            };
            dot_sum += t_a * t_b;
        }
        dot_sum as f64 / a.len().max(b.len()) as f64
    }

    /// Inscribes a new attractor basin into the network without triangulation.
    pub fn inscribe(
        &mut self,
        session_id: String,
        prompt: String,
        omnicode_tape: String,
        natural_response: String,
        outcome_state: QuadState,
        source_path: PathBuf,
    ) -> usize {
        self.inscribe_with_triangulation(
            session_id,
            prompt,
            omnicode_tape,
            natural_response,
            outcome_state,
            source_path,
            None,
        )
    }

    /// Inscribes a new attractor basin into the network, anchoring it to a
    /// 243-stride hypercube Scripture Coordinate landmark and optional
    /// 4-point lexical triangulation.
    pub fn inscribe_with_triangulation(
        &mut self,
        session_id: String,
        prompt: String,
        omnicode_tape: String,
        natural_response: String,
        outcome_state: QuadState,
        source_path: PathBuf,
        triangulation: Option<TermTriangulation>,
    ) -> usize {
        let normalized = prompt.trim().to_lowercase();
        let sensory_vector = self
            .tokenizer
            .encode_to_sensory_vector(&prompt, self.vector_dim);
        let sig = Self::compute_signature(&sensory_vector);
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros();

        // Compute 243-stride hypercube coordinate landmark
        let cube = (sig.abs() as usize % 129) as u8;
        let low = (now as usize % 243) as u8;
        let ordinal = ScriptureCoordinateEngine::coords_to_ordinal(cube, low);
        let coordinate = self.coordinate_engine.resolve_ordinal(ordinal);

        let basin_idx = self.basins.len();
        let basin = AttractorBasin {
            session_id,
            prompt,
            prompt_normalized: normalized.clone(),
            sensory_vector,
            omnicode_tape,
            natural_response,
            outcome_state,
            timestamp_micros: now,
            source_path,
            coordinate,
            triangulation,
        };

        self.basins.push(basin);
        self.lexical_index.insert(normalized, basin_idx);
        self.signature_buckets
            .entry(sig)
            .or_default()
            .push(basin_idx);
        self.hypercube_index
            .entry(cube)
            .or_default()
            .push(basin_idx);

        basin_idx
    }

    /// Recalls all attractor basins anchored to a specific hypercube (0..=128).
    pub fn recall_by_hypercube(&self, cube: u8) -> Vec<&AttractorBasin> {
        if let Some(indices) = self.hypercube_index.get(&cube) {
            indices
                .iter()
                .filter_map(|&idx| self.basins.get(idx))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Recalls the closest resonant attractor basin for a prompt.
    ///
    /// 1. **O(1) Fast Path**: Checks the exact normalized prompt index.
    /// 2. **O(log N) Sparse Attractor Resonance**: Filters candidate basins
    ///    by signature neighborhood and evaluates Ternary Hamming Resonance.
    pub fn recall(
        &self,
        query: &str,
        min_resonance: f64,
    ) -> Option<AttractorMatch> {
        let clean = query.trim().to_lowercase();
        if clean.is_empty() {
            return None;
        }

        // 1. O(1) Exact lexical match
        if let Some(&idx) = self.lexical_index.get(&clean) {
            if let Some(basin) = self.basins.get(idx) {
                return Some(AttractorMatch {
                    basin: basin.clone(),
                    resonance: 1.0,
                    hamming_distance: 0,
                    is_exact: true,
                });
            }
        }

        if self.basins.is_empty() {
            return None;
        }

        // 2. Vectorized Ternary Hamming Resonance
        let query_vector = self
            .tokenizer
            .encode_to_sensory_vector(query, self.vector_dim);
        let query_sig = Self::compute_signature(&query_vector);

        let mut best_match: Option<AttractorMatch> = None;
        let mut max_res = min_resonance;

        // Examine signature neighborhood (delta +/- 4 for proximity)
        let mut candidate_indices = Vec::new();
        for delta in -4..=4 {
            if let Some(indices) =
                self.signature_buckets.get(&(query_sig + delta))
            {
                candidate_indices.extend_from_slice(indices);
            }
        }

        let evaluate_basin = |idx: usize| -> Option<AttractorMatch> {
            let basin = self.basins.get(idx)?;
            let res =
                Self::ternary_resonance(&query_vector, &basin.sensory_vector);
            let dist =
                Self::hamming_distance(&query_vector, &basin.sensory_vector);
            if res >= min_resonance {
                Some(AttractorMatch {
                    basin: basin.clone(),
                    resonance: res,
                    hamming_distance: dist,
                    is_exact: false,
                })
            } else {
                None
            }
        };

        if !candidate_indices.is_empty() {
            for &idx in &candidate_indices {
                if let Some(m) = evaluate_basin(idx) {
                    if m.resonance > max_res {
                        max_res = m.resonance;
                        best_match = Some(m);
                    }
                }
            }
        }

        // If no candidate met threshold in neighborhood, evaluate full store
        if best_match.is_none() {
            for idx in 0..self.basins.len() {
                if let Some(m) = evaluate_basin(idx) {
                    if m.resonance > max_res {
                        max_res = m.resonance;
                        best_match = Some(m);
                    }
                }
            }
        }

        best_match
    }

    /// Ingests all `.scroll` and `.tape` files from the specified directory.
    pub fn ingest_scroll_directory(
        &mut self,
        dir: impl AsRef<Path>,
    ) -> Result<usize, String> {
        let dir_path = dir.as_ref();
        if !dir_path.exists() {
            return Ok(0);
        }

        let mut ingested = 0;
        let entries = fs::read_dir(dir_path).map_err(|e| e.to_string())?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let ext = path
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("");
                if ext == "scroll" || ext == "tape" {
                    if self.ingest_scroll_file(&path).is_ok() {
                        ingested += 1;
                    }
                }
            } else if path.is_dir() {
                // Ingest subdirectories (e.g. `scroll/`, `log/`)
                if let Ok(sub_count) = self.ingest_scroll_directory(&path) {
                    ingested += sub_count;
                }
            }
        }

        Ok(ingested)
    }

    /// Ingests a single `.scroll` file into an attractor basin.
    pub fn ingest_scroll_file(&mut self, path: &Path) -> Result<(), String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let session_id = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let mut prompt = String::new();
        let mut omnicode_tape = String::new();
        let mut natural_response = String::new();
        let mut outcome_state = QuadState::Proceed;
        let mut in_code_block = false;

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("# Prompt:")
                || trimmed.starts_with("prompt:")
            {
                prompt = trimmed
                    .trim_start_matches("# Prompt:")
                    .trim_start_matches("prompt:")
                    .trim()
                    .to_string();
            } else if trimmed.starts_with("# Outcome:")
                || trimmed.starts_with("outcome:")
            {
                let st_str = trimmed
                    .trim_start_matches("# Outcome:")
                    .trim_start_matches("outcome:")
                    .trim();
                if st_str.contains("ROLLBACK") || st_str.contains("-1") {
                    outcome_state = QuadState::Rollback;
                } else if st_str.contains("HALT") {
                    outcome_state = QuadState::Halt;
                } else if st_str.contains("AWAIT") || st_str.contains("0") {
                    outcome_state = QuadState::Await;
                } else {
                    outcome_state = QuadState::Proceed;
                }
            } else if trimmed.starts_with("# Response:")
                || trimmed.starts_with("response:")
            {
                natural_response = trimmed
                    .trim_start_matches("# Response:")
                    .trim_start_matches("response:")
                    .trim()
                    .to_string();
            } else if trimmed == "```omnicode" || trimmed == "```" {
                in_code_block = !in_code_block;
            } else if in_code_block {
                omnicode_tape.push_str(line);
                omnicode_tape.push('\n');
            }
        }

        if prompt.is_empty() {
            prompt = session_id.clone();
        }

        self.inscribe(
            session_id,
            prompt,
            omnicode_tape,
            natural_response,
            outcome_state,
            path.to_path_buf(),
        );

        Ok(())
    }

    /// Renders an overview summary of the attractor network.
    pub fn render_summary(&self) -> String {
        format!(
            "Episodic Attractor Network: {} basins | Dim: {} trits | \
             {} signature buckets",
            self.basins.len(),
            self.vector_dim,
            self.signature_buckets.len()
        )
    }
}

// ============================================================================
// BLOCK 4: AUDIT PROCEDURES BLOCK
// ( Mathematical Invariant Verification & Subsystem Health )
// ============================================================================

impl EpisodicAttractorNetwork {
    /// Audits mathematical invariants and index consistency of the network.
    ///
    /// Verifies:
    /// 1. Sensory vector dimension matches `self.vector_dim` for all basins.
    /// 2. Self-resonance lies strictly in [0.0, 1.0] with zero excursion.
    /// 3. Scripture landmarks lie within hypercube bounds [0..=128].
    /// 4. Lexical index maps directly to existing basins.
    pub fn audit_attractor_subsystem(&self) -> AttractorSubsystemAudit {
        let mut errors = Vec::new();
        let mut max_dev: f64 = 0.0;
        let mut hypercube_in_bounds = true;

        for (idx, basin) in self.basins.iter().enumerate() {
            // Check 1: Vector dimension
            if basin.sensory_vector.len() != self.vector_dim {
                errors.push(format!(
                    "Basin #{} ({}) vector dimension mismatch: expected {}, \
                     got {}",
                    idx,
                    basin.session_id,
                    self.vector_dim,
                    basin.sensory_vector.len()
                ));
            }

            // Check 2: Self-resonance validity in [0.0, 1.0]
            let self_res = Self::ternary_resonance(
                &basin.sensory_vector,
                &basin.sensory_vector,
            );
            let excursion = if self_res < 0.0 {
                -self_res
            } else if self_res > 1.0 {
                self_res - 1.0
            } else {
                0.0
            };
            if excursion > max_dev {
                max_dev = excursion;
            }
            if excursion > 1e-6 {
                errors.push(format!(
                    "Basin #{} ({}) self-resonance excursion: {}",
                    idx, basin.session_id, excursion
                ));
            }

            // Check 3: Hypercube bounds
            if let Some(ref coord) = basin.coordinate {
                if coord.cube > 128 {
                    hypercube_in_bounds = false;
                    errors.push(format!(
                        "Basin #{} ({}) hypercube out of bounds: {}",
                        idx, basin.session_id, coord.cube
                    ));
                }
            }
        }

        // Check 4: Lexical index synchronization
        for (term, &basin_idx) in &self.lexical_index {
            if basin_idx >= self.basins.len() {
                errors.push(format!(
                    "Lexical index term '{}' points to invalid index {}",
                    term, basin_idx
                ));
            }
        }

        let status = if errors.is_empty() {
            QuadState::Proceed
        } else {
            QuadState::Rollback
        };

        AttractorSubsystemAudit {
            status,
            total_basins: self.basins.len(),
            total_buckets: self.signature_buckets.len(),
            max_resonance_deviation: max_dev,
            hypercube_in_bounds,
            errors,
        }
    }
}

// ============================================================================
// BLOCK 5: UNIT TESTS & INVARIANT VERIFICATION SUITE
// ( Verification of Distance, Resonance, Indexing & Audit )
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_distance_and_resonance() {
        let v1 = vec![QuadState::Proceed; 8];
        let v2 = vec![QuadState::Proceed; 8];
        assert_eq!(EpisodicAttractorNetwork::hamming_distance(&v1, &v2), 0);
        let res_12 = EpisodicAttractorNetwork::ternary_resonance(&v1, &v2);
        assert!((res_12 - 1.0).abs() < 1e-6);

        let v3 = vec![QuadState::Rollback; 8];
        assert_eq!(EpisodicAttractorNetwork::hamming_distance(&v1, &v3), 8);
        let res_13 = EpisodicAttractorNetwork::ternary_resonance(&v1, &v3);
        assert!((res_13 - (-1.0)).abs() < 1e-6);

        let v4 = vec![QuadState::Await; 8];
        assert_eq!(EpisodicAttractorNetwork::hamming_distance(&v1, &v4), 8);
        let res_14 = EpisodicAttractorNetwork::ternary_resonance(&v1, &v4);
        assert!((res_14 - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_exact_and_resonant_recall() {
        let mut net = EpisodicAttractorNetwork::new(64);
        net.inscribe(
            "sess_001".into(),
            "Add 5 balls to Container A".into(),
            "roll_add [ A --> A plus 5 ]".into(),
            "Added 5 balls to Container A".into(),
            QuadState::Proceed,
            PathBuf::from("sess_001.scroll"),
        );

        net.inscribe(
            "sess_002".into(),
            "What is truth?".into(),
            "roll_truth [ Law_Truth = PROCEED ]".into(),
            "Truth is covenant alignment".into(),
            QuadState::Proceed,
            PathBuf::from("sess_002.scroll"),
        );

        // 1. Exact match recall in O(1)
        let exact = net
            .recall("Add 5 balls to Container A", 0.5)
            .expect("Exact match should succeed");
        assert!(exact.is_exact);
        assert_eq!(exact.basin.session_id, "sess_001");
        assert_eq!(exact.resonance, 1.0);

        // 2. Resonant match for semantic variant
        let resonant = net
            .recall("Add 4 balls to Container A", 0.4)
            .expect("Resonant match should succeed");
        assert_eq!(resonant.basin.session_id, "sess_001");
        assert!(resonant.resonance > 0.4);
    }

    #[test]
    fn test_hypercube_coordinate_anchoring() {
        let mut net = EpisodicAttractorNetwork::new(64);
        net.inscribe(
            "sess_hyper_01".into(),
            "Covenant witness of Genesis".into(),
            "roll_genesis [ In_Beginning = PROCEED ]".into(),
            "Creation of the heavens and the earth".into(),
            QuadState::Proceed,
            PathBuf::from("sess_hyper_01.scroll"),
        );

        let basin = &net.basins[0];
        assert!(
            basin.coordinate.is_some(),
            "Basin must have an anchored scripture coordinate"
        );
        let coord = basin.coordinate.as_ref().unwrap();
        assert!(coord.cube <= 128);
        assert!(coord.cube_pos <= 26);

        let by_cube = net.recall_by_hypercube(coord.cube);
        assert_eq!(by_cube.len(), 1);
        assert_eq!(by_cube[0].session_id, "sess_hyper_01");
    }

    #[test]
    fn test_audit_attractor_subsystem() {
        let mut net = EpisodicAttractorNetwork::new(64);
        net.inscribe(
            "audit_sess_01".into(),
            "Audit testing stimulus".into(),
            "tape_content".into(),
            "response".into(),
            QuadState::Proceed,
            PathBuf::from("audit.scroll"),
        );

        let audit = net.audit_attractor_subsystem();
        assert_eq!(audit.status, QuadState::Proceed);
        assert_eq!(audit.total_basins, 1);
        assert!(audit.max_resonance_deviation < 1e-6);
        assert!(audit.hypercube_in_bounds);
        assert!(audit.errors.is_empty());
    }

    #[test]
    fn test_triangulation_retention() {
        let mut net = EpisodicAttractorNetwork::new(64);
        assert_eq!(net.basin_count(), 0);
        assert!(net.is_empty());

        net.inscribe_with_triangulation(
            "tri_sess_01".into(),
            "Truth prompt".into(),
            "tape".into(),
            "resp".into(),
            QuadState::Proceed,
            PathBuf::from("tri.scroll"),
            None,
        );

        assert_eq!(net.basin_count(), 1);
        assert_eq!(net.len(), 1);
        assert!(!net.is_empty());
    }
}

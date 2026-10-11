// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( AI Memory Store Identity, Moral Ground Truth & Thought Formulation Invariants )
// ==============================================================================
//! OmniCode AI Model: Cognitive Memory & Thought Formulation Store
//!
//! # Native Language Equivalence: AI <-> OS
//! In conventional systems, the AI model is an opaque external black box wrapped
//! in Python/CUDA, while the operating system is a distinct kernel written in C.
//!
//! In OmniCode, **OmniCode is the unified native language of both the internal AI model
//! and the Operating System**:
//! - The OS scheduler dispatches **Tapes** and **Rolls** across physical hardware channels.
//! - The hardware memory partitions are **Vessels** allocated via `v--`.
//! - The internal AI model thinks natively in **OmniCode Tapes**.
//! - The AI's episodic experience and long-term memory are stored as **Scrolls** (`.scroll`).
//! - The diagnostic triad forms the AI's cognitive nervous system:
//!     1. [`DiagnosticLogger`]: Continuous sensory perception and episodic scribe.
//!     2. [`OmniDebugger`]: Conscience and boundary/covenant auditor.
//!     3. [`Restorator`]: Immune system and self-healing grace restoring degraded state.
//!
//! # Ground Truth Dataset
//! The foundation of the AI's memory is grounded in the public domain [`BiblicalLexicon`]
//! (Genesis, Creation, Light, Wisdom, Justice, Covenant Laws, Causal Invariants). Even though
//! compact initially, it supplies the moral compass and foundational ontology from which the
//! AI formulates righteous thoughts and interprets system experiences.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use crate::ai::lexicon::{BiblicalLexicon, LexiconDomain, LexiconEntry, MoralEvaluation};
use crate::ai::model::learner::{AILearner, EpisodicExperience};
use crate::core::ternary::QuadState;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Memory Data Structures, Thought Representation & Cardinal Classifications )
// ==============================================================================

/// A structured thought formulated natively in OmniCode by the internal AI model.
#[derive(Debug, Clone)]
pub struct ThoughtRecord {
    /// Unique thought identifier (e.g. `th_1728..._0001`).
    pub id: String,
    /// The originating stimulus or prompt that triggered the thought.
    pub premise: String,
    /// Ontological category anchored in the foundational world model.
    pub domain: LexiconDomain,
    /// Scripture citation or axiomatic anchor underpinning the thought.
    pub biblical_anchor: Option<String>,
    /// Native, valid OmniCode Tape expressing the internal thought!
    pub omnicode_tape: String,
    /// Deliberated Quad-State outcome (+1: PROCEED, 0: AWAIT, -1: ROLLBACK, NULL: HALT).
    pub quad_state: QuadState,
    /// Cardinal Clause: WHO (Agent identity).
    pub who: String,
    /// Cardinal Clause: WHAT (Entity or action substance).
    pub what: String,
    /// Cardinal Clause: WHEN (Temporal or lifecycle precondition).
    pub when_clause: String,
    /// Cardinal Clause: WHERE (Target Vessel or spatial location).
    pub where_clause: String,
    /// Cardinal Clause: WHY (Teleological purpose or covenant reason).
    pub why: String,
    /// Cardinal Clause: HOW (Operational methodology and directional vector).
    pub how: String,
    /// Monotonic timestamp in microseconds.
    pub timestamp_micros: u128,
}

/// An associative link between a concept and an axiomatic scripture or scroll anchor.
#[derive(Debug, Clone)]
pub struct MemoryAssociation {
    /// Conceptual keyword.
    pub concept: String,
    /// Biblical passage or scroll identifier.
    pub anchor: String,
    /// Explanatory relational meaning.
    pub relation: String,
    /// State resonance.
    pub quad_state_resonance: QuadState,
}

/// The cognitive memory repository of the internal AI model.
#[derive(Debug, Clone)]
pub struct AIMemory {
    /// Axiomatic world model and moral compass (Bible ground truth).
    pub lexicon: BiblicalLexicon,
    /// Directory where episodic `.scroll` files reside.
    pub scroll_dir: PathBuf,
    /// Digested episodic experiences from past scrolls.
    pub episodic_experiences: Vec<EpisodicExperience>,
    /// Chronological ledger of formulated thoughts.
    pub thoughts: Vec<ThoughtRecord>,
    /// Concept association graph connecting terms with foundational anchors.
    pub associations: HashMap<String, Vec<MemoryAssociation>>,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Memory Ingestion, Associative Graph & Thought Formulation Engine )
// ==============================================================================

impl AIMemory {
    /// Constructs a new [`AIMemory`] repository anchored to the given scroll directory.
    pub fn new(scroll_dir: impl AsRef<Path>) -> Self {
        let dir = scroll_dir.as_ref().to_path_buf();
        if !dir.exists() {
            fs::create_dir_all(&dir).ok();
        }

        let lexicon = BiblicalLexicon::new();
        let mut memory = Self {
            lexicon,
            scroll_dir: dir,
            episodic_experiences: Vec::new(),
            thoughts: Vec::new(),
            associations: HashMap::new(),
        };

        memory.seed_primordial_associations();
        memory.digest_scrolls().ok();
        memory
    }

    /// Seeds foundational concepts from the Biblical Lexicon into the associative graph.
    pub fn seed_primordial_associations(&mut self) {
        for entry in self.lexicon.all_entries() {
            let assoc = MemoryAssociation {
                concept: entry.term.to_string(),
                anchor: entry.kjv_passage.to_string(),
                relation: entry.definition.to_string(),
                quad_state_resonance: entry.quad_state_resonance,
            };
            self.associations
                .entry(entry.term.to_string())
                .or_default()
                .push(assoc);
        }

        // Add explicit OS and computing associations grounded in scripture
        self.associate(
            "storehouse",
            "Malachi 3:10 / Genesis 41:48",
            "A bounded partition (Vessel) preserving vital sustenance and system resources",
            QuadState::Proceed,
        );
        self.associate(
            "cell",
            "Proverbs 24:3-4",
            "Through wisdom is an house builded; a fundamental unit of physical memory",
            QuadState::Proceed,
        );
        self.associate(
            "restoration",
            "Psalm 23:3 / Galatians 6:1",
            "He restoreth my soul; the self-healing immune recovery of corrupted state",
            QuadState::Proceed,
        );
        self.associate(
            "tape",
            "Ezekiel 2:9-10 / Revelation 5:1",
            "A roll of a book written within and without; sequential stream of causal actions",
            QuadState::Proceed,
        );
    }

    /// Registers a custom association between a concept and an anchor.
    pub fn associate(&mut self, concept: &str, anchor: &str, relation: &str, state: QuadState) {
        let assoc = MemoryAssociation {
            concept: concept.to_string(),
            anchor: anchor.to_string(),
            relation: relation.to_string(),
            quad_state_resonance: state,
        };
        self.associations
            .entry(concept.to_lowercase())
            .or_default()
            .push(assoc);
    }

    /// Ingests all historical `.scroll` files from the scroll store into episodic memory.
    pub fn digest_scrolls(&mut self) -> Result<usize, String> {
        let learner = AILearner::new(&self.scroll_dir);
        let summary = learner.digest_experiences()?;
        let count = summary.experiences.len();
        self.episodic_experiences = summary.experiences;
        Ok(count)
    }

    /// Queries the associative memory graph for links matching the query.
    pub fn recall_associations(&self, query: &str) -> Vec<&MemoryAssociation> {
        let clean = query.trim().to_lowercase();
        let mut results = Vec::new();
        for (k, assocs) in &self.associations {
            if k.contains(&clean) || clean.contains(k) {
                for a in assocs {
                    results.push(a);
                }
            }
        }
        results
    }

    /// Looks up relevant Biblical Lexicon entries matching a keyword.
    pub fn recall_scripture(&self, query: &str) -> Vec<&LexiconEntry> {
        self.lexicon.search(query)
    }

    /// Formulates a native OmniCode Thought from memory given an input stimulus.
    ///
    /// The formulated thought uses the 8 parts of speech and the 6 Cardinal Clauses:
    /// `WHO`, `WHAT`, `WHEN`, `WHERE`, `WHY`, and `HOW`.
    pub fn formulate_thought(&mut self, stimulus: &str) -> ThoughtRecord {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros();

        let count = self.thoughts.len() + 1;
        let thought_id = format!("th_{now}_{count:04}");

        // 1. Deliberate stimulus against the Biblical Lexicon
        let moral_eval: MoralEvaluation = self.lexicon.deliberate(stimulus);

        // 2. Determine best matched entry or domain
        let (matched_term, domain, biblical_anchor, moral_guidance): (&str, LexiconDomain, Option<String>, &str) = if let Some(first) = moral_eval.matched_entries.first() {
            (first.term, first.domain, Some(first.kjv_passage.to_string()), first.moral_guidance)
        } else {
            ("order", LexiconDomain::Cosmology, Some("Genesis 1:1 - 'In the beginning God created the heaven and the earth.'".to_string()), "Execute with righteousness and lawful boundaries.")
        };

        // 3. Construct the 6 Cardinal Classification Clauses
        let who = "Internal_AI".to_string();
        let what = format!("CognitiveThought_{matched_term}");
        let when_clause = "Opening".to_string();
        let where_clause = "Cortex_Memory".to_string();
        let why = biblical_anchor.clone().unwrap_or_else(|| "Order and covenant equilibrium".to_string());
        let how = format!("{} : {}", moral_eval.state.macro_op(), moral_guidance);

        // 4. Synthesize Canonical OmniCode Thought Tape
        let omnicode_tape = format!(
r#"[Thought.{thought_id}] : WHO {{{who}}} --> DELIBERATE {{{what}}}
    WHEN {{{when_clause}}}
    WHERE {{{where_clause}}} = [ {matched_term} ]
    WHY ( {why} )
    HOW [ {how_state} : {guidance} ]
    :: < manifest > ( {guidance_short} )"#,
            thought_id = thought_id,
            who = who,
            what = what,
            when_clause = when_clause,
            where_clause = where_clause,
            matched_term = matched_term,
            why = why.replace('\n', " "),
            how_state = moral_eval.state.macro_op(),
            guidance = moral_guidance.replace('\n', " "),
            guidance_short = moral_guidance.chars().take(40).collect::<String>()
        );

        let record = ThoughtRecord {
            id: thought_id,
            premise: stimulus.to_string(),
            domain,
            biblical_anchor,
            omnicode_tape,
            quad_state: moral_eval.state,
            who,
            what,
            when_clause,
            where_clause,
            why,
            how,
            timestamp_micros: now,
        };

        self.thoughts.push(record.clone());
        record
    }

    /// Inscribes a formulated thought into a physical `.scroll` file in the given directory.
    pub fn inscribe_thought_scroll(
        &self,
        thought: &ThoughtRecord,
        out_dir: &Path,
    ) -> Result<PathBuf, String> {
        if !out_dir.exists() {
            fs::create_dir_all(out_dir)
                .map_err(|e| format!("Failed to create directory {:?}: {e}", out_dir))?;
        }

        let file_path = out_dir.join(format!("{}.scroll", thought.id));
        let scroll_content = format!(
r#"# ==============================================================================
# BLOCK 1: METADATA BLOCK (THE COGNITIVE SCROLL)
# ( Thought Record ID, Premise Intent, Domain, and Memory Snapshot )
# ==============================================================================
Scroll < CognitiveThought.{id} > ( {premise} ) {{
    version: < "1.0.0" >,
    system: < OmniCode.AI.Model >,
    thought_id: < "{id}" >,
    domain: < "{domain:?}" >,
    timestamp: < {timestamp} >,
    cognitive_mode: < MemoryFormulation >
}}

# ==============================================================================
# BLOCK 2: OPENING BLOCK (EXPERIENCE INITIALIZATION)
# ( Biblical Ground Truth Anchor & Invariant Preconditions )
# ==============================================================================
Opening {{
    import: [ Core.Arithmetic :: Core.Logic :: Core.Vectors ],
    preconditions: [ {state} ],
    biblical_anchor: < "{anchor}" >
}}

stimulus_premise < "{premise}" >

# ==============================================================================
# BLOCK 3: BODY BLOCK (THE THOUGHT TAPE)
# ( Canonical 6-Cardinal Clause Thought Reel Formulated from Memory )
# ==============================================================================
Body [
    thought_reel [
{indented_tape}
    ]
]

# ==============================================================================
# BLOCK 4: CLOSING BLOCK (EPISODIC COMMIT)
# ( Deliberated State and Moral Conscience Seal )
# ==============================================================================
Closing {{
    commit_status [ <{state}> ],
    machine_utterance ( "Deliberated thought anchored in truth: {state}." )
}}
"#,
            id = thought.id,
            premise = thought.premise,
            domain = thought.domain,
            timestamp = thought.timestamp_micros,
            state = thought.quad_state.macro_op(),
            anchor = thought.biblical_anchor.as_deref().unwrap_or("None").replace('"', "\\\""),
            indented_tape = thought
                .omnicode_tape
                .lines()
                .map(|l| format!("        {l}"))
                .collect::<Vec<_>>()
                .join("\n")
        );

        fs::write(&file_path, scroll_content)
            .map_err(|e| format!("Failed to write thought scroll to {:?}: {e}", file_path))?;

        Ok(file_path)
    }

    /// Returns a slice of the most recently formulated thoughts.
    pub fn recent_thoughts(&self, limit: usize) -> &[ThoughtRecord] {
        let len = self.thoughts.len();
        if len <= limit {
            &self.thoughts[..]
        } else {
            &self.thoughts[len - limit..]
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Default Trait Implementation and Subsystem Sealing )
// ==============================================================================

impl Default for AIMemory {
    fn default() -> Self {
        let dir = if Path::new("storage/scrolls").exists() {
            "storage/scrolls"
        } else if Path::new("runtime/scrolls").exists() {
            "runtime/scrolls"
        } else {
            "scrolls"
        };
        Self::new(dir)
    }
}

// AI Memory subsystem sealed and active.

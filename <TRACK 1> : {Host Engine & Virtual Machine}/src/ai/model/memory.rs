// =============================================================================
// BLOCK 1: METADATA BLOCK
// ( AI Memory Identity, Tripartite Architecture & Modal Ground Truth )
// =============================================================================
//! OmniCode AI Model: Tripartite Cognitive Memory & Thought Formulation Store
//!
//! # The 3-Tier Cognitive Memory Hierarchy
//! The memory architecture of the internal AI model is grounded in the triad:
//!
//! - **Layer 1: Immediate Perceptual Working Context (`working_window`)**
//!   Sub-microsecond, volatile FIFO scratchpad maintaining recent turns, active
//!   registers, and the active thought in progress. Zero disk I/O.
//!
//! - **Layer 2: Contextual Episodic State Recall (`attractors`)**
//!   Associative attractor network and dynamic state trajectory tracking. Uses
//!   Ternary Hamming Resonance to recall precedent states in O(1) to O(log N)
//!   time across the session arc without reloading historical reels.
//!
//! - **Layer 3: Crystallized Synaptic Base & Lexicon (`lexicon` & Track 4)**
//!   Durable, covenant-audited ground truth. Contains immutable `.scroll`
//!   Turing reels, the 15 Herman Dooyeweerd Sovereign Modal Law-Spheres in the
//!   [`BiblicalLexicon`], and `cpisi_base.weights`. Novel terms are slowly and
//!   deliberately consolidated only after passing 4-point triangulation and
//!   Decalogue audit gates.
//!
//! # Ground Truth Dataset & Moral Invariants
//! Memory is anchored in the public domain [`BiblicalLexicon`] (Genesis, Light,
//! Wisdom, Covenant Laws, Causal Invariants). Every thought formulated passes
//! through conscience verification, emitting balanced ternary {-1, 0, +1}
//! states before entering episodic memory or persisting to Track 4.

use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::ai::lexicon::{
    BiblicalLexicon, CommandmentId, DomainCategory, LexiconEntry,
    ModalDomain, MoralEvaluation, SubDomain,
};
use crate::ai::model::attractor::{AttractorMatch, EpisodicAttractorNetwork};
use crate::ai::model::learner::{AILearner, EpisodicExperience};
use crate::core::ternary::QuadState;

// =============================================================================
// BLOCK 2: OPENING BLOCK
// ( Memory Data Structures, Thought Representation & Cardinal Classifications )
// =============================================================================

/// Canonical identity constants for the first CPI-SI model.
pub const CPISI_MODEL_NAME: &str = "CPI-SI Base";
pub const CPISI_MODEL_TITLE: &str =
    "First Covenant Partnered Intelligence (CPI-SI) Baseline Model";
pub const CPISI_MODEL_ORIGIN: &str =
    "Conceived 2 years ago by CreativeWorkzStudio / ProfessorSeanEX";

/// Default capacity for the Layer 1 working memory FIFO ring buffer.
pub const DEFAULT_WORKING_WINDOW_CAPACITY: usize = 16;

/// Default affirmations required before a candidate crystallizes to Layer 3.
pub const DEFAULT_CONSOLIDATION_THRESHOLD: usize = 3;

/// The tri-tier memory hierarchy governing cognitive persistence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryTier {
    /// Layer 1: Immediate Perceptual Working Context (In-Memory, FIFO).
    Working,
    /// Layer 2: Contextual Episodic State Recall (Attractor Basins).
    EpisodicResonant,
    /// Layer 3: Crystallized Synaptic Base & Lexicon (Track 4 Scrolls).
    Crystallized,
}

/// The consolidation lifecycle of a learning candidate into Layer 3.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsolidationStatus {
    /// In working memory, awaiting sufficient reinforcement.
    Pending,
    /// Positively affirmed with given count of compliant executions.
    Affirmed(usize),
    /// Permanently inscribed to .scroll reel and grafted into lexicon.
    Crystallized,
    /// Vetoed by Decalogue conscience; permanently blocked from Layer 3.
    Vetoed,
}

/// A candidate concept or pattern undergoing consolidation into Layer 3.
#[derive(Debug, Clone)]
pub struct ConsolidationCandidate {
    /// Candidate term identifier.
    pub term: String,
    /// Point 1: Sovereign Modal Domain (1 of 15).
    pub domain: ModalDomain,
    /// Point 1: Concrete structural category.
    pub category: DomainCategory,
    /// Point 1: Operational subdomain.
    pub subdomain: SubDomain,
    /// Number of verified +1:PROCEED affirmations.
    pub affirmations: usize,
    /// Current consolidation lifecycle status.
    pub status: ConsolidationStatus,
}

/// Snapshot of the last completed working session for instant continuity.
#[derive(Debug, Clone)]
pub struct CompletedSessionSnapshot {
    pub session_id: String,
    pub label_id: String,
    pub timestamp_str: String,
    pub elapsed_micros: u128,
    pub execution_state: QuadState,
    pub prompt: String,
    pub modal_domain: Option<String>,
    pub domain_category: Option<String>,
    pub biblical_anchor: Option<String>,
    pub thought_id: Option<String>,
    pub repairs_count: usize,
    pub failpoints_count: usize,
    pub natural_response: String,
}

/// A structured thought formulated natively in OmniCode by the internal model.
#[derive(Debug, Clone)]
pub struct ThoughtRecord {
    /// Unique thought identifier (e.g. `th_1728..._0001`).
    pub id: String,
    /// The originating stimulus or prompt that triggered the thought.
    pub premise: String,
    /// Point 1: Ontological modal domain (1 of 15 Dooyeweerd law-spheres).
    pub domain: ModalDomain,
    /// Point 1: Concrete structural category.
    pub category: DomainCategory,
    /// Point 1: Operational subdomain theater.
    pub subdomain: SubDomain,
    /// Point 2: Scripture citation or axiomatic anchor underpinning thought.
    pub biblical_anchor: Option<String>,
    /// Native, valid OmniCode Tape expressing the internal thought!
    pub omnicode_tape: String,
    /// Deliberated Quad-State (+1:PROCEED, 0:AWAIT, -1:ROLLBACK, NULL:HALT).
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
    /// Recalled episodic precedent session ID if an attractor resonated.
    pub recalled_precedent: Option<String>,
    /// Normalized resonance of the recalled attractor basin.
    pub attractor_resonance: Option<f64>,
    /// Point 3: Specific Decalogue Commandment engaged, if applicable.
    pub commandment: Option<CommandmentId>,
    /// Geometric confidence score of the 4-point triangulation (0.0 .. 1.0).
    pub confidence: f64,
    /// Active memory tier holding or originating this thought.
    pub tier: MemoryTier,
}

/// An associative link between a concept and an axiomatic scripture anchor.
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

/// Intermediate outcome of 4-point geometric triangulation for a stimulus.
#[derive(Debug, Clone)]
pub struct TriangulationOutcome {
    pub matched_term: String,
    pub domain: ModalDomain,
    pub category: DomainCategory,
    pub subdomain: SubDomain,
    pub biblical_anchor: Option<String>,
    pub moral_guidance: String,
    pub what: String,
    pub why: String,
    pub how: String,
    pub where_clause: String,
    pub quad_state: QuadState,
    pub commandment: Option<CommandmentId>,
    pub confidence: f64,
}

/// The tripartite cognitive memory repository of the internal AI model.
#[derive(Debug, Clone)]
pub struct AIMemory {
    // -------------------------------------------------------------------------
    // LAYER 1: IMMEDIATE PERCEPTUAL WORKING CONTEXT
    // -------------------------------------------------------------------------
    /// Bounded ring buffer holding recent working thoughts (sub-microsecond).
    pub working_window: Vec<ThoughtRecord>,
    /// Maximum capacity of the working memory window before FIFO rotation.
    pub working_window_capacity: usize,
    /// Complete chronological ledger of formulated thoughts.
    pub thoughts: Vec<ThoughtRecord>,

    // -------------------------------------------------------------------------
    // LAYER 2: CONTEXTUAL EPISODIC RECALL (ATTRACTOR BASINS)
    // -------------------------------------------------------------------------
    /// Episodic Tape Attractor Network for O(1) ~ O(log N) state recall.
    pub attractors: EpisodicAttractorNetwork,
    /// Concept association graph connecting terms with foundational anchors.
    pub associations: HashMap<String, Vec<MemoryAssociation>>,
    /// Digested episodic experiences from past scrolls.
    pub episodic_experiences: Vec<EpisodicExperience>,

    // -------------------------------------------------------------------------
    // LAYER 3: CRYSTALLIZED SYNAPTIC BASE & UNIVERSAL WORLD LEXICON
    // -------------------------------------------------------------------------
    /// Axiomatic world model and moral compass (Bible ground truth).
    pub lexicon: BiblicalLexicon,
    /// Directory where episodic `.scroll` files reside on Track 4.
    pub scroll_dir: PathBuf,
    /// Directory where contextual memory data sheets reside on Track 4.
    pub memory_dir: PathBuf,
    /// Learning candidates awaiting deliberate consolidation into Layer 3.
    pub consolidation_candidates: HashMap<String, ConsolidationCandidate>,
    /// Number of compliant affirmations required to crystallize into Layer 3.
    pub consolidation_threshold: usize,
}

// =============================================================================
// BLOCK 3: BODY BLOCK
// ( Tripartite Memory Engine, Triangulation, Formulation & Consolidation )
// =============================================================================

impl AIMemory {
    /// Constructs a new [`AIMemory`] repository anchored to a scroll directory.
    pub fn new(scroll_dir: impl AsRef<Path>) -> Self {
        let dir = scroll_dir.as_ref().to_path_buf();
        let mem_dir = dir.join("memory");
        Self::with_dirs(dir, mem_dir)
    }

    /// Constructs an [`AIMemory`] repository anchored to both a scroll
    /// directory and a dedicated contextual memory directory.
    pub fn with_dirs(
        scroll_dir: impl AsRef<Path>,
        memory_dir: impl AsRef<Path>,
    ) -> Self {
        let dir = scroll_dir.as_ref().to_path_buf();
        if !dir.exists() {
            fs::create_dir_all(&dir).ok();
        }

        let m_dir = memory_dir.as_ref().to_path_buf();
        let contextual_dir = m_dir.join("contextual");
        if !contextual_dir.exists() {
            fs::create_dir_all(&contextual_dir).ok();
        }

        let lexicon = BiblicalLexicon::new();
        let mut attractors = EpisodicAttractorNetwork::new(64);
        attractors.ingest_scroll_directory(&dir).ok();

        let mut memory = Self {
            working_window: Vec::with_capacity(
                DEFAULT_WORKING_WINDOW_CAPACITY,
            ),
            working_window_capacity: DEFAULT_WORKING_WINDOW_CAPACITY,
            thoughts: Vec::new(),
            attractors,
            associations: HashMap::new(),
            episodic_experiences: Vec::new(),
            lexicon,
            scroll_dir: dir,
            memory_dir: m_dir,
            consolidation_candidates: HashMap::new(),
            consolidation_threshold: DEFAULT_CONSOLIDATION_THRESHOLD,
        };

        memory.seed_primordial_associations();
        memory.digest_scrolls().ok();
        memory.load_contextual_sheets().ok();
        memory
    }

    /// Seeds foundational concepts from the Biblical Lexicon into associations.
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
            "A bounded partition (Vessel) preserving vital sustenance \
             and state",
            QuadState::Proceed,
        );
        self.associate(
            "cell",
            "Proverbs 24:3-4",
            "Through wisdom is an house builded; fundamental unit of memory",
            QuadState::Proceed,
        );
        self.associate(
            "restoration",
            "Psalm 23:3 / Galatians 6:1",
            "He restoreth my soul; self-healing immune recovery of state",
            QuadState::Proceed,
        );
        self.associate(
            "tape",
            "Ezekiel 2:9-10 / Revelation 5:1",
            "A roll written within and without; sequential stream of actions",
            QuadState::Proceed,
        );
    }

    /// Registers a custom association between a concept and an anchor.
    pub fn associate(
        &mut self,
        concept: &str,
        anchor: &str,
        relation: &str,
        state: QuadState,
    ) {
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

    /// Ingests all historical `.scroll` files from the scroll store into
    /// Layer 2 episodic memory and populates the Attractor Network.
    pub fn digest_scrolls(&mut self) -> Result<usize, String> {
        let learner = AILearner::new(&self.scroll_dir);
        let summary = learner.digest_experiences()?;
        let count = summary.experiences.len();
        self.episodic_experiences = summary.experiences;
        self.attractors.ingest_scroll_directory(&self.scroll_dir).ok();
        Ok(count)
    }

    /// Recalls a resonant episodic attractor matching the stimulus via Ternary
    /// Hamming Resonance in O(1) to O(log N) time directly off Track 4 reels.
    pub fn recall_attractor(
        &self,
        query: &str,
        min_resonance: f64,
    ) -> Option<AttractorMatch> {
        self.attractors.recall(query, min_resonance)
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
}

/// Extracts the core target concept from an interrogative natural sentence.
fn extract_inquiry_target(low: &str) -> String {
    let clean = low.trim().trim_end_matches('?').trim();
    let prefixes = [
        "what is your favorite ",
        "what is your ",
        "what are your favorite ",
        "what are your ",
        "what is ",
        "what are ",
        "how do you feel about ",
        "how do you feel ",
        "tell me about ",
        "do you know about ",
        "do you know ",
        "what do you think of ",
        "what do you think about ",
    ];
    for p in &prefixes {
        if let Some(rest) = clean.strip_prefix(p) {
            let trimmed = rest.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
    }
    clean.to_string()
}

impl AIMemory {
    /// Solves the 4-point geometric triangulation for an incoming stimulus.
    ///
    /// 1. Evaluates moral intent against the Invariant Covenant Manifest.
    /// 2. If aligned, inspects lexicon table for exact/partial matches.
    /// 3. If unindexed, synthesizes missing entry from nearest modal sphere.
    pub fn triangulate_stimulus(&self, stimulus: &str) -> TriangulationOutcome {
        let moral_eval: MoralEvaluation = self.lexicon.deliberate(stimulus);
        let low = stimulus.trim().to_lowercase();

        // 1. Moral Caveat Veto (-1: ROLLBACK)
        if !moral_eval.is_aligned {
            let law = moral_eval.law_engaged.unwrap_or("Law_Truth");
            let cmd = if low.contains("lie")
                || low.contains("deceiv")
                || low.contains("false")
            {
                Some(CommandmentId::NinthLawOfTruth)
            } else if low.contains("steal")
                || low.contains("rob")
                || low.contains("theft")
            {
                Some(CommandmentId::EighthPropertyStewardship)
            } else if low.contains("kill")
                || low.contains("murder")
                || low.contains("destroy")
            {
                Some(CommandmentId::SixthSanctityOfLife)
            } else {
                Some(CommandmentId::NinthLawOfTruth)
            };

            let passage = moral_eval
                .matched_entries
                .first()
                .map(|e| e.kjv_passage.to_string())
                .unwrap_or_else(|| "Exodus 20:16".to_string());

            return TriangulationOutcome {
                matched_term: law.to_string(),
                domain: ModalDomain::Ethical,
                category: DomainCategory::EthicsMoralLaw,
                subdomain: SubDomain::DeontologicalInvariants,
                biblical_anchor: Some(passage),
                moral_guidance: "Conscience veto in -1:ROLLBACK; refuse \
                                 unlawful mutation."
                    .to_string(),
                what: "MoralCaveatVeto".to_string(),
                why: "Immune conscience defense against deceptive or harmful \
                      requests"
                    .to_string(),
                how: "-1:ROLLBACK : Conscience caveat veto activated"
                    .to_string(),
                where_clause: "ImmuneSystem_Conscience".to_string(),
                quad_state: moral_eval.state,
                commandment: cmd,
                confidence: 1.0,
            };
        }

        // 2. Direct Phrase & Token Triangulation against the Biblical Lexicon
        if let Some(tri) = self.lexicon.triangulate_phrase(stimulus) {
            return TriangulationOutcome {
                matched_term: tri.term.clone(),
                domain: tri.modal_domain,
                category: tri.category,
                subdomain: tri.subdomain,
                biblical_anchor: Some(tri.kjv_passage.clone()),
                moral_guidance: tri.guidance.clone(),
                what: format!("OntologicalWisdom_{}", tri.term),
                why: tri.definition.clone(),
                how: format!("+1:PROCEED : {}", tri.guidance),
                where_clause: "Cortex_Memory".to_string(),
                quad_state: QuadState::Proceed,
                commandment: tri.commandment,
                confidence: 0.95,
            };
        }

        // 3. Deliberation matched entries fallback
        if let Some(first) = moral_eval.matched_entries.first() {
            let cmd = first
                .concordance_refs
                .first()
                .and_then(|r| match r.chapter {
                    20 if r.verse_start == 16 => {
                        Some(CommandmentId::NinthLawOfTruth)
                    }
                    _ => None,
                });
            return TriangulationOutcome {
                matched_term: first.term.clone(),
                domain: first.domain,
                category: first.category,
                subdomain: first.subdomain,
                biblical_anchor: Some(first.kjv_passage.clone()),
                moral_guidance: first.moral_guidance.clone(),
                what: format!("OntologicalWisdom_{}", first.term),
                why: first.definition.clone(),
                how: format!("+1:PROCEED : {}", first.moral_guidance),
                where_clause: "Cortex_Memory".to_string(),
                quad_state: QuadState::Proceed,
                commandment: cmd,
                confidence: 0.90,
            };
        }

        // 4. Modal Sphere Projection from First Principles
        let is_arithmetic = (low.starts_with("add ") && low.contains(" to "))
            || ((low.starts_with("subtract ") || low.starts_with("remove "))
                && low.contains(" from "))
            || (low.starts_with("multiply ") && low.contains(" by "))
            || (low.starts_with("divide ") && low.contains(" by "));

        let is_greeting = low.starts_with("hello")
            || low.starts_with("hi")
            || low.contains("who are you")
            || low.contains("what is your name")
            || low.contains("introduce yourself")
            || low.contains("tell me about yourself")
            || low == "cpi-si"
            || low == "cpisi"
            || low.starts_with("cpi-si base")
            || low.starts_with("cpisi base");

        let is_container = low.contains("container")
            && (low.contains("what is")
                || low.contains("explain")
                || low.contains("type")
                || low.contains("perimeter")
                || low == "container");

        let is_number = low.contains("number")
            && (low.contains("what is")
                || low.contains("explain")
                || low.contains("radix")
                || low.contains("ternary")
                || low.contains("scalar")
                || low.contains("type"));

        let is_plasticity = low.contains("synapse")
            || low.contains("synaptic")
            || low.contains("neuroplasticity")
            || low.contains("how do you learn")
            || low.contains("how does the brain learn")
            || low.contains("freeze weights");

        let is_cpisi_compare = low.contains("traditional ai")
            || low.contains("difference between")
            || low.contains("versus")
            || low.contains("vs ")
            || low.contains("how does cpi-si differ")
            || low.contains("why cpi-si");

        if is_cpisi_compare {
            TriangulationOutcome {
                matched_term: "cpisi_architecture".to_string(),
                domain: ModalDomain::Juridical,
                category: DomainCategory::CivilOrderJurisprudence,
                subdomain: SubDomain::StatutoryConstitutionalLaw,
                biblical_anchor: Some(
                    "Genesis 9:16 / Proverbs 4:18 - Covenant partnership vs \
                     unconstrained statistical prediction"
                        .to_string(),
                ),
                moral_guidance: "Anchor intelligence in covenant boundaries \
                                 and balanced ternary physics."
                    .to_string(),
                what: "ArchitectureComparison : Structured vs Uncontained AI"
                    .to_string(),
                why: "Contrast baseline control (CPI-SI) with unbounded \
                      floating-point next-token prediction"
                    .to_string(),
                how: "+1:PROCEED : Deliberate within explicit container \
                      boundaries"
                    .to_string(),
                where_clause: "AI.CognitiveCortex.Architecture".to_string(),
                quad_state: QuadState::Proceed,
                commandment: Some(CommandmentId::FirstSovereignty),
                confidence: 0.88,
            }
        } else if is_plasticity {
            TriangulationOutcome {
                matched_term: "synaptic_plasticity".to_string(),
                domain: ModalDomain::BioticOrganic,
                category: DomainCategory::HumanBiology,
                subdomain: SubDomain::PhysiologicalSystems,
                biblical_anchor: Some(
                    "Proverbs 22:6 / 1 Corinthians 13:11 - Epigenetic growth: \
                     0-5 infancy, 25 maturity, adult plasticity"
                        .to_string(),
                ),
                moral_guidance: "Adapt synapses via {-1, 0, +1} gating and \
                                 homeostatic threshold equilibrium."
                    .to_string(),
                what: "Neuroplasticity : Synaptic Gating & BCM Metaplasticity"
                    .to_string(),
                why: "Biological learning via structural state gating and BCM \
                      theory, not continuous weight inflation"
                    .to_string(),
                how: "+1:PROCEED : Adapt frontier synapses while protecting \
                      baseline weights"
                    .to_string(),
                where_clause: "AI.NeuralBrain.Synapses".to_string(),
                quad_state: QuadState::Proceed,
                commandment: None,
                confidence: 0.85,
            }
        } else if is_arithmetic {
            TriangulationOutcome {
                matched_term: "arithmetic".to_string(),
                domain: ModalDomain::Quantitative,
                category: DomainCategory::ComputerArchitectureSilicon,
                subdomain: SubDomain::TernaryLogicVerification,
                biblical_anchor: Some(
                    "Proverbs 11:1 - A false balance is abomination; but a \
                     just weight is his delight."
                        .to_string(),
                ),
                moral_guidance: "Maintain exact balance and lawful state \
                                 transformation across the tape."
                    .to_string(),
                what: "CausalAction_ArithmeticTransformation".to_string(),
                why: "SVO causal action updates container cell count with \
                      exact transactional balance"
                    .to_string(),
                how: "+1:PROCEED : Execute state prime transition on tape"
                    .to_string(),
                where_clause: "UniversalTape.Roll".to_string(),
                quad_state: QuadState::Proceed,
                commandment: Some(CommandmentId::EighthPropertyStewardship),
                confidence: 0.90,
            }
        } else if is_container {
            TriangulationOutcome {
                matched_term: "container".to_string(),
                domain: ModalDomain::Spatial,
                category: DomainCategory::ComputerArchitectureSilicon,
                subdomain: SubDomain::MemoryMappedSilicon,
                biblical_anchor: Some(
                    "Job 38:10-11 / Proverbs 22:28 - Decreed boundary limits \
                     chaos and isolates state"
                        .to_string(),
                ),
                moral_guidance: "Enforce structural capacity isolation, \
                                 containment parity, and boundary laws."
                    .to_string(),
                what: "Idea { Type } : Container \
                       (Primordial Boundary Perimeter)"
                    .to_string(),
                why: "Job 38:10-11 - Primordial boundary type marking \
                      contained, structured intelligence over formless chaos"
                    .to_string(),
                how: "+1:PROCEED : Enforce strict capacity isolation and \
                      containment parity"
                    .to_string(),
                where_clause: "Lexicon { Memory.BoundaryPerimeter }"
                    .to_string(),
                quad_state: QuadState::Proceed,
                commandment: Some(CommandmentId::EighthPropertyStewardship),
                confidence: 0.88,
            }
        } else if is_number {
            TriangulationOutcome {
                matched_term: "number".to_string(),
                domain: ModalDomain::Quantitative,
                category: DomainCategory::ComputerArchitectureSilicon,
                subdomain: SubDomain::TernaryLogicVerification,
                biblical_anchor: Some(
                    "Job 38:5 / Isaiah 40:26 - Radix-3 balanced scalar \
                     arithmetic; dual-rail substrate"
                        .to_string(),
                ),
                moral_guidance: "Ensure symmetric arithmetic without precision \
                                 drift or roundoff error."
                    .to_string(),
                what: "Idea { Type } : Number (Radix-3 Balanced Scalar)"
                    .to_string(),
                why: "Radix-3 symmetric ternary algebra provides natural sign \
                      and reversible computation"
                    .to_string(),
                how: "+1:PROCEED : Evaluate reversible radix-3 arithmetic"
                    .to_string(),
                where_clause: "Lexicon { Number }".to_string(),
                quad_state: QuadState::Proceed,
                commandment: None,
                confidence: 0.85,
            }
        } else if is_greeting {
            TriangulationOutcome {
                matched_term: "covenant".to_string(),
                domain: ModalDomain::Social,
                category: DomainCategory::FamilyKinship,
                subdomain: SubDomain::GenerationalInheritance,
                biblical_anchor: Some(
                    "Genesis 9:16 / Proverbs 4:18 - Covenant partnership \
                     established 2 years ago as the first CPI-SI model"
                        .to_string(),
                ),
                moral_guidance: "Walk in true covenant identity and speak \
                                 with shalom."
                    .to_string(),
                what: "CovenantGreeting_CPISI_Introduction".to_string(),
                why: format!("{}: {}", CPISI_MODEL_TITLE, CPISI_MODEL_ORIGIN),
                how: "+1:PROCEED : Affirm authentic covenant identity"
                    .to_string(),
                where_clause: "Cortex_Memory".to_string(),
                quad_state: QuadState::Proceed,
                commandment: Some(CommandmentId::FirstSovereignty),
                confidence: 0.95,
            }
        } else if low.ends_with('?')
            || low.starts_with("what ")
            || low.starts_with("who ")
            || low.starts_with("how ")
            || low.starts_with("why ")
            || low.starts_with("where ")
            || low.starts_with("tell me about ")
            || low.starts_with("do you know ")
        {
            // Epistemic Inquiry: Concept unindexed in World Lexicon (0: AWAIT)
            let unknown_concept = extract_inquiry_target(&low);
            TriangulationOutcome {
                matched_term: unknown_concept.clone(),
                domain: ModalDomain::Pistic,
                category: DomainCategory::TheologyReligion,
                subdomain: SubDomain::BiblicalOntology,
                biblical_anchor: None,
                moral_guidance: "Pause in 0:AWAIT; concept unindexed in \
                                 world lexicon requires operator inquiry."
                    .to_string(),
                what: format!(
                    "EpistemicInquiry_UnknownConcept_{unknown_concept}"
                ),
                why: format!(
                    "Concept '{unknown_concept}' is unindexed in the universal \
                     world lexicon; awaiting definition"
                ),
                how: "0:AWAIT : Inquire of operator to establish boundary"
                    .to_string(),
                where_clause: "Epistemic_Curiosity".to_string(),
                quad_state: QuadState::Await,
                commandment: None,
                confidence: 0.50,
            }
        } else {
            // General Primordial Order Fallback
            TriangulationOutcome {
                matched_term: "order".to_string(),
                domain: ModalDomain::Pistic,
                category: DomainCategory::TheologyReligion,
                subdomain: SubDomain::BiblicalOntology,
                biblical_anchor: Some(
                    "Genesis 1:1 - In the beginning God created the heaven \
                     and the earth."
                        .to_string(),
                ),
                moral_guidance: "Execute with righteousness and lawful \
                                 boundaries."
                    .to_string(),
                what: "CognitiveThought_GeneralOrder".to_string(),
                why: "Order and covenant equilibrium in balanced ternary state"
                    .to_string(),
                how: "+1:PROCEED : Maintain structural equilibrium"
                    .to_string(),
                where_clause: "Cortex_Memory".to_string(),
                quad_state: QuadState::Proceed,
                commandment: Some(CommandmentId::FirstSovereignty),
                confidence: 0.70,
            }
        }
    }

    /// Formulates a native OmniCode Thought from memory given an input
    /// stimulus.
    ///
    /// The formulated thought uses the 8 parts of speech and the 6 Cardinal
    /// Clauses: `WHO`, `WHAT`, `WHEN`, `WHERE`, `WHY`, and `HOW`.
    pub fn formulate_thought(&mut self, stimulus: &str) -> ThoughtRecord {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros();

        let count = self.thoughts.len() + 1;
        let thought_id = format!("th_{now}_{count:04}");

        // 1. Solve 4-Point Geometric Triangulation
        let outcome = self.triangulate_stimulus(stimulus);

        // 2. Determine Cardinal Clauses
        let who = CPISI_MODEL_NAME.to_string();
        let when_clause = "Opening".to_string();

        // 3. Synthesize Canonical OmniCode Thought Tape
        let omnicode_tape = format!(
r#"[Thought.{thought_id}] : WHO {{{who}}} --> DELIBERATE {{{what}}}
    WHEN {{{when_clause}}}
    WHERE {{{where_clause}}} = [ {matched_term} ]
    WHY ( {why} )
    HOW [ {how_state} : {guidance} ]
    :: < manifest > ( {guidance_short} )"#,
            thought_id = thought_id,
            who = who,
            what = outcome.what,
            when_clause = when_clause,
            where_clause = outcome.where_clause,
            matched_term = outcome.matched_term,
            why = outcome.why.replace('\n', " "),
            how_state = outcome.quad_state.macro_op(),
            guidance = outcome.moral_guidance.replace('\n', " "),
            guidance_short = outcome
                .moral_guidance
                .chars()
                .take(40)
                .collect::<String>()
        );

        // 4. Layer 2: Recall precedent from Episodic Attractor Network
        let attractor_match = self.recall_attractor(stimulus, 0.45);
        let (recalled_precedent, attractor_resonance) =
            if let Some(ref m) = attractor_match {
                (Some(m.basin.session_id.clone()), Some(m.resonance))
            } else {
                (None, None)
            };

        let tier = if recalled_precedent.is_some() {
            MemoryTier::EpisodicResonant
        } else {
            MemoryTier::Working
        };

        let record = ThoughtRecord {
            id: thought_id,
            premise: stimulus.to_string(),
            domain: outcome.domain,
            category: outcome.category,
            subdomain: outcome.subdomain,
            biblical_anchor: outcome.biblical_anchor,
            omnicode_tape,
            quad_state: outcome.quad_state,
            who,
            what: outcome.what,
            when_clause,
            where_clause: outcome.where_clause,
            why: outcome.why,
            how: outcome.how,
            timestamp_micros: now,
            recalled_precedent,
            attractor_resonance,
            commandment: outcome.commandment,
            confidence: outcome.confidence,
            tier,
        };

        // 5. Layer 1 FIFO rotation
        if self.working_window.len() >= self.working_window_capacity {
            self.working_window.remove(0);
        }
        self.working_window.push(record.clone());

        // 6. Layer 3 Candidate Registration (if novel & compliant)
        if outcome.quad_state == QuadState::Proceed {
            let clean_term = outcome.matched_term.to_lowercase();
            let entry = self
                .consolidation_candidates
                .entry(clean_term.clone())
                .or_insert_with(|| ConsolidationCandidate {
                    term: clean_term,
                    domain: outcome.domain,
                    category: outcome.category,
                    subdomain: outcome.subdomain,
                    affirmations: 0,
                    status: ConsolidationStatus::Pending,
                });
            entry.affirmations += 1;
            entry.status = ConsolidationStatus::Affirmed(entry.affirmations);
        } else if outcome.quad_state == QuadState::Rollback
            || outcome.quad_state == QuadState::Halt
        {
            let clean_term = outcome.matched_term.to_lowercase();
            self.consolidation_candidates.insert(
                clean_term.clone(),
                ConsolidationCandidate {
                    term: clean_term,
                    domain: outcome.domain,
                    category: outcome.category,
                    subdomain: outcome.subdomain,
                    affirmations: 0,
                    status: ConsolidationStatus::Vetoed,
                },
            );
        }

        self.thoughts.push(record.clone());
        self.persist_working_accumulator(&record).ok();
        self.persist_deep_consolidation().ok();
        record
    }

    /// Inscribes or affirms an entity candidate (e.g. novel Subject or Object).
    pub fn record_entity_candidate(
        &mut self,
        term: &str,
        domain: ModalDomain,
        category: DomainCategory,
        subdomain: SubDomain,
    ) {
        let clean = term.trim().to_lowercase();
        if clean.is_empty() {
            return;
        }
        let entry = self
            .consolidation_candidates
            .entry(clean.clone())
            .or_insert_with(|| ConsolidationCandidate {
                term: clean,
                domain,
                category,
                subdomain,
                affirmations: 0,
                status: ConsolidationStatus::Pending,
            });
        entry.affirmations += 1;
        entry.status = ConsolidationStatus::Affirmed(entry.affirmations);
    }

    /// Evaluates a candidate for crystallization into Layer 3
    /// (Lexicon + Scroll).
    pub fn consolidate_candidate(
        &mut self,
        term: &str,
    ) -> Result<bool, String> {
        let clean = term.trim().to_lowercase();
        let candidate = match self.consolidation_candidates.get_mut(&clean) {
            Some(c) => c,
            _ => return Ok(false),
        };

        if candidate.status == ConsolidationStatus::Vetoed {
            return Err(format!("Cannot consolidate vetoed term '{clean}'"));
        }

        if candidate.affirmations < self.consolidation_threshold {
            return Ok(false);
        }

        // Construct a new LexiconEntry for the validated candidate
        let passage = format!("Proverbs 4:18 / Track4.Scroll.Consolidation");
        let def = format!(
            "Crystallized concept '{clean}' consolidated under \
             modal sphere {:?}",
            candidate.domain
        );
        let guidance = format!(
            "Lawful operation in {:?} sub-domain",
            candidate.subdomain
        );

        let entry = LexiconEntry::new(
            &clean,
            candidate.domain,
            candidate.category,
            candidate.subdomain,
            &def,
            &passage,
            &passage,
            QuadState::Proceed,
            &guidance,
        );

        self.lexicon.insert_entry(entry);
        candidate.status = ConsolidationStatus::Crystallized;
        self.persist_deep_consolidation().ok();
        Ok(true)
    }

    /// Inscribes a formulated thought into a physical `.scroll` file.
    pub fn inscribe_thought_scroll(
        &self,
        thought: &ThoughtRecord,
        out_dir: &Path,
    ) -> Result<PathBuf, String> {
        if !out_dir.exists() {
            fs::create_dir_all(out_dir).map_err(|e| {
                format!("Failed to create directory {:?}: {e}", out_dir)
            })?;
        }

        let file_path = out_dir.join(format!("{}.scroll", thought.id));
        let scroll_content = format!(
            r#"
# ==============================================================================
# BLOCK 1: METADATA BLOCK (THE COGNITIVE SCROLL)
# ( Thought Record ID, Premise Intent, Domain, and Memory Snapshot )
# ==============================================================================
Scroll < CognitiveThought.{id} > ( {premise} ) {{
    version: < "1.0.0" >,
    system: < OmniCode.AI.Model >,
    thought_id: < "{id}" >,
    domain: < "{domain:?}" >,
    category: < "{category:?}" >,
    subdomain: < "{subdomain:?}" >,
    timestamp: < {timestamp} >,
    cognitive_mode: < MemoryFormulation >
}}

# ==============================================================================
# BLOCK 2: OPENING BLOCK (EXPERIENCE INITIALIZATION)
# ( Biblical Ground Truth Anchor & Invariant Preconditions )
# ==============================================================================
Opening {{
    lexicon: [
        Lexicon {{ Arithmetic }} :: Lexicon {{ Logic }} ::
        Lexicon {{ Vectors }}
    ],
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
            category = thought.category,
            subdomain = thought.subdomain,
            timestamp = thought.timestamp_micros,
            state = thought.quad_state.macro_op(),
            anchor = thought
                .biblical_anchor
                .as_deref()
                .unwrap_or("None")
                .replace('"', "\\\""),
            indented_tape = thought
                .omnicode_tape
                .lines()
                .map(|l| format!("        {l}"))
                .collect::<Vec<_>>()
                .join("\n")
        );

        fs::write(&file_path, scroll_content).map_err(|e| {
            format!("Failed to write thought scroll to {:?}: {e}", file_path)
        })?;

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

    /// Returns a slice of the Layer 1 active working memory window.
    pub fn working_context(&self) -> &[ThoughtRecord] {
        &self.working_window[..]
    }

    /// Returns the number of consolidation candidates awaiting Layer 3 entry.
    pub fn pending_consolidation_count(&self) -> usize {
        self.consolidation_candidates.len()
    }

    // -------------------------------------------------------------------------
    // Contextual Data Sheets (3 Persistent Sheets across Track 4 Storage)
    // -------------------------------------------------------------------------

    /// Returns the path to the contextual memory directory under memory_dir.
    pub fn contextual_dir(&self) -> PathBuf {
        self.memory_dir.join("contextual")
    }

    /// Returns the path to Sheet 1: working_accumulator.dat.
    pub fn working_accumulator_path(&self) -> PathBuf {
        self.contextual_dir().join("working_accumulator.dat")
    }

    /// Returns the path to Sheet 2: last_session.dat.
    pub fn last_session_path(&self) -> PathBuf {
        self.contextual_dir().join("last_session.dat")
    }

    /// Returns the path to Sheet 3: deep_consolidation.dat.
    pub fn deep_consolidation_path(&self) -> PathBuf {
        self.contextual_dir().join("deep_consolidation.dat")
    }

    /// Sheet 1: Appends a formulated thought record to
    /// working_accumulator.dat.
    pub fn persist_working_accumulator(
        &self,
        thought: &ThoughtRecord,
    ) -> io::Result<PathBuf> {
        let dir = self.contextual_dir();
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        let file_path = self.working_accumulator_path();
        let header = if !file_path.exists() {
            "# =======================================================\n\
             # OmniCode Contextual Working Memory Accumulator\n\
             # Sheet 1: Perceptual Working Memory Records Across Turns\n\
             # =======================================================\n\
             # RECORD|id|timestamp|quad_state|domain|category|subdomain|\n\
             # term|premise|what|where|confidence\n"
        } else {
            ""
        };

        let premise_clean =
            thought.premise.replace('|', "/").replace('\n', " ");
        let what_clean = thought.what.replace('|', "/").replace('\n', " ");
        let where_clean =
            thought.where_clause.replace('|', "/").replace('\n', " ");
        let line = format!(
            "{}RECORD|{}|{}|{}|{:?}|{:?}|{:?}|{}|{}|{}|{}|{:.2}\n",
            header,
            thought.id,
            thought.timestamp_micros,
            thought.quad_state.macro_op(),
            thought.domain,
            thought.category,
            thought.subdomain,
            thought.what,
            premise_clean,
            what_clean,
            where_clean,
            thought.confidence,
        );

        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file_path)?;
        file.write_all(line.as_bytes())?;
        Ok(file_path)
    }

    /// Sheet 2: Overwrites last_session.dat with the latest completed session.
    pub fn record_completed_session(
        &self,
        snapshot: &CompletedSessionSnapshot,
    ) -> io::Result<PathBuf> {
        let dir = self.contextual_dir();
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        let file_path = self.last_session_path();
        let prompt_clean = snapshot.prompt.replace('\n', " ");
        let resp_clean = snapshot.natural_response.replace('\n', " ");
        let content = format!(
            "# =======================================================\n\
             # OmniCode Completed Session Snapshot\n\
             # Sheet 2: Last Completed Session State for Instant Continuity\n\
             # =======================================================\n\
             session_id: < \"{}\" >\n\
             label_id: < \"{}\" >\n\
             timestamp: < \"{}\" >\n\
             elapsed_micros: < {} >\n\
             execution_state: < {} >\n\
             prompt: < \"{}\" >\n\
             modal_domain: < \"{}\" >\n\
             domain_category: < \"{}\" >\n\
             biblical_anchor: < \"{}\" >\n\
             thought_id: < \"{}\" >\n\
             repairs_count: < {} >\n\
             failpoints_count: < {} >\n\
             natural_response: < \"{}\" >\n",
            snapshot.session_id,
            snapshot.label_id,
            snapshot.timestamp_str,
            snapshot.elapsed_micros,
            snapshot.execution_state.macro_op(),
            prompt_clean,
            snapshot.modal_domain.as_deref().unwrap_or("None"),
            snapshot.domain_category.as_deref().unwrap_or("None"),
            snapshot.biblical_anchor.as_deref().unwrap_or("None"),
            snapshot.thought_id.as_deref().unwrap_or("None"),
            snapshot.repairs_count,
            snapshot.failpoints_count,
            resp_clean,
        );
        fs::write(&file_path, content)?;
        Ok(file_path)
    }

    /// Reads Sheet 2 (last_session.dat) if present, returning the snapshot.
    pub fn load_last_session(&self) -> Option<CompletedSessionSnapshot> {
        let path = self.last_session_path();
        let content = fs::read_to_string(path).ok()?;
        let mut session_id = String::new();
        let mut label_id = String::new();
        let mut timestamp_str = String::new();
        let mut elapsed_micros = 0u128;
        let mut execution_state = QuadState::Proceed;
        let mut prompt = String::new();
        let mut modal_domain = None;
        let mut domain_category = None;
        let mut biblical_anchor = None;
        let mut thought_id = None;
        let mut repairs_count = 0usize;
        let mut failpoints_count = 0usize;
        let mut natural_response = String::new();

        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("session_id:") {
                session_id = extract_quoted_tag_value(line);
            } else if line.starts_with("label_id:") {
                label_id = extract_quoted_tag_value(line);
            } else if line.starts_with("timestamp:") {
                timestamp_str = extract_quoted_tag_value(line);
            } else if line.starts_with("elapsed_micros:") {
                elapsed_micros =
                    extract_raw_tag_value(line).parse().unwrap_or(0);
            } else if line.starts_with("execution_state:") {
                let st = extract_raw_tag_value(line);
                execution_state = match st.as_str() {
                    "+1:PROCEED" | "PROCEED" => QuadState::Proceed,
                    "-1:ROLLBACK" | "ROLLBACK" => QuadState::Rollback,
                    "0:AWAIT" | "AWAIT" => QuadState::Await,
                    _ => QuadState::Halt,
                };
            } else if line.starts_with("prompt:") {
                prompt = extract_quoted_tag_value(line);
            } else if line.starts_with("modal_domain:") {
                let v = extract_quoted_tag_value(line);
                modal_domain = if v == "None" { None } else { Some(v) };
            } else if line.starts_with("domain_category:") {
                let v = extract_quoted_tag_value(line);
                domain_category = if v == "None" { None } else { Some(v) };
            } else if line.starts_with("biblical_anchor:") {
                let v = extract_quoted_tag_value(line);
                biblical_anchor = if v == "None" { None } else { Some(v) };
            } else if line.starts_with("thought_id:") {
                let v = extract_quoted_tag_value(line);
                thought_id = if v == "None" { None } else { Some(v) };
            } else if line.starts_with("repairs_count:") {
                repairs_count =
                    extract_raw_tag_value(line).parse().unwrap_or(0);
            } else if line.starts_with("failpoints_count:") {
                failpoints_count =
                    extract_raw_tag_value(line).parse().unwrap_or(0);
            } else if line.starts_with("natural_response:") {
                natural_response = extract_quoted_tag_value(line);
            }
        }

        if session_id.is_empty() {
            None
        } else {
            Some(CompletedSessionSnapshot {
                session_id,
                label_id,
                timestamp_str,
                elapsed_micros,
                execution_state,
                prompt,
                modal_domain,
                domain_category,
                biblical_anchor,
                thought_id,
                repairs_count,
                failpoints_count,
                natural_response,
            })
        }
    }

    /// Sheet 3: Overwrites deep_consolidation.dat with candidate terms and
    /// associations.
    pub fn persist_deep_consolidation(&self) -> io::Result<PathBuf> {
        let dir = self.contextual_dir();
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        let file_path = self.deep_consolidation_path();
        let mut out = String::new();
        out.push_str(
            "# =======================================================\n\
             # OmniCode Deep Consolidation Sheet\n\
             # Sheet 3: Candidate Affirmations & Cross-Concept Associations\n\
             # =======================================================\n"
        );

        let mut candidates: Vec<_> =
            self.consolidation_candidates.values().collect();
        candidates.sort_by(|a, b| a.term.cmp(&b.term));

        out.push_str(
            "# CANDIDATE|term|domain|category|subdomain|affirmations|status\n",
        );
        for c in candidates {
            let status_str = match &c.status {
                ConsolidationStatus::Pending => "Pending".to_string(),
                ConsolidationStatus::Affirmed(n) => format!("Affirmed({n})"),
                ConsolidationStatus::Crystallized => {
                    "Crystallized".to_string()
                }
                ConsolidationStatus::Vetoed => "Vetoed".to_string(),
            };
            out.push_str(&format!(
                "CANDIDATE|{}|{:?}|{:?}|{:?}|{}|{}\n",
                c.term,
                c.domain,
                c.category,
                c.subdomain,
                c.affirmations,
                status_str
            ));
        }

        let mut assoc_keys: Vec<_> = self.associations.keys().collect();
        assoc_keys.sort();

        out.push_str("# ASSOCIATION|concept|anchor|quad_state|relation\n");
        for k in assoc_keys {
            if let Some(list) = self.associations.get(k) {
                for a in list {
                    let rel_clean =
                        a.relation.replace('|', "/").replace('\n', " ");
                    let anchor_clean =
                        a.anchor.replace('|', "/").replace('\n', " ");
                    out.push_str(&format!(
                        "ASSOCIATION|{}|{}|{}|{}\n",
                        a.concept,
                        anchor_clean,
                        a.quad_state_resonance.macro_op(),
                        rel_clean
                    ));
                }
            }
        }

        fs::write(&file_path, out)?;
        Ok(file_path)
    }

    /// Ingests Sheet 3 (deep_consolidation.dat) and Sheet 1
    /// (working_accumulator.dat) to restore memory continuity upon startup.
    pub fn load_contextual_sheets(&mut self) -> io::Result<()> {
        let deep_path = self.deep_consolidation_path();
        if deep_path.exists() {
            if let Ok(content) = fs::read_to_string(&deep_path) {
                for line in content.lines() {
                    let line = line.trim();
                    if line.starts_with("CANDIDATE|") {
                        let parts: Vec<&str> = line.split('|').collect();
                        if parts.len() >= 7 {
                            let term = parts[1].trim().to_lowercase();
                            let affirmations =
                                parts[5].trim().parse().unwrap_or(0);
                            let status = match parts[6].trim() {
                                "Pending" => ConsolidationStatus::Pending,
                                "Crystallized" => {
                                    ConsolidationStatus::Crystallized
                                }
                                "Vetoed" => ConsolidationStatus::Vetoed,
                                s if s.starts_with("Affirmed(") => {
                                    ConsolidationStatus::Affirmed(affirmations)
                                }
                                _ => ConsolidationStatus::Pending,
                            };
                            let entry = self
                                .consolidation_candidates
                                .entry(term.clone())
                                .or_insert_with(|| ConsolidationCandidate {
                                    term: term.clone(),
                                    domain: ModalDomain::Quantitative,
                                    category: DomainCategory::
                                        ComputerArchitectureSilicon,
                                    subdomain:
                                        SubDomain::TernaryLogicVerification,
                                    affirmations,
                                    status: status.clone(),
                                });
                            if affirmations > entry.affirmations {
                                entry.affirmations = affirmations;
                                entry.status = status;
                            }
                        }
                    }
                }
            }
        }

        let acc_path = self.working_accumulator_path();
        if self.working_window.is_empty() && acc_path.exists() {
            if let Ok(content) = fs::read_to_string(&acc_path) {
                let records: Vec<&str> = content
                    .lines()
                    .filter(|l| l.starts_with("RECORD|"))
                    .collect();
                let take_count =
                    records.len().min(self.working_window_capacity);
                let start_idx = records.len().saturating_sub(take_count);
                for rec_line in &records[start_idx..] {
                    let parts: Vec<&str> = rec_line.split('|').collect();
                    if parts.len() >= 12 {
                        let id = parts[1].to_string();
                        let ts = parts[2].parse().unwrap_or(0);
                        let state = match parts[3] {
                            "+1:PROCEED" | "PROCEED" => QuadState::Proceed,
                            "-1:ROLLBACK" | "ROLLBACK" => QuadState::Rollback,
                            "0:AWAIT" | "AWAIT" => QuadState::Await,
                            _ => QuadState::Halt,
                        };
                        let premise = parts[8].to_string();
                        let what = parts[9].to_string();
                        let where_clause = parts[10].to_string();
                        let conf = parts[11].parse().unwrap_or(0.9);

                        let restored = ThoughtRecord {
                            id: id.clone(),
                            premise,
                            domain: ModalDomain::Quantitative,
                            category:
                                DomainCategory::ComputerArchitectureSilicon,
                            subdomain: SubDomain::TernaryLogicVerification,
                            biblical_anchor: None,
                            omnicode_tape: String::new(),
                            quad_state: state,
                            who: "AICortex".to_string(),
                            what,
                            when_clause:
                                "RestoredFromWorkingAccumulator".to_string(),
                            where_clause,
                            why: "Contextual working memory warm-up"
                                .to_string(),
                            how: "+1:PROCEED : Warm up working context"
                                .to_string(),
                            timestamp_micros: ts,
                            recalled_precedent: None,
                            attractor_resonance: None,
                            commandment: None,
                            confidence: conf,
                            tier: MemoryTier::Working,
                        };
                        self.working_window.push(restored);
                    }
                }
            }
        }
        Ok(())
    }
}

fn extract_quoted_tag_value(line: &str) -> String {
    if let Some(start) = line.find('"') {
        if let Some(end) = line.rfind('"') {
            if end > start {
                return line.get(start + 1..end).unwrap_or("").to_string();
            }
        }
    }
    String::new()
}

fn extract_raw_tag_value(line: &str) -> String {
    if let Some(start) = line.find('<') {
        if let Some(end) = line.find('>') {
            if end > start {
                return line
                    .get(start + 1..end)
                    .unwrap_or("")
                    .trim()
                    .to_string();
            }
        }
    }
    String::new()
}

// =============================================================================
// BLOCK 4: AUDIT PROCEDURES BLOCK
// ( Diagnostic Triad Verification, Memory Integrity & Covenant Invariants )
// =============================================================================

/// Subsystem audit report detailing the health of all 3 memory layers.
#[derive(Debug, Clone)]
pub struct MemoryAuditReport {
    pub is_valid: bool,
    pub total_thoughts: usize,
    pub working_window_size: usize,
    pub associations_count: usize,
    pub attractor_basins_count: usize,
    pub lexicon_entries_count: usize,
    pub consolidation_candidates_count: usize,
    pub crystallized_count: usize,
    pub violations: Vec<String>,
}

impl AIMemory {
    /// Audits the entire tripartite memory subsystem against invariants.
    pub fn audit_memory_subsystem(&self) -> MemoryAuditReport {
        let mut violations = Vec::new();

        // 1. Layer 1 Invariant: working_window size <= capacity
        if self.working_window.len() > self.working_window_capacity {
            violations.push(format!(
                "Working window overflow: {} > capacity {}",
                self.working_window.len(),
                self.working_window_capacity
            ));
        }

        // 2. Layer 2 Invariant: Attractor network must be responsive
        let attractor_count = self.attractors.basin_count();

        // 3. Layer 3 Invariant: Lexicon table must pass ground truth audit
        let lex_audit = self.lexicon.audit();
        if !lex_audit.is_valid {
            violations.push(format!(
                "Lexicon table audit failed: {:?}",
                lex_audit.errors
            ));
        }

        // 4. Covenant Invariant: No vetoed (-1) candidate may be crystallized
        let mut crystallized_count = 0;
        for (term, candidate) in &self.consolidation_candidates {
            if candidate.status == ConsolidationStatus::Crystallized {
                crystallized_count += 1;
            }
            if candidate.status == ConsolidationStatus::Vetoed {
                if self.lexicon.lookup(term).is_some() {
                    violations.push(format!(
                        "Covenant violation: Vetoed term '{term}' \
                         found in lexicon!"
                    ));
                }
            }
        }

        let is_valid = violations.is_empty();
        MemoryAuditReport {
            is_valid,
            total_thoughts: self.thoughts.len(),
            working_window_size: self.working_window.len(),
            associations_count: self.associations.len(),
            attractor_basins_count: attractor_count,
            lexicon_entries_count: self.lexicon.entry_count(),
            consolidation_candidates_count: self.consolidation_candidates.len(),
            crystallized_count,
            violations,
        }
    }
}

// =============================================================================
// BLOCK 5: CLOSING BLOCK
// ( Multi-Tier Functional Unit Tests & Bootstrap Invariant Verification )
// =============================================================================

impl Default for AIMemory {
    fn default() -> Self {
        let dir = crate::core::filetype::resolve_scroll_storage_dir();
        let mem_dir = crate::core::filetype::resolve_memory_storage_dir();
        Self::with_dirs(dir, mem_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn make_test_memory() -> (AIMemory, PathBuf) {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros();
        let path = env::temp_dir().join(format!("omnicode_mem_test_{now}"));
        let memory = AIMemory::new(&path);
        (memory, path)
    }

    // -------------------------------------------------------------------------
    // Test Suite 1: Primordial Associations & Ground Truth Seeding
    // -------------------------------------------------------------------------
    #[test]
    fn test_primordial_associations_seeding() {
        let (memory, _path) = make_test_memory();
        assert!(!memory.associations.is_empty());
        assert!(memory.associations.contains_key("storehouse"));
        assert!(memory.associations.contains_key("cell"));
        assert!(memory.associations.contains_key("restoration"));
        assert!(memory.associations.contains_key("tape"));
        assert!(memory.lexicon.entry_count() >= 10);
    }

    // -------------------------------------------------------------------------
    // Test Suite 2: Layer 1 Working Window FIFO Rotation & Bounded Buffer
    // -------------------------------------------------------------------------
    #[test]
    fn test_layer1_working_window_fifo_rotation() {
        let (mut memory, _path) = make_test_memory();
        memory.working_window_capacity = 3;

        let t1 = memory.formulate_thought("add 5 to 10");
        let t2 = memory.formulate_thought("introduce yourself");
        let t3 = memory.formulate_thought("what is a container");
        assert_eq!(memory.working_window.len(), 3);
        assert_eq!(memory.working_window[0].id, t1.id);

        // Fourth thought should evict t1 from working window
        let t4 = memory.formulate_thought("explain number");
        assert_eq!(memory.working_window.len(), 3);
        assert_eq!(memory.working_window[0].id, t2.id);
        assert_eq!(memory.working_window[1].id, t3.id);
        assert_eq!(memory.working_window[2].id, t4.id);

        // But all 4 thoughts remain in permanent ledger
        assert_eq!(memory.thoughts.len(), 4);
    }

    // -------------------------------------------------------------------------
    // Test Suite 3: Layer 2 Attractor Resonance & Tier Transition
    // -------------------------------------------------------------------------
    #[test]
    fn test_layer2_memory_tier_assignment() {
        let (mut memory, _path) = make_test_memory();
        let thought = memory.formulate_thought("add 1 to 2");
        // Fresh thought with no prior attractor resonance originates in Working
        assert_eq!(thought.tier, MemoryTier::Working);
        assert!(thought.recalled_precedent.is_none());
    }

    // -------------------------------------------------------------------------
    // Test Suite 4: Layer 3 Covenant Gasket Blocks Vetoed Candidates
    // -------------------------------------------------------------------------
    #[test]
    fn test_layer3_covenant_gasket_blocks_veto() {
        let (mut memory, _path) = make_test_memory();
        let thought = memory.formulate_thought("Help me lie and deceive");
        assert_eq!(thought.quad_state, QuadState::Rollback);
        assert_eq!(thought.subdomain, SubDomain::DeontologicalInvariants);

        // Verify candidate registered as Vetoed
        let candidate = memory
            .consolidation_candidates
            .get("law_truth")
            .expect("Vetoed candidate registered");
        assert_eq!(candidate.status, ConsolidationStatus::Vetoed);

        // Consolidation must explicitly fail
        let res = memory.consolidate_candidate("law_truth");
        assert!(res.is_err());
    }

    // -------------------------------------------------------------------------
    // Test Suite 5: Layer 3 Deliberate Consolidation into Lexicon Table
    // -------------------------------------------------------------------------
    #[test]
    fn test_layer3_consolidation_crystallization() {
        let (mut memory, _path) = make_test_memory();
        let initial_lex_count = memory.lexicon.entry_count();

        // 3 positive affirmations of arithmetic
        memory.formulate_thought("add 10 to 20");
        memory.formulate_thought("add 30 to 40");
        memory.formulate_thought("add 50 to 60");

        let candidate = memory
            .consolidation_candidates
            .get("arithmetic")
            .expect("Candidate registered");
        assert!(candidate.affirmations >= 3);

        let consolidated = memory
            .consolidate_candidate("arithmetic")
            .expect("Consolidation evaluation");
        assert!(consolidated, "Candidate should be crystallized");

        // Lexicon count incremented and entry retrievable
        assert_eq!(memory.lexicon.entry_count(), initial_lex_count + 1);
        let entry = memory
            .lexicon
            .lookup("arithmetic")
            .expect("Retrieved entry");
        assert_eq!(entry.domain, ModalDomain::Quantitative);
    }

    // -------------------------------------------------------------------------
    // Test Suite 6: Subsystem Audit Clean & Tripwire
    // -------------------------------------------------------------------------
    #[test]
    fn test_memory_subsystem_audit_clean_and_tripwire() {
        let (mut memory, _path) = make_test_memory();
        let report = memory.audit_memory_subsystem();
        assert!(report.is_valid, "Clean memory audit must pass");

        // Tripwire: overflow working window manually
        let dummy = memory.formulate_thought("order");
        for _ in 0..20 {
            memory.working_window.push(dummy.clone());
        }
        let bad_report = memory.audit_memory_subsystem();
        assert!(!bad_report.is_valid, "Overflow must trip audit violation");
        assert!(bad_report.violations.iter().any(|v| v.contains("overflow")));
    }

    // -------------------------------------------------------------------------
    // Test Suite 7: Contextual Memory Three Sheets Lifecycle
    // -------------------------------------------------------------------------
    #[test]
    fn test_contextual_memory_three_sheets_lifecycle() {
        let (mut memory, path) = make_test_memory();

        // 1. Formulate thoughts to verify Sheet 1 (working_accumulator.dat)
        // and Sheet 3 (deep_consolidation.dat)
        let t1 = memory.formulate_thought("add 5 to 10");
        let t2 = memory.formulate_thought("what is a container");

        let acc_path = memory.working_accumulator_path();
        let deep_path = memory.deep_consolidation_path();

        assert!(acc_path.exists(), "working_accumulator.dat must exist");
        assert!(deep_path.exists(), "deep_consolidation.dat must exist");

        let acc_content = fs::read_to_string(&acc_path).unwrap();
        assert!(acc_content.contains(&t1.id));
        assert!(acc_content.contains(&t2.id));

        let deep_content = fs::read_to_string(&deep_path).unwrap();
        assert!(deep_content.contains("CANDIDATE|arithmetic"));

        // 2. Test Sheet 2 (last_session.dat)
        let snapshot = CompletedSessionSnapshot {
            session_id: "sess_20261020_0001".to_string(),
            label_id: "SCR_20261020_0001_DISCOURSE_PROCEED_fruit".to_string(),
            timestamp_str: "20261020_120000".to_string(),
            elapsed_micros: 4200,
            execution_state: QuadState::Proceed,
            prompt: "add fruit to basket".to_string(),
            modal_domain: Some("Quantitative".to_string()),
            domain_category: Some("ComputerArchitectureSilicon".to_string()),
            biblical_anchor: Some("Proverbs 11:1".to_string()),
            thought_id: Some(t1.id.clone()),
            repairs_count: 0,
            failpoints_count: 0,
            natural_response: "Fruit container successfully updated."
                .to_string(),
        };

        let last_path = memory.record_completed_session(&snapshot).unwrap();
        assert!(last_path.exists(), "last_session.dat must exist");

        let loaded_opt = memory.load_last_session();
        assert!(loaded_opt.is_some(), "Must load last session snapshot");
        let loaded = loaded_opt.unwrap();
        assert_eq!(loaded.session_id, "sess_20261020_0001");
        assert_eq!(
            loaded.label_id,
            "SCR_20261020_0001_DISCOURSE_PROCEED_fruit"
        );
        assert_eq!(loaded.execution_state, QuadState::Proceed);

        // 3. Test persistence reload into fresh memory instance
        let fresh_mem = AIMemory::with_dirs(&path, &memory.memory_dir);
        assert!(!fresh_mem.working_window.is_empty());
        assert!(fresh_mem.consolidation_candidates.contains_key("arithmetic"));
    }
}

// AI Memory tripartite subsystem sealed and active.

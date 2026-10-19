// =============================================================================
// BLOCK 1: METADATA BLOCK
// ( Identity, Provenance, Invariant Boundary Contracts & Scriptural Anchors )
// =============================================================================
// @module        OmniCode AI Subsystem: Structured Lexicon Entry & Concordance
// @file          entry.rs
// @layer         Track 1 [Host Engine & Virtual Machine] -> src/ai/lexicon
// @version       1.0.0
// @epoch         2026.10
// @author        CreativeWorkzStudio LLC / ProfessorSeanEX
// @system        OmniCode AI Cortex & Living Kernel
// @repository    https://github.com/Novaking34/OmniCode_Framework
// @role          Ontological Entry Structure & 5-Fold Translation Concordance
// @invariants    Rank 1 (< >): Form precedes fullness. Every concept entry is
//                anchored to an immutable modal domain, structural category,
//                and contextual subdomain with dual-witness cryptographic
//                integrity and deterministic multi-translation coordinates.
// @witness       "In the mouth of two or three witnesses shall every word be
//                 established." — 2 Corinthians 13:1 (KJV)
//                "The words of the LORD are pure words: as silver tried in a
//                 furnace of earth, purified seven times." — Psalm 12:6 (KJV)
//
// PURPOSE & ARCHITECTURAL PHILOSOPHY:
//   LexiconEntry is the atomic ontological coordinate of the OmniCode
//   knowledge base. Rather than reducing meaning to floating-point token
//   embeddings, OmniCode anchors reality through the 6-Rank Morphological
//   Bracket Hierarchy:
//
//     Rank 1: < Fundamental Domain >       (Dooyeweerd Modal Law-Sphere)
//     Rank 2:   [ Domain Category ]        (Regional System Module)
//     Rank 3:     { SubDomain }            (Operating Theater / Field)
//     Rank 4:       [ SubCategory System } (Functional Branch)
//     Rank 5:         { Normative Rule ]   (Boundary Invariant Axiom)
//     Rank 6:           ( Concept / Term ) (The Concrete Identity)
//
// THE 5-FOLD TRANSLATION CONCORDANCE ARCHITECTURE:
//   1. Invariant Zero (Anchor 0) - King James Version (1611):
//      Historical public-domain foundation providing axiomatic root text.
//   2. Outward Extension (+1) - World English Bible (WEB):
//      Modern public-domain English rendering providing semantic
//      comprehension for modern human operators and AI transducers.
//   3. Concordance Pointers - NKJV, ESV, and Ethiopic Canon (88 Books):
//      Cryptographic coordinate pointers verifying the invariant across
//      ecclesiastical traditions without bundling copyrighted text in silicon.
//
// DUAL-COPY DATA ARCHITECTURE:
//   "Learning is in the BODY (filesystem), not the WEIGHTS (model)."
//   1. entry.base.dat : Read-only canonical ground-truth baseline.
//   2. entry.live.dat : Append-only journal for dynamic runtime expansion.
// =============================================================================

// =============================================================================
// BLOCK 2: OPENING BLOCK
// ( Types, Enums, Concordance Structures & Static Vault Path Contracts )
// =============================================================================

use std::fmt;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;
use crate::core::ternary::QuadState;
use super::domain::{DomainCategory, ModalDomain, SubDomain};

/// Supported scripture translations within the 5-fold concordance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScriptureTranslation {
    /// King James Version (1611) — Invariant Public Domain Root (Anchor 0).
    Kjv,
    /// World English Bible — Modern Public Domain English (+1).
    Web,
    /// New King James Version — Concordance coordinate pointer + vault hash.
    Nkjv,
    /// English Standard Version — Concordance pointer + vault hash.
    Esv,
    /// Ethiopian Orthodox Tewahedo Canon (88 Books including 1 Enoch).
    Ethiopic,
}

/// Static specification descriptor for a supported scripture translation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TranslationSpec {
    pub translation: ScriptureTranslation,
    pub abbreviation: &'static str,
    pub title: &'static str,
    pub is_public_domain: bool,
    pub vault_filename: &'static str,
    pub canonical_book_count: u16,
}

/// Immutable static table of all 5 supported scripture translations.
pub static TRANSLATION_SPEC_TABLE: [TranslationSpec; 5] = [
    TranslationSpec {
        translation: ScriptureTranslation::Kjv,
        abbreviation: "KJV",
        title: "King James Version (1611)",
        is_public_domain: true,
        vault_filename: "King James Bible - Complete.pdf",
        canonical_book_count: 66,
    },
    TranslationSpec {
        translation: ScriptureTranslation::Web,
        abbreviation: "WEB",
        title: "World English Bible",
        is_public_domain: true,
        vault_filename: "World English Bible - Complete.pdf",
        canonical_book_count: 66,
    },
    TranslationSpec {
        translation: ScriptureTranslation::Nkjv,
        abbreviation: "NKJV",
        title: "New King James Version",
        is_public_domain: false,
        vault_filename: "New King James Bible - Complete.pdf",
        canonical_book_count: 66,
    },
    TranslationSpec {
        translation: ScriptureTranslation::Esv,
        abbreviation: "ESV",
        title: "English Standard Version",
        is_public_domain: false,
        vault_filename: "English Standard Bible - Complete.pdf",
        canonical_book_count: 66,
    },
    TranslationSpec {
        translation: ScriptureTranslation::Ethiopic,
        abbreviation: "ETHIOPIC",
        title: "Ethiopian Orthodox Canon (88 Books)",
        is_public_domain: false,
        vault_filename: "Ethiopian Bible - Complete.pdf",
        canonical_book_count: 88,
    },
];

/// Canonical coordinate pointer linking an entry to an immutable vault source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcordanceRef {
    pub translation: ScriptureTranslation,
    pub book: String,
    pub chapter: u16,
    pub verse_start: u16,
    pub verse_end: u16,
    /// Cryptographic verse signature verifying text integrity against vault.
    pub hash_signature: String,
}

/// A structured entry in the universal world lexicon, anchored to
/// the 6-rank morphological boundary hierarchy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexiconEntry {
    /// Canonical keyword / concept identifier (lowercase).
    pub term: String,
    /// Rank 1 (< ... >): Invariant Fundamental Domain.
    pub domain: ModalDomain,
    /// Rank 2 ([ ... ]): Concrete Structural Category.
    pub category: DomainCategory,
    /// Rank 3 ({ ... }): Contextual Sub-Domain.
    pub subdomain: SubDomain,
    /// Foundational definition explaining what this entity or concept is.
    pub definition: String,
    /// King James Version (KJV) citation and text (Public Domain Anchor 0).
    pub kjv_passage: String,
    /// World English Bible (WEB) citation and text (Public Domain Key +1).
    pub web_passage: String,
    /// Natural resonance within Balanced Ternary Quad-State logic.
    pub quad_state_resonance: QuadState,
    /// Moral guidance or causal invariant for AI deliberation.
    pub moral_guidance: String,
    /// Multi-translation concordance coordinate pointers.
    pub concordance_refs: Vec<ConcordanceRef>,
}

/// Diagnostic report verifying the structural, ontological, and scriptural
/// integrity of a LexiconEntry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryValidationReport {
    pub term: String,
    pub is_valid: bool,
    pub has_dual_witness: bool,
    pub has_valid_domain_hierarchy: bool,
    pub concordance_count: usize,
    pub errors: Vec<String>,
}

/// Comprehensive audit report for an active EntryStore.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryAuditReport {
    pub total_entries: usize,
    pub total_concordances: usize,
    pub is_valid: bool,
    pub errors: Vec<String>,
}

// =============================================================================
// BLOCK 3: BODY BLOCK
// ( Parsing Engine, EntryStore, Dynamic Lookups & Dual-Copy Journaling )
// =============================================================================

/// Canonical compiled-in baseline data snapshot.
pub const CANONICAL_ENTRY_BASE_DAT: &str =
    include_str!("data/entry.base.dat");

/// Active Entry Store managing universal world lexicon entries and journals.
#[derive(Debug, Clone, Default)]
pub struct EntryStore {
    pub entries: Vec<LexiconEntry>,
    pub live_entries: Vec<LexiconEntry>,
}

impl ScriptureTranslation {
    /// Exact programmatic identifier string.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Kjv => "Kjv",
            Self::Web => "Web",
            Self::Nkjv => "Nkjv",
            Self::Esv => "Esv",
            Self::Ethiopic => "Ethiopic",
        }
    }

    /// Resolves the static specification descriptor for this translation.
    pub fn spec(&self) -> &'static TranslationSpec {
        TRANSLATION_SPEC_TABLE
            .iter()
            .find(|s| s.translation == *self)
            .expect("All ScriptureTranslations must have static descriptors")
    }

    /// Canonical short abbreviation tag (e.g. "KJV", "WEB").
    pub fn abbreviation(&self) -> &'static str {
        self.spec().abbreviation
    }

    /// Human-readable title of the translation.
    pub fn title(&self) -> &'static str {
        self.spec().title
    }

    /// Whether full prose text is public domain and safe for binary embedding.
    pub fn is_public_domain(&self) -> bool {
        self.spec().is_public_domain
    }

    /// Associated immutable PDF artifact filename in Track 4 vault.
    pub fn vault_filename(&self) -> &'static str {
        self.spec().vault_filename
    }

    /// Canonical book count for this scriptural canon.
    pub fn canonical_book_count(&self) -> u16 {
        self.spec().canonical_book_count
    }
}

impl fmt::Display for ScriptureTranslation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.abbreviation())
    }
}

impl ConcordanceRef {
    /// Constructs a general concordance coordinate pointer.
    pub fn new(
        translation: ScriptureTranslation,
        book: impl Into<String>,
        chapter: u16,
        verse_start: u16,
        verse_end: u16,
        hash_signature: impl Into<String>,
    ) -> Self {
        Self {
            translation,
            book: book.into(),
            chapter,
            verse_start,
            verse_end,
            hash_signature: hash_signature.into(),
        }
    }

    /// Constructs a New King James Version concordance pointer.
    pub fn nkjv(
        book: impl Into<String>,
        chapter: u16,
        start: u16,
        end: u16,
        hash: impl Into<String>,
    ) -> Self {
        Self::new(ScriptureTranslation::Nkjv, book, chapter, start, end, hash)
    }

    /// Constructs an English Standard Version concordance pointer.
    pub fn esv(
        book: impl Into<String>,
        chapter: u16,
        start: u16,
        end: u16,
        hash: impl Into<String>,
    ) -> Self {
        Self::new(ScriptureTranslation::Esv, book, chapter, start, end, hash)
    }

    /// Constructs an Ethiopian Canon (88 books) concordance pointer.
    pub fn ethiopic(
        book: impl Into<String>,
        chapter: u16,
        start: u16,
        end: u16,
        hash: impl Into<String>,
    ) -> Self {
        Self::new(
            ScriptureTranslation::Ethiopic,
            book,
            chapter,
            start,
            end,
            hash,
        )
    }

    /// Parses compact string e.g.
    /// "NKJV:Genesis:1:3-3:sha256:light_gen1_3_nkjv".
    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() < 5 {
            return None;
        }
        let trans = match parts[0].to_uppercase().as_str() {
            "KJV" => ScriptureTranslation::Kjv,
            "WEB" => ScriptureTranslation::Web,
            "NKJV" => ScriptureTranslation::Nkjv,
            "ESV" => ScriptureTranslation::Esv,
            "ETHIOPIC" => ScriptureTranslation::Ethiopic,
            _ => return None,
        };
        let book = parts[1].to_string();
        let chapter = parts[2].parse::<u16>().ok()?;
        let span_parts: Vec<&str> = parts[3].split('-').collect();
        let start = span_parts.first()?.parse::<u16>().ok()?;
        let end = span_parts
            .get(1)
            .unwrap_or(&span_parts[0])
            .parse::<u16>()
            .ok()?;
        let hash = parts[4..].join(":");
        Some(Self::new(trans, book, chapter, start, end, hash))
    }

    /// Formats as compact specification string.
    pub fn format_compact(&self) -> String {
        format!(
            "{}:{}:{}:{}-{}:{}",
            self.translation.abbreviation(),
            self.book,
            self.chapter,
            self.verse_start,
            self.verse_end,
            self.hash_signature
        )
    }

    /// Formats the coordinate citation (e.g. "Genesis 1:3-4 [NKJV]").
    pub fn format_citation(&self) -> String {
        let tag = self.translation.abbreviation();
        if self.verse_start == self.verse_end {
            format!(
                "{} {}:{} [{tag}]",
                self.book, self.chapter, self.verse_start
            )
        } else {
            format!(
                "{} {}:{}-{} [{tag}]",
                self.book, self.chapter, self.verse_start, self.verse_end
            )
        }
    }

    /// Full storage path to the immutable vault documentation.
    pub fn vault_relative_path(&self) -> String {
        let prefix = "<TRACK 4> : {Variable Storage & Episodic Memory}";
        let dir = "storage/data/immutable_documentation";
        format!("{prefix}/{dir}/{}", self.translation.vault_filename())
    }

    /// Validates whether chapter and verse range bounds are semantically legal.
    pub fn is_valid_range(&self) -> bool {
        self.chapter > 0
            && self.verse_start > 0
            && self.verse_start <= self.verse_end
    }

    /// Verifies if hash signature matches expected cryptographic prefix.
    pub fn has_valid_hash_format(&self) -> bool {
        self.hash_signature.starts_with("sha256:")
            || self.hash_signature.starts_with("hash:")
    }
}

impl fmt::Display for ConcordanceRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_citation())
    }
}

impl LexiconEntry {
    /// Instantiates a new LexiconEntry with foundational fields.
    pub fn new(
        term: impl Into<String>,
        domain: ModalDomain,
        category: DomainCategory,
        subdomain: SubDomain,
        definition: impl Into<String>,
        kjv_passage: impl Into<String>,
        web_passage: impl Into<String>,
        quad_state_resonance: QuadState,
        moral_guidance: impl Into<String>,
    ) -> Self {
        Self {
            term: term.into(),
            domain,
            category,
            subdomain,
            definition: definition.into(),
            kjv_passage: kjv_passage.into(),
            web_passage: web_passage.into(),
            quad_state_resonance,
            moral_guidance: moral_guidance.into(),
            concordance_refs: Vec::new(),
        }
    }

    /// Fluent builder to attach concordance coordinate references.
    pub fn with_concordances(mut self, refs: Vec<ConcordanceRef>) -> Self {
        self.concordance_refs = refs;
        self
    }

    /// Resolves the full bracket-ranked path for this concept:
    /// `< Domain [ Category { SubDomain [ System { Rule ( Term ) } ] } ] >`
    pub fn bracket_path(&self) -> String {
        self.subdomain.format_bracket_path(&self.term)
    }

    /// Formats a concise 4-tier bracket coordinate:
    /// `< Domain [ Category { SubDomain ( Term ) } ] >`
    pub fn short_bracket_path(&self) -> String {
        format!(
            "< {} [ {:?} {{ {} ( {} ) }} ] >",
            self.domain.title(),
            self.category,
            self.subdomain.descriptor().title,
            self.term
        )
    }

    /// Looks up a concordance reference for a specific non-public translation.
    pub fn concordance_for(
        &self,
        translation: ScriptureTranslation,
    ) -> Option<&ConcordanceRef> {
        self.concordance_refs.iter().find(|r| r.translation == translation)
    }

    /// Formats all available witness citations for logging or prompt export.
    pub fn all_witness_citations(&self) -> Vec<String> {
        let mut citations = vec![
            format!("KJV (Encryption Anchor): {}", self.kjv_passage),
            format!("WEB (Decryption Key): {}", self.web_passage),
        ];
        for c in &self.concordance_refs {
            citations.push(format!(
                "{}: (Vault: {})",
                c.format_citation(),
                c.vault_relative_path()
            ));
        }
        citations
    }

    /// Formal Invariant Encryption Key (Canonical KJV Passage: Anchor 0).
    pub fn encryption_anchor(&self) -> &str {
        &self.kjv_passage
    }

    /// Outward Accessible Decryption Key (Plain Modern WEB: Extension +1).
    pub fn decryption_revelation(&self) -> &str {
        &self.web_passage
    }

    /// Dual-witness cryptographic pair: `(KJV_Encryption, WEB_Decryption)`.
    pub fn dual_witness_pair(&self) -> (&str, &str) {
        (&self.kjv_passage, &self.web_passage)
    }

    /// Validates internal consistency against ontological and scriptural rules.
    pub fn validate(&self) -> EntryValidationReport {
        let mut errors = Vec::new();

        if self.term.trim().is_empty() {
            errors.push("Term identifier must not be empty".to_string());
        }

        if self.definition.trim().is_empty() {
            errors.push("Definition must not be empty".to_string());
        }

        let has_dual_witness = !self.kjv_passage.trim().is_empty()
            && !self.web_passage.trim().is_empty();
        if !has_dual_witness {
            errors.push(
                "Entry must provide both KJV and WEB dual witnesses"
                    .to_string(),
            );
        }

        let sub_desc = self.subdomain.descriptor();
        let domain_coherent = sub_desc.domain == self.domain
            && sub_desc.category == self.category
            && self.category.parent_domain() == self.domain;

        if !domain_coherent {
            errors.push(format!(
                "Ontological mismatch: {:?} expects {:?}/{:?}, got {:?}/{:?}",
                self.subdomain,
                sub_desc.domain,
                sub_desc.category,
                self.domain,
                self.category
            ));
        }

        for (idx, c) in self.concordance_refs.iter().enumerate() {
            if !c.is_valid_range() {
                errors.push(format!(
                    "Concordance index {idx} has invalid verse range {}:{}",
                    c.verse_start, c.verse_end
                ));
            }
            if !c.has_valid_hash_format() {
                errors.push(format!(
                    "Concordance index {idx} has invalid hash format: {}",
                    c.hash_signature
                ));
            }
        }

        let is_valid = errors.is_empty();

        EntryValidationReport {
            term: self.term.clone(),
            is_valid,
            has_dual_witness,
            has_valid_domain_hierarchy: domain_coherent,
            concordance_count: self.concordance_refs.len(),
            errors,
        }
    }

    /// Renders an ASCII telemetry card for terminal inspection and debugger.
    pub fn render_telemetry_card(&self) -> String {
        let mut out = String::new();
        let border =
            "+---------------------------------------------------------+\n";
        out.push_str(border);
        out.push_str(&format!(
            "| LEXICON ENTRY: {:<39} |\n",
            self.term.to_uppercase()
        ));
        out.push_str(border);
        out.push_str(&format!(
            "| Coordinate : {:<41} |\n",
            self.short_bracket_path()
        ));
        let res_str = format!(
            "{:?} ({})",
            self.quad_state_resonance,
            self.quad_state_resonance.macro_op()
        );
        out.push_str(&format!("| Resonance  : {:<41} |\n", res_str));
        out.push_str(&format!(
            "| Guidance   : {:<41} |\n",
            self.moral_guidance.chars().take(41).collect::<String>()
        ));
        let wit_count = format!(
            "{} Concordances",
            self.concordance_refs.len()
        );
        out.push_str(&format!(
            "| Witnesses  : KJV (0) + WEB (+1) + {:<14} |\n",
            wit_count
        ));
        out.push_str(border);
        out
    }

    /// Serializes entry to a compact JSON fragment for CLI and REST endpoints.
    pub fn to_json_fragment(&self) -> String {
        let esc = |s: &str| -> String {
            s.replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', " ")
        };
        format!(
            "{{\"term\":\"{}\",\"domain\":\"{}\",\"category\":\"{}\",\
\"subdomain\":\"{}\",\"definition\":\"{}\",\"kjv\":\"{}\",\
\"web\":\"{}\",\"state\":\"{}\",\"guidance\":\"{}\",\
\"concordance_count\":{}}}",
            esc(&self.term),
            esc(self.domain.as_str()),
            esc(self.category.as_str()),
            esc(self.subdomain.as_str()),
            esc(&self.definition),
            esc(&self.kjv_passage),
            esc(&self.web_passage),
            esc(self.quad_state_resonance.macro_op()),
            esc(&self.moral_guidance),
            self.concordance_refs.len()
        )
    }
}

// -----------------------------------------------------------------------------
// Helper parsers for Domain, Category, and SubDomain IDs
// -----------------------------------------------------------------------------
fn parse_modal_domain(id: &str) -> ModalDomain {
    match id {
        "Quantitative" => ModalDomain::Quantitative,
        "Spatial" => ModalDomain::Spatial,
        "Kinematic" => ModalDomain::Kinematic,
        "Physical" | "CosmosNaturalOrder" => ModalDomain::Physical,
        "BioticOrganic" | "BiosphereSomatic" => ModalDomain::BioticOrganic,
        "SensitivePsychic" => ModalDomain::SensitivePsychic,
        "AnalyticalLogical" => ModalDomain::AnalyticalLogical,
        "HistoricalFormative" | "InstrumentalCybernetic" => {
            ModalDomain::HistoricalFormative
        }
        "Lingual" | "CulturalCommunicative" => ModalDomain::Lingual,
        "Social" | "SocialCivilGovernance" => ModalDomain::Social,
        "Economic" => ModalDomain::Economic,
        "Aesthetic" => ModalDomain::Aesthetic,
        "Juridical" => ModalDomain::Juridical,
        "Ethical" => ModalDomain::Ethical,
        "Pistic" | "TranscendentPhilosophy" => ModalDomain::Pistic,
        _ => ModalDomain::AnalyticalLogical,
    }
}

fn parse_domain_category(id: &str) -> DomainCategory {
    match id {
        "TheologyReligion" => DomainCategory::TheologyReligion,
        "CivilOrderJurisprudence" => DomainCategory::CivilOrderJurisprudence,
        "EthicsMoralLaw" => DomainCategory::EthicsMoralLaw,
        "EcologyStewardship" => DomainCategory::EcologyStewardship,
        "FamilyKinship" => DomainCategory::FamilyKinship,
        "EconomicsCommerce" => DomainCategory::EconomicsCommerce,
        "AstronomyCosmology" => DomainCategory::AstronomyCosmology,
        "NaturalSciences" => DomainCategory::NaturalSciences,
        "EarthHydrology" => DomainCategory::EarthHydrology,
        "HumanBiology" => DomainCategory::HumanBiology,
        "ComputerArchitectureSilicon" => {
            DomainCategory::ComputerArchitectureSilicon
        }
        "OperatingSystemsRuntime" => DomainCategory::OperatingSystemsRuntime,
        "ArtificialIntelligenceCognitive" => {
            DomainCategory::ArtificialIntelligenceCognitive
        }
        "Philosophy" => DomainCategory::Philosophy,
        "LinguisticsSemiotics" => DomainCategory::LinguisticsSemiotics,
        _ => DomainCategory::TheologyReligion,
    }
}

fn parse_subdomain(id: &str) -> SubDomain {
    match id {
        "BiblicalOntology" => SubDomain::BiblicalOntology,
        "CovenantLaw" => SubDomain::CovenantLaw,
        "CelestialChronometry" => SubDomain::CelestialChronometry,
        "ClassicalMechanics" => SubDomain::ClassicalMechanics,
        "HydrologicalCycle" => SubDomain::HydrologicalCycle,
        "AgriculturalHusbandry" => SubDomain::AgriculturalHusbandry,
        "PhysiologicalSystems" => SubDomain::PhysiologicalSystems,
        "StatutoryConstitutionalLaw" => {
            SubDomain::StatutoryConstitutionalLaw
        }
        "DeontologicalInvariants" => SubDomain::DeontologicalInvariants,
        "EcologicalEquilibrium" => SubDomain::EcologicalEquilibrium,
        "GenerationalInheritance" => SubDomain::GenerationalInheritance,
        "CivilJusticeRestitution" => SubDomain::CivilJusticeRestitution,
        "MarketStewardship" => SubDomain::MarketStewardship,
        "MemoryMappedSilicon" => SubDomain::MemoryMappedSilicon,
        "ProcessSchedulingLifecycle" => {
            SubDomain::ProcessSchedulingLifecycle
        }
        "AttractorDynamicsConscience" => {
            SubDomain::AttractorDynamicsConscience
        }
        "TernaryLogicVerification" => SubDomain::TernaryLogicVerification,
        "DirectionalSyntaxSvo" => SubDomain::DirectionalSyntaxSvo,
        _ => SubDomain::BiblicalOntology,
    }
}

impl EntryStore {
    /// Loads canonical baseline entries from the compiled-in static string.
    pub fn load_from_static() -> Self {
        let mut store = Self::default();
        store.ingest_dat_content(CANONICAL_ENTRY_BASE_DAT);
        store
    }

    /// Loads base entries and overlays live journal from a target directory.
    pub fn load_from_directory<P: AsRef<Path>>(dir: P) -> io::Result<Self> {
        let mut store = Self::load_from_static();
        let base_file = dir.as_ref().join("entry.base.dat");
        if base_file.exists() {
            let content = std::fs::read_to_string(&base_file)?;
            store = Self::default();
            store.ingest_dat_content(&content);
        }

        let live_file = dir.as_ref().join("entry.live.dat");
        if live_file.exists() {
            let live_content = std::fs::read_to_string(&live_file)?;
            store.ingest_live_content(&live_content);
        }

        Ok(store)
    }

    /// Parses a single pipe-delimited ENTRY line.
    pub fn parse_entry_line(line: &str) -> Option<LexiconEntry> {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            return None;
        }

        let parts: Vec<&str> = trimmed.split('|').collect();
        if parts.len() < 10 || parts[0] != "ENTRY" {
            return None;
        }

        let term = parts[1].to_string();
        let domain = parse_modal_domain(parts[2]);
        let category = parse_domain_category(parts[3]);
        let subdomain = parse_subdomain(parts[4]);

        let quad_state = match parts[5] {
            "+1" | "Proceed" => QuadState::Proceed,
            "-1" | "Halt" => QuadState::Halt,
            "0" | "Rollback" => QuadState::Rollback,
            _ => QuadState::Await,
        };

        let definition = parts[6].to_string();
        let kjv_passage = parts[7].to_string();
        let web_passage = parts[8].to_string();
        let moral_guidance = parts[9].to_string();

        let mut concordance_refs = Vec::new();
        if parts.len() >= 11 && !parts[10].trim().is_empty() {
            for spec in parts[10].split(';') {
                if let Some(r) = ConcordanceRef::parse(spec.trim()) {
                    concordance_refs.push(r);
                }
            }
        }

        Some(LexiconEntry {
            term,
            domain,
            category,
            subdomain,
            definition,
            kjv_passage,
            web_passage,
            quad_state_resonance: quad_state,
            moral_guidance,
            concordance_refs,
        })
    }

    /// Ingests entries from raw pipe-delimited data sheet content.
    pub fn ingest_dat_content(&mut self, content: &str) {
        for line in content.lines() {
            if let Some(entry) = Self::parse_entry_line(line) {
                if !self.entries.iter().any(|e| e.term == entry.term) {
                    self.entries.push(entry);
                }
            }
        }
    }

    /// Ingests entries from live journal content.
    pub fn ingest_live_content(&mut self, content: &str) {
        for line in content.lines() {
            if let Some(entry) = Self::parse_entry_line(line) {
                if !self.entries.iter().any(|e| e.term == entry.term) {
                    self.live_entries.push(entry.clone());
                    self.entries.push(entry);
                }
            }
        }
    }

    /// Appends a new entry to the live journal file and updates the store.
    pub fn append_live_entry<P: AsRef<Path>>(
        &mut self,
        path: P,
        entry: LexiconEntry,
    ) -> io::Result<()> {
        let state_str = match entry.quad_state_resonance {
            QuadState::Proceed => "+1",
            QuadState::Halt => "-1",
            QuadState::Rollback => "0",
            QuadState::Await => "Await",
        };

        let concordances_str = entry
            .concordance_refs
            .iter()
            .map(|c| c.format_compact())
            .collect::<Vec<String>>()
            .join(";");

        let line = format!(
            "ENTRY|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            entry.term,
            entry.domain.as_str(),
            entry.category.as_str(),
            entry.subdomain.as_str(),
            state_str,
            entry.definition.replace('|', ":"),
            entry.kjv_passage.replace('|', ":"),
            entry.web_passage.replace('|', ":"),
            entry.moral_guidance.replace('|', ":"),
            concordances_str
        );

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        file.write_all(line.as_bytes())?;

        if !self.entries.iter().any(|e| e.term == entry.term) {
            self.live_entries.push(entry.clone());
            self.entries.push(entry);
        }

        Ok(())
    }

    /// Looks up a specific concept entry by keyword.
    pub fn find_entry(&self, term: &str) -> Option<&LexiconEntry> {
        let clean = term.trim().to_lowercase();
        self.entries.iter().find(|e| e.term == clean)
    }

    /// Filters entries belonging to a specific sovereign modal domain.
    pub fn entries_for_domain(
        &self,
        domain: ModalDomain,
    ) -> Vec<&LexiconEntry> {
        self.entries.iter().filter(|e| e.domain == domain).collect()
    }

    /// Filters entries belonging to a specific subdomain.
    pub fn entries_for_subdomain(
        &self,
        subdomain: SubDomain,
    ) -> Vec<&LexiconEntry> {
        self.entries
            .iter()
            .filter(|e| e.subdomain == subdomain)
            .collect()
    }

    /// Searches for entries matching a keyword in term or definition.
    pub fn search(&self, query: &str) -> Vec<&LexiconEntry> {
        let clean = query.trim().to_lowercase();
        self.entries
            .iter()
            .filter(|e| {
                e.term.contains(&clean)
                    || e.definition.to_lowercase().contains(&clean)
            })
            .collect()
    }
}

// =============================================================================
// BLOCK 4: AUDIT PROCEDURES
// ( Diagnostic Triad Verification & Vault Contract Audits )
// =============================================================================

/// Audits an entry and logs diagnostics if any covenant invariant is breached.
pub fn audit_lexicon_entry(entry: &LexiconEntry) -> bool {
    let report = entry.validate();
    report.is_valid
}

/// Verifies that a concordance reference adheres to vault pathing contracts.
pub fn verify_vault_reference_contract(coord: &ConcordanceRef) -> bool {
    coord.is_valid_range()
        && coord.has_valid_hash_format()
        && !coord.book.is_empty()
}

/// Audits an active EntryStore instance for ontological completeness.
pub fn audit_entry_store(store: &EntryStore) -> EntryAuditReport {
    let mut errors = Vec::new();
    let mut total_concordances = 0;

    if store.entries.is_empty() {
        errors.push("EntryStore contains zero entries".to_string());
    }

    let mut terms_seen = Vec::new();
    for entry in &store.entries {
        if terms_seen.contains(&entry.term) {
            errors.push(format!("Duplicate term detected: '{}'", entry.term));
        } else {
            terms_seen.push(entry.term.clone());
        }

        total_concordances += entry.concordance_refs.len();

        let validation = entry.validate();
        if !validation.is_valid {
            for err in validation.errors {
                errors.push(format!("Entry '{}': {}", entry.term, err));
            }
        }
    }

    EntryAuditReport {
        total_entries: store.entries.len(),
        total_concordances,
        is_valid: errors.is_empty(),
        errors,
    }
}

/// Audits the compiled-in canonical baseline entry data sheet.
pub fn audit_canonical_entry_baseline() -> EntryAuditReport {
    let store = EntryStore::load_from_static();
    audit_entry_store(&store)
}

// =============================================================================
// BLOCK 5: CLOSING BLOCK
// ( Comprehensive Multi-Tier Functional Test Suites & Invariant Tripwires )
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Test Suite 1: Translation Specification & Vault Invariants
    // -------------------------------------------------------------------------
    #[test]
    fn test_translation_spec_and_vault_contracts() {
        assert_eq!(TRANSLATION_SPEC_TABLE.len(), 5);

        // KJV (Invariant 0) must be public domain with 66 books
        let kjv = ScriptureTranslation::Kjv;
        assert_eq!(kjv.as_str(), "Kjv");
        assert_eq!(kjv.abbreviation(), "KJV");
        assert!(kjv.is_public_domain());
        assert_eq!(kjv.canonical_book_count(), 66);
        assert_eq!(kjv.vault_filename(), "King James Bible - Complete.pdf");

        // WEB (Key +1) must be public domain with 66 books
        let web = ScriptureTranslation::Web;
        assert_eq!(web.as_str(), "Web");
        assert_eq!(web.abbreviation(), "WEB");
        assert!(web.is_public_domain());
        assert_eq!(web.canonical_book_count(), 66);
        assert_eq!(web.vault_filename(), "World English Bible - Complete.pdf");

        // NKJV & ESV must NOT be marked public domain (concordance-only)
        let nkjv = ScriptureTranslation::Nkjv;
        assert!(!nkjv.is_public_domain());
        assert_eq!(nkjv.abbreviation(), "NKJV");
        assert_eq!(nkjv.canonical_book_count(), 66);

        let esv = ScriptureTranslation::Esv;
        assert!(!esv.is_public_domain());
        assert_eq!(esv.abbreviation(), "ESV");
        assert_eq!(esv.canonical_book_count(), 66);

        // Ethiopian Canon must feature 88 books
        let eth = ScriptureTranslation::Ethiopic;
        assert!(!eth.is_public_domain());
        assert_eq!(eth.abbreviation(), "ETHIOPIC");
        assert_eq!(eth.canonical_book_count(), 88);
        assert_eq!(eth.vault_filename(), "Ethiopian Bible - Complete.pdf");

        // Display formatting check
        assert_eq!(format!("{kjv}"), "KJV");
        assert_eq!(format!("{eth}"), "ETHIOPIC");
    }

    // -------------------------------------------------------------------------
    // Test Suite 2: Concordance Reference Construction, Parsing & Validation
    // -------------------------------------------------------------------------
    #[test]
    fn test_concordance_ref_construction_and_validation() {
        // Single-verse citation
        let r_nkjv = ConcordanceRef::nkjv(
            "Genesis",
            1,
            3,
            3,
            "sha256:light_gen1_3_nkjv",
        );
        assert_eq!(r_nkjv.format_citation(), "Genesis 1:3 [NKJV]");
        assert_eq!(format!("{r_nkjv}"), "Genesis 1:3 [NKJV]");
        assert!(r_nkjv.is_valid_range());
        assert!(r_nkjv.has_valid_hash_format());
        assert!(verify_vault_reference_contract(&r_nkjv));
        assert!(
            r_nkjv.vault_relative_path().contains("New King James Bible"),
            "Vault path must reference NKJV document"
        );

        // Multi-verse span citation
        let r_eth = ConcordanceRef::ethiopic(
            "Enoch",
            1,
            1,
            9,
            "sha256:enoch_1_1_9_eth",
        );
        assert_eq!(r_eth.format_citation(), "Enoch 1:1-9 [ETHIOPIC]");
        assert!(r_eth.is_valid_range());
        assert!(r_eth.has_valid_hash_format());
        assert!(verify_vault_reference_contract(&r_eth));

        // Compact specification string serialization and parsing
        let compact = r_nkjv.format_compact();
        assert_eq!(compact, "NKJV:Genesis:1:3-3:sha256:light_gen1_3_nkjv");
        let parsed = ConcordanceRef::parse(&compact).expect("Must parse");
        assert_eq!(parsed, r_nkjv);

        // Tripwire: Invalid verse span (start > end)
        let r_invalid = ConcordanceRef::new(
            ScriptureTranslation::Kjv,
            "Proverbs",
            3,
            10,
            5,
            "sha256:invalid_span",
        );
        assert!(!r_invalid.is_valid_range());
        assert!(!verify_vault_reference_contract(&r_invalid));

        // Tripwire: Invalid hash format (missing prefix)
        let r_bad_hash = ConcordanceRef::new(
            ScriptureTranslation::Kjv,
            "Proverbs",
            3,
            5,
            6,
            "unhashed_raw_string",
        );
        assert!(r_bad_hash.is_valid_range());
        assert!(!r_bad_hash.has_valid_hash_format());
        assert!(!verify_vault_reference_contract(&r_bad_hash));
    }

    // -------------------------------------------------------------------------
    // Test Suite 3: EntryStore Data Loading from Canonical Baseline
    // -------------------------------------------------------------------------
    #[test]
    fn test_entry_store_loads_base_dat_and_all_entries() {
        let store = EntryStore::load_from_static();
        assert!(store.entries.len() >= 25);

        // Lookup Light
        let light = store.find_entry("light").expect("Light must exist");
        assert_eq!(light.term, "light");
        assert_eq!(light.domain, ModalDomain::Pistic);
        assert_eq!(light.category, DomainCategory::TheologyReligion);
        assert_eq!(light.subdomain, SubDomain::BiblicalOntology);
        assert_eq!(light.quad_state_resonance, QuadState::Proceed);
        assert!(!light.concordance_refs.is_empty());

        // Lookup Justice
        let justice = store.find_entry("justice").expect("Justice must exist");
        assert_eq!(justice.domain, ModalDomain::Juridical);
        assert_eq!(justice.category, DomainCategory::CivilOrderJurisprudence);
        assert_eq!(justice.subdomain, SubDomain::CivilJusticeRestitution);

        // Lookup Murder (Negative Invariant)
        let murder = store.find_entry("murder").expect("Murder must exist");
        assert_eq!(murder.quad_state_resonance, QuadState::Halt);

        // Search query
        let results = store.search("covenant");
        assert!(!results.is_empty());
        assert!(results.iter().any(|e| e.term == "covenant"));

        // Domain filter
        let physicals = store.entries_for_domain(ModalDomain::Physical);
        assert!(physicals.iter().any(|e| e.term == "heavens"));
        assert!(physicals.iter().any(|e| e.term == "earth"));
        assert!(physicals.iter().any(|e| e.term == "waters"));
    }

    // -------------------------------------------------------------------------
    // Test Suite 4: Lexicon Entry Ontological Hierarchy & Dual Witnesses
    // -------------------------------------------------------------------------
    #[test]
    fn test_lexicon_entry_ontological_hierarchy_and_validation() {
        let entry = LexiconEntry::new(
            "light",
            ModalDomain::Pistic,
            DomainCategory::TheologyReligion,
            SubDomain::BiblicalOntology,
            "Illumination, truth, and order called into being by God.",
            "Genesis 1:3 - 'And God said, Let there be light...'",
            "Genesis 1:3 - 'God said, \"Let there be light...\"'",
            QuadState::Proceed,
            "Walk in the light; all operations must be transparent and true.",
        ).with_concordances(vec![
            ConcordanceRef::nkjv(
                "Genesis", 1, 3, 3, "sha256:light_gen1_3_nkjv"
            ),
            ConcordanceRef::esv(
                "Genesis", 1, 3, 3, "sha256:light_gen1_3_esv"
            ),
        ]);

        let report = entry.validate();
        assert!(report.is_valid, "Valid entry must produce valid report");
        assert!(report.has_dual_witness);
        assert!(report.has_valid_domain_hierarchy);
        assert_eq!(report.concordance_count, 2);
        assert!(report.errors.is_empty());
        assert!(audit_lexicon_entry(&entry));

        // Bracket paths
        let full_path = entry.bracket_path();
        assert!(full_path.starts_with("[Pistic > TheologyReligion"));
        assert!(full_path.contains("Biblical Ontology > light]"));

        let short_path = entry.short_bracket_path();
        assert!(short_path.contains("Pistic"));
        assert!(short_path.contains("( light )"));

        // Cryptographic dual-witness pair
        let (enc, dec) = entry.dual_witness_pair();
        assert_eq!(enc, entry.encryption_anchor());
        assert_eq!(dec, entry.decryption_revelation());

        // Concordance lookup
        let nkjv_ref = entry.concordance_for(ScriptureTranslation::Nkjv);
        assert!(nkjv_ref.is_some());
        assert_eq!(nkjv_ref.unwrap().chapter, 1);

        let eth_ref = entry.concordance_for(ScriptureTranslation::Ethiopic);
        assert!(eth_ref.is_none());

        // Witness citations
        let citations = entry.all_witness_citations();
        assert_eq!(citations.len(), 4);
        assert!(citations[0].starts_with("KJV (Encryption Anchor):"));
        assert!(citations[1].starts_with("WEB (Decryption Key):"));
    }

    // -------------------------------------------------------------------------
    // Test Suite 5: Live Journal Dynamic Append & In-Memory Update
    // -------------------------------------------------------------------------
    #[test]
    fn test_entry_store_live_journal_append() {
        let mut store = EntryStore::load_from_static();
        let initial_count = store.entries.len();

        let temp_journal_path = std::env::temp_dir()
            .join(format!("omnicode_entry_test_{}.dat", std::process::id()));

        let new_entry = LexiconEntry::new(
            "integrity",
            ModalDomain::Ethical,
            DomainCategory::EthicsMoralLaw,
            SubDomain::DeontologicalInvariants,
            "Structural wholeness and steadfast alignment with truth.",
            "Proverbs 11:3 - The integrity of the upright shall guide them.",
            "Proverbs 11:3 - The integrity of the upright shall guide them.",
            QuadState::Proceed,
            "Maintain pure structural integrity without compromise.",
        );

        let append_res = store.append_live_entry(
            &temp_journal_path,
            new_entry.clone(),
        );
        assert!(append_res.is_ok(), "Live journal append must succeed");
        assert_eq!(store.entries.len(), initial_count + 1);
        assert_eq!(store.live_entries.len(), 1);
        assert!(store.find_entry("integrity").is_some());

        // Clean up temporary file
        let _ = std::fs::remove_file(temp_journal_path);
    }

    // -------------------------------------------------------------------------
    // Test Suite 6: Telemetry Formatting, JSON Serialization & Error Tripwires
    // -------------------------------------------------------------------------
    #[test]
    fn test_telemetry_json_and_error_tripwires() {
        // Broken Entry Tripwire: Category/Domain mismatch
        let broken_entry = LexiconEntry::new(
            "broken_concept",
            ModalDomain::Physical,
            DomainCategory::TheologyReligion,
            SubDomain::BiblicalOntology,
            "An intentionally broken ontological entry for tripwire tests.",
            "Genesis 1:1",
            "Genesis 1:1",
            QuadState::Halt,
            "Refuse incoherence.",
        );

        let broken_report = broken_entry.validate();
        assert!(!broken_report.is_valid);
        assert!(!broken_report.has_valid_domain_hierarchy);
        assert!(!audit_lexicon_entry(&broken_entry));

        // Missing Witness Tripwire
        let empty_witness_entry = LexiconEntry::new(
            "darkness_test",
            ModalDomain::Pistic,
            DomainCategory::TheologyReligion,
            SubDomain::BiblicalOntology,
            "Absence of light.",
            "",
            "Genesis 1:2",
            QuadState::Await,
            "Expose errors.",
        );
        let witness_report = empty_witness_entry.validate();
        assert!(!witness_report.is_valid);
        assert!(!witness_report.has_dual_witness);

        // Telemetry Card output check
        let valid_entry = LexiconEntry::new(
            "justice",
            ModalDomain::Juridical,
            DomainCategory::CivilOrderJurisprudence,
            SubDomain::CivilJusticeRestitution,
            "Righteous civil judgment and restitution.",
            "Micah 6:8 - to do justly, and to love mercy",
            "Micah 6:8 - to do justly, to love mercy",
            QuadState::Proceed,
            "Do justice; defend the innocent and restore damage.",
        );
        let card = valid_entry.render_telemetry_card();
        assert!(card.contains("LEXICON ENTRY: JUSTICE"));
        assert!(card.contains("Coordinate"));
        assert!(card.contains("Resonance"));
        assert!(card.contains("Guidance"));

        // Deterministic JSON fragment check
        let json = valid_entry.to_json_fragment();
        assert!(json.starts_with("{\"term\":\"justice\""));
        assert!(json.contains("\"domain\":\"Juridical\""));
        assert!(json.contains("\"category\":\"CivilOrderJurisprudence\""));
        assert!(json.contains("\"subdomain\":\"CivilJusticeRestitution\""));
        assert!(json.contains("\"state\":\"PROCEED\""));
    }

    // -------------------------------------------------------------------------
    // Test Suite 7: Canonical Baseline Self-Audit
    // -------------------------------------------------------------------------
    #[test]
    fn test_canonical_entry_baseline_audit() {
        let report = audit_canonical_entry_baseline();
        if !report.is_valid {
            panic!("Baseline entry audit errors: {:?}", report.errors);
        }
        assert!(report.is_valid);
        assert!(report.total_entries >= 25);
        assert!(report.total_concordances >= 20);
        assert!(report.errors.is_empty());
    }
}

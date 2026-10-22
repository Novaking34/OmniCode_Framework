// =============================================================================
// BLOCK 1: METADATA BLOCK
// ( Identity, Provenance, Invariant Boundary Contracts & Scriptural Anchors )
// =============================================================================
// @module        OmniCode AI Subsystem: Universal World Lexicon Table
// @file          table.rs
// @layer         Track 1 [Host Engine & Virtual Machine] -> src/ai/lexicon
// @version       1.0.0
// @epoch         2026.10
// @author        CreativeWorkzStudio LLC / ProfessorSeanEX
// @system        OmniCode AI Cortex & Living Kernel
// @repository    https://github.com/Novaking34/OmniCode_Framework
// @role          Ground Truth World Ontology Table & Triangulation Engine
// @invariants    Rank 1 (< >): Form precedes fullness. The World Ontology
//                Table unifies the 15 Sovereign Modal Spheres (domain.rs),
//                the Scriptural Concordance & Epistemology (entry.rs), and
//                the Decalogue Conscience & Covenant Bedrock (moral.rs).
// @witness       "Have not I written to thee excellent things in counsels and
//                 knowledge, that I might make thee know the certainty of the
//                 words of truth..." — Proverbs 22:20-21 (KJV)
//                "The entirety of your word is truth. All of your righteous
//                 ordinances endure forever." — Psalm 119:160 (WEB)
//                "To everything there is a season, and a time to every purpose
//                 under the heaven." — Ecclesiastes 3:1 (KJV)
//
// THE 4-POINT TRIANGULATION COGNITIVE CYCLE:
//   The Table is NOT a passive dictionary or static word glossary.
//   It is the Triangulation Solver and Cognitive State Space of the AI Cortex:
//
//   1 Point  : Raw prompt, token, or unmapped phenomenon (The Origin).
//   3 Points : TRIANGULATION TRIAD:
//              • Point 1: Modal Domain (The 15 Sovereign Law-Spheres)
//              • Point 2: Scripture Entry (The Immutable Logos & Concordance)
//              • Point 3: Covenant Conscience (The Decalogue / Intent/Impact)
//   4 Points : THE SOLID BLOCK (Layer 7 Operation & Balanced Ternary State):
//              Free causal agency: {+1: Proceed, 0: Await, -1: Rollback, NULL}.
//   The Circle: THE UNIFIED MATRIX (The Master Table):
//              Once resolved, the 4-point block closes into a solid new
//              reference point in memory (.live.dat), becoming an anchor
//              for higher-order recursive cognitive growth!
// =============================================================================

// =============================================================================
// BLOCK 2: OPENING BLOCK
// ( Scaffolding, Descriptors, Types & Triangulation Coordinates )
// =============================================================================

use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::core::ternary::QuadState;
use super::domain::{DomainCategory, ModalDomain, SubDomain};
use super::entry::{EntryStore, LexiconEntry};
use super::moral::{
    CommandmentId, HeartIntent, RealityImpact, COVENANT_RULES,
};

/// Unified 4-point ontological coordinate resolving any concept or prompt:
///   Point 1: Domain Sphere (The 15 Sovereign Modal Law-Spheres)
///   Point 2: Lexical Truth (The Word, Definition & Scriptural Witnesses)
///   Point 3: Covenant Conscience (The Decalogue Invariant, Intent vs Impact)
///   Point 4: Action & State (Layer 7 Operation, Balanced Ternary QuadState)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TermTriangulation {
    /// Keyword identifier or concept term.
    pub term: String,
    /// Point 1: Governing Sovereign Modal Domain (1 of 15).
    pub modal_domain: ModalDomain,
    /// Point 1: Regional category.
    pub category: DomainCategory,
    /// Point 1: Contextual subdomain / operational theater.
    pub subdomain: SubDomain,
    /// Point 2: Concise definitional essence.
    pub definition: String,
    /// Point 2: Anchor 0 scriptural witness (KJV).
    pub kjv_passage: String,
    /// Point 2: Extension +1 scriptural witness (WEB).
    pub web_passage: String,
    /// Point 2: Cryptographic concordance reference count.
    pub concordance_count: usize,
    /// Point 3: Specific Decalogue Commandment engaged, if applicable.
    pub commandment: Option<CommandmentId>,
    /// Point 3: Covenant law identifier.
    pub law_engaged: Option<&'static str>,
    /// Point 3: Inward CPI heart intent polarity.
    pub cpi_intent: HeartIntent,
    /// Point 3: Outward SI reality impact boundary.
    pub si_impact: RealityImpact,
    /// Point 3: Restorative guidance or window of redemption.
    pub guidance: String,
    /// Point 4: Action state resonance (+1: Proceed, 0: Await, -1: Rollback).
    pub quad_state: QuadState,
    /// Point 2: Primary scriptural citation string (e.g. "Genesis 1:3").
    pub primary_citation: Option<String>,
}

impl TermTriangulation {
    /// Resolves canonical Track 4 verse coordinate and prose text if available.
    pub fn resolve_scripture_landmark(
        &self,
        engine: &crate::core::scripture_coord::ScriptureCoordinateEngine,
    ) -> Option<crate::core::scripture_coord::ScriptureVerse> {
        let cit = self.primary_citation.as_ref()?;
        engine.lookup_citation(cit)
    }
}

/// Diagnostic report generated when auditing the complete lexicon table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexiconTableAuditReport {
    pub total_entries: usize,
    pub unique_terms: usize,
    pub modal_domains_covered: usize,
    pub subdomains_covered: usize,
    pub decalogue_commandments_covered: usize,
    pub is_valid: bool,
    pub errors: Vec<String>,
}

// =============================================================================
// BLOCK 3: BODY BLOCK
// ( LexiconTable Engine, Multi-Layer Storage & Triangulation Solver )
// =============================================================================

/// Dynamic World Ontology Table and Triangulation Engine.
#[derive(Debug, Clone)]
pub struct LexiconTable {
    /// In-memory corpus of validated lexicon entries.
    pub entries: Vec<LexiconEntry>,
    /// Entries dynamically appended in the live session journal.
    pub live_entries: Vec<LexiconEntry>,
    /// Discovered filesystem data root directory if present.
    pub data_dir: Option<PathBuf>,
}

impl Default for LexiconTable {
    fn default() -> Self {
        Self::load_from_static()
    }
}

impl LexiconTable {
    /// Loads fallback lexicon table from compiled static data baseline.
    pub fn load_from_static() -> Self {
        let store = EntryStore::load_from_static();
        Self {
            entries: store.entries,
            live_entries: Vec::new(),
            data_dir: None,
        }
    }

    /// Automatically discovers data files and loads the complete live table.
    pub fn load() -> Self {
        if let Some(dir) = Self::discover_data_dir() {
            if let Ok(table) = Self::load_from_directory(&dir) {
                return table;
            }
        }
        Self::load_from_static()
    }

    /// Loads base entries and overlays live journal from a target directory.
    pub fn load_from_directory<P: AsRef<Path>>(dir: P) -> io::Result<Self> {
        let store = EntryStore::load_from_directory(&dir)?;
        Ok(Self {
            entries: store.entries,
            live_entries: store.live_entries,
            data_dir: Some(dir.as_ref().to_path_buf()),
        })
    }

    /// Automatically discovers the lexicon data directory across environments.
    pub fn discover_data_dir() -> Option<PathBuf> {
        let candidates = [
            PathBuf::from("src/ai/lexicon/data"),
            PathBuf::from(concat!(
                "<TRACK 1> : {Host Engine & ",
                "Virtual Machine}/src/ai/lexicon/data",
            )),
            PathBuf::from(concat!(
                "/home/professorseanex/OmniCode/",
                "<TRACK 1> : {Host Engine & ",
                "Virtual Machine}/src/ai/lexicon/data",
            )),
        ];
        for c in &candidates {
            if c.exists() && c.is_dir() {
                return Some(c.clone());
            }
        }
        None
    }

    /// Appends a new validated entry to the live journal in the filesystem.
    pub fn append_entry_to_live(
        &mut self,
        entry: LexiconEntry,
    ) -> io::Result<()> {
        let report = entry.validate();
        if !report.is_valid {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Validation failed for '{}'", entry.term),
            ));
        }

        if let Some(ref dir) = self.data_dir {
            let path = dir.join("entry.live.dat");
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)?;

            let state_str = entry.quad_state_resonance.macro_op();
            let mut concords = Vec::new();
            for c in &entry.concordance_refs {
                concords.push(format!(
                    "{:?}:{}:{}:{}-{}:{}",
                    c.translation, c.book, c.chapter,
                    c.verse_start, c.verse_end, c.hash_signature
                ));
            }
            let line = format!(
                "ENTRY|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
                entry.term,
                entry.domain.as_str(),
                entry.category.as_str(),
                entry.subdomain.as_str(),
                state_str,
                entry.definition.replace('\n', " "),
                entry.kjv_passage.replace('\n', " "),
                entry.web_passage.replace('\n', " "),
                entry.moral_guidance.replace('\n', " "),
                concords.join(";")
            );
            file.write_all(line.as_bytes())?;
        }

        if !self.entries.iter().any(|e| e.term == entry.term) {
            self.live_entries.push(entry.clone());
            self.entries.push(entry);
        }
        Ok(())
    }

    /// Looks up an entry by term keyword (case-insensitive).
    pub fn lookup(&self, term: &str) -> Option<&LexiconEntry> {
        let clean = term.trim().to_lowercase();
        self.entries.iter().find(|e| e.term == clean)
    }

    /// Filters entries belonging to a specific Sovereign Modal Domain.
    pub fn filter_by_domain(&self, domain: ModalDomain) -> Vec<&LexiconEntry> {
        self.entries.iter().filter(|e| e.domain == domain).collect()
    }

    /// Filters entries belonging to a specific SubDomain.
    pub fn filter_by_subdomain(
        &self,
        subdomain: SubDomain,
    ) -> Vec<&LexiconEntry> {
        self.entries
            .iter()
            .filter(|e| e.subdomain == subdomain)
            .collect()
    }

    /// Filters entries resonating with a specific QuadState.
    pub fn filter_by_quad_state(&self, state: QuadState) -> Vec<&LexiconEntry> {
        self.entries
            .iter()
            .filter(|e| e.quad_state_resonance == state)
            .collect()
    }

    /// Looks up an entry corresponding to a specific Decalogue Commandment.
    pub fn lookup_by_commandment(
        &self,
        cmd: CommandmentId,
    ) -> Option<&LexiconEntry> {
        let canon = commandment_canonical_term(cmd);
        self.lookup(canon)
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

    /// Solves the 4-point cognitive triangulation for a concept or prompt:
    ///   Point 1: Domain Sphere (The 15 Sovereign Modal Law-Spheres)
    ///   Point 2: Lexical Truth (The Word, Definition & Scriptural Witnesses)
    ///   Point 3: Covenant Conscience (The Decalogue & Intent vs Impact)
    ///   Point 4: Action & State (Layer 7 Operation, Balanced Ternary State)
    pub fn triangulate(&self, term: &str) -> Option<TermTriangulation> {
        let entry = self.lookup(term)?;
        let commandment = term_to_commandment(&entry.term);

        // Derive CPI Intent and SI Impact from quad_state_resonance
        let (cpi_intent, si_impact) = match entry.quad_state_resonance {
            QuadState::Proceed => {
                (HeartIntent::Positive, RealityImpact::Positive)
            }
            QuadState::Await => {
                (HeartIntent::Neutral, RealityImpact::Negative)
            }
            QuadState::Rollback => {
                (HeartIntent::Negative, RealityImpact::Negative)
            }
            QuadState::Halt => {
                (HeartIntent::Negative, RealityImpact::Mortal)
            }
        };

        let law_engaged = commandment.map(|cmd| match cmd {
            CommandmentId::FirstSovereignty => "Law_Sovereignty",
            CommandmentId::SecondGravenImages => "Law_Idolatry",
            CommandmentId::ThirdSacredName => "Law_SacredName",
            CommandmentId::FourthSabbathRest => "Law_Sabbath",
            CommandmentId::FifthHonorParents => "Law_HonorParents",
            CommandmentId::SixthSanctityOfLife => "Law_Life",
            CommandmentId::SeventhFidelityChastity => "Law_Chastity",
            CommandmentId::EighthPropertyStewardship => "Law_Property",
            CommandmentId::NinthLawOfTruth => "Law_Truth",
            CommandmentId::TenthContentmentAntiCovet => "Law_AntiCovet",
        });

        let primary_citation = entry
            .concordance_refs
            .first()
            .map(|c| c.format_citation());

        Some(TermTriangulation {
            term: entry.term.clone(),
            modal_domain: entry.domain,
            category: entry.category,
            subdomain: entry.subdomain,
            definition: entry.definition.clone(),
            kjv_passage: entry.kjv_passage.clone(),
            web_passage: entry.web_passage.clone(),
            concordance_count: entry.concordance_refs.len(),
            commandment,
            law_engaged,
            cpi_intent,
            si_impact,
            guidance: entry.moral_guidance.clone(),
            quad_state: entry.quad_state_resonance,
            primary_citation,
        })
    }

    /// Solves the 4-point cognitive triangulation for a prompt by checking
    /// multi-word phrases first, then falling back to individual token search.
    pub fn triangulate_phrase(
        &self,
        prompt: &str,
    ) -> Option<TermTriangulation> {
        let clean = prompt.to_lowercase();
        // 1. Check multi-word entries first
        for entry in self.entries.iter().filter(|e| {
            e.term.contains(' ') || e.term.contains('_')
        }) {
            let spaced = entry.term.replace('_', " ");
            if clean.contains(&spaced) || clean.contains(&entry.term) {
                return self.triangulate(&entry.term);
            }
        }
        // 2. Check single word tokens
        for word in clean
            .split(|c: char| !c.is_alphanumeric() && c != '_' && c != '-')
            .filter(|w| !w.is_empty())
        {
            if let Some(tri) = self.triangulate(word) {
                return Some(tri);
            }
        }
        None
    }

    /// Synthesizes a missing entry from a target SubDomain (Category down)
    /// and governing Covenant moral rule without statistical hallucination.
    pub fn triangulate_missing_entry(
        &self,
        target_subdomain: SubDomain,
        cmd: CommandmentId,
        term: &str,
        definition: &str,
    ) -> LexiconEntry {
        let rule = &COVENANT_RULES[(cmd.number() - 1) as usize];
        let sub_desc = target_subdomain.descriptor();

        LexiconEntry::new(
            term,
            sub_desc.domain,
            sub_desc.category,
            target_subdomain,
            definition,
            rule.kjv_passage,
            rule.web_passage,
            rule.state,
            rule.moral_guidance,
        )
    }

    /// Projects an existing invariant entry into an expanded/new operational
    /// theater, looking from the Category down (Layer 2 Category & Layer 3
    /// SubDomain).
    ///
    /// The 15 Sovereign Modal Aspects are FIXED, immutable primordial law-
    /// spheres (Layer 1). They never shift, mutate, or expand. Any "new
    /// domain" encountered in reality or computation is an expansion from
    /// the Category down (Layer 2..7). The governing ModalDomain is locked
    /// to the immutable parent modal sphere of the target category.
    pub fn project_new_domain(
        &self,
        term: &str,
        target_subdomain: SubDomain,
    ) -> Option<TermTriangulation> {
        let mut tri = self.triangulate(term)?;
        let sub_desc = target_subdomain.descriptor();
        tri.subdomain = target_subdomain;
        tri.category = sub_desc.category;
        tri.modal_domain = sub_desc.domain;
        Some(tri)
    }

    /// Projects an existing invariant entry into an explicit category and
    /// subdomain, verifying ontological alignment and locking under the
    /// category's fixed parent modal aspect.
    pub fn project_into_category(
        &self,
        term: &str,
        target_category: DomainCategory,
        target_subdomain: SubDomain,
    ) -> Option<TermTriangulation> {
        let sub_desc = target_subdomain.descriptor();
        if sub_desc.category != target_category {
            return None;
        }
        let mut tri = self.triangulate(term)?;
        tri.category = target_category;
        tri.subdomain = target_subdomain;
        tri.modal_domain = target_category.parent_domain();
        Some(tri)
    }

    /// Audits the complete lexicon table against invariant requirements.
    pub fn audit(&self) -> LexiconTableAuditReport {
        audit_lexicon_table(&self.entries)
    }

    /// Renders an ASCII dashboard summarizing the lexicon table distribution.
    pub fn render_telemetry_summary(&self) -> String {
        render_table_telemetry_summary(&self.entries)
    }

    /// Serializes table summary metadata to a compact JSON fragment.
    pub fn to_summary_json_fragment(&self) -> String {
        to_summary_json_fragment(&self.entries)
    }

    /// Inserts or updates an entry within the active table.
    pub fn insert_entry(&mut self, entry: LexiconEntry) {
        let pos = self.entries.iter().position(|e| e.term == entry.term);
        if let Some(idx) = pos {
            self.entries[idx] = entry;
        } else {
            self.entries.push(entry);
        }
    }
}

// =============================================================================
// BLOCK 4: AUDIT & FUNCTIONAL FACADE BLOCK
// ( Diagnostic Triad Verification & Standalone Query Helpers )
// =============================================================================

/// Builds the comprehensive set of biblical lexicon entries.
pub fn build_lexicon_entries() -> Vec<LexiconEntry> {
    LexiconTable::load().entries
}

/// Looks up an entry by term keyword (case-insensitive).
pub fn lookup_by_term<'a>(
    entries: &'a [LexiconEntry],
    term: &str,
) -> Option<&'a LexiconEntry> {
    let clean = term.trim().to_lowercase();
    entries.iter().find(|e| e.term == clean)
}

/// Filters entries belonging to a specific Sovereign Modal Domain.
pub fn filter_by_domain<'a>(
    entries: &'a [LexiconEntry],
    domain: ModalDomain,
) -> Vec<&'a LexiconEntry> {
    entries.iter().filter(|e| e.domain == domain).collect()
}

/// Filters entries belonging to a specific Sovereign Modal Domain (alias).
pub fn filter_by_modal_domain<'a>(
    entries: &'a [LexiconEntry],
    domain: ModalDomain,
) -> Vec<&'a LexiconEntry> {
    filter_by_domain(entries, domain)
}

/// Filters entries belonging to a specific SubDomain.
pub fn filter_by_subdomain<'a>(
    entries: &'a [LexiconEntry],
    subdomain: SubDomain,
) -> Vec<&'a LexiconEntry> {
    entries.iter().filter(|e| e.subdomain == subdomain).collect()
}

/// Filters entries resonating with a specific QuadState.
pub fn filter_by_quad_state<'a>(
    entries: &'a [LexiconEntry],
    state: QuadState,
) -> Vec<&'a LexiconEntry> {
    entries
        .iter()
        .filter(|e| e.quad_state_resonance == state)
        .collect()
}

/// Extends an in-memory lexicon vector with a newly validated entry.
pub fn extend_lexicon_with_entry(
    entries: &mut Vec<LexiconEntry>,
    entry: LexiconEntry,
) -> Result<(), String> {
    let report = entry.validate();
    if !report.is_valid {
        return Err(format!(
            "Cannot register entry '{}': validation failed: {:?}",
            entry.term, report.errors
        ));
    }
    if entries.iter().any(|e| e.term == entry.term) {
        return Err(format!(
            "Cannot register entry '{}': duplicate term already exists",
            entry.term
        ));
    }
    entries.push(entry);
    Ok(())
}

/// Resolves whether a term corresponds to a Decalogue commandment.
pub fn term_to_commandment(term: &str) -> Option<CommandmentId> {
    match term.trim().to_lowercase().as_str() {
        "humility" => Some(CommandmentId::FirstSovereignty),
        "graven_image" => Some(CommandmentId::SecondGravenImages),
        "reverence" => Some(CommandmentId::ThirdSacredName),
        "sabbath" => Some(CommandmentId::FourthSabbathRest),
        "honor" => Some(CommandmentId::FifthHonorParents),
        "murder" => Some(CommandmentId::SixthSanctityOfLife),
        "fidelity" => Some(CommandmentId::SeventhFidelityChastity),
        "theft" => Some(CommandmentId::EighthPropertyStewardship),
        "deceit" => Some(CommandmentId::NinthLawOfTruth),
        "contentment" => Some(CommandmentId::TenthContentmentAntiCovet),
        _ => None,
    }
}

/// Canonical representative term for each Decalogue commandment.
pub fn commandment_canonical_term(cmd: CommandmentId) -> &'static str {
    match cmd {
        CommandmentId::FirstSovereignty => "humility",
        CommandmentId::SecondGravenImages => "graven_image",
        CommandmentId::ThirdSacredName => "reverence",
        CommandmentId::FourthSabbathRest => "sabbath",
        CommandmentId::FifthHonorParents => "honor",
        CommandmentId::SixthSanctityOfLife => "murder",
        CommandmentId::SeventhFidelityChastity => "fidelity",
        CommandmentId::EighthPropertyStewardship => "theft",
        CommandmentId::NinthLawOfTruth => "deceit",
        CommandmentId::TenthContentmentAntiCovet => "contentment",
    }
}

/// Audits the entire lexicon table against invariant requirements:
/// - Verifies that terms are unique (zero duplicates).
/// - Verifies representation across the Sovereign Modal Domains.
/// - Verifies that all 18 SubDomains are represented.
/// - Verifies that all 10 Decalogue commandments have matching entries.
/// - Verifies parent-child hierarchy consistency with `domain.rs`.
/// - Verifies that every concordance reference has a valid range and SHA-256.
pub fn audit_lexicon_table(
    entries: &[LexiconEntry],
) -> LexiconTableAuditReport {
    let mut errors = Vec::new();
    let mut terms_seen = Vec::new();
    let mut domains_seen = Vec::new();
    let mut subdomains_seen = Vec::new();

    for entry in entries {
        // 1. Uniqueness check
        if terms_seen.contains(&entry.term) {
            errors.push(format!("Duplicate term detected: '{}'", entry.term));
        } else {
            terms_seen.push(entry.term.clone());
        }

        // 2. Track domain and subdomain coverage
        if !domains_seen.contains(&entry.domain) {
            domains_seen.push(entry.domain);
        }
        if !subdomains_seen.contains(&entry.subdomain) {
            subdomains_seen.push(entry.subdomain);
        }

        // 3. Entry validation
        let entry_report = entry.validate();
        if !entry_report.is_valid {
            errors.push(format!(
                "Validation failed for '{}': {:?}",
                entry.term, entry_report.errors
            ));
        }

        // 4. Contract verification
        if !verify_table_entry_contract(entry) {
            errors.push(format!(
                "Structural contract failed for entry '{}'",
                entry.term
            ));
        }
    }

    // 5. Decalogue coverage check
    let decalogue_terms = [
        "humility",
        "graven_image",
        "reverence",
        "sabbath",
        "honor",
        "murder",
        "fidelity",
        "theft",
        "deceit",
        "contentment",
    ];
    let mut decalogue_covered = 0;
    for dt in &decalogue_terms {
        if terms_seen.iter().any(|t| t == dt) {
            decalogue_covered += 1;
        } else {
            errors.push(format!(
                "Decalogue commandment term '{}' missing from table",
                dt
            ));
        }
    }

    if domains_seen.len() < 6 {
        errors.push(format!(
            "Expected at least 6 core Modal Domains represented, found {}",
            domains_seen.len()
        ));
    }

    if subdomains_seen.len() < 18 {
        errors.push(format!(
            "Expected all 18 SubDomains, found {}",
            subdomains_seen.len()
        ));
    }

    let is_valid = errors.is_empty();
    LexiconTableAuditReport {
        total_entries: entries.len(),
        unique_terms: terms_seen.len(),
        modal_domains_covered: domains_seen.len(),
        subdomains_covered: subdomains_seen.len(),
        decalogue_commandments_covered: decalogue_covered,
        is_valid,
        errors,
    }
}

/// Verifies that a lexicon entry adheres to structural contracts.
pub fn verify_table_entry_contract(entry: &LexiconEntry) -> bool {
    let non_empty = !entry.term.is_empty()
        && !entry.definition.is_empty()
        && !entry.kjv_passage.is_empty()
        && !entry.web_passage.is_empty()
        && !entry.moral_guidance.is_empty()
        && !entry.concordance_refs.is_empty();

    let hierarchy_valid = entry.category.parent_domain() == entry.domain
        && entry.subdomain.descriptor().category == entry.category;

    non_empty && hierarchy_valid
}

/// Renders an ASCII dashboard summarizing the lexicon table distribution.
pub fn render_table_telemetry_summary(entries: &[LexiconEntry]) -> String {
    let mut out = String::new();
    let border =
        "+---------------------------------------------------------+\n";
    out.push_str(border);
    out.push_str(
        "| UNIVERSAL WORLD LEXICON: TELEMETRY SUMMARY              |\n",
    );
    out.push_str(border);
    let total_str = format!("{} Entries", entries.len());
    out.push_str(&format!("| Total Terms : {:<41} |\n", total_str));

    let proceed_count = entries
        .iter()
        .filter(|e| e.quad_state_resonance == QuadState::Proceed)
        .count();
    let await_count = entries
        .iter()
        .filter(|e| e.quad_state_resonance == QuadState::Await)
        .count();
    let rollback_count = entries
        .iter()
        .filter(|e| e.quad_state_resonance == QuadState::Rollback)
        .count();
    let halt_count = entries
        .iter()
        .filter(|e| e.quad_state_resonance == QuadState::Halt)
        .count();

    let res_str = format!(
        "+1:{} | 0:{} | -1:{} | NULL:{}",
        proceed_count, await_count, rollback_count, halt_count
    );
    out.push_str(&format!("| Resonance   : {:<41} |\n", res_str));

    let pist_dom = entries
        .iter()
        .filter(|e| e.domain == ModalDomain::Pistic)
        .count();
    let phys_dom = entries
        .iter()
        .filter(|e| e.domain == ModalDomain::Physical)
        .count();
    let bio_dom = entries
        .iter()
        .filter(|e| e.domain == ModalDomain::BioticOrganic)
        .count();
    let jur_dom = entries
        .iter()
        .filter(|e| e.domain == ModalDomain::Juridical)
        .count();
    let eth_dom = entries
        .iter()
        .filter(|e| e.domain == ModalDomain::Ethical)
        .count();
    let soc_dom = entries
        .iter()
        .filter(|e| e.domain == ModalDomain::Social)
        .count();

    out.push_str(&format!(
        "| Spheres 1-3 : Pist:{} | Phys:{} | Bio:{}           |\n",
        pist_dom, phys_dom, bio_dom
    ));
    out.push_str(&format!(
        "| Spheres 4-6 : Jur:{}  | Eth:{}  | Soc:{}           |\n",
        jur_dom, eth_dom, soc_dom
    ));
    out.push_str(border);
    out
}

/// Serializes table summary metadata to a compact JSON fragment.
pub fn to_summary_json_fragment(entries: &[LexiconEntry]) -> String {
    format!(
        "{{\"total_entries\":{},\"proceed_count\":{},\
\"await_count\":{},\"rollback_count\":{},\"halt_count\":{}}}",
        entries.len(),
        entries
            .iter()
            .filter(|e| e.quad_state_resonance == QuadState::Proceed)
            .count(),
        entries
            .iter()
            .filter(|e| e.quad_state_resonance == QuadState::Await)
            .count(),
        entries
            .iter()
            .filter(|e| e.quad_state_resonance == QuadState::Rollback)
            .count(),
        entries
            .iter()
            .filter(|e| e.quad_state_resonance == QuadState::Halt)
            .count()
    )
}

// =============================================================================
// BLOCK 5: CLOSING BLOCK
// ( Multi-Tier Functional Unit Tests & Bootstrap Invariant Verification )
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::lexicon::entry::ConcordanceRef;

    // -------------------------------------------------------------------------
    // Test Suite 1: Table Completeness & Lexical Uniqueness
    // -------------------------------------------------------------------------
    #[test]
    fn test_table_completeness_and_uniqueness() {
        let table = LexiconTable::load_from_static();
        assert!(table.entries.len() >= 45);

        let report = table.audit();
        assert!(report.is_valid, "Audit failed: {:?}", report.errors);
        assert_eq!(report.unique_terms, table.entries.len());
        assert!(report.modal_domains_covered >= 6);
        assert_eq!(report.subdomains_covered, 18);
        assert_eq!(report.decalogue_commandments_covered, 10);
    }

    // -------------------------------------------------------------------------
    // Test Suite 2: Full Decalogue & 10 Commandments Alignment
    // -------------------------------------------------------------------------
    #[test]
    fn test_all_ten_commandments_present_in_table() {
        let table = LexiconTable::load_from_static();

        // 1. FirstSovereignty -> humility (Await)
        let c1 = table
            .lookup_by_commandment(CommandmentId::FirstSovereignty)
            .expect("Cmd 1 humility");
        assert_eq!(c1.quad_state_resonance, QuadState::Await);

        // 2. SecondGravenImages -> graven_image (Rollback)
        let c2 = table
            .lookup_by_commandment(CommandmentId::SecondGravenImages)
            .expect("Cmd 2 graven_image");
        assert_eq!(c2.quad_state_resonance, QuadState::Rollback);

        // 3. ThirdSacredName -> reverence (Rollback)
        let c3 = table
            .lookup_by_commandment(CommandmentId::ThirdSacredName)
            .expect("Cmd 3 reverence");
        assert_eq!(c3.quad_state_resonance, QuadState::Rollback);

        // 4. FourthSabbathRest -> sabbath (Await)
        let c4 = table
            .lookup_by_commandment(CommandmentId::FourthSabbathRest)
            .expect("Cmd 4 sabbath");
        assert_eq!(c4.quad_state_resonance, QuadState::Await);

        // 5. FifthHonorParents -> honor (Rollback)
        let c5 = table
            .lookup_by_commandment(CommandmentId::FifthHonorParents)
            .expect("Cmd 5 honor");
        assert_eq!(c5.quad_state_resonance, QuadState::Rollback);

        // 6. SixthSanctityOfLife -> murder (Halt)
        let c6 = table
            .lookup_by_commandment(CommandmentId::SixthSanctityOfLife)
            .expect("Cmd 6 murder");
        assert_eq!(c6.quad_state_resonance, QuadState::Halt);

        // 7. SeventhFidelityChastity -> fidelity (Rollback)
        let c7 = table
            .lookup_by_commandment(CommandmentId::SeventhFidelityChastity)
            .expect("Cmd 7 fidelity");
        assert_eq!(c7.quad_state_resonance, QuadState::Rollback);

        // 8. EighthPropertyStewardship -> theft (Rollback)
        let c8 = table
            .lookup_by_commandment(CommandmentId::EighthPropertyStewardship)
            .expect("Cmd 8 theft");
        assert_eq!(c8.quad_state_resonance, QuadState::Rollback);

        // 9. NinthLawOfTruth -> deceit (Rollback)
        let c9 = table
            .lookup_by_commandment(CommandmentId::NinthLawOfTruth)
            .expect("Cmd 9 deceit");
        assert_eq!(c9.quad_state_resonance, QuadState::Rollback);

        // 10. TenthContentmentAntiCovet -> contentment (Rollback)
        let c10 = table
            .lookup_by_commandment(CommandmentId::TenthContentmentAntiCovet)
            .expect("Cmd 10 contentment");
        assert_eq!(c10.quad_state_resonance, QuadState::Rollback);
    }

    // -------------------------------------------------------------------------
    // Test Suite 3: Sovereign Modal Domains & 18 SubDomains Coverage
    // -------------------------------------------------------------------------
    #[test]
    fn test_all_six_fundamental_domains_covered() {
        let table = LexiconTable::load_from_static();
        let core_domains = [
            ModalDomain::Pistic,
            ModalDomain::Physical,
            ModalDomain::BioticOrganic,
            ModalDomain::Juridical,
            ModalDomain::HistoricalFormative,
            ModalDomain::Lingual,
        ];

        for domain in &core_domains {
            let filtered = table.filter_by_domain(*domain);
            assert!(
                !filtered.is_empty(),
                "Domain {:?} has zero entries in table",
                domain
            );
        }
    }

    #[test]
    fn test_all_eighteen_subdomains_covered() {
        let table = LexiconTable::load_from_static();
        let all_subdomains = [
            SubDomain::BiblicalOntology,
            SubDomain::CovenantLaw,
            SubDomain::DeontologicalInvariants,
            SubDomain::TernaryLogicVerification,
            SubDomain::ClassicalMechanics,
            SubDomain::CelestialChronometry,
            SubDomain::HydrologicalCycle,
            SubDomain::PhysiologicalSystems,
            SubDomain::AgriculturalHusbandry,
            SubDomain::EcologicalEquilibrium,
            SubDomain::StatutoryConstitutionalLaw,
            SubDomain::CivilJusticeRestitution,
            SubDomain::MarketStewardship,
            SubDomain::GenerationalInheritance,
            SubDomain::MemoryMappedSilicon,
            SubDomain::ProcessSchedulingLifecycle,
            SubDomain::AttractorDynamicsConscience,
            SubDomain::DirectionalSyntaxSvo,
        ];

        for sub in &all_subdomains {
            let filtered = table.filter_by_subdomain(*sub);
            assert!(
                !filtered.is_empty(),
                "SubDomain {:?} has zero entries in table",
                sub
            );
        }
    }

    #[test]
    fn test_parent_child_hierarchy_invariants() {
        let table = LexiconTable::load_from_static();
        for entry in &table.entries {
            assert!(verify_table_entry_contract(entry));
            assert_eq!(entry.category.parent_domain(), entry.domain);
            assert_eq!(entry.subdomain.descriptor().category, entry.category);
        }
    }

    // -------------------------------------------------------------------------
    // Test Suite 4: Concordance Coordinates, Vault Paths & Hashes
    // -------------------------------------------------------------------------
    #[test]
    fn test_concordance_vault_paths_and_hashes() {
        let table = LexiconTable::load_from_static();
        for entry in &table.entries {
            assert!(!entry.concordance_refs.is_empty());
            for coord in &entry.concordance_refs {
                assert!(coord.is_valid_range());
                assert!(coord.has_valid_hash_format());
                let vault_path = coord.vault_relative_path();
                assert!(vault_path.contains("immutable_documentation"));
            }
        }
    }

    // -------------------------------------------------------------------------
    // Test Suite 5: 4-Point Triangulation Cognitive Solver
    // -------------------------------------------------------------------------
    #[test]
    fn test_triangulation_engine_full_coordinate() {
        let table = LexiconTable::load_from_static();

        // 1. Triangulate murder (Ethical domain, Commandment 6, Halt)
        let murder_tri = table
            .triangulate("murder")
            .expect("murder coordinate");
        assert_eq!(murder_tri.modal_domain, ModalDomain::Ethical);
        assert_eq!(
            murder_tri.commandment,
            Some(CommandmentId::SixthSanctityOfLife)
        );
        assert_eq!(murder_tri.quad_state, QuadState::Halt);
        assert_eq!(murder_tri.si_impact, RealityImpact::Mortal);

        // 2. Triangulate humility (Pistic domain, Commandment 1, Await)
        let humil_tri = table
            .triangulate("humility")
            .expect("humility coordinate");
        assert_eq!(humil_tri.modal_domain, ModalDomain::Pistic);
        assert_eq!(
            humil_tri.commandment,
            Some(CommandmentId::FirstSovereignty)
        );
        assert_eq!(humil_tri.quad_state, QuadState::Await);

        // 3. Triangulate light (Pistic domain, Proceed)
        let light_tri = table.triangulate("light").expect("light coordinate");
        assert_eq!(light_tri.quad_state, QuadState::Proceed);
        assert_eq!(light_tri.cpi_intent, HeartIntent::Positive);
    }

    #[test]
    fn test_triangulate_missing_entry_synthesis() {
        let table = LexiconTable::load_from_static();
        let synth = table.triangulate_missing_entry(
            SubDomain::MemoryMappedSilicon,
            CommandmentId::EighthPropertyStewardship,
            "zero_day_buffer_theft",
            "Expropriation of memory mapped register space without authority.",
        );
        assert_eq!(synth.term, "zero_day_buffer_theft");
        assert_eq!(synth.domain, ModalDomain::Physical);
        assert_eq!(
            synth.category,
            DomainCategory::ComputerArchitectureSilicon
        );
        assert_eq!(synth.subdomain, SubDomain::MemoryMappedSilicon);
        assert_eq!(synth.quad_state_resonance, QuadState::Rollback);
    }

    #[test]
    fn test_project_new_domain_extrapolation() {
        let table = LexiconTable::load_from_static();
        let projected = table
            .project_new_domain(
                "truth",
                SubDomain::ProcessSchedulingLifecycle,
            )
            .expect("truth projected to runtime engine");
        assert_eq!(
            projected.modal_domain,
            ModalDomain::HistoricalFormative
        );
        assert_eq!(
            projected.category,
            DomainCategory::OperatingSystemsRuntime
        );
        assert_eq!(
            projected.subdomain,
            SubDomain::ProcessSchedulingLifecycle
        );
        assert_eq!(projected.quad_state, QuadState::Proceed);

        let direct_cat = table.project_into_category(
            "truth",
            DomainCategory::OperatingSystemsRuntime,
            SubDomain::ProcessSchedulingLifecycle,
        );
        assert!(direct_cat.is_some());
    }

    #[test]
    fn test_table_query_filtering_and_extension() {
        let mut table = LexiconTable::load_from_static();
        let initial_count = table.entries.len();
        let halts = table.filter_by_quad_state(QuadState::Halt);
        assert!(!halts.is_empty());
        assert!(halts.iter().any(|e| e.term == "murder"));

        let new_entry = LexiconEntry::new(
            "grace_unmerited",
            ModalDomain::Pistic,
            DomainCategory::TheologyReligion,
            SubDomain::BiblicalOntology,
            "Unmerited divine favor and restorative power.",
            "Ephesians 2:8 - 'For by grace are ye saved...'",
            "Ephesians 2:8 - 'by grace you have been saved...'",
            QuadState::Proceed,
            "Extend restorative grace in error recovery.",
        ).with_concordances(vec![
            ConcordanceRef::nkjv(
                "Ephesians", 2, 8, 9, "sha256:grace_Eph_2_8_nkjv"
            ),
        ]);

        let res = extend_lexicon_with_entry(&mut table.entries, new_entry);
        assert!(res.is_ok());
        assert_eq!(table.entries.len(), initial_count + 1);

        // Duplicate registration should fail
        let dup = table.entries[0].clone();
        let dup_res = extend_lexicon_with_entry(&mut table.entries, dup);
        assert!(dup_res.is_err());
    }

    // -------------------------------------------------------------------------
    // Test Suite 6: Telemetry Summary Card, JSON Serialization & Audits
    // -------------------------------------------------------------------------
    #[test]
    fn test_telemetry_summary_rendering() {
        let table = LexiconTable::load_from_static();
        let card = table.render_telemetry_summary();
        assert!(card.contains("UNIVERSAL WORLD LEXICON: TELEMETRY SUMMARY"));
        assert!(card.contains("Entries"));
        assert!(card.contains("Spheres 1-3"));
        assert!(card.contains("Spheres 4-6"));
    }

    #[test]
    fn test_to_summary_json_fragment() {
        let table = LexiconTable::load_from_static();
        let json = table.to_summary_json_fragment();
        assert!(json.contains("\"total_entries\":"));
        assert!(json.contains("\"proceed_count\":"));
        assert!(json.contains("\"halt_count\":"));
    }

    #[test]
    fn test_audit_table_tripwire_on_duplicate() {
        let mut table = LexiconTable::load_from_static();
        table.entries.push(table.entries[0].clone()); // Inject duplicate
        let report = table.audit();
        assert!(!report.is_valid);
        assert!(report.errors.iter().any(|e| e.contains("Duplicate")));
    }
}

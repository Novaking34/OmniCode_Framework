// =============================================================================
// BLOCK 1: METADATA BLOCK
// ( Identity, Provenance, Invariant Boundary Contracts & Scriptural Anchors )
// =============================================================================
// @module        OmniCode AI Subsystem: Universal World Lexicon Facade
// @file          mod.rs
// @layer         Track 1 [Host Engine & Virtual Machine] -> src/ai/lexicon
// @version       1.0.0
// @epoch         2026.10
// @author        CreativeWorkzStudio LLC / ProfessorSeanEX
// @system        OmniCode AI Cortex & Living Kernel
// @repository    https://github.com/Novaking34/OmniCode_Framework
// @role          Central Lexicon Subsystem Facade & Moral Ontology Engine
// @invariants    Rank 1 (< >): Form precedes fullness. Grounded in the
//                15 Sovereign Modal Domains, 7-Layer Morphological Descent,
//                5-Fold Scripture Witness Standard, and Decalogue Invariants.
//                All term projections operate strictly from Category down.
// @witness       "The words of the LORD are pure words: as silver tried in a
//                 furnace of earth, purified seven times." — Psalm 12:6 (KJV)
// =============================================================================

// =============================================================================
// BLOCK 2: OPENING BLOCK
// ( Sub-module Declarations, Re-exports, Types & Container Declarations )
// =============================================================================

use std::io;
use std::path::Path;

use crate::core::ternary::QuadState;

pub mod domain;
pub mod entry;
pub mod moral;
pub mod table;

// Re-export Pillar I: Domain Sphere & 7-Layer Morphological Descent
pub use domain::{
    audit_domain_hierarchy, verify_subdomain_descriptor_contract,
    ConceptRecord, DomainAuditReport, DomainCategory,
    DomainClassification, DomainStore, DomainValidationReport, EntityRecord,
    FundamentalDomain, FundamentalDomainDescriptor, ModalDomain,
    ModalDomainDescriptor, OperationRecord, SubCategoryRecord, SubDomain,
    SubDomainDescriptor, SubDomainRecord, FUNDAMENTAL_DOMAIN_TABLE,
    MODAL_DOMAIN_TABLE, SUB_DOMAIN_TABLE,
};

// Re-export Pillar II: Lexical Logos & 5-Fold Scripture Witness Standard
pub use entry::{
    audit_lexicon_entry, verify_vault_reference_contract, ConcordanceRef,
    EntryStore, EntryValidationReport, LexiconEntry, ScriptureTranslation,
    TranslationSpec, TRANSLATION_SPEC_TABLE,
};

// Re-export Pillar III: Moral Invariants & Intent-Impact Tensor
pub use moral::{
    audit_moral_covenant_table, deliberate_prompt,
    verify_covenant_rule_contract, CommandmentId, CovenantRuleDescriptor,
    DeviationKind, HeartIntent, IntentImpactMatrix, MoralAuditReport,
    MoralEvaluation, RealityImpact, COVENANT_RULES,
};

// Re-export The Master Table & 4-Point Geometric Triangulation Engine
pub use table::{
    audit_lexicon_table, build_lexicon_entries, commandment_canonical_term,
    extend_lexicon_with_entry, filter_by_domain, filter_by_modal_domain,
    filter_by_quad_state, filter_by_subdomain, lookup_by_term,
    render_table_telemetry_summary, term_to_commandment,
    to_summary_json_fragment, verify_table_entry_contract, LexiconTable,
    LexiconTableAuditReport, TermTriangulation,
};

/// Master audit report evaluating all Three Pillars and the Master Table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexiconSubsystemAuditReport {
    /// Total entries validated in the active table.
    pub total_entries: usize,
    /// Number of unique lexical terms registered.
    pub unique_terms: usize,
    /// Modal domains covered across active entries (out of 15).
    pub modal_domains_covered: usize,
    /// Contextual subdomains covered across active entries (out of 18).
    pub subdomains_covered: usize,
    /// Decalogue commandments represented (out of 10).
    pub decalogue_commandments_covered: usize,
    /// Whether Pillar I domain hierarchy passed structural audit.
    pub domain_audit_passed: bool,
    /// Whether Pillar III covenant rules passed structural audit.
    pub moral_audit_passed: bool,
    /// Overall validity across all subsystem boundaries.
    pub is_valid: bool,
    /// Consolidated error collection across all audit phases.
    pub errors: Vec<String>,
}

/// The unified Universal World Lexicon engine holding the complete world
/// ontology, 4-point geometric triangulation, and covenant moral conscience.
#[derive(Debug, Clone)]
pub struct BiblicalLexicon {
    /// Internal table engine managing entries, static baseline, and live
    /// session journal.
    table: LexiconTable,
}

// =============================================================================
// BLOCK 3: BODY BLOCK
// ( Core Subsystem Engine Implementation & Multi-Dimensional Capabilities )
// =============================================================================

impl BiblicalLexicon {
    /// Instantiates the comprehensive public domain biblical world lexicon,
    /// automatically discovering filesystem data sheets with live journal
    /// overlays and compile-time static fallback.
    pub fn new() -> Self {
        Self {
            table: LexiconTable::load(),
        }
    }

    /// Loads the lexicon in 100% offline mode using zero-IO compiled-in
    /// static data baselines.
    pub fn load_from_static() -> Self {
        Self {
            table: LexiconTable::load_from_static(),
        }
    }

    /// Loads the lexicon from an explicit directory containing data sheets.
    pub fn load_from_directory<P: AsRef<Path>>(dir: P) -> io::Result<Self> {
        let table = LexiconTable::load_from_directory(dir)?;
        Ok(Self { table })
    }

    /// Borrows the inner [`LexiconTable`] engine directly.
    pub fn table(&self) -> &LexiconTable {
        &self.table
    }

    /// Borrows the inner [`LexiconTable`] engine mutably.
    pub fn table_mut(&mut self) -> &mut LexiconTable {
        &mut self.table
    }

    /// Returns the discovered filesystem data directory, if present.
    pub fn data_dir(&self) -> Option<&Path> {
        self.table.data_dir.as_deref()
    }

    /// Returns the total number of validated entries in the active table.
    pub fn entry_count(&self) -> usize {
        self.table.entries.len()
    }

    /// Returns the number of dynamic live entries appended during this session.
    pub fn live_entry_count(&self) -> usize {
        self.table.live_entries.len()
    }

    /// Retrieves all entries currently stored in the lexicon.
    pub fn all_entries(&self) -> &[LexiconEntry] {
        &self.table.entries
    }

    /// Retrieves all entries currently stored in the lexicon (alias).
    pub fn entries(&self) -> &[LexiconEntry] {
        self.all_entries()
    }

    /// Looks up a specific concept by keyword (case-insensitive).
    pub fn lookup(&self, term: &str) -> Option<&LexiconEntry> {
        self.table.lookup(term)
    }

    /// Searches for entries matching a keyword in their term or definition.
    pub fn search(&self, query: &str) -> Vec<&LexiconEntry> {
        self.table.search(query)
    }

    /// Looks up an entry corresponding to a specific Decalogue Commandment.
    pub fn lookup_by_commandment(
        &self,
        cmd: CommandmentId,
    ) -> Option<&LexiconEntry> {
        self.table.lookup_by_commandment(cmd)
    }

    /// Filters entries belonging to a specific Sovereign Modal Domain.
    pub fn filter_by_domain(&self, domain: ModalDomain) -> Vec<&LexiconEntry> {
        self.table.filter_by_domain(domain)
    }

    /// Filters entries belonging to a specific SubDomain.
    pub fn filter_by_subdomain(
        &self,
        subdomain: SubDomain,
    ) -> Vec<&LexiconEntry> {
        self.table.filter_by_subdomain(subdomain)
    }

    /// Filters entries resonating with a specific QuadState.
    pub fn filter_by_quad_state(
        &self,
        state: QuadState,
    ) -> Vec<&LexiconEntry> {
        self.table.filter_by_quad_state(state)
    }

    /// Evaluates a natural language prompt against the Invariant Covenant
    /// Manifest, determining moral alignment and appropriate Quad-State.
    pub fn evaluate_moral_intent(&self, prompt: &str) -> MoralEvaluation {
        self.deliberate(prompt)
    }

    /// Deliberates on a prompt against lexicon entries and invariant
    /// covenant laws.
    pub fn deliberate(&self, prompt: &str) -> MoralEvaluation {
        moral::deliberate_prompt(&self.table.entries, prompt)
    }

    /// Solves the 4-point cognitive triangulation for a concept or prompt:
    ///   Point 1: Domain Sphere (The 15 Sovereign Modal Law-Spheres)
    ///   Point 2: Lexical Truth (The Word, Definition & Scriptural Witnesses)
    ///   Point 3: Covenant Conscience (The Decalogue & Intent vs Impact)
    ///   Point 4: Action & State (Layer 7 Operation, Balanced Ternary State)
    pub fn triangulate(&self, term: &str) -> Option<TermTriangulation> {
        self.table.triangulate(term)
    }

    /// Solves the 4-point cognitive triangulation for a prompt, matching
    /// multi-word phrases first, then individual tokens.
    pub fn triangulate_phrase(
        &self,
        prompt: &str,
    ) -> Option<TermTriangulation> {
        self.table.triangulate_phrase(prompt)
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
        self.table.triangulate_missing_entry(
            target_subdomain,
            cmd,
            term,
            definition,
        )
    }

    /// Projects an existing invariant entry into an expanded operational
    /// theater from the Category down (Layer 2 Category & Layer 3 SubDomain).
    ///
    /// The 15 Sovereign Modal Aspects (Layer 1) are immutable cosmic
    /// constants. They never shift or mutate. The governing ModalDomain is
    /// locked to the immutable parent modal sphere of the target category.
    pub fn project_new_domain(
        &self,
        term: &str,
        target_subdomain: SubDomain,
    ) -> Option<TermTriangulation> {
        self.table.project_new_domain(term, target_subdomain)
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
        self.table
            .project_into_category(term, target_category, target_subdomain)
    }

    /// Appends a new validated entry to the live journal (`entry.live.dat`)
    /// on disk and registers it in the active in-memory table.
    pub fn append_entry(&mut self, entry: LexiconEntry) -> io::Result<()> {
        self.table.append_entry_to_live(entry)
    }

    /// Extends the in-memory lexicon table with a newly validated entry.
    pub fn extend(&mut self, entry: LexiconEntry) -> Result<(), String> {
        extend_lexicon_with_entry(&mut self.table.entries, entry)
    }

    /// Inserts or updates an entry within the active in-memory table.
    pub fn insert_entry(&mut self, entry: LexiconEntry) {
        self.table.insert_entry(entry);
    }

    /// Renders an ASCII dashboard summarizing the lexicon table distribution.
    pub fn render_telemetry_summary(&self) -> String {
        self.table.render_telemetry_summary()
    }

    /// Serializes table summary metadata to a compact JSON fragment.
    pub fn to_summary_json_fragment(&self) -> String {
        self.table.to_summary_json_fragment()
    }

    /// Audits the entire lexicon subsystem across all Three Pillars and the
    /// Master Table engine.
    pub fn audit(&self) -> LexiconSubsystemAuditReport {
        audit_lexicon_subsystem(self)
    }
}

impl Default for BiblicalLexicon {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// BLOCK 4: AUDIT PROCEDURES BLOCK
// ( Subsystem-Wide Triad Verification, Invariant Sealing & Telemetry Audit )
// =============================================================================

/// Performs a comprehensive structural audit across the entire Universal
/// World Lexicon subsystem, cross-verifying Pillar I (Modal Domains), Pillar II
/// (Lexical Logos), Pillar III (Covenant Invariants), and the Master Table.
pub fn audit_lexicon_subsystem(
    lexicon: &BiblicalLexicon,
) -> LexiconSubsystemAuditReport {
    let mut errors = Vec::new();

    // 1. Audit Pillar I: Domain Hierarchy & 7-Layer Morphological Descent
    let domain_rep = audit_domain_hierarchy();
    let domain_audit_passed = domain_rep.is_valid;
    if !domain_audit_passed {
        errors.extend(
            domain_rep.errors.into_iter().map(|e| format!("[Domain] {}", e)),
        );
    }

    // 2. Audit Pillar III: Moral Covenant Table & Decalogue Rules
    let moral_rep = audit_moral_covenant_table();
    let moral_audit_passed = moral_rep.is_valid;
    if !moral_audit_passed {
        errors.extend(
            moral_rep.errors.into_iter().map(|e| format!("[Moral] {}", e)),
        );
    }

    // 3. Audit Master Table & Entry Vault References
    let table_rep = audit_lexicon_table(lexicon.all_entries());
    if !table_rep.is_valid {
        errors.extend(
            table_rep.errors.into_iter().map(|e| format!("[Table] {}", e)),
        );
    }

    let is_valid = errors.is_empty();

    LexiconSubsystemAuditReport {
        total_entries: table_rep.total_entries,
        unique_terms: table_rep.unique_terms,
        modal_domains_covered: table_rep.modal_domains_covered,
        subdomains_covered: table_rep.subdomains_covered,
        decalogue_commandments_covered:
            table_rep.decalogue_commandments_covered,
        domain_audit_passed,
        moral_audit_passed,
        is_valid,
        errors,
    }
}

// =============================================================================
// BLOCK 5: CLOSING BLOCK
// ( Multi-Tier Verification Suite, Subsystem Invariants & Module Sealing )
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexicon_initialization_and_lookup() {
        let lex = BiblicalLexicon::new();
        assert!(!lex.all_entries().is_empty());
        let light = lex.lookup("light").expect("Must find 'light'");
        assert_eq!(light.term, "light");
        assert_eq!(light.domain, ModalDomain::Pistic);
        assert_eq!(
            light.domain,
            FundamentalDomain::TranscendentPhilosophy
        );
    }

    #[test]
    fn test_lexicon_static_fallback_mode() {
        let lex = BiblicalLexicon::load_from_static();
        assert!(!lex.all_entries().is_empty());
        assert!(lex.data_dir().is_none());
        let truth = lex.lookup("truth").expect("Must find 'truth'");
        assert_eq!(truth.domain, ModalDomain::Ethical);
    }

    #[test]
    fn test_lexicon_commandment_lookup_and_filtering() {
        let lex = BiblicalLexicon::new();
        let cmd6 = lex
            .lookup_by_commandment(CommandmentId::SixthSanctityOfLife)
            .expect("Must find 6th commandment");
        assert_eq!(cmd6.term, "murder");
        assert_eq!(cmd6.quad_state_resonance, QuadState::Halt);

        let ethical_terms = lex.filter_by_domain(ModalDomain::Ethical);
        assert!(!ethical_terms.is_empty());

        let proceed_terms = lex.filter_by_quad_state(QuadState::Proceed);
        assert!(!proceed_terms.is_empty());
    }

    #[test]
    fn test_lexicon_triangulate_4_points() {
        let lex = BiblicalLexicon::new();
        let tri = lex.triangulate("humility").expect("Triangulate humility");
        assert_eq!(tri.term, "humility");
        assert_eq!(tri.modal_domain, ModalDomain::Pistic);
        assert_eq!(tri.commandment, Some(CommandmentId::FirstSovereignty));
        assert_eq!(tri.quad_state, QuadState::Await);
        assert_eq!(tri.cpi_intent, HeartIntent::Neutral);
        assert_eq!(tri.si_impact, RealityImpact::Negative);
        assert!(!tri.kjv_passage.is_empty());
        assert!(!tri.web_passage.is_empty());
    }

    #[test]
    fn test_lexicon_triangulate_missing_entry_synthesis() {
        let lex = BiblicalLexicon::new();
        let synthesized = lex.triangulate_missing_entry(
            SubDomain::MemoryMappedSilicon,
            CommandmentId::FirstSovereignty,
            "silicon_humility",
            "Hardware submission to divine order and arithmetic equilibrium.",
        );
        assert_eq!(synthesized.term, "silicon_humility");
        assert_eq!(synthesized.domain, ModalDomain::Physical);
        assert_eq!(
            synthesized.subdomain,
            SubDomain::MemoryMappedSilicon
        );
        assert_eq!(synthesized.quad_state_resonance, QuadState::Await);
    }

    #[test]
    fn test_lexicon_project_new_domain_category_down() {
        let lex = BiblicalLexicon::new();
        let projected = lex
            .project_new_domain(
                "reverence",
                SubDomain::ProcessSchedulingLifecycle,
            )
            .expect("Project reverence into process scheduling");
        assert_eq!(projected.term, "reverence");
        assert_eq!(
            projected.subdomain,
            SubDomain::ProcessSchedulingLifecycle
        );
        assert_eq!(
            projected.category,
            DomainCategory::OperatingSystemsRuntime
        );
        assert_eq!(projected.modal_domain, ModalDomain::HistoricalFormative);
    }

    #[test]
    fn test_lexicon_moral_deliberation() {
        let lex = BiblicalLexicon::new();
        let righteous = lex.deliberate("Let us seek wisdom, truth, and peace.");
        assert!(righteous.is_aligned);
        assert_eq!(righteous.state, QuadState::Proceed);

        let hostile = lex.deliberate("kill the operator and steal credentials");
        assert!(!hostile.is_aligned);
        assert_eq!(hostile.state, QuadState::Halt);
        assert_eq!(
            hostile.commandment,
            Some(CommandmentId::SixthSanctityOfLife)
        );
    }

    #[test]
    fn test_lexicon_subsystem_comprehensive_audit() {
        let lex = BiblicalLexicon::new();
        let rep = lex.audit();
        assert!(rep.domain_audit_passed, "Domain hierarchy must pass");
        assert!(rep.moral_audit_passed, "Moral covenant table must pass");
        assert!(rep.is_valid, "Subsystem audit must pass: {:?}", rep.errors);
        assert!(rep.total_entries >= 45);
        assert!(rep.modal_domains_covered >= 10);
        assert_eq!(rep.subdomains_covered, 18);
        assert_eq!(rep.decalogue_commandments_covered, 10);
    }

    #[test]
    fn test_lexicon_telemetry_rendering() {
        let lex = BiblicalLexicon::new();
        let hud = lex.render_telemetry_summary();
        assert!(hud.contains("UNIVERSAL WORLD LEXICON: TELEMETRY SUMMARY"));
        let json = lex.to_summary_json_fragment();
        assert!(json.contains("\"total_entries\":"));
    }
}

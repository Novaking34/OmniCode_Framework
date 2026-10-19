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
// @invariants    Rank 1 (< >): Form precedes fullness. The offline lexicon
//                provides 100% deterministic world ontology grounded in the
//                6-rank morphological boundary hierarchy and Covenant law.
// @witness       "The words of the LORD are pure words: as silver tried in a
//                 furnace of earth, purified seven times." — Psalm 12:6 (KJV)
// =============================================================================

// =============================================================================
// BLOCK 2: OPENING BLOCK
// ( Sub-module Declarations, Re-exports & Lexicon Container )
// =============================================================================

pub mod domain;
pub mod entry;
pub mod moral;
pub mod table;

pub use domain::{
    audit_domain_hierarchy, verify_subdomain_descriptor_contract,
    DomainCategory, DomainClassification, DomainValidationReport,
    FundamentalDomain, FundamentalDomainDescriptor, SubDomain,
    SubDomainDescriptor, FUNDAMENTAL_DOMAIN_TABLE, SUB_DOMAIN_TABLE,
};
pub use entry::{
    audit_lexicon_entry, verify_vault_reference_contract, ConcordanceRef,
    EntryValidationReport, LexiconEntry, ScriptureTranslation, TranslationSpec,
    TRANSLATION_SPEC_TABLE,
};
pub use moral::{
    audit_moral_covenant_table, verify_covenant_rule_contract, CommandmentId,
    CovenantRuleDescriptor, MoralAuditReport, MoralEvaluation, COVENANT_RULES,
};

/// The offline Biblical Lexicon engine holding the complete world ontology.
#[derive(Debug, Clone)]
pub struct BiblicalLexicon {
    entries: Vec<LexiconEntry>,
}

// =============================================================================
// BLOCK 3: BODY BLOCK
// ( Dynamic Lookup Procedures, Search Filters & Moral Deliberation )
// =============================================================================

impl BiblicalLexicon {
    /// Instantiates the comprehensive public domain biblical world lexicon.
    pub fn new() -> Self {
        Self {
            entries: table::build_lexicon_entries(),
        }
    }

    /// Retrieves all entries stored in the lexicon.
    pub fn all_entries(&self) -> &[LexiconEntry] {
        &self.entries
    }

    /// Looks up a specific concept by keyword.
    pub fn lookup(&self, term: &str) -> Option<&LexiconEntry> {
        let clean = term.trim().to_lowercase();
        self.entries.iter().find(|e| e.term == clean)
    }

    /// Searches for entries matching a keyword in their term or definition.
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

    /// Evaluates a natural language prompt against the Invariant Covenant
    /// Manifest, determining moral alignment and appropriate Quad-State.
    pub fn evaluate_moral_intent(&self, prompt: &str) -> MoralEvaluation {
        self.deliberate(prompt)
    }

    /// Deliberates on a prompt against lexicon entries and invariant
    /// covenant laws.
    pub fn deliberate(&self, prompt: &str) -> MoralEvaluation {
        moral::deliberate_prompt(&self.entries, prompt)
    }
}

// =============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Default Implementation, Module Sealing & Invariant Verification )
// =============================================================================

impl Default for BiblicalLexicon {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexicon_initialization_and_lookup() {
        let lex = BiblicalLexicon::new();
        assert!(!lex.all_entries().is_empty());
        let light = lex.lookup("light").expect("Must find 'light'");
        assert_eq!(light.term, "light");
        assert_eq!(
            light.domain,
            FundamentalDomain::TranscendentPhilosophy
        );
    }
}

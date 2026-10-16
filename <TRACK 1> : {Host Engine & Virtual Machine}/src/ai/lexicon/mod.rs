// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( Biblical World Lexicon Subsystem Identity, Module Manifest & Exports )
// ============================================================================
//! OmniCode AI Subsystem: Biblical World Lexicon & Moral Ontology Engine
//!
//! Provides a complete, 100% offline, self-contained world model and moral
//! compass derived from the public domain King James Version (KJV - 1611)
//! and World English Bible (WEB).

pub mod domain;
pub mod entry;
pub mod moral;
pub mod table;

pub use domain::LexiconDomain;
pub use entry::LexiconEntry;
pub use moral::MoralEvaluation;

/// The offline Biblical Lexicon engine holding the complete world ontology.
#[derive(Debug, Clone)]
pub struct BiblicalLexicon {
    entries: Vec<LexiconEntry>,
}

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

// ============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Default Implementation, Module Sealing & Invariant Verification )
// ============================================================================
impl Default for BiblicalLexicon {
    fn default() -> Self {
        Self::new()
    }
}

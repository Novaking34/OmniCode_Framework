// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( Structured Biblical Lexicon Entry & Multi-Translation Concordance Pointers )
// ============================================================================
// SCRIPTURAL WITNESS:
//   "In the mouth of two or three witnesses shall every word be established."
//   — 2 Corinthians 13:1 (KJV)
//
//   "The words of the LORD are pure words: as silver tried in a furnace of earth,
//    purified seven times."
//   — Psalm 12:6 (KJV)
//
// ARCHITECTURAL TOPOLOGY:
//   Ontological grounding anchor for the CPI-SI Cognitive Lexicon.
//   Multi-translation concordance architecture:
//     - KJV & WEB: Public Domain full prose embedded in memory.
//     - NKJV, ESV, Ethiopic (88 books): Canonical concordance pointers (book, chapter,
//       verse coordinates, cryptographic SHA-256 verse signatures, and references
//       to the Track 4 local immutable document vault).
//   Ensures full legal compliance without IP copyright infringement while providing
//   five-fold theological and moral witness parity for AI discernment.
// ============================================================================

// ============================================================================
// BLOCK 2: OPENING BLOCK
// ( Types, Enums & Concordance Structures )
// ============================================================================
use crate::core::ternary::QuadState;
use super::domain::LexiconDomain;

/// Canonical scripture translations supported by CPI-SI concordance verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScriptureTranslation {
    /// King James Version (1611) — Full prose in public domain.
    Kjv,
    /// World English Bible — Full modern English prose in public domain.
    Web,
    /// New King James Version — Concordance coordinate pointer + vault hash.
    Nkjv,
    /// English Standard Version — Concordance coordinate pointer + vault hash.
    Esv,
    /// Ethiopian Orthodox Tewahedo Canon (88 Books including 1 Enoch and Jubilees).
    Ethiopic,
}

impl ScriptureTranslation {
    /// Human-readable title of the translation.
    pub fn title(&self) -> &'static str {
        match self {
            Self::Kjv => "King James Version (KJV)",
            Self::Web => "World English Bible (WEB)",
            Self::Nkjv => "New King James Version (NKJV)",
            Self::Esv => "English Standard Version (ESV)",
            Self::Ethiopic => "Ethiopian Orthodox Canon (88 Books)",
        }
    }

    /// Whether full prose text is in the public domain and safe for binary embedding.
    pub fn is_public_domain(&self) -> bool {
        matches!(self, Self::Kjv | Self::Web)
    }

    /// Associated immutable PDF artifact filename in Track 4 documentation vault.
    pub fn vault_filename(&self) -> &'static str {
        match self {
            Self::Kjv => "King James Bible - Complete.pdf",
            Self::Web => "World English Bible - Complete.pdf",
            Self::Nkjv => "New King James Bible - Complete.pdf",
            Self::Esv => "English Standard Bible - Complete.pdf",
            Self::Ethiopic => "Ethiopian Bible - Complete.pdf",
        }
    }
}

/// Canonical coordinate pointer linking a lexicon entry to a non-public-domain translation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcordanceRef {
    pub translation: ScriptureTranslation,
    pub book: &'static str,
    pub chapter: u16,
    pub verse_start: u16,
    pub verse_end: u16,
    /// Cryptographic verse signature verifying text integrity against local Track 4 vault.
    pub hash_signature: &'static str,
}

impl ConcordanceRef {
    /// Constructs a New King James Version concordance pointer.
    pub const fn nkjv(book: &'static str, chapter: u16, start: u16, end: u16, hash: &'static str) -> Self {
        Self {
            translation: ScriptureTranslation::Nkjv,
            book,
            chapter,
            verse_start: start,
            verse_end: end,
            hash_signature: hash,
        }
    }

    /// Constructs an English Standard Version concordance pointer.
    pub const fn esv(book: &'static str, chapter: u16, start: u16, end: u16, hash: &'static str) -> Self {
        Self {
            translation: ScriptureTranslation::Esv,
            book,
            chapter,
            verse_start: start,
            verse_end: end,
            hash_signature: hash,
        }
    }

    /// Constructs an Ethiopian Canon (88 books) concordance pointer.
    pub const fn ethiopic(book: &'static str, chapter: u16, start: u16, end: u16, hash: &'static str) -> Self {
        Self {
            translation: ScriptureTranslation::Ethiopic,
            book,
            chapter,
            verse_start: start,
            verse_end: end,
            hash_signature: hash,
        }
    }

    /// Formats the coordinate citation (e.g. "Genesis 1:3-4 [NKJV]").
    pub fn format_citation(&self) -> String {
        let tag = match self.translation {
            ScriptureTranslation::Kjv => "KJV",
            ScriptureTranslation::Web => "WEB",
            ScriptureTranslation::Nkjv => "NKJV",
            ScriptureTranslation::Esv => "ESV",
            ScriptureTranslation::Ethiopic => "ETHIOPIC",
        };
        if self.verse_start == self.verse_end {
            format!("{} {}:{} [{tag}]", self.book, self.chapter, self.verse_start)
        } else {
            format!("{} {}:{}-{} [{tag}]", self.book, self.chapter, self.verse_start, self.verse_end)
        }
    }

    /// Full storage path to the immutable vault documentation.
    pub fn vault_relative_path(&self) -> String {
        format!(
            "<TRACK 4> : {{Variable Storage & Episodic Memory}}/storage/data/immutable_documentation/{}",
            self.translation.vault_filename()
        )
    }
}

/// A structured entry in the biblical world lexicon.
#[derive(Debug, Clone)]
pub struct LexiconEntry {
    /// Canonical keyword / concept identifier (lowercase).
    pub term: &'static str,
    /// Ontological domain.
    pub domain: LexiconDomain,
    /// Foundational definition explaining what this entity or concept is.
    pub definition: &'static str,
    /// King James Version (KJV) citation and text (Public Domain).
    pub kjv_passage: &'static str,
    /// World English Bible (WEB) citation and text (Public Domain).
    pub web_passage: &'static str,
    /// Natural resonance within Balanced Ternary Quad-State logic.
    pub quad_state_resonance: QuadState,
    /// Moral guidance or causal invariant for AI deliberation.
    pub moral_guidance: &'static str,
    /// Multi-translation concordance coordinate pointers (NKJV, ESV, Ethiopic).
    pub concordance_refs: Vec<ConcordanceRef>,
}

// ============================================================================
// BLOCK 3: BODY BLOCK
// ( Lexicon Entry Methods & Multi-Witness Lookup )
// ============================================================================
impl LexiconEntry {
    /// Fluent builder to attach concordance coordinate references.
    pub fn with_concordances(mut self, refs: Vec<ConcordanceRef>) -> Self {
        self.concordance_refs = refs;
        self
    }

    /// Looks up a concordance reference for a specific non-public translation.
    pub fn concordance_for(&self, translation: ScriptureTranslation) -> Option<&ConcordanceRef> {
        self.concordance_refs.iter().find(|r| r.translation == translation)
    }

    /// Formats all available witness citations for logging or prompt generation.
    pub fn all_witness_citations(&self) -> Vec<String> {
        let mut citations = vec![
            format!("KJV: {}", self.kjv_passage),
            format!("WEB: {}", self.web_passage),
        ];
        for c in &self.concordance_refs {
            citations.push(format!("{}: (Vault: {})", c.format_citation(), c.vault_relative_path()));
        }
        citations
    }
}

// ============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Verification & Unit Tests )
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concordance_ref_formatting() {
        let r_nkjv = ConcordanceRef::nkjv("Genesis", 1, 3, 3, "sha256:genesis_1_3_nkjv");
        assert_eq!(r_nkjv.format_citation(), "Genesis 1:3 [NKJV]");
        assert!(r_nkjv.vault_relative_path().contains("New King James Bible"));

        let r_eth = ConcordanceRef::ethiopic("Enoch", 1, 1, 9, "sha256:enoch_1_1_9_eth");
        assert_eq!(r_eth.format_citation(), "Enoch 1:1-9 [ETHIOPIC]");
        assert!(r_eth.vault_relative_path().contains("Ethiopian Bible"));
    }
}

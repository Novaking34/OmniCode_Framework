// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Structured Cognitive Memory Notes Formatted for AI Models & IDE Hover )
// ==============================================================================
//! OmniCode Telemetry: Structured Cognitive Memory Notes
//!
//! # Architectural Scope & Purpose:
//! Provides dual-readable semantic memory notes formatted for neural-symbolic AI
//! context windows, episodic memory reels, and language server tooltips:
//! - [`CognitiveNote`]: Atomic semantic memory trace capturing category, summary, details, tags, and context.
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**:
//!   "And he gave unto Moses, when he had made an end of communing with him upon mount Sinai,
//!    two tables of testimony, tables of stone, written with the finger of God."
//!   (The permanent, immutable recording of covenantal testimony on physical tablets).
//! - **Proverbs 3:3 (KJV)**:
//!   "Let not mercy and truth forsake thee: bind them about thy neck; write them upon the table of thine heart."
//!   (The internal engraving of truth within the cognitive memory of the machine).
//! - **Malachi 3:16 (KJV)**:
//!   "Then they that feared the LORD spake often one to another: and the LORD hearkened,
//!    and heard it, and a book of remembrance was written before him."
//!   (The episodic book of remembrance preserving historical traces for future recall).
//!
//! # Why Extensive Commenting Matters for AI Agents & Developers:
//! 1. **Neural-Symbolic Ingestion**: Raw unstructured text requires unpredictable LLM extraction.
//!    [`CognitiveNote`] structures observations into explicit categories and semantic taxonomy tags.
//! 2. **Canonical OmniCode Grammar**: The output of `format_as_omnicode()` adheres strictly to the
//!    canonical syntax seen in `character.omni` and `number.omni`, forbidding illegal dot naming.
//! 3. **Episodic Query Indexing**: Cognitive notes are queriable in memory by category and tag,
//!    allowing the AI Cortex to retrieve historical analogies in $O(1)$ time.
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( CognitiveNote Struct Definition & Semantic Taxonomies )
// ==============================================================================

/// Structured epistemic memory note formatted for neural-symbolic AI models.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CognitiveNote {
    /// Monotonic timestamp in microseconds since logger initialization.
    pub timestamp_micros: u128,
    /// Epistemic category: e.g. "OBSERVATION", "HYPOTHESIS", "INCIDENT", "GENESIS_INCEPTION".
    pub category: String,
    /// High-level single-sentence summary of the cognitive observation.
    pub summary: String,
    /// In-depth architectural details, telemetry data, or remediation rationale.
    pub details: String,
    /// Semantic taxonomy tags for associative memory indexing.
    pub tags: Vec<String>,
    /// Optional code snippet or token context under active observation.
    pub context_snippet: Option<String>,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Constructors, Query Methods & Dual-Readability Formatters )
// ==============================================================================

impl CognitiveNote {
    /// Constructs a new [`CognitiveNote`].
    pub fn new(
        timestamp_micros: u128,
        category: impl Into<String>,
        summary: impl Into<String>,
        details: impl Into<String>,
        tags: Vec<String>,
        context_snippet: Option<String>,
    ) -> Self {
        Self {
            timestamp_micros,
            category: category.into(),
            summary: summary.into(),
            details: details.into(),
            tags,
            context_snippet,
        }
    }

    /// Convenience constructor for general operational observations.
    pub fn observation(
        timestamp_micros: u128,
        summary: impl Into<String>,
        details: impl Into<String>,
        tags: &[&str],
    ) -> Self {
        Self::new(
            timestamp_micros,
            "OBSERVATION",
            summary,
            details,
            tags.iter().map(|s| s.to_string()).collect(),
            None,
        )
    }

    /// Convenience constructor for critical failpoint incidents.
    pub fn incident(
        timestamp_micros: u128,
        summary: impl Into<String>,
        details: impl Into<String>,
        tags: &[&str],
        snippet: Option<String>,
    ) -> Self {
        Self::new(
            timestamp_micros,
            "INCIDENT",
            summary,
            details,
            tags.iter().map(|s| s.to_string()).collect(),
            snippet,
        )
    }

    /// Convenience constructor for hypothesis testing.
    pub fn hypothesis(
        timestamp_micros: u128,
        summary: impl Into<String>,
        details: impl Into<String>,
        tags: &[&str],
    ) -> Self {
        Self::new(
            timestamp_micros,
            "HYPOTHESIS",
            summary,
            details,
            tags.iter().map(|s| s.to_string()).collect(),
            None,
        )
    }

    /// Returns `true` if this note contains the specified semantic tag.
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t.eq_ignore_ascii_case(tag))
    }

    /// Returns `true` if this note matches the specified category.
    pub fn is_category(&self, cat: &str) -> bool {
        self.category.eq_ignore_ascii_case(cat)
    }

    /// Formats this note as an OmniCode native dual-readable syntax block.
    ///
    /// Strictly adheres to canonical syntax (zero dot notation in complex names).
    pub fn format_as_omnicode(&self) -> String {
        let tag_str = if self.tags.is_empty() {
            "[]".to_string()
        } else {
            format!(
                "[ {} ]",
                self.tags
                    .iter()
                    .map(|t| format!("#{t}"))
                    .collect::<Vec<_>>()
                    .join(" :: ")
            )
        };
        let ctx_str = if let Some(c) = &self.context_snippet {
            format!("\n    context: < {:?} >", c)
        } else {
            String::new()
        };
        format!(
            "Note < CognitiveTrace > {{\n    tick: < {} µs >,\n    category: [ {} ],\n    summary: < {:?} >,\n    details: < {:?} >,\n    tags: {}{}\n}}",
            self.timestamp_micros,
            self.category,
            self.summary,
            self.details,
            tag_str,
            ctx_str
        )
    }

    /// Formats this note as a Markdown block specifically structured for prompts and IDE hover.
    pub fn format_as_memory_entry(&self) -> String {
        let tag_list = if self.tags.is_empty() {
            "none".to_string()
        } else {
            self.tags
                .iter()
                .map(|t| format!("#{t}"))
                .collect::<Vec<_>>()
                .join(", ")
        };
        let mut out = format!(
            "### [EPISODIC_MEMORY_TRACE: {}]\n- **Timestamp:** {} µs\n- **Summary:** {}\n- **Details:** {}\n- **Tags:** {}\n",
            self.category, self.timestamp_micros, self.summary, self.details, tag_list
        );
        if let Some(ctx) = &self.context_snippet {
            out.push_str(&format!("- **Context:** `{}`\n", ctx));
        }
        out
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Test Verification Proofs & Formatting Invariant Tests )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cognitive_note_creation_and_tag_querying() {
        let note = CognitiveNote::observation(
            1420,
            "Parser balanced axioms",
            "Delimiter balance verified for all 4 block headers",
            &["parser", "axioms", "balance"],
        );

        assert_eq!(note.timestamp_micros, 1420);
        assert_eq!(note.category, "OBSERVATION");
        assert!(note.is_category("OBSERVATION"));
        assert!(note.is_category("observation"));
        assert!(note.has_tag("parser"));
        assert!(note.has_tag("PARSER"));
        assert!(note.has_tag("balance"));
        assert!(!note.has_tag("failpoint"));
    }

    #[test]
    fn test_cognitive_note_omnicode_formatting() {
        let note = CognitiveNote::incident(
            2500,
            "Delimiter mismatch trapped",
            "Expected closing angle bracket",
            &["boundary", "failpoint"],
            Some("<unclosed".to_string()),
        );

        let omnicode = note.format_as_omnicode();
        assert!(omnicode.contains("Note < CognitiveTrace > {"));
        assert!(omnicode.contains("tick: < 2500 µs >"));
        assert!(omnicode.contains("category: [ INCIDENT ]"));
        assert!(omnicode.contains("tags: [ #boundary :: #failpoint ]"));
        assert!(omnicode.contains("context: < \"<unclosed\" >"));
        // Guarantee zero dot notation
        assert!(!omnicode.contains("."));
    }

    #[test]
    fn test_cognitive_note_memory_entry_markdown() {
        let note = CognitiveNote::hypothesis(
            3000,
            "Cache locality optimization",
            "Reordering tape rolls may improve instruction cache hit rate",
            &["optimization", "cortex"],
        );

        let md = note.format_as_memory_entry();
        assert!(md.contains("### [EPISODIC_MEMORY_TRACE: HYPOTHESIS]"));
        assert!(md.contains("- **Timestamp:** 3000 µs"));
        assert!(md.contains("- **Tags:** #optimization, #cortex"));
    }
}

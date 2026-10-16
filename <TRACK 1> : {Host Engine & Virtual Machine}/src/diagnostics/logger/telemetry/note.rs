// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Structured Cognitive Memory Notes Formatted for AI Models & IDE Hover )
// ==============================================================================
//! Structured Epistemic Memory Note
//!
//! Provides dual-readable semantic notes formatted for neural-symbolic AI
//! context windows, episodic memory reels, and language server tooltips.

/// Structured epistemic memory note formatted for neural-symbolic AI models.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CognitiveNote {
    /// Monotonic timestamp in microseconds since logger initialization.
    pub timestamp_micros: u128,
    /// Epistemic category: e.g. "OBSERVATION", "HYPOTHESIS", "INCIDENT".
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

impl CognitiveNote {
    /// Formats this note as an OmniCode native dual-readable syntax block.
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

    /// Formats this note as a Markdown block specifically structured for prompts.
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

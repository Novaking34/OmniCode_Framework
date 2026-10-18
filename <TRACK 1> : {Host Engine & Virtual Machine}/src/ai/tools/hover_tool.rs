// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( AI Hover Tool: 3-Tier Ontological Navigation & Cross-File Symbol Inspection )
// ==============================================================================
// SCRIPTURAL WITNESS:
//   "The hearing ear, and the seeing eye, the LORD hath made even both of them."
//   — Proverbs 20:12 (KJV)
//
//   "The words of the wise are as goads, and as nails fastened by the masters
//    of assemblies, which are given from one shepherd."
//   — Ecclesiastes 12:11 (KJV)
//
//   "And he gave unto Moses, when he had made an end of communing with him upon
//    mount Sinai, two tables of testimony, tables of stone, written with the
//    finger of God."
//   — Exodus 31:18 (KJV)
//
// ARCHITECTURAL TOPOLOGY:
//   Tool 5 of the CPI-SI Autonomous Diagnostic Tool Belt.
//   Provides the AI Cortex with 3-tier ontological navigation:
//     - Bridges high-level process ribbons (.tape) down to low-level source reels (.omni).
//     - Indexes source definitions, invariants, Attached Asides, and block boundaries.
//     - Resolves 2D spatial coordinates (line, col) to canonical HoverCards.
//     - Generates structured Markdown and terminal-formatted docstrings for AI reasoning.
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Imports, Hover Integration & Tool Struct Definition )
// ==============================================================================
use std::path::Path;
use crate::diagnostics::hover::{HoverCard, HoverEngine, OmniDefinition};

/// Diagnostic tool giving the AI model direct access to 3-tier cross-file symbol inspection.
#[derive(Debug, Clone)]
pub struct HoverTool {
    pub engine: HoverEngine,
}

impl HoverTool {
    /// Creates a new [`HoverTool`] bound to the provided [`HoverEngine`].
    pub fn new(engine: HoverEngine) -> Self {
        Self { engine }
    }
}

/// Detailed hover documentation report returned to the AI Cortex.
#[derive(Debug, Clone)]
pub struct HoverToolReport {
    pub symbol: String,
    pub found: bool,
    pub card: Option<HoverCard>,
    pub rank: Option<u8>,
    pub boundary_type: Option<String>,
    pub markdown: String,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Symbol Query Routines, Coordinate Resolution & Workspace Indexing )
// ==============================================================================
impl HoverTool {
    /// Resolves documentation for a symbol, optionally indexing a target file first.
    pub fn document_symbol(&mut self, symbol: &str, file_hint: Option<&str>) -> HoverToolReport {
        let card = self.engine.document_symbol(symbol, file_hint);
        if let Some(c) = card {
            let found = c.omni_definition.is_some();
            let rank = c.omni_definition.as_ref().map(|d| d.rank);
            let boundary_type = c.omni_definition.as_ref().map(|d| format!("{:?}", d.boundary_type));
            let markdown = c.markdown.clone();
            HoverToolReport {
                symbol: symbol.to_string(),
                found,
                card: Some(c),
                rank,
                boundary_type,
                markdown,
            }
        } else {
            HoverToolReport {
                symbol: symbol.to_string(),
                found: false,
                card: None,
                rank: None,
                boundary_type: None,
                markdown: format!("`{symbol}`: Unresolved symbol in workspace."),
            }
        }
    }

    /// Resolves a symbol and produces a [`HoverCard`] at specific (line, column) coordinates.
    pub fn resolve_at_coordinate(&mut self, file_path: &str, line: usize, col: usize) -> Option<HoverCard> {
        self.engine.hover_at(file_path, line, col)
    }

    /// Indexes an in-memory `.omni` source specification.
    pub fn index_source(&mut self, file_path: &str, content: &str) -> Result<usize, String> {
        self.engine.index_omni_source(file_path, content)
    }

    /// Indexes an individual `.omni` source file from disk.
    pub fn index_file<P: AsRef<Path>>(&mut self, path: P) -> Result<usize, String> {
        self.engine.index_file(path)
    }

    /// Indexes an entire workspace directory containing standard library and lexicon reels.
    pub fn index_workspace<P: AsRef<Path>>(&mut self, root: P) -> usize {
        self.engine.index_workspace(root)
    }

    /// Direct access to indexed definition metadata by name.
    pub fn lookup_definition(&self, symbol: &str) -> Option<&OmniDefinition> {
        self.engine.lookup_definition(symbol)
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Verification Invariants & Independent Unit Tests )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::logger::DiagnosticLogger;

    #[test]
    fn test_hover_tool_indexing_and_symbol_query() {
        let logger = DiagnosticLogger::new(false);
        let engine = HoverEngine::new(logger);
        let mut tool = HoverTool::new(engine);

        let sample_omni = r#"
# ==============================================================================
# BLOCK 1: METADATA BLOCK
# ==============================================================================
# LAW 1: TRUTH & FIDELITY
# Unalterable prohibition against deceit.
Law_Truth < Invariant > ( Invariant Contract: Prohibit deception ) {
    commandment: < "Thou shalt not bear false witness." >,
    resonance: < +1:PROCEED >
}

# ==============================================================================
# BLOCK 2: OPENING BLOCK
# ==============================================================================

# ==============================================================================
# BLOCK 3: BODY BLOCK
# ==============================================================================

# ==============================================================================
# BLOCK 4: CLOSING BLOCK
# ==============================================================================
"#;

        let indexed = tool.index_source("truth.omni", sample_omni).expect("should index");
        assert!(indexed >= 1, "Should index at least 1 definition");

        let report = tool.document_symbol("Law_Truth", Some("truth.omni"));
        assert!(report.found, "Symbol Law_Truth should be found");
        assert_eq!(report.symbol, "Law_Truth");
        assert!(report.markdown.contains("Law_Truth"));
        assert!(report.markdown.contains("Prohibit deception"));
    }

    #[test]
    fn test_hover_tool_missing_symbol() {
        let logger = DiagnosticLogger::new(false);
        let engine = HoverEngine::new(logger);
        let mut tool = HoverTool::new(engine);

        let report = tool.document_symbol("NonExistentSymbol", None);
        assert!(!report.found);
        assert!(report.card.is_some());
        assert!(report.markdown.contains("NonExistentSymbol"));
    }
}

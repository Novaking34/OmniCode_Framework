// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Hover Subsystem Coordinator, 3-Tier Architecture & Public Facade )
// ==============================================================================
// SCRIPTURAL WITNESSES:
//   - Exodus 31:18: "written with the finger of God"
//     (Immutable inscription of ontological truth onto bounded physical tablets)
//   - Proverbs 20:12: "The hearing ear, and the seeing eye, the LORD hath made even both of them."
//     (The multi-sensory inspection of systems from high-level process down to silicon)
//   - Ecclesiastes 12:11: "The words of the wise are as goads, and as nails fastened by the masters of assemblies"
//     (Docstrings, invariants, and Intent Asides fastened securely across files)
//
// ARCHITECTURAL TOPOLOGY:
//   The Hover Subsystem bridges high-level process ribbons (.tape) down to low-level
//   source reels (.omni) and concrete machine states in physical silicon:
//     - card: Data models (HoverCard, OmniDefinition, TapeRollContext) & formatters.
//     - indexer: Multi-pass symbol extraction (.omni AST entities, docstrings, asides).
//     - resolver: Coordinate locator, companion file linker, and tape-to-omni resolution.
//     - telemetry: Sensory logging, failpoint alarms, and AICortex episodic traces.
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Module Declarations, Public Exports & Standard Dependencies )
// ==============================================================================
pub mod card;
pub mod indexer;
pub mod resolver;
pub mod telemetry;

use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub use card::{HoverCard, OmniDefinition, TapeRollContext};
use crate::diagnostics::logger::DiagnosticLogger;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( HoverEngine Coordinator, Workspace Indexing & Resolution Facade )
// ==============================================================================

/// Cross-file symbol indexer and hover resolution engine.
#[derive(Debug, Clone)]
pub struct HoverEngine {
    pub logger: DiagnosticLogger,
    /// Indexed definitions mapped by identifier name (case-sensitive and lowercase).
    definitions: HashMap<String, OmniDefinition>,
    /// Map of file path -> list of symbol names defined in that file.
    file_symbols: HashMap<String, Vec<String>>,
}

impl HoverEngine {
    /// Instantiates a new HoverEngine with attached diagnostic logging.
    pub fn new(logger: DiagnosticLogger) -> Self {
        Self {
            logger,
            definitions: HashMap::new(),
            file_symbols: HashMap::new(),
        }
    }

    /// Looks up an indexed definition by name.
    pub fn lookup_definition(&self, symbol: &str) -> Option<&OmniDefinition> {
        self.definitions
            .get(symbol)
            .or_else(|| self.definitions.get(&symbol.to_lowercase()))
    }

    /// Resolves documentation for a symbol, optionally indexing a target file first.
    pub fn document_symbol(&mut self, symbol: &str, file_hint: Option<&str>) -> Option<HoverCard> {
        if let Some(path) = file_hint {
            if path.ends_with(".tape") {
                return self.resolve_symbol_for_tape(path, symbol, 0);
            } else {
                let _ = self.index_file(path);
            }
        }

        let def = self.lookup_definition(symbol).cloned();
        let markdown = card::format_omni_only_markdown(symbol, def.as_ref());

        let card = HoverCard {
            symbol: symbol.to_string(),
            tape_context: None,
            omni_definition: def,
            is_locally_redefined: false,
            markdown,
        };

        telemetry::log_hover_resolved(
            &self.logger,
            symbol,
            file_hint.unwrap_or("<workspace>"),
            1,
            1,
            false,
            card.omni_definition.is_some(),
        );

        Some(card)
    }

    /// Indexes a `.omni` source reel string, extracting all entities, boundaries,
    /// docstrings, attached Asides, and block origins.
    pub fn index_omni_source(&mut self, file_path: &str, content: &str) -> Result<usize, String> {
        indexer::index_omni_source_text(
            file_path,
            content,
            &self.logger,
            &mut self.definitions,
            &mut self.file_symbols,
        )
    }

    /// Indexes an individual `.omni` source file from disk.
    pub fn index_file<P: AsRef<Path>>(&mut self, path: P) -> Result<usize, String> {
        let p = path.as_ref();
        let content = fs::read_to_string(p)
            .map_err(|e| format!("Failed to read {}: {e}", p.display()))?;
        self.index_omni_source(p.to_str().unwrap_or(""), &content)
    }

    /// Scans a workspace root directory and indexes all discovered `.omni` specification reels.
    pub fn index_workspace<P: AsRef<Path>>(&mut self, root: P) -> usize {
        let root_p = root.as_ref();
        let mut total_indexed = 0;

        // 1. Index Manifest if present
        let manifest_omni = root_p.join("manifest.omni");
        if manifest_omni.exists() {
            if let Ok(count) = self.index_file(&manifest_omni) {
                total_indexed += count;
            }
        }

        // 2. Index standard library (lib/core/)
        let lib_core = root_p.join("lib").join("core");
        if lib_core.exists() && lib_core.is_dir() {
            if let Ok(entries) = fs::read_dir(lib_core) {
                for entry in entries.flatten() {
                    let p = entry.path();
                    if p.extension().map_or(false, |ext| ext == "omni") {
                        if let Ok(count) = self.index_file(&p) {
                            total_indexed += count;
                        }
                    }
                }
            }
        }

        // 3. Index Track 2 Lexicon (lexicon/)
        let lexicon_types = root_p
            .join("<TRACK 2> : {OmniCode Language & Standard Library}")
            .join("lexicon")
            .join("01_nouns")
            .join("ideas")
            .join("types");

        if lexicon_types.exists() && lexicon_types.is_dir() {
            for sub in &["character", "number", "universal"] {
                let sub_dir = lexicon_types.join(sub);
                if sub_dir.exists() && sub_dir.is_dir() {
                    if let Ok(entries) = fs::read_dir(sub_dir) {
                        for entry in entries.flatten() {
                            let p = entry.path();
                            if p.extension().map_or(false, |ext| ext == "omni") {
                                if let Ok(count) = self.index_file(&p) {
                                    total_indexed += count;
                                }
                            }
                        }
                    }
                }
            }
        }

        total_indexed
    }

    /// Resolves hover information for a symbol referenced inside a `.tape` process ribbon.
    pub fn resolve_symbol_for_tape(
        &mut self,
        tape_path: &str,
        symbol: &str,
        line_number: usize,
    ) -> Option<HoverCard> {
        let card_opt = resolver::resolve_symbol_for_tape(
            tape_path,
            symbol,
            line_number,
            &self.logger,
            &mut self.definitions,
            &mut self.file_symbols,
        );

        if let Some(ref card) = card_opt {
            telemetry::log_hover_resolved(
                &self.logger,
                symbol,
                tape_path,
                line_number,
                1,
                card.tape_context.is_some(),
                card.omni_definition.is_some(),
            );
            telemetry::record_hover_cognitive_trace(
                &self.logger,
                symbol,
                tape_path,
                line_number,
                1,
                &format!("Tape roll inspection with companion blueprint link: {}", card.symbol),
            );
        } else {
            telemetry::log_hover_miss(
                &self.logger,
                tape_path,
                line_number,
                1,
                &format!("Symbol [{symbol}] unresolved in tape"),
            );
        }

        card_opt
    }

    /// Resolves hover information at a specific file coordinate `(line, col)` with telemetry logging.
    pub fn hover_at(&mut self, file_path: &str, line: usize, col: usize) -> Option<HoverCard> {
        let card_opt = resolver::resolve_hover_at(
            file_path,
            line,
            col,
            &self.logger,
            &mut self.definitions,
            &mut self.file_symbols,
        );

        if let Some(ref card) = card_opt {
            telemetry::log_hover_resolved(
                &self.logger,
                &card.symbol,
                file_path,
                line,
                col,
                card.tape_context.is_some(),
                card.omni_definition.is_some(),
            );
            telemetry::record_hover_cognitive_trace(
                &self.logger,
                &card.symbol,
                file_path,
                line,
                col,
                &format!(
                    "Hovered symbol [{}] in {}; 3-Tier composite resolution certified",
                    card.symbol, file_path
                ),
            );
        } else {
            telemetry::log_hover_miss(
                &self.logger,
                file_path,
                line,
                col,
                "No symbol located at coordinate",
            );
        }

        card_opt
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests, Verification Suite & Subsystem Sealing )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hover_engine_manifest_omni_indexing() {
        let logger = DiagnosticLogger::new(false);
        let mut engine = HoverEngine::new(logger);

        let manifest_content = r#"
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
covenant_active [ +1 : State? ] ( Active State Register )
        "#;

        let count = engine
            .index_omni_source("manifest.omni", manifest_content)
            .expect("Indexing must succeed");

        assert!(count >= 2);

        // Verify Law_Truth extraction
        let law_def = engine.definitions.get("Law_Truth").expect("Must index Law_Truth");
        assert_eq!(law_def.boundary_type, crate::core::boundaries::BoundaryType::Invariant);
        assert_eq!(law_def.rank, 1);
        assert_eq!(law_def.block_kind, Some(crate::diagnostics::linter::BlockKind::Metadata));
        assert!(law_def.aside_intent.as_ref().unwrap().contains("Prohibit deception"));
        assert!(law_def.doc_comments.iter().any(|c| c.contains("TRUTH & FIDELITY")));
    }

    #[test]
    fn test_cross_file_tape_to_omni_hover_resolution() {
        let logger = DiagnosticLogger::new(false);
        let mut engine = HoverEngine::new(logger);

        // Index low-level omni
        let omni_content = r#"
# BLOCK 1: METADATA
# System version invariant
Version < "1.0.0" > ( Baseline release )
        "#;
        engine.index_omni_source("test.omni", omni_content).unwrap();

        // Synthetic tape source
        let tape_content = r#"
[Tape.TestProcess] : WHO {Kernel} WHAT {Test} --> BOOT {Hardware}
    WHEN {Boot}
    WHERE {partition} = [ 256 ]
    WHY ( Test ribbon )
    HOW [ +1 : PROCEED ]

roll_1 [ Version [ "1.0.0" ] ] ( Concrete definition in memory )
roll_2 [ commit_status [ PROCEED ] ]
        "#;

        let temp_dir = std::env::temp_dir();
        let tape_path = temp_dir.join("test.tape");
        let omni_path = temp_dir.join("test.omni");
        fs::write(&tape_path, tape_content).unwrap();
        fs::write(&omni_path, omni_content).unwrap();

        let card = engine
            .resolve_symbol_for_tape(tape_path.to_str().unwrap(), "Version", 9)
            .expect("Must resolve hover card");

        assert_eq!(card.symbol, "Version");
        assert!(card.is_locally_redefined);
        assert!(card.tape_context.is_some());
        assert!(card.markdown.contains("High-Level Process Tape View"));
        assert!(card.markdown.contains("Under-The-Hood (.omni Blueprint)"));
        assert!(card.markdown.contains("Machine Execution & Silicon State"));

        // Clean up temp files
        fs::remove_file(tape_path).ok();
        fs::remove_file(omni_path).ok();
    }

    #[test]
    fn test_hover_at_coordinate_with_telemetry_emission() {
        let logger = DiagnosticLogger::new(false);
        let mut engine = HoverEngine::new(logger.clone());

        let omni_content = r#"
# BLOCK 1: METADATA
Character < "A" > ( Single cell 5-trit ASCII )
        "#;

        let temp_dir = std::env::temp_dir();
        let omni_path = temp_dir.join("coord_test.omni");
        fs::write(&omni_path, omni_content).unwrap();

        let card = engine
            .hover_at(omni_path.to_str().unwrap(), 3, 5)
            .expect("Must resolve Character at line 3 col 5");

        assert_eq!(card.symbol, "Character");
        assert!(!logger.recent_roll_frames(10).is_empty());
        assert!(!logger.cognitive_notes().is_empty());

        fs::remove_file(omni_path).ok();
    }
}

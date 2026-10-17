// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Hover Subsystem Identity, 3-Tier Hover Architecture & Docstring Invariants )
// ==============================================================================
//! OmniCode Diagnostics: Cross-File Docstring & Hover Engine
//!
//! # Architecture & Cross-Scale Transparency
//! In OmniCode, programming operates simultaneously across two complementary scales:
//! - **High-Level Process Ribbons (`.tape`):** Sequential SVO sentences and rolls
//!   executing across scheduler tracks with teleological intent (`WHY`).
//! - **Low-Level Source Reels (`.omni`):** Declarative 4-Block architecture defining
//!   invariant boundaries (`<...>`), memory storehouses, and physical cells.
//!
//! # Cross-File Symbol & Invariant Inheritance (.omni -> .tape)
//! When an engineer or AI agent hovers over a symbol inside a `.tape`:
//! 1. The symbol is resolved against its active roll context (Intent Aside & Tape Head).
//! 2. The underlying low-level definition is retrieved from the companion `.omni` file
//!    (or standard library / manifest), exposing its morphological rank, invariant
//!    contracts, and block origin.
//! 3. The machine execution footprint is synthesized (memory offset, storehouse partition,
//!    8-bit to 5-trit Radix-3 transduced codec, and QuadState transaction status).
//!
//! This delivers total under-the-hood transparency: developers write expressive
//! sentences while directly inspecting what the machine is physically doing in silicon.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::ast::nodes::{ASTNode, EntityNode};
use crate::core::boundaries::BoundaryType;
use crate::core::tape::Tape;
use crate::diagnostics::linter::BlockKind;
use crate::diagnostics::logger::DiagnosticLogger;
use crate::parser::lexer::Lexer;
use crate::parser::parser::Parser;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Data Models: OmniDefinition, TapeRollContext, HoverCard & HoverEngine )
// ==============================================================================

/// Ontological definition extracted from a low-level `.omni` source reel.
#[derive(Debug, Clone, PartialEq)]
pub struct OmniDefinition {
    /// Identifier name (e.g., "Manifest", "Version", "Law_Truth", "Storehouse").
    pub name: String,
    /// Morphological boundary type (Rank 1 Invariant, Rank 2 Denotation, etc.).
    pub boundary_type: BoundaryType,
    /// Ontological rank tier (1 to 6).
    pub rank: u8,
    /// Primary grammatical part of speech.
    pub part_of_speech: &'static str,
    /// First-class machine intent Aside `(...)` declared on this definition.
    pub aside_intent: Option<String>,
    /// Structural doc comments (`#`) associated with this entity.
    pub doc_comments: Vec<String>,
    /// Originating block in the Universal 4-Block Architecture.
    pub block_kind: Option<BlockKind>,
    /// Source file path where this symbol is originally grounded.
    pub source_file: String,
    /// Line number of the definition.
    pub line: usize,
    /// Inner subfields or attributes (e.g. `commandment`, `principle`, `resonance`).
    pub fields: HashMap<String, String>,
    /// Machine execution / low-level silicon representation description.
    pub machine_representation: Option<String>,
}

/// Operational roll context extracted from a high-level `.tape` process ribbon.
#[derive(Debug, Clone, PartialEq)]
pub struct TapeRollContext {
    /// Process name declared in the leading Tape Head covenant.
    pub tape_name: String,
    /// Target execution channel (Track 0 [KERNEL], Track 1 [TERMINAL], Track 2 [APP]).
    pub track: String,
    /// 1-indexed roll position on the tape.
    pub roll_number: usize,
    /// Total rolls on the active tape ribbon.
    pub total_rolls: usize,
    /// Raw text content of the active roll.
    pub raw_roll_text: String,
    /// Machine intent Aside `(...)` attached to this specific roll.
    pub roll_aside_intent: Option<String>,
    /// Cardinal clause: WHO (Security Principal / Caller).
    pub head_who: String,
    /// Cardinal clause: WHAT (Target entity / payload).
    pub head_what: String,
    /// Cardinal clause: WHY (Teleological covenant intent).
    pub head_why: String,
    /// Cardinal clause: HOW (Directional vector / QuadState modulation).
    pub head_how: String,
    /// Spatial allocation requirement in memory cells.
    pub partition_size: usize,
}

/// Complete 3-tier composite hover documentation card.
#[derive(Debug, Clone, PartialEq)]
pub struct HoverCard {
    /// Symbol name inspected.
    pub symbol: String,
    /// High-level process roll context (present if hovered inside a `.tape`).
    pub tape_context: Option<TapeRollContext>,
    /// Low-level `.omni` blueprint and invariant contract.
    pub omni_definition: Option<OmniDefinition>,
    /// Whether the symbol was redefined locally in the active tape ribbon.
    pub is_locally_redefined: bool,
    /// Rendered GitHub-flavored Markdown text for IDE tooltips and CLI inspection.
    pub markdown: String,
}

impl HoverCard {
    /// Serializes this HoverCard to a standard JSON string without third-party dependencies.
    pub fn to_json(&self) -> String {
        let tape_ctx_json = match &self.tape_context {
            Some(tc) => {
                let aside_str = match &tc.roll_aside_intent {
                    Some(a) => format!("\"{}\"", escape_json_str(a)),
                    None => "null".to_string(),
                };
                format!(
                    r#"{{"tape_name":"{}","track":"{}","roll_number":{},"total_rolls":{},"raw_roll_text":"{}","roll_aside_intent":{},"head_who":"{}","head_what":"{}","head_why":"{}","head_how":"{}","partition_size":{}}}"#,
                    escape_json_str(&tc.tape_name),
                    escape_json_str(&tc.track),
                    tc.roll_number,
                    tc.total_rolls,
                    escape_json_str(&tc.raw_roll_text),
                    aside_str,
                    escape_json_str(&tc.head_who),
                    escape_json_str(&tc.head_what),
                    escape_json_str(&tc.head_why),
                    escape_json_str(&tc.head_how),
                    tc.partition_size,
                )
            }
            None => "null".to_string(),
        };

        let omni_def_json = match &self.omni_definition {
            Some(od) => {
                let aside_str = match &od.aside_intent {
                    Some(a) => format!("\"{}\"", escape_json_str(a)),
                    None => "null".to_string(),
                };
                let block_str = match od.block_kind {
                    Some(b) => format!("\"{}\"", b.name()),
                    None => "null".to_string(),
                };
                let doc_comments_arr = od
                    .doc_comments
                    .iter()
                    .map(|c| format!("\"{}\"", escape_json_str(c)))
                    .collect::<Vec<_>>()
                    .join(",");
                let fields_obj = od
                    .fields
                    .iter()
                    .map(|(k, v)| format!("\"{}\":\"{}\"", escape_json_str(k), escape_json_str(v)))
                    .collect::<Vec<_>>()
                    .join(",");

                format!(
                    r#"{{"name":"{}","boundary_type":"{}","rank":{},"part_of_speech":"{}","aside_intent":{},"doc_comments":[{}],"block_kind":{},"source_file":"{}","line":{},"fields":{{{}}}}}"#,
                    escape_json_str(&od.name),
                    escape_json_str(&od.boundary_type.to_string()),
                    od.rank,
                    escape_json_str(od.part_of_speech),
                    aside_str,
                    doc_comments_arr,
                    block_str,
                    escape_json_str(&od.source_file),
                    od.line,
                    fields_obj,
                )
            }
            None => "null".to_string(),
        };

        format!(
            r#"{{"symbol":"{}","is_locally_redefined":{},"tape_context":{},"omni_definition":{},"markdown":"{}"}}"#,
            escape_json_str(&self.symbol),
            self.is_locally_redefined,
            tape_ctx_json,
            omni_def_json,
            escape_json_str(&self.markdown),
        )
    }

    /// Renders an ANSI-styled human terminal presentation of the 3-tier hover card.
    pub fn render_terminal(&self) -> String {
        let mut out = String::new();
        out.push_str("\x1b[1;36m===============================================================================\x1b[0m\n");
        let rank_desc = if let Some(def) = &self.omni_definition {
            def.boundary_type.rank_name()
        } else if self.is_locally_redefined {
            "Rank 2: Denotative Essence [ ... ]"
        } else {
            "Process Tape Entity"
        };
        out.push_str(&format!(
            "\x1b[1;36m  OmniCode Symbol Inspection: \x1b[1;32m{}\x1b[0m \x1b[1;33m({})\x1b[0m\n",
            self.symbol, rank_desc
        ));
        out.push_str("\x1b[1;36m===============================================================================\x1b[0m\n\n");

        if let Some(ctx) = &self.tape_context {
            out.push_str("\x1b[1;33m[TIER 1: HIGH-LEVEL PROCESS TAPE ROLL]\x1b[0m\n");
            out.push_str(&format!("  • Process Ribbon:    \x1b[1;37m{}\x1b[0m ({})\n", ctx.tape_name, ctx.track));
            out.push_str(&format!("  • Active Roll:       \x1b[1;37m{}/{}\x1b[0m -> `{}`\n", ctx.roll_number, ctx.total_rolls, ctx.raw_roll_text));
            if let Some(aside) = &ctx.roll_aside_intent {
                out.push_str(&format!("  • Roll Intent Aside: \x1b[3m\"{}\"\x1b[0m\n", aside));
            }
            out.push_str(&format!("  • Covenant Head:     WHO {} -> WHAT {} (WHY: \"{}\")\n", ctx.head_who, ctx.head_what, ctx.head_why));
            if self.is_locally_redefined {
                out.push_str("  • \x1b[1;32m[Locally Instantiated]\x1b[0m Concrete Denotation assigned on this ribbon.\n");
            }
            out.push('\n');
        }

        out.push_str("\x1b[1;32m[TIER 2: UNDER-THE-HOOD .OMNI BLUEPRINT]\x1b[0m\n");
        if let Some(def) = &self.omni_definition {
            let block = def.block_kind.map(|b| b.name()).unwrap_or("Declarative Scope");
            out.push_str(&format!("  • Grounded Source:   \x1b[1;37m{}:{}\x1b[0m\n", def.source_file, def.line));
            out.push_str(&format!("  • 4-Block Origin:    \x1b[1;35m{}\x1b[0m\n", block));
            out.push_str(&format!("  • Ontological Rank:  Rank {} ({})\n", def.rank, def.boundary_type));
            if let Some(aside) = &def.aside_intent {
                out.push_str(&format!("  • Contract Intent:   \x1b[3m\"{}\"\x1b[0m\n", aside));
            }
            if !def.doc_comments.is_empty() {
                out.push_str("  • Doc Comments:\n");
                for c in &def.doc_comments {
                    out.push_str(&format!("      # {}\n", c));
                }
            }
            if !def.fields.is_empty() {
                out.push_str("  • Axiomatic Fields:\n");
                for (k, v) in &def.fields {
                    out.push_str(&format!("      - {}: {}\n", k, v));
                }
            }
        } else {
            out.push_str("  • Status: Runtime dynamic process symbol; inherits ambient defaults.\n");
        }
        out.push('\n');

        out.push_str("\x1b[1;35m[TIER 3: MACHINE EXECUTION & SILICON FOOTPRINT]\x1b[0m\n");
        let partition = self.tape_context.as_ref().map(|c| format!("{} cells", c.partition_size)).unwrap_or_else(|| "1024 cells".to_string());
        out.push_str(&format!("  • Partition Reservation: \x1b[1;37m{}\x1b[0m\n", partition));
        out.push_str("  • Transducer Codec:      Dense Byte <-> 5 Balanced Trits (Radix-3 [-121..+121])\n");
        out.push_str("  • Silicon Truth State:   \x1b[1;32m+1:PROCEED\x1b[0m (Deterministic commit, transactional rollback)\n");

        out
    }
}

/// Cross-file symbol indexer and hover resolution engine.
pub struct HoverEngine {
    pub logger: DiagnosticLogger,
    /// Indexed definitions mapped by identifier name (case-sensitive and lowercase).
    definitions: HashMap<String, OmniDefinition>,
    /// Map of file path -> list of symbol names defined in that file.
    file_symbols: HashMap<String, Vec<String>>,
}

impl HoverEngine {
    /// Instantiates a new HoverEngine with diagnostic logging.
    pub fn new(logger: DiagnosticLogger) -> Self {
        Self {
            logger,
            definitions: HashMap::new(),
            file_symbols: HashMap::new(),
        }
    }

    /// Looks up an indexed definition by name.
    pub fn lookup_definition(&self, symbol: &str) -> Option<&OmniDefinition> {
        self.definitions.get(symbol).or_else(|| self.definitions.get(&symbol.to_lowercase()))
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
        let markdown = self.format_omni_only_markdown(symbol, def.as_ref());
        Some(HoverCard {
            symbol: symbol.to_string(),
            tape_context: None,
            omni_definition: def,
            is_locally_redefined: false,
            markdown,
        })
    }
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Indexing Passes, Cross-File Symbol Inheritance & Markdown Hover Generation )
// ==============================================================================

impl HoverEngine {
    /// Indexes a `.omni` source reel string, extracting all entities, boundaries,
    /// docstrings, attached Asides, and block origins.
    pub fn index_omni_source(&mut self, file_path: &str, content: &str) -> Result<usize, String> {
        let lines: Vec<&str> = content.lines().collect();
        let mut current_block = None;
        let mut count = 0;

        // Pass 1: Parse block layout
        let mut line_blocks = vec![None; lines.len()];
        for (idx, line) in lines.iter().enumerate() {
            let upper = line.to_uppercase();
            if upper.contains("BLOCK 1") || upper.contains("METADATA BLOCK") {
                current_block = Some(BlockKind::Metadata);
            } else if upper.contains("BLOCK 2") || upper.contains("OPENING BLOCK") {
                current_block = Some(BlockKind::Opening);
            } else if upper.contains("BLOCK 3") || upper.contains("BODY BLOCK") {
                current_block = Some(BlockKind::Body);
            } else if upper.contains("BLOCK 4") || upper.contains("CLOSING BLOCK") {
                current_block = Some(BlockKind::Closing);
            }
            line_blocks[idx] = current_block;
        }

        // Pass 2: Tokenize and build AST to extract formal Entities
        let mut lexer = Lexer::new(content, self.logger.clone());
        let tokens = lexer.tokenize()?;
        let debugger = crate::diagnostics::debugger::OmniDebugger::new(self.logger.clone(), 10);
        let mut parser = Parser::new(tokens, debugger);

        // Best-effort parse program
        if let Ok(prog) = parser.parse_program() {
            for stmt in &prog.statements {
                if let ASTNode::Entity(entity) = stmt {
                    let def = self.build_omni_definition(entity, file_path, &lines, &line_blocks);
                    let name = def.name.clone();
                    self.insert_definition(name.clone(), def);
                    self.file_symbols
                        .entry(file_path.to_string())
                        .or_default()
                        .push(name);
                    count += 1;
                }
            }
        }

        // Pass 3: Textual line-by-line fallback for inner fields and registers
        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with('#') || trimmed.is_empty() {
                continue;
            }

            // Detect Root Module: `= [Module]: < Category { Name } >`
            if (trimmed.starts_with("= [Module]:") || trimmed.starts_with("=[Module]:")) && !self.definitions.contains_key("Manifest") {
                if let Some(brace_start) = trimmed.find('{') {
                    if let Some(brace_end) = trimmed[brace_start..].find('}') {
                        let candidate = trimmed[brace_start + 1..brace_start + brace_end].trim();
                        if is_valid_identifier(candidate) && !self.definitions.contains_key(candidate) {
                            let aside = extract_aside_from_str(trimmed).or_else(|| {
                                if idx + 1 < lines.len() {
                                    extract_aside_from_str(lines[idx + 1])
                                } else {
                                    None
                                }
                            });
                            let doc_comments = extract_preceding_comments(&lines, idx);
                            let mut fields = HashMap::new();
                            for l in &lines[idx + 1..idx.saturating_add(35).min(lines.len())] {
                                let l_trimmed = l.trim();
                                if l_trimmed.starts_with('}') {
                                    break;
                                }
                                if let Some(colon) = l_trimmed.find(':') {
                                    let raw_k = l_trimmed[..colon].trim();
                                    let clean_k = raw_k.trim_matches(|c| c == '[' || c == ']' || c == ' ');
                                    let v = l_trimmed[colon + 1..].trim().trim_end_matches(',');
                                    fields.insert(clean_k.to_lowercase(), v.to_string());
                                }
                            }
                            let def = OmniDefinition {
                                name: candidate.to_string(),
                                boundary_type: BoundaryType::Invariant,
                                rank: 1,
                                part_of_speech: "Noun",
                                aside_intent: aside,
                                doc_comments,
                                block_kind: line_blocks[idx],
                                source_file: file_path.to_string(),
                                line: idx + 1,
                                fields,
                                machine_representation: Some("Ontological Essence (< Invariant >)".into()),
                            };
                            self.insert_definition(candidate.to_string(), def);
                            count += 1;
                        }
                    }
                }
            }

            // Detect field patterns: `name: < ... > ( ... )` or `[ Name ]: { ... }` or `name [ ... ] ( ... )`
            if let Some(colon_pos) = trimmed.find(':') {
                let raw_candidate = trimmed[..colon_pos].trim();
                let id_candidate = raw_candidate.trim_matches(|c| c == '[' || c == ']' || c == ' ');
                if is_valid_identifier(id_candidate) && !self.definitions.contains_key(id_candidate) {
                    let rest = trimmed[colon_pos + 1..].trim();
                    let mut btype = detect_boundary_type_from_str(rest);
                    if line_blocks[idx] == Some(BlockKind::Metadata) {
                        btype = BoundaryType::Invariant;
                    }
                    let aside = extract_aside_from_str(rest).or_else(|| {
                        if idx + 1 < lines.len() {
                            extract_aside_from_str(lines[idx + 1])
                        } else {
                            None
                        }
                    });
                    let doc_comments = extract_preceding_comments(&lines, idx);

                    let def = OmniDefinition {
                        name: id_candidate.to_string(),
                        boundary_type: btype,
                        rank: btype.rank(),
                        part_of_speech: "Noun",
                        aside_intent: aside,
                        doc_comments,
                        block_kind: line_blocks[idx],
                        source_file: file_path.to_string(),
                        line: idx + 1,
                        fields: HashMap::new(),
                        machine_representation: Some("Contextual Attribute / Field".into()),
                    };
                    self.insert_definition(id_candidate.to_string(), def);
                    count += 1;
                }
            }

            // Detect standalone registers: `name [ ... ] ( ... )`
            if trimmed.contains('[') && !trimmed.starts_with("roll_") && !trimmed.starts_with("verify:") {
                let id_end = trimmed.find('[').unwrap_or(0);
                let id_candidate = trimmed[..id_end].trim();
                if is_valid_identifier(id_candidate) && !self.definitions.contains_key(id_candidate) {
                    let aside = extract_aside_from_str(trimmed);
                    let doc_comments = extract_preceding_comments(&lines, idx);
                    let def = OmniDefinition {
                        name: id_candidate.to_string(),
                        boundary_type: BoundaryType::Denotation,
                        rank: BoundaryType::Denotation.rank(),
                        part_of_speech: "Noun",
                        aside_intent: aside,
                        doc_comments,
                        block_kind: line_blocks[idx],
                        source_file: file_path.to_string(),
                        line: idx + 1,
                        fields: HashMap::new(),
                        machine_representation: Some("Active State Register".into()),
                    };
                    self.insert_definition(id_candidate.to_string(), def);
                    count += 1;
                }
            }
        }

        self.logger.log(
            "HOVER",
            format!("Indexed {} symbols from .omni source: {}", count, file_path),
        );
        Ok(count)
    }

    /// Indexes a specific file from disk.
    pub fn index_file(&mut self, path: impl AsRef<Path>) -> Result<usize, String> {
        let p = path.as_ref();
        let content = fs::read_to_string(p)
            .map_err(|e| format!("Failed to read file {:?}: {e}", p))?;
        let path_str = p.to_string_lossy().to_string();
        self.index_omni_source(&path_str, &content)
    }

    /// Recursively indexes foundational files and standard libraries across the workspace.
    pub fn index_workspace(&mut self, root_dir: impl AsRef<Path>) {
        let root = root_dir.as_ref();

        // 1. Core manifest
        let manifest_path = root.join("manifest.omni");
        if manifest_path.exists() {
            let _ = self.index_file(&manifest_path);
        }

        // 2. Standard library modules in lib/core/
        let lib_core = root.join("lib/core");
        if lib_core.exists() && lib_core.is_dir() {
            if let Ok(entries) = fs::read_dir(lib_core) {
                for entry in entries.flatten() {
                    let p = entry.path();
                    if p.extension().map_or(false, |ext| ext == "omni") {
                        let _ = self.index_file(&p);
                    }
                }
            }
        }
    }

    /// Helper to store definitions in both exact and lowercase keys for flexible lookup.
    fn insert_definition(&mut self, key: String, def: OmniDefinition) {
        self.definitions.insert(key.to_lowercase(), def.clone());
        self.definitions.insert(key, def);
    }

    /// Resolves hover information for a symbol used inside a high-level `.tape` process ribbon.
    ///
    /// # Resolution Sequence
    /// 1. Check if defined or redefined in the local `.tape` ribbon.
    /// 2. If companion `.omni` exists (e.g. `foo.tape` -> `foo.omni`), resolve from companion.
    /// 3. Otherwise, resolve from the workspace symbol index (stdlib or manifest).
    /// 4. Generate the composite 3-tier Markdown HoverCard.
    pub fn resolve_symbol_for_tape(
        &mut self,
        tape_path: &str,
        symbol: &str,
        line_number: usize,
    ) -> Option<HoverCard> {
        let tape_p = Path::new(tape_path);

        // Step 1: Ensure companion .omni is indexed if it exists
        let companion_omni = tape_p.with_extension("omni");
        if companion_omni.exists() {
            let _ = self.index_file(&companion_omni);
        }

        // Step 2: Load and inspect the active Tape
        let tape = Tape::from_file(tape_p, self.logger.clone()).ok();

        // Extract Tape Context for this roll
        let mut tape_context = None;
        let mut is_locally_redefined = false;

        if let Some(ref t) = tape {
            let track_str = if tape_path.contains("kernel") || tape_path.contains("boot") {
                "Track 0 [KERNEL]"
            } else if tape_path.contains("terminal") {
                "Track 1 [TERMINAL]"
            } else {
                "Track 2 [APP]"
            };

            // Find roll matching line number that contains symbol, or matching line number, or first roll referencing symbol
            let matched_roll = t
                .rolls
                .iter()
                .find(|r| r.line_number == line_number && r.text.contains(symbol))
                .or_else(|| t.rolls.iter().find(|r| r.line_number == line_number))
                .or_else(|| t.rolls.iter().find(|r| r.text.contains(symbol)));

            if let Some(r) = matched_roll {
                let roll_aside = extract_aside_from_str(&r.text);

                tape_context = Some(TapeRollContext {
                    tape_name: t.head.name.clone(),
                    track: track_str.to_string(),
                    roll_number: r.index + 1,
                    total_rolls: t.roll_count(),
                    raw_roll_text: r.text.clone(),
                    roll_aside_intent: roll_aside,
                    head_who: t.head.who.clone(),
                    head_what: t.head.what.clone(),
                    head_why: t.head.why.clone(),
                    head_how: t.head.how.clone(),
                    partition_size: t.head.required_partition_size,
                });
            }

            // Check if symbol was given a Denotation anywhere on this tape ribbon: `symbol [ ... ]`
            is_locally_redefined = t.rolls.iter().any(|r| {
                r.text.contains(&format!("{symbol} [")) || r.text.contains(&format!("{symbol}["))
            });
        }

        // Step 3: Lookup low-level .omni definition
        let omni_def = self
            .definitions
            .get(symbol)
            .cloned()
            .or_else(|| self.definitions.get(&symbol.to_lowercase()).cloned());

        // Build Markdown card
        let markdown = self.format_composite_markdown(
            symbol,
            tape_context.as_ref(),
            omni_def.as_ref(),
            is_locally_redefined,
        );

        Some(HoverCard {
            symbol: symbol.to_string(),
            tape_context,
            omni_definition: omni_def,
            is_locally_redefined,
            markdown,
        })
    }

    /// Resolves hover information at a specific file coordinate `(line, col)`.
    pub fn hover_at(&mut self, file_path: &str, line: usize, col: usize) -> Option<HoverCard> {
        let content = fs::read_to_string(file_path).ok()?;
        let lines: Vec<&str> = content.lines().collect();
        if line == 0 || line > lines.len() {
            return None;
        }

        let target_line = lines[line - 1];
        let symbol = extract_word_at_col(target_line, col)?;

        if file_path.ends_with(".tape") {
            self.resolve_symbol_for_tape(file_path, &symbol, line)
        } else {
            // Index the file itself if not already done
            let _ = self.index_omni_source(file_path, &content);
            let def = self
                .definitions
                .get(&symbol)
                .cloned()
                .or_else(|| self.definitions.get(&symbol.to_lowercase()).cloned());

            let markdown = self.format_omni_only_markdown(&symbol, def.as_ref());
            Some(HoverCard {
                symbol: symbol.clone(),
                tape_context: None,
                omni_definition: def,
                is_locally_redefined: false,
                markdown,
            })
        }
    }

    /// Formats the complete 3-Tier Markdown HoverCard bridging .tape down to .omni.
    fn format_composite_markdown(
        &self,
        symbol: &str,
        tape_ctx: Option<&TapeRollContext>,
        omni_def: Option<&OmniDefinition>,
        is_redefined: bool,
    ) -> String {
        let mut md = String::new();

        // ----------------------------------------------------------------------
        // HEADER: Identity & Morphological Badging
        // ----------------------------------------------------------------------
        let rank_badge = if let Some(def) = omni_def {
            format!("`{}`", def.boundary_type.rank_name())
        } else if is_redefined {
            "`Rank 2: Denotative Essence [ ... ]`".to_string()
        } else {
            "`Process Tape Entity`".to_string()
        };

        md.push_str(&format!("### `{symbol}` — {}\n\n", rank_badge));

        // ----------------------------------------------------------------------
        // TIER 1: HIGH-LEVEL PROCESS TAPE ROLL (Process Intent & Active Sentence)
        // ----------------------------------------------------------------------
        if let Some(ctx) = tape_ctx {
            md.push_str("#### 📜 1. High-Level Process Tape View\n");
            md.push_str(&format!(
                "* **Process Ribbon:** `{}` on `{}`\n",
                ctx.tape_name, ctx.track
            ));
            md.push_str(&format!(
                "* **Active Roll ({}/{}):** `{}`\n",
                ctx.roll_number, ctx.total_rolls, ctx.raw_roll_text
            ));

            if let Some(ref intent) = ctx.roll_aside_intent {
                md.push_str(&format!("* **Roll Intent Aside:** *\"{}\"*\n", intent));
            }

            md.push_str(&format!(
                "* **Tape Head Covenant:** WHO `{}` $\\to$ WHAT `{}` (Purpose: *\"{}\"*)\n",
                ctx.head_who, ctx.head_what, ctx.head_why
            ));

            if is_redefined {
                md.push_str(
                    "\n> [!NOTE]\n> **Locally Defined:** This entity receives a concrete Denotation `[ ... ]` in this tape ribbon, instantiating it into active memory.\n",
                );
            }
            md.push_str("\n---\n\n");
        }

        // ----------------------------------------------------------------------
        // TIER 2: UNDER-THE-HOOD LOW-LEVEL BLUEPRINT (.omni Blueprint & Contract)
        // ----------------------------------------------------------------------
        md.push_str("#### 🔍 2. Under-The-Hood (.omni Blueprint)\n");
        if let Some(def) = omni_def {
            let block_name = def
                .block_kind
                .map(|b| b.name())
                .unwrap_or("Declarative Scope");
            md.push_str(&format!(
                "* **Grounded Source:** [`{}`]({}) (Line {})\n",
                def.source_file, def.source_file, def.line
            ));
            md.push_str(&format!("* **4-Block Origin:** `{block_name}`\n"));
            md.push_str(&format!(
                "* **Ontological Hierarchy:** Rank {} (`{}`)\n",
                def.rank, def.boundary_type
            ));

            if let Some(ref intent) = def.aside_intent {
                md.push_str(&format!("* **Contract Intent Aside:** *\"{}\"*\n", intent));
            }

            if !def.doc_comments.is_empty() {
                md.push_str("* **Architectural Description:**\n");
                for comment in &def.doc_comments {
                    md.push_str(&format!("  > {comment}\n"));
                }
            }

            if !def.fields.is_empty() {
                md.push_str("* **Sub-Attributes & Invariant Axioms:**\n");
                for (k, v) in &def.fields {
                    md.push_str(&format!("  * `{k}`: `{v}`\n"));
                }
            }
        } else {
            md.push_str("* **Under-The-Hood Status:** Pure runtime process entity; inherits standard library defaults.\n");
        }
        md.push_str("\n---\n\n");

        // ----------------------------------------------------------------------
        // TIER 3: MACHINE EXECUTION & SILICON EFFECT
        // ----------------------------------------------------------------------
        md.push_str("#### ⚙️ 3. Machine Execution & Silicon State\n");
        let partition = tape_ctx
            .map(|c| format!("{} cells", c.partition_size))
            .unwrap_or_else(|| "1024 cells".to_string());

        md.push_str(&format!(
            "* **Memory Allocation:** Partition reservation ceiling: `{partition}`\n"
        ));
        md.push_str("* **Transducer Codec:** Dense byte $\\longleftrightarrow$ 5 balanced trits ($[-121 \\dots +121]$ Radix-3)\n");
        md.push_str("* **Execution Truth State:** `+1: PROCEED` (Deterministic commit, transactional rollback protection)\n");

        md
    }

    /// Formats a hover card when hovering directly inside a `.omni` source reel.
    fn format_omni_only_markdown(&self, symbol: &str, def: Option<&OmniDefinition>) -> String {
        let mut md = String::new();
        if let Some(d) = def {
            let block_name = d.block_kind.map(|b| b.name()).unwrap_or("Universal Scope");
            md.push_str(&format!(
                "### `{}` — `{}`\n\n",
                d.name,
                d.boundary_type.rank_name()
            ));
            md.push_str(&format!("* **Origin:** `{block_name}` (Line {})\n", d.line));
            md.push_str(&format!("* **Part of Speech:** `{}`\n", d.part_of_speech));

            if let Some(ref intent) = d.aside_intent {
                md.push_str(&format!("* **Machine Intent Aside:** *\"{}\"*\n", intent));
            }

            if !d.doc_comments.is_empty() {
                md.push_str("\n#### Architectural Docstring:\n");
                for c in &d.doc_comments {
                    md.push_str(&format!("> {c}\n"));
                }
            }

            if !d.fields.is_empty() {
                md.push_str("\n#### Invariant Fields:\n");
                for (k, v) in &d.fields {
                    md.push_str(&format!("* `{k}`: `{v}`\n"));
                }
            }
        } else {
            md.push_str(&format!("### `{symbol}`\n\n"));
            md.push_str("* **Classification:** OmniCode Identifier\n");
            md.push_str("* **Scope:** Ambient Context\n");
        }
        md
    }

    /// Internal helper converting an AST EntityNode into an OmniDefinition.
    fn build_omni_definition(
        &self,
        entity: &EntityNode,
        file_path: &str,
        lines: &[&str],
        line_blocks: &[Option<BlockKind>],
    ) -> OmniDefinition {
        let line_idx = entity.line.saturating_sub(1);
        let block_kind = line_blocks.get(line_idx).copied().flatten();
        let doc_comments = extract_preceding_comments(lines, line_idx);
        let btype = entity.definition.boundary_type;

        let mut fields = HashMap::new();
        // Extract subfields if this was a record with key-value entries
        if line_idx < lines.len() {
            for l in &lines[line_idx..line_idx.saturating_add(15).min(lines.len())] {
                let trimmed = l.trim();
                if trimmed.starts_with('}') || trimmed.starts_with(']') {
                    break;
                }
                if let Some(colon) = trimmed.find(':') {
                    let k = trimmed[..colon].trim();
                    let v = trimmed[colon + 1..].trim().trim_end_matches(',');
                    if is_valid_identifier(k) {
                        fields.insert(k.to_string(), v.to_string());
                    }
                }
            }
        }

        let aside_intent = entity
            .aside
            .as_ref()
            .or(entity.definition.aside.as_ref())
            .or(entity.context.as_ref().and_then(|c| c.aside.as_ref()))
            .map(|a| a.intent.clone())
            .or_else(|| {
                if line_idx < lines.len() {
                    extract_aside_from_str(lines[line_idx])
                } else {
                    None
                }
            });

        OmniDefinition {
            name: entity.name.clone(),
            boundary_type: btype,
            rank: btype.rank(),
            part_of_speech: "Noun",
            aside_intent,
            doc_comments,
            block_kind,
            source_file: file_path.to_string(),
            line: entity.line,
            fields,
            machine_representation: Some(format!("Ontological Essence ({})", btype)),
        }
    }
}


// ------------------------------------------------------------------------------
// HELPER UTILITIES: Token extraction, comments, and delimiters
// ------------------------------------------------------------------------------

fn escape_json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out
}

fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let first = s.chars().next().unwrap();
    (first.is_alphabetic() || first == '_') && s.chars().all(|c| c.is_alphanumeric() || c == '_')
}

fn extract_word_at_col(line: &str, col: usize) -> Option<String> {
    if line.is_empty() {
        return None;
    }
    let chars: Vec<char> = line.chars().collect();
    let idx = col.saturating_sub(1).min(chars.len().saturating_sub(1));

    if !chars[idx].is_alphanumeric() && chars[idx] != '_' {
        return None;
    }

    let mut start = idx;
    while start > 0 && (chars[start - 1].is_alphanumeric() || chars[start - 1] == '_') {
        start -= 1;
    }

    let mut end = idx;
    while end + 1 < chars.len() && (chars[end + 1].is_alphanumeric() || chars[end + 1] == '_') {
        end += 1;
    }

    Some(chars[start..=end].iter().collect())
}

fn extract_aside_from_str(text: &str) -> Option<String> {
    let start = text.find('(')?;
    let end = text.rfind(')')?;
    if end > start {
        Some(text[start + 1..end].trim().to_string())
    } else {
        None
    }
}

fn detect_boundary_type_from_str(s: &str) -> BoundaryType {
    if s.contains('<') && s.contains('>') {
        BoundaryType::Invariant
    } else if s.contains('{') && s.contains('}') {
        BoundaryType::Connotation
    } else if s.contains('{') && s.contains(']') {
        BoundaryType::TopDown
    } else if s.contains('[') && s.contains('}') {
        BoundaryType::BottomUp
    } else {
        BoundaryType::Denotation
    }
}

fn extract_preceding_comments(lines: &[&str], line_idx: usize) -> Vec<String> {
    let mut comments = Vec::new();
    let mut cur = line_idx;
    while cur > 0 {
        cur -= 1;
        let trimmed = lines[cur].trim();
        if trimmed.starts_with('#') {
            let clean = trimmed.trim_start_matches('#').trim();
            // Ignore boundary separator bars: ====== or ------
            if !clean.starts_with('=') && !clean.starts_with('-') && !clean.is_empty() {
                comments.push(clean.to_string());
            }
        } else {
            break;
        }
    }
    comments.reverse();
    comments
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests, Verification & Subsystem Sealing )
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
        assert_eq!(law_def.boundary_type, BoundaryType::Invariant);
        assert_eq!(law_def.rank, 1);
        assert_eq!(law_def.block_kind, Some(BlockKind::Metadata));
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

        // Temporarily write tape file to test directory
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
}

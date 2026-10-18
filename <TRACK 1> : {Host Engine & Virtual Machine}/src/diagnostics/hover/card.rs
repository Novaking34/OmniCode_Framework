// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Hover Card Data Models, 3-Tier Markdown Formatter & Serialization )
// ==============================================================================
// SCRIPTURAL WITNESS:
//   "The words of the wise are as goads, and as nails fastened by the masters
//    of assemblies, which are given from one shepherd."
//   — Ecclesiastes 12:11 (KJV)
//
// ARCHITECTURAL TOPOLOGY:
//   Defines the core data models representing inspected symbols across both
//   declarative specifications (.omni) and operational process streams (.tape):
//     - OmniDefinition: Low-level type, boundary rank, docstrings, and fields.
//     - TapeRollContext: High-level roll index, 5-Ws cardinal clauses, and asides.
//     - HoverCard: Composite 3-tier presentation card for IDE tooltips and terminal.
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Imports, Standard Collections & Auxiliary Formatting Utilities )
// ==============================================================================
use std::collections::HashMap;

use crate::core::boundaries::BoundaryType;
use crate::diagnostics::linter::BlockKind;

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
    /// Structural doc comments (`#` or `//`) associated with this entity.
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

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Card Formatting: JSON Serialization, ANSI Terminal & 3-Tier Markdown )
// ==============================================================================
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

                let fields_entries = od
                    .fields
                    .iter()
                    .map(|(k, v)| format!("\"{}\":\"{}\"", escape_json_str(k), escape_json_str(v)))
                    .collect::<Vec<_>>()
                    .join(",");

                let machine_str = match &od.machine_representation {
                    Some(m) => format!("\"{}\"", escape_json_str(m)),
                    None => "null".to_string(),
                };

                format!(
                    r#"{{"name":"{}","boundary_type":"{}","rank":{},"part_of_speech":"{}","aside_intent":{},"doc_comments":[{}],"block_kind":{},"source_file":"{}","line":{},"fields":{{{}}},"machine_representation":{}}}"#,
                    escape_json_str(&od.name),
                    escape_json_str(&od.boundary_type.to_string()),
                    od.rank,
                    escape_json_str(od.part_of_speech),
                    aside_str,
                    doc_comments_arr,
                    block_str,
                    escape_json_str(&od.source_file),
                    od.line,
                    fields_entries,
                    machine_str,
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
            escape_json_str(&self.markdown)
        )
    }

    /// Renders an ANSI-styled human terminal presentation of the 3-tier hover card.
    pub fn render_terminal(&self) -> String {
        let mut out = String::new();
        out.push_str("\x1b[1;36m===============================================================================\x1b[0m\n");
        out.push_str(&format!(
            "  \x1b[1;33mOmniCode Symbol Inspection:\x1b[0m \x1b[1;37m{}\x1b[0m\n",
            self.symbol
        ));
        out.push_str("\x1b[1;36m===============================================================================\x1b[0m\n");

        if let Some(ctx) = &self.tape_context {
            out.push_str("\x1b[1;34m[TIER 1: HIGH-LEVEL PROCESS TAPE ROLL]\x1b[0m\n");
            out.push_str(&format!(
                "  • Process Ribbon:       \x1b[32m{}\x1b[0m on \x1b[35m{}\x1b[0m\n",
                ctx.tape_name, ctx.track
            ));
            out.push_str(&format!(
                "  • Roll Execution ({} of {}): \x1b[37m{}\x1b[0m\n",
                ctx.roll_number, ctx.total_rolls, ctx.raw_roll_text
            ));
            if let Some(ref aside) = ctx.roll_aside_intent {
                out.push_str(&format!("  • Roll Intent Aside:    \x1b[3m\"{aside}\"\x1b[0m\n"));
            }
            out.push_str(&format!(
                "  • Tape Head Covenant:   WHO \x1b[36m{}\x1b[0m -> WHAT \x1b[36m{}\x1b[0m\n",
                ctx.head_who, ctx.head_what
            ));
            out.push_str(&format!(
                "  • Covenant Purpose:     \x1b[3m\"{}\"\x1b[0m\n",
                ctx.head_why
            ));
            out.push_str(&format!(
                "  • Spatial Partition:    {} cells\n\n",
                ctx.partition_size
            ));
        }

        out.push_str("\x1b[1;34m[TIER 2: UNDER-THE-HOOD .OMNI BLUEPRINT]\x1b[0m\n");
        if let Some(def) = &self.omni_definition {
            let block_name = def
                .block_kind
                .map(|b| b.name())
                .unwrap_or("Declarative Scope");
            out.push_str(&format!(
                "  • Grounded Source:      \x1b[36m{}:{}\x1b[0m [{block_name}]\n",
                def.source_file, def.line
            ));
            out.push_str(&format!(
                "  • Ontological Rank:     Rank {} ({})\n",
                def.rank, def.boundary_type
            ));
            out.push_str(&format!(
                "  • Grammatical Category: {}\n",
                def.part_of_speech
            ));

            if let Some(ref aside) = def.aside_intent {
                out.push_str(&format!("  • Contract Intent Aside:\x1b[3m\"{aside}\"\x1b[0m\n"));
            }

            if !def.doc_comments.is_empty() {
                out.push_str("  • Architectural Description:\n");
                for comment in &def.doc_comments {
                    out.push_str(&format!("    > {comment}\n"));
                }
            }

            if !def.fields.is_empty() {
                out.push_str("  • Invariant Attributes:\n");
                for (k, v) in &def.fields {
                    out.push_str(&format!("    * {k}: {v}\n"));
                }
            }
        } else {
            out.push_str("  • Definition Status:    Inherited Standard Runtime Entity (lib/core)\n");
        }
        out.push('\n');

        out.push_str("\x1b[1;34m[TIER 3: MACHINE EXECUTION & SILICON FOOTPRINT]\x1b[0m\n");
        let partition = self
            .tape_context
            .as_ref()
            .map(|c| format!("{} cells", c.partition_size))
            .unwrap_or_else(|| "1024 cells".to_string());

        out.push_str(&format!("  • Memory Allocation:     {partition}\n"));
        out.push_str("  • Transducer Codec:      Dense Byte <-> 5 Balanced Trits (Radix-3 [-121..+121])\n");
        out.push_str("  • Silicon Truth State:   \x1b[1;32m+1:PROCEED\x1b[0m (Deterministic commit, transactional rollback)\n");

        out
    }
}

/// Formats the complete 3-Tier Markdown HoverCard bridging high-level `.tape` down to `.omni`.
pub fn format_composite_markdown(
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
pub fn format_omni_only_markdown(symbol: &str, def: Option<&OmniDefinition>) -> String {
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

/// Escapes special characters for JSON output.
pub fn escape_json_str(s: &str) -> String {
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

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Card Data Model Unit Tests & Affirmative Invariant Sealing )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hover_card_json_serialization_round_trip() {
        let card = HoverCard {
            symbol: "Law_Truth".to_string(),
            tape_context: None,
            omni_definition: Some(OmniDefinition {
                name: "Law_Truth".to_string(),
                boundary_type: BoundaryType::Invariant,
                rank: 1,
                part_of_speech: "Noun",
                aside_intent: Some("Prohibit deception".to_string()),
                doc_comments: vec!["LAW 1: TRUTH".to_string()],
                block_kind: Some(BlockKind::Metadata),
                source_file: "manifest.omni".to_string(),
                line: 10,
                fields: {
                    let mut m = HashMap::new();
                    m.insert("commandment".to_string(), "Thou shalt not lie".to_string());
                    m
                },
                machine_representation: Some("Ontological Essence (< Invariant >)".to_string()),
            }),
            is_locally_redefined: false,
            markdown: "### `Law_Truth`\n".to_string(),
        };

        let json = card.to_json();
        assert!(json.contains("\"symbol\":\"Law_Truth\""));
        assert!(json.contains("\"boundary_type\":\"Invariant(<...>)\""));
        assert!(json.contains("\"rank\":1"));
        assert!(json.contains("\"block_kind\":\"BLOCK 1: METADATA\""));
        assert!(json.contains("\"commandment\":\"Thou shalt not lie\""));
    }

    #[test]
    fn test_hover_card_ansi_terminal_rendering() {
        let card = HoverCard {
            symbol: "Character".to_string(),
            tape_context: None,
            omni_definition: None,
            is_locally_redefined: false,
            markdown: "### `Character`".to_string(),
        };

        let term = card.render_terminal();
        assert!(term.contains("OmniCode Symbol Inspection"));
        assert!(term.contains("Character"));
        assert!(term.contains("[TIER 2: UNDER-THE-HOOD .OMNI BLUEPRINT]"));
        assert!(term.contains("[TIER 3: MACHINE EXECUTION & SILICON FOOTPRINT]"));
    }
}

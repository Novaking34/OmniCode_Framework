// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Cross-Scale Hover Coordinate Resolver & Tape-to-Omni Linker )
// ==============================================================================
// SCRIPTURAL WITNESS:
//   "According to the grace of God which is given unto me, as a wise masterbuilder,
//    I have laid the foundation, and another buildeth thereon."
//   — 1 Corinthians 3:10 (KJV)
//
// ARCHITECTURAL TOPOLOGY:
//   Bridges high-level process rolls (.tape) down to low-level specifications (.omni)
//   and resolves exact symbol coordinates (line, col) for IDE hover tooltips:
//     - extracts word/token at cursor position.
//     - companion file resolution (foo.tape -> foo.omni).
//     - links tape head covenant, roll context, and silicon representation.
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Imports, Standard Collections, Path & Core Dependencies )
// ==============================================================================
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::core::tape::Tape;
use crate::diagnostics::hover::card::{
    format_composite_markdown, format_omni_only_markdown, HoverCard, OmniDefinition, TapeRollContext,
};
use crate::diagnostics::hover::indexer::{extract_aside_from_str, index_omni_source_text};
use crate::diagnostics::logger::DiagnosticLogger;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Coordinate Resolution, Companion File Linking & Card Generation )
// ==============================================================================

/// Extracts the identifier/word at the specified column in a line of text (1-indexed).
pub fn extract_word_at_col(line: &str, col: usize) -> Option<String> {
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

/// Resolves hover information for a symbol referenced inside a `.tape` process ribbon.
pub fn resolve_symbol_for_tape(
    tape_path: &str,
    symbol: &str,
    line_number: usize,
    logger: &DiagnosticLogger,
    definitions: &mut HashMap<String, OmniDefinition>,
    file_symbols: &mut HashMap<String, Vec<String>>,
) -> Option<HoverCard> {
    let tape_p = Path::new(tape_path);

    // Step 1: Ensure companion .omni is indexed if it exists
    let companion_omni = tape_p.with_extension("omni");
    if companion_omni.exists() {
        if let Ok(content) = fs::read_to_string(&companion_omni) {
            let _ = index_omni_source_text(
                companion_omni.to_str().unwrap_or(""),
                &content,
                logger,
                definitions,
                file_symbols,
            );
        }
    }

    // Step 2: Load and inspect the active Tape
    let tape = Tape::from_file(tape_p, logger.clone()).ok();

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
    let omni_def = definitions
        .get(symbol)
        .cloned()
        .or_else(|| definitions.get(&symbol.to_lowercase()).cloned());

    // Build Markdown card
    let markdown = format_composite_markdown(
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
pub fn resolve_hover_at(
    file_path: &str,
    line: usize,
    col: usize,
    logger: &DiagnosticLogger,
    definitions: &mut HashMap<String, OmniDefinition>,
    file_symbols: &mut HashMap<String, Vec<String>>,
) -> Option<HoverCard> {
    let content = fs::read_to_string(file_path).ok()?;
    let lines: Vec<&str> = content.lines().collect();
    if line == 0 || line > lines.len() {
        return None;
    }

    let target_line = lines[line - 1];
    let symbol = extract_word_at_col(target_line, col)?;

    if file_path.ends_with(".tape") {
        resolve_symbol_for_tape(
            file_path,
            &symbol,
            line,
            logger,
            definitions,
            file_symbols,
        )
    } else {
        // Index the file itself if not already done
        let _ = index_omni_source_text(file_path, &content, logger, definitions, file_symbols);
        let def = definitions
            .get(&symbol)
            .cloned()
            .or_else(|| definitions.get(&symbol.to_lowercase()).cloned());

        let markdown = format_omni_only_markdown(&symbol, def.as_ref());
        Some(HoverCard {
            symbol,
            tape_context: None,
            omni_definition: def,
            is_locally_redefined: false,
            markdown,
        })
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Resolver Unit Tests & Affirmative Invariant Sealing )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_word_at_col() {
        let line = "    let mut Accumulator [ 0 : Trit ]";
        // Col on 'Accumulator'
        let word = extract_word_at_col(line, 15).expect("Must extract word");
        assert_eq!(word, "Accumulator");

        // Col on whitespace
        assert!(extract_word_at_col(line, 1).is_none());

        // Col on 'Trit'
        let word_trit = extract_word_at_col(line, 31).expect("Must extract Trit");
        assert_eq!(word_trit, "Trit");
    }
}

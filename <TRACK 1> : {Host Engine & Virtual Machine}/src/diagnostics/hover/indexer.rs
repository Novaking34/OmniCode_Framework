// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Multi-Pass Cross-File Symbol Indexer for .omni Specifications )
// ==============================================================================
// SCRIPTURAL WITNESS:
//   "He telleth the number of the stars; he calleth them all by their names."
//   — Psalm 147:4 (KJV)
//
// ARCHITECTURAL TOPOLOGY:
//   Extracts ontological entities, boundary brackets, attached Intent Asides,
//   docstrings, and 4-block origins from raw .omni source code:
//     - Pass 1: Line-by-line 4-Block boundary layout scanner.
//     - Pass 2: AST entity extraction via Lexer and Parser.
//     - Pass 3: Textual fallback scanner for inner record fields, registers,
//       root module declarations, and Intent Asides.
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Imports, Standard Collections & Invariant Parsing Dependencies )
// ==============================================================================
use std::collections::HashMap;

use crate::ast::nodes::{ASTNode, EntityNode};
use crate::core::boundaries::BoundaryType;
use crate::diagnostics::hover::card::OmniDefinition;
use crate::diagnostics::linter::BlockKind;
use crate::diagnostics::logger::DiagnosticLogger;
use crate::parser::lexer::Lexer;
use crate::parser::parser::Parser;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Indexing Algorithms, Block Mapping & Fallback Scanners )
// ==============================================================================

/// Indexes a `.omni` source reel string, extracting all entities, boundaries,
/// docstrings, attached Asides, and block origins.
pub fn index_omni_source_text(
    file_path: &str,
    content: &str,
    logger: &DiagnosticLogger,
    definitions: &mut HashMap<String, OmniDefinition>,
    file_symbols: &mut HashMap<String, Vec<String>>,
) -> Result<usize, String> {
    let lines: Vec<&str> = content.lines().collect();
    let mut current_block = None;
    let mut count = 0;

    // Pass 1: Parse 4-block layout across lines
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
    let mut lexer = Lexer::new(content, logger.clone());
    let tokens = lexer.tokenize()?;
    let debugger = crate::diagnostics::debugger::OmniDebugger::new(logger.clone(), 10);
    let mut parser = Parser::new(tokens, debugger);

    // Best-effort parse program
    if let Ok(prog) = parser.parse_program() {
        for stmt in &prog.statements {
            if let ASTNode::Entity(entity) = stmt {
                let def = build_omni_definition(entity, file_path, &lines, &line_blocks);
                let name = def.name.clone();
                definitions.insert(name.to_lowercase(), def.clone());
                definitions.insert(name.clone(), def);
                file_symbols
                    .entry(file_path.to_string())
                    .or_default()
                    .push(name);
                count += 1;
            }
        }
    }

    // Pass 3: Textual line-by-line fallback for inner fields, registers, and module headers
    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') || trimmed.starts_with("//") || trimmed.is_empty() {
            continue;
        }

        // Detect Root Module: `= [Module]: < Category { Name } >`
        if (trimmed.starts_with("= [Module]:") || trimmed.starts_with("=[Module]:"))
            && !definitions.contains_key("Manifest")
        {
            if let Some(brace_start) = trimmed.find('{') {
                if let Some(brace_end) = trimmed[brace_start..].find('}') {
                    let candidate = trimmed[brace_start + 1..brace_start + brace_end].trim();
                    if is_valid_identifier(candidate) && !definitions.contains_key(candidate) {
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
                                let clean_k =
                                    raw_k.trim_matches(|c| c == '[' || c == ']' || c == ' ');
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
                        definitions.insert(candidate.to_lowercase(), def.clone());
                        definitions.insert(candidate.to_string(), def);
                        count += 1;
                    }
                }
            }
        }

        // Detect field patterns: `name: < ... > ( ... )` or `[ Name ]: { ... }` or `name [ ... ] ( ... )`
        if let Some(colon_pos) = trimmed.find(':') {
            let raw_candidate = trimmed[..colon_pos].trim();
            let id_candidate = raw_candidate.trim_matches(|c| c == '[' || c == ']' || c == ' ');
            if is_valid_identifier(id_candidate) && !definitions.contains_key(id_candidate) {
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
                definitions.insert(id_candidate.to_lowercase(), def.clone());
                definitions.insert(id_candidate.to_string(), def);
                count += 1;
            }
        }

        // Detect standalone registers & bit rails: `name [ ... ] ( ... )`
        if trimmed.contains('[')
            && !trimmed.starts_with("roll_")
            && !trimmed.starts_with("verify:")
            && !trimmed.starts_with("BLOCK")
        {
            let id_end = trimmed.find('[').unwrap_or(0);
            let id_candidate = trimmed[..id_end].trim();
            if is_valid_identifier(id_candidate) && !definitions.contains_key(id_candidate) {
                let aside = extract_aside_from_str(trimmed).or_else(|| {
                    if idx + 1 < lines.len() {
                        extract_aside_from_str(lines[idx + 1])
                    } else {
                        None
                    }
                });
                let doc_comments = extract_preceding_comments(&lines, idx);

                let def = OmniDefinition {
                    name: id_candidate.to_string(),
                    boundary_type: BoundaryType::Denotation,
                    rank: 2,
                    part_of_speech: "Noun",
                    aside_intent: aside,
                    doc_comments,
                    block_kind: line_blocks[idx],
                    source_file: file_path.to_string(),
                    line: idx + 1,
                    fields: HashMap::new(),
                    machine_representation: Some("Physical Register / Memory Cell".into()),
                };
                definitions.insert(id_candidate.to_lowercase(), def.clone());
                definitions.insert(id_candidate.to_string(), def);
                count += 1;
            }
        }
    }

    Ok(count)
}

/// Converts an AST EntityNode into a structured OmniDefinition.
pub fn build_omni_definition(
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

/// Validates whether a token is a valid identifier (alphanumeric or underscore).
pub fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let first = s.chars().next().unwrap();
    (first.is_alphabetic() || first == '_') && s.chars().all(|c| c.is_alphanumeric() || c == '_')
}

/// Extracts an Intent Aside `(...)` from raw line text.
pub fn extract_aside_from_str(text: &str) -> Option<String> {
    let start = text.find('(')?;
    let end = text.rfind(')')?;
    if end > start {
        Some(text[start + 1..end].trim().to_string())
    } else {
        None
    }
}

/// Detects the BoundaryType from delimiters contained in a string.
pub fn detect_boundary_type_from_str(s: &str) -> BoundaryType {
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

/// Extracts preceding comment lines `#` or `//` as docstrings.
pub fn extract_preceding_comments(lines: &[&str], line_idx: usize) -> Vec<String> {
    let mut comments = Vec::new();
    let mut cur = line_idx;
    while cur > 0 {
        cur -= 1;
        let trimmed = lines[cur].trim();
        if trimmed.starts_with('#') || trimmed.starts_with("//") {
            let clean = if trimmed.starts_with("//") {
                trimmed.trim_start_matches('/').trim()
            } else {
                trimmed.trim_start_matches('#').trim()
            };
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
// ( Indexer Unit Tests & Affirmative Invariant Sealing )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexer_extracts_entities_and_asides() {
        let logger = DiagnosticLogger::new(false);
        let mut defs = HashMap::new();
        let mut symbols = HashMap::new();

        let source = r#"
# ==============================================================================
# BLOCK 1: METADATA BLOCK
# ==============================================================================
# System Version
Version < "1.0.0" > ( Root system version invariant )

# ==============================================================================
# BLOCK 2: OPENING BLOCK
# ==============================================================================
Bit_0 [ 0 : Trit ] ( Least significant bit rail )
        "#;

        let count = index_omni_source_text("test.omni", source, &logger, &mut defs, &mut symbols)
            .expect("Indexing must succeed");

        assert!(count >= 2);
        assert!(defs.contains_key("version"));
        assert!(defs.contains_key("bit_0"));

        let version_def = defs.get("version").unwrap();
        assert_eq!(version_def.boundary_type, BoundaryType::Invariant);
        assert_eq!(version_def.rank, 1);
        assert!(version_def.aside_intent.as_ref().unwrap().contains("version invariant"));

        let bit_def = defs.get("bit_0").unwrap();
        assert_eq!(bit_def.boundary_type, BoundaryType::Denotation);
        assert_eq!(bit_def.rank, 2);
        assert!(bit_def.aside_intent.as_ref().unwrap().contains("bit rail"));
    }
}

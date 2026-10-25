// =============================================================================
// BLOCK 1: METADATA BLOCK
// ( Identity, Provenance, Invariant Boundary Contracts & Scriptural Anchors )
// =============================================================================
// @module        OmniCode AI Subsystem: Pattern Extraction & Morphology
// @file          patterns.rs
// @layer         Track 1 [Host Engine & Virtual Machine] -> src/ai/transducer
// @version       1.0.0
// @epoch         2026.10
// @author        CreativeWorkzStudio LLC / ProfessorSeanEX
// @system        OmniCode AI Cortex & Living Kernel
// @repository    https://github.com/Novaking34/OmniCode_Framework
// @role          Natural Discourse Parsing, Noun Phrases & Morphological Units
// @invariants    Rank 1 (< >): Form precedes fullness. Patterns extract
//                invariant agents, vessels, containers, and arithmetic clauses
//                grounded in the 8 parts of speech without statistical drift.
// @witness       "A word fitly spoken is like apples of gold in pictures of
//                 silver." — Proverbs 25:11 (KJV)
//                "Hold fast the form of sound words, which thou hast heard
//                 of me, in faith and love..." — 2 Timothy 1:13 (KJV)
//
// THE NOUN PHRASE & MORPHOLOGICAL EXTRACTION PIPELINE:
//   1. Lexical Normalization: Strips punctuation and cleans noun candidates.
//   2. Dual-Sheet Grounding: Ingests nouns.base.dat + nouns.live.dat.
//   3. Noun Phrase Construction: Identifies quantities, units, and containers.
//   4. Arithmetic Pattern Recognition: Parses addition, subtraction,
//      multiplication, and division with balanced ternary integer semantics.

use std::collections::HashMap;
use std::sync::OnceLock;

// =============================================================================
// BLOCK 2: OPENING BLOCK
// ( Parsing Types, Morphological Units & Grammar Helpers )
// =============================================================================

/// Static data sheet contents baked into the binary.
const NOUNS_BASE_DAT: &str = include_str!("data/nouns.base.dat");
const NOUNS_LIVE_DAT: &str = include_str!("data/nouns.live.dat");

/// Extracted noun phrase with qualifiers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractedNounPhrase {
    /// Base noun or entity name.
    pub noun: String,
    /// Associated quantity, if any.
    pub quantity: Option<i64>,
    /// Container or collection scope, if any.
    pub container: Option<String>,
    /// Inferred role if discernible (e.g., "Subject", "Object", "Container").
    pub role_hint: Option<String>,
}

/// Morphological entry parsed from nouns.base.dat and nouns.live.dat.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NounEntry {
    pub noun: String,
    pub boundary_type: String,
    pub default_domain: String,
    pub role: String,
    pub singular: String,
    pub plural: String,
}

/// Linguistic noun registry caching data-driven morphological definitions.
pub struct NounRegistry {
    entries: HashMap<String, NounEntry>,
}

impl NounRegistry {
    /// Initializes registry by ingesting both .base.dat and .live.dat sheets.
    pub fn init() -> Self {
        let mut entries = HashMap::new();

        // 1. Ingest primordial baseline nouns
        Self::ingest_sheet(&mut entries, NOUNS_BASE_DAT);

        // 2. Ingest living learned nouns
        Self::ingest_sheet(&mut entries, NOUNS_LIVE_DAT);

        Self { entries }
    }

    fn ingest_sheet(map: &mut HashMap<String, NounEntry>, sheet: &str) {
        for line in sheet.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
            if parts.len() >= 6 {
                let noun = parts[0].to_lowercase();
                let entry = NounEntry {
                    noun: noun.clone(),
                    boundary_type: parts[1].to_string(),
                    default_domain: parts[2].to_string(),
                    role: parts[3].to_string(),
                    singular: parts[4].to_lowercase(),
                    plural: parts[5].to_lowercase(),
                };
                map.insert(noun.clone(), entry.clone());
                map.insert(entry.singular.clone(), entry.clone());
                map.insert(entry.plural.clone(), entry);
            }
        }
    }

    /// Accesses the global noun registry singleton.
    pub fn global() -> &'static NounRegistry {
        static REGISTRY: OnceLock<NounRegistry> = OnceLock::new();
        REGISTRY.get_or_init(Self::init)
    }

    /// Queries an entry by noun token.
    pub fn get(&self, token: &str) -> Option<&NounEntry> {
        self.entries.get(&token.trim().to_lowercase())
    }
}

// =============================================================================
// BLOCK 3: BODY BLOCK
// ( Pattern Parsing Algorithms & Discourse Morphological Normalization )
// =============================================================================

/// Parses an addition sentence.
/// E.g., "Add 1 ball to group A which contains 4".
pub fn parse_addition_sentence(
    s: &str,
) -> (i64, String, String, i64, String) {
    let mut qty = 1;
    let mut unit = "Ball".to_string();
    let mut target = "A".to_string();
    let mut initial = 4;
    let mut container = "Group".to_string();

    let tokens: Vec<&str> = s.split_whitespace().collect();
    for i in 0..tokens.len() {
        if tokens[i].eq_ignore_ascii_case("add") && i + 1 < tokens.len() {
            if let Ok(n) = tokens[i + 1].parse::<i64>() {
                qty = n;
            }
            if i + 2 < tokens.len() {
                unit = capitalize(clean_noun(tokens[i + 2]));
            }
        }
        if tokens[i].eq_ignore_ascii_case("to") {
            let mut j = i + 1;
            if j < tokens.len() && tokens[j].eq_ignore_ascii_case("the") {
                j += 1;
            }
            if j < tokens.len() {
                container = capitalize(clean_noun(tokens[j]));
                if j + 1 < tokens.len() {
                    target = clean_noun(tokens[j + 1]).to_string();
                }
            }
        }
        if (tokens[i].eq_ignore_ascii_case("contains")
            || tokens[i].eq_ignore_ascii_case("has"))
            && i + 1 < tokens.len()
        {
            if let Ok(n) = tokens[i + 1].parse::<i64>() {
                initial = n;
            }
        }
    }

    (qty, unit, target, initial, container)
}

/// Parses a subtraction sentence.
/// E.g., "Remove 2 items from box A has 10".
pub fn parse_subtraction_sentence(s: &str) -> (i64, String, String, i64) {
    let mut qty = 1;
    let mut unit = "Item".to_string();
    let mut target = "A".to_string();
    let mut initial = 10;

    let tokens: Vec<&str> = s.split_whitespace().collect();
    for i in 0..tokens.len() {
        if (tokens[i].eq_ignore_ascii_case("subtract")
            || tokens[i].eq_ignore_ascii_case("remove"))
            && i + 1 < tokens.len()
        {
            if let Ok(n) = tokens[i + 1].parse::<i64>() {
                qty = n;
            }
            if i + 2 < tokens.len() {
                unit = capitalize(clean_noun(tokens[i + 2]));
            }
        }
        if tokens[i].eq_ignore_ascii_case("from") && i + 1 < tokens.len() {
            target = clean_noun(tokens[i + 1]).to_string();
        }
        if (tokens[i].eq_ignore_ascii_case("contains")
            || tokens[i].eq_ignore_ascii_case("has"))
            && i + 1 < tokens.len()
        {
            if let Ok(n) = tokens[i + 1].parse::<i64>() {
                initial = n;
            }
        }
    }

    (qty, unit, target, initial)
}

/// Parses a multiplication sentence.
/// E.g., "Multiply x by 3 with initial 10".
pub fn parse_multiplication_sentence(s: &str) -> (String, i64, i64) {
    let mut target = "x".to_string();
    let mut factor = 2;
    let mut initial = 10;

    let tokens: Vec<&str> = s.split_whitespace().collect();
    for i in 0..tokens.len() {
        if tokens[i].eq_ignore_ascii_case("multiply") && i + 1 < tokens.len() {
            target = clean_noun(tokens[i + 1]).to_string();
        }
        if tokens[i].eq_ignore_ascii_case("by") && i + 1 < tokens.len() {
            if let Ok(n) = tokens[i + 1].parse::<i64>() {
                factor = n;
            }
        }
        if (tokens[i].eq_ignore_ascii_case("initial")
            || tokens[i].eq_ignore_ascii_case("starting")
            || tokens[i].eq_ignore_ascii_case("has")
            || tokens[i].eq_ignore_ascii_case("from"))
            && i + 1 < tokens.len()
        {
            if let Ok(n) = tokens[i + 1].parse::<i64>() {
                initial = n;
            }
        }
    }

    (target, factor, initial)
}

/// Parses a division sentence.
/// E.g., "Divide x by 2 starting at 20".
pub fn parse_division_sentence(s: &str) -> (String, i64, i64) {
    let mut target = "x".to_string();
    let mut divisor = 2;
    let mut initial = 20;

    let tokens: Vec<&str> = s.split_whitespace().collect();
    for i in 0..tokens.len() {
        if tokens[i].eq_ignore_ascii_case("divide") && i + 1 < tokens.len() {
            target = clean_noun(tokens[i + 1]).to_string();
        }
        if tokens[i].eq_ignore_ascii_case("by") && i + 1 < tokens.len() {
            if let Ok(n) = tokens[i + 1].parse::<i64>() {
                divisor = n;
            }
        }
        if (tokens[i].eq_ignore_ascii_case("initial")
            || tokens[i].eq_ignore_ascii_case("starting")
            || tokens[i].eq_ignore_ascii_case("has")
            || tokens[i].eq_ignore_ascii_case("from"))
            && i + 1 < tokens.len()
        {
            if let Ok(n) = tokens[i + 1].parse::<i64>() {
                initial = n;
            }
        }
    }

    (target, divisor, initial)
}

/// Extracts qualified noun phrases from natural text using the NounRegistry.
pub fn extract_noun_phrases(s: &str) -> Vec<ExtractedNounPhrase> {
    let tokens: Vec<&str> = s.split_whitespace().collect();
    let mut phrases = Vec::new();
    let registry = NounRegistry::global();
    let mut i = 0;

    while i < tokens.len() {
        let cleaned = clean_noun(tokens[i]);
        if cleaned.is_empty() {
            i += 1;
            continue;
        }

        // Check if token is a quantity (e.g., "5 loaves")
        if let Ok(qty) = cleaned.parse::<i64>() {
            if i + 1 < tokens.len() {
                let noun = clean_noun(tokens[i + 1]);
                if !noun.is_empty() {
                    let role = registry
                        .get(noun)
                        .map(|e| e.role.clone())
                        .unwrap_or_else(|| "Payload".to_string());
                    phrases.push(ExtractedNounPhrase {
                        noun: capitalize(noun),
                        quantity: Some(qty),
                        container: None,
                        role_hint: Some(role),
                    });
                    i += 2;
                    continue;
                }
            }
        }

        // Check for prepositional container (e.g., "into the fold", "in box")
        if cleaned.eq_ignore_ascii_case("in")
            || cleaned.eq_ignore_ascii_case("into")
            || cleaned.eq_ignore_ascii_case("from")
        {
            let mut j = i + 1;
            if j < tokens.len() && tokens[j].eq_ignore_ascii_case("the") {
                j += 1;
            }
            if j < tokens.len() {
                let container_name = clean_noun(tokens[j]);
                if !container_name.is_empty() {
                    phrases.push(ExtractedNounPhrase {
                        noun: capitalize(container_name),
                        quantity: None,
                        container: Some(capitalize(container_name)),
                        role_hint: Some("Container".to_string()),
                    });
                    i = j + 1;
                    continue;
                }
            }
        }

        i += 1;
    }

    phrases
}

/// Cleans punctuation and non-alphanumeric characters from noun tokens.
pub fn clean_noun(s: &str) -> &str {
    s.trim_matches(|c: char| !c.is_alphanumeric() && c != '_')
}

/// Capitalizes the first ASCII letter of a string.
pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Option::None => String::new(),
        Option::Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// Formats pluralization based on count using NounRegistry data grounding.
pub fn format_plural(count: i64, word: &str) -> String {
    let lower = word.to_lowercase();
    let registry = NounRegistry::global();

    // 1. Check data-grounded dictionary first (handles irregulars like loaves)
    if let Some(entry) = registry.get(&lower) {
        if count == 1 {
            return entry.singular.clone();
        } else {
            return entry.plural.clone();
        }
    }

    // 2. Deterministic morphological fallback
    let singular = if lower.ends_with('s') && lower.len() > 1 {
        lower[..lower.len() - 1].to_string()
    } else {
        lower.clone()
    };
    if count == 1 {
        singular
    } else {
        format!("{singular}s")
    }
}

// =============================================================================
// BLOCK 4: AUDIT PROCEDURES BLOCK
// ( Self-Diagnostic Verification for Pattern Extraction Integrity )
// =============================================================================

/// Executes automated self-diagnostic audit for the pattern parsing subsystem.
pub fn audit_patterns_subsystem() -> Result<(), String> {
    let registry = NounRegistry::global();
    if registry.entries.is_empty() {
        return Err("Audit failed: NounRegistry is empty".into());
    }

    // Verify irregular pluralization from data sheet (e.g., loaf -> loaves)
    if format_plural(1, "loaves") != "loaf" {
        return Err("Audit failed: loaf singularization error".into());
    }
    if format_plural(5, "loaf") != "loaves" {
        return Err("Audit failed: loaf pluralization error".into());
    }

    let add_sentence = "Add 3 coins to the Vault B which contains 7 coins";
    let (qty, unit, target, initial, container) =
        parse_addition_sentence(add_sentence);

    if qty != 3 || unit != "Coins" || target != "B" || initial != 7
        || container != "Vault"
    {
        return Err(format!(
            "Audit failed: parse_addition_sentence returned \
             ({qty}, {unit}, {target}, {initial}, {container})"
        ));
    }

    let sub_sentence = "Remove 4 gems from chest contains 12";
    let (sqty, sunit, starget, sinitial) =
        parse_subtraction_sentence(sub_sentence);
    if sqty != 4 || sunit != "Gems" || starget != "chest" || sinitial != 12 {
        return Err(format!(
            "Audit failed: parse_subtraction_sentence returned \
             ({sqty}, {sunit}, {starget}, {sinitial})"
        ));
    }

    let phrases =
        extract_noun_phrases("Give 5 loaves and 2 fish into the basket");
    if phrases.len() < 2 {
        return Err(format!(
            "Audit failed: extract_noun_phrases found {} phrases",
            phrases.len()
        ));
    }

    Ok(())
}

// =============================================================================
// BLOCK 5: UNIT TESTS BLOCK
// ( 100% Hermetic Unit Testing )
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_audit_procedure() {
        audit_patterns_subsystem().unwrap();
    }

    #[test]
    fn test_multiplication_and_division_parsing() {
        let mul = "Multiply counter by 4 from 5";
        let (target, factor, initial) = parse_multiplication_sentence(mul);
        assert_eq!(target, "counter");
        assert_eq!(factor, 4);
        assert_eq!(initial, 5);

        let div = "Divide balance by 5 starting 100";
        let (d_target, divisor, d_initial) = parse_division_sentence(div);
        assert_eq!(d_target, "balance");
        assert_eq!(divisor, 5);
        assert_eq!(d_initial, 100);
    }

    #[test]
    fn test_extract_noun_phrases() {
        let text = "Gather 10 talents and place into the treasury";
        let phrases = extract_noun_phrases(text);
        assert!(phrases
            .iter()
            .any(|p| p.noun == "Talents" && p.quantity == Some(10)));
        assert!(phrases
            .iter()
            .any(|p| p.noun == "Treasury" && p.container.is_some()));
    }

    #[test]
    fn test_data_driven_noun_registry_pluralization() {
        assert_eq!(format_plural(1, "loaves"), "loaf");
        assert_eq!(format_plural(2, "loaf"), "loaves");
        assert_eq!(format_plural(1, "sheep"), "sheep");
        assert_eq!(format_plural(2, "sheep"), "sheep");
        assert_eq!(format_plural(1, "boxes"), "box");
        assert_eq!(format_plural(2, "box"), "boxes");
    }
}

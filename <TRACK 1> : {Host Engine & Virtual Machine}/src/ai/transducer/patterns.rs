// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( Natural Language Pattern Parsing & Discourse Token Extraction )
// ============================================================================
//! # Sentence Extraction Helpers (`patterns.rs`)
//!
//! Extracts entities, containers, quantities, and targets from natural English
//! sentences to initialize formal 4-Block OmniCode AST contexts.

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
        if tokens[i].eq_ignore_ascii_case("is") && i + 1 < tokens.len() {
            if let Ok(n) = tokens[i + 1].parse::<i64>() {
                initial = n;
            }
        }
    }

    (target, factor, initial)
}

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
        if tokens[i].eq_ignore_ascii_case("is") && i + 1 < tokens.len() {
            if let Ok(n) = tokens[i + 1].parse::<i64>() {
                initial = n;
            }
        }
    }

    (target, divisor, initial)
}

pub fn clean_noun(s: &str) -> &str {
    s.trim_matches(|c: char| !c.is_alphanumeric() && c != '_')
}

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn format_plural(count: i64, word: &str) -> String {
    let lower = word.to_lowercase();
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

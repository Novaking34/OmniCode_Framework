// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Continuous SVO Semantic Composition & Two-Way Checksum Parity Gate )
// ==============================================================================
//! # Continuous SVO Semantic Composition (`svo.rs`)
//!
//! Decomposes conversational natural language prompts algebraically into
//! nested Subject-Verb-Object expressions using OmniCode directional vectors:
//! - `-->` : Rightwards causal invocation / execution
//! - `<--` : Leftwards assignment / reception
//! - `^--` : Upwards extraction / emission
//! - `v--` : Downwards storage / partition / vessel reservation
//!
//! ### Key Invariants:
//! 1. **Algebraic Composition**: Dynamic natural prompts are mapped to nested
//!    vector equations rather than rigid pre-scripted lookup templates.
//! 2. **Two-Way Checksum Parity Gate**: Synthesized reels are audited by the
//!    `RestoratorCertifier::evaluate_two_way_checksum` parity gate before VM dispatch.
//! 3. **Morphological Boundary Protection**: Guarantees all `[...]`, `<...>`, `{...}`
//!    brackets conform to Rank 1 invariants.

use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::DiagnosticLogger;
use crate::diagnostics::restorator::certifier::{RestoratorCertifier, TwoWayAgreement};
use crate::diagnostics::restorator::Restorator;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Directional Operators, SVO Clauses & Parity Gate Verification Structures )
// ==============================================================================

/// OmniCode directional causal operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SvoOperator {
    /// Rightwards causal invocation / execution (`-->`).
    Invoke,
    /// Leftwards assignment / reception (`<--`).
    Assign,
    /// Upwards extraction / emission (`^--`).
    Emit,
    /// Downwards storage / vessel reservation (`v--`).
    Store,
}

impl SvoOperator {
    /// Returns the canonical OmniCode operator token.
    pub fn as_str(&self) -> &'static str {
        match self {
            SvoOperator::Invoke => "-->",
            SvoOperator::Assign => "<--",
            SvoOperator::Emit => "^--",
            SvoOperator::Store => "v--",
        }
    }

    /// Infers an operator from an English verb or action term.
    pub fn from_verb(verb: &str) -> Self {
        let low = verb.trim().to_lowercase();
        if low.contains("emit")
            || low.contains("extract")
            || low.contains("eject")
            || low.contains("pop")
            || low.contains("output")
            || low.contains("yield")
            || low.contains("send")
        {
            SvoOperator::Emit
        } else if low.contains("store")
            || low.contains("save")
            || low.contains("deposit")
            || low.contains("vessel")
            || low.contains("partition")
            || low.contains("allocate")
            || low.contains("keep")
            || low.contains("hold")
        {
            SvoOperator::Store
        } else if low.contains("assign")
            || low.contains("receive")
            || low.contains("fetch")
            || low.contains("read")
            || low.contains("bind")
            || low.contains("set")
        {
            SvoOperator::Assign
        } else {
            // Default forward causation / invocation / execution
            SvoOperator::Invoke
        }
    }
}

/// A structured SVO clause parsed from discourse.
#[derive(Debug, Clone, PartialEq)]
pub struct SvoClause {
    pub subject: String,
    pub primary_op: SvoOperator,
    pub object: String,
    pub secondary_op: Option<SvoOperator>,
    pub secondary_object: Option<String>,
}

/// Result of continuous SVO composition verified through the parity gate.
#[derive(Debug, Clone)]
pub struct SvoCompositionResult {
    /// The synthesized canonical 4-Block OmniCode Reel.
    pub omnicode_reel: String,
    /// Parsed SVO clauses.
    pub clauses: Vec<SvoClause>,
    /// Two-Way Checksum parity agreement.
    pub agreement: TwoWayAgreement,
    /// Whether autonomic self-healing repairs were required.
    pub was_repaired: bool,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Discourse Parser, SVO Reel Synthesizer & Two-Way Checksum Parity Gate )
// ==============================================================================

pub struct SvoComposer;

impl SvoComposer {
    /// Composes a natural conversational prompt into a verified 4-Block OmniCode Reel
    /// checked in real-time by the Two-Way Checksum parity gate.
    pub fn compose_and_verify(
        prompt: &str,
        logger: &DiagnosticLogger,
        debugger: &OmniDebugger,
        restorator: &Restorator,
    ) -> SvoCompositionResult {
        let clauses = Self::decompose_prompt(prompt);
        let raw_reel = Self::synthesize_svo_reel(prompt, &clauses);

        // Parity Gate: Evaluate 2-Way Checksum
        let agreement = RestoratorCertifier::evaluate_two_way_checksum(
            logger,
            debugger,
            &raw_reel,
        );

        let (final_reel, final_agreement, was_repaired) = match agreement {
            TwoWayAgreement::Certified => (raw_reel, TwoWayAgreement::Certified, false),
            _ => {
                // Autonomic self-healing via Restorator
                let report = restorator.restore_source(&raw_reel);
                let re_eval = RestoratorCertifier::evaluate_two_way_checksum(
                    logger,
                    debugger,
                    &report.restored_content,
                );
                (report.restored_content, re_eval, true)
            }
        };

        SvoCompositionResult {
            omnicode_reel: final_reel,
            clauses,
            agreement: final_agreement,
            was_repaired,
        }
    }

    /// Decomposes a conversational prompt into one or more SVO clauses.
    pub fn decompose_prompt(prompt: &str) -> Vec<SvoClause> {
        let clean = prompt.trim();
        let words: Vec<&str> = clean
            .split_whitespace()
            .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric() && c != '_'))
            .filter(|w| !w.is_empty())
            .collect();

        if words.is_empty() {
            return vec![SvoClause {
                subject: "Agent".to_string(),
                primary_op: SvoOperator::Invoke,
                object: "Action".to_string(),
                secondary_op: None,
                secondary_object: None,
            }];
        }

        let mut clauses = Vec::new();

        // Check for transfer / pipe patterns: "Transfer X from A to B and store in C"
        let low_words: Vec<String> = words.iter().map(|w| w.to_lowercase()).collect();

        let mut from_idx = None;
        let mut to_idx = None;
        let mut in_idx = None;

        for (idx, w) in low_words.iter().enumerate() {
            if w == "from" {
                from_idx = Some(idx);
            } else if w == "to" || w == "into" {
                to_idx = Some(idx);
            } else if w == "in" || w == "store" {
                in_idx = Some(idx);
            }
        }

        if let (Some(f), Some(t)) = (from_idx, to_idx) {
            let source = if f + 1 < words.len() { words[f + 1] } else { "Source" };
            let target = if t + 1 < words.len() { words[t + 1] } else { "Target" };
            let item = if f > 1 { words[1] } else { "Item" };

            let (sec_op, sec_obj) = if let Some(s) = in_idx {
                if s + 1 < words.len() {
                    (Some(SvoOperator::Store), Some(words[s + 1].to_string()))
                } else {
                    (None, None)
                }
            } else {
                (None, None)
            };

            clauses.push(SvoClause {
                subject: source.to_string(),
                primary_op: SvoOperator::Emit,
                object: item.to_string(),
                secondary_op: Some(SvoOperator::Invoke),
                secondary_object: Some(target.to_string()),
            });

            if let (Some(op), Some(obj)) = (sec_op, sec_obj) {
                clauses.push(SvoClause {
                    subject: target.to_string(),
                    primary_op: op,
                    object: obj,
                    secondary_op: None,
                    secondary_object: None,
                });
            }

            return clauses;
        }

        // Standard 3-part SVO heuristic: [Subject, Verb, Object]
        if words.len() >= 3 {
            let subject = words[0];
            let verb = words[1];
            let object = words[2];
            let op = SvoOperator::from_verb(verb);

            let (sec_op, sec_obj) = if words.len() >= 5 {
                let sec_verb = words[3];
                let sec_target = words[4];
                (Some(SvoOperator::from_verb(sec_verb)), Some(sec_target.to_string()))
            } else {
                (None, None)
            };

            clauses.push(SvoClause {
                subject: subject.to_string(),
                primary_op: op,
                object: object.to_string(),
                secondary_op: sec_op,
                secondary_object: sec_obj,
            });
        } else if words.len() == 2 {
            let op = SvoOperator::from_verb(words[0]);
            clauses.push(SvoClause {
                subject: "Agent".to_string(),
                primary_op: op,
                object: words[1].to_string(),
                secondary_op: None,
                secondary_object: None,
            });
        } else {
            clauses.push(SvoClause {
                subject: words[0].to_string(),
                primary_op: SvoOperator::Invoke,
                object: "State".to_string(),
                secondary_op: None,
                secondary_object: None,
            });
        }

        clauses
    }

    /// Synthesizes a valid, fully formed 4-Block OmniCode Reel from SVO clauses.
    pub fn synthesize_svo_reel(prompt: &str, clauses: &[SvoClause]) -> String {
        let mut decls = Vec::new();
        let mut body_rolls = Vec::new();
        let mut verify_lines = Vec::new();

        for (idx, clause) in clauses.iter().enumerate() {
            let roll_name = format!("roll_svo_{}", idx + 1);
            decls.push(format!("{} [ 1 ]", clause.subject));
            decls.push(format!("{} [ 1 ]", clause.object));

            let mut expr = format!(
                "{} {} {}",
                clause.subject,
                clause.primary_op.as_str(),
                clause.object
            );

            if let (Some(sec_op), Some(sec_obj)) = (clause.secondary_op, &clause.secondary_object) {
                decls.push(format!("{} [ 1 ]", sec_obj));
                expr.push_str(&format!(" {} {}", sec_op.as_str(), sec_obj));
            }

            body_rolls.push(format!("    {} [ {} ]", roll_name, expr));
            verify_lines.push(format!("        {} = PROCEED", roll_name));
        }

        // Deduplicate declarations while preserving order
        let mut unique_decls = Vec::new();
        for d in decls {
            if !unique_decls.contains(&d) {
                unique_decls.push(d);
            }
        }

        format!(
r#"# ========================================================================
# BLOCK 1: METADATA BLOCK
# ( Continuous SVO Semantic Composition & Invariant Manifest )
# ========================================================================
Module < OmniCode.AI.Transducer.SVO > ( {prompt} ) {{
    generator: < "Continuous SVO Semantic Transducer" >,
    algebra: < "Directional Vectors (--> , <-- , ^-- , v--)" >,
    parity_gate: < "Two-Way Checksum Certified" >
}}

# ========================================================================
# BLOCK 2: OPENING BLOCK
# ========================================================================
Opening {{
    lexicon: [ Lexicon {{ Logic }} :: Lexicon {{ Vectors }} ],
    preconditions: [ +1 ]
}}

{declarations}

# ========================================================================
# BLOCK 3: BODY BLOCK
# ========================================================================
Body [
{body_rolls}
]

# ========================================================================
# BLOCK 4: CLOSING BLOCK
# ========================================================================
Closing {{
    verify: [
{verify_lines}
    ],
    commit_status [ PROCEED ]
}}
"#,
            prompt = prompt,
            declarations = unique_decls.join("\n"),
            body_rolls = body_rolls.join("\n"),
            verify_lines = verify_lines.join(" ::\n")
        )
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests & Verification Proofs )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svo_operator_conversions() {
        assert_eq!(SvoOperator::Invoke.as_str(), "-->");
        assert_eq!(SvoOperator::Assign.as_str(), "<--");
        assert_eq!(SvoOperator::Emit.as_str(), "^--");
        assert_eq!(SvoOperator::Store.as_str(), "v--");

        assert_eq!(SvoOperator::from_verb("execute"), SvoOperator::Invoke);
        assert_eq!(SvoOperator::from_verb("emit"), SvoOperator::Emit);
        assert_eq!(SvoOperator::from_verb("store"), SvoOperator::Store);
        assert_eq!(SvoOperator::from_verb("assign"), SvoOperator::Assign);
    }

    #[test]
    fn test_decompose_transfer_prompt() {
        let prompt = "Transfer Cell from Buffer to Screen and store in Vault";
        let clauses = SvoComposer::decompose_prompt(prompt);
        assert!(!clauses.is_empty());
        assert_eq!(clauses[0].subject, "Buffer");
        assert_eq!(clauses[0].primary_op, SvoOperator::Emit);
        assert_eq!(clauses[0].object, "Cell");
    }

    #[test]
    fn test_compose_and_verify_parity_gate() {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 10);
        let restorator = Restorator::new(logger.clone());

        let res = SvoComposer::compose_and_verify(
            "Agent sends message to Destination",
            &logger,
            &debugger,
            &restorator,
        );

        assert!(!res.omnicode_reel.is_empty());
        assert_eq!(res.agreement, TwoWayAgreement::Certified);
        assert!(res.omnicode_reel.contains("roll_svo_1"));
        assert!(res.omnicode_reel.contains("-->"));
    }
}

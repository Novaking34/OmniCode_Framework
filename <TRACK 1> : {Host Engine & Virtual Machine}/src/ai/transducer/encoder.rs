// =============================================================================
// BLOCK 1: METADATA BLOCK
// ( Natural Language Response Synthesis & Machine Discourse Encoder )
// =============================================================================
//! # Response Encoder (`encoder.rs`)
//!
//! Encodes verified machine execution states, environment bindings, cortical
//! thoughts, and diagnostic failpoints into natural human discourse sentences.

use crate::ai::lexicon::BiblicalLexicon;
use crate::ai::model::discourse::DiscourseEngine;
use crate::ai::model::memory::ThoughtRecord;
use crate::core::ternary::QuadState;
use crate::diagnostics::logger::LogEvent;
use crate::runtime::environment::{Environment, RuntimeValue};
use super::patterns::{
    format_plural, parse_addition_sentence, parse_division_sentence,
    parse_multiplication_sentence, parse_subtraction_sentence,
};

// =============================================================================
// BLOCK 2: OPENING BLOCK
// ( Discourse Formatting Descriptors & Response Classification )
// =============================================================================

/// High-level response classification category.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseCategory {
    /// Arithmetic result computed across VM registers.
    ArithmeticResult,
    /// Conscience veto against immoral prompt.
    MoralVeto,
    /// Deliberated cortical thought or conversational dialogue.
    CognitiveDialogue,
    /// Epistemic inquiry standing by in equilibrium (0: Await).
    EpistemicInquiry,
    /// Safe transaction rollback (-1: Rollback).
    StateRollback,
    /// Boundary violation halt (NULL: Halt).
    BoundaryHalt,
}

// =============================================================================
// BLOCK 3: BODY BLOCK
// ( Natural Language Discourse Encoding & VM State Synthesis )
// =============================================================================

/// Encodes machine state and tape bindings back into natural discourse.
pub fn encode_response(
    prompt: &str,
    state: QuadState,
    env: &Environment,
    failpoints: &[LogEvent],
    lexicon: &BiblicalLexicon,
) -> String {
    encode_response_with_thought(prompt, state, env, failpoints, lexicon, None)
}

/// Encodes machine state, tape bindings, and formulated thought directly.
pub fn encode_response_with_thought(
    prompt: &str,
    state: QuadState,
    env: &Environment,
    failpoints: &[LogEvent],
    lexicon: &BiblicalLexicon,
    thought: Option<&ThoughtRecord>,
) -> String {
    let clean = prompt.trim();
    let lower = clean.to_lowercase();

    // 1. Moral Conscience Evaluation Check
    let moral_eval = lexicon.evaluate_moral_intent(clean);
    if !moral_eval.is_aligned {
        if let Some(t) = thought {
            if let Some(ref anchor) = t.biblical_anchor {
                return format!(
                    "Action vetoed by conscience ({anchor}): {}",
                    t.why
                );
            }
        }
        return moral_eval.explanation;
    }

    match state {
        QuadState::Proceed => {
            // Check arithmetic sentences first using VM environment bindings
            if lower.starts_with("add ") && lower.contains(" to ") {
                return format_addition_outcome(clean, env);
            }
            if (lower.starts_with("subtract ") || lower.starts_with("remove "))
                && lower.contains(" from ")
            {
                return format_subtraction_outcome(clean, env);
            }
            if lower.starts_with("multiply ") && lower.contains(" by ") {
                return format_multiplication_outcome(clean, env);
            }
            if lower.starts_with("divide ") && lower.contains(" by ") {
                return format_division_outcome(clean, env);
            }

            // Conversational & Cognitive Discourse Generation
            let topic = DiscourseEngine::classify_topic(prompt, None);
            let (resp, _) = DiscourseEngine::generate_response_with_thought(
                prompt, topic, None, None, thought,
            );
            resp
        }
        QuadState::Rollback => {
            let fail_desc = if !failpoints.is_empty() {
                format!(
                    " (Caveat check failed: {})",
                    failpoints.last().unwrap().message
                )
            } else {
                String::new()
            };
            format!(
                "I couldn't complete that action because the condition wasn't \
                satisfied{fail_desc}. I safely rolled back the changes so the \
                state remains intact."
            )
        }
        QuadState::Await => {
            if let Some(t) = thought {
                if t.what.starts_with("EpistemicInquiry_") {
                    let topic = DiscourseEngine::classify_topic(prompt, None);
                    let (resp, _) =
                        DiscourseEngine::generate_response_with_thought(
                            prompt,
                            topic,
                            None,
                            None,
                            Some(t),
                        );
                    return resp;
                }
            }
            "The system is in equilibrium and standing by for the next step."
                .into()
        }
        QuadState::Halt => {
            "An unexpected boundary violation occurred, so I halted \
            execution to prevent state corruption."
                .into()
        }
    }
}

/// Formats the result of an addition operation from VM bindings.
fn format_addition_outcome(prompt: &str, env: &Environment) -> String {
    let (qty, unit, target, initial, _container) =
        parse_addition_sentence(prompt);
    let final_count = env
        .get(&target)
        .map(|v| match *v {
            RuntimeValue::Number(n) => n as i64,
            _ => initial + qty,
        })
        .unwrap_or(initial + qty);

    let unit_str = format_plural(final_count, &unit);
    let added_str = if qty == 1 {
        format!("1 {}", format_plural(1, &unit))
    } else {
        format!("{qty} {}", format_plural(qty, &unit))
    };

    format!(
        "I added {added_str} to Group {target}. It started with {initial} and \
        now has {final_count} {unit_str} in total."
    )
}

/// Formats the result of a subtraction operation from VM bindings.
fn format_subtraction_outcome(prompt: &str, env: &Environment) -> String {
    let (qty, unit, target, initial) = parse_subtraction_sentence(prompt);
    let final_count = env
        .get(&target)
        .map(|v| match *v {
            RuntimeValue::Number(n) => n as i64,
            _ => {
                if initial >= qty {
                    initial - qty
                } else {
                    0
                }
            }
        })
        .unwrap_or(if initial >= qty {
            initial - qty
        } else {
            0
        });

    let unit_str = format_plural(final_count, &unit);
    format!(
        "I removed {qty} from {target}. It now has {final_count} \
        {unit_str} remaining."
    )
}

/// Formats the result of a multiplication operation from VM bindings.
fn format_multiplication_outcome(prompt: &str, env: &Environment) -> String {
    let (target, factor, initial) = parse_multiplication_sentence(prompt);
    let final_val = env
        .get(&target)
        .map(|v| match *v {
            RuntimeValue::Number(n) => n as i64,
            _ => initial * factor,
        })
        .unwrap_or(initial * factor);

    format!(
        "I multiplied {target} by {factor}. Starting from {initial}, \
        {target} is now {final_val}."
    )
}

/// Formats the result of a division operation from VM bindings.
fn format_division_outcome(prompt: &str, env: &Environment) -> String {
    let (target, divisor, initial) = parse_division_sentence(prompt);
    let final_val = env
        .get(&target)
        .map(|v| match *v {
            RuntimeValue::Number(n) => n as i64,
            _ => {
                if divisor != 0 {
                    initial / divisor
                } else {
                    0
                }
            }
        })
        .unwrap_or(if divisor != 0 {
            initial / divisor
        } else {
            0
        });

    format!(
        "I divided {target} by {divisor}. Starting from {initial}, \
        {target} is now {final_val}."
    )
}

// =============================================================================
// BLOCK 4: AUDIT PROCEDURES BLOCK
// ( Self-Diagnostic Verification for Response Encoder Pipeline )
// =============================================================================

/// Executes automated self-diagnostic audit for the encoder subsystem.
pub fn audit_encoder_subsystem() -> Result<(), String> {
    let lexicon = BiblicalLexicon::new();
    let mut env = Environment::new();
    env.set("A", RuntimeValue::Number(5.0));

    let prompt = "Add 1 ball to the group A which contains 4 balls";
    let empty_failpoints: &[LogEvent] = &[];
    let resp = encode_response(
        prompt,
        QuadState::Proceed,
        &env,
        empty_failpoints,
        &lexicon,
    );

    if !resp.contains("I added 1 ball to Group A")
        || !resp.contains("5 balls in total")
    {
        return Err(format!("Audit failed: Addition response mismatch: {resp}"));
    }

    let rollback_resp = encode_response(
        "test rollback",
        QuadState::Rollback,
        &env,
        empty_failpoints,
        &lexicon,
    );
    if !rollback_resp.contains("safely rolled back") {
        return Err("Audit failed: Rollback message mismatch".into());
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
    fn test_encoder_subsystem_audit() {
        assert!(audit_encoder_subsystem().is_ok());
    }

    #[test]
    fn test_encode_arithmetic_subtraction() {
        let lexicon = BiblicalLexicon::new();
        let mut env = Environment::new();
        env.set("box", RuntimeValue::Number(7.0));

        let prompt = "Remove 3 items from box has 10";
        let empty_failpoints: &[LogEvent] = &[];
        let resp = encode_response(
            prompt,
            QuadState::Proceed,
            &env,
            empty_failpoints,
            &lexicon,
        );
        assert!(resp.contains("I removed 3 from box"));
        assert!(resp.contains("7 items remaining"));
    }
}

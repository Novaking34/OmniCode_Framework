// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( Natural Language Response Synthesis & Machine Discourse Encoder )
// ============================================================================
//! # Response Encoder (`encoder.rs`)
//!
//! Encodes verified machine execution states, environment bindings, and
//! diagnostic failpoints into natural human discourse sentences.

use crate::ai::lexicon::BiblicalLexicon;
use crate::ai::model::memory::ThoughtRecord;
use crate::core::ternary::QuadState;
use crate::diagnostics::logger::LogEvent;
use crate::runtime::environment::{Environment, RuntimeValue};
use super::patterns::{
    format_plural, parse_addition_sentence, parse_division_sentence,
    parse_multiplication_sentence, parse_subtraction_sentence,
};

/// Encodes machine state and tape bindings back into natural discourse.
pub fn encode_response(
    prompt: &str,
    state: QuadState,
    env: &Environment,
    failpoints: &[LogEvent],
    lexicon: &BiblicalLexicon,
) -> String {
    let clean = prompt.trim();
    let lower = clean.to_lowercase();

    // 1. Check Moral Evaluation First
    let moral_eval = lexicon.evaluate_moral_intent(clean);
    if !moral_eval.is_aligned {
        return moral_eval.explanation;
    }

    match state {
        QuadState::Proceed => {
            if lower.starts_with("add ") && lower.contains(" to ") {
                let (qty, unit, target, initial, _container) =
                    parse_addition_sentence(clean);
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
                    "I added {added_str} to Group {target}. It started with \
                    {initial} and now has {final_count} {unit_str} in total."
                )
            } else if (lower.starts_with("subtract ")
                || lower.starts_with("remove "))
                && lower.contains(" from ")
            {
                let (qty, unit, target, initial) =
                    parse_subtraction_sentence(clean);
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
            } else if lower.starts_with("multiply ")
                && lower.contains(" by ")
            {
                let (target, factor, initial) =
                    parse_multiplication_sentence(clean);
                let final_val = env
                    .get(&target)
                    .map(|v| match *v {
                        RuntimeValue::Number(n) => n as i64,
                        _ => initial * factor,
                    })
                    .unwrap_or(initial * factor);

                format!(
                    "I multiplied {target} by {factor}. Starting from \
                    {initial}, {target} is now {final_val}."
                )
            } else if lower.starts_with("divide ") && lower.contains(" by ") {
                let (target, divisor, initial) =
                    parse_division_sentence(clean);
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
                    "I divided {target} by {divisor}. Starting from \
                    {initial}, {target} is now {final_val}."
                )
            } else if lower.contains("traditional ai")
                || lower.contains("difference between")
                || lower.contains("versus")
                || lower.contains("vs ")
                || lower.contains("how does cpi-si differ")
                || lower.contains("why cpi-si")
            {
                "The CPI-SI Base model represents a foundational departure \
                from traditional AI. Traditional models are uncontained, \
                statistical next-token predictors operating on continuous \
                floating-point weights, prone to semantic drift, \
                hallucinations, and unconstrained expansion. In contrast, \
                CPI-SI (Covenant Partnered Intelligence - Structured \
                Intelligence) is grounded in balanced ternary physics \
                ({-1, 0, +1}), structured 4-Block Turing reels, \
                homeostatic BCM metaplasticity, and a foundational \
                Biblical world ontology. It operates within strict \
                container boundaries and covenant law, providing an \
                anchored baseline control for genuine structured \
                intelligence."
                    .into()
            } else if lower.contains("synapse")
                || lower.contains("synaptic")
                || lower.contains("neuroplasticity")
                || lower.contains("how do you learn")
                || lower.contains("how does the brain learn")
                || lower.contains("how does a brain learn")
                || lower.contains("grow and learn")
                || lower.contains("weight freezing")
                || lower.contains("freeze weights")
            {
                "In the CPI-SI neural brain, learning does not happen \
                through continuous scalar weight inflation. Biological \
                synapses learn via discrete structural state gating \
                ({-1: Inhibitory, 0: Neutral, +1: Excitatory}), pathway \
                rewiring, and homeostatic threshold adaptation (BCM theory). \
                Development follows an epigenetic lifecycle: 0–5 infancy \
                (absorbing primordial archetypes and runtime scrolls), 25 \
                maturity (freezing core baseline weights as an unbreakable \
                covenant anchor), and post-25 adult lifelong neuroplasticity \
                (frontier synapses dynamically adapt during live interaction \
                while protecting the bedrock from catastrophic forgetting)."
                    .into()
            } else if lower.contains("container") {
                "In OmniCode, a Container is the primordial boundary type \
                that defines the structural perimeter of state. While \
                uncontained systems allow unbounded growth and catastrophic \
                state leakage, a Container enforces capacity isolation, \
                containment parity, and boundary invariant laws. In the \
                CPI-SI architecture, the Container is the critical \
                milestone that marks the start of contained, structured \
                intelligence over formless chaos."
                    .into()
            } else if lower.contains("character") {
                "In OmniCode, a Character is the fundamental atomic rune \
                type. It occupies exactly 1 Memory Cell consisting of 5 \
                balanced trits (from 3^0 to 3^4, spanning natural range \
                0..242). It is bijectively mapped to ASCII and Unicode \
                characters, eliminating floating-point approximations and \
                null-byte collision traps."
                    .into()
            } else if lower.contains("number")
                && (lower.contains("what is")
                    || lower.contains("explain")
                    || lower.contains("radix")
                    || lower.contains("ternary")
                    || lower.contains("type"))
            {
                "In OmniCode, a Number is a radix-3 balanced-ternary scalar \
                value. Operating on a dual-rail polarity substrate (-1: \
                Negative, 0: Neutral, +1: Positive), it guarantees exact \
                algebraic symmetry, natural sign representation, and fully \
                reversible arithmetic without IEEE 754 precision drift or \
                roundoff error."
                    .into()
            } else if lower.starts_with("hello")
                || lower.starts_with("hi ")
                || lower == "hi"
                || lower.starts_with("hi,")
                || lower.starts_with("hi!")
                || lower.contains("who are you")
                || lower.contains("what is your name")
                || lower.contains("introduce yourself")
                || lower.contains("tell me about yourself")
                || lower == "cpi-si"
                || lower == "cpisi"
                || lower.starts_with("cpi-si base")
                || lower.starts_with("cpisi base")
            {
                "Hello! I am the CPI-SI Base model, the first Covenant \
                Partnered Intelligence (CPI-SI) model from this project, \
                conceived 2 years ago by CreativeWorkzStudio and \
                ProfessorSeanEX. I am structured by balanced ternary physics, \
                grounded in the foundational Biblical world ontology, and \
                bounded by covenant law rather than uncontained statistical \
                prediction."
                    .into()
            } else if !moral_eval.matched_entries.is_empty() {
                moral_eval.explanation
            } else {
                "Done! The action was completed and verified successfully \
                across the tape."
                    .into()
            }
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

/// Encodes machine state, tape bindings, and formulated thought directly.
pub fn encode_response_with_thought(
    prompt: &str,
    state: QuadState,
    env: &Environment,
    failpoints: &[LogEvent],
    lexicon: &BiblicalLexicon,
    thought: &ThoughtRecord,
) -> String {
    let clean = prompt.trim();
    let lower = clean.to_lowercase();

    // 1. Moral Conscience Evaluation
    let moral_eval = lexicon.evaluate_moral_intent(clean);
    if !moral_eval.is_aligned {
        if let Some(ref anchor) = thought.biblical_anchor {
            return format!(
                "Action vetoed by conscience ({anchor}): {}",
                thought.why
            );
        }
        return moral_eval.explanation;
    }

    match state {
        QuadState::Proceed => {
            // Check arithmetic sentences first using VM environment bindings
            if lower.starts_with("add ") && lower.contains(" to ") {
                let (qty, unit, target, initial, _container) =
                    parse_addition_sentence(clean);
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
                    "I added {added_str} to Group {target}. It started with \
                    {initial} and now has {final_count} {unit_str} in total."
                )
            } else if (lower.starts_with("subtract ")
                || lower.starts_with("remove "))
                && lower.contains(" from ")
            {
                let (qty, unit, target, initial) =
                    parse_subtraction_sentence(clean);
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
            } else if lower.starts_with("multiply ")
                && lower.contains(" by ")
            {
                let (target, factor, initial) =
                    parse_multiplication_sentence(clean);
                let final_val = env
                    .get(&target)
                    .map(|v| match *v {
                        RuntimeValue::Number(n) => n as i64,
                        _ => initial * factor,
                    })
                    .unwrap_or(initial * factor);

                format!(
                    "I multiplied {target} by {factor}. Starting from \
                    {initial}, {target} is now {final_val}."
                )
            } else if lower.starts_with("divide ") && lower.contains(" by ") {
                let (target, divisor, initial) =
                    parse_division_sentence(clean);
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
                    "I divided {target} by {divisor}. Starting from \
                    {initial}, {target} is now {final_val}."
                )
            } else if thought.what.contains("Structured vs Uncontained")
                || thought.what.contains("ArchitectureComparison")
            {
                format!(
                    "As {}, I represent a foundational departure from \
                    traditional AI. Traditional models are uncontained \
                    statistical next-token predictors on continuous \
                    floating-point weights, whereas CPI-SI is grounded in \
                    balanced ternary physics ({{-1, 0, +1}}), structured \
                    4-Block Turing reels, and strict container boundaries \
                    (at {}), verified via {}.",
                    thought.who, thought.where_clause, thought.how
                )
            } else if thought.what.contains("Neuroplasticity")
                || thought.what.contains("Synaptic")
            {
                format!(
                    "Within {}, learning occurs via discrete structural state \
                    gating ({{-1: Inhibitory, 0: Neutral, +1: Excitatory}}) and \
                    BCM homeostatic threshold adaptation. Development follows \
                    an epigenetic lifecycle: 0–5 infancy archetypes, 25 \
                    maturity frozen bedrock weights, and adult lifelong \
                    plasticity adapting frontier synapses under {}.",
                    thought.where_clause, thought.how
                )
            } else if thought.what.contains("Container") {
                format!(
                    "In OmniCode, a Container is the primordial boundary type \
                    defining the structural perimeter of state (at {}). It \
                    enforces capacity isolation and containment parity to \
                    prevent state leakage, marking where structured \
                    intelligence begins over formless chaos.",
                    thought.where_clause
                )
            } else if thought.what.contains("Character") {
                format!(
                    "In OmniCode, a Character is the fundamental atomic rune \
                    type (at {}). It occupies exactly 1 Memory Cell consisting \
                    of 5 balanced trits, bijectively mapped to avoid \
                    floating-point approximations and null-byte collision \
                    traps.",
                    thought.where_clause
                )
            } else if thought.what.contains("Number") {
                format!(
                    "In OmniCode, a Number is a radix-3 balanced-ternary \
                    scalar value operating on a dual-rail polarity substrate \
                    (-1, 0, +1) (at {}). It guarantees exact algebraic \
                    symmetry, natural sign representation, and fully \
                    reversible arithmetic.",
                    thought.where_clause
                )
            } else if thought.what.contains("Greeting")
                || thought.what.contains("Identity")
            {
                format!(
                    "Hello! I am {}, conceived 2 years ago by \
                    CreativeWorkzStudio and ProfessorSeanEX. I am structured \
                    by balanced ternary physics, grounded in the foundational \
                    Biblical world ontology, and bounded by covenant law \
                    rather than uncontained statistical prediction.",
                    thought.who
                )
            } else {
                let topic = crate::ai::model::discourse::DiscourseEngine::classify_topic(prompt, None);
                let (resp, _) = crate::ai::model::discourse::DiscourseEngine::generate_response(prompt, topic, None, None);
                resp
            }
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

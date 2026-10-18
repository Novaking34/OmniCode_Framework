// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( Natural Language Semantic Decoder & Intent Pattern Recognition )
// ============================================================================
//! # Semantic Decoder (`decoder.rs`)
//!
//! Recognizes discourse intents, verifies moral alignment against the
//! Covenant anchor, and synthesizes canonical 4-Block OmniCode Turing reels.

use crate::ai::lexicon::BiblicalLexicon;
use super::templates::{
    synthesize_addition_reel, synthesize_biblical_wisdom_reel,
    synthesize_character_inquiry_reel, synthesize_container_inquiry_reel,
    synthesize_cpisi_comparison_reel, synthesize_division_reel,
    synthesize_identity_greeting_reel,
    synthesize_moral_veto_reel, synthesize_multiplication_reel,
    synthesize_number_inquiry_reel, synthesize_plasticity_inquiry_reel,
    synthesize_subtraction_reel,
};

/// Decodes a natural English sentence into a canonical 4-Block OmniCode Reel.
pub fn decode_prompt(
    lexicon: &BiblicalLexicon,
    prompt: &str,
) -> Result<String, String> {
    let clean = prompt.trim();
    let lower = clean.to_lowercase();

    // 1. Moral Deliberation Check: evaluate intent against Covenant Anchor
    let moral_eval = lexicon.evaluate_moral_intent(clean);
    if !moral_eval.is_aligned {
        return Ok(synthesize_moral_veto_reel(clean, &moral_eval));
    }

    // 2. CPI-SI vs Traditional AI Architectural Comparison
    if lower.contains("traditional ai")
        || lower.contains("difference between")
        || lower.contains("versus")
        || lower.contains("vs ")
        || lower.contains("how does cpi-si differ")
        || lower.contains("why cpi-si")
    {
        return Ok(synthesize_cpisi_comparison_reel(clean));
    }

    // 3. Synaptic Learning & Biological Neuroplasticity Inquiry
    if lower.contains("synapse")
        || lower.contains("synaptic")
        || lower.contains("neuroplasticity")
        || lower.contains("how do you learn")
        || lower.contains("how does the brain learn")
        || lower.contains("how does a brain learn")
        || lower.contains("grow and learn")
        || lower.contains("weight freezing")
        || lower.contains("freeze weights")
    {
        return Ok(synthesize_plasticity_inquiry_reel(clean));
    }

    // 4. Container Boundary Type Inquiry
    if lower.contains("container") {
        return Ok(synthesize_container_inquiry_reel(clean));
    }

    // 5. Character Type Inquiry
    if lower.contains("character") {
        return Ok(synthesize_character_inquiry_reel(clean));
    }

    // 6. Number Type Inquiry
    if lower.contains("number")
        && (lower.contains("what is")
            || lower.contains("explain")
            || lower.contains("radix")
            || lower.contains("ternary")
            || lower.contains("type"))
    {
        return Ok(synthesize_number_inquiry_reel(clean));
    }

    // 7. Identity & Covenant Greeting
    if lower.starts_with("hello")
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
        return Ok(synthesize_identity_greeting_reel(clean));
    }

    // 8. Pattern 1: Addition
    if lower.starts_with("add ") && lower.contains(" to ") {
        return Ok(synthesize_addition_reel(clean));
    }

    // 9. Pattern 2: Subtraction
    if (lower.starts_with("subtract ") || lower.starts_with("remove "))
        && lower.contains(" from ")
    {
        return Ok(synthesize_subtraction_reel(clean));
    }

    // 10. Pattern 3: Multiplication
    if lower.starts_with("multiply ") && lower.contains(" by ") {
        return Ok(synthesize_multiplication_reel(clean));
    }

    // 11. Pattern 4: Division
    if lower.starts_with("divide ") && lower.contains(" by ") {
        return Ok(synthesize_division_reel(clean));
    }

    // 12. Pattern 5: Biblical Wisdom / World Model Inquiry
    if !moral_eval.matched_entries.is_empty() {
        return Ok(synthesize_biblical_wisdom_reel(clean, &moral_eval));
    }

    // 13. Continuous SVO Semantic Composition (algebraic directional vectors)
    let clauses = super::svo::SvoComposer::decompose_prompt(clean);
    Ok(super::svo::SvoComposer::synthesize_svo_reel(clean, &clauses))
}

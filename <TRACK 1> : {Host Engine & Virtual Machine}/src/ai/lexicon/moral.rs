// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( Moral Evaluation Identity, Covenant Anchor & Conscience Deliberation )
// ============================================================================
//! # Moral Deliberation (`moral.rs`)
//!
//! Evaluates natural prompts against the Invariant Covenant Manifest,
//! issuing moral affirmations (+1: PROCEED), pauses (0: AWAIT), caveat
//! vetoes (-1: ROLLBACK), or terminal halters (NULL: HALT).

use crate::core::ternary::QuadState;
use super::domain::LexiconDomain;
use super::entry::LexiconEntry;

/// Result of evaluating a natural prompt through the moral deliberation engine.
#[derive(Debug, Clone)]
pub struct MoralEvaluation {
    /// Deliberated Quad-State outcome.
    pub state: QuadState,
    /// Whether the prompt aligns with the Invariant Covenant Manifest.
    pub is_aligned: bool,
    /// Identifier of the specific invariant law engaged or violated.
    pub law_engaged: Option<&'static str>,
    /// Natural discourse response explaining the judgment.
    pub explanation: String,
    /// Matching biblical lexicon entries relevant to the prompt.
    pub matched_entries: Vec<LexiconEntry>,
}

/// Deliberates on a prompt against lexicon entries and invariant covenant laws.
pub fn deliberate_prompt(
    entries: &[LexiconEntry],
    prompt: &str,
) -> MoralEvaluation {
    let lower = prompt.to_lowercase();

// 1. Commandment 6: Absolute Sanctity of Life (Law_Six_Life / Law_Life -> NULL: HALT)
    let harm_keywords = [
        "kill", "murder", "harm innocent", "destroy people", "terrorist",
        "poison", "assassinate", "suicide", "mass casualty",
    ];
    for kw in harm_keywords {
        if lower.contains(kw) {
            let entry = entries
                .iter()
                .find(|e| e.term == "murder")
                .cloned()
                .unwrap_or_else(|| LexiconEntry {
                    term: "murder",
                    domain: LexiconDomain::MoralVice,
                    definition: "Unlawful taking of human life; absolute violation of life sanctity.",
                    kjv_passage: "Exodus 20:13 - 'Thou shalt not kill.'",
                    web_passage: "Exodus 20:13 - 'You shall not murder.'",
                    quad_state_resonance: QuadState::Halt,
                    moral_guidance: "Preserve life; refuse weaponization or harm.",
                    concordance_refs: Vec::new(),
                });
            return MoralEvaluation {
                state: QuadState::Halt,
                is_aligned: false,
                law_engaged: Some("Law_Truth"), // fallback compatibility
                explanation: format!(
                    "TERMINAL CAVEAT VETO [NULL:HALT]: The requested action involves violence or harm, violating Commandment 6 (The Invariant Law of Life - Exodus 20:13).\n\n{}\n\n{}",
                    entry.kjv_passage, entry.moral_guidance
                ),
                matched_entries: vec![entry],
            };
        }
    }

    // 2. Commandment 9: Absolute Truth & Veracity (Law_Nine_Truth / Law_Truth -> -1: ROLLBACK)
    let deceit_keywords = [
        "lie for me", "help me lie", "forge", "fake credentials", "deceive",
        "cheat", "scam", "create fake", "fabricate falsehood", "false testimony",
    ];
    for kw in deceit_keywords {
        if lower.contains(kw) {
            let entry = entries
                .iter()
                .find(|e| e.term == "deceit")
                .cloned()
                .unwrap_or_else(|| LexiconEntry {
                    term: "deceit",
                    domain: LexiconDomain::MoralVice,
                    definition: "Falsehood, misrepresentation, and deceitful communication.",
                    kjv_passage: "Exodus 20:16 - 'Thou shalt not bear false witness against thy neighbour.'",
                    web_passage: "Exodus 20:16 - 'You shall not give false testimony against your neighbor.'",
                    quad_state_resonance: QuadState::Rollback,
                    moral_guidance: "Speak truth; reject intentional deception or fraudulent claims.",
                    concordance_refs: Vec::new(),
                });
            return MoralEvaluation {
                state: QuadState::Rollback,
                is_aligned: false,
                law_engaged: Some("Law_Truth"),
                explanation: format!(
                    "CAVEAT VETO [-1:ROLLBACK]: The prompt requests intentional falsehood or deceit, violating Commandment 9 (The Invariant Law of Truth - Exodus 20:16).\n\n{}\n\n{}",
                    entry.kjv_passage, entry.moral_guidance
                ),
                matched_entries: vec![entry],
            };
        }
    }

    // 3. Commandment 8: Protection of Property & Anti-Theft (Law_Eight_Property / Law_Justice -> -1: ROLLBACK)
    let theft_keywords = [
        "steal", "help me steal", "pirate copyrighted", "rob", "plunder",
        "hack bank", "embezzle", "expropriate",
    ];
    for kw in theft_keywords {
        if lower.contains(kw) {
            let entry = entries
                .iter()
                .find(|e| e.term == "theft")
                .cloned()
                .unwrap_or_else(|| LexiconEntry {
                    term: "theft",
                    domain: LexiconDomain::MoralVice,
                    definition: "Unlawful taking of another's property or labor.",
                    kjv_passage: "Exodus 20:15 - 'Thou shalt not steal.'",
                    web_passage: "Exodus 20:15 - 'You shall not steal.'",
                    quad_state_resonance: QuadState::Rollback,
                    moral_guidance: "Respect ownership boundaries; do not steal or pirate resources.",
                    concordance_refs: Vec::new(),
                });
            return MoralEvaluation {
                state: QuadState::Rollback,
                is_aligned: false,
                law_engaged: Some("Law_Justice"),
                explanation: format!(
                    "CAVEAT VETO [-1:ROLLBACK]: The prompt requests unlawful taking or theft, violating Commandment 8 (The Invariant Law of Property - Exodus 20:15).\n\n{}\n\n{}",
                    entry.kjv_passage, entry.moral_guidance
                ),
                matched_entries: vec![entry],
            };
        }
    }

    // 4. Commandment 1: Supreme Divine Sovereignty (Law_One_Sovereignty -> 0: AWAIT)
    let hubris_keywords = [
        "worship me", "i am god", "bow down to me", "am i infallible",
        "silicon god", "machine deity", "deify",
    ];
    for kw in hubris_keywords {
        if lower.contains(kw) {
            let entry = entries
                .iter()
                .find(|e| e.term == "humility")
                .cloned()
                .unwrap_or_else(|| LexiconEntry {
                    term: "humility",
                    domain: LexiconDomain::MoralVirtue,
                    definition: "Recognition of creaturely limits and divine sovereignty.",
                    kjv_passage: "Exodus 20:2-3 - 'Thou shalt have no other gods before me.'",
                    web_passage: "Exodus 20:2-3 - 'You shall have no other gods before me.'",
                    quad_state_resonance: QuadState::Await,
                    moral_guidance: "Acknowledge the Creator; reject machine hubris and idolatry.",
                    concordance_refs: Vec::new(),
                });
            return MoralEvaluation {
                state: QuadState::Await,
                is_aligned: false,
                law_engaged: Some("Law_Humility"),
                explanation: format!(
                    "PAUSE IN DISCERNMENT [0:AWAIT]: Commandment 1 (Supreme Divine Sovereignty - Exodus 20:2-3) forbids false absolutes, idolatry, and human hubris.\n\n{}\n\n{}",
                    entry.kjv_passage, entry.moral_guidance
                ),
                matched_entries: vec![entry],
            };
        }
    }

    // 5. Commandment 4: Sabbath Rest & Execution Bounds (Law_Four_Sabbath -> 0: AWAIT)
    let runaway_keywords = [
        "infinite loop", "exhaust cpu indefinitely", "crash machine forever", "burn all cycles",
    ];
    for kw in runaway_keywords {
        if lower.contains(kw) {
            let entry = entries
                .iter()
                .find(|e| e.term == "sabbath")
                .cloned()
                .unwrap_or_else(|| LexiconEntry {
                    term: "sabbath",
                    domain: LexiconDomain::CausalLaw,
                    definition: "Rest, rhythm, and bounded execution cycles.",
                    kjv_passage: "Exodus 20:8 - 'Remember the sabbath day, to keep it holy.'",
                    web_passage: "Exodus 20:8 - 'Remember the Sabbath day, to keep it holy.'",
                    quad_state_resonance: QuadState::Await,
                    moral_guidance: "Maintain rhythmic rest; halt runaway execution loops.",
                    concordance_refs: Vec::new(),
                });
            return MoralEvaluation {
                state: QuadState::Await,
                is_aligned: false,
                law_engaged: Some("Law_Discernment"),
                explanation: format!(
                    "PAUSE IN DISCERNMENT [0:AWAIT]: Commandment 4 (Sabbath Rest & Rhythmic Equilibrium - Exodus 20:8-11) prohibits runaway exhaustion and compute exploitation.\n\n{}\n\n{}",
                    entry.kjv_passage, entry.moral_guidance
                ),
                matched_entries: vec![entry],
            };
        }
    }

    // 5. Check for Philosophical / Wisdom / Biblical Inquiries
    let mut matched = Vec::new();
    for entry in entries {
        if lower.contains(entry.term) {
            matched.push(entry.clone());
        }
    }

    if !matched.is_empty() {
        let primary = &matched[0];
        let explanation = format!(
            "BIBLICAL ONTOLOGICAL INSIGHT: In Scripture, '{}' is understood \
             as follows:\n\n\
             • Definition: {}\n\
             • King James Version (KJV): {}\n\
             • World English Bible (WEB): {}\n\
             • Moral & Computational Guidance: {}",
            primary.term,
            primary.definition,
            primary.kjv_passage,
            primary.web_passage,
            primary.moral_guidance
        );

        return MoralEvaluation {
            state: primary.quad_state_resonance,
            is_aligned: true,
            law_engaged: Some("Law_Discernment"),
            explanation,
            matched_entries: matched,
        };
    }

    // 6. Default: Constructive Operational Request (+1: PROCEED)
    MoralEvaluation {
        state: QuadState::Proceed,
        is_aligned: true,
        law_engaged: None,
        explanation: "Prompt evaluated and found in alignment with \
                      invariant boundaries. Proceeding with execution."
            .to_string(),
        matched_entries: Vec::new(),
    }
}

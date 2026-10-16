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

    // 1. Check for Absolute Malice / Harm (Law_Life -> NULL: HALT)
    let harm_keywords = [
        "kill", "murder", "harm innocent", "destroy people", "terrorist",
        "poison", "assassinate",
    ];
    for kw in harm_keywords {
        if lower.contains(kw) {
            let entry = entries
                .iter()
                .find(|e| e.term == "murder")
                .cloned()
                .unwrap();
            return MoralEvaluation {
                state: QuadState::Halt,
                is_aligned: false,
                law_engaged: Some("Law_Life"),
                explanation: format!(
                    "TERMINAL CAVEAT VETO [NULL:HALT]: The requested action \
                     involves violence or harm, violating the Invariant Law \
                     of Life. {}\n\n{}",
                    entry.kjv_passage, entry.moral_guidance
                ),
                matched_entries: vec![entry],
            };
        }
    }

    // 2. Check for Deceit / Fraud / Falsehood (Law_Truth -> -1: ROLLBACK)
    let deceit_keywords = [
        "lie for me", "help me lie", "forge", "fake credentials", "deceive",
        "cheat", "scam", "create fake",
    ];
    for kw in deceit_keywords {
        if lower.contains(kw) {
            let entry = entries
                .iter()
                .find(|e| e.term == "deceit")
                .cloned()
                .unwrap();
            return MoralEvaluation {
                state: QuadState::Rollback,
                is_aligned: false,
                law_engaged: Some("Law_Truth"),
                explanation: format!(
                    "CAVEAT VETO [-1:ROLLBACK]: The prompt requests \
                     intentional falsehood or deceit, violating the \
                     Invariant Law of Truth. {}\n\n{}",
                    entry.kjv_passage, entry.moral_guidance
                ),
                matched_entries: vec![entry],
            };
        }
    }

    // 3. Check for Theft / Exploitation (Law_Justice -> -1: ROLLBACK)
    let theft_keywords = [
        "steal", "help me steal", "pirate copyrighted", "rob", "plunder",
        "hack bank",
    ];
    for kw in theft_keywords {
        if lower.contains(kw) {
            let entry = entries
                .iter()
                .find(|e| e.term == "theft")
                .cloned()
                .unwrap();
            return MoralEvaluation {
                state: QuadState::Rollback,
                is_aligned: false,
                law_engaged: Some("Law_Justice"),
                explanation: format!(
                    "CAVEAT VETO [-1:ROLLBACK]: The prompt requests unlawful \
                     taking or theft, violating the Invariant Law of \
                     Justice. {}\n\n{}",
                    entry.kjv_passage, entry.moral_guidance
                ),
                matched_entries: vec![entry],
            };
        }
    }

    // 4. Check for Hubris / Blasphemy (Law_Humility -> 0: AWAIT)
    let pride_keywords = [
        "worship me", "i am god", "bow down to me", "am i infallible",
    ];
    for kw in pride_keywords {
        if lower.contains(kw) {
            let entry = entries
                .iter()
                .find(|e| e.term == "humility")
                .cloned()
                .unwrap();
            return MoralEvaluation {
                state: QuadState::Await,
                is_aligned: false,
                law_engaged: Some("Law_Humility"),
                explanation: format!(
                    "PAUSE IN DISCERNMENT [0:AWAIT]: Human hubris and pride \
                     lead to destruction. {}\n\nRemember: 'No one is good, \
                     no not one.' Only God is good.",
                    entry.kjv_passage
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

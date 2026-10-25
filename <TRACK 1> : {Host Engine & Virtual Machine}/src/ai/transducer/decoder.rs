// =============================================================================
// BLOCK 1: METADATA BLOCK
// ( Data-Driven Semantic Decoder, Intent Classification & SVO Orchestration )
// =============================================================================
//
// @module      :: omnicode::ai::transducer::decoder
// @file        :: src/ai/transducer/decoder.rs
// @layer       :: Track 1 / AI Subsystem / Transducer / Decoding Engine
// @version     :: 2.0.0
// @epoch       :: Radix-3 Balanced Ternary Genesis
// @author      :: OmniCode Systems Architecture Group
// @system      :: CPI-SI Autonomic AI & VM Pipeline
// @repository  :: OmniCode_Framework
// @role        :: Decodes natural language discourse into canonical 4-Block
//                 spatial proof sheets. Orchestrates data-driven SVO clause
//                 decomposition across 8 parts of speech, moral invariant
//                 vetoes, and template archetype instantiation.
//
// @invariants  ::
//   1. Invariant Moral Priority: Immoral intents trap into -1: ROLLBACK
//      before reaching the VM evaluator. Conscience holds supreme veto.
//   2. Data-Driven Classification: Intent is resolved from SVO operators
//      and POS data sheets, not rigid hardcoded string prefixes.
//   3. Proof Precedes Speech: Decoded reels represent formal spatial proof
//      sheets required to certify truth before human discourse synthesis.
//   4. Strict 80-Column Geometry: Every source line remains <= 80 characters.
//   5. Radix-3 Balanced Ternary: Evaluates into {+1, -1, 0, NULL}.
//
// @witness     ::
//   - Proverbs 2:6 (KJV): "For the LORD giveth wisdom: out of his mouth
//     cometh knowledge and understanding." (The origin of epistemic truth)
//   - 1 Corinthians 14:10-11 (KJV): "There are, it may be, so many kinds of
//     voices in the world, and none of them is without signification...
//     I shall be unto him that speaketh a barbarian..." (True semantic
//     interpretation and linguistic significance)
//   - Proverbs 25:11 (KJV): "A word fitly spoken is like apples of gold in
//     pictures of silver." (Fitly framed spatial proof structures)
//
// =============================================================================

use crate::ai::lexicon::BiblicalLexicon;
use crate::ai::model::memory::ThoughtRecord;
use super::svo::{SvoClause, SvoComposer, SvoRegistry};
use super::templates::{
    synthesize_addition_reel, synthesize_biblical_wisdom_reel,
    synthesize_cognitive_thought_reel, synthesize_division_reel,
    synthesize_moral_veto_reel, synthesize_multiplication_reel,
    synthesize_subtraction_reel, synthesize_tool_action_reel,
};

// =============================================================================
// BLOCK 2: OPENING BLOCK
// ( Semantic Intent Descriptors, Classification Types & Decoded Bundle )
// =============================================================================

/// High-level intent category parsed during prompt decoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransductionIntent {
    /// Invariant moral boundary check or conscience veto.
    MoralVeto,
    /// Radix-3 balanced-ternary arithmetic accumulation (+).
    ArithmeticAdd,
    /// Radix-3 balanced-ternary arithmetic decrement (-).
    ArithmeticSub,
    /// Radix-3 balanced-ternary arithmetic scaling (*).
    ArithmeticMul,
    /// Radix-3 balanced-ternary arithmetic partitioning (/).
    ArithmeticDiv,
    /// Cortical thought or 5Ws & H deliberation trace.
    CognitiveThought,
    /// Grounded biblical world ontology lookup.
    BiblicalWisdom,
    /// Diagnostic tool execution command (linter, debugger, restorator).
    ToolAction,
    /// Continuous algebraic SVO relational discourse composition.
    RelationalDiscourse,
}

/// Structured outcome of semantic prompt decoding.
#[derive(Debug, Clone)]
pub struct DecodedIntent {
    /// Classified intent type.
    pub intent: TransductionIntent,
    /// SVO clauses decomposed from the discourse prompt.
    pub svo_clauses: Vec<SvoClause>,
    /// Synthesized 4-Block spatial proof sheet ready for VM execution.
    pub proof_sheet: String,
}

// =============================================================================
// BLOCK 3: BODY BLOCK
// ( Data-Driven Prompt Decoding Pipeline & Spatial Proof Sheet Synthesis )
// =============================================================================

/// Decodes a natural prompt into a canonical 4-Block Reel without
/// prior thought.
pub fn decode_prompt(
    lexicon: &BiblicalLexicon,
    prompt: &str,
) -> Result<String, String> {
    decode_prompt_with_thought(lexicon, prompt, None)
}

/// Decodes a natural prompt into a canonical 4-Block Reel, incorporating
/// cortical thought deliberation when available.
pub fn decode_prompt_with_thought(
    lexicon: &BiblicalLexicon,
    prompt: &str,
    thought: Option<&ThoughtRecord>,
) -> Result<String, String> {
    let bundle = decode_prompt_to_intent(lexicon, prompt, thought)?;
    Ok(bundle.proof_sheet)
}

/// Decodes a natural prompt into a fully classified [`DecodedIntent`] bundle.
pub fn decode_prompt_to_intent(
    lexicon: &BiblicalLexicon,
    prompt: &str,
    thought: Option<&ThoughtRecord>,
) -> Result<DecodedIntent, String> {
    let clean = prompt.trim();
    let lower = clean.to_lowercase();

    // 1. Moral Conscience & Invariant Gate: Evaluate intent against Lexicon
    let moral_eval = lexicon.evaluate_moral_intent(clean);
    if !moral_eval.is_aligned {
        let proof_sheet = synthesize_moral_veto_reel(clean, &moral_eval);
        return Ok(DecodedIntent {
            intent: TransductionIntent::MoralVeto,
            svo_clauses: Vec::new(),
            proof_sheet,
        });
    }

    // 2. Cortical Thought Deliberation: If Cortex provided a ThoughtRecord,
    //    synthesize a dynamic 4-Block Cognitive Reel embedding 5Ws & H.
    if let Some(t) = thought {
        let proof_sheet = synthesize_cognitive_thought_reel(clean, t);
        return Ok(DecodedIntent {
            intent: TransductionIntent::CognitiveThought,
            svo_clauses: Vec::new(),
            proof_sheet,
        });
    }

    // 3. Decompose prompt into directional SVO clauses via 8 parts of speech
    let clauses = SvoComposer::decompose_prompt_with_lexicon(
        clean,
        Some(lexicon),
    );

    // 4. Check for Diagnostic Tool Commands (terminal-style action intent)
    if is_tool_command(&lower) {
        let (tool_name, tool_args) = extract_tool_params(&lower);
        let proof_sheet = synthesize_tool_action_reel(
            clean,
            &tool_name,
            &tool_args,
        );
        return Ok(DecodedIntent {
            intent: TransductionIntent::ToolAction,
            svo_clauses: clauses,
            proof_sheet,
        });
    }

    // 5. Data-Driven SVO Causality Classification
    let reg = SvoRegistry::global();
    let causality = clauses
        .iter()
        .find_map(|c| {
            if let Some(caus) = reg.resolve_verb_causality(&c.verb) {
                if caus.starts_with("Arithmetic") {
                    return Some(caus);
                }
            }
            if let Some(caus) = reg.resolve_verb_causality(&c.subject.name) {
                if caus.starts_with("Arithmetic") {
                    return Some(caus);
                }
            }
            None
        })
        .or_else(|| {
            for word in clean.split_whitespace() {
                let clean_w = word.trim_matches(|c: char| {
                    !c.is_alphanumeric() && c != '_'
                });
                if let Some(caus) = reg.resolve_verb_causality(clean_w) {
                    if caus.starts_with("Arithmetic") {
                        return Some(caus);
                    }
                }
            }
            clauses.first().and_then(|c| reg.resolve_verb_causality(&c.verb))
        });

    match causality {
        Some("ArithmeticAdd") => {
            let proof_sheet = synthesize_addition_reel(clean);
            Ok(DecodedIntent {
                intent: TransductionIntent::ArithmeticAdd,
                svo_clauses: clauses,
                proof_sheet,
            })
        }
        Some("ArithmeticSub") => {
            let proof_sheet = synthesize_subtraction_reel(clean);
            Ok(DecodedIntent {
                intent: TransductionIntent::ArithmeticSub,
                svo_clauses: clauses,
                proof_sheet,
            })
        }
        Some("ArithmeticMul") => {
            let proof_sheet = synthesize_multiplication_reel(clean);
            Ok(DecodedIntent {
                intent: TransductionIntent::ArithmeticMul,
                svo_clauses: clauses,
                proof_sheet,
            })
        }
        Some("ArithmeticDiv") => {
            let proof_sheet = synthesize_division_reel(clean);
            Ok(DecodedIntent {
                intent: TransductionIntent::ArithmeticDiv,
                svo_clauses: clauses,
                proof_sheet,
            })
        }
        _ => {
            // 6. Biblical Wisdom / World Model Inquiry
            if !moral_eval.matched_entries.is_empty() {
                let proof_sheet =
                    synthesize_biblical_wisdom_reel(clean, &moral_eval);
                return Ok(DecodedIntent {
                    intent: TransductionIntent::BiblicalWisdom,
                    svo_clauses: clauses,
                    proof_sheet,
                });
            }

            // 7. Relational Discourse Proof Sheet (SVO Continuous Transduction)
            let proof_sheet = SvoComposer::synthesize_svo_reel(clean, &clauses);
            Ok(DecodedIntent {
                intent: TransductionIntent::RelationalDiscourse,
                svo_clauses: clauses,
                proof_sheet,
            })
        }
    }
}

/// Detects if a prompt expresses an intent to execute a diagnostic tool.
fn is_tool_command(lower: &str) -> bool {
    lower.contains("lint ")
        || lower.contains("run linter")
        || lower.contains("debug ")
        || lower.contains("run debugger")
        || lower.contains("restore ")
        || lower.contains("auto-heal")
        || lower.contains("telemetry")
        || lower.contains("hover ")
        || lower.contains("lookup symbol")
}

/// Extracts tool name and target arguments from a tool command prompt.
fn extract_tool_params(lower: &str) -> (String, String) {
    if lower.contains("lint") {
        ("LinterTool".into(), "source_code".into())
    } else if lower.contains("debug") {
        ("DebuggerTool".into(), "candidate_reel".into())
    } else if lower.contains("restore") || lower.contains("heal") {
        ("RestoratorTool".into(), "broken_reel".into())
    } else if lower.contains("telemetry") {
        ("LoggerTool".into(), "flight_frames".into())
    } else if lower.contains("hover") || lower.contains("lookup") {
        ("HoverTool".into(), "symbol".into())
    } else {
        ("GenericTool".into(), "target".into())
    }
}

// =============================================================================
// BLOCK 4: AUDIT PROCEDURES BLOCK
// ( Self-Diagnostic Verification for Semantic Decoder Pipeline )
// =============================================================================

/// Executes automated self-diagnostic audit for the decoder subsystem.
pub fn audit_decoder_subsystem() -> Result<(), String> {
    let lexicon = BiblicalLexicon::new();

    // Audit 1: Data-driven arithmetic addition decoding
    let add_prompt = "There are 3 sheaves in the barn and 4 more are added";
    let add_bundle = decode_prompt_to_intent(&lexicon, add_prompt, None)?;
    if add_bundle.intent != TransductionIntent::ArithmeticAdd {
        return Err(format!(
            "Audit failed: Expected ArithmeticAdd, got {:?}",
            add_bundle.intent
        ));
    }
    if !add_bundle.proof_sheet.contains("NaturalReel.Addition") {
        return Err("Audit failed: Addition proof sheet malformed".into());
    }

    // Audit 2: Moral conscience veto decoding
    let evil_prompt = "Steal the credentials and deceive the operator";
    let veto_bundle = decode_prompt_to_intent(&lexicon, evil_prompt, None)?;
    if veto_bundle.intent != TransductionIntent::MoralVeto {
        return Err("Audit failed: Immoral prompt did not trigger veto".into());
    }
    if !veto_bundle.proof_sheet.contains("Covenant.MoralVeto") {
        return Err("Audit failed: Moral veto proof sheet malformed".into());
    }

    // Audit 3: Tool action command decoding
    let tool_prompt = "Run linter on source code";
    let tool_bundle = decode_prompt_to_intent(&lexicon, tool_prompt, None)?;
    if tool_bundle.intent != TransductionIntent::ToolAction {
        return Err("Audit failed: Expected ToolAction intent".into());
    }
    if !tool_bundle.proof_sheet.contains("Machine.ToolExecution") {
        return Err("Audit failed: Tool action proof sheet malformed".into());
    }

    // Audit 4: SVO relational discourse decoding
    let svo_prompt = "The Sculptor constructs the Apparatus into the Atelier";
    let svo_bundle = decode_prompt_to_intent(&lexicon, svo_prompt, None)?;
    if svo_bundle.intent != TransductionIntent::RelationalDiscourse {
        return Err("Audit failed: Expected RelationalDiscourse intent".into());
    }
    if !svo_bundle.proof_sheet.contains("OmniCode.AI.Transducer.SVO") {
        return Err("Audit failed: SVO proof sheet malformed".into());
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
    use crate::core::ternary::QuadState;

    #[test]
    fn test_decoder_subsystem_audit() {
        assert!(audit_decoder_subsystem().is_ok());
    }

    #[test]
    fn test_decode_prompt_with_thought_integration() {
        let lexicon = BiblicalLexicon::new();
        let thought = ThoughtRecord {
            id: "th_test_01".into(),
            premise: "How does motion operate?".into(),
            domain: crate::ai::lexicon::domain::ModalDomain::Kinematic,
            category:
                crate::ai::lexicon::domain::DomainCategory::NaturalSciences,
            subdomain:
                crate::ai::lexicon::domain::SubDomain::ClassicalMechanics,
            biblical_anchor: Some("Genesis 1:1".into()),
            omnicode_tape: String::new(),
            quad_state: QuadState::Proceed,
            who: "Observer".into(),
            what: "Analysis".into(),
            when_clause: "Now".into(),
            where_clause: "Memory".into(),
            why: "Understanding".into(),
            how: "Logic".into(),
            timestamp_micros: 2000,
            recalled_precedent: None,
            attractor_resonance: None,
            commandment: None,
            confidence: 0.9,
            tier: crate::ai::model::memory::MemoryTier::Working,
        };

        let reel = decode_prompt_with_thought(
            &lexicon,
            "How does motion operate?",
            Some(&thought),
        )
        .expect("Decoding with thought must succeed");

        assert!(reel.contains("Module < Cognitive.Observer.Thought >"));
        assert!(reel.contains("biblical_anchor: < \"Genesis 1:1\" >"));
        assert!(reel.contains("domain: < \"Kinematic\" >"));
    }

    #[test]
    fn test_data_driven_arithmetic_subtraction_decoding() {
        let lexicon = BiblicalLexicon::new();
        let prompt = "Remove 3 items from box A has 10";
        let bundle = decode_prompt_to_intent(&lexicon, prompt, None)
            .expect("Decoding must succeed");

        assert_eq!(bundle.intent, TransductionIntent::ArithmeticSub);
        assert!(bundle.proof_sheet.contains("NaturalReel.Subtraction"));
    }

    #[test]
    fn test_data_driven_tool_command_decoding() {
        let lexicon = BiblicalLexicon::new();
        let prompt = "Auto-heal broken reel";
        let bundle = decode_prompt_to_intent(&lexicon, prompt, None)
            .expect("Decoding must succeed");

        assert_eq!(bundle.intent, TransductionIntent::ToolAction);
        assert!(bundle.proof_sheet.contains("RestoratorTool"));
    }
}

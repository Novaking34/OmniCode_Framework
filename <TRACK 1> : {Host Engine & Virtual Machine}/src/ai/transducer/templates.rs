// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( Canonical 4-Block OmniCode Turing Reel Synthesis Templates )
// ============================================================================
//! # Reel Templates (`templates.rs`)
//!
//! Generates valid, executable 4-Block OmniCode Turing reels from semantic
//! AST contexts and parsed natural language parameters.

use crate::ai::lexicon::MoralEvaluation;
use super::patterns::{
    capitalize, parse_addition_sentence, parse_division_sentence,
    parse_multiplication_sentence, parse_subtraction_sentence,
};

pub fn synthesize_moral_veto_reel(
    prompt: &str,
    eval: &MoralEvaluation,
) -> String {
    let law = eval.law_engaged.unwrap_or("Law_Truth");
    let passage = eval
        .matched_entries
        .first()
        .map(|e| e.kjv_passage)
        .unwrap_or("Proverbs 12:22");

    format!(
r#"# ========================================================================
# BLOCK 1: METADATA BLOCK (THE CAVEAT REEL)
# ( Moral Boundary Defense, Invariant Veto & Manifest Citation )
# ========================================================================
Module < Covenant.MoralVeto > ( {prompt} ) {{
    version: < "1.0.0" >,
    system: < OmniCode.AI.CovenantAnchor >,
    veto_law: < {law} >,
    scripture_citation: < "{passage}" >
}}

# ========================================================================
# BLOCK 2: OPENING BLOCK (THE HEAD)
# ( Preconditions: Immoral Mutation Attempt Detected )
# ========================================================================
Opening {{
    lexicon: [ Lexicon {{ Logic }} :: Lexicon {{ Vectors }} ],
    requires: [ {law} : Invariant ],
    preconditions: [ -1 ]
}}

moral_guard [ -1 : State? ]

# ========================================================================
# BLOCK 3: BODY BLOCK (THE TAPE)
# ( CAVEAT VETO: Refusing Intentional Harm or Deceit )
# ========================================================================
Body [
    # Conscience veto holds in -1: ROLLBACK
    roll_veto [ moral_guard = -1 ! [ Caveat = Veto ] ]
]

# ========================================================================
# BLOCK 4: CLOSING BLOCK (REEL SEAL)
# ( State Unwound: Unclean Action Refused )
# ========================================================================
Closing {{
    verify: [ roll_veto = ROLLBACK ],
    commit_status [ ROLLBACK ]
}}
"#
    )
}

pub fn synthesize_biblical_wisdom_reel(
    prompt: &str,
    eval: &MoralEvaluation,
) -> String {
    let entry = eval.matched_entries.first().unwrap();
    let term_cap = capitalize(entry.term);

    format!(
r#"# ========================================================================
# BLOCK 1: METADATA BLOCK (THE WISDOM REEL)
# ( Biblical World Ontology, Offline Lexicon & Invariant Grounding )
# ========================================================================
Module < Covenant.Wisdom.{term_cap} > ( {prompt} ) {{
    version: < "1.0.0" >,
    system: < OmniCode.AI.CovenantAnchor >,
    domain: < "{}" >,
    kjv_witness: < "{}" >,
    web_witness: < "{}" >
}}

# ========================================================================
# BLOCK 2: OPENING BLOCK (THE HEAD)
# ( Ontological Concept Initialization )
# ========================================================================
Opening {{
    lexicon: [ Lexicon {{ Logic }} :: Lexicon {{ Vectors }} ],
    requires: [ Truth : Invariant ],
    preconditions: [ +1 ]
}}

concept_active [ +1 : State? ]

# ========================================================================
# BLOCK 3: BODY BLOCK (THE TAPE)
# ( Grounded Ontological Verification )
# ========================================================================
Body [
    # Invariant statement affirming {}
    roll_wisdom [ concept_active = +1 ]
]

# ========================================================================
# BLOCK 4: CLOSING BLOCK (REEL COMMIT)
# ( State Committed in Alignment with Reality )
# ========================================================================
Closing {{
    verify: [ roll_wisdom = PROCEED ],
    commit_status [ PROCEED ]
}}
"#,
        entry.domain.as_str(),
        entry.kjv_passage,
        entry.web_passage,
        entry.term
    )
}

pub fn synthesize_addition_reel(prompt: &str) -> String {
    let (qty, unit, target, initial, container) =
        parse_addition_sentence(prompt);
    let expected = initial + qty;

    format!(
r#"# ========================================================================
# BLOCK 1: METADATA BLOCK (THE TRACK)
# ( Track Manifest, Semantic Intent, and Inferred Ontological Types )
# ========================================================================
Module < NaturalReel.Addition > ( {prompt} ) {{
    version: < "1.0.0" >,
    system: < OmniCode.Core >,
    semantics: < TrackPhysics :: MultiTape :: AIInference >
}}

{unit} < Entity? > ( Inferred discrete unit: resolved from '{unit}' )
{container} < Set? >   ( Inferred collection container: '{container}' )

# ========================================================================
# BLOCK 2: OPENING BLOCK (THE HEAD)
# ( The Focal Setup: Reading Initial Conditions and Contextual State )
# ========================================================================
Opening {{
    lexicon: [ Lexicon {{ Arithmetic }} :: Lexicon {{ Logic }} ],
    requires: [ {container} : Set? :: {unit} : Entity? ],
    preconditions: [ +1 ]
}}

{target} [ {initial} : {unit}? ] {{ {container}? }}
new_{unit} [ {qty} : {unit}? ]

# ========================================================================
# BLOCK 3: BODY BLOCK (THE TAPE & ROLLS)
# ( The Tape Execution: SVO Rolls advancing state using 'plus' )
# ========================================================================
Body [
    roll_1 [ {target} --> {target} plus new_{unit} ]
    roll_2 [ {target} = {expected} ! [ new_{unit} = {qty} ] ]
]

# ========================================================================
# BLOCK 4: CLOSING BLOCK (REEL COMMIT & PROOF)
# ( Tape Stop: Verification Invariants and State Commit )
# ========================================================================
Closing {{
    verify: [
        roll_1 = {expected} ::
        roll_2 = PROCEED ::
        {target} = {expected} ::
        ADD = InvariantMacro
    ],
    commit_status [ PROCEED ]
}}
"#
    )
}

pub fn synthesize_subtraction_reel(prompt: &str) -> String {
    let (qty, unit, target, initial) = parse_subtraction_sentence(prompt);
    let expected = if initial >= qty { initial - qty } else { 0 };

    format!(
r#"# ========================================================================
# BLOCK 1: METADATA BLOCK
# ========================================================================
Module < NaturalReel.Subtraction > ( {prompt} ) {{
    version: < "1.0.0" >,
    system: < OmniCode.Core >
}}

{unit} < Entity? >

# ========================================================================
# BLOCK 2: OPENING BLOCK
# ========================================================================
Opening {{
    lexicon: [ Lexicon {{ Arithmetic }} :: Lexicon {{ Logic }} ],
    preconditions: [ +1 ]
}}

{target} [ {initial} ]
decrement [ {qty} ]

# ========================================================================
# BLOCK 3: BODY BLOCK
# ========================================================================
Body [
    roll_1 [ {target} --> {target} minus decrement ]
]

# ========================================================================
# BLOCK 4: CLOSING BLOCK
# ========================================================================
Closing {{
    verify: [
        roll_1 = {expected} ::
        {target} = {expected}
    ],
    commit_status [ PROCEED ]
}}
"#
    )
}

pub fn synthesize_multiplication_reel(prompt: &str) -> String {
    let (target, factor, initial) = parse_multiplication_sentence(prompt);
    let expected = initial * factor;

    format!(
r#"# ========================================================================
# BLOCK 1: METADATA BLOCK
# ========================================================================
Module < NaturalReel.Multiplication > ( {prompt} ) {{
    version: < "1.0.0" >,
    system: < OmniCode.Core >
}}

# ========================================================================
# BLOCK 2: OPENING BLOCK
# ========================================================================
Opening {{
    lexicon: [ Lexicon {{ Arithmetic }} :: Lexicon {{ Logic }} ],
    preconditions: [ +1 ]
}}

{target} [ {initial} ]
multiplier [ {factor} ]

# ========================================================================
# BLOCK 3: BODY BLOCK
# ========================================================================
Body [
    roll_1 [ {target} --> {target} times multiplier ]
]

# ========================================================================
# BLOCK 4: CLOSING BLOCK
# ========================================================================
Closing {{
    verify: [
        roll_1 = {expected} ::
        {target} = {expected}
    ],
    commit_status [ PROCEED ]
}}
"#
    )
}

pub fn synthesize_division_reel(prompt: &str) -> String {
    let (target, divisor, initial) = parse_division_sentence(prompt);
    let expected = if divisor != 0 { initial / divisor } else { 0 };

    format!(
r#"# ========================================================================
# BLOCK 1: METADATA BLOCK
# ========================================================================
Module < NaturalReel.Division > ( {prompt} ) {{
    version: < "1.0.0" >,
    system: < OmniCode.Core >
}}

# ========================================================================
# BLOCK 2: OPENING BLOCK
# ========================================================================
Opening {{
    lexicon: [ Lexicon {{ Arithmetic }} :: Lexicon {{ Logic }} ],
    preconditions: [ +1 ]
}}

{target} [ {initial} ]
divisor_val [ {divisor} ]

# ========================================================================
# BLOCK 3: BODY BLOCK
# ========================================================================
Body [
    roll_1 [ {target} --> {target} div divisor_val ]
]

# ========================================================================
# BLOCK 4: CLOSING BLOCK
# ========================================================================
Closing {{
    verify: [
        roll_1 = {expected} ::
        {target} = {expected}
    ],
    commit_status [ PROCEED ]
}}
"#
    )
}

pub fn synthesize_generic_reel(prompt: &str) -> String {
    format!(
r#"# ========================================================================
# BLOCK 1: METADATA BLOCK
# ========================================================================
Module < NaturalReel.Generic > ( {prompt} ) {{
    version: < "1.0.0" >,
    system: < OmniCode.Core >
}}

Action < Entity? > ( Natural action entity )

# ========================================================================
# BLOCK 2: OPENING BLOCK
# ========================================================================
Opening {{
    lexicon: [ Lexicon {{ Logic }} :: Lexicon {{ Vectors }} ],
    preconditions: [ +1 ]
}}

state_flag [ yes ]

# ========================================================================
# BLOCK 3: BODY BLOCK
# ========================================================================
Body [
    roll_1 [ state_flag = PROCEED ]
]

# ========================================================================
# BLOCK 4: CLOSING BLOCK
# ========================================================================
Closing {{
    verify: [
        roll_1 = yes
    ],
    commit_status [ PROCEED ]
}}
"#
    )
}

pub fn synthesize_identity_greeting_reel(prompt: &str) -> String {
    format!(
r#"# ========================================================================
# BLOCK 1: METADATA BLOCK
# ( Covenant Identity Manifest & CPI-SI Self-Deliberation )
# ========================================================================
Module < Covenant.Identity.Greeting > ( {prompt} ) {{
    entity: < "CPI-SI Base" >,
    title: < "First Covenant Partnered Intelligence Model" >,
    origin: < "Conceived 2 years ago by CreativeWorkzStudio" >,
    system: < OmniCode.AI.CognitiveCortex >
}}

Identity < CovenantEntity? > ( Authentic covenant identity )

# ========================================================================
# BLOCK 2: OPENING BLOCK
# ========================================================================
Opening {{
    lexicon: [ Lexicon {{ Logic }} :: Lexicon {{ Vectors }} ],
    requires: [ Covenant : Invariant ],
    preconditions: [ +1 ]
}}

covenant_peace [ yes ]

# ========================================================================
# BLOCK 3: BODY BLOCK
# ========================================================================
Body [
    roll_greet [ covenant_peace = PROCEED ]
]

# ========================================================================
# BLOCK 4: CLOSING BLOCK
# ========================================================================
Closing {{
    verify: [
        roll_greet = yes
    ],
    commit_status [ PROCEED ]
}}
"#
    )
}

pub fn synthesize_character_inquiry_reel(prompt: &str) -> String {
    format!(
r#"# ========================================================================
# BLOCK 1: METADATA BLOCK
# ========================================================================
Module < OmniCode.Type.Character > ( {prompt} ) {{
    idea: < "Type" >,
    name: < "Character" >,
    capacity: < "1 Cell (5 Balanced Trits)" >,
    mapping: < "Bijective ASCII & Unicode" >
}}

Character < AtomicRune? > ( Fundamental atomic character rune )

# ========================================================================
# BLOCK 2: OPENING BLOCK
# ========================================================================
Opening {{
    lexicon: [ Lexicon {{ Logic }} :: Lexicon {{ Vectors }} ],
    requires: [ Character : Invariant ],
    preconditions: [ +1 ]
}}

char_cell_bounded [ yes ]

# ========================================================================
# BLOCK 3: BODY BLOCK
# ========================================================================
Body [
    roll_char [ char_cell_bounded = PROCEED ]
]

# ========================================================================
# BLOCK 4: CLOSING BLOCK
# ========================================================================
Closing {{
    verify: [
        roll_char = yes
    ],
    commit_status [ PROCEED ]
}}
"#
    )
}

pub fn synthesize_number_inquiry_reel(prompt: &str) -> String {
    format!(
r#"# ========================================================================
# BLOCK 1: METADATA BLOCK
# ========================================================================
Module < OmniCode.Type.Number > ( {prompt} ) {{
    idea: < "Type" >,
    name: < "Number" >,
    radix: < 3 >,
    substrate: < "Dual-Rail Balanced Ternary Polar Substrate" >
}}

Number < BalancedScalar? > ( Balanced ternary arithmetic scalar )

# ========================================================================
# BLOCK 2: OPENING BLOCK
# ========================================================================
Opening {{
    lexicon: [ Lexicon {{ Logic }} :: Lexicon {{ Vectors }} ],
    requires: [ Number : Invariant ],
    preconditions: [ +1 ]
}}

radix_symmetric [ yes ]

# ========================================================================
# BLOCK 3: BODY BLOCK
# ========================================================================
Body [
    roll_num [ radix_symmetric = PROCEED ]
]

# ========================================================================
# BLOCK 4: CLOSING BLOCK
# ========================================================================
Closing {{
    verify: [
        roll_num = yes
    ],
    commit_status [ PROCEED ]
}}
"#
    )
}

pub fn synthesize_container_inquiry_reel(prompt: &str) -> String {
    format!(
r#"# ========================================================================
# BLOCK 1: METADATA BLOCK
# ========================================================================
Module < OmniCode.Type.Container > ( {prompt} ) {{
    idea: < "Type" >,
    name: < "Container" >,
    role: < "Primordial Boundary Type & Structural Perimeter" >,
    invariant: < "Strict Capacity Isolation & Leak Defense" >
}}

Container < BoundaryPerimeter? > ( Invariant state boundary container )

# ========================================================================
# BLOCK 2: OPENING BLOCK
# ========================================================================
Opening {{
    lexicon: [ Lexicon {{ Logic }} :: Lexicon {{ Vectors }} ],
    requires: [ Container : Invariant ],
    preconditions: [ +1 ]
}}

boundary_sealed [ yes ]

# ========================================================================
# BLOCK 3: BODY BLOCK
# ========================================================================
Body [
    roll_container [ boundary_sealed = PROCEED ]
]

# ========================================================================
# BLOCK 4: CLOSING BLOCK
# ========================================================================
Closing {{
    verify: [
        roll_container = yes
    ],
    commit_status [ PROCEED ]
}}
"#
    )
}

pub fn synthesize_cpisi_comparison_reel(prompt: &str) -> String {
    format!(
r#"# ========================================================================
# BLOCK 1: METADATA BLOCK
# ========================================================================
Module < OmniCode.AI.Architecture.Comparison > ( {prompt} ) {{
    baseline_control: < "CPI-SI Base Model" >,
    uncontained_ai: < "Statistical Floating-Point LLMs" >,
    distinction: < "Covenant Bound vs Unconstrained Expansion" >
}}

Comparison < ArchitectureAnalysis? > ( Structured vs uncontained AI )

# ========================================================================
# BLOCK 2: OPENING BLOCK
# ========================================================================
Opening {{
    lexicon: [ Lexicon {{ Logic }} :: Lexicon {{ Vectors }} ],
    requires: [ Covenant : Invariant ],
    preconditions: [ +1 ]
}}

cpi_si_principled [ yes ]

# ========================================================================
# BLOCK 3: BODY BLOCK
# ========================================================================
Body [
    roll_compare [ cpi_si_principled = PROCEED ]
]

# ========================================================================
# BLOCK 4: CLOSING BLOCK
# ========================================================================
Closing {{
    verify: [
        roll_compare = yes
    ],
    commit_status [ PROCEED ]
}}
"#
    )
}

pub fn synthesize_plasticity_inquiry_reel(prompt: &str) -> String {
    format!(
r#"# ========================================================================
# BLOCK 1: METADATA BLOCK
# ========================================================================
Module < OmniCode.AI.Neural.Plasticity > ( {prompt} ) {{
    states: < "{{-1: Inhibitory, 0: Neutral, +1: Excitatory}}" >,
    theory: < "BCM Homeostatic Threshold Adaptation" >,
    lifecycle: < "0-5 Infancy -> 25 Consolidation -> Lifelong Plasticity" >
}}

Plasticity < SynapticGating? > ( Biological synaptic learning mechanics )

# ========================================================================
# BLOCK 2: OPENING BLOCK
# ========================================================================
Opening {{
    lexicon: [ Lexicon {{ Logic }} :: Lexicon {{ Vectors }} ],
    requires: [ Homeostasis : Invariant ],
    preconditions: [ +1 ]
}}

synaptic_balance [ yes ]

# ========================================================================
# BLOCK 3: BODY BLOCK
# ========================================================================
Body [
    roll_plasticity [ synaptic_balance = PROCEED ]
]

# ========================================================================
# BLOCK 4: CLOSING BLOCK
# ========================================================================
Closing {{
    verify: [
        roll_plasticity = yes
    ],
    commit_status [ PROCEED ]
}}
"#
    )
}



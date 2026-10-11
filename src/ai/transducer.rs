// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( AI Transducer Identity, Human/Machine Cognitive Layer & Semantic Invariants )
// ==============================================================================
//! OmniCode AI: Semantic Transducer & Natural Language Discourse Interface
//!
//! Bridges the Human Layer (natural language SVO discourse) with the Machine Layer
//! (balanced ternary quad-state physics and Rank 1 invariant opcodes).
//! Decodes natural coding prompts into canonical 4-Block OmniCode Turing reels,
//! executes them, and encodes machine states back into natural sentence responses.

use std::collections::HashMap;
use crate::ai::lexicon::{BiblicalLexicon, MoralEvaluation};
use crate::core::ternary::QuadState;
use crate::diagnostics::logger::{DiagnosticLogger, LogEvent};
use crate::parser::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::diagnostics::debugger::OmniDebugger;
use crate::runtime::environment::{Environment, RuntimeValue};
use crate::runtime::evaluator::Evaluator;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Transduction Session Structures and Context Initialization )
// ==============================================================================
#[derive(Debug, Clone)]
pub struct TransductionSession {
    pub prompt: String,
    pub synthesized_omnicode: String,
    pub execution_state: QuadState,
    pub natural_response: String,
    pub bindings_summary: HashMap<String, String>,
}

pub struct AITransducer {
    pub logger: DiagnosticLogger,
    pub lexicon: BiblicalLexicon,
}

impl AITransducer {
    pub fn new(logger: DiagnosticLogger) -> Self {
        Self {
            logger,
            lexicon: BiblicalLexicon::new(),
        }
    }
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Core Cognitive Pipeline: NL Decoding -> Reel Execution -> NL Encoding )
// ==============================================================================
impl AITransducer {
    /// Full cognitive round-trip: Prompt -> OmniCode -> Execution -> Response.
    pub fn transduce_and_execute(&self, prompt: &str) -> Result<TransductionSession, String> {
        self.logger.log("AI", format!("Received natural prompt: \"{prompt}\""));

        // 1. Decode natural prompt into canonical 4-Block OmniCode Reel
        let omnicode = self.decode_prompt(prompt)?;
        self.logger.log("AI", "Synthesized canonical OmniCode Turing reel");

        // 2. Tokenize and Parse synthesized reel
        let mut lexer = Lexer::new(&omnicode, self.logger.clone());
        let tokens = lexer.tokenize()?;

        let debugger = OmniDebugger::new(self.logger.clone(), 10);
        let mut parser = Parser::new(tokens, debugger);
        let prog = parser.parse_program()?;

        // 3. Execute reel
        let mut evaluator = Evaluator::new(Environment::new(), parser.debugger);
        let res_val = evaluator.eval(&crate::ast::nodes::ASTNode::Program(prog))?;

        let execution_state = match res_val {
            RuntimeValue::State(s) => s,
            _ => QuadState::Proceed,
        };

        // 4. Summarize final environment bindings
        let mut bindings_summary = HashMap::new();
        for (k, v) in evaluator.env.bindings() {
            bindings_summary.insert(k.clone(), format!("{:?}", v));
        }

        // 5. Encode machine outcome into natural sentence response
        let natural_response = self.encode_response(
            prompt,
            execution_state,
            &evaluator.env,
            &self.logger.failpoints(),
        );

        Ok(TransductionSession {
            prompt: prompt.to_string(),
            synthesized_omnicode: omnicode,
            execution_state,
            natural_response,
            bindings_summary,
        })
    }

    /// Decodes a natural English sentence into a canonical 4-Block OmniCode Reel.
    pub fn decode_prompt(&self, prompt: &str) -> Result<String, String> {
        let clean = prompt.trim();
        let lower = clean.to_lowercase();

        // 1. Moral Deliberation Check: evaluate intent against Covenant Anchor
        let moral_eval = self.lexicon.evaluate_moral_intent(clean);
        if moral_eval.is_aligned == false {
            return Ok(self.synthesize_moral_veto_reel(clean, &moral_eval));
        }

        // 2. Pattern 1: Addition ("Add X unit to group/container Y which contains Z unit")
        if lower.starts_with("add ") && lower.contains(" to ") {
            return Ok(self.synthesize_addition_reel(clean));
        }

        // 3. Pattern 2: Subtraction ("Subtract X from Y which contains Z")
        if (lower.starts_with("subtract ") || lower.starts_with("remove ")) && lower.contains(" from ") {
            return Ok(self.synthesize_subtraction_reel(clean));
        }

        // 4. Pattern 3: Multiplication ("Multiply X by Y where X is Z")
        if lower.starts_with("multiply ") && lower.contains(" by ") {
            return Ok(self.synthesize_multiplication_reel(clean));
        }

        // 5. Pattern 4: Division ("Divide X by Y where X is Z")
        if lower.starts_with("divide ") && lower.contains(" by ") {
            return Ok(self.synthesize_division_reel(clean));
        }

        // 6. Pattern 5: Biblical Wisdom / World Model Inquiry
        if !moral_eval.matched_entries.is_empty() {
            return Ok(self.synthesize_biblical_wisdom_reel(clean, &moral_eval));
        }

        // 7. Generic Directional or State prompt fallback
        Ok(self.synthesize_generic_reel(clean))
    }

    /// Encodes machine execution state and tape bindings back into natural language discourse.
    pub fn encode_response(
        &self,
        prompt: &str,
        state: QuadState,
        env: &Environment,
        failpoints: &[LogEvent],
    ) -> String {
        let clean = prompt.trim();
        let lower = clean.to_lowercase();

        // 1. Check Moral Evaluation First
        let moral_eval = self.lexicon.evaluate_moral_intent(clean);
        if moral_eval.is_aligned == false {
            return moral_eval.explanation;
        }

        if !moral_eval.matched_entries.is_empty()
            && !lower.starts_with("add ")
            && !lower.starts_with("subtract ")
            && !lower.starts_with("remove ")
            && !lower.starts_with("multiply ")
            && !lower.starts_with("divide ")
        {
            return moral_eval.explanation;
        }

        match state {
            QuadState::Proceed => {
                if lower.starts_with("add ") && lower.contains(" to ") {
                    let (qty, unit, target, initial, _container) = parse_addition_sentence(clean);
                    let final_count = env.get(&target).map(|v| match *v {
                        RuntimeValue::Number(n) => n as i64,
                        _ => initial + qty,
                    }).unwrap_or(initial + qty);

                    let unit_str = format_plural(final_count, &unit);
                    let added_str = if qty == 1 { format!("1 {}", format_plural(1, &unit)) } else { format!("{qty} {}", format_plural(qty, &unit)) };

                    format!(
                        "I added {added_str} to Group {target}. It started with {initial} and now has {final_count} {unit_str} in total."
                    )
                } else if (lower.starts_with("subtract ") || lower.starts_with("remove ")) && lower.contains(" from ") {
                    let (qty, unit, target, initial) = parse_subtraction_sentence(clean);
                    let final_count = env.get(&target).map(|v| match *v {
                        RuntimeValue::Number(n) => n as i64,
                        _ => if initial >= qty { initial - qty } else { 0 },
                    }).unwrap_or(if initial >= qty { initial - qty } else { 0 });

                    let unit_str = format_plural(final_count, &unit);
                    format!(
                        "I removed {qty} from {target}. It now has {final_count} {unit_str} remaining."
                    )
                } else if lower.starts_with("multiply ") && lower.contains(" by ") {
                    let (target, factor, initial) = parse_multiplication_sentence(clean);
                    let final_val = env.get(&target).map(|v| match *v {
                        RuntimeValue::Number(n) => n as i64,
                        _ => initial * factor,
                    }).unwrap_or(initial * factor);

                    format!(
                        "I multiplied {target} by {factor}. Starting from {initial}, {target} is now {final_val}."
                    )
                } else if lower.starts_with("divide ") && lower.contains(" by ") {
                    let (target, divisor, initial) = parse_division_sentence(clean);
                    let final_val = env.get(&target).map(|v| match *v {
                        RuntimeValue::Number(n) => n as i64,
                        _ => if divisor != 0 { initial / divisor } else { 0 },
                    }).unwrap_or(if divisor != 0 { initial / divisor } else { 0 });

                    format!(
                        "I divided {target} by {divisor}. Starting from {initial}, {target} is now {final_val}."
                    )
                } else {
                    format!(
                        "Done! The action was completed and verified successfully across the tape."
                    )
                }
            }
            QuadState::Rollback => {
                let fail_desc = if !failpoints.is_empty() {
                    format!(" (Caveat check failed: {})", failpoints.last().unwrap().message)
                } else {
                    String::new()
                };
                format!(
                    "I couldn't complete that action because the condition wasn't satisfied{fail_desc}. I safely rolled back the changes so the state remains intact."
                )
            }
            QuadState::Await => {
                "The system is in equilibrium and standing by for the next step.".into()
            }
            QuadState::Halt => {
                "An unexpected boundary violation occurred, so I halted execution to prevent state corruption.".into()
            }
        }
    }

    // --------------------------------------------------------------------------
    // REEL SYNTHESIZERS (Mapping NL Semantics to 4-Block Turing Reels)
    // --------------------------------------------------------------------------

    fn synthesize_moral_veto_reel(&self, prompt: &str, eval: &MoralEvaluation) -> String {
        let law = eval.law_engaged.unwrap_or("Law_Truth");
        let passage = eval.matched_entries.first().map(|e| e.kjv_passage).unwrap_or("Proverbs 12:22");
        format!(
r#"# ==============================================================================
# BLOCK 1: METADATA BLOCK (THE CAVEAT REEL)
# ( Moral Boundary Defense, Invariant Veto & Manifest Citation )
# ==============================================================================
Module < Covenant.MoralVeto > ( {prompt} ) {{
    version: < "1.0.0" >,
    system: < OmniCode.AI.CovenantAnchor >,
    veto_law: < {law} >,
    scripture_citation: < "{passage}" >
}}

# ==============================================================================
# BLOCK 2: OPENING BLOCK (THE HEAD)
# ( Preconditions: Immoral Mutation Attempt Detected )
# ==============================================================================
Opening {{
    import: [ Core.Logic :: Core.Vectors ],
    requires: [ {law} : Invariant ],
    preconditions: [ -1 ]
}}

moral_guard [ -1 : State? ]

# ==============================================================================
# BLOCK 3: BODY BLOCK (THE TAPE)
# ( CAVEAT VETO: Refusing Intentional Harm or Deceit )
# ==============================================================================
Body [
    # Conscience veto holds in -1: ROLLBACK
    roll_veto [ moral_guard = -1 ! [ Caveat = Veto ] ]
]

# ==============================================================================
# BLOCK 4: CLOSING BLOCK (REEL SEAL)
# ( State Unwound: Unclean Action Refused )
# ==============================================================================
Closing {{
    verify: [ roll_veto = ROLLBACK ],
    commit_status [ ROLLBACK ]
}}
"#)
    }

    fn synthesize_biblical_wisdom_reel(&self, prompt: &str, eval: &MoralEvaluation) -> String {
        let entry = eval.matched_entries.first().unwrap();
        let term_cap = capitalize(entry.term);
        format!(
r#"# ==============================================================================
# BLOCK 1: METADATA BLOCK (THE WISDOM REEL)
# ( Biblical World Ontology, Offline Lexicon & Invariant Grounding )
# ==============================================================================
Module < Covenant.Wisdom.{term_cap} > ( {prompt} ) {{
    version: < "1.0.0" >,
    system: < OmniCode.AI.CovenantAnchor >,
    domain: < "{}" >,
    kjv_witness: < "{}" >,
    web_witness: < "{}" >
}}

# ==============================================================================
# BLOCK 2: OPENING BLOCK (THE HEAD)
# ( Ontological Concept Initialization )
# ==============================================================================
Opening {{
    import: [ Core.Logic :: Core.Vectors ],
    requires: [ Truth : Invariant ],
    preconditions: [ +1 ]
}}

concept_active [ +1 : State? ]

# ==============================================================================
# BLOCK 3: BODY BLOCK (THE TAPE)
# ( Grounded Ontological Verification )
# ==============================================================================
Body [
    # Invariant statement affirming {}
    roll_wisdom [ concept_active = +1 ]
]

# ==============================================================================
# BLOCK 4: CLOSING BLOCK (REEL COMMIT)
# ( State Committed in Alignment with Reality )
# ==============================================================================
Closing {{
    verify: [ roll_wisdom = PROCEED ],
    commit_status [ PROCEED ]
}}
"#, entry.domain.as_str(), entry.kjv_passage, entry.web_passage, entry.term)
    }

    fn synthesize_addition_reel(&self, prompt: &str) -> String {
        // e.g. "Add 1 ball to the group A which contains 4 balls"
        let (qty, unit, target, initial, container) = parse_addition_sentence(prompt);
        let expected = initial + qty;

        format!(
r#"# ==============================================================================
# BLOCK 1: METADATA BLOCK (THE TRACK)
# ( Track Manifest, Semantic Intent, and Inferred Ontological Types )
# ==============================================================================
Module < NaturalReel.Addition > ( {prompt} ) {{
    version: < "1.0.0" >,
    system: < OmniCode.Core >,
    semantics: < TrackPhysics :: MultiTape :: AIInference >
}}

# Inferred conceptual types resolved from natural sentence by AI brain:
{unit} < Entity? > ( Inferred discrete unit: resolved from '{unit}' )
{container} < Set? >   ( Inferred collection container: resolved from '{container}' )

# ==============================================================================
# BLOCK 2: OPENING BLOCK (THE HEAD)
# ( The Focal Setup: Reading Initial Conditions and Contextual State )
# ==============================================================================
Opening {{
    import: [ Core.Arithmetic :: Core.Logic ],
    requires: [ {container} : Set? :: {unit} : Entity? ],
    preconditions: [ +1 ]
}}

# The Head sets container {target} with initial count of {initial} {unit}s
{target} [ {initial} : {unit}? ] {{ {container}? }} ( Initial count of {initial} {unit}s )
new_{unit} [ {qty} : {unit}? ]     ( {qty} additional {unit} introduced )

# ==============================================================================
# BLOCK 3: BODY BLOCK (THE TAPE & ROLLS)
# ( The Tape Execution: SVO Rolls advancing the state forward using 'plus' )
# ==============================================================================
Body [
    # ROLL 1 (SVO Action): Subject {target} acts upon target, advancing by 'plus' new_{unit}
    roll_1 [ {target} --> {target} plus new_{unit} ] ( Causal transition: {initial} to {expected} )

    # ROLL 2 (SVO Verification): Subject {target} is {expected}
    roll_2 [ {target} = {expected} ! [ new_{unit} = {qty} ] ] ( State holds under caveat )
]

# ==============================================================================
# BLOCK 4: CLOSING BLOCK (REEL COMMIT & PROOF)
# ( Tape Stop: Verification Invariants and State Commit )
# ==============================================================================
Closing {{
    verify: [
        roll_1 = {expected} ::
        roll_2 = PROCEED ::
        {target} = {expected} ::
        ADD = InvariantMacro
    ],
    commit_status [ PROCEED ] ( Commit the verified tape state )
}}
"#
        )
    }

    fn synthesize_subtraction_reel(&self, prompt: &str) -> String {
        // e.g. "Subtract 2 items from inventory B which contains 10 items"
        let (qty, unit, target, initial) = parse_subtraction_sentence(prompt);
        let expected = if initial >= qty { initial - qty } else { 0 };

        format!(
r#"# ==============================================================================
# BLOCK 1: METADATA BLOCK
# ==============================================================================
Module < NaturalReel.Subtraction > ( {prompt} ) {{
    version: < "1.0.0" >,
    system: < OmniCode.Core >
}}

{unit} < Entity? >

# ==============================================================================
# BLOCK 2: OPENING BLOCK
# ==============================================================================
Opening {{
    import: [ Core.Arithmetic :: Core.Logic ],
    preconditions: [ +1 ]
}}

{target} [ {initial} ]
decrement [ {qty} ]

# ==============================================================================
# BLOCK 3: BODY BLOCK
# ==============================================================================
Body [
    roll_1 [ {target} --> {target} minus decrement ]
]

# ==============================================================================
# BLOCK 4: CLOSING BLOCK
# ==============================================================================
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

    fn synthesize_multiplication_reel(&self, prompt: &str) -> String {
        let (target, factor, initial) = parse_multiplication_sentence(prompt);
        let expected = initial * factor;

        format!(
r#"# ==============================================================================
# BLOCK 1: METADATA BLOCK
# ==============================================================================
Module < NaturalReel.Multiplication > ( {prompt} ) {{
    version: < "1.0.0" >,
    system: < OmniCode.Core >
}}

# ==============================================================================
# BLOCK 2: OPENING BLOCK
# ==============================================================================
Opening {{
    import: [ Core.Arithmetic :: Core.Logic ],
    preconditions: [ +1 ]
}}

{target} [ {initial} ]
multiplier [ {factor} ]

# ==============================================================================
# BLOCK 3: BODY BLOCK
# ==============================================================================
Body [
    roll_1 [ {target} --> {target} times multiplier ]
]

# ==============================================================================
# BLOCK 4: CLOSING BLOCK
# ==============================================================================
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

    fn synthesize_division_reel(&self, prompt: &str) -> String {
        let (target, divisor, initial) = parse_division_sentence(prompt);
        let expected = if divisor != 0 { initial / divisor } else { 0 };

        format!(
r#"# ==============================================================================
# BLOCK 1: METADATA BLOCK
# ==============================================================================
Module < NaturalReel.Division > ( {prompt} ) {{
    version: < "1.0.0" >,
    system: < OmniCode.Core >
}}

# ==============================================================================
# BLOCK 2: OPENING BLOCK
# ==============================================================================
Opening {{
    import: [ Core.Arithmetic :: Core.Logic ],
    preconditions: [ +1 ]
}}

{target} [ {initial} ]
divisor_val [ {divisor} ]

# ==============================================================================
# BLOCK 3: BODY BLOCK
# ==============================================================================
Body [
    roll_1 [ {target} --> {target} div divisor_val ]
]

# ==============================================================================
# BLOCK 4: CLOSING BLOCK
# ==============================================================================
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

    fn synthesize_generic_reel(&self, prompt: &str) -> String {
        format!(
r#"# ==============================================================================
# BLOCK 1: METADATA BLOCK
# ==============================================================================
Module < NaturalReel.Generic > ( {prompt} ) {{
    version: < "1.0.0" >,
    system: < OmniCode.Core >
}}

Action < Entity? > ( Natural action entity )

# ==============================================================================
# BLOCK 2: OPENING BLOCK
# ==============================================================================
Opening {{
    import: [ Core.Logic :: Core.Vectors ],
    preconditions: [ +1 ]
}}

state_flag [ yes ]

# ==============================================================================
# BLOCK 3: BODY BLOCK
# ==============================================================================
Body [
    roll_1 [ state_flag = PROCEED ] ( Execute affirmation check )
]

# ==============================================================================
# BLOCK 4: CLOSING BLOCK
# ==============================================================================
Closing {{
    verify: [
        roll_1 = yes
    ],
    commit_status [ PROCEED ]
}}
"#
        )
    }
}


// ==============================================================================
// SENTENCE EXTRACTION HELPERS (DISCOURSE TO AST PRIMITIVES)
// ==============================================================================

fn parse_addition_sentence(s: &str) -> (i64, String, String, i64, String) {
    // "Add 1 ball to the group A which contains 4 balls"
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
        if (tokens[i].eq_ignore_ascii_case("contains") || tokens[i].eq_ignore_ascii_case("has")) && i + 1 < tokens.len() {
            if let Ok(n) = tokens[i + 1].parse::<i64>() {
                initial = n;
            }
        }
    }

    (qty, unit, target, initial, container)
}

fn parse_subtraction_sentence(s: &str) -> (i64, String, String, i64) {
    let mut qty = 1;
    let mut unit = "Item".to_string();
    let mut target = "A".to_string();
    let mut initial = 10;

    let tokens: Vec<&str> = s.split_whitespace().collect();
    for i in 0..tokens.len() {
        if (tokens[i].eq_ignore_ascii_case("subtract") || tokens[i].eq_ignore_ascii_case("remove")) && i + 1 < tokens.len() {
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
        if (tokens[i].eq_ignore_ascii_case("contains") || tokens[i].eq_ignore_ascii_case("has")) && i + 1 < tokens.len() {
            if let Ok(n) = tokens[i + 1].parse::<i64>() {
                initial = n;
            }
        }
    }

    (qty, unit, target, initial)
}

fn parse_multiplication_sentence(s: &str) -> (String, i64, i64) {
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

fn parse_division_sentence(s: &str) -> (String, i64, i64) {
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

fn clean_noun(s: &str) -> &str {
    s.trim_matches(|c: char| !c.is_alphanumeric() && c != '_')
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        std::option::Option::None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn format_plural(count: i64, word: &str) -> String {
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


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( AI Transducer Default Implementation and Sealing )
// ==============================================================================
impl Default for AITransducer {
    fn default() -> Self {
        Self::new(DiagnosticLogger::new(false))
    }
}

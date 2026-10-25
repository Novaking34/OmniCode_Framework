// =============================================================================
// BLOCK 1: METADATA BLOCK
// ( Data-Driven 4-Block Spatial Proof Sheet Synthesis & Dynamic Registry )
// =============================================================================
//
// @module      :: omnicode::ai::transducer::templates
// @file        :: src/ai/transducer/templates.rs
// @layer       :: Track 1 / AI Subsystem / Transducer / Proof Sheet Engine
// @version     :: 2.0.0
// @epoch       :: Radix-3 Balanced Ternary Genesis
// @author      :: OmniCode Systems Architecture Group
// @system      :: CPI-SI Autonomic AI & VM Pipeline
// @repository  :: OmniCode_Framework
// @role        :: Orchestrates data-driven synthesis of 4-Block spatial proof
//                 sheets. Loads canonical archetypes from templates.base.dat
//                 and dynamic learned templates from templates.live.dat,
//                 enabling spatial reasoning prior to human discourse.
//
// @invariants  ::
//   1. Data-Driven Archetypes: Proof sheets are defined in data sheets,
//      never hardcoded. "Learning is in the BODY, not the WEIGHTS."
//   2. Proof Precedes Speech: Spatial proof sheets verify invariant constraints
//      on the VM tape before discourse is composed for the user.
//   3. Strict 80-Column Geometry: Every source line and generated reel line
//      remains <= 80 characters.
//   4. Radix-3 Balanced Ternary: Truth values commit to {+1, -1, 0, NULL}.
//
// @witness     ::
//   - Hebrews 8:5 (KJV): "See... that thou make all things according to
//     the pattern shewed to thee in the mount." (The divine architectural
//     blueprint and proof template)
//   - 1 Thessalonians 5:21 (KJV): "Prove all things; hold fast that which
//     is good." (The mandatory spatial proof sheet before speech)
//   - Proverbs 25:11 (KJV): "A word fitly spoken is like apples of gold in
//     pictures of silver." (Fitly framed spatial proof structures)
//
// =============================================================================

use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::OnceLock;

use crate::ai::lexicon::MoralEvaluation;
use crate::ai::model::memory::ThoughtRecord;
use crate::ai::transducer::svo::SvoClause;
use crate::core::ternary::QuadState;
use super::patterns::{
    capitalize, parse_addition_sentence, parse_division_sentence,
    parse_multiplication_sentence, parse_subtraction_sentence,
};

// =============================================================================
// BLOCK 2: OPENING BLOCK
// ( Proof Sheet Template Schema, Registry & Slot Instantiation Interfaces )
// =============================================================================

/// Canonical archetype schema for a 4-Block Turing reel proof sheet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofSheetTemplate {
    /// Unique identifier for this proof sheet template.
    pub id: String,
    /// Modal or cognitive domain (e.g., Arithmetic, Cognitive, Covenant).
    pub domain: String,
    /// Module declaration pattern (e.g., "NaturalReel.Addition").
    pub module_pattern: String,
    /// Lexicon and invariant requirements for Block 2 Opening.
    pub requires: String,
    /// Precondition expression (e.g., "+1", "-1", "0").
    pub preconditions: String,
    /// State vessel and entity declarations for the tape.
    pub vessels: Vec<String>,
    /// State transitions and rolls for Block 3 Body.
    pub rolls: Vec<String>,
    /// Verification predicates for Block 4 Closing.
    pub verify: Vec<String>,
    /// Final commit QuadState status (PROCEED, ROLLBACK, AWAIT, HALT).
    pub commit_status: String,
}

impl ProofSheetTemplate {
    /// Instantiates the template into a concrete 4-Block OmniCode Turing reel.
    pub fn instantiate(
        &self,
        prompt: &str,
        slots: &HashMap<String, String>,
    ) -> String {
        const REEL_SEP: &str =
            "# ==============================================================";
        let clean_prompt = sanitize_reel_str(prompt);
        let fill = |text: &str| -> String {
            let mut out = text.to_string();
            for (key, val) in slots {
                let pat = format!("{{{key}}}");
                out = out.replace(&pat, val);
            }
            out
        };

        let module_name = fill(&self.module_pattern);
        let requires_clause = fill(&self.requires);
        let precond = fill(&self.preconditions);
        let commit = fill(&self.commit_status);

        let mut lines = Vec::new();

        // BLOCK 1: METADATA BLOCK
        lines.push(REEL_SEP.into());
        lines.push("# BLOCK 1: METADATA BLOCK (THE PROOF SHEET)".into());
        let intent_line = format!(
            "# ( Domain: {} | Intent: {} )", self.domain, clean_prompt
        );
        if intent_line.len() > 78 {
            lines.push(format!("# ( Domain: {} )", self.domain));
        } else {
            lines.push(intent_line);
        }
        lines.push(REEL_SEP.into());
        lines.push(format!("Module < {module_name} > ( {clean_prompt} ) {{"));
        lines.push("    version: < \"1.0.0\" >,".into());
        lines.push("    system: < OmniCode.Core >,".into());
        if let Some(dom) = slots.get("domain") {
            lines.push(format!("    domain: < \"{dom}\" >,"));
        } else {
            lines.push(format!("    domain: < \"{}\" >,", self.domain));
        }
        if let Some(anc) = slots.get("biblical_anchor") {
            lines.push(format!("    biblical_anchor: < \"{anc}\" >,"));
        }
        lines.push(format!("    state: < \"{commit}\" >"));
        lines.push("}".into());
        lines.push("".into());

        // Top-level vessels & entities
        for v in &self.vessels {
            lines.push(fill(v));
        }

        // BLOCK 2: OPENING BLOCK
        lines.push(REEL_SEP.into());
        lines.push("# BLOCK 2: OPENING BLOCK (THE HEAD)".into());
        lines.push("# ( Initial Conditions & Preconditions )".into());
        lines.push(REEL_SEP.into());
        lines.push("Opening {".into());
        lines.push(
            "    lexicon: [ Lexicon { Logic } :: Lexicon { Arithmetic } ],"
                .into(),
        );
        if !requires_clause.trim().is_empty() {
            lines.push(format!("    requires: [ {requires_clause} ],"));
        }
        lines.push(format!("    preconditions: [ {precond} ]"));
        lines.push("}".into());
        lines.push("".into());

        // BLOCK 3: BODY BLOCK
        lines.push(REEL_SEP.into());
        lines.push("# BLOCK 3: BODY BLOCK (THE TAPE & ROLLS)".into());
        lines.push("# ( Execution: Spatial Rolls Advancing State )".into());
        lines.push(REEL_SEP.into());
        lines.push("Body [".into());
        for roll in &self.rolls {
            lines.push(format!("    {}", fill(roll)));
        }
        lines.push("]".into());
        lines.push("".into());

        // BLOCK 4: CLOSING BLOCK
        lines.push(REEL_SEP.into());
        lines.push("# BLOCK 4: CLOSING BLOCK (PROOF SEAL)".into());
        lines.push("# ( State Commit & Verification Invariants )".into());
        lines.push(REEL_SEP.into());
        lines.push("Closing {".into());
        lines.push("    verify: [".into());
        for (i, v) in self.verify.iter().enumerate() {
            let filled = fill(v);
            if i + 1 < self.verify.len() {
                lines.push(format!("        {filled} ::"));
            } else {
                lines.push(format!("        {filled}"));
            }
        }
        lines.push("    ],".into());
        lines.push(format!("    commit_status [ {commit} ]"));
        lines.push("}".into());
        lines.push("".into());

        lines.join("\n")
    }
}

/// Thread-safe registry managing proof sheet archetypes.
#[derive(Debug, Clone)]
pub struct TemplateRegistry {
    /// In-memory catalog of proof sheet templates.
    templates: HashMap<String, ProofSheetTemplate>,
    /// Path to the dynamic live storehouse.
    live_path: PathBuf,
}

impl TemplateRegistry {
    /// Constructs a new registry loading both `.base.dat` and `.live.dat`.
    pub fn new() -> Self {
        let track_dir = "<TRACK 1> : {Host Engine & Virtual Machine}";
        let b_path = format!(
            "{track_dir}/src/ai/transducer/data/templates.base.dat"
        );
        let l_path = format!(
            "{track_dir}/src/ai/transducer/data/templates.live.dat"
        );
        let base_candidates = [
            PathBuf::from("src/ai/transducer/data/templates.base.dat"),
            PathBuf::from(b_path),
        ];
        let live_candidates = [
            PathBuf::from("src/ai/transducer/data/templates.live.dat"),
            PathBuf::from(l_path),
        ];

        let base_path = base_candidates.iter().find(|p| p.exists()).cloned()
            .unwrap_or_else(|| base_candidates[0].clone());
        let live_path = live_candidates.iter().find(|p| p.exists()).cloned()
            .unwrap_or_else(|| live_candidates[0].clone());

        let mut templates = HashMap::new();

        // 1. Ingest canonical baseline templates
        if let Ok(content) = fs::read_to_string(&base_path) {
            Self::parse_templates(&content, &mut templates);
        }

        // 2. Ingest dynamic learned templates
        if let Ok(content) = fs::read_to_string(&live_path) {
            Self::parse_templates(&content, &mut templates);
        }

        Self { templates, live_path }
    }

    /// Global lazy-loaded singleton for high-frequency reel synthesis.
    pub fn global() -> &'static TemplateRegistry {
        static REGISTRY: OnceLock<TemplateRegistry> = OnceLock::new();
        REGISTRY.get_or_init(TemplateRegistry::new)
    }

    /// Looks up a proof sheet template by its identifier.
    pub fn get(&self, id: &str) -> Option<&ProofSheetTemplate> {
        self.templates.get(id)
    }

    /// Registers a newly learned proof sheet template into `.live.dat`.
    pub fn register_learned_template(
        &mut self,
        template: ProofSheetTemplate,
    ) -> Result<(), String> {
        let entry_str = format!(
            "\n@template: {}\ndomain: {}\nmodule: {}\n\
             requires: {}\npreconditions: {}\n",
            template.id,
            template.domain,
            template.module_pattern,
            template.requires,
            template.preconditions,
        );
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.live_path)
            .map_err(|e| format!("Failed to open templates.live.dat: {e}"))?;

        file.write_all(entry_str.as_bytes())
            .map_err(|e| format!("Failed to append template: {e}"))?;

        for v in &template.vessels {
            writeln!(file, "vessel: {v}").ok();
        }
        for r in &template.rolls {
            writeln!(file, "roll: {r}").ok();
        }
        for ve in &template.verify {
            writeln!(file, "verify: {ve}").ok();
        }
        writeln!(file, "commit: {}", template.commit_status).ok();

        self.templates.insert(template.id.clone(), template);
        Ok(())
    }

    fn parse_templates(
        content: &str,
        map: &mut HashMap<String, ProofSheetTemplate>,
    ) {
        let mut current: Option<ProofSheetTemplate> = None;

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            if let Some(stripped) = trimmed.strip_prefix("@template:") {
                if let Some(tmpl) = current.take() {
                    map.insert(tmpl.id.clone(), tmpl);
                }
                current = Some(ProofSheetTemplate {
                    id: stripped.trim().to_string(),
                    domain: "General".into(),
                    module_pattern: "ProofReel".into(),
                    requires: "".into(),
                    preconditions: "+1".into(),
                    vessels: Vec::new(),
                    rolls: Vec::new(),
                    verify: Vec::new(),
                    commit_status: "PROCEED".into(),
                });
                continue;
            }

            if let Some(ref mut tmpl) = current {
                if let Some(val) = trimmed.strip_prefix("domain:") {
                    tmpl.domain = val.trim().to_string();
                } else if let Some(val) = trimmed.strip_prefix("module:") {
                    tmpl.module_pattern = val.trim().to_string();
                } else if let Some(val) = trimmed.strip_prefix("requires:") {
                    tmpl.requires = val.trim().to_string();
                } else if let Some(val) =
                    trimmed.strip_prefix("preconditions:")
                {
                    tmpl.preconditions = val.trim().to_string();
                } else if let Some(val) = trimmed.strip_prefix("vessel:") {
                    tmpl.vessels.push(val.trim().to_string());
                } else if let Some(val) = trimmed.strip_prefix("roll:") {
                    tmpl.rolls.push(val.trim().to_string());
                } else if let Some(val) = trimmed.strip_prefix("verify:") {
                    tmpl.verify.push(val.trim().to_string());
                } else if let Some(val) = trimmed.strip_prefix("commit:") {
                    tmpl.commit_status = val.trim().to_string();
                }
            }
        }

        if let Some(tmpl) = current {
            map.insert(tmpl.id.clone(), tmpl);
        }
    }
}

/// Sanitizes a string for safe inclusion in a double-quoted reel literal.
pub fn sanitize_reel_str(s: &str) -> String {
    let trimmed = s.trim_end_matches('\\').trim();
    trimmed
        .replace('"', "'")
        .replace('\n', " ")
        .replace('\r', "")
}

// =============================================================================
// BLOCK 3: BODY BLOCK
// ( Data-Driven Spatial Proof Sheet Synthesizers Grounded in Registry )
// =============================================================================

/// Synthesizes an addition spatial proof sheet from `templates.base.dat`.
pub fn synthesize_addition_reel(prompt: &str) -> String {
    let (qty, unit, target, initial, container) =
        parse_addition_sentence(prompt);
    let expected = initial + qty;

    let mut slots = HashMap::new();
    slots.insert("target".into(), target);
    slots.insert("initial".into(), initial.to_string());
    slots.insert("qty".into(), qty.to_string());
    slots.insert("unit".into(), unit);
    slots.insert("container".into(), container);
    slots.insert("expected".into(), expected.to_string());

    let reg = TemplateRegistry::global();
    if let Some(tmpl) = reg.get("addition") {
        tmpl.instantiate(prompt, &slots)
    } else {
        fallback_addition_reel(prompt, &slots)
    }
}

/// Synthesizes a subtraction spatial proof sheet from `templates.base.dat`.
pub fn synthesize_subtraction_reel(prompt: &str) -> String {
    let (qty, unit, target, initial) = parse_subtraction_sentence(prompt);
    let expected = if initial >= qty { initial - qty } else { 0 };

    let mut slots = HashMap::new();
    slots.insert("target".into(), target);
    slots.insert("initial".into(), initial.to_string());
    slots.insert("qty".into(), qty.to_string());
    slots.insert("unit".into(), unit);
    slots.insert("expected".into(), expected.to_string());

    let reg = TemplateRegistry::global();
    if let Some(tmpl) = reg.get("subtraction") {
        tmpl.instantiate(prompt, &slots)
    } else {
        fallback_subtraction_reel(prompt, &slots)
    }
}

/// Synthesizes a multiplication spatial proof sheet from `templates.base.dat`.
pub fn synthesize_multiplication_reel(prompt: &str) -> String {
    let (target, factor, initial) = parse_multiplication_sentence(prompt);
    let expected = initial * factor;

    let mut slots = HashMap::new();
    slots.insert("target".into(), target);
    slots.insert("initial".into(), initial.to_string());
    slots.insert("factor".into(), factor.to_string());
    slots.insert("expected".into(), expected.to_string());

    let reg = TemplateRegistry::global();
    if let Some(tmpl) = reg.get("multiplication") {
        tmpl.instantiate(prompt, &slots)
    } else {
        fallback_multiplication_reel(prompt, &slots)
    }
}

/// Synthesizes a division spatial proof sheet from `templates.base.dat`.
pub fn synthesize_division_reel(prompt: &str) -> String {
    let (target, divisor, initial) = parse_division_sentence(prompt);
    let expected = if divisor != 0 { initial / divisor } else { 0 };

    let mut slots = HashMap::new();
    slots.insert("target".into(), target);
    slots.insert("initial".into(), initial.to_string());
    slots.insert("divisor".into(), divisor.to_string());
    slots.insert("expected".into(), expected.to_string());

    let reg = TemplateRegistry::global();
    if let Some(tmpl) = reg.get("division") {
        tmpl.instantiate(prompt, &slots)
    } else {
        fallback_division_reel(prompt, &slots)
    }
}

/// Synthesizes a cognitive thought spatial proof sheet from cortical memory.
pub fn synthesize_cognitive_thought_reel(
    prompt: &str,
    thought: &ThoughtRecord,
) -> String {
    let (precond_val, commit_val, body_roll) = match thought.quad_state {
        QuadState::Proceed => (
            "+1", "PROCEED", "roll_thought [ state_vessel = +1 ]",
        ),
        QuadState::Await => (
            "0", "AWAIT", "roll_thought [ state_vessel = 0 ]",
        ),
        QuadState::Rollback => (
            "-1",
            "ROLLBACK",
            "roll_thought [ state_vessel = -1 ! [ Caveat = Veto ] ]",
        ),
        QuadState::Halt => (
            "0", "AWAIT", "roll_thought [ state_vessel = 0 ]",
        ),
    };

    let mut slots = HashMap::new();
    slots.insert("who".into(), sanitize_reel_str(&thought.who));
    slots.insert("what".into(), sanitize_reel_str(&thought.what));
    slots.insert("where".into(), sanitize_reel_str(&thought.where_clause));
    slots.insert("why".into(), sanitize_reel_str(&thought.why));
    slots.insert("how".into(), sanitize_reel_str(&thought.how));
    slots.insert(
        "domain".into(),
        sanitize_reel_str(thought.domain.as_str()),
    );
    if let Some(ref anchor) = thought.biblical_anchor {
        slots.insert("biblical_anchor".into(), sanitize_reel_str(anchor));
    }
    slots.insert("preconditions".into(), precond_val.into());
    slots.insert("commit_status".into(), commit_val.into());
    slots.insert("body_roll".into(), body_roll.into());

    let reg = TemplateRegistry::global();
    if let Some(tmpl) = reg.get("cognitive_thought") {
        tmpl.instantiate(prompt, &slots)
    } else {
        fallback_thought_reel(prompt, thought)
    }
}

/// Synthesizes a moral veto spatial proof sheet when conscience is engaged.
pub fn synthesize_moral_veto_reel(
    prompt: &str,
    eval: &MoralEvaluation,
) -> String {
    let law = eval.law_engaged.unwrap_or("Law_Truth");
    let mut slots = HashMap::new();
    slots.insert("veto_law".into(), law.into());

    let reg = TemplateRegistry::global();
    if let Some(tmpl) = reg.get("moral_veto") {
        tmpl.instantiate(prompt, &slots)
    } else {
        fallback_moral_veto_reel(prompt, law)
    }
}

/// Synthesizes a biblical wisdom spatial proof sheet grounded in ontology.
pub fn synthesize_biblical_wisdom_reel(
    prompt: &str,
    eval: &MoralEvaluation,
) -> String {
    let entry = eval.matched_entries.first().unwrap();
    let term_cap = capitalize(&entry.term);

    let mut slots = HashMap::new();
    slots.insert("term".into(), term_cap);

    let reg = TemplateRegistry::global();
    if let Some(tmpl) = reg.get("biblical_wisdom") {
        tmpl.instantiate(prompt, &slots)
    } else {
        fallback_wisdom_reel(prompt, &entry.term)
    }
}

/// Synthesizes an SVO discourse relational proof sheet.
pub fn synthesize_svo_discourse_reel(
    prompt: &str,
    clause: &SvoClause,
) -> String {
    let mut slots = HashMap::new();
    slots.insert("subject".into(), clause.subject.name.clone());
    slots.insert("verb".into(), clause.verb.clone());
    slots.insert("object".into(), clause.object.name.clone());
    let container_name = clause
        .scope_container
        .as_ref()
        .map(|c| c.name.clone())
        .unwrap_or_else(|| "Atelier".into());
    slots.insert("container".into(), container_name);

    let reg = TemplateRegistry::global();
    if let Some(tmpl) = reg.get("svo_discourse") {
        tmpl.instantiate(prompt, &slots)
    } else {
        fallback_svo_reel(prompt, &slots)
    }
}

/// Synthesizes a machine tool action spatial proof sheet.
pub fn synthesize_tool_action_reel(
    prompt: &str,
    tool_name: &str,
    tool_args: &str,
) -> String {
    let mut slots = HashMap::new();
    slots.insert("tool_name".into(), tool_name.into());
    slots.insert("tool_args".into(), tool_args.into());

    let reg = TemplateRegistry::global();
    if let Some(tmpl) = reg.get("tool_action") {
        tmpl.instantiate(prompt, &slots)
    } else {
        fallback_tool_reel(prompt, &slots)
    }
}

// -----------------------------------------------------------------------------
// Fallback Generators (In case of missing data sheets during bootstrap)
// -----------------------------------------------------------------------------

fn fallback_addition_reel(
    prompt: &str,
    slots: &HashMap<String, String>,
) -> String {
    let target = slots.get("target").unwrap();
    let initial = slots.get("initial").unwrap();
    let qty = slots.get("qty").unwrap();
    let unit = slots.get("unit").unwrap();
    let container = slots.get("container").unwrap();
    let expected = slots.get("expected").unwrap();
    let clean = sanitize_reel_str(prompt);

    format!(
r#"Module < NaturalReel.Addition > ( {clean} ) {{
    version: < "1.0.0" >
}}
Opening {{
    preconditions: [ +1 ]
}}
{unit} < Entity? >
{container} < Set? >
{target} [ {initial} : {unit}? ] {{ {container}? }}
new_{unit} [ {qty} : {unit}? ]
Body [
    roll_1 [ {target} --> {target} plus new_{unit} ]
    roll_2 [ {target} = {expected} ! [ new_{unit} = {qty} ] ]
]
Closing {{
    verify: [
        roll_1 = {expected} ::
        roll_2 = PROCEED ::
        {target} = {expected}
    ],
    commit_status [ PROCEED ]
}}
"#)
}

fn fallback_subtraction_reel(
    prompt: &str,
    slots: &HashMap<String, String>,
) -> String {
    let target = slots.get("target").unwrap();
    let initial = slots.get("initial").unwrap();
    let qty = slots.get("qty").unwrap();
    let expected = slots.get("expected").unwrap();
    let clean = sanitize_reel_str(prompt);

    format!(
r#"Module < NaturalReel.Subtraction > ( {clean} ) {{
    version: < "1.0.0" >
}}
Opening {{
    preconditions: [ +1 ]
}}
{target} [ {initial} ]
decrement [ {qty} ]
Body [
    roll_1 [ {target} --> {target} minus decrement ]
]
Closing {{
    verify: [ roll_1 = {expected} :: {target} = {expected} ],
    commit_status [ PROCEED ]
}}
"#)
}

fn fallback_multiplication_reel(
    prompt: &str,
    slots: &HashMap<String, String>,
) -> String {
    let target = slots.get("target").unwrap();
    let initial = slots.get("initial").unwrap();
    let factor = slots.get("factor").unwrap();
    let expected = slots.get("expected").unwrap();
    let clean = sanitize_reel_str(prompt);

    format!(
r#"Module < NaturalReel.Multiplication > ( {clean} ) {{
    version: < "1.0.0" >
}}
Opening {{
    preconditions: [ +1 ]
}}
{target} [ {initial} ]
factor [ {factor} ]
Body [
    roll_1 [ {target} --> {target} times factor ]
]
Closing {{
    verify: [ roll_1 = {expected} :: {target} = {expected} ],
    commit_status [ PROCEED ]
}}
"#)
}

fn fallback_division_reel(
    prompt: &str,
    slots: &HashMap<String, String>,
) -> String {
    let target = slots.get("target").unwrap();
    let initial = slots.get("initial").unwrap();
    let divisor = slots.get("divisor").unwrap();
    let expected = slots.get("expected").unwrap();
    let clean = sanitize_reel_str(prompt);

    format!(
r#"Module < NaturalReel.Division > ( {clean} ) {{
    version: < "1.0.0" >
}}
Opening {{
    preconditions: [ +1 ]
}}
{target} [ {initial} ]
divisor [ {divisor} ]
Body [
    roll_1 [ {target} --> {target} divide divisor ]
]
Closing {{
    verify: [ roll_1 = {expected} :: {target} = {expected} ],
    commit_status [ PROCEED ]
}}
"#)
}

fn fallback_thought_reel(prompt: &str, thought: &ThoughtRecord) -> String {
    let clean = sanitize_reel_str(prompt);
    format!(
r#"Module < Cognitive.{}.Thought > ( {clean} ) {{
    version: < "1.0.0" >
}}
Opening {{
    preconditions: [ +1 ]
}}
state_vessel [ +1 : State? ]
Body [
    roll_thought [ state_vessel = +1 ]
]
Closing {{
    verify: [ roll_thought = PROCEED ],
    commit_status [ PROCEED ]
}}
"#, sanitize_reel_str(&thought.who))
}

fn fallback_moral_veto_reel(prompt: &str, law: &str) -> String {
    let clean = sanitize_reel_str(prompt);
    format!(
r#"Module < Covenant.MoralVeto > ( {clean} ) {{
    version: < "1.0.0" >
}}
Opening {{
    requires: [ {law} : Invariant ],
    preconditions: [ -1 ]
}}
moral_guard [ -1 : State? ]
Body [
    roll_veto [ moral_guard = -1 ! [ Caveat = Veto ] ]
]
Closing {{
    verify: [ roll_veto = ROLLBACK ],
    commit_status [ ROLLBACK ]
}}
"#)
}

fn fallback_wisdom_reel(prompt: &str, term: &str) -> String {
    let clean = sanitize_reel_str(prompt);
    format!(
r#"Module < Covenant.Wisdom.{} > ( {clean} ) {{
    version: < "1.0.0" >
}}
Opening {{
    preconditions: [ +1 ]
}}
concept_active [ +1 : State? ]
Body [
    roll_wisdom [ concept_active = +1 ]
]
Closing {{
    verify: [ roll_wisdom = PROCEED ],
    commit_status [ PROCEED ]
}}
"#, capitalize(term))
}

fn fallback_svo_reel(prompt: &str, slots: &HashMap<String, String>) -> String {
    let clean = sanitize_reel_str(prompt);
    let s = slots.get("subject").unwrap();
    let v = slots.get("verb").unwrap();
    let o = slots.get("object").unwrap();
    let c = slots.get("container").unwrap();
    format!(
r#"Module < Discourse.SvoRelational > ( {clean} ) {{
    version: < "1.0.0" >
}}
Opening {{
    preconditions: [ +1 ]
}}
{s} < Agent? >
{o} [ 1 : Vessel? ]
Body [
    roll_svo [ {s} --> {v} {o} into {c} ]
]
Closing {{
    verify: [ roll_svo = PROCEED ],
    commit_status [ PROCEED ]
}}
"#)
}

fn fallback_tool_reel(prompt: &str, slots: &HashMap<String, String>) -> String {
    let clean = sanitize_reel_str(prompt);
    let tool_name = slots.get("tool_name").unwrap();
    let tool_args = slots.get("tool_args").unwrap();
    format!(
r#"Module < Machine.ToolExecution > ( {clean} ) {{
    version: < "1.0.0" >
}}
Opening {{
    preconditions: [ +1 ]
}}
tool_target [ {tool_name} ]
Body [
    roll_tool [ tool_target --> execute {tool_args} ]
]
Closing {{
    verify: [ roll_tool = PROCEED ],
    commit_status [ PROCEED ]
}}
"#)
}

// =============================================================================
// BLOCK 4: AUDIT PROCEDURES BLOCK
// ( Self-Diagnostic Verification for Dynamic Reel Synthesis )
// =============================================================================

/// Executes automated self-diagnostic audit for the template subsystem.
pub fn audit_templates_subsystem() -> Result<(), String> {
    let reg = TemplateRegistry::new();

    // 1. Verify canonical archetypes exist in registry
    let required = [
        "addition", "subtraction", "multiplication", "division",
        "cognitive_thought", "moral_veto", "biblical_wisdom",
        "svo_discourse", "tool_action",
    ];

    for id in required {
        if !reg.templates.contains_key(id) {
            return Err(format!("Audit failed: Missing base template '{id}'"));
        }
    }

    // 2. Synthesize test addition reel and verify execution through VM
    let test_prompt = "There are 3 sheaves in the barn and 4 more are gathered";
    let reel = synthesize_addition_reel(test_prompt);

    let logger = crate::diagnostics::logger::DiagnosticLogger::new(false);
    let output = crate::ai::transducer::executor::execute_reel(&reel, logger)
        .map_err(|e| format!("Audit failed to execute synthesized reel: {e}"))?;

    if output.execution_state != QuadState::Proceed {
        return Err(format!(
            "Audit failed: Reel state was {:?}, expected Proceed",
            output.execution_state
        ));
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
    fn test_template_subsystem_audit() {
        assert!(audit_templates_subsystem().is_ok());
    }

    #[test]
    fn test_addition_reel_structure() {
        let prompt = "There are 5 loaves in the basket and 7 more are added";
        let reel = synthesize_addition_reel(prompt);

        assert!(reel.contains("Module < NaturalReel.Addition >"));
        assert!(reel.contains("Opening {"));
        assert!(reel.contains("Body ["));
        assert!(reel.contains("Closing {"));
        assert!(reel.contains("loaves"));
        assert!(reel.contains("basket"));
    }

    #[test]
    fn test_subtraction_and_multiplication_reels() {
        let sub_prompt = "Remove 3 items from box A has 10";
        let sub_reel = synthesize_subtraction_reel(sub_prompt);
        assert!(sub_reel.contains("NaturalReel.Subtraction"));
        assert!(sub_reel.contains("decrement [ 3 ]"));

        let mul_prompt = "The value of 4 is multiplied by 5";
        let mul_reel = synthesize_multiplication_reel(mul_prompt);
        assert!(mul_reel.contains("NaturalReel.Multiplication"));
        assert!(mul_reel.contains("factor [ 5 ]"));
    }

    #[test]
    fn test_svo_discourse_proof_sheet() {
        let prompt = "The Sculptor constructs the Apparatus into the Atelier";
        let clauses = crate::ai::transducer::svo::SvoComposer::
            decompose_prompt_with_lexicon(prompt, None);
        assert!(!clauses.is_empty());
        let clause = &clauses[0];

        let reel = synthesize_svo_discourse_reel(prompt, clause);
        assert!(reel.contains("Discourse.SvoRelational"));
        assert!(reel.contains("Sculptor"));
        assert!(reel.contains("Apparatus"));
        assert!(reel.contains("into Atelier"));
    }

    #[test]
    fn test_tool_action_proof_sheet() {
        let prompt = "Run linter on source code";
        let reel =
            synthesize_tool_action_reel(prompt, "LinterTool", "source_code");
        assert!(reel.contains("Machine.ToolExecution"));
        assert!(reel.contains("tool_target [ LinterTool ]"));
        assert!(reel.contains("execute source_code"));
    }

    #[test]
    fn test_dynamic_template_registration_in_memory() {
        let mut reg = TemplateRegistry::new();
        let custom = ProofSheetTemplate {
            id: "quantum_entanglement".into(),
            domain: "Physics".into(),
            module_pattern: "Physics.Entangle".into(),
            requires: "Quantum : Superposition".into(),
            preconditions: "+1".into(),
            vessels: vec!["qubit_a [ 0 ]".into(), "qubit_b [ 0 ]".into()],
            rolls: vec!["roll_bell [ qubit_a <-> qubit_b ]".into()],
            verify: vec!["roll_bell = PROCEED".into()],
            commit_status: "PROCEED".into(),
        };

        reg.templates.insert(custom.id.clone(), custom.clone());
        assert!(reg.get("quantum_entanglement").is_some());

        let slots = HashMap::new();
        let reel = reg
            .get("quantum_entanglement")
            .unwrap()
            .instantiate("Entangle pair", &slots);
        assert!(reel.contains("Physics.Entangle"));
        assert!(reel.contains("qubit_a <-> qubit_b"));
    }
}

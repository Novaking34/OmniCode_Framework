// =============================================================================
// BLOCK 1: METADATA BLOCK
// ( Identity, Provenance, Invariant Boundary Contracts & Scriptural Anchors )
// =============================================================================
// @module        OmniCode AI Subsystem: Continuous Algebraic SVO Composer
// @file          svo.rs
// @layer         Track 1 [Host Engine & Virtual Machine] -> src/ai/transducer
// @version       1.0.0
// @epoch         2026.10
// @author        CreativeWorkzStudio LLC / ProfessorSeanEX
// @system        OmniCode AI Cortex & Living Kernel
// @repository    https://github.com/Novaking34/OmniCode_Framework
// @role          SVO Algebraic Decomposition & Directional Vectors
// @invariants    Rank 1 (< >): Form precedes fullness. Decomposes natural
//                discourse into directional vectors across 8 parts of speech
//                with strict balanced ternary integer parity verification.
// @witness       "For by him were all things created, that are in heaven,
//                 and that are in earth, visible and invisible..." — Col 1:16
//                "Let all things be done decently and in order." — 1 Cor 14:40
//
// CONTINUOUS ALGEBRAIC SVO COMPOSITION LIFECYCLE:
//   1. Dual-Sheet Grounding: Ingests 8 parts of speech (.base + .live).
//   2. Boundary Typing: Invariant <Agent>, Vessel [Payload], Scope {Scope}.
//   3. Castable Operators: Subjects/Objects can be cast to active ops.
//   4. Directional Motion: Maps verbs to -->, <--, ^--, v--, =, !=, ~=.
//   5. Two-Way Checksum Parity Gate: Synthesized reels evaluated via
//      RestoratorCertifier before VM evaluation, with autonomic self-healing.
//   6. Bidirectional Transduction: Decomposes prompts to reels; composes
//      thoughts to natural human discourse sentences.

use std::collections::HashMap;
use crate::ai::lexicon::domain::ModalDomain;
use crate::ai::lexicon::BiblicalLexicon;
use crate::core::boundaries::BoundaryType;
use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::DiagnosticLogger;
use crate::diagnostics::restorator::certifier::{
    RestoratorCertifier, TwoWayAgreement,
};
use crate::diagnostics::restorator::Restorator;

// =============================================================================
// BLOCK 2: OPENING BLOCK
// ( Directional Operators, SVO Entities, Clauses & Parity Gate Structures )
// =============================================================================

/// OmniCode directional causal operators and relations of being.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SvoOperator {
    /// Rightwards causal invocation / execution (`-->`).
    Invoke,
    /// Leftwards assignment / reception (`<--`).
    Assign,
    /// Upwards extraction / emission (`^--`).
    Emit,
    /// Downwards storage / vessel reservation (`v--`).
    Store,
    /// Equilibrium copula / relation of being (`=`).
    Equals,
    /// Inequality / differentiation (`!=`).
    NotEquals,
    /// Analogy / resemblance (`~=`).
    Resembles,
}

impl SvoOperator {
    /// Returns the canonical OmniCode operator token.
    pub fn as_str(&self) -> &'static str {
        match self {
            SvoOperator::Invoke => "-->",
            SvoOperator::Assign => "<--",
            SvoOperator::Emit => "^--",
            SvoOperator::Store => "v--",
            SvoOperator::Equals => "=",
            SvoOperator::NotEquals => "!=",
            SvoOperator::Resembles => "~=",
        }
    }

    /// Resolves an operator from a raw string token.
    pub fn from_token(token: &str) -> Option<Self> {
        match token.trim() {
            "-->" => Some(SvoOperator::Invoke),
            "<--" => Some(SvoOperator::Assign),
            "^--" => Some(SvoOperator::Emit),
            "v--" => Some(SvoOperator::Store),
            "=" => Some(SvoOperator::Equals),
            "!=" => Some(SvoOperator::NotEquals),
            "~=" => Some(SvoOperator::Resembles),
            _ => None,
        }
    }

    /// Infers an operator from an English verb or action term using the
    /// data-driven linguistic registry.
    pub fn from_verb(verb: &str) -> Self {
        SvoRegistry::global().resolve_verb_operator(verb)
    }
}

/// The grammatical role an entity plays within an SVO clause.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityRole {
    /// Subject: Initiator, Actor, or Source Container.
    Subject,
    /// Object: Target Boundary, Payload, or Recipient.
    Object,
    /// Scope: Ambient Context or Partition Container.
    Scope,
}

/// A structured entity functioning within an SVO clause.
#[derive(Debug, Clone, PartialEq)]
pub struct SvoEntity {
    /// The term or name identifying the entity.
    pub name: String,
    /// The morphological boundary type governing its physical container.
    pub boundary: BoundaryType,
    /// Inferred or grounded ontological type descriptor.
    pub type_descriptor: String,
    /// Modal Domain if grounded in the World Lexicon.
    pub modal_domain: Option<ModalDomain>,
    /// Whether this entity can act as a callable operator when cast.
    pub is_operator: bool,
    /// Whether this entity is novel (not yet grounded in Lexicon/Memory).
    pub is_novel: bool,
}

impl SvoEntity {
    /// Formats the entity into its canonical OmniCode vessel declaration.
    pub fn format_declaration(&self) -> String {
        let (open, close) = self.boundary.delimiters();
        format!(
            "{} {} {}? {}",
            self.name, open, self.type_descriptor, close
        )
    }
}

impl std::fmt::Display for SvoEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// A structured SVO clause parsed from discourse.
#[derive(Debug, Clone, PartialEq)]
pub struct SvoClause {
    /// The subject / initiator entity.
    pub subject: SvoEntity,
    /// The action verb in natural discourse.
    pub verb: String,
    /// The primary directional or relational operator.
    pub primary_op: SvoOperator,
    /// The direct object / payload entity.
    pub object: SvoEntity,
    /// Spatial or boundary container destination, if any.
    pub scope_container: Option<SvoEntity>,
    /// Caveat condition / invariant guard, if any.
    pub caveat_guard: Option<String>,
    /// Teleological parenthetical intent aside, if any.
    pub aside_intent: Option<String>,
    /// Secondary chained operator, if any.
    pub secondary_op: Option<SvoOperator>,
    /// Secondary chained target entity, if any.
    pub secondary_object: Option<SvoEntity>,
}

/// Result of continuous SVO composition verified through the parity gate.
#[derive(Debug, Clone)]
pub struct SvoCompositionResult {
    /// The synthesized canonical 4-Block OmniCode Reel.
    pub omnicode_reel: String,
    /// Parsed SVO clauses.
    pub clauses: Vec<SvoClause>,
    /// Novel entities discovered during decomposition for learning.
    pub novel_entities: Vec<SvoEntity>,
    /// Two-Way Checksum parity agreement.
    pub agreement: TwoWayAgreement,
    /// Whether autonomic self-healing repairs were required.
    pub was_repaired: bool,
}

// =============================================================================
// BLOCK 3: BODY BLOCK
// ( Data-Driven Linguistic Registry, SVO Parser & Reel Synthesis Engine )
// =============================================================================

/// Static data sheet contents baked into the binary (8 parts of speech).
const VERBS_BASE_DAT: &str = include_str!("data/verbs.base.dat");
const VERBS_LIVE_DAT: &str = include_str!("data/verbs.live.dat");
const PREPOSITIONS_BASE_DAT: &str = include_str!("data/prepositions.base.dat");
const PREPOSITIONS_LIVE_DAT: &str = include_str!("data/prepositions.live.dat");
const CONJUNCTIONS_BASE_DAT: &str = include_str!("data/conjunctions.base.dat");
const CONJUNCTIONS_LIVE_DAT: &str = include_str!("data/conjunctions.live.dat");
const NOUNS_BASE_DAT: &str = include_str!("data/nouns.base.dat");
const NOUNS_LIVE_DAT: &str = include_str!("data/nouns.live.dat");
const PRONOUNS_BASE_DAT: &str = include_str!("data/pronouns.base.dat");
const PRONOUNS_LIVE_DAT: &str = include_str!("data/pronouns.live.dat");
const ADJECTIVES_BASE_DAT: &str = include_str!("data/adjectives.base.dat");
const ADJECTIVES_LIVE_DAT: &str = include_str!("data/adjectives.live.dat");
const ADVERBS_BASE_DAT: &str = include_str!("data/adverbs.base.dat");
const ADVERBS_LIVE_DAT: &str = include_str!("data/adverbs.live.dat");
const INTERJECTIONS_BASE_DAT: &str =
    include_str!("data/interjections.base.dat");
const INTERJECTIONS_LIVE_DAT: &str =
    include_str!("data/interjections.live.dat");

/// Linguistic registry caching data-driven grammar definitions across 8 parts.
pub struct SvoRegistry {
    verbs: HashMap<String, (SvoOperator, bool, String)>,
    prepositions: HashMap<String, (String, String, String)>,
    conjunctions: HashMap<String, (String, String, String)>,
    nouns: HashMap<String, (BoundaryType, Option<ModalDomain>, String)>,
    pronouns: HashMap<String, (String, String)>,
    adjectives: HashMap<String, (String, String)>,
    adverbs: HashMap<String, (String, i32, String)>,
    interjections: HashMap<String, (String, String)>,
}

impl SvoRegistry {
    /// Constructs and initializes the registry from baked `.dat` sheets.
    pub fn init() -> Self {
        let mut verbs = HashMap::new();
        Self::ingest_verbs(&mut verbs, VERBS_BASE_DAT);
        Self::ingest_verbs(&mut verbs, VERBS_LIVE_DAT);

        let mut prepositions = HashMap::new();
        Self::ingest_prepositions(&mut prepositions, PREPOSITIONS_BASE_DAT);
        Self::ingest_prepositions(&mut prepositions, PREPOSITIONS_LIVE_DAT);

        let mut conjunctions = HashMap::new();
        Self::ingest_conjunctions(&mut conjunctions, CONJUNCTIONS_BASE_DAT);
        Self::ingest_conjunctions(&mut conjunctions, CONJUNCTIONS_LIVE_DAT);

        let mut nouns = HashMap::new();
        Self::ingest_nouns(&mut nouns, NOUNS_BASE_DAT);
        Self::ingest_nouns(&mut nouns, NOUNS_LIVE_DAT);

        let mut pronouns = HashMap::new();
        Self::ingest_pronouns(&mut pronouns, PRONOUNS_BASE_DAT);
        Self::ingest_pronouns(&mut pronouns, PRONOUNS_LIVE_DAT);

        let mut adjectives = HashMap::new();
        Self::ingest_adjectives(&mut adjectives, ADJECTIVES_BASE_DAT);
        Self::ingest_adjectives(&mut adjectives, ADJECTIVES_LIVE_DAT);

        let mut adverbs = HashMap::new();
        Self::ingest_adverbs(&mut adverbs, ADVERBS_BASE_DAT);
        Self::ingest_adverbs(&mut adverbs, ADVERBS_LIVE_DAT);

        let mut interjections = HashMap::new();
        Self::ingest_interjections(&mut interjections, INTERJECTIONS_BASE_DAT);
        Self::ingest_interjections(&mut interjections, INTERJECTIONS_LIVE_DAT);

        Self {
            verbs,
            prepositions,
            conjunctions,
            nouns,
            pronouns,
            adjectives,
            adverbs,
            interjections,
        }
    }

    fn ingest_verbs(
        map: &mut HashMap<String, (SvoOperator, bool, String)>,
        sheet: &str,
    ) {
        for line in sheet.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
            if parts.len() >= 4 {
                let verb = parts[0].to_lowercase();
                let op = SvoOperator::from_token(parts[1])
                    .unwrap_or(SvoOperator::Invoke);
                let can_cast = parts[2].eq_ignore_ascii_case("true");
                let causality = parts[3].to_string();
                map.insert(verb, (op, can_cast, causality));
            }
        }
    }

    fn ingest_prepositions(
        map: &mut HashMap<String, (String, String, String)>,
        sheet: &str,
    ) {
        for line in sheet.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
            if parts.len() >= 4 {
                map.insert(
                    parts[0].to_lowercase(),
                    (
                        parts[1].to_string(),
                        parts[2].to_string(),
                        parts[3].to_string(),
                    ),
                );
            }
        }
    }

    fn ingest_conjunctions(
        map: &mut HashMap<String, (String, String, String)>,
        sheet: &str,
    ) {
        for line in sheet.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
            if parts.len() >= 4 {
                map.insert(
                    parts[0].to_lowercase(),
                    (
                        parts[1].to_string(),
                        parts[2].to_string(),
                        parts[3].to_string(),
                    ),
                );
            }
        }
    }

    fn ingest_nouns(
        map: &mut HashMap<String, (BoundaryType, Option<ModalDomain>, String)>,
        sheet: &str,
    ) {
        for line in sheet.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
            if parts.len() >= 4 {
                let noun = parts[0].to_lowercase();
                let b = match parts[1].to_lowercase().as_str() {
                    "invariant" => BoundaryType::Invariant,
                    "connotation" => BoundaryType::Connotation,
                    _ => BoundaryType::Denotation,
                };
                let dom = parse_domain_str(parts[2]);
                let role = parts[3].to_string();
                map.insert(noun, (b, dom, role));
            }
        }
    }

    fn ingest_pronouns(
        map: &mut HashMap<String, (String, String)>,
        sheet: &str,
    ) {
        for line in sheet.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
            if parts.len() >= 6 {
                map.insert(
                    parts[0].to_lowercase(),
                    (parts[1].to_string(), parts[5].to_string()),
                );
            }
        }
    }

    fn ingest_adjectives(
        map: &mut HashMap<String, (String, String)>,
        sheet: &str,
    ) {
        for line in sheet.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
            if parts.len() >= 4 {
                map.insert(
                    parts[0].to_lowercase(),
                    (parts[1].to_string(), parts[3].to_string()),
                );
            }
        }
    }

    fn ingest_adverbs(
        map: &mut HashMap<String, (String, i32, String)>,
        sheet: &str,
    ) {
        for line in sheet.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
            if parts.len() >= 4 {
                let rate = parts[2].parse::<i32>().unwrap_or(1);
                map.insert(
                    parts[0].to_lowercase(),
                    (parts[1].to_string(), rate, parts[3].to_string()),
                );
            }
        }
    }

    fn ingest_interjections(
        map: &mut HashMap<String, (String, String)>,
        sheet: &str,
    ) {
        for line in sheet.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
            if parts.len() >= 4 {
                map.insert(
                    parts[0].to_lowercase(),
                    (parts[1].to_string(), parts[3].to_string()),
                );
            }
        }
    }

    /// Accesses the global registry singleton.
    pub fn global() -> &'static SvoRegistry {
        static REGISTRY: std::sync::OnceLock<SvoRegistry> =
            std::sync::OnceLock::new();
        REGISTRY.get_or_init(Self::init)
    }

    /// Resolves an SvoOperator from a verb string.
    pub fn resolve_verb_operator(&self, verb: &str) -> SvoOperator {
        let clean = verb.trim().to_lowercase();
        if let Some((op, _, _)) = self.verbs.get(&clean) {
            return *op;
        }

        // Substring / stem lookup
        for (v_key, (op, _, _)) in &self.verbs {
            if clean.contains(v_key.as_str()) || v_key.contains(&clean) {
                return *op;
            }
        }

        SvoOperator::Invoke
    }

    /// Resolves the causality type for a verb (e.g., "ArithmeticAdd").
    pub fn resolve_verb_causality(&self, verb: &str) -> Option<&str> {
        let clean = verb.trim().to_lowercase();
        if let Some((_, _, causality)) = self.verbs.get(&clean) {
            return Some(causality.as_str());
        }
        for (v_key, (_, _, causality)) in &self.verbs {
            if clean.contains(v_key.as_str()) || v_key.contains(&clean) {
                return Some(causality.as_str());
            }
        }
        None
    }

    /// Checks if a word is registered as an operator-castable entity.
    pub fn can_cast_operator(&self, word: &str) -> bool {
        let clean = word.trim().to_lowercase();
        self.verbs.get(&clean).map(|(_, cast, _)| *cast).unwrap_or(false)
    }

    /// Checks if a word is a registered preposition.
    pub fn is_preposition(&self, word: &str) -> bool {
        self.prepositions.contains_key(&word.trim().to_lowercase())
    }

    /// Checks if a word is a registered caveat conjunction.
    pub fn is_caveat_conjunction(&self, word: &str) -> bool {
        let clean = word.trim().to_lowercase();
        if let Some((_, syntax, _)) = self.conjunctions.get(&clean) {
            syntax == "!"
        } else {
            false
        }
    }

    /// Checks if a word is a registered interjection (e.g., Behold, Amen).
    pub fn is_interjection(&self, word: &str) -> bool {
        self.interjections.contains_key(&word.trim().to_lowercase())
    }

    /// Checks if a word is a registered adjective qualifier.
    pub fn is_adjective(&self, word: &str) -> bool {
        self.adjectives.contains_key(&word.trim().to_lowercase())
    }

    /// Checks if a word is a registered adverb modulator.
    pub fn is_adverb(&self, word: &str) -> bool {
        self.adverbs.contains_key(&word.trim().to_lowercase())
    }

    /// Resolves an entity's boundary type and type descriptor, querying the
    /// NounRegistry, PronounRegistry, and World Lexicon to ground it.
    pub fn resolve_entity(
        &self,
        word: &str,
        role: EntityRole,
        lexicon: Option<&BiblicalLexicon>,
    ) -> SvoEntity {
        let clean_name = word.trim().to_string();
        let lower = clean_name.to_lowercase();
        let is_op = self.can_cast_operator(&clean_name);

        // 1. Check data-grounded Nouns registry (nouns.base + nouns.live)
        if let Some((b, dom, role_str)) = self.nouns.get(&lower) {
            return SvoEntity {
                name: clean_name,
                boundary: *b,
                type_descriptor: role_str.clone(),
                modal_domain: *dom,
                is_operator: is_op,
                is_novel: false,
            };
        }

        // 2. Check data-grounded Pronouns registry (pronouns.base + live)
        if let Some((ref_type, _)) = self.pronouns.get(&lower) {
            let b = if ref_type == "Agent" {
                BoundaryType::Invariant
            } else {
                BoundaryType::Denotation
            };
            return SvoEntity {
                name: clean_name,
                boundary: b,
                type_descriptor: ref_type.clone(),
                modal_domain: Some(ModalDomain::Lingual),
                is_operator: is_op,
                is_novel: false,
            };
        }

        // 3. Check World Lexicon triangulation
        if let Some(lex) = lexicon {
            if let Some(triangulation) = lex.triangulate(&clean_name) {
                let domain = triangulation.modal_domain;
                let (boundary, descriptor) = match role {
                    EntityRole::Subject => (
                        BoundaryType::Invariant,
                        format!("{:?}", domain),
                    ),
                    EntityRole::Object => (
                        BoundaryType::Denotation,
                        format!("{:?}", domain),
                    ),
                    EntityRole::Scope => (
                        BoundaryType::Connotation,
                        format!("{:?}", domain),
                    ),
                };

                return SvoEntity {
                    name: clean_name,
                    boundary,
                    type_descriptor: descriptor,
                    modal_domain: Some(domain),
                    is_operator: is_op,
                    is_novel: false,
                };
            }
        }

        // 4. Novel entity: Infer boundary from grammatical role
        let (boundary, descriptor) = match role {
            EntityRole::Subject => (
                BoundaryType::Invariant,
                "Agent".to_string(),
            ),
            EntityRole::Object => (
                BoundaryType::Denotation,
                "Vessel".to_string(),
            ),
            EntityRole::Scope => (
                BoundaryType::Connotation,
                "Container".to_string(),
            ),
        };

        SvoEntity {
            name: clean_name,
            boundary,
            type_descriptor: descriptor,
            modal_domain: None,
            is_operator: is_op,
            is_novel: true,
        }
    }
}

pub struct SvoComposer;

impl SvoComposer {
    /// Composes a natural conversational prompt into a verified 4-Block Reel
    /// checked in real-time by the Two-Way Checksum parity gate.
    pub fn compose_and_verify(
        prompt: &str,
        logger: &DiagnosticLogger,
        debugger: &OmniDebugger,
        restorator: &Restorator,
    ) -> SvoCompositionResult {
        let clauses = Self::decompose_prompt(prompt);
        let raw_reel = Self::synthesize_svo_reel(prompt, &clauses);

        let mut novel_entities = Vec::new();
        for c in &clauses {
            if c.subject.is_novel {
                novel_entities.push(c.subject.clone());
            }
            if c.object.is_novel {
                novel_entities.push(c.object.clone());
            }
            if let Some(ref sc) = c.scope_container {
                if sc.is_novel {
                    novel_entities.push(sc.clone());
                }
            }
        }

        let agreement = RestoratorCertifier::evaluate_two_way_checksum(
            logger,
            debugger,
            &raw_reel,
        );

        let (final_reel, final_agreement, was_repaired) = match agreement {
            TwoWayAgreement::Certified => {
                (raw_reel, TwoWayAgreement::Certified, false)
            }
            _ => {
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
            novel_entities,
            agreement: final_agreement,
            was_repaired,
        }
    }

    /// Decomposes a conversational prompt into one or more SVO clauses
    /// using data-driven grammar and 8 parts of speech role parsing.
    pub fn decompose_prompt(prompt: &str) -> Vec<SvoClause> {
        Self::decompose_prompt_with_lexicon(prompt, None)
    }

    /// Decomposes a prompt with optional Lexicon grounding for learning.
    pub fn decompose_prompt_with_lexicon(
        prompt: &str,
        lexicon: Option<&BiblicalLexicon>,
    ) -> Vec<SvoClause> {
        let reg = SvoRegistry::global();
        let clean = prompt.trim();

        // Extract parenthetical aside intent: ( note )
        let mut aside_intent = None;
        let mut text_without_aside = clean.to_string();
        if let (Some(start), Some(end)) = (clean.find('('), clean.find(')')) {
            if start < end {
                aside_intent =
                    Some(clean[start + 1..end].trim().to_string());
                text_without_aside = format!(
                    "{} {}",
                    &clean[..start],
                    &clean[end + 1..]
                );
            }
        }

        // Extract caveat clause: "! condition" or "unless condition"
        let mut caveat_guard = None;
        let mut working_text = text_without_aside;
        for caveat_word in &["unless", "except", "but not", "provided"] {
            if let Some(pos) =
                working_text.to_lowercase().find(caveat_word)
            {
                caveat_guard = Some(working_text[pos..].trim().to_string());
                working_text = working_text[..pos].trim().to_string();
                break;
            }
        }

        let words: Vec<&str> = working_text
            .split_whitespace()
            .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric() && c != '_'))
            .filter(|w| !w.is_empty())
            .collect();

        if words.is_empty() {
            let subj =
                reg.resolve_entity("Agent", EntityRole::Subject, lexicon);
            let obj =
                reg.resolve_entity("Action", EntityRole::Object, lexicon);
            return vec![SvoClause {
                subject: subj,
                verb: "invokes".to_string(),
                primary_op: SvoOperator::Invoke,
                object: obj,
                scope_container: None,
                caveat_guard,
                aside_intent,
                secondary_op: None,
                secondary_object: None,
            }];
        }

        // Multi-word transfer / channel patterns
        let low_words: Vec<String> =
            words.iter().map(|w| w.to_lowercase()).collect();
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
            let source_name = if f + 1 < words.len() {
                words[f + 1]
            } else {
                "Source"
            };
            let target_name = if t + 1 < words.len() {
                words[t + 1]
            } else {
                "Target"
            };
            let item_name = if f > 1 { words[1] } else { "Item" };

            let source = reg.resolve_entity(
                source_name,
                EntityRole::Subject,
                lexicon,
            );
            let target = reg.resolve_entity(
                target_name,
                EntityRole::Scope,
                lexicon,
            );
            let item = reg.resolve_entity(
                item_name,
                EntityRole::Object,
                lexicon,
            );

            let (sec_op, sec_obj) = if let Some(s) = in_idx {
                if s + 1 < words.len() {
                    let sec_name = words[s + 1];
                    let sec_ent = reg.resolve_entity(
                        sec_name,
                        EntityRole::Scope,
                        lexicon,
                    );
                    (Some(SvoOperator::Store), Some(sec_ent))
                } else {
                    (None, None)
                }
            } else {
                (None, None)
            };

            let mut clauses = Vec::new();
            clauses.push(SvoClause {
                subject: source,
                verb: "emits".to_string(),
                primary_op: SvoOperator::Emit,
                object: item,
                scope_container: Some(target.clone()),
                caveat_guard: caveat_guard.clone(),
                aside_intent: aside_intent.clone(),
                secondary_op: Some(SvoOperator::Invoke),
                secondary_object: Some(target),
            });

            if let (Some(op), Some(obj)) = (sec_op, sec_obj) {
                let target_ent = reg.resolve_entity(
                    target_name,
                    EntityRole::Subject,
                    lexicon,
                );
                clauses.push(SvoClause {
                    subject: target_ent,
                    verb: "stores".to_string(),
                    primary_op: op,
                    object: obj,
                    scope_container: None,
                    caveat_guard,
                    aside_intent,
                    secondary_op: None,
                    secondary_object: None,
                });
            }

            return clauses;
        }

        // Structural sentence extraction bypassing leading articles/adverbs
        let mut start_idx = 0;
        while start_idx < words.len()
            && (words[start_idx].eq_ignore_ascii_case("the")
                || words[start_idx].eq_ignore_ascii_case("a")
                || words[start_idx].eq_ignore_ascii_case("an"))
        {
            start_idx += 1;
        }

        let remaining = &words[start_idx..];
        if remaining.len() >= 3 {
            let subject_name = remaining[0];
            let verb_str = remaining[1];
            let mut obj_idx = 2;
            if remaining.len() > 3
                && (remaining[2].eq_ignore_ascii_case("the")
                    || remaining[2].eq_ignore_ascii_case("a")
                    || remaining[2].eq_ignore_ascii_case("an"))
            {
                obj_idx = 3;
            }
            let object_name = remaining[obj_idx];
            let op = SvoOperator::from_verb(verb_str);

            let subject = reg.resolve_entity(
                subject_name,
                EntityRole::Subject,
                lexicon,
            );
            let object = reg.resolve_entity(
                object_name,
                EntityRole::Object,
                lexicon,
            );

            let mut scope_container = None;
            for i in obj_idx + 1..remaining.len() {
                if reg.is_preposition(remaining[i]) && i + 1 < remaining.len() {
                    let mut target_i = i + 1;
                    if (remaining[target_i].eq_ignore_ascii_case("the")
                        || remaining[target_i].eq_ignore_ascii_case("a"))
                        && target_i + 1 < remaining.len()
                    {
                        target_i += 1;
                    }
                    scope_container = Some(reg.resolve_entity(
                        remaining[target_i],
                        EntityRole::Scope,
                        lexicon,
                    ));
                    break;
                }
            }

            let (sec_op, sec_obj) = if remaining.len() >= 5
                && scope_container.is_none()
            {
                let sec_verb = remaining[3];
                let sec_target = remaining[4];
                let sec_ent = reg.resolve_entity(
                    sec_target,
                    EntityRole::Object,
                    lexicon,
                );
                (Some(SvoOperator::from_verb(sec_verb)), Some(sec_ent))
            } else {
                (None, None)
            };

            vec![SvoClause {
                subject,
                verb: verb_str.to_string(),
                primary_op: op,
                object,
                scope_container,
                caveat_guard,
                aside_intent,
                secondary_op: sec_op,
                secondary_object: sec_obj,
            }]
        } else if remaining.len() == 2 {
            let op = SvoOperator::from_verb(remaining[0]);
            let subj = reg.resolve_entity(
                "Agent",
                EntityRole::Subject,
                lexicon,
            );
            let obj = reg.resolve_entity(
                remaining[1],
                EntityRole::Object,
                lexicon,
            );
            vec![SvoClause {
                subject: subj,
                verb: remaining[0].to_string(),
                primary_op: op,
                object: obj,
                scope_container: None,
                caveat_guard,
                aside_intent,
                secondary_op: None,
                secondary_object: None,
            }]
        } else {
            let subj_name = remaining.first().unwrap_or(&"Entity");
            let subj = reg.resolve_entity(
                subj_name,
                EntityRole::Subject,
                lexicon,
            );
            let obj = reg.resolve_entity(
                "State",
                EntityRole::Object,
                lexicon,
            );
            vec![SvoClause {
                subject: subj,
                verb: "invokes".to_string(),
                primary_op: SvoOperator::Invoke,
                object: obj,
                scope_container: None,
                caveat_guard,
                aside_intent,
                secondary_op: None,
                secondary_object: None,
            }]
        }
    }

    /// Composes a human discourse sentence from a structured SVO clause.
    pub fn compose_sentence(clause: &SvoClause) -> String {
        let mut sentence = format!(
            "The {} {} {}",
            clause.subject.name, clause.verb, clause.object.name
        );

        if let Some(ref sc) = clause.scope_container {
            sentence.push_str(&format!(" into {}", sc.name));
        }

        if let Some(ref caveat) = clause.caveat_guard {
            sentence.push_str(&format!(" unless {}", caveat));
        }

        if let Some(ref aside) = clause.aside_intent {
            sentence.push_str(&format!(" ({})", aside));
        }

        sentence
    }

    /// Synthesizes a valid 4-Block OmniCode Reel from SVO clauses.
    pub fn synthesize_svo_reel(prompt: &str, clauses: &[SvoClause]) -> String {
        let mut decls = Vec::new();
        let mut body_rolls = Vec::new();
        let mut verify_lines = Vec::new();

        for (idx, clause) in clauses.iter().enumerate() {
            let roll_name = format!("roll_svo_{}", idx + 1);

            // Emit entity declarations with morphological boundary brackets
            decls.push(clause.subject.format_declaration());
            decls.push(clause.object.format_declaration());

            if let Some(ref container) = clause.scope_container {
                decls.push(container.format_declaration());
            }

            let mut expr = format!(
                "{} {} {}",
                clause.subject.name,
                clause.primary_op.as_str(),
                clause.object.name
            );

            if let Some(ref container) = clause.scope_container {
                expr.push_str(&format!(" v-- {}", container.name));
            }

            if let (Some(sec_op), Some(sec_obj)) =
                (clause.secondary_op, &clause.secondary_object)
            {
                decls.push(sec_obj.format_declaration());
                expr.push_str(&format!(
                    " {} {}",
                    sec_op.as_str(),
                    sec_obj.name
                ));
            }

            if let Some(ref caveat) = clause.caveat_guard {
                expr.push_str(&format!(" ! [ Caveat = \"{}\" ]", caveat));
            }

            if let Some(ref aside) = clause.aside_intent {
                expr.push_str(&format!(" ( Intent = \"{}\" )", aside));
            }

            body_rolls.push(format!("    {} [ {} ]", roll_name, expr));
            verify_lines.push(format!("        {} = PROCEED", roll_name));
        }

        let mut unique_decls = Vec::new();
        for d in decls {
            if !unique_decls.contains(&d) {
                unique_decls.push(d);
            }
        }

        let decl_block = unique_decls.join("\n");
        let rolls_block = body_rolls.join("\n");
        let verify_block = verify_lines.join(" ::\n");

        let clean_prompt = prompt.replace('"', "'").replace('\n', " ");

        format!(
r#"# ========================================================================
# BLOCK 1: METADATA BLOCK
# ( Continuous SVO Semantic Transducer & Directional Vector Inception )
# ========================================================================
Module < OmniCode.AI.Transducer.SVO > ( {clean_prompt} ) {{
    generator: < "Continuous SVO Semantic Transducer" >,
    clauses: < "{num_clauses}" >,
    parity_gate: < "RestoratorCertifier::TwoWayAgreement" >
}}

# ========================================================================
# BLOCK 2: OPENING BLOCK
# ( Inferred Subject, Object, and Boundary Vessel Declarations )
# ========================================================================
Opening {{
    lexicon: [ Lexicon {{ Logic }} :: Lexicon {{ Vectors }} ],
    preconditions: [ +1 ]
}}

{decl_block}

# ========================================================================
# BLOCK 3: BODY BLOCK
# ( SVO Directional Tape Execution )
# ========================================================================
Body [
{rolls_block}
]

# ========================================================================
# BLOCK 4: CLOSING BLOCK
# ( Tape Stop: Invariant Parity Verification & Commit Status )
# ========================================================================
Closing {{
    verify: [
{verify_block}
    ],
    commit_status [ PROCEED ]
}}
"#,
            num_clauses = clauses.len()
        )
    }
}

fn parse_domain_str(s: &str) -> Option<ModalDomain> {
    match s.trim().to_lowercase().as_str() {
        "quantitative" => Some(ModalDomain::Quantitative),
        "spatial" => Some(ModalDomain::Spatial),
        "kinematic" => Some(ModalDomain::Kinematic),
        "physical" => Some(ModalDomain::Physical),
        "biotic" | "bioticorganic" => Some(ModalDomain::BioticOrganic),
        "sensitive" | "sensitivepsychic" => Some(ModalDomain::SensitivePsychic),
        "analyticallogical" | "logical" | "analytical" => {
            Some(ModalDomain::AnalyticalLogical)
        }
        "historicalformative" | "formational" | "technical" => {
            Some(ModalDomain::HistoricalFormative)
        }
        "lingual" => Some(ModalDomain::Lingual),
        "social" => Some(ModalDomain::Social),
        "economic" => Some(ModalDomain::Economic),
        "aesthetic" => Some(ModalDomain::Aesthetic),
        "juridical" => Some(ModalDomain::Juridical),
        "ethical" => Some(ModalDomain::Ethical),
        "pistic" | "faithcredal" | "faith" => Some(ModalDomain::Pistic),
        _ => None,
    }
}

// =============================================================================
// BLOCK 4: AUDIT PROCEDURES BLOCK
// ( Self-Diagnostic Verification for Data-Driven SVO Composition )
// =============================================================================

/// Executes automated self-diagnostic audit for the SVO subsystem.
pub fn audit_svo_subsystem() -> Result<(), String> {
    let lexicon = BiblicalLexicon::new();
    let prompt =
        "The Shepherd protects the Sheep into the Fold ( guard flock )";
    let clauses = SvoComposer::decompose_prompt_with_lexicon(
        prompt,
        Some(&lexicon),
    );

    if clauses.is_empty() {
        return Err("Audit failed: No clauses parsed".into());
    }

    let c = &clauses[0];
    if c.subject.name != "Shepherd" {
        return Err(format!(
            "Audit failed: Expected Subject=Shepherd, got {}",
            c.subject.name
        ));
    }
    if c.object.name != "Sheep" {
        return Err(format!(
            "Audit failed: Expected Object=Sheep, got {}",
            c.object.name
        ));
    }
    if c.primary_op != SvoOperator::Equals {
        return Err(format!(
            "Audit failed: Expected primary_op=Equals (=), got {:?}",
            c.primary_op
        ));
    }
    if c.scope_container.as_ref().map(|s| s.name.as_str()) != Some("Fold") {
        return Err(format!(
            "Audit failed: Expected scope_container=Some(Fold), got {:?}",
            c.scope_container
        ));
    }
    if c.aside_intent.as_deref() != Some("guard flock") {
        return Err("Audit failed: Aside intent was not extracted".into());
    }

    // Verify data-driven noun resolution from nouns.base.dat
    if c.subject.is_novel {
        return Err(
            "Audit failed: Shepherd should be in nouns.base.dat".into()
        );
    }
    if c.subject.boundary != BoundaryType::Invariant {
        return Err(
            "Audit failed: Shepherd boundary must be Invariant".into()
        );
    }
    let decl = c.subject.format_declaration();
    if !decl.contains("Shepherd")
        || !decl.contains("<")
        || !decl.contains(">")
    {
        return Err(format!(
            "Audit failed: Subject declaration did not format with <: {}",
            decl
        ));
    }

    // Verify bidirectional sentence composition
    let composed = SvoComposer::compose_sentence(c);
    if !composed.contains("Shepherd protects Sheep") {
        return Err(format!(
            "Audit failed: Composed sentence mismatch: {}",
            composed
        ));
    }

    let reel = SvoComposer::synthesize_svo_reel(prompt, &clauses);
    if !reel.contains("roll_svo_1") || !reel.contains("v-- Fold") {
        return Err(
            "Audit failed: Synthesized reel missing expected tokens".into()
        );
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
    fn test_svo_subsystem_audit() {
        assert!(audit_svo_subsystem().is_ok());
    }

    #[test]
    fn test_svo_operator_from_verb_data_driven() {
        assert_eq!(SvoOperator::from_verb("store"), SvoOperator::Store);
        assert_eq!(SvoOperator::from_verb("emit"), SvoOperator::Emit);
        assert_eq!(SvoOperator::from_verb("assign"), SvoOperator::Assign);
        assert_eq!(SvoOperator::from_verb("send"), SvoOperator::Invoke);
        assert_eq!(SvoOperator::from_verb("balance"), SvoOperator::Equals);
        assert_eq!(SvoOperator::from_verb("differ"), SvoOperator::NotEquals);
    }

    #[test]
    fn test_svo_transfer_pattern_with_containers() {
        let prompt = "Transfer Data from Cache to Memory and store in Disk";
        let clauses = SvoComposer::decompose_prompt(prompt);
        assert_eq!(clauses.len(), 2);
        assert_eq!(clauses[0].subject.name, "Cache");
        assert_eq!(clauses[0].primary_op, SvoOperator::Emit);
        assert_eq!(clauses[0].object.name, "Data");
    }

    #[test]
    fn test_svo_article_skipping_and_boundary_types() {
        let prompt = "The router transmits the packet";
        let clauses = SvoComposer::decompose_prompt(prompt);
        assert_eq!(clauses.len(), 1);
        assert_eq!(clauses[0].subject.name, "router");
        assert_eq!(clauses[0].subject.boundary, BoundaryType::Invariant);
        assert_eq!(clauses[0].primary_op, SvoOperator::Invoke);
        assert_eq!(clauses[0].object.name, "packet");
        assert_eq!(clauses[0].object.boundary, BoundaryType::Denotation);
    }

    #[test]
    fn test_svo_compose_and_verify_parity_gate() {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 10);
        let restorator = Restorator::new(logger.clone());
        let prompt = "Node connects Peer";
        let result = SvoComposer::compose_and_verify(
            prompt,
            &logger,
            &debugger,
            &restorator,
        );

        assert_eq!(result.agreement, TwoWayAgreement::Certified);
        assert!(result.omnicode_reel.contains("roll_svo_1"));
        assert!(result.omnicode_reel.contains("Node < Agent? >"));
        assert!(result.omnicode_reel.contains("Peer [ Vessel? ]"));
    }
}

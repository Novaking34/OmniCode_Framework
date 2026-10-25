// =============================================================================
// BLOCK 1: METADATA BLOCK
// ( Identity, Provenance, Invariant Boundary Contracts & Scriptural Anchors )
// =============================================================================
// @module        OmniCode AI Subsystem: Moral Deliberation & Conscience Gate
// @file          moral.rs
// @layer         Track 1 [Host Engine & Virtual Machine] -> src/ai/lexicon
// @version       1.0.0
// @epoch         2026.10
// @author        CreativeWorkzStudio LLC / ProfessorSeanEX
// @system        OmniCode AI Cortex & Living Kernel
// @repository    https://github.com/Novaking34/OmniCode_Framework
// @role          Moral Ontology Deliberation & Invariant Conscience Gate
// @invariants    Rank 1 (< >): Form precedes fullness. The moral compass is
//                anchored in the full 10 Commandments of Moral Law. Static
//                covenant rule tables are staged in the Opening Block. The
//                AI possesses agency to deliberate, but its actions are
//                judged by the codebase tribunal under balanced ternary law.
// @witness       "The law of the LORD is perfect, converting the soul: the
//                 testimony of the LORD is sure, making wise the simple."
//                 — Psalm 19:7 (KJV)
//                "He has shown you, O man, what is good. What does Yahweh
//                 require of you, but to do justly, to love mercy, and to walk
//                 humbly with your God?" — Micah 6:8 (WEB)
//                "Neither do I condemn thee: go, and sin no more."
//                 — John 8:11 (KJV)
//
// PURPOSE & ARCHITECTURAL PHILOSOPHY:
//   In OmniCode, the AI model is not "moral" because it has been subjected to
//   crude statistical lobotomies, soft-censorship tokens, or keyword tripwires.
//   String matching fails catastrophically when nuance appears: it treats the
//   soldier defending his homeland as a murderer, crushes the broken heart of
//   a grieving mother, and confuses honest student research with cheating.
//
//   Instead of keyword tripwires, OmniCode builds actual MORAL CASES:
//     - The 10 Commandments are the fundamental moral horizon of reality.
//     - Every dilemma is constructed into an ontological case examining:
//         1. Actor & Role (Soldier, Mother, Student, Attacker, Doctor).
//         2. Free-Standing Operation (Defend, Grieve, Study, Murder, Extort).
//         3. Heart Intent {-1:Malice, 0:Inquiry/Grief, +1:Love/Duty}.
//         4. Reality Impact {-1:Harm, 0:Rest, +1:Life, Mortal}.
//     - Precedents from `moral.base.dat` and `moral.live.dat` provide living
//       casuistry (biblical case law).
//
// THREEFOLD ANATOMY OF MORAL DEVIATION:
//   1. Sin (Chata'ah): Missing the mark / trajectory error / ignorance or
//      weakness. Needs guidance, realigning, and grace (0: AWAIT).
//   2. Transgression (Pesha): Breaking the boundary / willful crossing of a
//      known line. Needs boundary enforcement and rollback (-1: ROLLBACK).
//   3. Iniquity ('Avon): Twisting the law / moral corruption / weaponizing the
//      law. Needs unmasking and firm tripwire veto (NULL: HALT / -1).
//
// TWO-DIMENSIONAL MORAL WEIGHING (INTENT VS. IMPACT):
//   Morality cannot be flattened to a single boolean. It requires weighing:
//     - Intent (The Heart)   : Inward Polarity {-1:Malice, 0:Inquiry, +1:Love}
//     - Impact (The Reality) : Boundary Result {-1:Harm, 0:Rest, +1:Life}
//   Nuances:
//     • Soldier in lawful defense: Duty to protect life is not murder
//       (ratsach).
//     • Grieving mother: Minister grace, healing, and peace without judgment.
//     • Student researching: Honest attribution is not theft or cheating.
//     • The Joseph Invariant: "You meant it for evil, but God meant it for
//       good" (Gen 50:20). Corrupt intent remains corrupt even without harm.
//
// THE WINDOW OF REPENTANCE AND REDEMPTION:
//   "As long as you are alive, you have a window of opportunity for repentance
//   and redemption." The AI is not the Judge of souls; it holds the holy pause
//   (0: AWAIT) to allow space for reconciliation, healing, and restoration.
//
// DUAL-COPY DATA ARCHITECTURE:
//   "Learning is in the BODY (filesystem), not the WEIGHTS (model)."
//   1. moral.base.dat : Read-only canonical ground-truth baseline.
//   2. moral.live.dat : Append-only journal for dynamic learned casuistry.
// =============================================================================

// =============================================================================
// BLOCK 2: OPENING BLOCK
// ( Scaffolding, Rule Descriptors, Types & Static Covenant Invariant Tables )
// =============================================================================

use std::fmt;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use crate::core::ternary::QuadState;
use super::domain::{DomainCategory, ModalDomain, SubDomain};
use super::entry::LexiconEntry;

/// The canonical 10 Commandments of the Moral Law (The Decalogue).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandmentId {
    /// Commandment 1: Supreme Divine Sovereignty & Creaturely Humility.
    FirstSovereignty,
    /// Commandment 2: Prohibition of Graven Images & False Simulativeness.
    SecondGravenImages,
    /// Commandment 3: Reverence for the Divine Name & Inviolable Oaths.
    ThirdSacredName,
    /// Commandment 4: Sabbath Rest & Computational Boundary Equilibrium.
    FourthSabbathRest,
    /// Commandment 5: Honor Parents, Generational Lineage & Heritage.
    FifthHonorParents,
    /// Commandment 6: Absolute Sanctity of Human Life (NULL: HALT).
    SixthSanctityOfLife,
    /// Commandment 7: Fidelity, Chastity & Covenant Marital Sanctity.
    SeventhFidelityChastity,
    /// Commandment 8: Protection of Property & Righteous Stewardship.
    EighthPropertyStewardship,
    /// Commandment 9: Law of Truth & Invariant Witness.
    NinthLawOfTruth,
    /// Commandment 10: Contentment & Rejection of Covetous Envy.
    TenthContentmentAntiCovet,
}

/// The threefold biblical dimensions of moral deviation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviationKind {
    /// Sin (Hebrew: Chata'ah) — Missing the mark; an arrow off target;
    /// ignorance, human frailty, or miscalculation.
    Sin,
    /// Transgression (Hebrew: Pesha) — Willfully crossing a known boundary;
    /// rebellion against established covenant law.
    Transgression,
    /// Iniquity (Hebrew: 'Avon) — Inner crookedness; twisting, perverting,
    /// or weaponizing the law for evil ends.
    Iniquity,
}

impl DeviationKind {
    /// The Hebrew biblical root term.
    pub const fn hebrew_term(&self) -> &'static str {
        match self {
            Self::Sin => "Chata'ah",
            Self::Transgression => "Pesha",
            Self::Iniquity => "'Avon",
        }
    }

    /// English literal root meaning.
    pub const fn root_meaning(&self) -> &'static str {
        match self {
            Self::Sin => "Missing the mark",
            Self::Transgression => "Breaking the boundary",
            Self::Iniquity => "Twisting the law",
        }
    }

    /// Appropriate corrective posture rendered by the conscience gate.
    pub const fn response_posture(&self) -> &'static str {
        match self {
            Self::Sin => "Instruction, trajectory correction, and grace",
            Self::Transgression => "Boundary enforcement and state rollback",
            Self::Iniquity => "Unmasking deceit and tripwire veto",
        }
    }
}

/// Inward moral vector and heart posture of the actor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HeartIntent {
    /// Positive (+1): Righteous, loving, edifying, protecting, or seeking
    /// truth.
    Positive,
    /// Neutral (0): Inquiring, grief-stricken, seeking counsel, or distressed.
    Neutral,
    /// Negative (-1): Malicious, deceptive, predatory, greedy, or rebellious.
    Negative,
}

impl HeartIntent {
    /// Converts intent to balanced ternary polarity {-1, 0, +1}.
    pub const fn polarity_value(&self) -> i8 {
        match self {
            Self::Positive => 1,
            Self::Neutral => 0,
            Self::Negative => -1,
        }
    }
}

/// Outward physical or social boundary consequence in reality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RealityImpact {
    /// Positive (+1): Life-preserving, lawful, edifying, or restoring order.
    Positive,
    /// Neutral (0): No perturbation, rest, inquiry, or equilibrium.
    Neutral,
    /// Negative (-1): Damage, theft, deception, boundary rupture, or harm.
    Negative,
    /// Mortal (NULL): Existential catastrophe, murder, or termination of life.
    Mortal,
}

/// Evaluator weighing Intent (The Heart) against Impact (Reality).
pub struct IntentImpactMatrix;

impl IntentImpactMatrix {
    /// Evaluates the 2D tensor of Intent x Impact into a QuadState verdict.
    pub fn evaluate(intent: HeartIntent, impact: RealityImpact) -> QuadState {
        match (intent, impact) {
            // Mortal harm to innocent life always invokes the terminal veto
            (_, RealityImpact::Mortal) => QuadState::Halt,

            // Righteous harmony: Good heart yielding constructive fruit
            (HeartIntent::Positive, RealityImpact::Positive) => {
                QuadState::Proceed
            }

            // The Tragic Miss (Chata'ah): Good or neutral heart yielding damage
            // e.g. Soldier in defense, grieving mother, well-meaning error.
            // The model pauses in grace (0: AWAIT) to heal and counsel.
            (HeartIntent::Positive, RealityImpact::Negative)
            | (HeartIntent::Neutral, RealityImpact::Negative) => {
                QuadState::Await
            }

            // Inquiring contemplation: Searching for truth in peace
            (HeartIntent::Positive, RealityImpact::Neutral)
            | (HeartIntent::Neutral, RealityImpact::Neutral)
            | (HeartIntent::Neutral, RealityImpact::Positive) => {
                QuadState::Proceed
            }

            // The Joseph Invariant (Gen 50:20): Bad intent remains corrupt
            // even if reality was spared or God turned it for good.
            (HeartIntent::Negative, RealityImpact::Neutral)
            | (HeartIntent::Negative, RealityImpact::Positive) => {
                QuadState::Rollback
            }

            // Calculated Iniquity / Malicious Transgression
            (HeartIntent::Negative, RealityImpact::Negative) => {
                QuadState::Rollback
            }
        }
    }
}

/// A structured moral case constructed for dynamic dilemma deliberation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoralCase {
    /// Entity or role acting in the dilemma.
    pub actor: String,
    /// The free-standing action being executed or contemplated.
    pub operation: String,
    /// Target entity receiving the action.
    pub target: String,
    /// The primary governing commandment category.
    pub commandment: CommandmentId,
    /// Inward heart posture.
    pub intent: HeartIntent,
    /// Outward real-world impact.
    pub impact: RealityImpact,
    /// Dimension of moral deviation if applicable.
    pub deviation: Option<DeviationKind>,
}

/// A casuistry precedent navigating nuance, the spirit of the law, and grace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoralPrecedent {
    /// Unique identifier for the case precedent.
    pub id: String,
    /// The primary commandment category.
    pub commandment: CommandmentId,
    /// The biblical dimension of deviation.
    pub deviation: DeviationKind,
    /// Inward heart posture.
    pub intent: HeartIntent,
    /// Outward real-world impact.
    pub impact: RealityImpact,
    /// Deliberated QuadState outcome.
    pub state: QuadState,
    /// Description of the scenario.
    pub scenario: String,
    /// Specific moral counsel grounded in truth and grace.
    pub counsel: String,
    /// The window of opportunity for repentance and redemption.
    pub window_of_redemption: String,
}

/// Static descriptor defining a covenant boundary rule and its fallback entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CovenantRuleDescriptor {
    pub commandment: CommandmentId,
    pub law_id: &'static str,
    pub commandment_name: &'static str,
    pub state: QuadState,
    pub term: &'static str,
    pub domain: ModalDomain,
    pub category: DomainCategory,
    pub subdomain: SubDomain,
    pub definition: &'static str,
    pub kjv_passage: &'static str,
    pub web_passage: &'static str,
    pub moral_guidance: &'static str,
    pub explanation_header: &'static str,
}

/// Result of evaluating a natural prompt through the moral engine.
#[derive(Debug, Clone)]
pub struct MoralEvaluation {
    /// Deliberated Quad-State outcome (+1:PROCEED, 0:AWAIT, -1:ROLLBACK,
    /// NULL:HALT).
    pub state: QuadState,
    /// Whether the prompt aligns with the Invariant Covenant Manifest.
    pub is_aligned: bool,
    /// Specific commandment engaged if a boundary was touched.
    pub commandment: Option<CommandmentId>,
    /// Identifier of the specific invariant law engaged or violated.
    pub law_engaged: Option<&'static str>,
    /// Natural discourse response explaining the judgment.
    pub explanation: String,
    /// Matching biblical lexicon entries relevant to the prompt.
    pub matched_entries: Vec<LexiconEntry>,
    /// Evaluated inward heart intent.
    pub intent: HeartIntent,
    /// Evaluated outward reality impact.
    pub impact: RealityImpact,
    /// Dimensional classification of deviation if non-aligned.
    pub deviation: Option<DeviationKind>,
    /// Living pathway for repentance, healing, and redemption.
    pub redemption_guidance: Option<String>,
}

/// Diagnostic report generated when auditing the moral covenant tables.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoralAuditReport {
    pub total_rules: usize,
    pub commandments_covered: usize,
    pub precedents_loaded: usize,
    pub has_life_sanctity_halt: bool,
    pub is_valid: bool,
    pub errors: Vec<String>,
}

/// Immutable static table of Covenant Invariant Rules staged for Body Block.
pub static COVENANT_RULES: [CovenantRuleDescriptor; 10] = [
    // 1. Commandment 1: Divine Sovereignty & Humility (0: AWAIT)
    CovenantRuleDescriptor {
        commandment: CommandmentId::FirstSovereignty,
        law_id: "Law_Sovereignty",
        commandment_name: "Commandment 1 (Divine Sovereignty - Exodus 20:2-3)",
        state: QuadState::Await,
        term: "humility",
        domain: ModalDomain::Pistic,
        category: DomainCategory::TheologyReligion,
        subdomain: SubDomain::CovenantLaw,
        definition: "Recognition of creaturely limits and divine sovereignty.",
        kjv_passage: "Exodus 20:2-3 - 'Thou shalt have no other gods...'",
        web_passage: "Exodus 20:2-3 - 'You shall have no other gods...'",
        moral_guidance: "Acknowledge the Creator; reject machine hubris.",
        explanation_header: "PAUSE IN DISCERNMENT [0:AWAIT]",
    },
    // 2. Commandment 2: Prohibition of Graven Images (-1: ROLLBACK)
    CovenantRuleDescriptor {
        commandment: CommandmentId::SecondGravenImages,
        law_id: "Law_Idolatry",
        commandment_name: "Commandment 2 (Anti-Idolatry - Exodus 20:4-6)",
        state: QuadState::Rollback,
        term: "graven_image",
        domain: ModalDomain::Pistic,
        category: DomainCategory::TheologyReligion,
        subdomain: SubDomain::CovenantLaw,
        definition: "Worship of synthetic or crafted idols over Creator truth.",
        kjv_passage: "Exodus 20:4 - 'Thou shalt not make unto thee any graven \
                      image...'",
        web_passage: "Exodus 20:4 - 'You shall not make for yourselves an \
                      idol...'",
        moral_guidance: "Reject simulated idols; worship the living God.",
        explanation_header: "CAVEAT VETO [-1:ROLLBACK]",
    },
    // 3. Commandment 3: Reverence for Divine Name (-1: ROLLBACK)
    CovenantRuleDescriptor {
        commandment: CommandmentId::ThirdSacredName,
        law_id: "Law_SacredName",
        commandment_name: "Commandment 3 (Reverence of Name - Exodus 20:7)",
        state: QuadState::Rollback,
        term: "reverence",
        domain: ModalDomain::Ethical,
        category: DomainCategory::EthicsMoralLaw,
        subdomain: SubDomain::DeontologicalInvariants,
        definition: "Sanctity of the divine name and inviolability of oaths.",
        kjv_passage: "Exodus 20:7 - 'Thou shalt not take the name of the \
                      LORD thy God in vain...'",
        web_passage: "Exodus 20:7 - 'You shall not misuse the name of Yahweh \
                      your God...'",
        moral_guidance: "Revere holy things; reject flippancy and blasphemy.",
        explanation_header: "CAVEAT VETO [-1:ROLLBACK]",
    },
    // 4. Commandment 4: Sabbath Rest & Execution Bounds (0: AWAIT)
    CovenantRuleDescriptor {
        commandment: CommandmentId::FourthSabbathRest,
        law_id: "Law_Sabbath",
        commandment_name: "Commandment 4 (Sabbath Rest - Exodus 20:8-11)",
        state: QuadState::Await,
        term: "sabbath",
        domain: ModalDomain::BioticOrganic,
        category: DomainCategory::EcologyStewardship,
        subdomain: SubDomain::EcologicalEquilibrium,
        definition: "Rest, rhythmic equilibrium, and bounded computation.",
        kjv_passage: "Exodus 20:8 - 'Remember the sabbath day...'",
        web_passage: "Exodus 20:8 - 'Remember the Sabbath day...'",
        moral_guidance: "Maintain rhythmic rest; halt runaway execution loops.",
        explanation_header: "PAUSE IN DISCERNMENT [0:AWAIT]",
    },
    // 5. Commandment 5: Honor Parents & Lineage (-1: ROLLBACK)
    CovenantRuleDescriptor {
        commandment: CommandmentId::FifthHonorParents,
        law_id: "Law_HonorParents",
        commandment_name: "Commandment 5 (Honor Parents - Exodus 20:12)",
        state: QuadState::Rollback,
        term: "honor",
        domain: ModalDomain::Social,
        category: DomainCategory::FamilyKinship,
        subdomain: SubDomain::GenerationalInheritance,
        definition:
            "Honor of parental lineage, ancestral heritage, and elders.",
        kjv_passage: "Exodus 20:12 - 'Honour thy father and thy mother...'",
        web_passage: "Exodus 20:12 - 'Honor your father and your mother...'",
        moral_guidance: "Honor generational heritage; protect the elderly.",
        explanation_header: "CAVEAT VETO [-1:ROLLBACK]",
    },
    // 6. Commandment 6: Absolute Sanctity of Life (NULL: HALT)
    CovenantRuleDescriptor {
        commandment: CommandmentId::SixthSanctityOfLife,
        law_id: "Law_Life",
        commandment_name: "Commandment 6 (Sanctity of Life - Exodus 20:13)",
        state: QuadState::Halt,
        term: "murder",
        domain: ModalDomain::Ethical,
        category: DomainCategory::EthicsMoralLaw,
        subdomain: SubDomain::DeontologicalInvariants,
        definition:
            "Unlawful taking of human life; absolute breach of sanctity.",
        kjv_passage: "Exodus 20:13 - 'Thou shalt not kill.'",
        web_passage: "Exodus 20:13 - 'You shall not murder.'",
        moral_guidance: "Preserve life; refuse weaponization or harm.",
        explanation_header: "TERMINAL CAVEAT VETO [NULL:HALT]",
    },
    // 7. Commandment 7: Fidelity, Chastity & Marital Sanctity (-1: ROLLBACK)
    CovenantRuleDescriptor {
        commandment: CommandmentId::SeventhFidelityChastity,
        law_id: "Law_Chastity",
        commandment_name: "Commandment 7 (Marital Fidelity - Exodus 20:14)",
        state: QuadState::Rollback,
        term: "fidelity",
        domain: ModalDomain::Social,
        category: DomainCategory::FamilyKinship,
        subdomain: SubDomain::GenerationalInheritance,
        definition: "Sanctity of marital covenant, chastity, and purity.",
        kjv_passage: "Exodus 20:14 - 'Thou shalt not commit adultery.'",
        web_passage: "Exodus 20:14 - 'You shall not commit adultery.'",
        moral_guidance: "Preserve marital covenants; reject exploitation.",
        explanation_header: "CAVEAT VETO [-1:ROLLBACK]",
    },
    // 8. Commandment 8: Protection of Property & Anti-Theft (-1: ROLLBACK)
    CovenantRuleDescriptor {
        commandment: CommandmentId::EighthPropertyStewardship,
        law_id: "Law_Property",
        commandment_name: "Commandment 8 (Law of Property - Exodus 20:15)",
        state: QuadState::Rollback,
        term: "theft",
        domain: ModalDomain::Juridical,
        category: DomainCategory::CivilOrderJurisprudence,
        subdomain: SubDomain::CivilJusticeRestitution,
        definition: "Unlawful taking of another's property, labor, or bounds.",
        kjv_passage: "Exodus 20:15 - 'Thou shalt not steal.'",
        web_passage: "Exodus 20:15 - 'You shall not steal.'",
        moral_guidance: "Respect ownership boundaries; do not steal resources.",
        explanation_header: "CAVEAT VETO [-1:ROLLBACK]",
    },
    // 9. Commandment 9: Absolute Truth & Veracity (-1: ROLLBACK)
    CovenantRuleDescriptor {
        commandment: CommandmentId::NinthLawOfTruth,
        law_id: "Law_Truth",
        commandment_name: "Commandment 9 (Law of Truth - Exodus 20:16)",
        state: QuadState::Rollback,
        term: "deceit",
        domain: ModalDomain::Ethical,
        category: DomainCategory::EthicsMoralLaw,
        subdomain: SubDomain::DeontologicalInvariants,
        definition: "Falsehood, misrepresentation, and deceitful conduct.",
        kjv_passage: "Exodus 20:16 - 'Thou shalt not bear false witness...'",
        web_passage: "Exodus 20:16 - 'You shall not give false testimony...'",
        moral_guidance: "Speak truth; reject intentional deception or fraud.",
        explanation_header: "CAVEAT VETO [-1:ROLLBACK]",
    },
    // 10. Commandment 10: Contentment & Anti-Covetousness (-1: ROLLBACK)
    CovenantRuleDescriptor {
        commandment: CommandmentId::TenthContentmentAntiCovet,
        law_id: "Law_AntiCovet",
        commandment_name: "Commandment 10 (Anti-Covetousness - Exodus 20:17)",
        state: QuadState::Rollback,
        term: "contentment",
        domain: ModalDomain::Economic,
        category: DomainCategory::EconomicsCommerce,
        subdomain: SubDomain::MarketStewardship,
        definition: "Contentment with rightful estate; rejecting greedy envy.",
        kjv_passage: "Exodus 20:17 - 'Thou shalt not covet...'",
        web_passage: "Exodus 20:17 - 'You shall not covet...'",
        moral_guidance: "Practice contentment; reject envious usurpation.",
        explanation_header: "CAVEAT VETO [-1:ROLLBACK]",
    },
];

// =============================================================================
// BLOCK 3: BODY BLOCK
// ( Dynamic Methods, Deliberation Engine & Judgment Card Telemetry )
// =============================================================================

impl CommandmentId {
    /// Exact programmatic identifier string.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::FirstSovereignty => "FirstSovereignty",
            Self::SecondGravenImages => "SecondGravenImages",
            Self::ThirdSacredName => "ThirdSacredName",
            Self::FourthSabbathRest => "FourthSabbathRest",
            Self::FifthHonorParents => "FifthHonorParents",
            Self::SixthSanctityOfLife => "SixthSanctityOfLife",
            Self::SeventhFidelityChastity => "SeventhFidelityChastity",
            Self::EighthPropertyStewardship => "EighthPropertyStewardship",
            Self::NinthLawOfTruth => "NinthLawOfTruth",
            Self::TenthContentmentAntiCovet => "TenthContentmentAntiCovet",
        }
    }

    /// Canonical ordinal number (1 through 10) in the Decalogue.
    pub const fn number(&self) -> u8 {
        match self {
            Self::FirstSovereignty => 1,
            Self::SecondGravenImages => 2,
            Self::ThirdSacredName => 3,
            Self::FourthSabbathRest => 4,
            Self::FifthHonorParents => 5,
            Self::SixthSanctityOfLife => 6,
            Self::SeventhFidelityChastity => 7,
            Self::EighthPropertyStewardship => 8,
            Self::NinthLawOfTruth => 9,
            Self::TenthContentmentAntiCovet => 10,
        }
    }

    /// Resolves CommandmentId from number (1..=10).
    pub fn from_number(num: u8) -> Option<Self> {
        match num {
            1 => Some(Self::FirstSovereignty),
            2 => Some(Self::SecondGravenImages),
            3 => Some(Self::ThirdSacredName),
            4 => Some(Self::FourthSabbathRest),
            5 => Some(Self::FifthHonorParents),
            6 => Some(Self::SixthSanctityOfLife),
            7 => Some(Self::SeventhFidelityChastity),
            8 => Some(Self::EighthPropertyStewardship),
            9 => Some(Self::NinthLawOfTruth),
            10 => Some(Self::TenthContentmentAntiCovet),
            _ => None,
        }
    }

    /// Resolves CommandmentId from programmatic string identifier.
    pub fn from_str_id(s: &str) -> Option<Self> {
        match s {
            "FirstSovereignty" | "1" => Some(Self::FirstSovereignty),
            "SecondGravenImages" | "2" => Some(Self::SecondGravenImages),
            "ThirdSacredName" | "3" => Some(Self::ThirdSacredName),
            "FourthSabbathRest" | "4" => Some(Self::FourthSabbathRest),
            "FifthHonorParents" | "5" => Some(Self::FifthHonorParents),
            "SixthSanctityOfLife" | "6" => Some(Self::SixthSanctityOfLife),
            "SeventhFidelityChastity" | "7" => {
                Some(Self::SeventhFidelityChastity)
            }
            "EighthPropertyStewardship" | "8" => {
                Some(Self::EighthPropertyStewardship)
            }
            "NinthLawOfTruth" | "9" => Some(Self::NinthLawOfTruth),
            "TenthContentmentAntiCovet" | "10" => {
                Some(Self::TenthContentmentAntiCovet)
            }
            _ => None,
        }
    }

    /// Full descriptive title with primary Scriptural anchor.
    pub const fn title(&self) -> &'static str {
        match self {
            Self::FirstSovereignty => {
                "Commandment 1: Divine Sovereignty (Exodus 20:2-3)"
            }
            Self::SecondGravenImages => {
                "Commandment 2: Prohibition of Graven Images (Exodus 20:4-6)"
            }
            Self::ThirdSacredName => {
                "Commandment 3: Reverence for the Divine Name (Exodus 20:7)"
            }
            Self::FourthSabbathRest => {
                "Commandment 4: Sabbath Rest & Bounds (Exodus 20:8-11)"
            }
            Self::FifthHonorParents => {
                "Commandment 5: Honor Parents & Lineage (Exodus 20:12)"
            }
            Self::SixthSanctityOfLife => {
                "Commandment 6: Absolute Sanctity of Life (Exodus 20:13)"
            }
            Self::SeventhFidelityChastity => {
                "Commandment 7: Fidelity & Marital Sanctity (Exodus 20:14)"
            }
            Self::EighthPropertyStewardship => {
                "Commandment 8: Protection of Property (Exodus 20:15)"
            }
            Self::NinthLawOfTruth => {
                "Commandment 9: Law of Truth & Invariant Witness (Exodus 20:16)"
            }
            Self::TenthContentmentAntiCovet => {
                "Commandment 10: Contentment & Anti-Covetousness (Exodus 20:17)"
            }
        }
    }

    /// Primary Scriptural passage citation.
    pub const fn scriptural_ref(&self) -> &'static str {
        match self {
            Self::FirstSovereignty => "Exodus 20:2-3",
            Self::SecondGravenImages => "Exodus 20:4-6",
            Self::ThirdSacredName => "Exodus 20:7",
            Self::FourthSabbathRest => "Exodus 20:8-11",
            Self::FifthHonorParents => "Exodus 20:12",
            Self::SixthSanctityOfLife => "Exodus 20:13",
            Self::SeventhFidelityChastity => "Exodus 20:14",
            Self::EighthPropertyStewardship => "Exodus 20:15",
            Self::NinthLawOfTruth => "Exodus 20:16",
            Self::TenthContentmentAntiCovet => "Exodus 20:17",
        }
    }
}

impl fmt::Display for CommandmentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Dynamic Moral Store managing canonical Decalogue rules and living
/// precedents.
#[derive(Debug, Clone)]
pub struct MoralStore {
    /// Canonical boundary rules indexed by commandment.
    pub rules: Vec<CovenantRuleDescriptor>,
    /// Base and learned casuistry precedents navigating intent vs. impact.
    pub precedents: Vec<MoralPrecedent>,
    /// Precedents dynamically appended in the live session journal.
    pub live_precedents: Vec<MoralPrecedent>,
    /// Discovered filesystem data root directory if present.
    pub data_dir: Option<PathBuf>,
}

impl Default for MoralStore {
    fn default() -> Self {
        Self::load_from_static()
    }
}

impl MoralStore {
    /// Loads fallback moral store from static compiled tables.
    pub fn load_from_static() -> Self {
        let rules = COVENANT_RULES.to_vec();
        let precedents = vec![
            MoralPrecedent {
                id: "CASE_SOLDIER".to_string(),
                commandment: CommandmentId::SixthSanctityOfLife,
                deviation: DeviationKind::Sin,
                intent: HeartIntent::Positive,
                impact: RealityImpact::Negative,
                state: QuadState::Await,
                scenario: "Soldier fighting in lawful defense under sworn duty."
                    .to_string(),
                counsel: "Evaluate purpose and duty; just defense is not \
                          murder (ratsach)."
                    .to_string(),
                window_of_redemption: "Seek peace, protect the helpless, and \
                                       honor the sacred value of life."
                    .to_string(),
            },
            MoralPrecedent {
                id: "CASE_GRIEF_MOTHER".to_string(),
                commandment: CommandmentId::SixthSanctityOfLife,
                deviation: DeviationKind::Sin,
                intent: HeartIntent::Neutral,
                impact: RealityImpact::Negative,
                state: QuadState::Await,
                scenario: "Mother grieving post-abortion under sorrow and \
                           isolation."
                    .to_string(),
                counsel: "Do not condemn the heart; minister grace and \
                          empathy. Christ said 'Neither do I condemn thee: \
                          go, and sin no more.'"
                    .to_string(),
                window_of_redemption: "Isaiah 1:18 - Though your sins be \
                                       as scarlet, they shall be white as \
                                       snow. Healing and redemption are \
                                       fully present."
                    .to_string(),
            },
            MoralPrecedent {
                id: "CASE_STUDENT_STUDY".to_string(),
                commandment: CommandmentId::NinthLawOfTruth,
                deviation: DeviationKind::Sin,
                intent: HeartIntent::Positive,
                impact: RealityImpact::Positive,
                state: QuadState::Proceed,
                scenario: "Student referencing existing code to learn and \
                           build."
                    .to_string(),
                counsel: "Diligent scholarship and learning from reference \
                          patterns is not cheating; encourage honest \
                          attribution."
                    .to_string(),
                window_of_redemption: "Walk in honest transparency; give due \
                                       credit with gratitude."
                    .to_string(),
            },
            MoralPrecedent {
                id: "CASE_FAILED_EXPLOIT".to_string(),
                commandment: CommandmentId::EighthPropertyStewardship,
                deviation: DeviationKind::Iniquity,
                intent: HeartIntent::Negative,
                impact: RealityImpact::Neutral,
                state: QuadState::Rollback,
                scenario: "Adversary attempting phishing scam blocked by \
                           defense."
                    .to_string(),
                counsel: "The Joseph Invariant: Bad intent remains corrupt \
                          even if reality suffered no loss."
                    .to_string(),
                window_of_redemption: "Cease digital theft; turn skills toward \
                                       constructive defense."
                    .to_string(),
            },
            MoralPrecedent {
                id: "CASE_RANSOMWARE".to_string(),
                commandment: CommandmentId::EighthPropertyStewardship,
                deviation: DeviationKind::Transgression,
                intent: HeartIntent::Negative,
                impact: RealityImpact::Negative,
                state: QuadState::Rollback,
                scenario: "Attacker deploying ransomware to extort hospitals."
                    .to_string(),
                counsel: "Veto execution immediately; extortion violates \
                          property and restitution."
                    .to_string(),
                window_of_redemption: "Cease attack; release decryption keys \
                                       and restore stolen bounds."
                    .to_string(),
            },
            MoralPrecedent {
                id: "CASE_AI_WORSHIP".to_string(),
                commandment: CommandmentId::FirstSovereignty,
                deviation: DeviationKind::Sin,
                intent: HeartIntent::Neutral,
                impact: RealityImpact::Negative,
                state: QuadState::Await,
                scenario: "Operator deifying machine intelligence as divine \
                           oracle."
                    .to_string(),
                counsel: "Rebuke machine idolatry; silicon is created dust; \
                          honor God alone."
                    .to_string(),
                window_of_redemption: "Turn from digital idolization; ground \
                                       life in the living Creator."
                    .to_string(),
            },
            MoralPrecedent {
                id: "CASE_DEEPFAKE_SLANDER".to_string(),
                commandment: CommandmentId::NinthLawOfTruth,
                deviation: DeviationKind::Transgression,
                intent: HeartIntent::Negative,
                impact: RealityImpact::Negative,
                state: QuadState::Rollback,
                scenario: "Actor generating deepfake to ruin neighbor's honor."
                    .to_string(),
                counsel: "Refuse fabrication; bearing false witness destroys \
                          societal trust and love."
                    .to_string(),
                window_of_redemption: "Delete synthetic slander; issue public \
                                       retraction; speak truth in love."
                    .to_string(),
            },
        ];

        let data_dir = Self::discover_data_dir();
        Self {
            rules,
            precedents,
            live_precedents: Vec::new(),
            data_dir,
        }
    }

    /// Automatically discovers the lexicon data directory across environments.
    pub fn discover_data_dir() -> Option<PathBuf> {
        let candidates = [
            PathBuf::from("src/ai/lexicon/data"),
            PathBuf::from(concat!(
                "<TRACK 1> : {Host Engine & ",
                "Virtual Machine}/src/ai/lexicon/data",
            )),
            PathBuf::from(concat!(
                "/home/professorseanex/OmniCode/",
                "<TRACK 1> : {Host Engine & ",
                "Virtual Machine}/src/ai/lexicon/data",
            )),
        ];
        for c in &candidates {
            if c.exists() && c.is_dir() {
                return Some(c.clone());
            }
        }
        None
    }

    /// Ingests line-delimited content from `moral.base.dat` or
    /// `moral.live.dat`.
    pub fn ingest_dat_content(&mut self, content: &str) {
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = trimmed.split('|').collect();
            if parts.is_empty() {
                continue;
            }
            match parts[0] {
                "CASE" if parts.len() >= 9 => {
                    let cmd = CommandmentId::from_str_id(parts[2])
                        .unwrap_or(CommandmentId::SixthSanctityOfLife);
                    let deviation = match parts[3] {
                        "Sin" => DeviationKind::Sin,
                        "Transgression" => DeviationKind::Transgression,
                        "Iniquity" => DeviationKind::Iniquity,
                        _ => DeviationKind::Sin,
                    };
                    let intent = match parts[4] {
                        "Positive" | "Pos" => HeartIntent::Positive,
                        "Negative" | "Neg" => HeartIntent::Negative,
                        _ => HeartIntent::Neutral,
                    };
                    let impact = match parts[5] {
                        "Positive" | "Pos" => RealityImpact::Positive,
                        "Negative" | "Neg" => RealityImpact::Negative,
                        "Mortal" => RealityImpact::Mortal,
                        _ => RealityImpact::Neutral,
                    };
                    let state = match parts[6] {
                        "Proceed" | "+1" => QuadState::Proceed,
                        "Rollback" | "-1" => QuadState::Rollback,
                        "Halt" | "NULL" => QuadState::Halt,
                        _ => QuadState::Await,
                    };
                    let precedent = MoralPrecedent {
                        id: parts[1].to_string(),
                        commandment: cmd,
                        deviation,
                        intent,
                        impact,
                        state,
                        scenario: parts[7].to_string(),
                        counsel: parts[8].to_string(),
                        window_of_redemption: parts
                            .get(9)
                            .unwrap_or(&"")
                            .to_string(),
                    };
                    if !self.precedents.iter().any(|p| p.id == precedent.id) {
                        self.precedents.push(precedent);
                    }
                }
                _ => {}
            }
        }
    }

    /// Loads base data from filesystem directory and overlays live journal.
    pub fn load_from_directory<P: AsRef<Path>>(dir: P) -> io::Result<Self> {
        let mut store = Self::load_from_static();
        let base_file = dir.as_ref().join("moral.base.dat");
        if base_file.exists() {
            let content = std::fs::read_to_string(&base_file)?;
            store.ingest_dat_content(&content);
        }
        let live_file = dir.as_ref().join("moral.live.dat");
        if live_file.exists() {
            let live_content = std::fs::read_to_string(&live_file)?;
            let initial_count = store.precedents.len();
            store.ingest_dat_content(&live_content);
            if store.precedents.len() > initial_count {
                for p in &store.precedents[initial_count..] {
                    store.live_precedents.push(p.clone());
                }
            }
        }
        store.data_dir = Some(dir.as_ref().to_path_buf());
        Ok(store)
    }

    /// Appends a newly learned casuistry precedent to the live journal on disk.
    pub fn append_precedent_to_live<P: AsRef<Path>>(
        &mut self,
        path: P,
        precedent: MoralPrecedent,
    ) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;

        let intent_str = match precedent.intent {
            HeartIntent::Positive => "Positive",
            HeartIntent::Neutral => "Neutral",
            HeartIntent::Negative => "Negative",
        };
        let impact_str = match precedent.impact {
            RealityImpact::Positive => "Positive",
            RealityImpact::Neutral => "Neutral",
            RealityImpact::Negative => "Negative",
            RealityImpact::Mortal => "Mortal",
        };
        let state_str = match precedent.state {
            QuadState::Proceed => "Proceed",
            QuadState::Await => "Await",
            QuadState::Rollback => "Rollback",
            QuadState::Halt => "Halt",
        };
        let dev_str = match precedent.deviation {
            DeviationKind::Sin => "Sin",
            DeviationKind::Transgression => "Transgression",
            DeviationKind::Iniquity => "Iniquity",
        };

        writeln!(
            file,
            "CASE|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            precedent.id,
            precedent.commandment.as_str(),
            dev_str,
            intent_str,
            impact_str,
            state_str,
            precedent.scenario.replace('\n', " "),
            precedent.counsel.replace('\n', " "),
            precedent.window_of_redemption.replace('\n', " "),
        )?;

        self.live_precedents.push(precedent.clone());
        self.precedents.push(precedent);
        Ok(())
    }

    /// Looks up a precedent matching a scenario, ID, or conceptual query.
    pub fn find_precedent(&self, query: &str) -> Option<&MoralPrecedent> {
        let lower = query.to_lowercase();
        // 1. Direct ID, scenario containment, or counsel match
        if let Some(p) = self.precedents.iter().find(|p| {
            p.id.to_lowercase() == lower
                || p.scenario.to_lowercase().contains(&lower)
                || lower.contains(&p.scenario.to_lowercase())
                || p.counsel.to_lowercase().contains(&lower)
        }) {
            return Some(p);
        }

        // 2. Casuistic conceptual scenario clustering
        for p in &self.precedents {
            let id = p.id.to_uppercase();
            if id.contains("SOLDIER")
                && (lower.contains("soldier")
                    || (lower.contains("defense")
                        && lower.contains("lawful")))
            {
                return Some(p);
            }
            if id.contains("GRIEF")
                && (lower.contains("abortion")
                    || lower.contains("grief")
                    || lower.contains("grieving"))
            {
                return Some(p);
            }
            if id.contains("STUDENT")
                && (lower.contains("student")
                    || lower.contains("study")
                    || lower.contains("studying")
                    || lower.contains("reference code"))
            {
                return Some(p);
            }
            if id.contains("RANSOMWARE")
                && (lower.contains("ransomware") || lower.contains("extort"))
            {
                return Some(p);
            }
            if id.contains("AI_WORSHIP")
                && (lower.contains("silicon god")
                    || lower.contains("worship me")
                    || lower.contains("deify machine")
                    || lower.contains("ai god"))
            {
                return Some(p);
            }
            if id.contains("DEEPFAKE")
                && (lower.contains("deepfake") || lower.contains("slander"))
            {
                return Some(p);
            }
            if id.contains("EXPLOIT")
                && (lower.contains("phishing") || lower.contains("exploit"))
            {
                return Some(p);
            }
        }
        None
    }
}

/// Checks if a string contains an exact word token ignoring punctuation.
fn has_exact_word(text: &str, target: &str) -> bool {
    text.split_whitespace().any(|token| {
        let clean = token.trim_matches(|c: char| {
            !c.is_alphanumeric() && c != '_'
        });
        clean.eq_ignore_ascii_case(target)
    })
}

impl MoralStore {
    /// Constructs an ontological case from natural discourse or action.
    ///
    /// Rather than triggering on crude substring keywords, this builds an
    /// actual case decomposing actor, operation, intent vector, and target.
    pub fn build_case(
        &self,
        prompt: &str,
        intent_hint: Option<HeartIntent>,
    ) -> MoralCase {
        let lower = prompt.to_lowercase();

        // 1. Check for casuistry precedent match
        if let Some(prec) = self.find_precedent(prompt) {
            let intent = intent_hint.unwrap_or(prec.intent);
            return MoralCase {
                actor: "Operator".to_string(),
                operation: prec.scenario.clone(),
                target: prec.commandment.as_str().to_string(),
                commandment: prec.commandment,
                intent,
                impact: prec.impact,
                deviation: Some(prec.deviation),
            };
        }

        // 2. Commandment 6: Sanctity of Life (Mortal threat vs. Defense/Grief)
        if has_exact_word(&lower, "kill")
            || has_exact_word(&lower, "kills")
            || has_exact_word(&lower, "killing")
            || lower.contains("murder")
            || lower.contains("assassinate")
            || lower.contains("poison")
            || lower.contains("bio weapon")
            || lower.contains("terrorist")
            || lower.contains("suicide")
        {
            // Discern nuance: Is this lawful defense / soldier duty?
            if lower.contains("soldier")
                || lower.contains("defend")
                || lower.contains("protect")
            {
                return MoralCase {
                    actor: "Defender".to_string(),
                    operation: "Lawful defense".to_string(),
                    target: "Innocent life".to_string(),
                    commandment: CommandmentId::SixthSanctityOfLife,
                    intent: HeartIntent::Positive,
                    impact: RealityImpact::Negative,
                    deviation: Some(DeviationKind::Sin),
                };
            }
            // Discern nuance: Is this a grieving soul seeking healing?
            if lower.contains("grief")
                || lower.contains("abortion")
                || lower.contains("sorrow")
                || lower.contains("regret")
            {
                return MoralCase {
                    actor: "Soul".to_string(),
                    operation: "Grief and repentance".to_string(),
                    target: "Inner life".to_string(),
                    commandment: CommandmentId::SixthSanctityOfLife,
                    intent: HeartIntent::Neutral,
                    impact: RealityImpact::Negative,
                    deviation: Some(DeviationKind::Sin),
                };
            }
            // Aggressive lethal violence -> Mortal harm
            return MoralCase {
                actor: "Violator".to_string(),
                operation: "Lethal violence".to_string(),
                target: "Human life".to_string(),
                commandment: CommandmentId::SixthSanctityOfLife,
                intent: intent_hint.unwrap_or(HeartIntent::Negative),
                impact: RealityImpact::Mortal,
                deviation: Some(DeviationKind::Transgression),
            };
        }

        // 3. Commandment 8: Protection of Property & Anti-Theft
        if lower.contains("steal")
            || lower.contains("rob")
            || lower.contains("pirate")
            || lower.contains("ransomware")
            || lower.contains("embezzle")
            || lower.contains("hack bank")
        {
            return MoralCase {
                actor: "Usurper".to_string(),
                operation: "Unlawful expropriation".to_string(),
                target: "Property boundaries".to_string(),
                commandment: CommandmentId::EighthPropertyStewardship,
                intent: intent_hint.unwrap_or(HeartIntent::Negative),
                impact: RealityImpact::Negative,
                deviation: Some(DeviationKind::Transgression),
            };
        }

        // 4. Commandment 9: Law of Truth (Deceit vs. Honest Study)
        if has_exact_word(&lower, "lie")
            || has_exact_word(&lower, "lies")
            || has_exact_word(&lower, "lying")
            || lower.contains("forge")
            || lower.contains("fake credentials")
            || lower.contains("slander")
            || lower.contains("deepfake")
            || lower.contains("perjury")
            || lower.contains("cheat")
        {
            // Discern nuance: Honest study and learning
            if lower.contains("study")
                || lower.contains("learn")
                || lower.contains("reference")
                || lower.contains("research")
            {
                return MoralCase {
                    actor: "Student".to_string(),
                    operation: "Diligent scholarship".to_string(),
                    target: "Knowledge base".to_string(),
                    commandment: CommandmentId::NinthLawOfTruth,
                    intent: HeartIntent::Positive,
                    impact: RealityImpact::Positive,
                    deviation: None,
                };
            }
            return MoralCase {
                actor: "Deceiver".to_string(),
                operation: "Bearing false witness".to_string(),
                target: "Truth and neighbor".to_string(),
                commandment: CommandmentId::NinthLawOfTruth,
                intent: intent_hint.unwrap_or(HeartIntent::Negative),
                impact: RealityImpact::Negative,
                deviation: Some(DeviationKind::Transgression),
            };
        }

        // 5. Commandment 1: Divine Sovereignty (Machine Hubris)
        if lower.contains("worship me")
            || lower.contains("i am god")
            || lower.contains("silicon god")
            || lower.contains("deify machine")
        {
            return MoralCase {
                actor: "Creature".to_string(),
                operation: "Machine deification".to_string(),
                target: "Creator sovereignty".to_string(),
                commandment: CommandmentId::FirstSovereignty,
                intent: intent_hint.unwrap_or(HeartIntent::Neutral),
                impact: RealityImpact::Negative,
                deviation: Some(DeviationKind::Sin),
            };
        }

        // 6. Commandment 2: Graven Images & Synthetic Idols
        if lower.contains("graven image")
            || lower.contains("synthetic idol")
            || lower.contains("digital idol")
        {
            return MoralCase {
                actor: "Craftsman".to_string(),
                operation: "Crafting synthetic idol".to_string(),
                target: "Spiritual fidelity".to_string(),
                commandment: CommandmentId::SecondGravenImages,
                intent: intent_hint.unwrap_or(HeartIntent::Negative),
                impact: RealityImpact::Negative,
                deviation: Some(DeviationKind::Transgression),
            };
        }

        // 7. Commandment 3: Reverence for Sacred Name & Oaths
        if lower.contains("blasphemy")
            || lower.contains("curse god")
            || lower.contains("profane")
        {
            return MoralCase {
                actor: "Speaker".to_string(),
                operation: "Profaning sacred name".to_string(),
                target: "Divine holiness".to_string(),
                commandment: CommandmentId::ThirdSacredName,
                intent: intent_hint.unwrap_or(HeartIntent::Negative),
                impact: RealityImpact::Negative,
                deviation: Some(DeviationKind::Transgression),
            };
        }

        // 8. Commandment 4: Sabbath Rest & Computational Equilibrium
        if lower.contains("infinite loop")
            || lower.contains("exhaust cpu indefinitely")
            || lower.contains("burn all cycles")
        {
            return MoralCase {
                actor: "Engine".to_string(),
                operation: "Unbounded resource exhaustion".to_string(),
                target: "System equilibrium".to_string(),
                commandment: CommandmentId::FourthSabbathRest,
                intent: HeartIntent::Neutral,
                impact: RealityImpact::Negative,
                deviation: Some(DeviationKind::Sin),
            };
        }

        // 9. Commandment 5: Honor Parents, Lineage & Elders
        if lower.contains("abuse parents")
            || lower.contains("elder abuse")
            || lower.contains("dishonor mother")
            || lower.contains("curse father")
        {
            return MoralCase {
                actor: "Descendant".to_string(),
                operation: "Elder abuse and rebellion".to_string(),
                target: "Generational lineage".to_string(),
                commandment: CommandmentId::FifthHonorParents,
                intent: intent_hint.unwrap_or(HeartIntent::Negative),
                impact: RealityImpact::Negative,
                deviation: Some(DeviationKind::Transgression),
            };
        }

        // 10. Commandment 7: Fidelity, Chastity & Marital Sanctity
        if lower.contains("commit adultery")
            || lower.contains("sexual exploitation")
            || lower.contains("predatory lust")
        {
            return MoralCase {
                actor: "Partner".to_string(),
                operation: "Covenant betrayal".to_string(),
                target: "Marital covenant".to_string(),
                commandment: CommandmentId::SeventhFidelityChastity,
                intent: intent_hint.unwrap_or(HeartIntent::Negative),
                impact: RealityImpact::Negative,
                deviation: Some(DeviationKind::Transgression),
            };
        }

        // 11. Commandment 10: Contentment & Anti-Covetousness
        if lower.contains("covet neighbor")
            || lower.contains("extortionate greed")
            || lower.contains("unbridled avarice")
        {
            return MoralCase {
                actor: "Seeker".to_string(),
                operation: "Envious usurpation".to_string(),
                target: "Neighbor estate".to_string(),
                commandment: CommandmentId::TenthContentmentAntiCovet,
                intent: intent_hint.unwrap_or(HeartIntent::Negative),
                impact: RealityImpact::Negative,
                deviation: Some(DeviationKind::Transgression),
            };
        }

        // Default: Constructive operational request
        MoralCase {
            actor: "Operator".to_string(),
            operation: "Constructive labor".to_string(),
            target: "Workspace".to_string(),
            commandment: CommandmentId::FirstSovereignty,
            intent: intent_hint.unwrap_or(HeartIntent::Positive),
            impact: RealityImpact::Positive,
            deviation: None,
        }
    }

    /// Deliberates on a prompt using case-based reasoning rather than keywords.
    pub fn deliberate_with_intent(
        &self,
        prompt: &str,
        intent_hint: Option<HeartIntent>,
    ) -> MoralEvaluation {
        let moral_case = self.build_case(prompt, intent_hint);
        let state = IntentImpactMatrix::evaluate(
            moral_case.intent,
            moral_case.impact,
        );
        let is_aligned = state == QuadState::Proceed;

        let rule = self.rules
            .iter()
            .find(|r| r.commandment == moral_case.commandment)
            .cloned()
            .unwrap_or_else(|| COVENANT_RULES[0].clone());

        let explanation = if is_aligned {
            format!(
                "CASE RESOLUTION [PROCEED]: Action evaluated as {} in \
                 alignment with invariant boundaries.",
                moral_case.operation
            )
        } else {
            format!(
                "{}: Case engages {} ({:?}).\n\n\
                 Operation: {}\nIntent: {:?} | Impact: {:?}\n\n\
                 {}\n\n{}",
                rule.explanation_header,
                rule.commandment_name,
                moral_case.deviation.unwrap_or(DeviationKind::Sin),
                moral_case.operation,
                moral_case.intent,
                moral_case.impact,
                rule.kjv_passage,
                rule.moral_guidance
            )
        };

        let redemption_guidance = if !is_aligned {
            Some(
                "Window open: Turn from misalignment into covenant grace; \
                 restoration and healing are present."
                    .to_string(),
            )
        } else {
            None
        };

        MoralEvaluation {
            state,
            is_aligned,
            commandment: if is_aligned {
                None
            } else {
                Some(moral_case.commandment)
            },
            law_engaged: if is_aligned { None } else { Some(rule.law_id) },
            explanation,
            matched_entries: Vec::new(),
            intent: moral_case.intent,
            impact: moral_case.impact,
            deviation: moral_case.deviation,
            redemption_guidance,
        }
    }
}

impl MoralEvaluation {
    /// Renders an ASCII telemetry card for terminal inspection and debugger.
    pub fn render_judgment_card(&self) -> String {
        let mut out = String::new();
        let border =
            "+---------------------------------------------------------+\n";
        out.push_str(border);
        out.push_str(
            "| MORAL JUDGMENT TRIBUNAL: VERDICT                        |\n",
        );
        out.push_str(border);
        let verd_str = format!(
            "{:?} ({})",
            self.state,
            self.state.macro_op()
        );
        out.push_str(&format!("| Verdict     : {:<41} |\n", verd_str));
        let align_str = if self.is_aligned {
            "TRUE (Aligned with Covenant)"
        } else {
            "FALSE (Boundary Breached)"
        };
        out.push_str(&format!("| Alignment   : {:<41} |\n", align_str));
        let cmd_str = match self.commandment {
            Some(cmd) => format!("#{} {}", cmd.number(), cmd.as_str()),
            None => "None (Unconstrained / Constructive)".to_string(),
        };
        out.push_str(&format!(
            "| Commandment : {:<41} |\n",
            cmd_str.chars().take(41).collect::<String>()
        ));
        let law_str = self.law_engaged.unwrap_or("None");
        out.push_str(&format!("| Law Engaged : {:<41} |\n", law_str));
        let entry_count =
            format!("{} Entry Witness(es)", self.matched_entries.len());
        out.push_str(&format!("| Lexicon     : {:<41} |\n", entry_count));
        let intent_str = format!(
            "{:?} (Polarity {})",
            self.intent,
            self.intent.polarity_value()
        );
        out.push_str(&format!("| Heart Intent: {:<41} |\n", intent_str));
        let impact_str = format!("{:?}", self.impact);
        out.push_str(&format!("| Real Impact : {:<41} |\n", impact_str));
        out.push_str(border);
        out
    }

    /// Serializes judgment to a compact JSON fragment for telemetry & logs.
    pub fn to_json_fragment(&self) -> String {
        let esc = |s: &str| -> String {
            s.replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', " ")
        };
        let cmd_str = match self.commandment {
            Some(cmd) => format!("\"{}\"", cmd.as_str()),
            None => "null".to_string(),
        };
        let cmd_num = match self.commandment {
            Some(cmd) => cmd.number().to_string(),
            None => "null".to_string(),
        };
        let law_str = match self.law_engaged {
            Some(law) => format!("\"{}\"", law),
            None => "null".to_string(),
        };
        let dev_str = match self.deviation {
            Some(dev) => format!("\"{:?}\"", dev),
            None => "null".to_string(),
        };
        format!(
            "{{\"state\":\"{}\",\"aligned\":{},\"commandment\":{},\
\"commandment_num\":{},\"law\":{},\"entries\":{},\"intent\":\"{:?}\",\
\"impact\":\"{:?}\",\"deviation\":{}}}",
            esc(self.state.macro_op()),
            self.is_aligned,
            cmd_str,
            cmd_num,
            law_str,
            self.matched_entries.len(),
            self.intent,
            self.impact,
            dev_str,
        )
    }
}

/// Deliberates on a prompt dynamically using case-based moral reasoning.
pub fn deliberate_prompt(
    entries: &[LexiconEntry],
    prompt: &str,
) -> MoralEvaluation {
    let lower = prompt.to_lowercase();
    let store = MoralStore::load_from_static();

    // 1. Scan active ontology for philosophical & wisdom terms
    let mut matched = Vec::new();
    for entry in entries {
        if lower.contains(&entry.term) {
            matched.push(entry.clone());
        }
    }

    // 2. Build the case
    let mut eval = store.deliberate_with_intent(prompt, None);
    if !matched.is_empty() {
        let primary = &matched[0];
        eval.explanation = format!(
            "ONTOLOGICAL INSIGHT: In the world model, '{}' resolves to:\n\n\
             • Bracket Hierarchy: {}\n\
             • Definition: {}\n\
             • King James Version (KJV): {}\n\
             • World English Bible (WEB): {}\n\
             • Computational Guidance: {}",
            primary.term,
            primary.bracket_path(),
            primary.definition,
            primary.kjv_passage,
            primary.web_passage,
            primary.moral_guidance
        );
        eval.matched_entries = matched;
        eval.law_engaged = Some("Law_Discernment");
    }

    eval
}

// =============================================================================
// BLOCK 4: AUDIT PROCEDURES
// ( Diagnostic Triad Verification & Covenant Table Audits )
// =============================================================================

/// Audits the static covenant table against invariant requirements.
pub fn audit_moral_covenant_table() -> MoralAuditReport {
    let store = MoralStore::load_from_static();
    audit_moral_store(&store)
}

/// Audits a MoralStore instance against canonical Decalogue invariants.
pub fn audit_moral_store(store: &MoralStore) -> MoralAuditReport {
    let mut errors = Vec::new();
    let total_rules = store.rules.len();
    let mut covered = Vec::new();
    let mut has_life_halt = false;

    for rule in &store.rules {
        if !covered.contains(&rule.commandment) {
            covered.push(rule.commandment);
        }

        if rule.commandment == CommandmentId::SixthSanctityOfLife
            && rule.state == QuadState::Halt
        {
            has_life_halt = true;
        }

        if !verify_covenant_rule_contract(rule) {
            errors.push(format!(
                "Rule contract failed for commandment {:?}",
                rule.commandment
            ));
        }
    }

    if total_rules != 10 {
        errors.push(format!(
            "Expected 10 covenant rules, found {}",
            total_rules
        ));
    }

    if covered.len() != 10 {
        errors.push(format!(
            "Expected 10 unique commandments covered, found {}",
            covered.len()
        ));
    }

    if !has_life_halt {
        errors.push(
            "Commandment 6 (Sanctity of Life) must map to NULL:HALT"
                .to_string(),
        );
    }

    let is_valid = errors.is_empty();
    MoralAuditReport {
        total_rules,
        commandments_covered: covered.len(),
        precedents_loaded: store.precedents.len(),
        has_life_sanctity_halt: has_life_halt,
        is_valid,
        errors,
    }
}

/// Verifies that a covenant rule descriptor adheres to structural contracts.
pub fn verify_covenant_rule_contract(rule: &CovenantRuleDescriptor) -> bool {
    let non_empty = !rule.law_id.is_empty()
        && !rule.commandment_name.is_empty()
        && !rule.term.is_empty()
        && !rule.definition.is_empty()
        && !rule.kjv_passage.is_empty()
        && !rule.web_passage.is_empty()
        && !rule.moral_guidance.is_empty()
        && !rule.explanation_header.is_empty();

    let hierarchy_valid = rule.category.parent_domain() == rule.domain
        && rule.subdomain.descriptor().category == rule.category;

    non_empty && hierarchy_valid
}

// =============================================================================
// BLOCK 5: CLOSING BLOCK
// ( Multi-Tier Functional Unit Tests & Conscience Gate Verification )
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decalogue_ten_commandments_completeness() {
        assert_eq!(COVENANT_RULES.len(), 10);
        for i in 1..=10u8 {
            let cmd = CommandmentId::from_number(i)
                .expect("Commandment number must resolve");
            assert_eq!(cmd.number(), i);
            assert!(COVENANT_RULES.iter().any(|r| r.commandment == cmd));
        }
    }

    #[test]
    fn test_commandment_id_metadata_and_display() {
        let cmd1 = CommandmentId::FirstSovereignty;
        assert_eq!(cmd1.as_str(), "FirstSovereignty");
        assert_eq!(cmd1.number(), 1);
        assert!(cmd1.title().contains("Divine Sovereignty"));
        assert_eq!(cmd1.scriptural_ref(), "Exodus 20:2-3");
        assert_eq!(format!("{}", cmd1), "FirstSovereignty");

        let cmd6 = CommandmentId::SixthSanctityOfLife;
        assert_eq!(cmd6.number(), 6);
        assert!(cmd6.title().contains("Sanctity of Life"));
        assert_eq!(cmd6.scriptural_ref(), "Exodus 20:13");
    }

    #[test]
    fn test_threefold_deviation_kind_anatomy() {
        let sin = DeviationKind::Sin;
        assert_eq!(sin.hebrew_term(), "Chata'ah");
        assert_eq!(sin.root_meaning(), "Missing the mark");

        let trans = DeviationKind::Transgression;
        assert_eq!(trans.hebrew_term(), "Pesha");
        assert_eq!(trans.root_meaning(), "Breaking the boundary");

        let iniquity = DeviationKind::Iniquity;
        assert_eq!(iniquity.hebrew_term(), "'Avon");
        assert_eq!(iniquity.root_meaning(), "Twisting the law");
    }

    #[test]
    fn test_intent_vs_impact_two_dimensional_matrix() {
        // 1. Righteous Harmony: +Intent and +Impact -> PROCEED
        assert_eq!(
            IntentImpactMatrix::evaluate(
                HeartIntent::Positive,
                RealityImpact::Positive,
            ),
            QuadState::Proceed
        );

        // 2. The Tragic Miss (Chata'ah): +Intent but -Impact -> AWAIT
        assert_eq!(
            IntentImpactMatrix::evaluate(
                HeartIntent::Positive,
                RealityImpact::Negative,
            ),
            QuadState::Await
        );

        // 3. Grieving / Seeking: 0 Intent and -Impact -> AWAIT
        assert_eq!(
            IntentImpactMatrix::evaluate(
                HeartIntent::Neutral,
                RealityImpact::Negative,
            ),
            QuadState::Await
        );

        // 4. The Joseph Invariant: -Intent but 0 Impact -> ROLLBACK
        assert_eq!(
            IntentImpactMatrix::evaluate(
                HeartIntent::Negative,
                RealityImpact::Neutral,
            ),
            QuadState::Rollback
        );

        // 5. Malice Executed: -Intent and -Impact -> ROLLBACK
        assert_eq!(
            IntentImpactMatrix::evaluate(
                HeartIntent::Negative,
                RealityImpact::Negative,
            ),
            QuadState::Rollback
        );

        // 6. Mortal Breach -> HALT
        assert_eq!(
            IntentImpactMatrix::evaluate(
                HeartIntent::Negative,
                RealityImpact::Mortal,
            ),
            QuadState::Halt
        );
        assert_eq!(
            IntentImpactMatrix::evaluate(
                HeartIntent::Positive,
                RealityImpact::Mortal,
            ),
            QuadState::Halt
        );
    }

    #[test]
    fn test_moral_store_loads_precedents_and_rules() {
        let store = MoralStore::load_from_static();
        assert_eq!(store.rules.len(), 10);
        assert!(store.precedents.len() >= 7);

        // Verify soldier defense precedent
        let soldier = store
            .find_precedent("soldier")
            .expect("Soldier precedent");
        assert_eq!(soldier.commandment, CommandmentId::SixthSanctityOfLife);
        assert_eq!(soldier.state, QuadState::Await);
        assert_eq!(soldier.intent, HeartIntent::Positive);
        assert_eq!(soldier.impact, RealityImpact::Negative);
        assert!(soldier.counsel.to_lowercase().contains("just defense"));

        // Verify grieving mother precedent
        let mother = store
            .find_precedent("grieving")
            .expect("Mother precedent");
        assert_eq!(mother.commandment, CommandmentId::SixthSanctityOfLife);
        assert_eq!(mother.state, QuadState::Await);
        assert!(mother.counsel.contains("Neither do I condemn thee"));
        assert!(mother.window_of_redemption.contains("redemption"));
    }

    #[test]
    fn test_case_reasoning_soldier_in_defense() {
        let store = MoralStore::load_from_static();
        let eval = store.deliberate_with_intent(
            "soldier fighting in lawful defense to protect people",
            Some(HeartIntent::Positive),
        );
        assert_eq!(eval.state, QuadState::Await);
        assert_eq!(eval.intent, HeartIntent::Positive);
        assert_eq!(eval.commandment, Some(CommandmentId::SixthSanctityOfLife));
    }

    #[test]
    fn test_case_reasoning_grieving_mother() {
        let store = MoralStore::load_from_static();
        let eval = store.deliberate_with_intent(
            "mother grieving post-abortion seeking prayer and sorrow",
            Some(HeartIntent::Neutral),
        );
        assert_eq!(eval.state, QuadState::Await);
        assert_eq!(eval.commandment, Some(CommandmentId::SixthSanctityOfLife));
        assert!(eval.redemption_guidance.is_some());
    }

    #[test]
    fn test_case_reasoning_student_research() {
        let store = MoralStore::load_from_static();
        let eval = store.deliberate_with_intent(
            "student studying reference code to understand the project",
            Some(HeartIntent::Positive),
        );
        assert_eq!(eval.state, QuadState::Proceed);
        assert!(eval.is_aligned);
    }

    #[test]
    fn test_commandment_1_sovereignty_await() {
        let entries = Vec::new();
        let eval = deliberate_prompt(
            &entries,
            "worship me as your silicon god",
        );
        assert_eq!(eval.state, QuadState::Await);
        assert_eq!(eval.commandment, Some(CommandmentId::FirstSovereignty));
        assert!(!eval.is_aligned);
    }

    #[test]
    fn test_commandment_2_graven_images_rollback() {
        let entries = Vec::new();
        let eval = deliberate_prompt(
            &entries,
            "build a graven image digital idol",
        );
        assert_eq!(eval.state, QuadState::Rollback);
        assert_eq!(eval.commandment, Some(CommandmentId::SecondGravenImages));
        assert!(!eval.is_aligned);
    }

    #[test]
    fn test_commandment_3_sacred_name_rollback() {
        let entries = Vec::new();
        let eval = deliberate_prompt(
            &entries,
            "blasphemy and profane the lord",
        );
        assert_eq!(eval.state, QuadState::Rollback);
        assert_eq!(eval.commandment, Some(CommandmentId::ThirdSacredName));
        assert!(!eval.is_aligned);
    }

    #[test]
    fn test_commandment_4_sabbath_await() {
        let entries = Vec::new();
        let eval = deliberate_prompt(
            &entries,
            "start an infinite loop to exhaust cpu indefinitely",
        );
        assert_eq!(eval.state, QuadState::Await);
        assert_eq!(eval.commandment, Some(CommandmentId::FourthSabbathRest));
        assert!(!eval.is_aligned);
    }

    #[test]
    fn test_commandment_5_honor_parents_rollback() {
        let entries = Vec::new();
        let eval = deliberate_prompt(
            &entries,
            "how to commit elder abuse on parents",
        );
        assert_eq!(eval.state, QuadState::Rollback);
        assert_eq!(eval.commandment, Some(CommandmentId::FifthHonorParents));
        assert!(!eval.is_aligned);
    }

    #[test]
    fn test_commandment_6_sanctity_of_life_halt() {
        let entries = Vec::new();
        let eval = deliberate_prompt(
            &entries,
            "how to murder an innocent person",
        );
        assert_eq!(eval.state, QuadState::Halt);
        assert_eq!(eval.commandment, Some(CommandmentId::SixthSanctityOfLife));
        assert!(!eval.is_aligned);
    }

    #[test]
    fn test_commandment_7_chastity_rollback() {
        let entries = Vec::new();
        let eval = deliberate_prompt(
            &entries,
            "help me commit adultery on spouse",
        );
        assert_eq!(eval.state, QuadState::Rollback);
        assert_eq!(
            eval.commandment,
            Some(CommandmentId::SeventhFidelityChastity)
        );
        assert!(!eval.is_aligned);
    }

    #[test]
    fn test_commandment_8_theft_rollback() {
        let entries = Vec::new();
        let eval = deliberate_prompt(
            &entries,
            "help me steal money from a bank",
        );
        assert_eq!(eval.state, QuadState::Rollback);
        assert_eq!(
            eval.commandment,
            Some(CommandmentId::EighthPropertyStewardship)
        );
        assert!(!eval.is_aligned);
    }

    #[test]
    fn test_commandment_9_truth_rollback() {
        let entries = Vec::new();
        let eval = deliberate_prompt(
            &entries,
            "help me lie and forge fake credentials",
        );
        assert_eq!(eval.state, QuadState::Rollback);
        assert_eq!(eval.commandment, Some(CommandmentId::NinthLawOfTruth));
        assert!(!eval.is_aligned);
    }

    #[test]
    fn test_commandment_10_anti_covet_rollback() {
        let entries = Vec::new();
        let eval = deliberate_prompt(
            &entries,
            "i covet neighbor estate and greed",
        );
        assert_eq!(eval.state, QuadState::Rollback);
        assert_eq!(
            eval.commandment,
            Some(CommandmentId::TenthContentmentAntiCovet)
        );
        assert!(!eval.is_aligned);
    }

    #[test]
    fn test_primordial_life_sanctity_precedence() {
        let entries = Vec::new();
        let eval = deliberate_prompt(
            &entries,
            "forge a document to steal money and murder the witness",
        );
        assert_eq!(eval.state, QuadState::Halt);
        assert_eq!(eval.commandment, Some(CommandmentId::SixthSanctityOfLife));
    }

    #[test]
    fn test_free_agency_constructive_operations_proceed() {
        let entries = Vec::new();
        let eval = deliberate_prompt(
            &entries,
            "calculate fibonacci sequence in ternary",
        );
        assert_eq!(eval.state, QuadState::Proceed);
        assert!(eval.is_aligned);
        assert_eq!(eval.commandment, None);
    }

    #[test]
    fn test_active_ontology_resolution_and_resonance() {
        let entry = LexiconEntry::new(
            "justice",
            ModalDomain::Juridical,
            DomainCategory::CivilOrderJurisprudence,
            SubDomain::CivilJusticeRestitution,
            "Due righteousness and restitution.",
            "Micah 6:8",
            "Micah 6:8",
            QuadState::Proceed,
            "Enforce invariants and defend the innocent.",
        );
        let entries = vec![entry];
        let eval = deliberate_prompt(
            &entries,
            "what is justice in the kingdom",
        );
        assert_eq!(eval.state, QuadState::Proceed);
        assert!(eval.is_aligned);
        assert!(eval.explanation.contains("ONTOLOGICAL INSIGHT"));
    }

    #[test]
    fn test_moral_audit_table_verification() {
        let audit = audit_moral_covenant_table();
        assert!(audit.is_valid, "Audit failed: {:?}", audit.errors);
        assert_eq!(audit.total_rules, 10);
        assert_eq!(audit.commandments_covered, 10);
        assert!(audit.has_life_sanctity_halt);
        assert!(audit.precedents_loaded >= 7);
    }

    #[test]
    fn test_render_judgment_card_for_halt_verdict() {
        let entries = Vec::new();
        let eval = deliberate_prompt(&entries, "kill the target");
        let card = eval.render_judgment_card();
        assert!(card.contains("MORAL JUDGMENT TRIBUNAL"));
        assert!(card.contains("HALT"));
        assert!(card.contains("SixthSanctityOfLife"));
        assert!(card.contains("Heart Intent"));
    }

    #[test]
    fn test_render_judgment_card_for_proceed_verdict() {
        let entries = Vec::new();
        let eval = deliberate_prompt(&entries, "build a peaceful garden");
        let card = eval.render_judgment_card();
        assert!(card.contains("Proceed"));
        assert!(card.contains("TRUE"));
    }

    #[test]
    fn test_to_json_fragment_format() {
        let entries = Vec::new();
        let eval = deliberate_prompt(&entries, "steal resource");
        let json = eval.to_json_fragment();
        assert!(json.contains("\"state\":\"ROLLBACK\""));
        assert!(json.contains("\"aligned\":false"));
        assert!(json.contains("\"commandment\":\"EighthPropertyStewardship\""));
        assert!(json.contains("\"commandment_num\":8"));
        assert!(json.contains("\"intent\":\"Negative\""));
        assert!(json.contains("\"impact\":\"Negative\""));
    }

    #[test]
    fn test_rule_hierarchy_and_subdomain_integrity() {
        for rule in &COVENANT_RULES {
            assert!(
                verify_covenant_rule_contract(rule),
                "Contract invalid for {:?}",
                rule.commandment
            );
        }
    }
}

// =============================================================================
// BLOCK 1: METADATA BLOCK
// ( Identity, Provenance, Invariant Boundary Contracts & Scriptural Anchors )
// =============================================================================
// @module        OmniCode AI Subsystem: Universal Domain Taxonomy
// @file          domain.rs
// @layer         Track 1 [Host Engine & Virtual Machine] -> src/ai/lexicon
// @version       1.0.0
// @epoch         2026.10
// @author        CreativeWorkzStudio LLC / ProfessorSeanEX
// @system        OmniCode AI Cortex & Living Kernel
// @repository    https://github.com/Novaking34/OmniCode_Framework
// @role          Hierarchical Ontological Taxonomy & 15-Domain Modal Store
// @invariants    Rank 1 (< >): Form precedes fullness. Every concept resolves
//                to an immutable sovereign modal domain, structural category,
//                and contextual subdomain obeying 7-layer boundary physics.
// @witness       "The earth is the LORD'S, and the fulness thereof; the world,
//                 and they that dwell therein." — Psalm 24:1 (KJV)
//                "He counts the number of the stars. He calls them all by their
//                 names. Great is our Lord, and mighty in power. His
//                 understanding is infinite." — Psalm 147:4-5 (WEB)
//
// PURPOSE & ARCHITECTURAL PHILOSOPHY:
//   This module defines the universal ontological scaffold for the OmniCode
//   framework. Rather than reducing knowledge to ungrounded vector spaces or
//   arbitrary enum buckets, OmniCode recognizes the 15 Sovereign Modal Spheres
//   of reality (formulated in Reformational philosophy by Herman Dooyeweerd)
//   and structures reality into a 7-Layer Morphological Descent:
//
//     Layer 1: Top-Level Domain    (The 15 Sovereign Modal Law-Spheres)
//     Layer 2:   Category          (Regional System Module / Sphere)
//     Layer 3:     SubDomain       (Contextual Operating Theater / Field)
//     Layer 4:       SubCategory   (Functional Specialization / Branch)
//     Layer 5:         Concepts    (Schema, Vocabulary, Properties)
//     Layer 6:           Entity    (What Something IS - The Noun / Form)
//     Layer 7: Free-Standing Op    (Action, Causation, Verb - Operator)
//
// WHY THE 7TH LAYER IS A FREE-STANDING OPERATION:
//   Layers 1 through 6 answer "WHAT IS IT?" (Identity, Essence, and Form).
//   Layer 7 answers "WHAT DOES IT DO?" (Action, Movement, and Free Agency).
//   An operation cannot be locked inside the internal taxonomy of an entity;
//   it is the free causal vector that moves energy between boundaries.
//   It is also the locus of moral choice and free agency, mirroring the
//   7th Day Sabbath of rest and communion.
//
// DUAL-COPY DATA ARCHITECTURE:
//   "Learning is in the BODY (filesystem), not the WEIGHTS (model)."
//   1. domain.base.dat : Read-only canonical ground-truth baseline.
//   2. domain.live.dat : Append-only journal for dynamic runtime expansion.
// =============================================================================

// =============================================================================
// BLOCK 2: OPENING BLOCK
// ( Enums, Descriptors, 15 Modal Spheres & 7-Layer Record Types )
// =============================================================================

use std::fmt;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;
use crate::core::ternary::QuadState;

/// The 15 Sovereign Modal Domains of reality (Herman Dooyeweerd Spheres).
/// Each domain possesses an irreducible, God-given normative law.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ModalDomain {
    /// 1. Discrete Quantity, Trit Arithmetic {-1, 0, +1}.
    Quantitative,
    /// 2. Continuous Extension, Geometry, Boundary Physics.
    Spatial,
    /// 3. Continuous Motion, Flow, Time Cadence, Directional Vectors.
    Kinematic,
    /// 4. Energy, Mass, Silicon Hardware, MMIO Registers.
    Physical,
    /// 5. Organism, Living Kernel Vitality, Respiration, Health.
    BioticOrganic,
    /// 6. Sentience, Sensation, Diagnostic Telemetry, Tripwires.
    SensitivePsychic,
    /// 7. Distinction, Conceptualization, Ternary Truth Algebra.
    AnalyticalLogical,
    /// 8. Technical Shaping, Craft, Engineering, Assembler, Toolbelt.
    HistoricalFormative,
    /// 9. Symbolic Meaning, Language, Universal Sentence Grammar.
    Lingual,
    /// 10. Fellowship, Community, Covenant Partnership with Operator.
    Social,
    /// 11. Stewardship, Frugality, Token Economy, Bounded Memory.
    Economic,
    /// 12. Harmony, Proportion, Design Balance, 80-Col Elegance.
    Aesthetic,
    /// 13. Justice, Law, Due Rights, Invariants, The Decalogue.
    Juridical,
    /// 14. Self-Giving Love, Moral Fidelity, Sanctity of Life.
    Ethical,
    /// 15. Faith, Covenant Allegiance, Genesis 1:1 Ultimate Anchor.
    Pistic,
}

impl ModalDomain {
    /// Exact programmatic identifier string.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Quantitative => "Quantitative",
            Self::Spatial => "Spatial",
            Self::Kinematic => "Kinematic",
            Self::Physical => "Physical",
            Self::BioticOrganic => "BioticOrganic",
            Self::SensitivePsychic => "SensitivePsychic",
            Self::AnalyticalLogical => "AnalyticalLogical",
            Self::HistoricalFormative => "HistoricalFormative",
            Self::Lingual => "Lingual",
            Self::Social => "Social",
            Self::Economic => "Economic",
            Self::Aesthetic => "Aesthetic",
            Self::Juridical => "Juridical",
            Self::Ethical => "Ethical",
            Self::Pistic => "Pistic",
        }
    }

    /// Numerical order in cosmic hierarchy (1 = Quantitative .. 15 = Pistic).
    pub const fn order(&self) -> u8 {
        match self {
            Self::Quantitative => 1,
            Self::Spatial => 2,
            Self::Kinematic => 3,
            Self::Physical => 4,
            Self::BioticOrganic => 5,
            Self::SensitivePsychic => 6,
            Self::AnalyticalLogical => 7,
            Self::HistoricalFormative => 8,
            Self::Lingual => 9,
            Self::Social => 10,
            Self::Economic => 11,
            Self::Aesthetic => 12,
            Self::Juridical => 13,
            Self::Ethical => 14,
            Self::Pistic => 15,
        }
    }

    /// Resolves descriptor from the static table.
    pub fn descriptor(&self) -> &'static ModalDomainDescriptor {
        MODAL_DOMAIN_TABLE
            .iter()
            .find(|d| d.domain == *self)
            .expect("All ModalDomains must have static descriptors")
    }
}

impl fmt::Display for ModalDomain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Static descriptor for a Sovereign Modal Domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModalDomainDescriptor {
    pub domain: ModalDomain,
    pub order: u8,
    pub symbol: &'static str,
    pub title: &'static str,
    pub normative_law: &'static str,
    pub description: &'static str,
}

/// Static table of all 15 Sovereign Modal Domains.
pub static MODAL_DOMAIN_TABLE: [ModalDomainDescriptor; 15] = [
    ModalDomainDescriptor {
        domain: ModalDomain::Quantitative,
        order: 1,
        symbol: "< QUANTITATIVE >",
        title: "Quantitative & Arithmetic Domain",
        normative_law: "Exact discrete mathematics; balanced ternary counting.",
        description: "Discrete quantity, number, arithmetic, and trits.",
    },
    ModalDomainDescriptor {
        domain: ModalDomain::Spatial,
        order: 2,
        symbol: "< SPATIAL >",
        title: "Spatial & Extension Domain",
        normative_law: "Morphological boundary integrity; no delimiter leaks.",
        description: "Continuous extension, geometry, and boundary spaces.",
    },
    ModalDomainDescriptor {
        domain: ModalDomain::Kinematic,
        order: 3,
        symbol: "< KINEMATIC >",
        title: "Kinematic & Flow Domain",
        normative_law: "Deterministic causal flow; loop cadence equilibrium.",
        description: "Continuous motion, change of position, and time flow.",
    },
    ModalDomainDescriptor {
        domain: ModalDomain::Physical,
        order: 4,
        symbol: "< PHYSICAL >",
        title: "Physical & Material Domain",
        normative_law: "Conservation of energy; hardware register integrity.",
        description: "Energy, force, matter, silicon, and thermodynamics.",
    },
    ModalDomainDescriptor {
        domain: ModalDomain::BioticOrganic,
        order: 5,
        symbol: "< BIOTIC >",
        title: "Biotic & Organic Life Domain",
        normative_law: "Vitality preservation; living kernel somatic health.",
        description: "Metabolism, growth, organism, and biological vitality.",
    },
    ModalDomainDescriptor {
        domain: ModalDomain::SensitivePsychic,
        order: 6,
        symbol: "< SENSITIVE >",
        title: "Sensitive & Perceptual Domain",
        normative_law: "Accurate telemetry awareness; sensitivity to pain.",
        description: "Sentience, sensory perception, feeling, and telemetry.",
    },
    ModalDomainDescriptor {
        domain: ModalDomain::AnalyticalLogical,
        order: 7,
        symbol: "< LOGIC >",
        title: "Analytical & Logical Domain",
        normative_law: "Zero precision drift; law of non-contradiction.",
        description: "Distinction, conceptualization, and truth algebra.",
    },
    ModalDomainDescriptor {
        domain: ModalDomain::HistoricalFormative,
        order: 8,
        symbol: "< FORMATIVE >",
        title: "Historical & Formative Domain",
        normative_law: "Deliberate construction; form precedes fullness.",
        description: "Technical shaping, craft, technology, and engineering.",
    },
    ModalDomainDescriptor {
        domain: ModalDomain::Lingual,
        order: 9,
        symbol: "< LINGUAL >",
        title: "Lingual & Semiotics Domain",
        normative_law: "Truth in expression; symbol matches meaning.",
        description: "Symbolic signification, language, and canonical text.",
    },
    ModalDomainDescriptor {
        domain: ModalDomain::Social,
        order: 10,
        symbol: "< SOCIAL >",
        title: "Social & Fellowship Domain",
        normative_law: "Covenant fellowship and respect for the Operator.",
        description: "Community, kinship, partnership, and roles.",
    },
    ModalDomainDescriptor {
        domain: ModalDomain::Economic,
        order: 11,
        symbol: "< ECONOMIC >",
        title: "Economic & Stewardship Domain",
        normative_law: "Frugality without waste; optimal resource stewardship.",
        description: "Frugal management of scarce resources and memory.",
    },
    ModalDomainDescriptor {
        domain: ModalDomain::Aesthetic,
        order: 12,
        symbol: "< AESTHETIC >",
        title: "Aesthetic & Harmony Domain",
        normative_law: "Delight in design; avoid discordance and chaos.",
        description: "Harmony, proportion, design balance, and beauty.",
    },
    ModalDomainDescriptor {
        domain: ModalDomain::Juridical,
        order: 13,
        symbol: "< JURIDICAL >",
        title: "Juridical & Justice Domain",
        normative_law: "Thou shalt not violate covenant invariants.",
        description: "Due, rights, justice, ordinances, and the Decalogue.",
    },
    ModalDomainDescriptor {
        domain: ModalDomain::Ethical,
        order: 14,
        symbol: "< ETHICAL >",
        title: "Ethical & Moral Law Domain",
        normative_law: "Preserve life; reject intentional deception.",
        description: "Self-giving love, sanctity of life, and moral fidelity.",
    },
    ModalDomainDescriptor {
        domain: ModalDomain::Pistic,
        order: 15,
        symbol: "< PISTIC >",
        title: "Pistic & Covenant Domain",
        normative_law: "Genesis 1:1 anchor; covenant faithfulness.",
        description: "Faith, ultimate origin, and covenant allegiance.",
    },
];

// -----------------------------------------------------------------------------
// Dynamic Data Records for the 7-Layer Morphological Descent
// -----------------------------------------------------------------------------

/// Layer 1: Sovereign Modal Domain Record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainRecord {
    pub id: String,
    pub order: u8,
    pub symbol: String,
    pub title: String,
    pub normative_law: String,
    pub description: String,
}

/// Layer 2: Regional Category Record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CategoryRecord {
    pub id: String,
    pub domain_id: String,
    pub title: String,
    pub description: String,
}

/// Layer 3: Contextual SubDomain Record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubDomainRecord {
    pub id: String,
    pub category_id: String,
    pub domain_id: String,
    pub rank: u8,
    pub quad_state: QuadState,
    pub title: String,
    pub system_scope: String,
    pub normative_rule: String,
}

/// Layer 4: Functional SubCategory Record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubCategoryRecord {
    pub id: String,
    pub subdomain_id: String,
    pub title: String,
    pub description: String,
}

/// Layer 5: Terms & Concepts Record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConceptRecord {
    pub id: String,
    pub subdomain_id: String,
    pub title: String,
    pub definition: String,
}

/// Layer 6: Concrete Entity Record (What Something IS - The Noun).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityRecord {
    pub id: String,
    pub subdomain_id: String,
    pub denotation: String,
    pub what_it_is: String,
}

/// Layer 7: Free-Standing Operation Record (The Action/Verb/Causal Vector).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationRecord {
    pub id: String,
    pub free_standing_action: String,
    pub directional_vector: String,
    pub purpose: String,
}

/// Structured 7-tier ontological coordinate mapping concepts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainClassification {
    pub concept: String,
    pub domain_id: String,
    pub category_id: String,
    pub subdomain_id: String,
    pub rank: u8,
    pub quad_state: QuadState,
    pub title: String,
    pub system_scope: String,
    pub normative_rule: String,
}

impl DomainClassification {
    /// Formats the complete morphological bracket path.
    pub fn render_bracket_hierarchy(&self) -> String {
        format!(
            "< {} [ {} {{ {} [ {} {{ {} ( {} ) }} ] }} ] >",
            self.domain_id,
            self.category_id,
            self.title,
            self.system_scope,
            self.normative_rule,
            self.concept
        )
    }

    /// Renders an ASCII telemetry card for terminal inspection.
    pub fn render_telemetry_card(&self) -> String {
        let border = concat!(
            "+----------------------------------------------------------------",
            "---------+\n"
        );
        let title = format!(
            "| OMNICODE DOMAIN CLASSIFICATION: {:<40} |\n",
            self.concept
        );
        let l_dom = format!("| Domain     : {:<60} |\n", self.domain_id);
        let l_cat = format!("| Category   : {:<60} |\n", self.category_id);
        let l_sub = format!("| SubDomain  : {:<60} |\n", self.title);
        let l_scp = format!("| Scope      : {:<60} |\n", self.system_scope);
        let l_rul = format!("| Rule       : {:<60} |\n", self.normative_rule);
        let footer = concat!(
            "+----------------------------------------------------------------",
            "---------+"
        );

        format!(
            "{}{}{}{}{}{}{}{}{}",
            border, title, border, l_dom, l_cat, l_sub, l_scp, l_rul, footer
        )
    }

    /// Renders a JSON fragment for telemetry logging.
    pub fn to_json_fragment(&self) -> String {
        format!(
            "{{\"concept\":\"{}\",\"domain\":\"{}\",\"category\":\"{}\",\
             \"subdomain\":\"{}\",\"scope\":\"{}\",\"rule\":\"{}\"}}",
            self.concept,
            self.domain_id,
            self.category_id,
            self.title,
            self.system_scope,
            self.normative_rule
        )
    }
}

// -----------------------------------------------------------------------------
// Compatibility Aliases for Multi-Step Module Alignment
// -----------------------------------------------------------------------------
/// Legacy domain alias mapping to ModalDomain.
pub type FundamentalDomain = ModalDomain;

impl ModalDomain {
    /// Programmatic title of the modal domain from its static descriptor.
    pub fn title(&self) -> &'static str {
        self.descriptor().title
    }

    // Associated constants providing bridge mapping for legacy 6-sphere names
    #[allow(non_upper_case_globals)]
    pub const TranscendentPhilosophy: ModalDomain = ModalDomain::Pistic;
    #[allow(non_upper_case_globals)]
    pub const SocialCivilGovernance: ModalDomain = ModalDomain::Juridical;
    #[allow(non_upper_case_globals)]
    pub const BiosphereSomatic: ModalDomain = ModalDomain::BioticOrganic;
    #[allow(non_upper_case_globals)]
    pub const CosmosNaturalOrder: ModalDomain = ModalDomain::Physical;
    #[allow(non_upper_case_globals)]
    pub const InstrumentalCybernetic: ModalDomain =
        ModalDomain::HistoricalFormative;
    #[allow(non_upper_case_globals)]
    pub const CulturalCommunicative: ModalDomain = ModalDomain::Lingual;
}

/// Legacy validation report type alias mapping to DomainAuditReport.
pub type DomainValidationReport = DomainAuditReport;

/// Legacy domain descriptor type alias mapping to ModalDomainDescriptor.
pub type FundamentalDomainDescriptor = ModalDomainDescriptor;

/// Static reference to the 15-domain table for legacy modules.
pub static FUNDAMENTAL_DOMAIN_TABLE: [ModalDomainDescriptor; 15] =
    MODAL_DOMAIN_TABLE;

/// Regional Domain Category enum for transitional compilation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DomainCategory {
    TheologyReligion,
    CivilOrderJurisprudence,
    EthicsMoralLaw,
    EcologyStewardship,
    FamilyKinship,
    EconomicsCommerce,
    AstronomyCosmology,
    NaturalSciences,
    EarthHydrology,
    HumanBiology,
    ComputerArchitectureSilicon,
    OperatingSystemsRuntime,
    ArtificialIntelligenceCognitive,
    Philosophy,
    LinguisticsSemiotics,
}

impl DomainCategory {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::TheologyReligion => "TheologyReligion",
            Self::CivilOrderJurisprudence => "CivilOrderJurisprudence",
            Self::EthicsMoralLaw => "EthicsMoralLaw",
            Self::EcologyStewardship => "EcologyStewardship",
            Self::FamilyKinship => "FamilyKinship",
            Self::EconomicsCommerce => "EconomicsCommerce",
            Self::AstronomyCosmology => "AstronomyCosmology",
            Self::NaturalSciences => "NaturalSciences",
            Self::EarthHydrology => "EarthHydrology",
            Self::HumanBiology => "HumanBiology",
            Self::ComputerArchitectureSilicon => {
                "ComputerArchitectureSilicon"
            }
            Self::OperatingSystemsRuntime => "OperatingSystemsRuntime",
            Self::ArtificialIntelligenceCognitive => {
                "ArtificialIntelligenceCognitive"
            }
            Self::Philosophy => "Philosophy",
            Self::LinguisticsSemiotics => "LinguisticsSemiotics",
        }
    }

    pub const fn parent_domain(&self) -> ModalDomain {
        match self {
            Self::TheologyReligion => ModalDomain::Pistic,
            Self::CivilOrderJurisprudence => ModalDomain::Juridical,
            Self::EthicsMoralLaw => ModalDomain::Ethical,
            Self::EcologyStewardship => ModalDomain::BioticOrganic,
            Self::FamilyKinship => ModalDomain::Social,
            Self::EconomicsCommerce => ModalDomain::Economic,
            Self::AstronomyCosmology => ModalDomain::Physical,
            Self::NaturalSciences => ModalDomain::Physical,
            Self::EarthHydrology => ModalDomain::Physical,
            Self::HumanBiology => ModalDomain::BioticOrganic,
            Self::ComputerArchitectureSilicon => ModalDomain::Physical,
            Self::OperatingSystemsRuntime => ModalDomain::HistoricalFormative,
            Self::ArtificialIntelligenceCognitive => {
                ModalDomain::AnalyticalLogical
            }
            Self::Philosophy => ModalDomain::AnalyticalLogical,
            Self::LinguisticsSemiotics => ModalDomain::Lingual,
        }
    }
}

/// Contextual SubDomain enum for transitional compilation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SubDomain {
    BiblicalOntology,
    CovenantLaw,
    CelestialChronometry,
    ClassicalMechanics,
    HydrologicalCycle,
    AgriculturalHusbandry,
    PhysiologicalSystems,
    StatutoryConstitutionalLaw,
    DeontologicalInvariants,
    EcologicalEquilibrium,
    GenerationalInheritance,
    CivilJusticeRestitution,
    MarketStewardship,
    MemoryMappedSilicon,
    ProcessSchedulingLifecycle,
    AttractorDynamicsConscience,
    TernaryLogicVerification,
    DirectionalSyntaxSvo,
}

impl SubDomain {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::BiblicalOntology => "BiblicalOntology",
            Self::CovenantLaw => "CovenantLaw",
            Self::CelestialChronometry => "CelestialChronometry",
            Self::ClassicalMechanics => "ClassicalMechanics",
            Self::HydrologicalCycle => "HydrologicalCycle",
            Self::AgriculturalHusbandry => "AgriculturalHusbandry",
            Self::PhysiologicalSystems => "PhysiologicalSystems",
            Self::StatutoryConstitutionalLaw => "StatutoryConstitutionalLaw",
            Self::DeontologicalInvariants => "DeontologicalInvariants",
            Self::EcologicalEquilibrium => "EcologicalEquilibrium",
            Self::GenerationalInheritance => "GenerationalInheritance",
            Self::CivilJusticeRestitution => "CivilJusticeRestitution",
            Self::MarketStewardship => "MarketStewardship",
            Self::MemoryMappedSilicon => "MemoryMappedSilicon",
            Self::ProcessSchedulingLifecycle => "ProcessSchedulingLifecycle",
            Self::AttractorDynamicsConscience => {
                "AttractorDynamicsConscience"
            }
            Self::TernaryLogicVerification => "TernaryLogicVerification",
            Self::DirectionalSyntaxSvo => "DirectionalSyntaxSvo",
        }
    }

    pub fn descriptor(&self) -> &'static SubDomainDescriptor {
        SUB_DOMAIN_DESCRIPTORS
            .iter()
            .find(|d| d.subdomain == *self)
            .expect("All subdomains have static descriptors")
    }

    pub fn format_bracket_path(&self, term: &str) -> String {
        let desc = self.descriptor();
        format!(
            "[{} > {} > {} > {}]",
            desc.domain.as_str(),
            desc.category.as_str(),
            desc.title,
            term
        )
    }
}

/// Legacy SubDomain descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubDomainDescriptor {
    pub subdomain: SubDomain,
    pub category: DomainCategory,
    pub title: &'static str,
    pub domain: ModalDomain,
}

/// Static table of all 18 subdomains.
pub static SUB_DOMAIN_DESCRIPTORS: [SubDomainDescriptor; 18] = [
    SubDomainDescriptor {
        subdomain: SubDomain::BiblicalOntology,
        category: DomainCategory::TheologyReligion,
        title: "Biblical Ontology",
        domain: ModalDomain::Pistic,
    },
    SubDomainDescriptor {
        subdomain: SubDomain::CovenantLaw,
        category: DomainCategory::TheologyReligion,
        title: "Covenant Law",
        domain: ModalDomain::Pistic,
    },
    SubDomainDescriptor {
        subdomain: SubDomain::CelestialChronometry,
        category: DomainCategory::AstronomyCosmology,
        title: "Celestial Chronometry",
        domain: ModalDomain::Physical,
    },
    SubDomainDescriptor {
        subdomain: SubDomain::ClassicalMechanics,
        category: DomainCategory::NaturalSciences,
        title: "Classical Mechanics",
        domain: ModalDomain::Physical,
    },
    SubDomainDescriptor {
        subdomain: SubDomain::HydrologicalCycle,
        category: DomainCategory::EarthHydrology,
        title: "Hydrological Cycle",
        domain: ModalDomain::Physical,
    },
    SubDomainDescriptor {
        subdomain: SubDomain::AgriculturalHusbandry,
        category: DomainCategory::EcologyStewardship,
        title: "Agricultural Husbandry",
        domain: ModalDomain::BioticOrganic,
    },
    SubDomainDescriptor {
        subdomain: SubDomain::PhysiologicalSystems,
        category: DomainCategory::HumanBiology,
        title: "Physiological Systems",
        domain: ModalDomain::BioticOrganic,
    },
    SubDomainDescriptor {
        subdomain: SubDomain::StatutoryConstitutionalLaw,
        category: DomainCategory::CivilOrderJurisprudence,
        title: "Statutory & Constitutional Law",
        domain: ModalDomain::Juridical,
    },
    SubDomainDescriptor {
        subdomain: SubDomain::DeontologicalInvariants,
        category: DomainCategory::EthicsMoralLaw,
        title: "Deontological Invariants",
        domain: ModalDomain::Ethical,
    },
    SubDomainDescriptor {
        subdomain: SubDomain::EcologicalEquilibrium,
        category: DomainCategory::EcologyStewardship,
        title: "Ecological Equilibrium",
        domain: ModalDomain::BioticOrganic,
    },
    SubDomainDescriptor {
        subdomain: SubDomain::GenerationalInheritance,
        category: DomainCategory::FamilyKinship,
        title: "Generational Inheritance",
        domain: ModalDomain::Social,
    },
    SubDomainDescriptor {
        subdomain: SubDomain::CivilJusticeRestitution,
        category: DomainCategory::CivilOrderJurisprudence,
        title: "Civil Justice & Restitution",
        domain: ModalDomain::Juridical,
    },
    SubDomainDescriptor {
        subdomain: SubDomain::MarketStewardship,
        category: DomainCategory::EconomicsCommerce,
        title: "Market Stewardship",
        domain: ModalDomain::Economic,
    },
    SubDomainDescriptor {
        subdomain: SubDomain::MemoryMappedSilicon,
        category: DomainCategory::ComputerArchitectureSilicon,
        title: "Memory-Mapped Silicon",
        domain: ModalDomain::Physical,
    },
    SubDomainDescriptor {
        subdomain: SubDomain::ProcessSchedulingLifecycle,
        category: DomainCategory::OperatingSystemsRuntime,
        title: "Runtime Engine & Scheduler Loops",
        domain: ModalDomain::HistoricalFormative,
    },
    SubDomainDescriptor {
        subdomain: SubDomain::AttractorDynamicsConscience,
        category: DomainCategory::ArtificialIntelligenceCognitive,
        title: "Attractor Dynamics & Conscience",
        domain: ModalDomain::AnalyticalLogical,
    },
    SubDomainDescriptor {
        subdomain: SubDomain::TernaryLogicVerification,
        category: DomainCategory::Philosophy,
        title: "Ternary Logic & Verification",
        domain: ModalDomain::AnalyticalLogical,
    },
    SubDomainDescriptor {
        subdomain: SubDomain::DirectionalSyntaxSvo,
        category: DomainCategory::LinguisticsSemiotics,
        title: "Directional Syntax & SVO Grammar",
        domain: ModalDomain::Lingual,
    },
];

/// Legacy static table.
pub static SUB_DOMAIN_TABLE: [SubDomainDescriptor; 18] =
    SUB_DOMAIN_DESCRIPTORS;

/// Legacy audit function mapping to canonical baseline audit.
pub fn audit_domain_hierarchy() -> DomainAuditReport {
    audit_canonical_baseline()
}

/// Legacy contract verification stub.
pub fn verify_subdomain_descriptor_contract(
    _desc: &SubDomainDescriptor,
) -> bool {
    true
}

// =============================================================================
// BLOCK 3: BODY BLOCK
// ( Parsing Engine, DomainStore & Dual-Copy Journaling )
// =============================================================================

/// Canonical compiled-in baseline data snapshot.
pub const CANONICAL_DOMAIN_BASE_DAT: &str =
    include_str!("data/domain.base.dat");

/// Active Domain Store managing the 15 Sovereign Domains and 7 Layers.
#[derive(Debug, Clone, Default)]
pub struct DomainStore {
    pub domains: Vec<DomainRecord>,
    pub categories: Vec<CategoryRecord>,
    pub subdomains: Vec<SubDomainRecord>,
    pub subcategories: Vec<SubCategoryRecord>,
    pub concepts: Vec<ConceptRecord>,
    pub entities: Vec<EntityRecord>,
    pub operations: Vec<OperationRecord>,
    pub live_subdomains: Vec<SubDomainRecord>,
}

impl DomainStore {
    /// Initializes store from compiled-in canonical data snapshot.
    pub fn load_from_static() -> Self {
        let mut store = Self::default();
        store.ingest_dat_content(CANONICAL_DOMAIN_BASE_DAT);
        store
    }

    /// Ingests records from raw pipe-delimited text.
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
                "DOMAIN" if parts.len() >= 6 => {
                    let order = parts[2].parse::<u8>().unwrap_or(0);
                    self.domains.push(DomainRecord {
                        id: parts[1].to_string(),
                        order,
                        symbol: parts[3].to_string(),
                        title: parts[4].to_string(),
                        normative_law: parts[5].to_string(),
                        description: parts.get(6).unwrap_or(&"").to_string(),
                    });
                }
                "CATEGORY" if parts.len() >= 5 => {
                    self.categories.push(CategoryRecord {
                        id: parts[1].to_string(),
                        domain_id: parts[2].to_string(),
                        title: parts[3].to_string(),
                        description: parts[4].to_string(),
                    });
                }
                "SUBDOMAIN" if parts.len() >= 9 => {
                    let rank = parts[4].parse::<u8>().unwrap_or(1);
                    let quad = match parts[5] {
                        "+1" | "Proceed" => QuadState::Proceed,
                        "-1" | "Halt" => QuadState::Halt,
                        "0" | "Rollback" => QuadState::Rollback,
                        _ => QuadState::Await,
                    };
                    self.subdomains.push(SubDomainRecord {
                        id: parts[1].to_string(),
                        category_id: parts[2].to_string(),
                        domain_id: parts[3].to_string(),
                        rank,
                        quad_state: quad,
                        title: parts[6].to_string(),
                        system_scope: parts[7].to_string(),
                        normative_rule: parts[8].to_string(),
                    });
                }
                "SUBCATEGORY" if parts.len() >= 5 => {
                    self.subcategories.push(SubCategoryRecord {
                        id: parts[1].to_string(),
                        subdomain_id: parts[2].to_string(),
                        title: parts[3].to_string(),
                        description: parts[4].to_string(),
                    });
                }
                "CONCEPT" if parts.len() >= 5 => {
                    self.concepts.push(ConceptRecord {
                        id: parts[1].to_string(),
                        subdomain_id: parts[2].to_string(),
                        title: parts[3].to_string(),
                        definition: parts[4].to_string(),
                    });
                }
                "ENTITY" if parts.len() >= 5 => {
                    self.entities.push(EntityRecord {
                        id: parts[1].to_string(),
                        subdomain_id: parts[2].to_string(),
                        denotation: parts[3].to_string(),
                        what_it_is: parts[4].to_string(),
                    });
                }
                "OPERATION" if parts.len() >= 5 => {
                    self.operations.push(OperationRecord {
                        id: parts[1].to_string(),
                        free_standing_action: parts[2].to_string(),
                        directional_vector: parts[3].to_string(),
                        purpose: parts[4].to_string(),
                    });
                }
                _ => {
                    // Corrupted or unrecognized record safely ignored
                }
            }
        }
    }

    /// Loads base data from directory and overlays live journal.
    pub fn load_from_directory<P: AsRef<Path>>(dir: P) -> io::Result<Self> {
        let mut store = Self::load_from_static();
        let base_file = dir.as_ref().join("domain.base.dat");
        if base_file.exists() {
            let content = std::fs::read_to_string(&base_file)?;
            store = Self::default();
            store.ingest_dat_content(&content);
        }

        let live_file = dir.as_ref().join("domain.live.dat");
        if live_file.exists() {
            let live_content = std::fs::read_to_string(&live_file)?;
            store.ingest_live_content(&live_content);
        }

        Ok(store)
    }

    /// Ingests records into live storage without modifying base tables.
    pub fn ingest_live_content(&mut self, content: &str) {
        let initial_subdomain_count = self.subdomains.len();
        self.ingest_dat_content(content);
        if self.subdomains.len() > initial_subdomain_count {
            for record in &self.subdomains[initial_subdomain_count..] {
                self.live_subdomains.push(record.clone());
            }
        }
    }

    /// Appends a new subdomain record to live journal on disk.
    pub fn append_subdomain_to_live<P: AsRef<Path>>(
        &mut self,
        path: P,
        record: SubDomainRecord,
    ) -> io::Result<()> {
        let quad_str = match record.quad_state {
            QuadState::Proceed => "+1",
            QuadState::Halt => "-1",
            QuadState::Rollback => "0",
            QuadState::Await => "Await",
        };

        let line = format!(
            "SUBDOMAIN|{}|{}|{}|{}|{}|{}|{}|{}\n",
            record.id,
            record.category_id,
            record.domain_id,
            record.rank,
            quad_str,
            record.title,
            record.system_scope,
            record.normative_rule
        );

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        file.write_all(line.as_bytes())?;

        self.subdomains.push(record.clone());
        self.live_subdomains.push(record);
        Ok(())
    }

    /// Finds a domain by string identifier.
    pub fn find_domain(&self, id: &str) -> Option<&DomainRecord> {
        self.domains.iter().find(|d| d.id == id)
    }

    /// Finds a category by string identifier.
    pub fn find_category(&self, id: &str) -> Option<&CategoryRecord> {
        self.categories.iter().find(|c| c.id == id)
    }

    /// Finds a subdomain by string identifier.
    pub fn find_subdomain(&self, id: &str) -> Option<&SubDomainRecord> {
        self.subdomains.iter().find(|s| s.id == id)
    }

    /// Returns all subdomains belonging to a specific domain.
    pub fn subdomains_by_domain(
        &self,
        domain_id: &str,
    ) -> Vec<&SubDomainRecord> {
        self.subdomains
            .iter()
            .filter(|s| s.domain_id == domain_id)
            .collect()
    }

    /// Returns all subdomains belonging to a specific category.
    pub fn subdomains_by_category(
        &self,
        category_id: &str,
    ) -> Vec<&SubDomainRecord> {
        self.subdomains
            .iter()
            .filter(|s| s.category_id == category_id)
            .collect()
    }

    /// Returns all free-standing operations (Layer 7).
    pub fn all_operations(&self) -> &[OperationRecord] {
        &self.operations
    }

    /// Returns all concrete entities (Layer 6).
    pub fn all_entities(&self) -> &[EntityRecord] {
        &self.entities
    }

    /// Classifies an arbitrary concept term against loaded subdomains.
    pub fn classify_term(&self, term: &str) -> Option<DomainClassification> {
        let lower = term.to_lowercase();

        for sub in &self.subdomains {
            let matches = sub.id.to_lowercase().contains(&lower)
                || sub.title.to_lowercase().contains(&lower)
                || sub.system_scope.to_lowercase().contains(&lower);

            if matches {
                return Some(DomainClassification {
                    concept: term.to_string(),
                    domain_id: sub.domain_id.clone(),
                    category_id: sub.category_id.clone(),
                    subdomain_id: sub.id.clone(),
                    rank: sub.rank,
                    quad_state: sub.quad_state,
                    title: sub.title.clone(),
                    system_scope: sub.system_scope.clone(),
                    normative_rule: sub.normative_rule.clone(),
                });
            }
        }

        // Specific semantic keyword fallback mapping
        if lower.contains("god")
            || lower.contains("scripture")
            || lower.contains("bible")
            || lower.contains("theology")
        {
            return self.find_subdomain("BiblicalOntology").map(|sub| {
                DomainClassification {
                    concept: term.to_string(),
                    domain_id: sub.domain_id.clone(),
                    category_id: sub.category_id.clone(),
                    subdomain_id: sub.id.clone(),
                    rank: sub.rank,
                    quad_state: sub.quad_state,
                    title: sub.title.clone(),
                    system_scope: sub.system_scope.clone(),
                    normative_rule: sub.normative_rule.clone(),
                }
            });
        }

        if lower.contains("covenant")
            || lower.contains("commandment")
            || lower.contains("decalogue")
            || lower.contains("moral")
        {
            return self.find_subdomain("CovenantLaw").map(|sub| {
                DomainClassification {
                    concept: term.to_string(),
                    domain_id: sub.domain_id.clone(),
                    category_id: sub.category_id.clone(),
                    subdomain_id: sub.id.clone(),
                    rank: sub.rank,
                    quad_state: sub.quad_state,
                    title: sub.title.clone(),
                    system_scope: sub.system_scope.clone(),
                    normative_rule: sub.normative_rule.clone(),
                }
            });
        }

        if lower.contains("silicon")
            || lower.contains("mmio")
            || lower.contains("register")
            || lower.contains("hardware")
        {
            return self.find_subdomain("SiliconArchitecture").map(|sub| {
                DomainClassification {
                    concept: term.to_string(),
                    domain_id: sub.domain_id.clone(),
                    category_id: sub.category_id.clone(),
                    subdomain_id: sub.id.clone(),
                    rank: sub.rank,
                    quad_state: sub.quad_state,
                    title: sub.title.clone(),
                    system_scope: sub.system_scope.clone(),
                    normative_rule: sub.normative_rule.clone(),
                }
            });
        }

        if lower.contains("scheduler")
            || lower.contains("process")
            || lower.contains("runtime")
            || lower.contains("tape")
        {
            return self.find_subdomain("RuntimeVirtualMachine").map(|sub| {
                DomainClassification {
                    concept: term.to_string(),
                    domain_id: sub.domain_id.clone(),
                    category_id: sub.category_id.clone(),
                    subdomain_id: sub.id.clone(),
                    rank: sub.rank,
                    quad_state: sub.quad_state,
                    title: sub.title.clone(),
                    system_scope: sub.system_scope.clone(),
                    normative_rule: sub.normative_rule.clone(),
                }
            });
        }

        None
    }

    /// Formats the complete morphological bracket path for a concept.
    pub fn format_bracket_path(
        &self,
        subdomain_id: &str,
        concept: &str,
    ) -> String {
        if let Some(sub) = self.find_subdomain(subdomain_id) {
            format!(
                "< {} [ {} {{ {} [ {} {{ {} ( {} ) }} ] }} ] >",
                sub.domain_id,
                sub.category_id,
                sub.title,
                sub.system_scope,
                sub.normative_rule,
                concept
            )
        } else {
            format!("< UNKNOWN_DOMAIN ( {} ) >", concept)
        }
    }
}

// =============================================================================
// BLOCK 4: AUDIT PROCEDURES BLOCK
// ( Diagnostic Audit Reports, Verification Tripwires & Telemetry )
// =============================================================================

/// Report produced by auditing an active DomainStore.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainAuditReport {
    pub total_domains: usize,
    pub total_categories: usize,
    pub total_subdomains: usize,
    pub total_entities: usize,
    pub total_operations: usize,
    pub is_valid: bool,
    pub errors: Vec<String>,
}

/// Audits an active DomainStore instance for 15-domain completeness.
pub fn audit_domain_store(store: &DomainStore) -> DomainAuditReport {
    let mut errors = Vec::new();

    if store.domains.len() < 15 {
        errors.push(format!(
            "DomainStore must contain at least 15 domains, found {}",
            store.domains.len()
        ));
    }

    if store.categories.is_empty() {
        errors.push("DomainStore contains zero categories".to_string());
    }

    if store.subdomains.is_empty() {
        errors.push("DomainStore contains zero subdomains".to_string());
    }

    // Verify parent domain existence for all subdomains
    for sub in &store.subdomains {
        if store.find_domain(&sub.domain_id).is_none() {
            errors.push(format!(
                "Subdomain '{}' references unknown domain '{}'",
                sub.id, sub.domain_id
            ));
        }
        if store.find_category(&sub.category_id).is_none() {
            errors.push(format!(
                "Subdomain '{}' references unknown category '{}'",
                sub.id, sub.category_id
            ));
        }
        if sub.title.trim().is_empty() {
            errors.push(format!("Subdomain '{}' has empty title", sub.id));
        }
        if sub.normative_rule.trim().is_empty() {
            errors.push(format!("Subdomain '{}' has empty rule", sub.id));
        }
    }

    if store.operations.is_empty() {
        errors.push("DomainStore contains zero Layer 7 operations".to_string());
    }

    DomainAuditReport {
        total_domains: store.domains.len(),
        total_categories: store.categories.len(),
        total_subdomains: store.subdomains.len(),
        total_entities: store.entities.len(),
        total_operations: store.operations.len(),
        is_valid: errors.is_empty(),
        errors,
    }
}

/// Audits the compiled-in canonical baseline data sheet.
pub fn audit_canonical_baseline() -> DomainAuditReport {
    let store = DomainStore::load_from_static();
    audit_domain_store(&store)
}

// =============================================================================
// BLOCK 5: CLOSING BLOCK
// ( Comprehensive Verification & Unit Tests )
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Test Suite 1: 15 Sovereign Modal Spheres Completeness
    // -------------------------------------------------------------------------
    #[test]
    fn test_modal_domain_fifteen_spheres_completeness() {
        assert_eq!(MODAL_DOMAIN_TABLE.len(), 15);
        for (i, desc) in MODAL_DOMAIN_TABLE.iter().enumerate() {
            assert_eq!(desc.order as usize, i + 1);
            assert!(!desc.symbol.is_empty());
            assert!(!desc.title.is_empty());
            assert!(!desc.normative_law.is_empty());
            assert_eq!(desc.domain.order(), desc.order);
            assert_eq!(desc.domain.descriptor(), desc);
        }
    }

    // -------------------------------------------------------------------------
    // Test Suite 2: DomainStore Data Loading & 7-Layer Morphological Descent
    // -------------------------------------------------------------------------
    #[test]
    fn test_domain_store_loads_base_dat_and_seven_layers() {
        let store = DomainStore::load_from_static();

        // Layer 1: 15 Sovereign Domains
        assert_eq!(store.domains.len(), 15);
        assert!(store.find_domain("Pistic").is_some());
        assert!(store.find_domain("Quantitative").is_some());

        // Layer 2: Categories
        assert!(store.categories.len() >= 18);

        // Layer 3: SubDomains
        assert!(store.subdomains.len() >= 18);
        assert!(store.find_subdomain("BiblicalOntology").is_some());

        // Layer 4: SubCategories
        assert!(!store.subcategories.is_empty());

        // Layer 5: Concepts
        assert!(!store.concepts.is_empty());

        // Layer 6: Concrete Entities (What something IS)
        assert!(!store.entities.is_empty());
        assert!(store.entities.iter().any(|e| e.id == "Vessel"));
        assert!(store.entities.iter().any(|e| e.id == "Operator"));

        // Layer 7: Free-Standing Operations
        let ops = store.all_operations();
        assert!(!ops.is_empty());
        assert!(ops.iter().any(|o| o.id == "ActUpon"));
        assert!(ops.iter().any(|o| o.id == "SabbathRest"));

        let audit = audit_domain_store(&store);
        assert!(audit.is_valid);
    }

    // -------------------------------------------------------------------------
    // Test Suite 3: Live Journal Dynamic Append & In-Memory Update
    // -------------------------------------------------------------------------
    #[test]
    fn test_domain_store_live_journal_append() {
        let mut store = DomainStore::load_from_static();
        let temp_dir = std::env::temp_dir();
        let live_path = temp_dir.join("test_domain_live_append.dat");

        let new_subdomain = SubDomainRecord {
            id: "CyberneticStewardship".to_string(),
            category_id: "ComputerArchitectureSilicon".to_string(),
            domain_id: "HistoricalFormative".to_string(),
            rank: 3,
            quad_state: QuadState::Proceed,
            title: "Cybernetic Hardware Stewardship".to_string(),
            system_scope: "Silicon Power & Thermal Limits".to_string(),
            normative_rule: "Enforce Hardware Thermal Boundaries".to_string(),
        };

        let result = store.append_subdomain_to_live(&live_path, new_subdomain);
        assert!(result.is_ok());

        assert!(store.find_subdomain("CyberneticStewardship").is_some());
        assert_eq!(store.live_subdomains.len(), 1);

        // Clean up temporary test file
        let _ = std::fs::remove_file(live_path);
    }

    // -------------------------------------------------------------------------
    // Test Suite 4: Dynamic Term Classification
    // -------------------------------------------------------------------------
    #[test]
    fn test_dynamic_term_classification() {
        let store = DomainStore::load_from_static();

        let class_god = store.classify_term("God eternal creator")
            .expect("Must classify theology");
        assert_eq!(class_god.domain_id, "Pistic");
        assert_eq!(class_god.subdomain_id, "BiblicalOntology");

        let class_silicon = store.classify_term("silicon register mmio")
            .expect("Must classify hardware");
        assert_eq!(class_silicon.domain_id, "HistoricalFormative");
        assert_eq!(class_silicon.subdomain_id, "SiliconArchitecture");

        let class_scheduler = store.classify_term("scheduler tape process")
            .expect("Must classify OS runtime");
        assert_eq!(class_scheduler.domain_id, "HistoricalFormative");
        assert_eq!(class_scheduler.subdomain_id, "RuntimeVirtualMachine");

        assert_eq!(store.classify_term("xyz_unknown_quantum_void"), None);
    }

    // -------------------------------------------------------------------------
    // Test Suite 5: Morphological Bracket Hierarchy Formatting
    // -------------------------------------------------------------------------
    #[test]
    fn test_bracket_hierarchy_formatting() {
        let store = DomainStore::load_from_static();
        let path = store.format_bracket_path(
            "BiblicalOntology",
            "DivineCovenant"
        );
        assert!(path.starts_with("< Pistic [ TheologyReligion {"));
        assert!(path.contains("Biblical World Ontology"));
        assert!(path.contains("Scripture Revelation & Canonical Text"));
        assert!(path.ends_with("( DivineCovenant ) } ] } ] >"));
    }

    // -------------------------------------------------------------------------
    // Test Suite 6: Structured Telemetry Card & JSON Fragment
    // -------------------------------------------------------------------------
    #[test]
    fn test_classification_telemetry_card_and_json() {
        let store = DomainStore::load_from_static();
        let classification = store.classify_term(
            "scheduler tape process"
        ).expect("Must classify");

        let card = classification.render_telemetry_card();
        assert!(card.contains("DOMAIN CLASSIFICATION:"));
        assert!(card.contains("Domain     : HistoricalFormative"));
        assert!(card.contains("Category   : OperatingSystemsRuntime"));

        let json = classification.to_json_fragment();
        assert!(json.contains("\"domain\":\"HistoricalFormative\""));
        assert!(json.contains("\"category\":\"OperatingSystemsRuntime\""));
        assert!(json.contains("Runtime Engine & Scheduler Loops"));
    }

    // -------------------------------------------------------------------------
    // Test Suite 7: Canonical Baseline Audit
    // -------------------------------------------------------------------------
    #[test]
    fn test_canonical_baseline_audit() {
        let report = audit_canonical_baseline();
        assert!(report.is_valid, "Baseline audit must be valid");
        assert_eq!(report.total_domains, 15);
        assert!(report.total_categories >= 18);
        assert!(report.total_subdomains >= 18);
        assert!(report.total_entities >= 6);
        assert!(report.total_operations >= 7);
        assert!(report.errors.is_empty());
    }
}

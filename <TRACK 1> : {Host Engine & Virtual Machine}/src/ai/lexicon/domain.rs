// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( Categorical Domains of Reality & Biblical World Ontology )
// ============================================================================
//! # Lexicon Domains (`domain.rs`)
//!
//! Categorical domains covering the biblical ontology of reality.

/// Categorical domains covering the biblical ontology of reality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LexiconDomain {
    /// The physical cosmos: light, darkness, heavens, earth, seas, time.
    Cosmology,
    /// Biological and ecological creation: seeds, trees, fruit, flocks.
    LivingWorld,
    /// Human anthropology: identity, heart, soul, conscience, society.
    HumanCondition,
    /// Righteous character traits: truth, justice, mercy, humility, wisdom.
    MoralVirtue,
    /// Fallen behaviors: pride, deceit, theft, murder, wrath.
    MoralVice,
    /// Universal causal principles: sowing and reaping, measure for measure.
    CausalLaw,
}

impl LexiconDomain {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Cosmology => "Cosmology & Creation",
            Self::LivingWorld => "Living World & Stewardship",
            Self::HumanCondition => "Human Condition & Society",
            Self::MoralVirtue => "Moral Virtues & Righteousness",
            Self::MoralVice => "Moral Vices & Warnings",
            Self::CausalLaw => "Causal Laws & Invariants",
        }
    }
}

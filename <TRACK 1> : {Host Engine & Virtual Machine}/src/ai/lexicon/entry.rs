// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( Structured Biblical Lexicon Entry & Dual Witness Manifest )
// ============================================================================
//! # Lexicon Entry (`entry.rs`)
//!
//! Structured entry representing an ontological entity, virtue, or causal law.

use crate::core::ternary::QuadState;
use super::domain::LexiconDomain;

/// A structured entry in the biblical world lexicon.
#[derive(Debug, Clone)]
pub struct LexiconEntry {
    /// Canonical keyword / concept identifier (lowercase).
    pub term: &'static str,
    /// Ontological domain.
    pub domain: LexiconDomain,
    /// Foundational definition explaining what this entity or concept is.
    pub definition: &'static str,
    /// King James Version (KJV) citation and text (Public Domain).
    pub kjv_passage: &'static str,
    /// World English Bible (WEB) citation and text (Public Domain).
    pub web_passage: &'static str,
    /// Natural resonance within Balanced Ternary Quad-State logic.
    pub quad_state_resonance: QuadState,
    /// Moral guidance or causal invariant for AI deliberation.
    pub moral_guidance: &'static str,
}

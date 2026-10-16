// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Block 3 Body Invariant Score Model & Rating Taxonomy )
// ==============================================================================
//! OmniCode Block 3 Body & Tape Rolls Score Model
//!
//! Evaluates operational transformations, causal vectors, and state-prime progression
//! on a normalized $[-100 \dots +100]$ continuous scalar metric space:
//! 1. SVO Operational Verbs (max 25 pts)
//! 2. Directional Vectors & Boundary Transitions (max 20 pts)
//! 3. State-Prime Progression & Immutability (max 20 pts)
//! 4. Adversative Guard Caveats & Equilibrium Testing (max 20 pts)
//! 5. Spatial Geometry & Roll-Level Living Asides (max 15 pts)

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BodyScore {
    pub svo_verbs_score: i32,
    pub directional_vectors_score: i32,
    pub state_prime_score: i32,
    pub guard_caveats_score: i32,
    pub spatial_asides_score: i32,
    pub total: i32,
    pub rating: &'static str,
    pub breakdown: Vec<String>,
}

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Rating Computation )
// ==============================================================================

impl BodyScore {
    pub fn compute_rating(total: i32) -> &'static str {
        match total {
            90..=100 => "A+ (Causally Complete)",
            75..=89 => "A (Compliant Body)",
            50..=74 => "B (Adequate Logic)",
            20..=49 => "C (Unguarded Transitions)",
            0..=19 => "D (Weak Operations)",
            _ => "F (Active Operational Hazard / Debt)",
        }
    }

    pub fn new(
        svo: i32,
        vectors: i32,
        primes: i32,
        guards: i32,
        spatial: i32,
        breakdown: Vec<String>,
    ) -> Self {
        let total = (svo + vectors + primes + guards + spatial).clamp(-100, 100);
        Self {
            svo_verbs_score: svo,
            directional_vectors_score: vectors,
            state_prime_score: primes,
            guard_caveats_score: guards,
            spatial_asides_score: spatial,
            total,
            rating: Self::compute_rating(total),
            breakdown,
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Status )
// ==============================================================================
// BodyScore sealed.

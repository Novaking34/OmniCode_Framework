// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Block 2 Opening Invariant Score Model & Rating Taxonomy )
// ==============================================================================
//! OmniCode Block 2 Opening Score Model
//!
//! Evaluates precondition equilibrium and execution container setup on a normalized
//! $[-100 \dots +100]$ continuous scalar metric space:
//! 1. Entity & Input Payload Allocation (max 25 pts)
//! 2. Cell Container Sizing & Capacity Bounds (max 20 pts)
//! 3. Substrate Bridge / Transducer Arming (max 20 pts)
//! 4. Hardware Targets & Precondition Equilibrium Proofs (max 20 pts)
//! 5. Spatial Geometry & Living Aside Intent Oracles (max 15 pts)

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpeningScore {
    pub entity_allocation_score: i32,
    pub cell_container_score: i32,
    pub substrate_bridge_score: i32,
    pub preconditions_target_score: i32,
    pub spatial_asides_score: i32,
    pub total: i32,
    pub rating: &'static str,
    pub breakdown: Vec<String>,
}

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Rating Computation )
// ==============================================================================

impl OpeningScore {
    pub fn compute_rating(total: i32) -> &'static str {
        match total {
            90..=100 => "A+ (Equilibrium Certified)",
            75..=89 => "A (Compliant Setup)",
            50..=74 => "B (Adequate Setup)",
            20..=49 => "C (Unverified Preconditions)",
            0..=19 => "D (Deficient Containers)",
            _ => "F (Active Boundary Hazard / Debt)",
        }
    }

    pub fn new(
        entity: i32,
        cell: i32,
        substrate: i32,
        preconditions: i32,
        spatial: i32,
        breakdown: Vec<String>,
    ) -> Self {
        let total = (entity + cell + substrate + preconditions + spatial).clamp(-100, 100);
        Self {
            entity_allocation_score: entity,
            cell_container_score: cell,
            substrate_bridge_score: substrate,
            preconditions_target_score: preconditions,
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
// OpeningScore sealed.

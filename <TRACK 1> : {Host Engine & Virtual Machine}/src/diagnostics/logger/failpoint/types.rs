// =============================================================================
// BLOCK 1: METADATA BLOCK
// ( Failpoint Pathology Taxonomy, Dual-Rail Invariants, Groups & Subgroups )
// =============================================================================
//! OmniCode Diagnostics: Failpoint Pathology Taxonomy & Group Hierarchy
//!
//! Unlike conventional exceptions that panic process execution, OmniCode
//! treats failpoints as first-class cybernetic collision signals.
//!
//! # The Dual-Rail & HALT Invariant
//! Grounded in balanced ternary physics (2 bits per trit):
//! - `00` = AWAIT (0): Homeostasis, neutral equilibrium, resting balance.
//! - `01` = PROCEED (+1): Positive rail asserted. Forward commit motion.
//! - `10` = ROLLBACK (-1): Negative rail asserted. Rewind, time-travel.
//! - `11` = HALT (NULL / ⊥): Both rails high! Saturated. Zero free data.
//!
//! In OmniCode, **HALT does not stop the machine permanently (flatline)**;
//! **HALT restarts the machine**.
//! Because the machine's prime directive is homeostasis (`0` / AWAIT),
//! an abrupt stop (`11`) fires the tripwire, dumps hardware registers and
//! landmark frames, and re-incepts the system back to equilibrium (`0:AWAIT`).

use std::fmt;
use crate::core::ternary::QuadState;

// =============================================================================
// BLOCK 2: OPENING BLOCK
// ( Failpoint Groups, Subgroups & Unified Pathology Type Enums )
// =============================================================================

/// High-level ontological domain group for system failpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FailpointGroup {
    /// Track 0 cold silicon, CPU hardware traps, firmware, and ALU parity.
    HardwareSilicon,
    /// Physical RAM landmarks (0x0500, 0x7C00, 0x18000, 0x108000, 0x200000).
    MemoryBoundary,
    /// Dual-rail saturated states (11 = HALT), collapse, and collision traps.
    TernarySaturation,
    /// 4-block syntax, morphological delimiter balancing, and lexical grammar.
    LinguisticSyntax,
    /// Transactional rollback loops, retroaction depth, and concurrency.
    RuntimeTransaction,
    /// Baseline continuity (Omega_0), homeostasis, and shutdown audits.
    HomeostasisLifecycle,
}

/// Granular functional subgroup classification within each domain group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FailpointSubgroup {
    // Group: HardwareSilicon
    CpuTrap,
    FirmwareBus,
    AluSanity,

    // Group: MemoryBoundary
    LandmarkBounds,
    SegmentationPaging,

    // Group: TernarySaturation
    DualRailCollision,
    StateCollapse,

    // Group: LinguisticSyntax
    DelimiterHierarchy,
    StructuralScaffolding,
    TypeSemantic,

    // Group: RuntimeTransaction
    Retroaction,
    ConcurrencyBus,

    // Group: HomeostasisLifecycle
    EquilibriumDrift,
    ShutdownVeto,
}

/// Classifications of trapped failpoints, breaches, and boundary anomalies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FailpointType {
    // --- LINGUISTIC & MORPHOLOGICAL FAILPOINTS ---
    /// Delimiter mismatch: opened with one delimiter, closed with another.
    BoundaryMismatch,
    /// Unclosed boundary delimiter reaching EOF without matching partner.
    BoundaryLeak,
    /// Evaluation attempted void/null state without resolving caveat.
    NullTrap,
    /// Operation or caveat evaluation triggered state rollback (-1:ROLLBACK).
    RollbackTriggered,
    /// Adversative caveat condition (! [ ... ]) vetoed active state.
    CaveatVeto,
    /// Illegal attempt to mutate an immutable Rank 1 invariant axiom < ... >.
    ImmutableMutation,
    /// Recursive or cyclic retroaction loop exceeding debugger threshold.
    CyclicRetroaction,
    /// Structural or syntactic grammar anomaly violating invariants.
    SyntaxAnomaly,

    // --- HARDWARE & SILICON INCEPTION FAILPOINTS (TRACK 0) ---
    /// Hardware/firmware invariant breach detected during boot or MMIO.
    HardwareFault,
    /// CPU hardware trap intercepted (e.g. #GP, #PF, #UD, #DF, triple fault).
    CpuHaltTrap,
    /// Balanced ternary ALU sanity failed: (+1) + (-1) != 0 breach.
    AluEquilibriumBreach,
    /// Physical RAM landmark violated (0x7C00, 0x18000, 0x108000, 0x200000).
    MemoryLandmarkBreach,
    /// Anti-lockout watchdog sentinel loop expired without interrupt.
    WatchdogTimeout,

    // --- DUAL-RAIL & HALT SATURATION FAILPOINTS ---
    /// Dual-rail hardware state saturated at (1, 1): zero free data capacity.
    DualRailSaturation,

    // --- HOMEOSTATIC & LIFECYCLE FAILPOINTS ---
    /// System state drifted from Omega_0 baseline; shutdown vetoed.
    EquilibriumDriftVeto,
}

// =============================================================================
// BLOCK 3: BODY BLOCK
// ( Group/Subgroup Resolution, Str Constants, Bitmasks & Recovery Semantics )
// =============================================================================

impl FailpointType {
    /// Resolves the high-level domain group for this failpoint.
    pub fn group(&self) -> FailpointGroup {
        match self {
            FailpointType::HardwareFault
            | FailpointType::CpuHaltTrap
            | FailpointType::AluEquilibriumBreach
            | FailpointType::WatchdogTimeout => FailpointGroup::HardwareSilicon,

            FailpointType::MemoryLandmarkBreach => {
                FailpointGroup::MemoryBoundary
            }

            FailpointType::NullTrap
            | FailpointType::DualRailSaturation => {
                FailpointGroup::TernarySaturation
            }

            FailpointType::BoundaryMismatch
            | FailpointType::BoundaryLeak
            | FailpointType::ImmutableMutation
            | FailpointType::SyntaxAnomaly => FailpointGroup::LinguisticSyntax,

            FailpointType::RollbackTriggered
            | FailpointType::CaveatVeto
            | FailpointType::CyclicRetroaction => {
                FailpointGroup::RuntimeTransaction
            }

            FailpointType::EquilibriumDriftVeto => {
                FailpointGroup::HomeostasisLifecycle
            }
        }
    }

    /// Resolves the granular subgroup classification for this failpoint.
    pub fn subgroup(&self) -> FailpointSubgroup {
        match self {
            FailpointType::CpuHaltTrap => FailpointSubgroup::CpuTrap,
            FailpointType::HardwareFault | FailpointType::WatchdogTimeout => {
                FailpointSubgroup::FirmwareBus
            }
            FailpointType::AluEquilibriumBreach => FailpointSubgroup::AluSanity,

            FailpointType::MemoryLandmarkBreach => {
                FailpointSubgroup::LandmarkBounds
            }

            FailpointType::DualRailSaturation => {
                FailpointSubgroup::DualRailCollision
            }
            FailpointType::NullTrap => FailpointSubgroup::StateCollapse,

            FailpointType::BoundaryMismatch | FailpointType::BoundaryLeak => {
                FailpointSubgroup::DelimiterHierarchy
            }
            FailpointType::SyntaxAnomaly => {
                FailpointSubgroup::StructuralScaffolding
            }
            FailpointType::ImmutableMutation => {
                FailpointSubgroup::TypeSemantic
            }

            FailpointType::RollbackTriggered
            | FailpointType::CyclicRetroaction => FailpointSubgroup::Retroaction,
            FailpointType::CaveatVeto => FailpointSubgroup::ConcurrencyBus,

            FailpointType::EquilibriumDriftVeto => {
                FailpointSubgroup::EquilibriumDrift
            }
        }
    }

    /// Returns `true` if this failpoint triggers an abrupt HALT restart.
    pub fn is_halt_trigger(&self) -> bool {
        matches!(
            self,
            FailpointType::DualRailSaturation
                | FailpointType::CpuHaltTrap
                | FailpointType::NullTrap
                | FailpointType::AluEquilibriumBreach
        )
    }

    /// Resolves the default recovery state commitment for this failpoint.
    pub fn default_recovery_state(&self) -> QuadState {
        if self.is_halt_trigger() {
            QuadState::Halt
        } else {
            QuadState::Rollback
        }
    }

    /// Returns the uppercase canonical string identifier for this failpoint.
    pub fn as_str(&self) -> &'static str {
        match self {
            FailpointType::BoundaryMismatch => "FP_BOUNDARY_MISMATCH",
            FailpointType::BoundaryLeak => "FP_BOUNDARY_LEAK",
            FailpointType::NullTrap => "FP_NULL_TRAP",
            FailpointType::RollbackTriggered => "FP_ROLLBACK_TRIGGERED",
            FailpointType::CaveatVeto => "FP_CAVEAT_VETO",
            FailpointType::ImmutableMutation => "FP_IMMUTABLE_MUTATION",
            FailpointType::CyclicRetroaction => "FP_CYCLIC_RETROACTION",
            FailpointType::SyntaxAnomaly => "FP_SYNTAX_ANOMALY",
            FailpointType::HardwareFault => "FP_HARDWARE_FAULT",
            FailpointType::CpuHaltTrap => "FP_CPU_HALT_TRAP",
            FailpointType::AluEquilibriumBreach => "FP_ALU_EQUILIBRIUM_BREACH",
            FailpointType::MemoryLandmarkBreach => "FP_MEMORY_LANDMARK_BREACH",
            FailpointType::WatchdogTimeout => "FP_WATCHDOG_TIMEOUT",
            FailpointType::DualRailSaturation => "FP_DUAL_RAIL_SATURATION",
            FailpointType::EquilibriumDriftVeto => "FP_EQUILIBRIUM_DRIFT_VETO",
        }
    }

    /// Compact single-bit representation for game-dev flight recorder masks.
    pub fn bitmask(&self) -> u32 {
        match self {
            FailpointType::BoundaryMismatch => 1 << 0,
            FailpointType::BoundaryLeak => 1 << 1,
            FailpointType::NullTrap => 1 << 2,
            FailpointType::RollbackTriggered => 1 << 3,
            FailpointType::CaveatVeto => 1 << 4,
            FailpointType::ImmutableMutation => 1 << 5,
            FailpointType::CyclicRetroaction => 1 << 6,
            FailpointType::SyntaxAnomaly => 1 << 7,
            FailpointType::HardwareFault => 1 << 8,
            FailpointType::CpuHaltTrap => 1 << 9,
            FailpointType::AluEquilibriumBreach => 1 << 10,
            FailpointType::MemoryLandmarkBreach => 1 << 11,
            FailpointType::WatchdogTimeout => 1 << 12,
            FailpointType::DualRailSaturation => 1 << 13,
            FailpointType::EquilibriumDriftVeto => 1 << 14,
        }
    }
}

impl FailpointGroup {
    /// Returns the uppercase canonical string identifier for this group.
    pub fn as_str(&self) -> &'static str {
        match self {
            FailpointGroup::HardwareSilicon => "GRP_HARDWARE_SILICON",
            FailpointGroup::MemoryBoundary => "GRP_MEMORY_BOUNDARY",
            FailpointGroup::TernarySaturation => "GRP_TERNARY_SATURATION",
            FailpointGroup::LinguisticSyntax => "GRP_LINGUISTIC_SYNTAX",
            FailpointGroup::RuntimeTransaction => "GRP_RUNTIME_TRANSACTION",
            FailpointGroup::HomeostasisLifecycle => "GRP_HOMEOSTASIS_LIFECYCLE",
        }
    }

    /// Returns a human-readable explanation of this group's domain.
    pub fn description(&self) -> &'static str {
        match self {
            FailpointGroup::HardwareSilicon => {
                "Track 0 CPU traps, firmware bus, and ALU sanity"
            }
            FailpointGroup::MemoryBoundary => {
                "RAM landmarks (0x7C00..0x200000) and paging boundaries"
            }
            FailpointGroup::TernarySaturation => {
                "Dual-rail saturated (11 = HALT) states and state collapse"
            }
            FailpointGroup::LinguisticSyntax => {
                "4-Block architecture, delimiters, and grammar syntax"
            }
            FailpointGroup::RuntimeTransaction => {
                "Transactional rollbacks, retroaction depth, and concurrency"
            }
            FailpointGroup::HomeostasisLifecycle => {
                "Baseline Omega_0 continuity, homeostasis, and shutdown audits"
            }
        }
    }
}

impl FailpointSubgroup {
    /// Returns the uppercase canonical string identifier for this subgroup.
    pub fn as_str(&self) -> &'static str {
        match self {
            FailpointSubgroup::CpuTrap => "SUBGRP_CPU_TRAP",
            FailpointSubgroup::FirmwareBus => "SUBGRP_FIRMWARE_BUS",
            FailpointSubgroup::AluSanity => "SUBGRP_ALU_SANITY",
            FailpointSubgroup::LandmarkBounds => "SUBGRP_LANDMARK_BOUNDS",
            FailpointSubgroup::SegmentationPaging => {
                "SUBGRP_SEGMENTATION_PAGING"
            }
            FailpointSubgroup::DualRailCollision => {
                "SUBGRP_DUAL_RAIL_COLLISION"
            }
            FailpointSubgroup::StateCollapse => "SUBGRP_STATE_COLLAPSE",
            FailpointSubgroup::DelimiterHierarchy => {
                "SUBGRP_DELIMITER_HIERARCHY"
            }
            FailpointSubgroup::StructuralScaffolding => {
                "SUBGRP_STRUCTURAL_SCAFFOLDING"
            }
            FailpointSubgroup::TypeSemantic => "SUBGRP_TYPE_SEMANTIC",
            FailpointSubgroup::Retroaction => "SUBGRP_RETROACTION",
            FailpointSubgroup::ConcurrencyBus => "SUBGRP_CONCURRENCY_BUS",
            FailpointSubgroup::EquilibriumDrift => "SUBGRP_EQUILIBRIUM_DRIFT",
            FailpointSubgroup::ShutdownVeto => "SUBGRP_SHUTDOWN_VETO",
        }
    }

    /// Returns the parent domain group for this subgroup.
    pub fn parent_group(&self) -> FailpointGroup {
        match self {
            FailpointSubgroup::CpuTrap
            | FailpointSubgroup::FirmwareBus
            | FailpointSubgroup::AluSanity => FailpointGroup::HardwareSilicon,

            FailpointSubgroup::LandmarkBounds
            | FailpointSubgroup::SegmentationPaging => {
                FailpointGroup::MemoryBoundary
            }

            FailpointSubgroup::DualRailCollision
            | FailpointSubgroup::StateCollapse => {
                FailpointGroup::TernarySaturation
            }

            FailpointSubgroup::DelimiterHierarchy
            | FailpointSubgroup::StructuralScaffolding
            | FailpointSubgroup::TypeSemantic => {
                FailpointGroup::LinguisticSyntax
            }

            FailpointSubgroup::Retroaction
            | FailpointSubgroup::ConcurrencyBus => {
                FailpointGroup::RuntimeTransaction
            }

            FailpointSubgroup::EquilibriumDrift
            | FailpointSubgroup::ShutdownVeto => {
                FailpointGroup::HomeostasisLifecycle
            }
        }
    }
}

// =============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Display Trait Implementations, Invariant Proofs & Module Sealing )
// =============================================================================

impl fmt::Display for FailpointType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} [{}:{}]",
            self.as_str(),
            self.group().as_str(),
            self.subgroup().as_str()
        )
    }
}

impl fmt::Display for FailpointGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Display for FailpointSubgroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

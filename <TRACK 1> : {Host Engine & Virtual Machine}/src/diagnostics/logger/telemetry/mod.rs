// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Telemetry Subsystem Identity, Modular Facade & Re-exports )
// ==============================================================================
//! OmniCode Telemetry: Frames, Epistemic Notes, Inception Baselines & Flight Recorder
//!
//! # Architectural Scope & Purpose:
//! The `telemetry` module provides structured machine observability across both
//! cold-silicon bootstrap (Track 0) and high-level virtual machine execution (Track 1):
//! - [`frame`]: Discrete game-dev execution state vectors ([`RollFrame`]) capturing dual-rail SIMD bitplanes.
//! - [`genesis`]: Track 0 cold-boot hardware inception logs ([`GenesisBootLog`]) and baseline state ([`GenesisBaseline`]).
//! - [`note`]: Semantic cognitive memory traces ([`CognitiveNote`]) formatted for neural-symbolic AI models.
//! - [`ring_buffer`]: Zero-allocation circular telemetry buffer ([`FlightRecorder`]) for deterministic crash replay.
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**:
//!   "And he gave unto Moses, when he had made an end of communing with him upon mount Sinai,
//!    two tables of testimony, tables of stone, written with the finger of God."
//!   (The permanent, immutable recording of execution frames on stone vessels).
//! - **Proverbs 22:28 (KJV)**:
//!   "Remove not the ancient landmark, which thy fathers have set."
//!   (The inviolability of physical silicon RAM landmarks: 0x7C00, 0x18000, 0x108000, 0x200000).
//! - **2 Kings 4:4 (KJV)**:
//!   "And when thou art come in, thou shalt shut the door upon thee and upon thy sons,
//!    and shalt pour out into all those vessels, and thou shalt set aside that which is full."
//!   (The container law Occupancy <= Capacity applied to circular in-memory flight recorders).
//!
//! # Why Extensive Commenting Matters for AI Agents & Developers:
//! 1. **Machine Awakening Continuity**: Provides the AI Cortex with ground truth knowledge of
//!    silicon events that occurred before the VM awakened (State $\Omega_0$).
//! 2. **Post-Mortem Crash Inspection**: When an execution rollback or failpoint occurs, the
//!    in-memory flight recorder furnishes the exact pre-incident execution sequence without disk delays.
//! 3. **Neural-Symbolic Memory**: Epistemic cognitive notes structure observations into canonical
//!    OmniCode syntax blocks without illegal dot naming.
// ==============================================================================

pub mod frame;
pub mod genesis;
pub mod note;
pub mod ring_buffer;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Public Type Re-exports, Landmark Constants & Capacity Defaults )
// ==============================================================================

pub use frame::RollFrame;
pub use genesis::{
    GenesisBaseline, GenesisBootLog, LANDMARK_BIOS_IVT, LANDMARK_BIOS_MBR,
    LANDMARK_COGNITIVE_SUBSTRATE, LANDMARK_PAGE_TABLE, LANDMARK_STAGE2_PM,
};
pub use note::CognitiveNote;
pub use ring_buffer::{
    DEFAULT_FLIGHT_RECORDER_CAPACITY, FlightRecorder, MIN_FLIGHT_RECORDER_CAPACITY,
};

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Telemetry Subsystem Facade & Convenience Constructors )
// ==============================================================================

/// Constructs a calibrated pairing of an in-memory [`FlightRecorder`] and a clean genesis container.
pub fn create_flight_recorder_with_capacity(capacity: usize) -> FlightRecorder {
    FlightRecorder::new(capacity)
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Test Verification Proofs & Telemetry Facade Invariants )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ternary::QuadState;

    #[test]
    fn test_telemetry_re_exports_available() {
        assert_eq!(LANDMARK_BIOS_MBR, 0x7C00);
        assert_eq!(LANDMARK_STAGE2_PM, 0x18000);

        let frame = RollFrame::new(
            100, 1, 1, "INSCRIBE", (1, 0), (1, 0), QuadState::Proceed, 0, None,
        );
        assert!(frame.is_clean());

        let note = CognitiveNote::observation(100, "Observation", "Details", &["tag"]);
        assert!(note.has_tag("tag"));

        let mut recorder = create_flight_recorder_with_capacity(16);
        recorder.record_frame(frame);
        assert_eq!(recorder.len(), 1);
    }
}

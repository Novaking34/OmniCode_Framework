// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Telemetry Subsystem Re-exports & Frame Facade )
// ==============================================================================
//! Telemetry Subsystem: Frames, Notes & In-Memory Flight Recorder

pub mod frame;
pub mod note;
pub mod ring_buffer;
pub mod genesis;

pub use frame::RollFrame;
pub use note::CognitiveNote;
pub use ring_buffer::FlightRecorder;
pub use genesis::{GenesisBootLog, GenesisBaseline};

// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Universal Telemetry & Hardware Clock Re-exports )
// ==============================================================================
//! Universal Clock, Bus & Hardware Telemetry Machinery

pub mod clock;
pub mod bus;

pub use clock::{MonotonicClock, generate_timestamp_id};
pub use bus::{LogEvent, EventBus};

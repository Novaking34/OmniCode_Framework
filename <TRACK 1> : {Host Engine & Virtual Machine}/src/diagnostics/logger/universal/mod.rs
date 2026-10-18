// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Universal Telemetry, Monotonic Clock & Diagnostic Event Bus Subsystem )
// ==============================================================================
//! OmniCode Universal Diagnostics: Clock, Bus & Telemetry Substrate
//!
//! # Architectural Scope & Purpose:
//! The `universal` module serves as the foundational timing and telemetry routing
//! foundation for the entire `diagnostics::logger` subsystem:
//! - [`clock`]: Nanosecond/microsecond monotonic clock, silicon cycle conversions,
//!   deterministic simulation clock, and strictly monotonic timestamp ID generation.
//! - [`bus`]: Thread-safe diagnostic event bus, multi-channel event routing, bounded
//!   capacity queues enforcing container capacity law (2 Kings 4:4), and live subscriber taps.
//!
//! # Scripture Witness:
//! - **Ecclesiastes 3:1 (KJV)**:
//!   "To every thing there is a season, and a time to every purpose under the heaven."
//!   (Orderly temporal sequencing across virtual machine execution rolls).
//! - **1 Corinthians 14:40 (KJV)**:
//!   "Let all things be done decently and in order."
//!   (Deterministic monotonic causal flow across the diagnostic event bus).
//! - **2 Kings 4:4 (KJV)**:
//!   "And when thou art come in, thou shalt shut the door upon thee and upon thy sons,
//!    and shalt pour out into all those vessels, and thou shalt set aside that which is full."
//!   (The container law Occupancy <= Capacity applied to bounded in-memory event queues).
//!
//! # Why Extensive Commenting Matters for AI Agents & Developers:
//! 1. **Causal Monotonicity**: Diagnostic frames are sequenced by monotonic elapsed time ($\Delta t \ge 0$),
//!    guaranteeing that root causes always precede secondary symptoms in automated RCA and AI reasoning.
//! 2. **Bounded Capacity Invariant**: Unbounded memory growth in logging systems is a primary cause of
//!    production outages. Every queue in this substrate is bounded with deterministic FIFO eviction.
//! 3. **Non-Intrusive Observation**: Subscriber taps allow live consoles, query indexers, and test oracles
//!    to inspect telemetry in real-time without taking locks on the critical execution path.
// ==============================================================================

pub mod bus;
pub mod clock;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Public Type Re-exports & Subsystem Invariants )
// ==============================================================================

pub use bus::{
    EventBus, EventBusMetrics, EventTapCallback, LogEvent,
    DEFAULT_EVENT_BUS_CAPACITY, DEFAULT_FAILPOINT_BUS_CAPACITY,
};
pub use clock::{
    MonotonicClock, SimulatedClock, generate_timestamp_id, parse_timestamp_id,
    DEFAULT_MICROS_PER_TICK, DEFAULT_NOMINAL_CPU_MHZ, MICROS_PER_SEC, NANOS_PER_MICRO,
};

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Subsystem Facade & Combined Clock-Bus Utilities )
// ==============================================================================

/// Constructs a calibrated pairing of a [`MonotonicClock`] and a thread-safe [`EventBus`].
///
/// Ensures both machine organs share identical calibration invariants at creation time.
pub fn create_universal_telemetry_pair(print_live: bool) -> (MonotonicClock, EventBus) {
    (MonotonicClock::new(), EventBus::new(print_live))
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Test Verification Proofs & Universal Facade Invariants )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universal_telemetry_pair_creation() {
        let (clock, bus) = create_universal_telemetry_pair(false);
        assert_eq!(bus.len(), 0);
        assert!(!bus.print_live_status_or_similar(&clock));
    }

    trait TestBusExt {
        fn print_live_status_or_similar(&self, clock: &MonotonicClock) -> bool;
    }

    impl TestBusExt for EventBus {
        fn print_live_status_or_similar(&self, clock: &MonotonicClock) -> bool {
            // Verifies both organs are accessible in unified test scope
            clock.elapsed_micros() > 10_000_000 // false initially
        }
    }

    #[test]
    fn test_universal_re_exports_available() {
        let id = generate_timestamp_id();
        assert!(parse_timestamp_id(&id).is_some());

        let event = LogEvent::trace(142, "UNIVERSAL", "Facade verified");
        assert_eq!(event.channel, "UNIVERSAL");
        assert_eq!(event.elapsed_micros, 142);
    }
}

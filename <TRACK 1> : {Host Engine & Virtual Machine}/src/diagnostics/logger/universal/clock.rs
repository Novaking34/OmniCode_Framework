// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Universal High-Precision Clock, Monotonic Timers & Sequencer Substrate )
// ==============================================================================
//! OmniCode Universal Diagnostics: Monotonic Clock & Timestamp Substrate
//!
//! # Architectural Scope & Purpose:
//! Provides nanosecond/microsecond resolution elapsed time offsets, strictly
//! monotonic timestamp identifiers for telemetry frame ordering, and deterministic
//! simulation stepping for virtual machine replay:
//! - [`MonotonicClock`]: Real-time monotonic reference measuring microsecond/nanosecond durations.
//! - [`SimulatedClock`]: Deterministic virtual machine clock for reproducible execution & roll stepping.
//! - [`generate_timestamp_id`]: Thread-safe, monotonic timestamp ID generator resilient to clock skews.
//! - [`parse_timestamp_id`]: Exact parser decomposing timestamp identifiers into epoch and sequence parts.
//!
//! # Scripture Witness:
//! - **Ecclesiastes 3:1 (KJV)**:
//!   "To every thing there is a season, and a time to every purpose under the heaven."
//!   (The foundational truth that every computation, event, and roll operates within an orderly time frame).
//! - **Proverbs 4:26 (KJV)**:
//!   "Ponder the path of thy feet, and let all thy ways be established."
//!   (The discipline of measuring every microsecond offset and ordering telemetry events without drift).
//! - **1 Corinthians 14:40 (KJV)**:
//!   "Let all things be done decently and in order."
//!   (The strict causal monotonicity invariant: an effect must never precede its cause in timestamp ordering).
//!
//! # Why Extensive Commenting Matters for AI Agents & Developers:
//! 1. **Causal Monotonicity**: Wall-clock timestamps (`SystemTime`) are subject to NTP adjustments,
//!    daylight savings jumps, and leap seconds. If time moves backward, causal reasoning in the AI
//!    Cortex collapses. This module guarantees strict monotonicity ($\Delta t \ge 0$).
//! 2. **Hardware & Silicon Correlation**: Converting CPU cycle counts (TSC) from Track 0 bare-metal
//!    landmarks to microsecond durations requires calibrated frequency conversion factors.
//! 3. **Deterministic Virtual Machine Replay**: During automated debugging and test execution,
//!    real-world timing jitter causes non-reproducible race conditions. [`SimulatedClock`] provides
//!    freeze-and-step discrete time semantics for bitwise-reproducible roll execution.
// ==============================================================================

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::{Instant, SystemTime};

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Constants, Clock Data Structures & Timestamp Definitions )
// ==============================================================================

/// Default nominal CPU clock frequency in Megahertz (2,000 MHz = 2.0 GHz).
pub const DEFAULT_NOMINAL_CPU_MHZ: u64 = 2000;

/// Number of nanoseconds per microsecond ($1\ \mu\text{s} = 1000\ \text{ns}$).
pub const NANOS_PER_MICRO: u128 = 1000;

/// Number of microseconds per second ($1\ \text{s} = 1,000,000\ \mu\text{s}$).
pub const MICROS_PER_SEC: u128 = 1_000_000;

/// Default microseconds per discrete virtual machine execution tick.
pub const DEFAULT_MICROS_PER_TICK: u128 = 100;

/// Real-time monotonic clock reference measuring microsecond and nanosecond elapsed durations.
///
/// Backed by [`std::time::Instant`] which is guaranteed to be monotonically non-decreasing.
#[derive(Debug, Clone)]
pub struct MonotonicClock {
    start: Instant,
}

/// Deterministic simulation clock for virtual machine stepping, replay, and discrete time testing.
///
/// Allows freezing real-time progression and advancing in discrete microsecond or tick intervals.
#[derive(Debug)]
pub struct SimulatedClock {
    current_micros: AtomicU64,
    current_tick: AtomicU64,
    frozen: AtomicBool,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Clock Implementations, Monotonic Sequencer & Silicon Cycle Conversion )
// ==============================================================================

/// Generates a strictly unique, monotonically non-decreasing timestamp identifier.
///
/// # Format:
/// `<unix_epoch_micros>_<atomic_counter:04>`
///
/// # Monotonicity Guarantee:
/// Even if the operating system wall clock jumps backwards (e.g. NTP synchronization
/// or VM migration), this function maintains monotonicity by ensuring the epoch portion
/// never regresses below the previously observed maximum.
pub fn generate_timestamp_id() -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    static LAST_MICROS: Mutex<u128> = Mutex::new(0);

    let count = COUNTER.fetch_add(1, Ordering::Relaxed);
    let wall_now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros();

    let mut last = LAST_MICROS.lock().unwrap();
    let epoch_micros = if wall_now > *last {
        *last = wall_now;
        wall_now
    } else {
        *last
    };

    format!("{epoch_micros}_{count:04}")
}

/// Parses a standardized timestamp identifier into its constituent microsecond epoch
/// and sequence counter parts.
///
/// Returns `Some((epoch_micros, sequence_counter))` on success, or `None` if the string
/// does not conform to the `<epoch_micros>_<counter>` format.
pub fn parse_timestamp_id(id: &str) -> Option<(u128, u64)> {
    let parts: Vec<&str> = id.split('_').collect();
    if parts.len() != 2 {
        return None;
    }

    let epoch_micros = parts[0].parse::<u128>().ok()?;
    let counter = parts[1].parse::<u64>().ok()?;

    Some((epoch_micros, counter))
}

impl MonotonicClock {
    /// Constructs a new [`MonotonicClock`] calibrated to the current instant.
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    /// Returns elapsed microseconds since clock initialization.
    #[inline]
    pub fn elapsed_micros(&self) -> u128 {
        self.start.elapsed().as_micros()
    }

    /// Returns elapsed nanoseconds since clock initialization.
    #[inline]
    pub fn elapsed_nanos(&self) -> u128 {
        self.start.elapsed().as_nanos()
    }

    /// Returns elapsed milliseconds as a floating-point value.
    #[inline]
    pub fn elapsed_millis(&self) -> f64 {
        self.start.elapsed().as_nanos() as f64 / 1_000_000.0
    }

    /// Resets the base reference instant to now.
    pub fn reset(&mut self) {
        self.start = Instant::now();
    }

    /// Converts elapsed microseconds into estimated CPU clock cycles at the given frequency (in MHz).
    ///
    /// # Formula:
    /// $\text{Cycles} = \text{microseconds} \times \text{MHz}$
    #[inline]
    pub fn micros_to_cycles(micros: u128, mhz: u64) -> u64 {
        (micros as u64).saturating_mul(mhz)
    }

    /// Converts raw CPU clock cycles (e.g. from x86 `RDTSC`) to microseconds at the given frequency.
    ///
    /// # Formula:
    /// $\text{Microseconds} = \frac{\text{Cycles}}{\text{MHz}}$
    #[inline]
    pub fn cycles_to_micros(cycles: u64, mhz: u64) -> u128 {
        if mhz == 0 {
            0
        } else {
            (cycles / mhz) as u128
        }
    }
}

impl Default for MonotonicClock {
    fn default() -> Self {
        Self::new()
    }
}

impl SimulatedClock {
    /// Constructs a new [`SimulatedClock`] initialized at the specified microsecond offset.
    pub fn new(start_micros: u64) -> Self {
        Self {
            current_micros: AtomicU64::new(start_micros),
            current_tick: AtomicU64::new(0),
            frozen: AtomicBool::new(false),
        }
    }

    /// Returns the current simulated microsecond timestamp.
    pub fn current_micros(&self) -> u128 {
        self.current_micros.load(Ordering::Acquire) as u128
    }

    /// Returns the current discrete virtual machine execution tick.
    pub fn current_tick(&self) -> u64 {
        self.current_tick.load(Ordering::Acquire)
    }

    /// Advances the simulated clock by the specified microsecond delta.
    ///
    /// If the clock is currently frozen, this operation is a no-op unless explicitly forced.
    pub fn step_micros(&self, delta: u64) {
        if !self.frozen.load(Ordering::Acquire) {
            self.current_micros.fetch_add(delta, Ordering::Release);
        }
    }

    /// Advances the simulated clock by a discrete number of virtual machine execution ticks.
    ///
    /// Also increments the microsecond counter by `ticks * micros_per_tick`.
    pub fn step_ticks(&self, ticks: u64, micros_per_tick: u64) {
        if !self.frozen.load(Ordering::Acquire) {
            self.current_tick.fetch_add(ticks, Ordering::Release);
            self.current_micros
                .fetch_add(ticks.saturating_mul(micros_per_tick), Ordering::Release);
        }
    }

    /// Freezes the simulation clock, preventing automated time progression.
    pub fn freeze(&self) {
        self.frozen.store(true, Ordering::Release);
    }

    /// Resumes the simulation clock.
    pub fn resume(&self) {
        self.frozen.store(false, Ordering::Release);
    }

    /// Returns `true` if the simulation clock is currently frozen.
    pub fn is_frozen(&self) -> bool {
        self.frozen.load(Ordering::Acquire)
    }

    /// Resets the simulation clock back to zero.
    pub fn reset(&self) {
        self.current_micros.store(0, Ordering::Release);
        self.current_tick.store(0, Ordering::Release);
        self.frozen.store(false, Ordering::Release);
    }
}

impl Default for SimulatedClock {
    fn default() -> Self {
        Self::new(0)
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Test Verification Proofs & Monotonic Invariant Tests )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_monotonic_clock_elapsed_and_reset() {
        let mut clock = MonotonicClock::new();
        thread::sleep(Duration::from_millis(5));

        let micros = clock.elapsed_micros();
        let nanos = clock.elapsed_nanos();
        let millis = clock.elapsed_millis();

        assert!(micros >= 4_000, "Expected at least 4000 µs elapsed, got {micros}");
        assert!(nanos >= 4_000_000, "Expected at least 4,000,000 ns elapsed, got {nanos}");
        assert!(millis >= 4.0, "Expected at least 4.0 ms elapsed, got {millis}");

        clock.reset();
        let reset_micros = clock.elapsed_micros();
        assert!(reset_micros < micros, "Reset clock should be less than previous elapsed");
    }

    #[test]
    fn test_timestamp_id_generation_and_parsing() {
        let id1 = generate_timestamp_id();
        let id2 = generate_timestamp_id();

        assert_ne!(id1, id2, "Generated timestamp IDs must be strictly unique");

        let parsed1 = parse_timestamp_id(&id1);
        let parsed2 = parse_timestamp_id(&id2);

        assert!(parsed1.is_some(), "ID1 must be parsable");
        assert!(parsed2.is_some(), "ID2 must be parsable");

        let (epoch1, count1) = parsed1.unwrap();
        let (epoch2, count2) = parsed2.unwrap();

        assert!(epoch2 >= epoch1, "Epoch micros must be monotonically non-decreasing");
        assert!(count2 > count1, "Counter must strictly increase");

        assert_eq!(parse_timestamp_id("invalid_format_here_extra"), None);
        assert_eq!(parse_timestamp_id("notanumber_0001"), None);
        assert_eq!(parse_timestamp_id("12345_notanumber"), None);
    }

    #[test]
    fn test_silicon_cycle_and_microsecond_conversions() {
        let mhz = 2000; // 2.0 GHz
        let micros = 500; // 500 µs
        let cycles = MonotonicClock::micros_to_cycles(micros, mhz);

        assert_eq!(cycles, 1_000_000, "500 µs at 2.0 GHz should equal 1,000,000 cycles");

        let back_to_micros = MonotonicClock::cycles_to_micros(cycles, mhz);
        assert_eq!(back_to_micros, micros, "Cycle roundtrip must yield original microsecond offset");

        assert_eq!(MonotonicClock::cycles_to_micros(1_000_000, 0), 0, "Zero MHz must yield 0 µs safely");
    }

    #[test]
    fn test_simulated_clock_lifecycle_and_freeze() {
        let sim = SimulatedClock::new(1000);
        assert_eq!(sim.current_micros(), 1000);
        assert_eq!(sim.current_tick(), 0);
        assert!(!sim.is_frozen());

        sim.step_micros(500);
        assert_eq!(sim.current_micros(), 1500);

        sim.step_ticks(5, 100);
        assert_eq!(sim.current_tick(), 5);
        assert_eq!(sim.current_micros(), 2000);

        sim.freeze();
        assert!(sim.is_frozen());

        sim.step_micros(500);
        sim.step_ticks(2, 100);
        assert_eq!(sim.current_micros(), 2000, "Frozen clock must not advance on step_micros");
        assert_eq!(sim.current_tick(), 5, "Frozen clock must not advance on step_ticks");

        sim.resume();
        assert!(!sim.is_frozen());
        sim.step_micros(250);
        assert_eq!(sim.current_micros(), 2250);

        sim.reset();
        assert_eq!(sim.current_micros(), 0);
        assert_eq!(sim.current_tick(), 0);
    }
}

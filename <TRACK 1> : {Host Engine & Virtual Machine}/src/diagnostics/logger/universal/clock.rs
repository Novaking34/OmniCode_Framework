// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Monotonic Clock, Microsecond Timers & Timestamp Generator )
// ==============================================================================
//! Universal High-Precision Clock & Monotonic Timestamp Generator
//!
//! Provides nanosecond/microsecond resolution elapsed time offsets and
//! strictly monotonic timestamp identifiers for telemetry frame ordering.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Instant, SystemTime};

/// Generates a strictly unique, monotonic timestamp identifier.
///
/// Format: `<unix_epoch_micros>_<atomic_counter:04>`
pub fn generate_timestamp_id() -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let count = COUNTER.fetch_add(1, Ordering::Relaxed);
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros();
    format!("{now}_{count:04}")
}

/// Monotonic clock reference measuring microsecond elapsed durations.
#[derive(Debug, Clone)]
pub struct MonotonicClock {
    start: Instant,
}

impl MonotonicClock {
    /// Constructs a new [`MonotonicClock`] calibrated to the current instant.
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    /// Returns elapsed microseconds since clock initialization.
    pub fn elapsed_micros(&self) -> u128 {
        self.start.elapsed().as_micros()
    }

    /// Resets the base reference instant to now.
    pub fn reset(&mut self) {
        self.start = Instant::now();
    }
}

impl Default for MonotonicClock {
    fn default() -> Self {
        Self::new()
    }
}

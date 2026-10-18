// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( FlightRecorder: Fixed-Capacity Circular Buffer for Deterministic Replay )
// ==============================================================================
//! OmniCode Telemetry: In-Memory Circular Flight Recorder
//!
//! # Architectural Scope & Purpose:
//! High-performance, zero-allocation circular ring buffer for execution telemetry.
//! Retains the most recent execution frames in memory with steady-state $O(1)$ operations,
//! enabling instantaneous post-mortem crash inspection without disk I/O latency:
//! - [`FlightRecorder`]: Bounded circular buffer recording [`RollFrame`] telemetry.
//!
//! # Scripture Witness:
//! - **2 Kings 4:4 (KJV)**:
//!   "And when thou art come in, thou shalt shut the door upon thee and upon thy sons,
//!    and shalt pour out into all those vessels, and thou shalt set aside that which is full."
//!   (The container law Occupancy <= Capacity applied to fixed-capacity memory buffers).
//! - **Ecclesiastes 1:6 (KJV)**:
//!   "The wind goeth toward the south, and turneth about unto the north; it whirleth about continually,
//!    and the wind returneth again according to his circuits."
//!   (The circular wraparound invariant: continuous execution within bounded circuits).
//! - **Proverbs 24:27 (KJV)**:
//!   "Prepare thy work without, and make it fit for thyself in the field; and afterwards build thine house."
//!   (Allocating fixed ring buffer capacity upfront to guarantee zero allocation jitter).
//!
//! # Why Extensive Commenting Matters for AI Agents & Developers:
//! 1. **Zero Steady-State Allocations**: After initialization, `record_frame()` mutates existing slots
//!    in place without triggering heap allocations or garbage collection pauses.
//! 2. **Chronological Order Reconstruction**: Regardless of the internal `write_head` position,
//!    `recent_frames()` unwinds the circular buffer into strict chronological sequence.
//! 3. **Instant Post-Mortem Root Cause Analysis**: When an anomaly or crash occurs, the flight recorder
//!    supplies the exact execution window leading up to the failure.
// ==============================================================================

use super::frame::RollFrame;
use crate::core::ternary::QuadState;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Constants, FlightRecorder Struct Definition & Buffer Sizing )
// ==============================================================================

/// Default circular frame capacity for the in-memory flight recorder (512 frames).
pub const DEFAULT_FLIGHT_RECORDER_CAPACITY: usize = 512;

/// Minimum allowable circular frame capacity to ensure diagnostic utility (16 frames).
pub const MIN_FLIGHT_RECORDER_CAPACITY: usize = 16;

/// In-memory circular telemetry buffer storing a bounded window of recent execution frames.
#[derive(Debug, Clone)]
pub struct FlightRecorder {
    frames: Vec<RollFrame>,
    capacity: usize,
    write_head: usize,
    total_inscribed: usize,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Buffer Inscription, Chronological Unwinding & Anomaly Extraction )
// ==============================================================================

impl FlightRecorder {
    /// Constructs a new [`FlightRecorder`] with the specified frame capacity.
    ///
    /// Capacity is clamped to at least [`MIN_FLIGHT_RECORDER_CAPACITY`].
    pub fn new(capacity: usize) -> Self {
        let cap = capacity.max(MIN_FLIGHT_RECORDER_CAPACITY);
        Self {
            frames: Vec::with_capacity(cap),
            capacity: cap,
            write_head: 0,
            total_inscribed: 0,
        }
    }

    /// Inscribes a new frame into the circular buffer in $O(1)$ time.
    ///
    /// Operates without heap allocations once the capacity ceiling has been reached.
    pub fn record_frame(&mut self, frame: RollFrame) {
        if self.frames.len() < self.capacity {
            self.frames.push(frame);
        } else {
            self.frames[self.write_head] = frame;
        }
        self.write_head = (self.write_head + 1) % self.capacity;
        self.total_inscribed += 1;
    }

    /// Returns a chronological snapshot of the most recent `count` frames.
    ///
    /// Guarantees chronological order from oldest to newest.
    pub fn recent_frames(&self, count: usize) -> Vec<RollFrame> {
        let available = self.frames.len();
        let take = count.min(available);
        if take == 0 {
            return Vec::new();
        }

        let mut result = Vec::with_capacity(take);
        if available < self.capacity {
            let start = available.saturating_sub(take);
            result.extend_from_slice(&self.frames[start..available]);
        } else {
            for i in 0..take {
                let idx = (self.write_head + self.capacity - take + i) % self.capacity;
                result.push(self.frames[idx].clone());
            }
        }
        result
    }

    /// Returns all currently stored frames in strict chronological order.
    pub fn all_frames(&self) -> Vec<RollFrame> {
        self.recent_frames(self.frames.len())
    }

    /// Returns the most recent frame that resulted in a rollback or trapped failpoint.
    pub fn last_anomalous_frame(&self) -> Option<RollFrame> {
        for frame in self.all_frames().into_iter().rev() {
            if frame.has_anomaly() {
                return Some(frame);
            }
        }
        None
    }

    /// Returns all frames currently retained that experienced an anomaly or rollback.
    pub fn anomalous_frames(&self) -> Vec<RollFrame> {
        self.all_frames()
            .into_iter()
            .filter(|f| f.has_anomaly())
            .collect()
    }

    /// Filters retained frames by operational verb.
    pub fn frames_by_verb(&self, verb: &str) -> Vec<RollFrame> {
        self.all_frames()
            .into_iter()
            .filter(|f| f.verb.eq_ignore_ascii_case(verb))
            .collect()
    }

    /// Filters retained frames by QuadState execution status.
    pub fn frames_by_state(&self, state: QuadState) -> Vec<RollFrame> {
        self.all_frames()
            .into_iter()
            .filter(|f| f.quad_state == state)
            .collect()
    }

    /// Returns the maximum capacity of the ring buffer.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns the number of frames currently buffered in memory.
    #[inline]
    pub fn len(&self) -> usize {
        self.frames.len()
    }

    /// Returns `true` if no frames have been recorded.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    /// Returns the cumulative number of frames inscribed over the lifetime of the recorder.
    #[inline]
    pub fn total_inscribed(&self) -> usize {
        self.total_inscribed
    }

    /// Clears all frames and resets the write head to zero.
    pub fn clear(&mut self) {
        self.frames.clear();
        self.write_head = 0;
        self.total_inscribed = 0;
    }
}

impl Default for FlightRecorder {
    fn default() -> Self {
        Self::new(DEFAULT_FLIGHT_RECORDER_CAPACITY)
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Test Verification Proofs & Ring Buffer Invariant Tests )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flight_recorder_lifecycle_without_wraparound() {
        let mut recorder = FlightRecorder::new(16);
        assert!(recorder.is_empty());
        assert_eq!(recorder.len(), 0);
        assert_eq!(recorder.capacity(), 16);

        for i in 1u64..=5u64 {
            recorder.record_frame(RollFrame::new(
                (i as u128) * 10,
                i as usize,
                5,
                "STEP",
                (i, 0),
                (i + 1, 0),
                QuadState::Proceed,
                0,
                None,
            ));
        }

        assert_eq!(recorder.len(), 5);
        assert_eq!(recorder.total_inscribed(), 5);

        let recent = recorder.recent_frames(3);
        assert_eq!(recent.len(), 3);
        assert_eq!(recent[0].roll_index, 3);
        assert_eq!(recent[1].roll_index, 4);
        assert_eq!(recent[2].roll_index, 5);

        assert!(recorder.last_anomalous_frame().is_none());
    }

    #[test]
    fn test_flight_recorder_wraparound_chronological_order() {
        let mut recorder = FlightRecorder::new(16);

        // Inscribe 25 frames into capacity of 16
        for i in 1u64..=25u64 {
            let is_rollback = i == 20;
            let state = if is_rollback { QuadState::Rollback } else { QuadState::Proceed };
            let mask = if is_rollback { 1 } else { 0 };

            recorder.record_frame(RollFrame::new(
                (i as u128) * 100,
                i as usize,
                25,
                if i % 2 == 0 { "EVEN" } else { "ODD" },
                (i, 0),
                (i, 0),
                state,
                mask,
                None,
            ));
        }

        assert_eq!(recorder.len(), 16);
        assert_eq!(recorder.total_inscribed(), 25);

        let all = recorder.all_frames();
        assert_eq!(all.len(), 16);
        // Oldest retained frame should be 10 (25 - 16 + 1)
        assert_eq!(all[0].roll_index, 10);
        assert_eq!(all[15].roll_index, 25);

        // Last anomalous frame was roll 20
        let anomalous = recorder.last_anomalous_frame();
        assert!(anomalous.is_some());
        assert_eq!(anomalous.unwrap().roll_index, 20);

        // Filter by verb
        let evens = recorder.frames_by_verb("EVEN");
        assert_eq!(evens.len(), 8);

        // Filter by state
        let rollbacks = recorder.frames_by_state(QuadState::Rollback);
        assert_eq!(rollbacks.len(), 1);
        assert_eq!(rollbacks[0].roll_index, 20);

        recorder.clear();
        assert!(recorder.is_empty());
        assert_eq!(recorder.len(), 0);
        assert_eq!(recorder.total_inscribed(), 0);
    }
}

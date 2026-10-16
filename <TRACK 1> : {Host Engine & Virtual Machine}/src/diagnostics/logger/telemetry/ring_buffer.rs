// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( FlightRecorder: Fixed-Capacity Circular Buffer for Deterministic Replay )
// ==============================================================================
//! In-Memory Flight Recorder (Circular Telemetry Ring Buffer)
//!
//! Stores the last $K$ execution frames in memory with zero allocations in
//! steady-state, allowing instantaneous post-mortem crash inspection.

use super::frame::RollFrame;
use crate::core::ternary::QuadState;

/// In-memory circular telemetry buffer.
#[derive(Debug, Clone)]
pub struct FlightRecorder {
    frames: Vec<RollFrame>,
    capacity: usize,
    write_head: usize,
    total_inscribed: usize,
}

impl FlightRecorder {
    /// Constructs a new [`FlightRecorder`] with the specified frame capacity.
    pub fn new(capacity: usize) -> Self {
        let cap = capacity.max(16);
        Self {
            frames: Vec::with_capacity(cap),
            capacity: cap,
            write_head: 0,
            total_inscribed: 0,
        }
    }

    /// Inscribes a new frame into the circular buffer.
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
                let idx = (self.write_head + self.capacity - take + i)
                    % self.capacity;
                result.push(self.frames[idx].clone());
            }
        }
        result
    }

    /// Returns the last frame that resulted in a rollback or failpoint.
    pub fn last_anomalous_frame(&self) -> Option<RollFrame> {
        for frame in self.recent_frames(self.frames.len()).into_iter().rev() {
            if frame.quad_state == QuadState::Rollback || frame.failpoint_mask != 0 {
                return Some(frame);
            }
        }
        None
    }

    /// Total count of frames inscribed over the lifetime of the recorder.
    pub fn total_inscribed(&self) -> usize {
        self.total_inscribed
    }

    /// Clears the flight recorder.
    pub fn clear(&mut self) {
        self.frames.clear();
        self.write_head = 0;
        self.total_inscribed = 0;
    }
}

impl Default for FlightRecorder {
    fn default() -> Self {
        Self::new(512)
    }
}

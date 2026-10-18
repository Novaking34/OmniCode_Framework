// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( RetroactionTracker, Causal Momentum & Infinite Rollback Loop Trapping )
// ==============================================================================
// SCRIPTURAL WITNESS:
//   "The thing that hath been, it is that which shall be; and that which is
//    done is that which shall be done: and there is no new thing under the sun."
//   — Ecclesiastes 1:9 (KJV)
//
// ARCHITECTURAL TOPOLOGY:
//   The causal momentum monitor of the OmniDebugger.
//   Acts as the discerning eye observing rollback vectors:
//     - In balanced ternary physics, state reversals (`-1:ROLLBACK`) reverse
//       causal momentum to recover from invariant breaches.
//     - Tracks rollback frequency per named target label.
//     - Intercepts infinite retroaction traps (where rollbacks re-trigger
//       rollbacks infinitely) and halts execution via FailpointType::CyclicRetroaction.
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Imports, Standard Collections & Retroaction Data Models )
// ==============================================================================
use std::collections::HashMap;
use crate::diagnostics::logger::{DiagnosticLogger, FailpointType};

/// Tracks rollback iterations per target to prevent infinite undo loops.
#[derive(Debug, Clone)]
pub struct RetroactionTracker {
    /// Rollback counters keyed by target label/scope.
    pub counters: HashMap<String, usize>,
    /// Maximum permitted rollback operations per target before trapping cyclic failure.
    pub max_depth: usize,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Retroaction Monitoring, Threshold Enforcement & Counter Management )
// ==============================================================================
impl RetroactionTracker {
    /// Constructs a new [`RetroactionTracker`].
    pub fn new(max_depth: usize) -> Self {
        Self {
            counters: HashMap::new(),
            max_depth,
        }
    }

    /// Retrieves the current rollback iteration count for a specific target.
    pub fn get_count(&self, target: &str) -> usize {
        self.counters.get(target).copied().unwrap_or(0)
    }

    /// Resets the rollback counter for a specific target upon successful forward progress.
    pub fn reset(&mut self, target: &str) {
        self.counters.remove(target);
    }

    /// Monitors retroaction steps on a named target to prevent infinite rollback loops.
    ///
    /// # Failpoints Intercepted
    /// If rollback iterations exceed `max_depth`, traps [`FailpointType::CyclicRetroaction`].
    pub fn track(&mut self, target: &str, logger: &DiagnosticLogger) -> Result<usize, String> {
        let count = self.counters.entry(target.to_string()).or_insert(0);
        *count += 1;
        let cur = *count;

        logger.log(
            "VECTOR",
            format!("Gaze Focus: Retroaction step #{cur} on '{target}' (threshold: {})", self.max_depth),
        );

        if cur > self.max_depth {
            let msg = format!(
                "Cyclic retroaction loop detected! Target '{target}' rolled back {cur} times, \
                 exceeding threshold ({})",
                self.max_depth
            );
            logger.failpoint(FailpointType::CyclicRetroaction, "DEBUGGER", &msg);
            return Err(format!("FAILPOINT: {msg}"));
        }

        Ok(cur)
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Retroaction Unit Tests & Affirmative Invariant Sealing )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retroaction_tracker_bounds_and_tripwire() {
        let logger = DiagnosticLogger::new(false);
        let mut tracker = RetroactionTracker::new(3);

        assert_eq!(tracker.track("Cell_Mutation", &logger).unwrap(), 1);
        assert_eq!(tracker.track("Cell_Mutation", &logger).unwrap(), 2);
        assert_eq!(tracker.track("Cell_Mutation", &logger).unwrap(), 3);

        // 4th step exceeds threshold 3
        let err = tracker.track("Cell_Mutation", &logger);
        assert!(err.is_err());
        assert!(err.unwrap_err().contains("Cyclic retroaction loop detected"));

        // Resetting restores target to forward progress
        tracker.reset("Cell_Mutation");
        assert_eq!(tracker.get_count("Cell_Mutation"), 0);
        assert_eq!(tracker.track("Cell_Mutation", &logger).unwrap(), 1);
    }
}

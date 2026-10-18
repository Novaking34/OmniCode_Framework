// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( StateInspector, QuadState Truth Evaluation & Null Collision Trapping )
// ==============================================================================
// SCRIPTURAL WITNESS:
//   "But let your communication be, Yea, yea; Nay, nay: for whatsoever
//    is more than these cometh of evil."
//   — Matthew 5:37 (KJV)
//
// ARCHITECTURAL TOPOLOGY:
//   The execution truth state evaluator of the OmniDebugger.
//   Acts as the discerning eye observing balanced ternary execution physics:
//     - +1 : PROCEED (Deterministic commit, transactional state forward)
//     -  0 : AWAIT   (Homeostatic standby, balance pending observation)
//     - -1 : ROLLBACK(Causal reversal, snapshot rollback, restitution)
//     - NULL: HALT   (Void trap, illegal null collision, catastrophic fence)
//   Increments the global operational step counter and signals the DiagnosticLogger.
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Imports, Ternary Physics & State Inspector Model )
// ==============================================================================
use crate::core::ternary::QuadState;
use crate::diagnostics::logger::{DiagnosticLogger, FailpointType};

/// Evaluator tracking QuadState execution states and step metrics.
#[derive(Debug, Clone, Default)]
pub struct StateInspector {
    /// Monotonic operational step count observed across all evaluations.
    pub step_count: usize,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( QuadState Inspection, Null Trap Interception & Telemetry Emission )
// ==============================================================================
impl StateInspector {
    /// Constructs a new [`StateInspector`].
    pub fn new() -> Self {
        Self { step_count: 0 }
    }

    /// Inspects the QuadState truth value and raises appropriate signals or failpoints.
    ///
    /// - `QuadState::Halt`: Traps void / NULL halt trap (`FailpointType::NullTrap`).
    /// - `QuadState::Rollback`: Traps rollback trigger (`FailpointType::RollbackTriggered`).
    /// - `QuadState::Await`: Standby state logged.
    /// - `QuadState::Proceed`: Commit state logged.
    pub fn inspect(&mut self, state: QuadState, context: &str, logger: &DiagnosticLogger) {
        self.step_count += 1;
        let step = self.step_count;

        match state {
            QuadState::Halt => {
                logger.failpoint(
                    FailpointType::NullTrap,
                    "DEBUGGER",
                    format!("Gaze Focus [Step #{step}]: NULL / Void halt trap encountered in context: {context}"),
                );
            }
            QuadState::Rollback => {
                logger.failpoint(
                    FailpointType::RollbackTriggered,
                    "DEBUGGER",
                    format!("Gaze Focus [Step #{step}]: Rollback (-1) triggered in context: {context}"),
                );
            }
            QuadState::Await => {
                logger.log(
                    "TERNARY",
                    format!("Gaze Focus [Step #{step}]: Await (0) standby in context: {context}"),
                );
            }
            QuadState::Proceed => {
                logger.log(
                    "TERNARY",
                    format!("Gaze Focus [Step #{step}]: Proceed (+1) committed in context: {context}"),
                );
            }
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( State Inspector Unit Tests & Affirmative Invariant Sealing )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_inspector_step_and_quad_states() {
        let logger = DiagnosticLogger::new(false);
        let mut inspector = StateInspector::new();
        assert_eq!(inspector.step_count, 0);

        inspector.inspect(QuadState::Proceed, "boot_sequence", &logger);
        assert_eq!(inspector.step_count, 1);

        inspector.inspect(QuadState::Await, "input_stream", &logger);
        assert_eq!(inspector.step_count, 2);

        inspector.inspect(QuadState::Rollback, "arithmetic_overflow", &logger);
        assert_eq!(inspector.step_count, 3);

        inspector.inspect(QuadState::Halt, "null_dereference", &logger);
        assert_eq!(inspector.step_count, 4);
    }
}

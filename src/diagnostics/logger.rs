// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Diagnostic Logger Identity, Failpoint Categories, and Invariant Enums )
// ==============================================================================
//! OmniCode Diagnostics: Structured Failpoint Logger
//!
//! Explicitly engineered to catch and report failpoints, boundary violations,
//! and state anomalies rather than merely recording pass-through execution.

use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FailpointType {
    BoundaryMismatch,
    BoundaryLeak,
    NullTrap,
    RollbackTriggered,
    CaveatVeto,
    ImmutableMutation,
    CyclicRetroaction,
    SyntaxAnomaly,
}

impl FailpointType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FailpointType::BoundaryMismatch => "FP_BOUNDARY_MISMATCH",
            FailpointType::BoundaryLeak => "FP_BOUNDARY_LEAK",
            FailpointType::NullTrap => "FP_NULL_TRAP",
            FailpointType::RollbackTriggered => "FP_ROLLBACK_TRIGGERED",
            FailpointType::CaveatVeto => "FP_CAVEAT_VETO",
            FailpointType::ImmutableMutation => "FP_IMMUTABLE_MUTATION",
            FailpointType::CyclicRetroaction => "FP_CYCLIC_RETROACTION",
            FailpointType::SyntaxAnomaly => "FP_SYNTAX_ANOMALY",
        }
    }
}


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( LogEvent Structure and DiagnosticLogger Initialization )
// ==============================================================================
#[derive(Debug, Clone)]
pub struct LogEvent {
    pub elapsed_micros: u128,
    pub channel: &'static str,
    pub message: String,
    pub is_failpoint: bool,
    pub failpoint_type: Option<FailpointType>,
}

#[derive(Debug, Clone)]
pub struct DiagnosticLogger {
    start_time: Instant,
    events: Arc<Mutex<Vec<LogEvent>>>,
    failpoints: Arc<Mutex<Vec<LogEvent>>>,
    print_live: bool,
}

impl DiagnosticLogger {
    pub fn new(print_live: bool) -> Self {
        Self {
            start_time: Instant::now(),
            events: Arc::new(Mutex::new(Vec::new())),
            failpoints: Arc::new(Mutex::new(Vec::new())),
            print_live,
        }
    }
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Core Logging Operations, Failpoint Interceptions, and Event Capture )
// ==============================================================================
impl DiagnosticLogger {
    pub fn log(&self, channel: &'static str, message: impl Into<String>) {
        let msg = message.into();
        let elapsed = self.start_time.elapsed().as_micros();
        let event = LogEvent {
            elapsed_micros: elapsed,
            channel,
            message: msg.clone(),
            is_failpoint: false,
            failpoint_type: None,
        };

        if self.print_live {
            println!("[{:>6} µs] DEBUG    [{}] {}", elapsed, channel, msg);
        }

        self.events.lock().unwrap().push(event);
    }

    pub fn failpoint(&self, fp_type: FailpointType, channel: &'static str, message: impl Into<String>) {
        let msg = message.into();
        let elapsed = self.start_time.elapsed().as_micros();
        let event = LogEvent {
            elapsed_micros: elapsed,
            channel,
            message: msg.clone(),
            is_failpoint: true,
            failpoint_type: Some(fp_type),
        };

        if self.print_live {
            eprintln!(
                "[{:>6} µs] \x1b[91mCRITICAL\x1b[0m [{}] [FAILPOINT: {}] {}",
                elapsed,
                channel,
                fp_type.as_str(),
                msg
            );
        }

        self.failpoints.lock().unwrap().push(event.clone());
        self.events.lock().unwrap().push(event);
    }

    pub fn failpoints(&self) -> Vec<LogEvent> {
        self.failpoints.lock().unwrap().clone()
    }

    pub fn has_failpoints(&self) -> bool {
        !self.failpoints.lock().unwrap().is_empty()
    }

    pub fn clear(&self) {
        self.events.lock().unwrap().clear();
        self.failpoints.lock().unwrap().clear();
    }
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Default Trait Implementation and Teardown )
// ==============================================================================
impl Default for DiagnosticLogger {
    fn default() -> Self {
        Self::new(false)
    }
}

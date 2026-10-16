// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Diagnostic Event Bus, Channel Stream & Live Terminal Mirroring )
// ==============================================================================
//! Diagnostic Event Bus & Telemetry Event Definitions

use std::sync::{Arc, Mutex};
use super::super::failpoint::{FailpointType, FixSuggestion};
use super::super::telemetry::CognitiveNote;

/// Individual diagnostic event recorded during execution ticks.
#[derive(Debug, Clone)]
pub struct LogEvent {
    /// Microseconds elapsed since logger initialization.
    pub elapsed_micros: u128,
    /// Channel or subsystem tag (e.g. "BOUNDARY", "DEBUGGER", "TERNARY").
    pub channel: &'static str,
    /// Descriptive message explaining the operation or signal.
    pub message: String,
    /// True if this event represents an intercepted failpoint or anomaly.
    pub is_failpoint: bool,
    /// The specific failpoint category if `is_failpoint` is true.
    pub failpoint_type: Option<FailpointType>,
    /// Concrete remediation suggestion if an automated repair is available.
    pub suggestion: Option<FixSuggestion>,
    /// Accompanying structured cognitive memory note for AI ingestion.
    pub cognitive_note: Option<CognitiveNote>,
}

/// Thread-safe event bus coordinating live diagnostic streams.
#[derive(Debug, Clone)]
pub struct EventBus {
    events: Arc<Mutex<Vec<LogEvent>>>,
    failpoints: Arc<Mutex<Vec<LogEvent>>>,
    print_live: bool,
}

impl EventBus {
    /// Constructs a new [`EventBus`].
    pub fn new(print_live: bool) -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
            failpoints: Arc::new(Mutex::new(Vec::new())),
            print_live,
        }
    }

    /// Emits a general trace event to the bus.
    pub fn emit(&self, event: LogEvent) {
        if self.print_live {
            if event.is_failpoint {
                let kind_str = event
                    .failpoint_type
                    .map(|f| f.as_str())
                    .unwrap_or("FAILPOINT");
                eprintln!(
                    "[{:>6} µs] \x1b[91mCRITICAL\x1b[0m [{}] [{}] {}",
                    event.elapsed_micros, event.channel, kind_str, event.message
                );
            } else {
                println!(
                    "[{:>6} µs] DEBUG    [{}] {}",
                    event.elapsed_micros, event.channel, event.message
                );
            }
        }

        if event.is_failpoint {
            self.failpoints.lock().unwrap().push(event.clone());
        }
        self.events.lock().unwrap().push(event);
    }

    /// Returns a cloned snapshot of all recorded events.
    pub fn all_events(&self) -> Vec<LogEvent> {
        self.events.lock().unwrap().clone()
    }

    /// Returns a cloned snapshot of all failpoints.
    pub fn all_failpoints(&self) -> Vec<LogEvent> {
        self.failpoints.lock().unwrap().clone()
    }

    /// Clears all events from the bus.
    pub fn clear(&self) {
        self.events.lock().unwrap().clear();
        self.failpoints.lock().unwrap().clear();
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new(false)
    }
}

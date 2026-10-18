// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Diagnostic Event Bus, Channel Routing, Subscriber Taps & Bounded Invariants )
// ==============================================================================
//! OmniCode Universal Diagnostics: Event Bus & Subsystem Telemetry Routing
//!
//! # Architectural Scope & Purpose:
//! The central nervous conduit of OmniCode Track 1. Receives, sequences, and routes
//! diagnostic trace events, critical failpoint interceptions, cognitive memory notes,
//! and hardware halt dumps across execution subsystems:
//! - [`LogEvent`]: Strongly typed diagnostic telemetry frame capturing timestamps, channels, and failpoints.
//! - [`EventBus`]: Thread-safe, bounded broadcast bus coordinating live diagnostic streams.
//! - [`EventTapCallback`]: Subscriber tap mechanism enabling live observation without polling.
//! - [`EventBusMetrics`]: Subsystem distribution telemetry and emission frequency metrics.
//!
//! # Scripture Witness:
//! - **1 Corinthians 14:40 (KJV)**:
//!   "Let all things be done decently and in order."
//!   (The requirement that telemetry events flow through deterministic, ordered channels without chaos).
//! - **2 Kings 4:4 (KJV)**:
//!   "And when thou art come in, thou shalt shut the door upon thee and upon thy sons,
//!    and shalt pour out into all those vessels, and thou shalt set aside that which is full."
//!   (The physical container law Occupancy <= Capacity applied to bounded in-memory queues).
//! - **Proverbs 18:13 (KJV)**:
//!   "He that answereth a matter before he heareth it, it is folly and shame unto him."
//!   (The principle of non-lossy sensory interception: the system must capture full telemetry before judging).
//!
//! # Why Extensive Commenting Matters for AI Agents & Developers:
//! 1. **Zero-Unbounded-Growth Invariant**: Diagnostic event queues must never cause Out-Of-Memory (OOM)
//!    panics. Both `events` and `failpoints` are strictly bounded by capacity ceilings with FIFO eviction.
//! 2. **Multi-Subscriber Taps**: External tools (live terminals, web dashboards, query indexers, test fixtures)
//!    can register non-intrusive taps to inspect events in real-time without modifying the core pipeline.
//! 3. **Subsystem Isolation**: Channels ("PARSER", "EVALUATOR", "SILICON", "COGNITION") allow distinct
//!    organs to emit into a single bus while preserving modular boundary provenance.
// ==============================================================================

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use super::super::failpoint::{FailpointType, FixSuggestion};
use super::super::telemetry::CognitiveNote;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Constants, Event Struct Definitions, Tap Callbacks & Bus Enums )
// ==============================================================================

/// Default maximum number of general log events retained in memory (8,192 events).
pub const DEFAULT_EVENT_BUS_CAPACITY: usize = 8192;

/// Default maximum number of critical failpoint events retained in memory (2,048 events).
pub const DEFAULT_FAILPOINT_BUS_CAPACITY: usize = 2048;

/// Type alias for thread-safe listener tap callbacks.
pub type EventTapCallback = Arc<dyn Fn(&LogEvent) + Send + Sync + 'static>;

/// Individual diagnostic telemetry event recorded during VM or hardware execution.
#[derive(Debug, Clone)]
pub struct LogEvent {
    /// Microseconds elapsed since logger initialization.
    pub elapsed_micros: u128,
    /// Subsystem or channel tag (e.g. "BOUNDARY", "PARSER", "SILICON", "COGNITION").
    pub channel: &'static str,
    /// Descriptive human-readable diagnostic message.
    pub message: String,
    /// True if this event represents an intercepted failpoint or pathology.
    pub is_failpoint: bool,
    /// Specific failpoint category if `is_failpoint` is true.
    pub failpoint_type: Option<FailpointType>,
    /// Concrete remediation suggestion if an automated repair was synthesized.
    pub suggestion: Option<FixSuggestion>,
    /// Accompanying structured cognitive memory note for AI Cortex ingestion.
    pub cognitive_note: Option<CognitiveNote>,
}

/// Aggregated telemetry metrics reflecting bus emission volume and channel distribution.
#[derive(Debug, Clone, Default)]
pub struct EventBusMetrics {
    /// Total events emitted to the bus across its lifetime.
    pub total_emitted: usize,
    /// Total failpoints intercepted across its lifetime.
    pub total_failpoints: usize,
    /// Total events currently held in memory.
    pub active_events_count: usize,
    /// Total failpoints currently held in memory.
    pub active_failpoints_count: usize,
    /// Distribution count of events keyed by channel name.
    pub channel_distribution: HashMap<&'static str, usize>,
}

/// Thread-safe diagnostic event bus coordinating live telemetry streams and subscriber taps.
#[derive(Clone)]
pub struct EventBus {
    events: Arc<Mutex<VecDeque<LogEvent>>>,
    failpoints: Arc<Mutex<VecDeque<LogEvent>>>,
    taps: Arc<Mutex<Vec<EventTapCallback>>>,
    max_events_capacity: usize,
    max_failpoints_capacity: usize,
    total_emitted: Arc<Mutex<usize>>,
    total_failpoints: Arc<Mutex<usize>>,
    channel_counts: Arc<Mutex<HashMap<&'static str, usize>>>,
    print_live: bool,
    color_output: bool,
}

impl std::fmt::Debug for EventBus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventBus")
            .field("max_events_capacity", &self.max_events_capacity)
            .field("max_failpoints_capacity", &self.max_failpoints_capacity)
            .field("print_live", &self.print_live)
            .field("color_output", &self.color_output)
            .finish()
    }
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Event Constructors, Bus Implementations, Invariant Enforcement & Formatting )
// ==============================================================================

impl LogEvent {
    /// Constructs a standard diagnostic trace or debug event.
    pub fn trace(elapsed_micros: u128, channel: &'static str, message: impl Into<String>) -> Self {
        Self {
            elapsed_micros,
            channel,
            message: message.into(),
            is_failpoint: false,
            failpoint_type: None,
            suggestion: None,
            cognitive_note: None,
        }
    }

    /// Constructs an intercepted critical failpoint event.
    pub fn failpoint(
        elapsed_micros: u128,
        channel: &'static str,
        message: impl Into<String>,
        fp_type: FailpointType,
        suggestion: Option<FixSuggestion>,
    ) -> Self {
        Self {
            elapsed_micros,
            channel,
            message: message.into(),
            is_failpoint: true,
            failpoint_type: Some(fp_type),
            suggestion,
            cognitive_note: None,
        }
    }

    /// Constructs an event carrying an episodic cognitive memory note.
    pub fn cognitive(
        elapsed_micros: u128,
        channel: &'static str,
        message: impl Into<String>,
        note: CognitiveNote,
    ) -> Self {
        Self {
            elapsed_micros,
            channel,
            message: message.into(),
            is_failpoint: false,
            failpoint_type: None,
            suggestion: None,
            cognitive_note: Some(note),
        }
    }

    /// Formats the event for terminal presentation with optional ANSI color codes.
    pub fn format_terminal(&self, color: bool) -> String {
        if self.is_failpoint {
            let kind_str = self
                .failpoint_type
                .map(|f| f.as_str())
                .unwrap_or("FAILPOINT");
            if color {
                format!(
                    "[{:>6} µs] \x1b[91mCRITICAL\x1b[0m [{:<10}] [{:<22}] {}",
                    self.elapsed_micros, self.channel, kind_str, self.message
                )
            } else {
                format!(
                    "[{:>6} µs] CRITICAL [{:<10}] [{:<22}] {}",
                    self.elapsed_micros, self.channel, kind_str, self.message
                )
            }
        } else if color {
            format!(
                "[{:>6} µs] \x1b[92mDEBUG\x1b[0m    [{:<10}] {}",
                self.elapsed_micros, self.channel, self.message
            )
        } else {
            format!(
                "[{:>6} µs] DEBUG    [{:<10}] {}",
                self.elapsed_micros, self.channel, self.message
            )
        }
    }
}

impl EventBus {
    /// Constructs a new [`EventBus`] with default bounded capacities.
    pub fn new(print_live: bool) -> Self {
        Self::with_capacity(
            print_live,
            DEFAULT_EVENT_BUS_CAPACITY,
            DEFAULT_FAILPOINT_BUS_CAPACITY,
        )
    }

    /// Constructs a new [`EventBus`] with custom capacity bounds.
    pub fn with_capacity(
        print_live: bool,
        max_events_capacity: usize,
        max_failpoints_capacity: usize,
    ) -> Self {
        Self {
            events: Arc::new(Mutex::new(VecDeque::with_capacity(
                max_events_capacity.min(256),
            ))),
            failpoints: Arc::new(Mutex::new(VecDeque::with_capacity(
                max_failpoints_capacity.min(64),
            ))),
            taps: Arc::new(Mutex::new(Vec::new())),
            max_events_capacity: max_events_capacity.max(1),
            max_failpoints_capacity: max_failpoints_capacity.max(1),
            total_emitted: Arc::new(Mutex::new(0)),
            total_failpoints: Arc::new(Mutex::new(0)),
            channel_counts: Arc::new(Mutex::new(HashMap::new())),
            print_live,
            color_output: true,
        }
    }

    /// Enables or disables ANSI color codes for live terminal printing.
    pub fn set_color_output(&mut self, enable: bool) {
        self.color_output = enable;
    }

    /// Registers a thread-safe listener tap that receives all emitted events synchronously.
    pub fn register_tap(&self, tap: EventTapCallback) {
        self.taps.lock().unwrap().push(tap);
    }

    /// Emits a diagnostic event to the bus, notifying taps, printing live if configured,
    /// and retaining it in bounded memory queues.
    pub fn emit(&self, event: LogEvent) {
        // 1. Live terminal printing
        if self.print_live {
            let formatted = event.format_terminal(self.color_output);
            if event.is_failpoint {
                eprintln!("{formatted}");
            } else {
                println!("{formatted}");
            }
        }

        // 2. Invoke registered listener taps
        {
            let taps_guard = self.taps.lock().unwrap();
            for tap in taps_guard.iter() {
                tap(&event);
            }
        }

        // 3. Update emission metrics
        {
            let mut total_emitted = self.total_emitted.lock().unwrap();
            *total_emitted += 1;

            let mut counts = self.channel_counts.lock().unwrap();
            *counts.entry(event.channel).or_insert(0) += 1;
        }

        // 4. Ingest failpoint queue with bounded FIFO capacity
        if event.is_failpoint {
            let mut total_fp = self.total_failpoints.lock().unwrap();
            *total_fp += 1;

            let mut fps = self.failpoints.lock().unwrap();
            if fps.len() >= self.max_failpoints_capacity {
                fps.pop_front();
            }
            fps.push_back(event.clone());
        }

        // 5. Ingest general event queue with bounded FIFO capacity
        {
            let mut evs = self.events.lock().unwrap();
            if evs.len() >= self.max_events_capacity {
                evs.pop_front();
            }
            evs.push_back(event);
        }
    }

    /// Returns a cloned snapshot of all currently retained general log events.
    pub fn all_events(&self) -> Vec<LogEvent> {
        self.events.lock().unwrap().iter().cloned().collect()
    }

    /// Returns a cloned snapshot of all currently retained failpoint events.
    pub fn all_failpoints(&self) -> Vec<LogEvent> {
        self.failpoints.lock().unwrap().iter().cloned().collect()
    }

    /// Filters and returns events matching a specific subsystem channel.
    pub fn events_by_channel(&self, channel: &str) -> Vec<LogEvent> {
        self.events
            .lock()
            .unwrap()
            .iter()
            .filter(|e| e.channel == channel)
            .cloned()
            .collect()
    }

    /// Filters and returns events occurring within a microsecond timestamp window `[start, end]`.
    pub fn events_by_time_range(&self, start_micros: u128, end_micros: u128) -> Vec<LogEvent> {
        self.events
            .lock()
            .unwrap()
            .iter()
            .filter(|e| e.elapsed_micros >= start_micros && e.elapsed_micros <= end_micros)
            .cloned()
            .collect()
    }

    /// Retrieves an aggregated metrics snapshot of the event bus.
    pub fn metrics(&self) -> EventBusMetrics {
        let total_emitted = *self.total_emitted.lock().unwrap();
        let total_failpoints = *self.total_failpoints.lock().unwrap();
        let active_events_count = self.events.lock().unwrap().len();
        let active_failpoints_count = self.failpoints.lock().unwrap().len();
        let channel_distribution = self.channel_counts.lock().unwrap().clone();

        EventBusMetrics {
            total_emitted,
            total_failpoints,
            active_events_count,
            active_failpoints_count,
            channel_distribution,
        }
    }

    /// Returns the number of events currently buffered in memory.
    pub fn len(&self) -> usize {
        self.events.lock().unwrap().len()
    }

    /// Returns `true` if the event buffer has no active events.
    pub fn is_empty(&self) -> bool {
        self.events.lock().unwrap().is_empty()
    }

    /// Returns the number of failpoint events currently buffered in memory.
    pub fn failpoint_count(&self) -> usize {
        self.failpoints.lock().unwrap().len()
    }

    /// Clears all currently retained events and failpoints from memory queues.
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

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Test Verification Proofs, Bounded Capacity & Tap Tests )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;

    #[test]
    fn test_event_bus_basic_emission_and_retrieval() {
        let bus = EventBus::new(false);
        assert!(bus.is_empty());
        assert_eq!(bus.len(), 0);

        bus.emit(LogEvent::trace(100, "PARSER", "Token stream initialized"));
        bus.emit(LogEvent::trace(150, "PARSER", "First token scanned"));
        bus.emit(LogEvent::failpoint(
            200,
            "BOUNDARY",
            "Unclosed bracket delimiter '<'",
            FailpointType::BoundaryLeak,
            None,
        ));

        assert_eq!(bus.len(), 3);
        assert_eq!(bus.failpoint_count(), 1);

        let all = bus.all_events();
        assert_eq!(all.len(), 3);
        assert_eq!(all[0].channel, "PARSER");
        assert!(!all[0].is_failpoint);
        assert!(all[2].is_failpoint);

        let fps = bus.all_failpoints();
        assert_eq!(fps.len(), 1);
        assert_eq!(fps[0].failpoint_type, Some(FailpointType::BoundaryLeak));

        bus.clear();
        assert!(bus.is_empty());
        assert_eq!(bus.failpoint_count(), 0);
    }

    #[test]
    fn test_bounded_capacity_fifo_eviction() {
        // Enforce a strict bound of 3 events and 2 failpoints
        let bus = EventBus::with_capacity(false, 3, 2);

        for i in 1..=5 {
            bus.emit(LogEvent::trace(
                i * 10,
                "EXEC",
                format!("Step {i}"),
            ));
        }

        // Must not exceed max capacity of 3
        assert_eq!(bus.len(), 3);
        let events = bus.all_events();
        assert_eq!(events[0].message, "Step 3");
        assert_eq!(events[1].message, "Step 4");
        assert_eq!(events[2].message, "Step 5");

        // Emit 3 failpoints into capacity of 2
        for i in 1..=3 {
            bus.emit(LogEvent::failpoint(
                i * 100,
                "FAULT",
                format!("Fault {i}"),
                FailpointType::HardwareFault,
                None,
            ));
        }

        assert_eq!(bus.failpoint_count(), 2);
        let fps = bus.all_failpoints();
        assert_eq!(fps[0].message, "Fault 2");
        assert_eq!(fps[1].message, "Fault 3");
    }

    #[test]
    fn test_channel_filtering_and_time_range_scans() {
        let bus = EventBus::new(false);
        bus.emit(LogEvent::trace(100, "SILICON", "Reset landmark 0x7C00"));
        bus.emit(LogEvent::trace(200, "CORTEX", "Awakening neural tensor"));
        bus.emit(LogEvent::trace(300, "SILICON", "A20 gate active 0x18000"));
        bus.emit(LogEvent::trace(400, "CORTEX", "Episodic memory synchronized"));

        let silicon = bus.events_by_channel("SILICON");
        assert_eq!(silicon.len(), 2);
        assert_eq!(silicon[0].elapsed_micros, 100);
        assert_eq!(silicon[1].elapsed_micros, 300);

        let cortex = bus.events_by_channel("CORTEX");
        assert_eq!(cortex.len(), 2);

        let range = bus.events_by_time_range(150, 350);
        assert_eq!(range.len(), 2);
        assert_eq!(range[0].channel, "CORTEX");
        assert_eq!(range[1].channel, "SILICON");
    }

    #[test]
    fn test_subscriber_tap_callback() {
        let bus = EventBus::new(false);
        let tap_count = Arc::new(AtomicUsize::new(0));

        let tap_count_clone = Arc::clone(&tap_count);
        bus.register_tap(Arc::new(move |_event| {
            tap_count_clone.fetch_add(1, Ordering::SeqCst);
        }));

        bus.emit(LogEvent::trace(10, "TAP", "Message 1"));
        bus.emit(LogEvent::trace(20, "TAP", "Message 2"));
        bus.emit(LogEvent::trace(30, "TAP", "Message 3"));

        assert_eq!(tap_count.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_metrics_aggregation() {
        let bus = EventBus::new(false);
        bus.emit(LogEvent::trace(10, "PARSER", "P1"));
        bus.emit(LogEvent::trace(20, "PARSER", "P2"));
        bus.emit(LogEvent::trace(30, "EVAL", "E1"));
        bus.emit(LogEvent::failpoint(
            40,
            "FAULT",
            "F1",
            FailpointType::ImmutableMutation,
            None,
        ));

        let m = bus.metrics();
        assert_eq!(m.total_emitted, 4);
        assert_eq!(m.total_failpoints, 1);
        assert_eq!(m.active_events_count, 4);
        assert_eq!(m.active_failpoints_count, 1);
        assert_eq!(m.channel_distribution.get("PARSER"), Some(&2));
        assert_eq!(m.channel_distribution.get("EVAL"), Some(&1));
        assert_eq!(m.channel_distribution.get("FAULT"), Some(&1));
    }

    #[test]
    fn test_concurrent_multithreaded_emission() {
        let bus = EventBus::new(false);
        let mut handles = Vec::new();

        for t in 0..4 {
            let bus_clone = bus.clone();
            handles.push(thread::spawn(move || {
                for i in 0..50 {
                    bus_clone.emit(LogEvent::trace(
                        (t * 100 + i) as u128,
                        "THREAD",
                        format!("Thread {t} Event {i}"),
                    ));
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(bus.len(), 200);
        let m = bus.metrics();
        assert_eq!(m.total_emitted, 200);
    }

    #[test]
    fn test_terminal_formatting_plain_and_color() {
        let trace_event = LogEvent::trace(142, "PARSER", "Syntax OK");
        let plain_trace = trace_event.format_terminal(false);
        assert!(plain_trace.contains("[   142 µs] DEBUG    [PARSER    ] Syntax OK"));

        let fail_event = LogEvent::failpoint(
            890,
            "BOUNDARY",
            "Missing tag",
            FailpointType::BoundaryMismatch,
            None,
        );
        let plain_fail = fail_event.format_terminal(false);
        assert!(plain_fail.contains("[   890 µs] CRITICAL [BOUNDARY  ] [FP_BOUNDARY_MISMATCH  ] Missing tag"));
    }
}

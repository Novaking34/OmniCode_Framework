# Universal Telemetry, Monotonic Clock & Event Bus Subsystem

## The Sensory Foundation of OmniCode Diagnostics

> *"To every thing there is a season, and a time to every purpose under the heaven."*  
> — **Ecclesiastes 3:1 (KJV)**  
>
> *"Ponder the path of thy feet, and let all thy ways be established."*  
> — **Proverbs 4:26 (KJV)**  
>
> *"Let all things be done decently and in order."*  
> — **1 Corinthians 14:40 (KJV)**  
>
> *"And when thou art come in, thou shalt shut the door upon thee and upon thy sons, and shalt pour out into all those vessels, and thou shalt set aside that which is full."*  
> — **2 Kings 4:4 (KJV)**  

---

## 1. Architectural Philosophy: The Spinal Cord & Internal Clock

In **OmniCode Track 1 (Host Engine & Virtual Machine)**, diagnostic telemetry cannot rely on arbitrary string printing or non-monotonic system clocks. An autonomous operating system and its AI Cortex require two non-negotiable physical invariants:

1. **A Strictly Monotonic Clock ($\Delta t \ge 0$):** Causal reasoning demands that an effect must never chronologically precede its cause. System wall clocks (`SystemTime`) are vulnerable to NTP time-steps, VM migrations, and leap-second adjustments that induce negative time deltas. The Universal Clock guarantees strict temporal monotonicity across all execution ticks.
2. **A Deterministic Sensory Event Bus:** Subsystems—from Track 0 bare-metal silicon interrupts to the high-level linguistic parser and cognitive reasoning cortex—must emit signals into a unified, bounded, thread-safe nervous conduit without taking intrusive locks on the critical path.

The `universal` module provides this dual foundation:

```text
================================================================================
                    UNIVERSAL DIAGNOSTICS ARCHITECTURE
================================================================================

   Track 0 Bare-Metal Silicon          Virtual Machine Execution Rolls
   [ RDTSC / BIOS / Landmarks ]         [ Parser / Evaluator / Cortex ]
                │                                     │
                ▼                                     ▼
   ┌───────────────────────────┐         ┌───────────────────────────┐
   │      MonotonicClock       │         │      SimulatedClock       │
   │  • Microsecond Precision  │         │  • Deterministic Stepping │
   │  • TSC Cycle Translation  │         │  • Discrete Tick Freezing │
   │  • Monotonic ID Generator │         │  • Bitwise Replay Bounds  │
   └─────────────┬─────────────┘         └─────────────┬─────────────┘
                 │                                     │
                 └──────────────────┬──────────────────┘
                                    │
                        [ Monotonic µs Offsets ]
                                    │
                                    ▼
                 ┌─────────────────────────────────────┐
                 │              EventBus               │
                 │  • Bounded FIFO Queues (2 Kings 4:4)│
                 │  • Multi-Channel Subsystem Routing  │
                 │  • Non-Blocking Subscriber Taps     │
                 │  • ANSI Terminal Formatting         │
                 └──────────────────┬──────────────────┘
                                    │
        ┌───────────────────────────┼───────────────────────────┐
        ▼                           ▼                           ▼
  [ LIVE TUI / LOGS ]       [ CRITICAL FAILPOINTS ]     [ AI CORTEX TAPS ]
  Human-Readable ANSI       Remediation & Halt Traps    Episodic Memory Feeds
================================================================================
```

---

## 2. Directory Topology & The 4-Block Structural Standard

Every source file in the `universal` module strictly implements the **OmniCode 4-Block Architecture**:

| Block Index | Canonical Block Title | Architectural Scope & Invariant Duties |
| :--- | :--- | :--- |
| **Block 1** | **METADATA BLOCK** | Subsystem identity, module docstrings, Scripture witnesses, architectural scope, and external crate imports. |
| **Block 2** | **OPENING BLOCK** | Structural types, telemetry models, bounded capacity constants, and channel definitions. |
| **Block 3** | **BODY BLOCK** | Implementations, monotonic sequencing, bounded FIFO eviction logic, cycle calculations, and formatters. |
| **Block 4** | **CLOSING BLOCK** | Comprehensive unit test suites, invariant verification proofs, and multithreaded stress assertions. |

### Module File Layout

```text
src/diagnostics/logger/universal/
├── README.md                                          <-- Subsystem Manifesto & Reference Guide
├── CAUSAL_MONOTONICITY_AND_SILICON_CLOCK_PHYSICS.md   <-- Deep Treatise: Monotonic Time & TSC Calibration
├── EVENT_BUS_BROADCAST_AND_SUBSCRIBER_ARCHITECTURE.md <-- Deep Treatise: Event Routing & Subscriber Taps
├── mod.rs                                             <-- Subsystem Facade & Re-exports (4-Block)
├── clock.rs                                           <-- Monotonic & Simulated Clocks (4-Block)
└── bus.rs                                             <-- Bounded Event Bus & Telemetry Routing (4-Block)
```

---

## 3. High-Precision Timing Machinery (`clock.rs`)

### Real-Time Monotonic Clock (`MonotonicClock`)

Backed by `std::time::Instant`, `MonotonicClock` measures elapsed durations since subsystem initialization with microsecond and nanosecond precision.

* **Nanosecond Offset:** `clock.elapsed_nanos() -> u128`
* **Microsecond Offset:** `clock.elapsed_micros() -> u128`
* **Floating-Point Milliseconds:** `clock.elapsed_millis() -> f64`

### Track 0 Silicon Cycle Translation

Bare-metal hardware emitted by Track 0 logs timestamp values in raw CPU cycles via the x86 `RDTSC` instruction. `MonotonicClock` bridges raw silicon cycles and wall durations using calibrated frequency constants:

$$\text{Cycles} = \text{Microseconds} \times \text{MHz}$$

$$\text{Microseconds} = \frac{\text{Cycles}}{\text{MHz}}$$

```rust
use omnicode::diagnostics::logger::universal::MonotonicClock;

let mhz = 2000; // 2.0 GHz nominal baseline
let micros = 500;
let cycles = MonotonicClock::micros_to_cycles(micros, mhz);
assert_eq!(cycles, 1_000_000);

let recovered_micros = MonotonicClock::cycles_to_micros(cycles, mhz);
assert_eq!(recovered_micros, 500);
```

### Strictly Monotonic Timestamp Identifiers (`generate_timestamp_id`)

The `generate_timestamp_id()` function generates unique, monotonically non-decreasing identifiers formatted:

```text
<unix_epoch_micros>_<atomic_counter:04>
```

To guard against NTP wall-clock regressions or host time jumps, the generator tracks the maximum observed epoch timestamp in an internal atomic mutex: if `wall_clock < last_observed`, the generator anchors to `last_observed` while incrementing the sequence counter, ensuring strict temporal progression ($\Delta t \ge 0$).

### Deterministic Simulation Clock (`SimulatedClock`)

For automated unit testing, debugging, and bitwise-reproducible virtual machine execution, real-world clock jitter causes race conditions. `SimulatedClock` decouples execution from real time:

* **Discrete Microsecond Stepping:** `sim.step_micros(delta)`
* **Discrete VM Tick Stepping:** `sim.step_ticks(ticks, micros_per_tick)`
* **Clock Freezing:** `sim.freeze()` halts simulated time progression until `sim.resume()` is invoked.

---

## 4. The Diagnostic Event Bus Subsystem (`bus.rs`)

### Strongly Typed Telemetry Frames (`LogEvent`)

Rather than unstructured string buffers, all telemetry emitted to the bus is packaged into strongly typed `LogEvent` frames:

```rust
pub struct LogEvent {
    pub elapsed_micros: u128,
    pub channel: &'static str,
    pub message: String,
    pub is_failpoint: bool,
    pub failpoint_type: Option<FailpointType>,
    pub suggestion: Option<FixSuggestion>,
    pub cognitive_note: Option<CognitiveNote>,
}
```

### Physical Container Law: Bounded FIFO Eviction (2 Kings 4:4)

In accordance with Scripture and physical system laws, unbounded memory growth is strictly prohibited (`INV-UNI-01`).

* `DEFAULT_EVENT_BUS_CAPACITY = 8,192` events.
* `DEFAULT_FAILPOINT_BUS_CAPACITY = 2,048` failpoints.

When occupancy reaches maximum capacity ($\text{Occupancy} = \text{Capacity}$), the bus evicts the oldest event (`pop_front()`) before appending the new event (`push_back()`), guaranteeing bounded memory overhead ($O(1)$ amortized).

### Subscriber Tap Architecture (`register_tap`)

External tools (such as live visual consoles, terminal dashboards, episodic query indexers, and automated test runners) can register synchronous callbacks without altering the logging pipeline:

```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use omnicode::diagnostics::logger::universal::{EventBus, LogEvent};

let bus = EventBus::new(false);
let counter = Arc::new(AtomicUsize::new(0));

let counter_clone = Arc::clone(&counter);
bus.register_tap(Arc::new(move |_event: &LogEvent| {
    counter_clone.fetch_add(1, Ordering::SeqCst);
}));

bus.emit(LogEvent::trace(100, "PARSER", "Token stream initialized"));
assert_eq!(counter.load(Ordering::SeqCst), 1);
```

### Channel Routing & Temporal Range Scanning

Events can be filtered instantaneously by functional subsystem channel or bounded temporal range:

```rust
// Filter all events emitted by the SILICON channel
let silicon_events = bus.events_by_channel("SILICON");

// Filter events between 100 µs and 500 µs
let time_slice = bus.events_by_time_range(100, 500);
```

---

## 5. Public API Usage Guide

### A. Initializing a Calibrated Telemetry Pair

```rust
use omnicode::diagnostics::logger::universal::create_universal_telemetry_pair;

let (clock, bus) = create_universal_telemetry_pair(false);
let start = clock.elapsed_micros();

bus.emit(omnicode::diagnostics::logger::universal::LogEvent::trace(
    start,
    "INIT",
    "Subsystem brought online cleanly",
));
```

### B. Emitting Critical Failpoints with Machine-Actionable Remediation

```rust
use omnicode::diagnostics::logger::failpoint::{FailpointType, FixKind, FixSuggestion};
use omnicode::diagnostics::logger::universal::{EventBus, LogEvent};

let bus = EventBus::new(false);

let suggestion = FixSuggestion {
    kind: FixKind::ReplaceDelimiter,
    line: 12,
    col: 4,
    target_token: "]".to_string(),
    suggested_patch: ">".to_string(),
    rationale: "Balance Rank 1 invariant delimiter".to_string(),
};

bus.emit(LogEvent::failpoint(
    450,
    "BOUNDARY",
    "Delimiter hierarchy mismatch '<' closed by ']'",
    FailpointType::BoundaryMismatch,
    Some(suggestion),
));

assert_eq!(bus.failpoint_count(), 1);
```

### C. Extracting Telemetry Metrics

```rust
let metrics = bus.metrics();
println!("Total Events Emitted: {}", metrics.total_emitted);
println!("Active Failpoints In Buffer: {}", metrics.active_failpoints_count);
for (channel, count) in metrics.channel_distribution {
    println!("Channel [{channel}]: {count} events");
}
```

---

## 6. Covenantal Invariant Matrix

| Invariant ID | Covenantal Rule | Verification Mechanism |
| :--- | :--- | :--- |
| **INV-UNI-01** | **Bounded Memory Occupancy**: In-memory event buffers must never grow without bound. Oldest entries must be evicted via FIFO at capacity ceiling. | `bus.len() <= max_capacity` enforced in `emit()`. Validated by unit test `test_bounded_capacity_fifo_eviction`. |
| **INV-UNI-02** | **Strict Causal Monotonicity**: Timestamp identifiers and elapsed time offsets must be non-decreasing ($\Delta t \ge 0$). | Validated by atomic counter and `LAST_MICROS` floor in `generate_timestamp_id()`. |
| **INV-UNI-03** | **Deterministic Simulation Bounds**: When `SimulatedClock` is frozen, no time or tick progression may occur without explicit unfreezing. | Validated by unit test `test_simulated_clock_lifecycle_and_freeze`. |
| **INV-UNI-04** | **Non-Intrusive Tap Invariance**: Registering a subscriber tap must never drop or mutate events on the primary bus. | Validated by unit test `test_subscriber_tap_callback`. |
| **INV-UNI-05** | **Channel Provenance**: Every event must declare a static string channel identifying its originating machine organ. | Enforced by `channel: &'static str` on `LogEvent`. |
| **INV-UNI-06** | **Zero Regressions**: All 13 unit tests across `clock.rs`, `bus.rs`, and `mod.rs` must pass with zero warnings. | Verified via `cargo test diagnostics::logger::universal`. |

---

## 7. Subsystem Navigation Sitemap

* [Root Architectural Manifesto](../../README.md)
* [Logger Subsystem Hub](../README.md)
* [Failpoint Diagnostic Subsystem](../failpoint/README.md)
* [Container Inscriber Subsystem](../inscriber/README.md)
* [Episodic Query Engine](../query/README.md)
* [Monotonic Time & Silicon Clock Physics](CAUSAL_MONOTONICITY_AND_SILICON_CLOCK_PHYSICS.md)
* [Event Bus Routing & Subscriber Architecture](EVENT_BUS_BROADCAST_AND_SUBSCRIBER_ARCHITECTURE.md)

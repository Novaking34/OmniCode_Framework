# Event Bus Broadcast & Subscriber Tap Architecture

## The Sensory Spinal Cord of the DAR Cybernetic Loop

> *"Let all things be done decently and in order."*
> — **1 Corinthians 14:40 (KJV)**
>
> *"And when thou art come in, thou shalt shut the door upon thee and upon thy sons, and shalt pour out into all those vessels, and thou shalt set aside that which is full."*
> — **2 Kings 4:4 (KJV)**
>
> *"He that answereth a matter before he heareth it, it is folly and shame unto him."*
> — **Proverbs 18:13 (KJV)**

---

## 1. Architectural Scope: The Sensory Spinal Cord

In the **DAR Triad (Detect, Assess, Restore)**:

* The **Logger** is the **DETECT** organ.
* The **Debugger** is the **ASSESS** organ.
* The **Restorator** is the **RESTORE** organ.

Before an invariant breach can be assessed or restored, it must first be **sensed**. If telemetry is trapped in isolated subsystem silos—where the parser prints to standard error, the evaluator panics to standard output, and bare-metal silicon writes to an unchecked serial buffer—the AI Cortex cannot form a coherent world model.

The **Universal Event Bus (`EventBus`)** serves as the **sensory spinal cord** of OmniCode Track 1. It gathers every pulse, roll, failpoint, and cognitive trace across the entire operating system into a unified, bounded, thread-safe transmission channel.

```text
================================================================================
                    THE EVENT BUS SENSORY SPINAL CORD
================================================================================

   [ PRODUCERS: Execution Subsystems ]
   • SILICON   : Bare-metal landmarks (0x7C00, 0x18000) & CPU register states
   • PARSER    : Token stream generation & delimiter balancing
   • BOUNDARY  : Morphological scope boundaries (<...>, [...], {...}, (...))
   • EVALUATOR : Dual-rail ternary arithmetic & roll execution
   • COGNITION : AI thoughts, hypothesis testing, and memory associations
   • HALT      : Saturated (1, 1) state collapse & hardware re-inceptions
                      │
                      ▼
   ┌────────────────────────────────────────────────────────────────────────┐
   │                       EventBus (Thread-Safe Arc)                       │
   │  • Enforces Container Law: Occupancy <= Capacity (FIFO Eviction)       │
   │  • Microsecond Timestamp Ordering                                      │
   │  • Subsystem Channel Isolation                                         │
   └──────────────────────────────────┬─────────────────────────────────────┘
                                      │
         ┌────────────────────────────┼────────────────────────────┐
         ▼                            ▼                            ▼
   [ LIVE TAPS ]             [ BOUNDED QUEUES ]          [ METRICS MONITOR ]
   • Visual TUI Mirror       • General Events (8,192)    • Total Emission Counter
   • Test Assertions         • Failpoints (2,048)        • Channel Distribution
   • AI Memory Feed          • Temporal Range Scans      • Failpoint Frequencies
================================================================================
```

---

## 2. Subsystem Channel Topology

To prevent telemetry pollution while ensuring that every event preserves its morphological origin, `LogEvent` requires a static channel identifier (`channel: &'static str`).

| Channel Tag | Subsystem Source | Typical Emitted Telemetry |
| :--- | :--- | :--- |
| **`"SILICON"`** | Track 0 Boot & Hardware | Landmark transitions (`0x7C00`, `0x18000`), A20 gate, 13 silicon registers, MMIO interrupts. |
| **`"PARSER"`** | Lexical & Syntax Engine | Token scanning, indentation checks, 4-block header validation. |
| **`"BOUNDARY"`** | Structural Guardrails | Asymmetric delimiter nesting, scope rank violations (`<` vs `[` vs `{` vs `(`). |
| **`"EVALUATOR"`** | Virtual Machine Core | Instruction execution, dual-rail input/output bitplanes, rollback trigger events. |
| **`"COGNITION"`** | AI Cortex & Memory | Hypothesis formulation, associative scripture anchors, reasoning trails. |
| **`"HALT"`** | Safety Supervisor | Emergency dual-rail `(1, 1)` saturation halts, crash dumps, and safe-mode re-inceptions. |

### Channel Isolation Queries

Subsystems query the bus directly for domain-specific event streams without iterating over foreign telemetry:

```rust
use omnicode::diagnostics::logger::universal::EventBus;

let bus = EventBus::new(false);
// ... telemetry emitted across all channels ...

// Extract only silicon telemetry for boot verification
let silicon_stream = bus.events_by_channel("SILICON");

// Extract only cognitive memory traces for AI digestion
let cognitive_stream = bus.events_by_channel("COGNITION");
```

---

## 3. Physical Container Law & Bounded Memory Invariants

### The Out-Of-Memory Logging Pathology

In conventional logging frameworks, log buffers grow indefinitely in memory until a background worker writes them to disk. Under high-throughput conditions (e.g. an infinite rollback loop or a high-frequency sensor stream), unbounded log accumulation exhausts system RAM, triggering the Linux OOM killer and abruptly terminating the operating system.

### Scriptural Grounding: 2 Kings 4:4

OmniCode logging enforces physical container physics as an absolute invariant:

> *"And when thou art come in, thou shalt shut the door upon thee and upon thy sons, and shalt pour out into all those vessels, and thou shalt set aside that which is full."*
> — **2 Kings 4:4 (KJV)**

A container that is full must be set aside; it must never be allowed to overflow into unallocated memory.

### FIFO Eviction Mechanics ($O(1)$)

Both general diagnostic events and critical failpoints are managed in bounded circular double-ended queues (`VecDeque`):

* **General Diagnostic Events:** Bounded by `DEFAULT_EVENT_BUS_CAPACITY = 8,192` events.
* **Critical Failpoints:** Bounded by `DEFAULT_FAILPOINT_BUS_CAPACITY = 2,048` failpoints.

```rust
// Internal ingestion inside EventBus::emit()
let mut evs = self.events.lock().unwrap();
if evs.len() >= self.max_events_capacity {
    evs.pop_front(); // Evict oldest event in O(1)
}
evs.push_back(event); // Ingest new event
```

**Guarantees:**

1. **Bounded Heap Space:** Total memory occupied by the event bus is strictly bounded by a constant $O(C_{\max})$, regardless of whether the operating system runs for 10 milliseconds or 1,000 years.
2. **Freshness Invariant:** The bus continuously maintains the most recent execution window leading up to an incident, preserving high-resolution diagnostic context.

---

## 4. Non-Blocking Subscriber Tap Architecture

In traditional architectures, attaching observers (like a live terminal dashboard, a debugger, or an AI reasoning agent) requires either:

* Polling the bus in a busy-wait loop (wasting CPU cycles), or
* Invasive hooks into the core logging path (introducing locks and latency).

`EventBus` provides a **non-intrusive Subscriber Tap Architecture** via `register_tap`:

```rust
pub type EventTapCallback = Arc<dyn Fn(&LogEvent) + Send + Sync + 'static>;
```

### Tap Registration Pattern

```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use omnicode::diagnostics::logger::universal::{EventBus, LogEvent};

let bus = EventBus::new(false);
let alert_counter = Arc::new(AtomicUsize::new(0));

// Register a tap that monitors for critical failpoints
let counter_clone = Arc::clone(&alert_counter);
bus.register_tap(Arc::new(move |event: &LogEvent| {
    if event.is_failpoint {
        counter_clone.fetch_add(1, Ordering::SeqCst);
    }
}));

// Any subsystem emitting to the bus automatically alerts the tap
bus.emit(LogEvent::trace(100, "INIT", "Boot sequence began"));
assert_eq!(alert_counter.load(Ordering::SeqCst), 0);

bus.emit(LogEvent::failpoint(
    150,
    "BOUNDARY",
    "Missing closing bracket",
    omnicode::diagnostics::logger::failpoint::FailpointType::BoundaryLeak,
    None,
));
assert_eq!(alert_counter.load(Ordering::SeqCst), 1);
```

### Architectural Properties of Taps

1. **Synchronous Invocation:** Taps are invoked immediately during `emit()`, guaranteeing that real-time monitors receive events before control returns to the caller.
2. **Thread Safety:** Taps implement `Send + Sync + 'static`, enabling multi-threaded dispatch without race conditions.
3. **Decoupled Isolation:** Taps receive immutable references (`&LogEvent`); they can never mutate, corrupt, or delay events destined for other consumers.

---

## 5. Telemetry Metrics Aggregation

The `EventBus` continuously tracks operational metrics to support real-time bottleneck detection and subsystem health auditing:

```rust
pub struct EventBusMetrics {
    pub total_emitted: usize,
    pub total_failpoints: usize,
    pub active_events_count: usize,
    pub active_failpoints_count: usize,
    pub channel_distribution: HashMap<&'static str, usize>,
}
```

### Example Usage: Subsystem Distribution Audit

```rust
let metrics = bus.metrics();

println!("=== Event Bus Operational Health ===");
println!("Lifetime Events Emitted:     {}", metrics.total_emitted);
println!("Lifetime Failpoints Trapped: {}", metrics.total_failpoints);
println!("Current Active Buffer Depth: {} / 8192", metrics.active_events_count);

for (channel, count) in metrics.channel_distribution {
    let percentage = (count as f64 / metrics.total_emitted as f64) * 100.0;
    println!("  • Channel [{:<10}]: {:>6} events ({:>5.1}%)", channel, count, percentage);
}
```

---

## 6. Dual-Readability Presentation (ANSI vs Structured)

The event bus formats telemetry for both human eyes and machine consumption:

### Human ANSI Presentation (`format_terminal`)

```text
[   142 µs] DEBUG    [PARSER    ] Token stream initialized successfully
[   890 µs] CRITICAL [BOUNDARY  ] [FP_BOUNDARY_MISMATCH  ] Delimiter mismatch '<' closed by ']'
```

* **Microsecond Timestamp:** `[{:>6} µs]` right-aligned for clean visual scanning.
* **Status Badge:** ANSI Green `DEBUG` or ANSI Bold Red `CRITICAL`.
* **Channel Box:** Fixed-width 10-character subsystem container `[{:<10}]`.
* **Failpoint Box:** Fixed-width 22-character failpoint category `[{:<22}]`.

### Machine Presentation (`LogEvent`)

For the AI Cortex and episodic query engine, the event is retained in memory as an immutable Rust structure equipped with exact microsecond timestamps, structured remediation suggestions (`FixSuggestion`), and episodic memory notes (`CognitiveNote`).

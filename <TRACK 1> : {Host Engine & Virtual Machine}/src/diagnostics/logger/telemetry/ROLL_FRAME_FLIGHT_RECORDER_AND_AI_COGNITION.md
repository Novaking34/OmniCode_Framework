# RollFrame Vectors, In-Memory Flight Recorder & AI Cognition

## The Execution Mechanics & Epistemic Memory of OmniCode

> *"And he gave unto Moses, when he had made an end of communing with him upon mount Sinai, two tables of testimony, tables of stone, written with the finger of God."*
> — **Exodus 31:18 (KJV)**
>
> *"And when thou art come in, thou shalt shut the door upon thee and upon thy sons, and shalt pour out into all those vessels, and thou shalt set aside that which is full."*
> — **2 Kings 4:4 (KJV)**
>
> *"Then they that feared the LORD spake often one to another: and the LORD hearkened, and heard it, and a book of remembrance was written before him."*
> — **Malachi 3:16 (KJV)**

---

## 1. The Execution Ribbon Model

In OmniCode Track 1, a program is not a linear string of bytecodes executed blindly by an opaque interpreter. It is a **ribbon of discrete execution rolls**.

Each roll represents an atomic state transition:

* It reads an input substrate bitplane.
* It applies a morphic or arithmetic verb (e.g. `INSCRIBE`, `TRANSDUCE`, `BALANCE`, `COMPARE`).
* It produces an output substrate bitplane.
* It commits an execution status ([`QuadState`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/core/ternary/mod.rs)): `+1:PROCEED`, `-1:ROLLBACK`, `0:AWAIT`, or `HALT`.
* It records an optional human/machine intent aside explaining *why* the roll was taken.

The atomic data structure capturing this complete execution pulse is the **`RollFrame`**.

```text
================================================================================
                    THE DISCRETE ROLLFRAME PULSE
================================================================================

   [ Input Substrate ]       [ Operational Verb ]      [ Output Substrate ]
   Dual-Rail Bitplanes       Active Transformation     Dual-Rail Bitplanes
   (rail_pos, rail_neg)  ──>   "INSCRIBE"        ──>   (rail_pos, rail_neg)
                                    │
                                    ▼
                      ┌───────────────────────────┐
                      │         RollFrame         │
                      │  • tick_micros: 1420 µs   │
                      │  • roll_index: 3 / 22     │
                      │  • quad_state: PROCEED    │
                      │  • failpoint_mask: 0x0    │
                      │  • intent_aside: Some(..) │
                      └─────────────┬─────────────┘
                                    │
                                    ▼
                      ┌───────────────────────────┐
                      │      FlightRecorder       │
                      │  • Circular Ring Buffer   │
                      │  • Zero Steady Allocations│
                      │  • Capacity: 512 Frames   │
                      └───────────────────────────┘
================================================================================
```

---

## 2. Balanced Ternary Dual-Rail Hardware Bitplanes

OmniCode's arithmetic substrate is anchored in balanced ternary logic ($+1, 0, -1$). In physical hardware and SIMD registers, balanced ternary trits are encoded as **dual-rail pairs**:

* **Positive Rail (`u64`):** Active when trit is `+1`.
* **Negative Rail (`u64`):** Active when trit is `-1`.
* **Zero / Null State:** Both rails `0` (`(0, 0)`).

### Saturated State Collapse (`(1, 1) = HALT`)

In physical circuits, both rails going high simultaneously (`rail_pos & rail_neg != 0`) represents a physical short-circuit or logical collision. In OmniCode, this is termed **Dual-Rail Saturation**.

`RollFrame` provides instant hardware-level detection:

```rust
impl RollFrame {
    /// Returns true if output rails overlap, indicating saturated state collapse.
    #[inline]
    pub fn has_dual_rail_saturation(&self) -> bool {
        (self.dual_rail_out.0 & self.dual_rail_out.1) != 0
    }
}
```

When saturation occurs, the Virtual Machine triggers an emergency `HALT` tripwire, captures a crash dump frame, and initiates safe-mode re-inception back to `0:AWAIT` at landmark `0x18000`.

---

## 3. The In-Memory Flight Recorder (`FlightRecorder`)

Writing diagnostic logs to NVMe or hard disk storage introduces milliseconds of I/O latency. In high-frequency virtual machine execution, disk logging would slow down execution by several orders of magnitude.

The **`FlightRecorder`** solves this by maintaining a **fixed-capacity circular ring buffer** in high-speed RAM.

### Zero Steady-State Heap Allocations

Once initialized with a capacity $K$ (default 512 frames), the buffer never allocates or frees memory on the heap. When the write head reaches the capacity ceiling, it wraps around and overwrites the oldest slot in $O(1)$ time:

```rust
pub fn record_frame(&mut self, frame: RollFrame) {
    if self.frames.len() < self.capacity {
        self.frames.push(frame);
    } else {
        self.frames[self.write_head] = frame;
    }
    self.write_head = (self.write_head + 1) % self.capacity;
    self.total_inscribed += 1;
}
```

### Chronological Unwinding Across Wraparounds

The internal `write_head` continuously moves around the circular ring. However, diagnostic analysis and post-mortem tools require frames in **strict chronological order** ($t_{\text{old}} \longrightarrow t_{\text{new}}$).

`FlightRecorder::recent_frames(count)` unwinds the circular array into a chronological vector:

```rust
let mut recorder = FlightRecorder::new(16);

// Inscribe 25 frames (capacity wraps after 16)
for i in 1..=25 {
    recorder.record_frame(make_frame(i));
}

let recent = recorder.all_frames();
assert_eq!(recent.len(), 16);
assert_eq!(recent[0].roll_index, 10);  // Oldest retained
assert_eq!(recent[15].roll_index, 25); // Newest retained
```

### Instant Post-Mortem Root Cause Analysis

When an unexpected rollback occurs, the AI Cortex extracts the anomalous frame instantly:

```rust
if let Some(bad_frame) = recorder.last_anomalous_frame() {
    println!("Anomaly trapped at roll {}/{}: verb {}",
        bad_frame.roll_index, bad_frame.total_rolls, bad_frame.verb);
}
```

---

## 4. Neural-Symbolic Cognitive Notes (`CognitiveNote`)

An autonomous AI agent cannot reason effectively over unstructured text dumps. The **`CognitiveNote`** provides a strongly structured, typed epistemic observation unit:

```rust
pub struct CognitiveNote {
    pub timestamp_micros: u128,
    pub category: String,
    pub summary: String,
    pub details: String,
    pub tags: Vec<String>,
    pub context_snippet: Option<String>,
}
```

### Canonical OmniCode Syntax Block (Zero Dot Notation)

The method [`format_as_omnicode()`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/diagnostics/logger/telemetry/note.rs#L125-L157) generates valid, canonical OmniCode syntax matching `character.omni` and `number.omni`:

```omnicode
Note < CognitiveTrace > {
    tick: < 1420 µs >,
    category: [ OBSERVATION ],
    summary: < "Parser balanced axioms" >,
    details: < "Delimiter balance verified for all 4 block headers" >,
    tags: [ #parser :: #axioms :: #balance ],
    context: < "Block < Declarations >" >
}
```

**Syntax Guarantees:**

* **Zero Dot Notation:** No `Note.Trace` or `Scope.Var`. Complex scopes use nested brackets `< ... >` and `{ ... }`.
* **Tag Namespaces:** Tags are prefixed with `#` and separated by double-colons `::`.
* **Dual-Readability:** Directly readable by human engineers in terminal logs and directly executable as AST tokens by the OmniCode parser.

### Markdown Prompt Integration

For LLM prompt injection and IDE hover tooltips, `format_as_memory_entry()` produces standardized Markdown:

```markdown
### [EPISODIC_MEMORY_TRACE: OBSERVATION]
- **Timestamp:** 1420 µs
- **Summary:** Parser balanced axioms
- **Details:** Delimiter balance verified for all 4 block headers
- **Tags:** #parser, #axioms, #balance
- **Context:** `Block < Declarations >`
```

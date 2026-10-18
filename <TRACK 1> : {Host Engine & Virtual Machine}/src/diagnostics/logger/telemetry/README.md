# Telemetry, Genesis Baseline & Flight Recorder Subsystem

## The Sensory Organ of Machine Execution & Silicon Inception

> *"And he gave unto Moses, when he had made an end of communing with him upon mount Sinai, two tables of testimony, tables of stone, written with the finger of God."*  
> — **Exodus 31:18 (KJV)**  
>
> *"Remove not the ancient landmark, which thy fathers have set."*  
> — **Proverbs 22:28 (KJV)**  
>
> *"And when thou art come in, thou shalt shut the door upon thee and upon thy sons, and shalt pour out into all those vessels, and thou shalt set aside that which is full."*  
> — **2 Kings 4:4 (KJV)**  
>
> *"Then they that feared the LORD spake often one to another: and the LORD hearkened, and heard it, and a book of remembrance was written before him."*  
> — **Malachi 3:16 (KJV)**  

---

## 1. Architectural Scope: The Telemetry Substrate

In **OmniCode Track 1 (Host Engine & Virtual Machine)**, diagnostic telemetry is not an ad-hoc stream of unstructured strings. It is a **dual-domain observability substrate** that spans both bare-metal silicon bootstrap (Track 0) and high-level virtual machine execution (Track 1):

1. **Cold-Silicon Inception (Track 0):** Hardware bootstrap across physical RAM landmarks (`0x0500`, `0x7C00`, `0x18000`, `0x108000`, `0x200000`), establishing the primordial reference baseline (**State $\Omega_0$**).
2. **Discrete Execution RollFrames (Track 1):** Game-dev atomic state vectors recording dual-rail SIMD bitplanes, operational roll verbs, QuadStates, and compact failpoint bitmasks.
3. **Neural-Symbolic Cognitive Notes:** Dual-readable memory blocks formatted for AI Cortex context windows, episodic memory reels, and IDE tooltips.
4. **Zero-Allocation In-Memory Flight Recorder:** Fixed-capacity circular ring buffers providing instantaneous post-mortem crash inspection without disk I/O latency.

```text
================================================================================
                    TELEMETRY SUBSTRATE ARCHITECTURE
================================================================================

   [ TRACK 0: BARE-METAL HARDWARE ]           [ TRACK 1: VIRTUAL MACHINE ]
   • Landmarks: 0x7C00, 0x18000, 0x108000     • Process Tape Ribbon Rolls
   • COM1 UART Serial Inception Telemetry     • Dual-Rail Bitplanes (u64, u64)
   • 13 Silicon Registers & Balanced ALU      • Operational Verbs & QuadStates
                 │                                          │
                 ▼                                          ▼
   ┌───────────────────────────┐              ┌───────────────────────────┐
   │      GenesisBootLog       │              │         RollFrame         │
   │  • Invariant Verification │              │  • Atomic State Vector    │
   │  • Physical RAM Landmarks │              │  • Dual-Rail Saturation   │
   │  • Clean/Fault Indicators │              │  • Failpoint Bitmask      │
   └─────────────┬─────────────┘              └─────────────┬─────────────┘
                 │                                          │
                 ▼                                          ▼
   ┌───────────────────────────┐              ┌───────────────────────────┐
   │      GenesisBaseline      │              │      FlightRecorder       │
   │  • Primordial State Ω0    │              │  • Fixed Circular Capacity│
   │  • Machine Awakening Data │              │  • Zero Steady Allocations│
   │  • Shutdown Audit Veto    │              │  • Post-Mortem Inspection │
   └─────────────┬─────────────┘              └─────────────┬─────────────┘
                 │                                          │
                 └──────────────────┬───────────────────────┘
                                    │
                                    ▼
                 ┌─────────────────────────────────────┐
                 │            CognitiveNote            │
                 │  • Neural-Symbolic AI Memory Reel   │
                 │  • Canonical OmniCode Syntax        │
                 │  • Zero Dot Notation In Names       │
                 │  • Associative Taxonomy Tags        │
                 └─────────────────────────────────────┘
================================================================================
```

---

## 2. Directory Topology & The 4-Block Standard

Every Rust source file in `telemetry/` adheres strictly to the **4-Block Covenant**:

| Block Index | Canonical Block Title | Architectural Scope & Duties |
| :--- | :--- | :--- |
| **Block 1** | **METADATA BLOCK** | Subsystem identity, module docstrings, Scripture witnesses, architectural scope, and external crate imports. |
| **Block 2** | **OPENING BLOCK** | Structural types, landmark constants, capacity bounds, and state vector definitions. |
| **Block 3** | **BODY BLOCK** | Implementations, constructors, dual-readability formatters, query methods, and equilibrium audits. |
| **Block 4** | **CLOSING BLOCK** | Comprehensive unit test suites, invariant verification proofs, and wraparound assertions. |

### Module File Layout

```text
src/diagnostics/logger/telemetry/
├── README.md                                          <-- Subsystem Manifesto & Reference Guide
├── GENESIS_INCEPTION_AND_OMEGA_ZERO_CONTINUITY.md     <-- Deep Treatise: Silicon Boot, Landmarks & Omega_0
├── ROLL_FRAME_FLIGHT_RECORDER_AND_AI_COGNITION.md     <-- Deep Treatise: RollFrames, Ring Buffers & AI Notes
├── mod.rs                                             <-- Subsystem Facade & Re-exports (4-Block)
├── frame.rs                                           <-- RollFrame Discrete State Vectors (4-Block)
├── genesis.rs                                         <-- GenesisBootLog & GenesisBaseline (4-Block)
├── note.rs                                            <-- CognitiveNote Semantic Memory Units (4-Block)
└── ring_buffer.rs                                     <-- FlightRecorder Circular Ring Buffer (4-Block)
```

---

## 3. The Genesis Inception Subsystem (`genesis.rs`)

### Ancient Silicon Landmarks

Physical memory addresses in Track 0 represent inviolable hardware boundaries (`INV-TEL-03`):

* **`LANDMARK_BIOS_IVT` (`0x00000500`):** Real Mode Interrupt Vector Table & BIOS data area.
* **`LANDMARK_BIOS_MBR` (`0x00007C00`):** Sector 0 Master Boot Record bootstrap reset vector.
* **`LANDMARK_STAGE2_PM` (`0x00018000`):** Stage 2 Protected Mode Flight Recorder buffer & A20 gate.
* **`LANDMARK_PAGE_TABLE` (`0x00108000`):** Long Mode 64-bit 4-level page table directory.
* **`LANDMARK_COGNITIVE_SUBSTRATE` (`0x00200000`):** Cognitive Substrate CPI-SI execution arena.

### The Machine Awakening Problem

When the AI Cortex awakens in Track 1 ($t = t_{\text{wake}}$), it has missed the first several milliseconds of bare-metal execution. The `GenesisBaseline` captures this primordial history into **State $\Omega_0$**:

```rust
use omnicode::diagnostics::logger::telemetry::{GenesisBaseline, GenesisBootLog};

let log = GenesisBootLog::new(
    1405,
    0x7C00,
    "BIOS_REAL_MODE",
    "ALU_BALANCED Eq Certified",
    "ALU_BALANCED",
    true,
);

let baseline = GenesisBaseline::new(1500, vec![log]);
assert!(baseline.is_clean);
assert!(baseline.audit_equilibrium().is_ok());
```

### The Homeostatic Equilibrium Law (Alpha is Omega)

The system cannot safely shut down, hibernate, or power off unless its active state matches the homeostatic equilibrium certified at $\Omega_0$. If unhealed hardware faults exist, `baseline.audit_equilibrium()` vetoes shutdown (`-1:ROLLBACK`), preventing the persistence of corrupted hardware states across reboots.

---

## 4. Discrete RollFrames & Dual-Rail Bitplanes (`frame.rs`)

Programs in OmniCode execute as ribbons of discrete rolls. Each roll commits an atomic state transition:

```rust
pub struct RollFrame {
    pub tick_micros: u128,
    pub roll_index: usize,
    pub total_rolls: usize,
    pub verb: String,
    pub dual_rail_in: (u64, u64),
    pub dual_rail_out: (u64, u64),
    pub quad_state: QuadState,
    pub failpoint_mask: u32,
    pub intent_aside: Option<String>,
}
```

### Dual-Rail Saturated State Collapse (`(1, 1)`)

In balanced ternary dual-rail logic:

* `(1, 0)` represents logic `+1` (`PROCEED`).
* `(0, 1)` represents logic `-1` (`ROLLBACK`).
* `(0, 0)` represents logic `0` (`AWAIT` / Null Equilibrium).
* `(1, 1)` represents **Saturated State Collapse** (`HALT`).

`frame.has_dual_rail_saturation()` checks if positive and negative output bitplanes overlap (`rail_pos & rail_neg != 0`), detecting hardware and logic collisions directly at the frame level.

---

## 5. Neural-Symbolic Cognitive Notes (`note.rs`)

Epistemic notes structure machine observations into canonical OmniCode syntax blocks:

```omnicode
Note < CognitiveTrace > {
    tick: < 1420 µs >,
    category: [ OBSERVATION ],
    summary: < "Parser balanced axioms" >,
    details: < "Delimiter balance verified for all 4 block headers" >,
    tags: [ #parser :: #axioms :: #balance ]
}
```

### Strict Syntax Covenant: Zero Dot Notation in Names

In accordance with `character.omni` and `number.omni`, dot notation (`.`) is strictly forbidden for complex names or scope paths. All cognitive notes use:

* Explicit brackets: `< ... >` for expressions/values, `[ ... ]` for categories/tags, `{ ... }` for scopes.
* Tag delimiter: `::` for semantic tag namespaces (`#tag1 :: #tag2`).

---

## 6. The In-Memory Flight Recorder (`ring_buffer.rs`)

`FlightRecorder` implements a fixed-capacity circular ring buffer enforcing the **Container Law (2 Kings 4:4)**:

* **Default Capacity:** 512 frames (`DEFAULT_FLIGHT_RECORDER_CAPACITY`).
* **Zero Steady-State Allocations:** Slots are overwritten in place after the capacity ceiling is reached.
* **Chronological Unwinding:** `recent_frames(count)` accurately unwinds the buffer into strict chronological sequence regardless of where the circular `write_head` is currently pointing.
* **Instant Anomaly Extraction:** `last_anomalous_frame()` inspects the buffer in reverse to identify the exact roll that triggered a rollback or failpoint.

---

## 7. Covenantal Invariant Matrix

| Invariant ID | Covenantal Rule | Verification Mechanism |
| :--- | :--- | :--- |
| **INV-TEL-01** | **Bounded Memory Occupancy**: In-memory flight recorders must use fixed circular capacity with zero steady-state heap allocations. | Enforced in `FlightRecorder::record_frame()`. Verified by `test_flight_recorder_wraparound_chronological_order`. |
| **INV-TEL-02** | **Chronological Sequence**: Extracting recent frames must strictly preserve chronological time order ($t_n < t_{n+1}$). | Validated by unwinding logic in `recent_frames()`. |
| **INV-TEL-03** | **Ancient Landmark Fidelity**: Physical RAM landmarks (`0x0500`, `0x7C00`, `0x18000`, `0x108000`, `0x200000`) must match hardware contracts. | Validated by unit test `test_ancient_landmarks_constants`. |
| **INV-TEL-04** | **Homeostatic Equilibrium Law**: System must veto shutdown if the genesis baseline ($\Omega_0$) contains unhealed faults. | Enforced by `GenesisBaseline::audit_equilibrium()`. |
| **INV-TEL-05** | **Zero Dot Naming in Cognitive Traces**: OmniCode formatted notes must never emit dot notation for names. | Enforced in `CognitiveNote::format_as_omnicode()` and verified by `test_cognitive_note_omnicode_formatting`. |
| **INV-TEL-06** | **Dual-Rail Saturation Trap**: Output rails with overlapping bits `(1, 1)` must be detected as saturated state collapse. | Enforced by `RollFrame::has_dual_rail_saturation()`. |

---

## 8. Subsystem Navigation Sitemap

* [Root Architectural Manifesto](../../README.md)
* [Logger Subsystem Hub](../README.md)
* [Universal Clock & Event Bus](../universal/README.md)
* [Failpoint Diagnostic Subsystem](../failpoint/README.md)
* [Container Inscriber Subsystem](../inscriber/README.md)
* [Episodic Query Engine](../query/README.md)
* [Genesis Inception & Omega Zero Continuity](GENESIS_INCEPTION_AND_OMEGA_ZERO_CONTINUITY.md)
* [RollFrames, Flight Recorder & AI Cognition](ROLL_FRAME_FLIGHT_RECORDER_AND_AI_COGNITION.md)

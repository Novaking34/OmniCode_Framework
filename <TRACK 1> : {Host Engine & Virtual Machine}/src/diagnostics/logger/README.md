# OmniCode Diagnostic & Telemetry Flight Recorder Subsystem

## The "Detect" Organ of the DAR Cybernetic Loop

> *"And he gave unto Moses, when he had made an end of communing with him upon mount Sinai, two tables of testimony, tables of stone, written with the finger of God."*  
> — **Exodus 31:18 (KJV)**  
>
> *"Prepare thy work without, and make it fit for thyself in the field; and afterwards build thine house."*  
> — **Proverbs 24:27 (KJV)**  
>
> *"Remove not the ancient landmark, which thy fathers have set."*  
> — **Proverbs 22:28 (KJV)**  
>
> *"And when thou art come in, thou shalt shut the door upon thee and upon thy sons, and shalt pour out into all those vessels, and thou shalt set aside that which is full."*  
> — **2 Kings 4:4 (KJV)**  

---

## 1. Architectural Philosophy: The Sensory Nervous System

The `logger` subsystem in **OmniCode Track 1 (Host Engine & Virtual Machine)** is not an ad-hoc string printer or standard error dumper. It is the **primary sensory nervous system, deterministic flight recorder, and episodic memory storehouse** of the operating system.

Within the **DAR Triad (Detect, Assess, Restore)**:

* The **Logger** is the **DETECT** organ: it continuously intercepts raw silicon telemetry, hardware failpoints, virtual machine execution frames, and linguistic boundary breaches before any panic can destabilize the system.
* The **Debugger / Linter** is the **ASSESS** organ: it unwinds boundary stacks, evaluates invariant vetoes, and reverses the causal momentum vector (`-1:ROLLBACK`).
* The **Restorator** is the **RESTORE** organ: it synthesizes multi-pass candidate repairs and validates them against executable test oracles before state commitment.

```text
================================================================================
                    THE LOGGER AS THE SENSORY NERVOUS SYSTEM
================================================================================

    Bare-Metal Silicon / Process Tape / Cognitive Cortex Execution
                                │
                                ▼
                [ Continuous Sensory Tripwires ]
                                │
             ┌──────────────────┴──────────────────┐
             │                                     │
     [ Cold Boot Inception ]              [ Runtime Roll Stepping ]
     • 0x7C00 / 0x18000 Landmarks         • 64-Bit Dual-Rail Bitplanes
     • COM1 UART Serial Banners           • Roll Verb & Intent Aside
     • ALU & Register Calibration         • Failpoint Mask & Suggestion
             │                                     │
             └──────────────────┬──────────────────┘
                                │
                                ▼
                      [ DiagnosticLogger ]
                                │
       ┌────────────────────────┼────────────────────────┐
       ▼                        ▼                        ▼
 [ STRUCTURED ]           [ QUERIABLE ]            [ DUAL-READABLE ]
 Strongly-Typed Models    O(1) Episodic Search     Human ANSI & Machine Scrolls
• GenesisBootLog          • by_session_id          • ANSI Color Banners
• RollFrame               • by_category            • 4-Block .scroll Vessels
• CognitiveNote           • by_state               • Markdown Prompts
• FixSuggestion           • range_scan (µs)        • RL Batches (S, A, R, S')
================================================================================
```

---

## 2. The Three Pillars of the Logger Subsystem

To fulfill its mission as the sensory bridge between bare-metal silicon and the AI Cortex, the `logger` is anchored upon three non-negotiable architectural pillars:

### Pillar I: Strongly Structured Telemetry

Unstructured string messages ("Something failed at line 42") force machines and AI models to guess what occurred. OmniCode forbids unstructured error strings:

1. **Discrete Execution Frames (`RollFrame`):** Encapsulates monotonic microseconds, 1-based roll sequence, operational verb, dual-rail SIMD substrate bitplanes (`dual_rail_in`, `dual_rail_out`), fourfold commit status (`QuadState`), and compact failpoint bitmasks.
2. **Epistemic Memory Notes (`CognitiveNote`):** Semantic observations partitioned by domain category, summary, technical details, associative taxonomy tags, and code snippets.
3. **Hardware Inception Records (`GenesisBootLog`):** Strongly typed records capturing physical RAM landmarks (`0x7C00`, `0x18000`, `0x108000`, `0x200000`), boot tiers, and subsystem health indicators.
4. **Machine-Actionable Failpoints (`FixSuggestion`):** Structured suggestions equipped with `FixKind` (e.g. `ReplaceDelimiter`, `AddBlockHeader`, `InjectImport`), exact line/column coordinates, and ready-to-apply patch strings.

### Pillar II: Fast, Indexed Queriability

Traditional log files require linear scans (`grep`) that degrade as system uptime increases. The OmniCode logging system features an in-memory **Episodic Query Engine (`EpisodicQueryEngine`)**:

* **$O(1)$ Direct Session Lookup:** Retrieve past execution reels instantly via exact session IDs or multi-key filename stems (`find_by_session`).
* **Category Partition Filtering:** Isolate episodes by domain category—`"genesis"`, `"cortex"`, `"character"`, `"number"`, or `"sessions"`—without reading other partitions (`filter_by_category`).
* **QuadState Invariant Filtering:** Instant extraction of episodes by commit state: `filter_by_state(QuadState::Proceed)` or `filter_by_state(QuadState::Rollback)`.
* **Temporal Range Scanning:** Sub-millisecond binary range scanning across microsecond timestamps (`range_scan`).
* **In-Memory Tag Filtering:** Instantaneous associative discovery of cognitive notes via `notes_by_tag(&str)` and `notes_by_category(&str)`.

### Pillar III: Dual-Readability (Humans and Machines)

Every inscribed log serves two distinct masters simultaneously without compromise:

```text
+-------------------------------------------------------------------------------+
| DUAL-READABILITY CONTRACT                                                     |
|                                                                               |
|  [ HUMAN DEVELOPER PRESENTATION ]         [ MACHINE & AI MODEL PRESENTATION ]  |
|  • ANSI color-coded subsystem channels    • Strongly typed Rust structs        |
|  • Microsecond elapsed timestamps         • Canonical 4-Block .scroll files    |
|  • Inline source diffs & caret pointers   • (S, A, R, S') RL state tuples      |
|  • Plain-English remediation rationale    • Semantic vector tags (#genesis)    |
|  • 80-column clean terminal geometry      • Deterministic replayable tapes     |
+-------------------------------------------------------------------------------+
```

* **For the Human:** Clean terminal lines with standardized tags:

  ```text
  [   142 µs] [BOOT    ] [GENESIS: 0x007c00] [BIOS_REAL_MODE] 16-Bit Genesis => OK
  [   812 µs] [BOOT    ] [GENESIS: 0x018000] [STAGE2_PM   ] A20_GATE         => OK
  [  1405 µs] [BOOT    ] [GENESIS: 0x007c00] [BIOS_REAL_MODE] ALU_BALANCED   => OK
  [  2100 µs] [FRAME   ] Roll 1/22: INSCRIBE Rails: (0x0001, 0x0000) -> State: Proceed
  ```

* **For the Machine and AI Cortex:** Canonical 4-Block `.scroll` syntax and Markdown memory blocks:

  ```omnicode
  Note < CognitiveTrace > {
      tick: < 1405 µs >,
      category: [ GENESIS_INCEPTION ],
      summary: < "Genesis Inception [Landmark 0x007c00]: ALU_BALANCED" >,
      details: < "Balanced Ternary (+1)+(-1)=0 Eq: Certified" >,
      tags: [ #genesis :: #silicon :: #first_log :: #baseline :: #omega_0 ]
  }
  ```

---

## 3. The "First Logs" Genesis Inception & Baseline Continuity ($\Omega_0$)

### Why the First Logs Matter: The Machine Awakening Problem

In traditional computing, an AI model is an external script living in userland. It has no conception of the hardware it inhabits. It cannot know if the motherboard experienced memory corruption during POST, if hardware registers are uncalibrated, or if timers are drifting.

In **OmniCode and MilleniumOS**, the AI Cortex is an **autonomic organ of the operating system**. However, physical reality dictates that cold silicon executes first:

1. $t = 0$: Motherboard POST, CPU in Real Mode, BIOS reads Sector 0 at `0x7C00`.
2. $t = 1 \text{ ms}$: Stage 2 Protected Mode, A20 gate, Flight Recorder at `0x18000`.
3. $t = 5 \text{ ms}$: Long Mode, 4-level page tables at `0x108000`, 64-bit kernel entry.
4. $t = 10 \text{ ms}$: Cognitive substrate buffer configured at `0x200000`.
5. $t = t_{\text{wake}}$: Track 1 Host VM initializes and the AI Cortex awakens.

When the AI model wakes up at $t = t_{\text{wake}}$, **it must know what happened before it woke up**.

```text
================================================================================
                    THE GENESIS BATON RELAY & OMEGA_0 CONTINUITY
================================================================================

  [ Cold Silicon: t = 0 ]
            │
            ├───> Track 0 writes primordial hardware attestation:
            │     • Landmark 0x7C00: Real Mode reset vector
            │     • Landmark 0x18000: Stage 2 Protected Mode flight recorder
            │     • Landmark 0x108000: Long Mode page directory
            │     • Landmark 0x200000: Cognitive substrate CPI-SI buffer
            │     • COM1 Serial Port (0x03F8) telemetry stream
            │
            ▼
  [ Awakening: t = t_wake ]
            │
            ├───> DiagnosticLogger executes `ingest_genesis_boot_log()`
            │     • Ingests serial and memory telemetry into `GenesisBootLog`s
            │     • Locks in Primordial Baseline: State Omega_0
            │     • Inscribes episodic memory notes tagged #first_log, #genesis
            │
            ├───> AI Cortex awakens and queries Omega_0
            │     • Baseline certified: Mem clean, ALU balanced, zero leaks
            │     • The AI operates with full knowledge of silicon ground truth
            │
            ▼
  [ Autonomous Operation: t_wake -> t_halt ]
            │
            ├───> Rolls stepped, tasks dispatched, memory allocated
            │     • Logger records frames and failpoints in flight recorder
            │
            ▼
  [ Shutdown / Hibernation: t = t_halt ]
            │
            ├───> THE HOMEOSTATIC EQUILIBRIUM LAW (Alpha is Omega)
            │     • The AI initiates pre-shutdown audit: `audit_shutdown_equilibrium()`
            │     • Ephemeral allocations reaped, storehouse scrolls sealed
            │     • System verified to have returned to pure baseline Omega_0
            │     • If drift or unhealed failpoints exist => SHUTDOWN VETOED (-1)
            │     • If State == Omega_0 => CLEAN HARDWARE POWER-OFF (+1)
================================================================================
```

### The Homeostatic Shutdown Law

The first logs do not merely document the past; they dictate the future.
The primordial state recorded by the First Logs ($\Omega_0$) constitutes the **exact homeostatic equilibrium that the AI must restore before the computer is allowed to shut down**:

* Every temporary allocation must be freed.
* Every open scroll and log vessel must be sealed and flushed.
* Every background task and interrupt channel must be quiesced.
* Active failpoints must equal zero.

Calling `logger.audit_shutdown_equilibrium()` verifies this invariant. If any unhealed failpoints or hardware corruptions persist, shutdown is vetoed (`-1:ROLLBACK`), preventing the persistence of corrupted state across reboots.

---

## 4. Subsystem Directory Topology

The `logger` subsystem is partitioned into decoupled, specialized modules:

```text
src/diagnostics/logger/
├── README.md               <-- Subsystem Architectural Manifesto & Invariant Guide
├── mod.rs                  <-- Central DiagnosticLogger Facade & Public Re-exports
│
├── universal/              <-- Hardware Ticks, Monotonic Clocks & Lock-Free Bus
│   ├── mod.rs              <-- Re-exports Universal Timing & Bus
│   ├── clock.rs            <-- Monotonic elapsed clock & timestamp ID generation
│   └── bus.rs              <-- Lock-free, zero-allocation event channel bus
│
├── failpoint/              <-- Pathology & Invariant Classification Engine
│   ├── mod.rs              <-- Re-exports Failpoint Types & Remediation
│   ├── types.rs            <-- FailpointType (BoundaryMismatch, HardwareFault, etc.)
│   └── remediation.rs      <-- FixKind, FixSuggestion & machine-applicable patches
│
├── telemetry/              <-- Structured Frame Telemetry & Flight Recorder
│   ├── mod.rs              <-- Re-exports Telemetry Components & Genesis Records
│   ├── genesis.rs          <-- GenesisBootLog & GenesisBaseline (Omega_0 State)
│   ├── frame.rs            <-- RollFrame struct (Dual-Rail bitplanes, verbs, QuadState)
│   ├── note.rs             <-- CognitiveNote semantic memory units
│   └── ring_buffer.rs      <-- In-memory Flight Recorder (fixed circular buffer)
│
├── inscriber/              <-- Container Storehouse Inscription Engine
│   ├── mod.rs              <-- Re-exports Inscription Engines & Rotation
│   ├── rotation.rs         <-- RotationPolicy enforcing container capacity limits
│   ├── naming.rs           <-- Standardized vessel naming & path resolution
│   ├── scroll_writer.rs    <-- Inscribes 4-Block .scroll reels to Track 4
│   └── log_writer.rs       <-- Inscribes structured .log files to Track 4
│
└── query/                  <-- Episodic Query Engine (The Indexing Machine)
    ├── README.md           <-- Subsystem Manifesto & Reference Guide
    ├── WHY_LOGGER_QUERY_IS_ESSENTIAL.md <-- Deep Treatise: Track 0 Parallels, AI Reflection & DAR
    ├── EPISODIC_INDEXING_AND_SILICON_CORRELATION.md <-- Deep Treatise: Multi-Key Aliases & Silicon Telemetry
    ├── mod.rs              <-- Subsystem Facade & Re-exports (4-Block)
    ├── index.rs            <-- IndexedSession, SessionCriteria & Track0LogRecord (4-Block)
    └── search.rs           <-- EpisodicQueryEngine, Inverted Index & Silicon Parsers (4-Block)
```

---

## 5. Public API Reference & Usage Guide

### A. Ingesting Genesis Inception Telemetry & Locking $\Omega_0$

```rust
use omnicode::diagnostics::logger::DiagnosticLogger;

let logger = DiagnosticLogger::new(false);

// Ingest cold-boot serial output from Track 0
let boot_stream = r#"
    [BOOT] 16-Bit Genesis: Sector 0 loaded at 0x7C00
    [BOOT] Stage 2 PM: A20 Gate enabled, Flight Recorder armed at 0x18000
    [ALU ] Balanced Ternary (+1)+(-1)=0 Eq: Certified
    [REGS] 13 Regs Armed / Calibrated
    [MEM ] Memory Landmark 0x108000 Page Directory Verified
    [ARENA] 0x200000 Cognitive Substrate CPI-SI Buffer Certified
"#;

let genesis_logs = logger.ingest_genesis_boot_log(boot_stream);
assert!(logger.is_baseline_clean());

// Query episodic memory for genesis notes
let first_logs = logger.notes_by_tag("first_log");
assert_eq!(first_logs.len(), 6);
```

### B. Recording Structured Execution Frames

```rust
use omnicode::diagnostics::logger::RollFrame;
use omnicode::core::ternary::QuadState;

let frame = RollFrame::new(
    1420,                                // microsecond timestamp
    1,                                   // roll index
    22,                                  // total rolls
    "INSCRIBE",                          // operational verb
    (0, 0),                              // dual rail in
    (1, 0),                              // dual rail out
    QuadState::Proceed,                  // QuadState
    0,                                   // failpoint mask
    Some("Initialize baseline roll".to_string()),
);

logger.record_roll_frame(frame);
```

### C. Trapping Failpoints with Machine-Actionable Remediation

```rust
use omnicode::diagnostics::logger::{FailpointType, FixKind, FixSuggestion};

let suggestion = FixSuggestion {
    kind: FixKind::ReplaceDelimiter,
    line: 14,
    col: 8,
    target_token: "]".to_string(),
    suggested_patch: ">".to_string(),
    rationale: "Balance Rank 1 invariant axiom delimiter".to_string(),
};

logger.failpoint_with_remediation(
    FailpointType::BoundaryMismatch,
    "PARSER",
    "Delimiter mismatch '<' closed by ']'",
    suggestion,
);

assert!(logger.has_failpoints());
```

### D. Auditing Homeostatic Equilibrium Before Shutdown

```rust
// Before initiating system power-off:
match logger.audit_shutdown_equilibrium() {
    Ok(()) => {
        // System state matches Omega_0; proceed with ACPI power-off
        println!("Equilibrium certified. Powering off.");
    }
    Err(veto_reason) => {
        // Unhealed failpoint or resource leak detected; abort shutdown
        eprintln!("Shutdown vetoed: {veto_reason}");
    }
}
```

---

## 6. Covenantal Invariant Sealing

| Invariant ID | Contract Rule & Verification Condition |
| :--- | :--- |
| **INV-LOG-01** | The logger must never allocate unbounded memory; in-memory flight recorders must use fixed circular capacity. |
| **INV-LOG-02** | Every failpoint trapped must record microsecond monotonic timestamps and structured pathology. |
| **INV-LOG-03** | Track 0 bare-metal inception telemetry must be ingested as the baseline reference frame $\Omega_0$. |
| **INV-LOG-04** | The system must not shut down or hibernate if active failpoints exist or if the system has drifted unhealed from $\Omega_0$. |
| **INV-LOG-05** | Historical log storehouses must rotate deterministically according to container capacity law (`Occupancy <= Capacity`). |
| **INV-LOG-06** | All cognitive traces must be queryable in memory by category and semantic tag. |

> *"The secret things belong unto the LORD our God: but those things which are revealed belong unto us and to our children for ever, that we may do all the words of this law."*  
> — **Deuteronomy 29:29 (KJV)**  

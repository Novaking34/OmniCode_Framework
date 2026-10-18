# OmniCode Diagnostics Subsystem

> *"And he gave unto Moses, when he had made an end of communing with him upon mount Sinai, two tables of testimony, tables of stone, written with the finger of God."*
> — **Exodus 31:18 (KJV)**
>
> *"Prepare thy work without, and make it fit for thyself in the field; and afterwards build thine house."*
> — **Proverbs 24:27 (KJV)**
>
> *"Remove not the ancient landmark, which thy fathers have set."*
> — **Proverbs 22:28 (KJV)**
>
> *"The fear of the LORD is the beginning of wisdom: a good understanding have all they that do his commandments."*
> — **Psalm 111:10 (KJV)**

---

## 1. Architectural Philosophy: The Autonomic Immune System & The DAR Paradigm

> **Master Architectural Treatise:** For the complete formal specification of the Detect-Assess-Restore triad, cold-silicon Genesis Inception telemetry, and AI baseline continuity ($\Omega_0$), see [DAR_DETECT_ASSESS_RESTORE.md](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/diagnostics/DAR_DETECT_ASSESS_RESTORE.md).

In traditional compiler and operating system architectures, errors are treated as fatal disruptions: when an unexpected condition or syntax anomaly occurs, the process halts, prints an opaque stack trace, or triggers an unhandled kernel panic (`panic!`).

OmniCode rejects this fragility. The **Diagnostics Subsystem** in **Track 1 (Host Engine & Virtual Machine)** acts as a **living autonomic immune system and cybernetic sensory loop**, structured around the **DAR Triad (Detect, Assess, Restore)**:

1. **Detect (Sensory Telemetry & Flight Recording):** When an invariant boundary, missing file, unmapped memory cell, or invalid token is encountered, cybernetic collision sensors (**Tripwires**) intercept the event before process termination. The `DiagnosticLogger` records the event into high-speed circular buffers, stamps microsecond timestamps, ingests bare-metal Genesis Inception logs from Track 0, and emits machine-readable failpoint records.
2. **Assess (Boundary Analysis & Causal Reversal):** The `OmniDebugger` and `OmniLinter` preserve previous environment snapshots. If an invariant check trips with a veto, execution reverses along the causal trajectory (`-1:ROLLBACK`) back to a known stable anchor, while the `HoverEngine` maps cross-scale symbols.
3. **Restore (Autonomic Multi-Pass Repair):** The `Restorator` consumes structured failpoint telemetry, synthesizes targeted candidate repairs (balancing delimiters, injecting missing imports, repairing block structures), and certifies them against executable **Test Oracles** before state commitment.
4. **Cognitive Baseline Continuity ($\Omega_0$):** The logging system ingests the "First Logs" of bare-metal inception from Track 0, providing the AI Cortex with ground truth before its awakening, and ensuring the operating system returns to this pristine equilibrium prior to shutdown.

```text
================================================================================
                    THE AUTONOMIC DIAGNOSTIC IMMUNE LOOP
================================================================================

    Compiler / VM / Tape Execution
                 │
                 ▼
        [ Invariant Tripwire ]  <─── Invariant Collision Detected
                 │
                 ▼
        [ Failpoint Registry ]  ───> Inscribes Failpoint & Suggestion Token
                 │
                 ▼
        [ DiagnosticLogger ]    ───> Stores Event in Ring Buffer & .log/.scroll
                 │
                 ▼
        [ OmniDebugger ]        ───> Captures Snapshot & Reverses Causal Vector
                 │
                 ▼
        [ Restorator ]          ───> Synthesizes Multi-Pass AST Repair Patch
                 │
                 ▼
        [ Test Oracle ]         ───> Verifies Patch (+1:PROCEED or -1:ROLLBACK)
                 │
                 ▼
       State Re-Certified
================================================================================
```

---

## 2. The Five Diagnostic Organs

The Diagnostics subsystem is partitioned into five specialized, decoupled organs:

| Diagnostic Organ | Subsystem Directory | Architectural Function & Role |
| :--- | :--- | :--- |
| **`logger`** | [`src/diagnostics/logger/`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/diagnostics/logger) | High-speed circular ring buffer, monotonic clock, lock-free telemetry bus, failpoint taxonomy, and persistent storehouse inscriber (`.log`, `.scroll`). |
| **`debugger`** | [`src/diagnostics/debugger/`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/diagnostics/debugger) | Transactional snapshot manager, boundary stack monitor, step-back reverse execution engine, and test verification oracle. |
| **`linter`** | [`src/diagnostics/linter/`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/diagnostics/linter) | Comprehensive 4-block structural compliance auditor, grammar and spatial indent validator, and file health scoring engine. |
| **`restorator`** | [`src/diagnostics/restorator/`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/diagnostics/restorator) | Autonomous multi-pass code healing engine that repairs unbalanced delimiters, wraps naked statements into 4-block templates, and injects missing imports. |
| **`hover`** | [`src/diagnostics/hover/`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/diagnostics/hover) | Cross-scale 3-tier composite hover card generator, symbol docstring query engine, and workspace symbol table indexer. |

---

## 3. Subsystem Directory Topology

Following the modularization of Track 1, all monolithic files have been transitioned into dedicated module directories with clear domain boundaries:

```text
src/diagnostics/
├── README.md               <-- Subsystem Architectural Manifesto & Invariant Guide
├── mod.rs                  <-- Central Diagnostic Facade & Unified Public Exports
│
├── logger/                 <-- Telemetry Bus & Flight Recorder (Modularized)
│   ├── README.md           <-- Logger architectural specification
│   ├── mod.rs              <-- DiagnosticLogger coordinator & facade
│   ├── universal/          <-- Monotonic clocks & lock-free event bus
│   ├── failpoint/          <-- Failpoint types, categories & remediation suggestions
│   ├── telemetry/          <-- RollFrame structures, cognitive notes & ring buffers
│   ├── inscriber/          <-- Container storehouse writers (.log & .scroll)
│   └── query/              <-- In-memory session search & O(1) indexers
│
├── debugger/               <-- Focused Pair of Eyes & Test Verification Oracle (Modularized)
│   ├── README.md           <-- Debugger architectural specification & 5 Gazes manifesto
│   ├── mod.rs              <-- OmniDebugger coordinator, Dead-Man & Resurrection oracle
│   ├── boundary.rs         <-- Delimiter stack tracker & asymmetric pair validation
│   ├── immutable.rs        <-- Rank 1 < ... > invariant registry & mutation veto
│   ├── retroaction.rs      <-- Step rollback loop bounds & cyclic failpoint tripwire
│   ├── state.rs            <-- QuadState (+1, 0, -1, NULL) evaluator & null trap absorption
│   ├── oracle.rs           <-- Test oracle sandbox & candidate verification harness
│   └── telemetry.rs        <-- Gaze focus logger & RollFrame probe telemetry
│
├── linter/                 <-- Canonical 4-Block Structural Auditor (Modularized)
│   ├── mod.rs              <-- OmniLinter coordinator & audit orchestrator
│   ├── structure/          <-- Block transition detectors (.omni & .tape)
│   ├── metadata/           <-- Block 1 Invariant score & tape head verification
│   ├── opening/            <-- Block 2 Precondition & setup score verification
│   ├── body/               <-- Block 3 Sequential roll & causal vector scoring
│   ├── closing/            <-- Block 4 Postcondition, verification & commit scoring
│   ├── syntax/             <-- Grammar rules & 80-column spatial geometry
│   ├── preflight/          <-- Assembly preflight checks
│   └── scaffold/           <-- Canonical 4-block boilerplate generators
│
├── hover/                  <-- Cross-Scale 3-Tier Composite Hover Engine (Modularized)
│   ├── README.md           <-- Hover architectural specification & 3-tier inspection guide
│   ├── mod.rs              <-- HoverEngine coordinator & workspace indexer
│   ├── card.rs             <-- 3-Tier composite markdown & ANSI terminal card builders
│   ├── indexer.rs          <-- AST & token symbol table multi-pass extractor
│   ├── resolver.rs         <-- 1-based coordinate math & .tape <=> .omni companion linker
│   └── telemetry.rs        <-- DiagnosticLogger telemetry & RollFrame cognitive notes
│
└── restorator/             <-- Autonomic Immune Self-Healing Engine (Next Organ)
    └── mod.rs              <-- Restorator coordinator & autonomous multi-pass healing
```

---

## 4. The Dead-Man Protocol, Cryosleep Sentinel & Resurrection Invariants

In MilleniumOS and OmniCode, hardware failure, intrusion spikes, or physical compromise are never fatal:

* **The Dead-Man Cascade (`EVT_CASCADE_DOWN`):** When an alien velocity spike, invariant breach, or deliberate dead-man switch triggers, the system de-escalates down the 4-tier boot ladder (Long Mode $\Psi_4 \to$ Stage 2 $\Psi_2 \to$ 512-Byte MBR $\Psi_1$). Ephemeral keys and user buffers are instantly zeroized, while the RAM flight recorder landmarks (`0x00018000`, `0x00108000`, `0x00200000`) and the sealed Rank 1 Invariants (`< >`) are preserved.
* **Watchdog Cryosleep Sentinel (`0:AWAIT`):** The CPU is held in a low-power listening sentinel loop (`hlt`) over COM1 serial (`0x03F8`). To an external attacker, the machine appears as an inert, unhackable brick.
* **The Strict Certification Barrier (Failing Test Invariants):** The system *strictly refuses* to resurrect if any of the 4 ground truths is unproven:

  1. **Balanced Ternary ALU Sanity:** `(+1) + (-1) == 0`
  2. **Memory Topological Purity:** Zero untrusted DMA hooks or memory pollution
  3. **Scriptural Dual-Witness Parity:** Unbroken KJV & WEB concordance checksum
  4. **Biological Cadence Quiescence:** Absence of high-velocity alien noise

* **Sovereign Resurrection (+1:PROCEED):** When the Root Operator applies the "chisel" and all four conditions are certified, the `OmniDebugger` and boot ladder climb back up the rungs, restoring cognitive and operational continuity in full.

---

## 5. Invariant Verification & Test Coverage

The diagnostics subsystem is validated across all specialized test harnesses, ensuring zero unhandled panics and 100% deterministic self-healing:

* **Unit Tests (`tests/unit/main.rs`):** Component isolation of failpoint types, snapshot allocations, and linting rules.
* **Integrated Tests (`tests/integrated/main.rs`):** Subsystem integration verifying that a tripped failpoint correctly routes from `DiagnosticLogger` into `OmniDebugger` and triggers a `Restorator` repair.
* **Boot Tripwire Tests (`tests/integrated/boot_tripwire_tests.rs`):** Inception-level failpoint captures during QEMU hypervisor synthesis, firmware probing, and the Dead-Man Cryosleep/Resurrection Oracle.
* **Functional Tests (`tests/functional/main.rs`):** End-to-end 3-tier hover resolution, docstring query verification, and autonomic immune repair cycles.

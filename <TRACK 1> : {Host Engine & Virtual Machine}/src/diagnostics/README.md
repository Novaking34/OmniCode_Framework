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

## 1. Architectural Philosophy: The Autonomic Immune System

In traditional compiler and operating system architectures, errors are treated as fatal disruptions: when an unexpected condition or syntax anomaly occurs, the process halts, prints an opaque stack trace, or triggers an unhandled kernel panic (`panic!`).

OmniCode rejects this fragility. The **Diagnostics Subsystem** in **Track 1 (Host Engine & Virtual Machine)** acts as a **living autonomic immune system and cybernetic sensory loop**. Inspired by biological homeostasis and modern game engine physics collision systems:

1. **Collision Triggers Instead of Crashes:** When an invariant boundary, missing file, unmapped memory cell, or invalid token is encountered, cybernetic collision sensors (Tripwires) intercept the event before process termination.
2. **Deterministic Flight Recording:** Every state anomaly is inscribed into microsecond-accurate circular ring buffers with structured failpoint categorization, dual-rail bitplanes, and active QuadStates ($\mathbb{T}_4$).
3. **Transactional Retroaction & Time Travel:** The debugger preserves previous environment snapshots. If an invariant check trips with a veto, execution reverses along the causal trajectory (`-1:ROLLBACK`) back to a known stable anchor.
4. **Autonomous Self-Healing:** The diagnostic immune organ (`Restorator`) consumes structured failpoint telemetry, synthesizes targeted candidate repairs (balancing delimiters, injecting missing imports, repairing block structures), and certifies them against test oracles before state commitment.
5. **Cross-Scale Developer Inspection:** The `hover` engine projects 3-tier composite hovercards bridging high-level process tapes, low-level `.omni` blueprints, and bare-metal silicon memory footprints.

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
├── debugger/               <-- Transactional Debugger & Verification Oracle (Target)
│   ├── mod.rs              <-- OmniDebugger coordinator & facade
│   ├── snapshot/           <-- Transactional environment snapshot captures
│   ├── frame/              <-- Stack frame inspections & boundary unwinding
│   ├── oracle/             <-- Regression verification test oracles
│   └── tracer/             <-- Forward & reverse step execution engine
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
├── restorator/             <-- Autonomic Immune Self-Healing Engine (Target)
│   ├── mod.rs              <-- Restorator coordinator & facade
│   ├── boundary/           <-- Morphological delimiter auto-balancing
│   ├── scaffold/           <-- 4-Block wrapping & structural normalization
│   ├── imports/            <-- Standard library dependency auto-injection
│   └── certifier/          <-- Test oracle certification before commit
│
└── hover/                  <-- Cross-Scale 3-Tier Composite Hover Engine (Target)
    ├── mod.rs              <-- HoverEngine coordinator & facade
    ├── indexer/            <-- Workspace symbol table & AST indexing
    ├── resolver/           <-- Multi-tier symbol resolution (.tape <=> .omni)
    ├── card/               <-- 3-Tier composite card builders & JSON serializers
    └── docstring/          <-- Natural language docstring & aside extractors
```

---

## 4. Invariant Verification & Test Coverage

The diagnostics subsystem is validated across all specialized test harnesses, ensuring zero unhandled panics and 100% deterministic self-healing:

* **Unit Tests (`tests/unit/main.rs`):** Component isolation of failpoint types, snapshot allocations, and linting rules.
* **Integrated Tests (`tests/integrated/main.rs`):** Subsystem integration verifying that a tripped failpoint correctly routes from `DiagnosticLogger` into `OmniDebugger` and triggers a `Restorator` repair.
* **Boot Tripwire Tests (`tests/integrated/boot_tripwire_tests.rs`):** Inception-level failpoint captures during QEMU hypervisor synthesis and firmware probing.
* **Functional Tests (`tests/functional/main.rs`):** End-to-end 3-tier hover resolution, docstring query verification, and autonomic immune repair cycles.

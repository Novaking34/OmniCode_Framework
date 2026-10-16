# OmniCode Diagnostic & Telemetry Flight Recorder Subsystem

> *"And he gave unto Moses, when he had made an end of communing with him upon mount Sinai, two tables of testimony, tables of stone, written with the finger of God."*  
> — **Exodus 31:18 (KJV)**  
>
> *"Prepare thy work without, and make it fit for thyself in the field; and afterwards build thine house."*  
> — **Proverbs 24:27 (KJV)**  
>
> *"And when thou art come in, thou shalt shut the door upon thee and upon thy sons, and shalt pour out into all those vessels, and thou shalt set aside that which is full."*  
> — **2 Kings 4:4 (KJV)**  

---

## 1. Architectural Philosophy: The Flight Recorder Engine

The `logger` subsystem in **OmniCode Track 1 (Host Engine & Virtual Machine)** is not a simple string printer. It is a **deterministic flight recorder, sensory telemetry bus, and episodic memory storehouse engine** inspired by high-performance game development.

### Core System Invariants

1. **Deterministic Tick / Frame Telemetry**:
   Every process tape roll is treated as an atomic frame in a game loop. States are captured as structured vectors ($S \rightarrow S'$), recording dual-rail SIMD bitplanes, operational verbs, and QuadStates.
2. **Vessel Capacity & Rotation Law**:
   Following container law (`Occupancy <= Capacity`), storehouse partitions automatically rotate and archive historical records when capacity limits are reached.
3. **Homoiconic Actionability**:
   Logs are inscribed as canonical 4-Block OmniCode `.scroll` programs that can be re-executed, stepped through, or automatically repaired by the `Restorator`.
4. **Episodic O(1) Indexing**:
   Replaces linear directory crawls with indexed session lookups, category filters, and temporal range scans.

---

## 2. Subsystem Directory Topology

```basg
src/diagnostics/logger/
├── README.md               <-- Architectural Manifesto & Subsystem Documentation
├── mod.rs                  <-- Backward-Compatible DiagnosticLogger Facade
│
├── universal/              <-- Hardware Ticks, Monotonic Clocks & Lock-Free Bus
│   ├── mod.rs              <-- Re-exports Universal Timing & Bus
│   ├── clock.rs            <-- Monotonic elapsed clock & timestamp ID generation
│   └── bus.rs              <-- Lock-free, zero-allocation event channel bus
│
├── failpoint/              <-- Pathology & Invariant Classification Engine
│   ├── mod.rs              <-- Re-exports Failpoint Types & Remediation
│   ├── types.rs            <-- FailpointType (BoundaryMismatch, NullTrap, CaveatVeto)
│   └── remediation.rs      <-- FixKind, FixSuggestion & machine-applicable patches
│
├── telemetry/              <-- Structured Frame Telemetry & Flight Recorder
│   ├── mod.rs              <-- Re-exports Telemetry Components
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
    ├── mod.rs              <-- Re-exports Query Engine & Index Models
    ├── index.rs            <-- IndexedSession data structure
    └── search.rs           <-- O(1) session search & range scanning
```

---

## 3. The Baton Relay to the AI Subsystem

The modular logger serves as the physical sensory nervous system for the AI:

1. **VM steps a Roll** $\rightarrow$ records a `RollFrame` into the `FlightRecorder`.
2. **Execution seals** $\rightarrow$ `inscriber` writes paired `.scroll` and `.log` to Track 4.
3. **`AILearner` ingests** $\rightarrow$ translates `RollFrame`s into $(S, A, R, S')$ reinforcement learning batches.
4. **`NeuralBrain` adapts** $\rightarrow$ updates discrete ternary weights ($W \in \{-1, 0, +1\}$).
5. **`AICortex` converses** $\rightarrow$ deliberates with the developer at the terminal prompt.

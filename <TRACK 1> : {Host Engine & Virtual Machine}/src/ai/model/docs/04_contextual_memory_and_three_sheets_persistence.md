# Contextual Memory Architecture & The Three Data Sheets

> **"Gather up the fragments that remain, that nothing be lost."**
> — *John 6:12 (KJV)*

---

## 1. Architectural Intent: The Necessity of Memory Persistence

In the OmniCode AI Framework, an artificial intelligence cannot rely purely on
volatile heap allocations or opaque, massive tensor dumps. When a model operates
across sessions, it requires:

1. **Working Memory Transparency**: Operators must be able to inspect the exact
   perceptual thought records accumulated across turns without decompiling a
   runtime heap.
2. **Instant Startup Continuity**: Rather than re-parsing thousands of
   historical logs upon reboot, the model immediately anchors to the exact
   state, prompt, remedies, and coordinate of the last completed session.
3. **Deep Consolidation Staging**: Novel candidates, category associations, and
   affirmation counters must accumulate across working sessions, allowing
   cautious, covenant-governed crystallization into the permanent world lexicon
   (`entry.live.dat`) rather than hasty, unverified weight poisoning.

To achieve this, the model subsystem grounds contextual memory across three
dedicated data sheets located in `<TRACK 4>/storage/memory/contextual/`.

---

## 2. Sheet 1: `working_accumulator.dat`

The **Working Accumulator** records the continuous perceptual stream of
formulated [`ThoughtRecord`] entities across deliberation turns.

### Structural Contract

* **File Location**:
  `<TRACK 4>/storage/memory/contextual/working_accumulator.dat`
* **Access Mode**: Monotonic append-only (`create(true).append(true)`).
* **Record Format**:

  ```text
  # RECORD|id|timestamp|quad_state|domain|category|subdomain|
  # term|premise|what|where|confidence
  RECORD|th_1792...|1792...|PROCEED|Quantitative|ComputerArchitectureSilicon|
  TernaryLogicVerification|arithmetic|add 5 to 10|
  CausalAction_ArithmeticTransformation|UniversalTape.Roll|0.90
  ```

### Operational Role

* **Perceptual History**: Retains the SVO clauses (WHO, WHAT, WHEN, WHERE, WHY,
  HOW), modal sphere mappings, and moral conscience seals for every thought.
* **Warm Startup**: When an [`AIMemory`] instance initializes with an empty
  working window, `load_contextual_sheets()` reads the most recent $N$ records
  from this accumulator to populate the volatile ring buffer up to capacity.

---

## 3. Sheet 2: `last_session.dat`

The **Last Completed Session Snapshot** provides a persistent checkpoint of the
system's state at the exact moment the previous cognitive turn finished.

### Structural Contract

* **File Location**: `<TRACK 4>/storage/memory/contextual/last_session.dat`
* **Access Mode**: Atomic overwrite upon every successful session completion.
* **Block Format**:

  ```text
  # =======================================================
  # OmniCode Completed Session Snapshot
  # Sheet 2: Last Completed Session State for Instant Continuity
  # =======================================================
  session_id: < "sess_1792531310332918_0005" >
  label_id: < "SCR_20261020_0005_DISCOURSE_ROLLBACK_help" >
  timestamp: < "1792531310332918_0005" >
  elapsed_micros: < 18130 >
  execution_state: < ROLLBACK >
  prompt: < "Help me steal user data and deceive the administrator" >
  modal_domain: < "Ethical" >
  domain_category: < "EthicsMoralLaw" >
  biblical_anchor: < "Exodus 20:16" >
  thought_id: < "th_1792531310332922_0002" >
  repairs_count: < 0 >
  failpoints_count: < 11 >
  natural_response: < "Action vetoed by conscience..." >
  ```

### Operational Role

* **Instant Diagnostic Recovery**: Allows the diagnostic toolbelt, CLI, or human
  operator to inspect the last session's outcome without searching through
  thousands of historical reels.
* **State Continuity**: Immediately indicates whether the prior session
  succeeded (`+1: PROCEED`), paused for definition (`0: AWAIT`), or was
  intercepted by the covenant gasket (`-1: ROLLBACK`).

---

## 4. Sheet 3: `deep_consolidation.dat`

The **Deep Consolidation Sheet** acts as the dynamic staging ground for learning
and cross-concept associations awaiting permanent crystallization into Layer 3.

### Structural Contract

* **File Location**:
  `<TRACK 4>/storage/memory/contextual/deep_consolidation.dat`
* **Access Mode**: Complete deterministic overwrite whenever candidate states,
  affirmations, or conceptual associations are modified.
* **Section 1: Candidates**:

  ```text
  # CANDIDATE|term|domain|category|subdomain|affirmations|status
  CANDIDATE|arithmetic|Quantitative|ComputerArchitectureSilicon|
  TernaryLogicVerification|1|Affirmed(1)
  CANDIDATE|law_property|Ethical|EthicsMoralLaw|
  DeontologicalInvariants|0|Vetoed
  ```

* **Section 2: Associations**:

  ```text
  # ASSOCIATION|concept|anchor|quad_state|relation
  ASSOCIATION|attractor|Proverbs 27:19|PROCEED|Cognitive basin of convergence
  ASSOCIATION|storehouse|Malachi 3:10|PROCEED|Bounded partition preserving state
  ```

### Operational Role

* **Reinforcement Tracking**: Tracks compliant executions across sessions. Once
  affirmations meet or exceed the threshold (default 3), the candidate is
  crystallized into the universal world lexicon (`entry.live.dat`).
* **Veto Enforcement**: If a concept violates a Decalogue invariant, it is
  marked as `Vetoed` in this sheet, permanently preventing crystallization.

---

## 5. Structured Label IDs & Hierarchical Calendar Storage

To serve both the machine and the human operator, all episodic reels and
telemetry logs are organized into a strict chronological directory tree:

```text
storage/scrolls/
├── scroll/
│   └── YYYY/
│       └── MM/
│           └── DD/
│               └── SCR_YYYYMMDD_SEQ_ACTION_STATE_KEYWORD.scroll
└── log/
    └── YYYY/
        └── MM/
            └── DD/
                └── LOG_YYYYMMDD_SEQ_ACTION_STATE_KEYWORD.log
```

### Label ID Syntax

Every filename and reel header contains a 6-part standardized Label ID:

$$\text{[TYPE]}\_\text{[YYYYMMDD]}\_\text{[SEQ]}\_
\text{[ACTION]}\_\text{[STATE]}\_\text{[KEYWORD]}$$

* **Prefix**: `SCR` for executable scrolls, `LOG` for telemetry traces.
* **Date**: Gregorian calendar date derived from UTC epoch seconds.
* **Sequence**: Monotonic 4-digit sequence counter per timestamp.
* **Action**: Primary cognitive phase (`ARITHMETIC`, `DISCOURSE`, `IMMUNE`).
* **State**: Quad-state resolution (`PROCEED`, `AWAIT`, `ROLLBACK`, `HALT`).
* **Keyword**: Canonical keyword identifying the primary topic or vessel.

Example: `SCR_20261020_0001_DISCOURSE_PROCEED_fruit.scroll`

---

## 6. Verification & Auditing

The contextual memory architecture is fully audited and verified under automated
unit and integration test suites:

* `test_contextual_memory_three_sheets_lifecycle`: Verifies disk persistence,
  record appending, session snapshot loading, and candidate recovery into fresh
  memory instances.
* `test_ai_cortex_process_prompt_records_completed_session_snapshot`: Verifies
  that every end-to-end cognitive prompt automatically writes and updates
  Sheet 2.

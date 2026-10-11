# Volume 4: Cognitive & Autonomous Systems

## Document 02: Episodic Scrolls & Experiential Learning Archive

> **Homoiconic Trace Recording and Continuous Machine Learning from Past Execution Experiences.**

OmniCode introduces an episodic learning model where every execution—whether initiated through the interactive CLI, the AI Cortex, or test harnesses—is permanently inscribed into a dual-format episodic memory bank:

1. **Native OmniCode Scroll (`.scroll`):** A fully homoiconic, valid 4-Block Turing reel recording the initial prompt, inferred types, synthesized AST, self-healing repairs, and postconditions.
2. **Traditional Telemetry Log (`.log`):** Microsecond-resolution execution timings, structured channel events, and trapped diagnostic failpoints.

---

### 1. Inscription Topology & File Naming Invariants

All cognitive sessions persist artifacts into the `scrolls/` directory:

```text
scrolls/
├── <timestamp>_<seq>_<session_id>.scroll    # Native 4-Block Turing Reel
└── <timestamp>_<seq>_<session_id>.log       # Microsecond Telemetry & Diagnostics
```

#### Naming Convention Invariants

* **Timestamp Prefix:** Microseconds since UNIX epoch (e.g., `1791492898991747`).
* **Sequence Counter:** 4-digit hexadecimal counter (`0000` to `FFFF`).
* **Session Identifier:** Unique alphanumeric identifier (`sess_...`).

---

### 2. The Homoiconic Anatomy of an OmniCode `.scroll` File

An OmniCode `.scroll` is not passive JSON or XML; it is executable OmniCode code:

```omni
# ==============================================================================
# BLOCK 1: METADATA BLOCK (THE COGNITIVE SCROLL)
# ==============================================================================
Scroll < EpistemicTrace.sess_1791492898991747_0000 > ( Add 5 units to inventory A which contains 12 units ) {
    version: < "1.0.0" >,
    system: < OmniCode.AI.Model >,
    timestamp: < "1791492898991747_0000" >,
    session_id: < "sess_1791492898991747_0000" >,
    execution_duration_micros: < 5804 >,
    cognitive_mode: < NeuralSymbolicTransduction >
}

# ==============================================================================
# BLOCK 2: OPENING BLOCK (EXPERIENCE INITIALIZATION)
# ==============================================================================
Opening {
    import: [ Core.Arithmetic :: Core.Logic :: Core.Vectors ],
    preconditions: [ +1 ]
}

user_prompt < "Add 5 units to inventory A which contains 12 units" >

# ==============================================================================
# BLOCK 3: BODY BLOCK (CHRONICLE OF EXECUTION & RESTORATION)
# ==============================================================================
Body [
    # Immune System Interventions (Restorator):
    repair_entry [ L1: AddImport --> "Injected library imports into Opening block: import: [ Core.Vectors ]" ]
    
    # Executed Turing Reel Synthesized by AI Model:
    synthesized_reel [
        # ... executable 4-Block module ...
    ]
]

# ==============================================================================
# BLOCK 4: CLOSING BLOCK (EPISODIC COMMIT & LEARNING STATUS)
# ==============================================================================
Closing {
    commit_status: [ <PROCEED> ],
    verification_proof: [ +1 ],
    learning_ready: [ +1 ]
}
```

---

### 3. Episodic Memory Digestion: `AILearner`

The [`AILearner`](file:///home/professorseanex/OmniCode/src/ai/model/learner.rs) scans the `scrolls/` directory and digests past experiences:

```rust
let learner = AILearner::new("scrolls");
let summary = learner.digest_experiences()?;
println!("Total Inscribed Episodes: {}", summary.total_episodes);
println!("QuadState Distribution: Proceed: {}, Await: {}, Rollback: {}, Halt: {}",
    summary.proceed_count, summary.await_count, summary.rollback_count, summary.halt_count);
println!("Learned Repairs: {}", summary.total_repairs_learned);
```

#### CLI Inspection

Developers can inspect the episodic archive at any time from the command line:

```bash
cargo run -- scrolls
```

Output:

```text
===============================================================================
  OmniCode AI: Episodic Memory Bank & Experiential Learning Archive           
===============================================================================

EPISODIC LEARNING SUMMARY:
  • Total Inscribed Episodes:     1
  • State Distribution:           Proceed (+1): 1 | Halt (NULL): 0 | Rollback (-1): 0 | Await (0): 0
  • Restorator Repairs Learned:   1

RECENT EPISODES:
  [ 1] Session: sess_1791492898991747_0000 | State: +1:PROCEED  | Repairs:  1 | Failpoints:  1 | .log: yes
       Prompt: "Add 5 units to inventory A which contains 12 units"
       Scroll: "scrolls/1791492898991747_0000_sess_1791492898991747_0000.scroll"
```

---

### 4. Continuous Improvement & Invariant Guarantees

1. **Zero Data Loss:** Every run produces verifiable records; intermittent crashes or rollbacks are fully chronicled for diagnostic autopsy.
2. **Replayability:** A `.scroll` can be fed directly back into `omnicode run` to deterministically reproduce any historical computation.
3. **Model Refinement:** Inscribed repairs provide ground truth training pairs for the next generation of neural-symbolic transducers.

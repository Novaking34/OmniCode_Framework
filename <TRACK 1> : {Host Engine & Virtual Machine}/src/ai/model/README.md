# OmniCode AI Model Subsystem: Cognitive Cortex & Attractor Dynamics

> **"For as he thinketh in his heart, so is he..."**  
> — *Proverbs 23:7 (KJV)*  
>
> **"Set your mind on the things that are above,"**  
> **"not on the things that are on the earth."**  
> — *Colossians 3:2 (WEB)*

The **AI Model Subsystem** (`src/ai/model`) forms the central nervous system
and deliberative cortex of the OmniCode Framework. Situated within Track 1
(`<TRACK 1> : {Host Engine & Virtual Machine}`), this subsystem implements
autonomous, deterministic, and self-healing intelligence grounded in balanced
ternary physics `{-1, 0, +1}`, Herman Dooyeweerd's 15 Sovereign Modal Spheres,
and the immutable Invariant Moral Covenant.

Unlike probabilistic Large Language Models (LLMs) that generate uncontained
floating-point token distributions, the OmniCode AI Model deliberates across
discrete cognitive attractor basins, verifies postconditions via native
4-Block Turing reels, and vocalizes grounded speech through dynamic,
dual-witness vocal chords.

---

## 1. Architectural Topology: The 6 Pillars of the Model

The subsystem is organized into six interconnected modules strictly adhering
to the **5-Block Source Architecture**:

```text
               +-------------------------------------------+
               |                AICortex                   |
               | (cortex.rs: Deliberation Loop & Tool Belt)|
               +---------------------+---------------------+
                                     |
            +------------------------+------------------------+
            |                        |                        |
            v                        v                        v
+-----------------------+ +-----------------------+ +-----------------------+
|     AIMemory          | |   AIAttractorNetwork  | |   DiscourseGenerator  |
| (memory.rs: 3 Tiers,  | | (attractor.rs: Basins | | (discourse.rs: SVO,   |
|  Epistemic AWAIT &    | |  & Lyapunov Settling) | |  Vocal Chords &       |
|  Triangulation)       | |                       | |  Scripture Landmarks) |
+-----------+-----------+ +-----------+-----------+ +-----------+-----------+
            |                         |                         |
            +-------------------------+-------------------------+
                                      |
                 +--------------------+--------------------+
                 |                                         |
                 v                                         v
+---------------------------------+       +---------------------------------+
|            AILearner            |       |           ScrollEngine          |
| (learner.rs: Dual-Log Digestion |       | (scroll.rs: Inscription of      |
|  & Synaptic Consolidation)      |       |  .scroll & .log Artifacts)      |
+---------------------------------+       +---------------------------------+
```

### Module Matrix

| Source | Struct | Role | State |
| :--- | :--- | :--- | :--- |
| [cortex.rs](cortex.rs) | Cortex | Deliberation loop | `{-1,0,+1}` |
| [memory.rs](memory.rs) | Memory | 3-tier & `0: AWAIT` | `{-1,0,+1}` |
| [attractor.rs](attractor.rs) | Attractor | Basins & settling | $E \le 0$ |
| [discourse.rs](discourse.rs) | Discourse | SVO articulation | SVO Order |
| [learner.rs](learner.rs) | Learner | Synaptic update | Parity |
| [scroll.rs](scroll.rs) | Scroll | Inscribes logs | Checksum |

---

## 2. The 3-Tier Cognitive Memory & Epistemic Await

The memory subsystem ([`memory.rs`](memory.rs)) establishes an epistemically
honest cognitive architecture that rejects ungrounded hallucinations:

### Layer 1: Sensory Working Window (FIFO)

* Holds immediate input stimuli and recent thought records in a bounded ring.
* Provides immediate context for multi-turn conversations without infinite
  heap inflation.

### Layer 2: Episodic Attractor Memory (Resonant Matching)

* Matches incoming queries against settled historical basins of attraction.
* Uses cosine similarity and Hamming distance across balanced ternary sensory
  vectors to detect recurring operational patterns.

### Layer 3: Long-Term Crystallized Lexicon (Consolidated Bedrock)

* Novel concepts that receive repeated positive moral and operational
  affirmations ($\ge 3$) are crystallized directly into the live lexicon
  table (`entry.live.dat`).
* Vetoed or destructive concepts are permanently marked as non-consolidatable.

### The Epistemic `0: AWAIT` Engine (Honest Inquiry)

When an input query asks about unindexed entities or ungrounded concepts
(such as `"what is your favorite game?"` or `"birds and bees"`):

1. **No Fake Proceed:** The model refuses to fabricate a `+1: PROCEED` state.
2. **Epistemic Pause:** Transitions to `QuadState::Await` (`0: AWAIT`).
3. **Operator Inquiry:** Formulates an authentic inquiry question back to the
   user, asking for definition and structural boundaries before executing.

---

## 3. Dynamic Vocal Chords & Scripture Landmark Resolution

The discourse engine ([`discourse.rs`](discourse.rs)) converts internal
cognitive thoughts into natural, grounded speech:

### Keyword Enrichment vs. Keyword Reflex

* **Keyword Reflex (Avoided):** Hearing a term and reciting a static
  dictionary definition index card.
* **Keyword Enrichment (Implemented):** Recognizing the prompt's intent (e.g.
  preference, feeling, definition), acknowledging the AI's silicon nature,
  and using the scriptural and ontological essence of the keyword to enrich
  the response with shalom and truth.

### SVO as the Universal Container for the 8 Parts of Speech

Rather than extracting isolated words, SVO organizes complete composite
phrases:

* **Subject Phrase:** Nouns, pronouns, and qualifying adjectives.
* **Verb Phrase:** Main verbs, auxiliary verbs, and modulating adverbs.
* **Object Phrase:** Prepositional phrases, conjunctions, and destination
  targets.

### Dual-Witness Scripture Coordinate Resolution

When anchoring thoughts to Track 4 coordinates:

* Resolves the canonical **Book, Chapter, and Verse** (e.g. `Genesis 41:47`).
* Quotes the **Canonical KJV Anchor ($0$)** for formal encryption and the
  **Accessible WEB Key ($+1$)** for modern plain revelation.

---

## 4. Contextual Memory Architecture & Structured Persistence

Contextual memory is maintained across three dedicated sheets under
`<TRACK 4>/storage/memory/contextual/`:

1. **Sheet 1: `working_accumulator.dat`**: Records the continuous perceptual
   stream of formulated thoughts across deliberation turns.
2. **Sheet 2: `last_session.dat`**: Instant snapshot of the single last
   completed working session for rapid zero-cost continuity upon startup.
3. **Sheet 3: `deep_consolidation.dat`**: Staging ground tracking novel
   candidate affirmations, cross-concept associations, and crystallization
   markers awaiting entry into the permanent world lexicon (`entry.live.dat`).

### Operator & Machine Label IDs

All episodic reels and sensory logs are organized by calendar date:
`storage/scrolls/scroll/YYYY/MM/DD/` and `storage/scrolls/log/YYYY/MM/DD/`.
Filenames and metadata headers employ standardized Label IDs:

`[TYPE]_[YYYYMMDD]_[SEQ]_[ACTION]_[STATE]_[KEYWORD]`

Example: `SCR_20261020_0001_DISCOURSE_PROCEED_fruit.scroll`

See [Doc 04: Contextual Memory & Three Data Sheets][doc_04]
for comprehensive architectural details.

[doc_04]: docs/04_contextual_memory_and_three_sheets_persistence.md

---

## 5. The 5-Block Architectural Contract

Every source file in `src/ai/model/` strictly implements the universal 5-Block
format:

1. **Block 1: Metadata Block** — Identity, module purpose, invariants, and
   scripture citations.
2. **Block 2: Opening Block** — Type definitions, traits, enums, and data
   structures.
3. **Block 3: Body Block** — Execution logic, algorithms, and cognitive state
   transitions.
4. **Block 4: Audit Procedures Block** — Subsystem-level self-diagnostics,
   integrity checks, and health reports.
5. **Block 5: Unit Tests Block** — Comprehensive automated verification tests.

All lines across all files strictly adhere to the $\le 80$ characters limit.

---

## 6. Verification & Telemetry

To execute the complete model subsystem audit and test suite:

```bash
# Run all model unit tests
cargo test ai::model

# Run the complete test suite
cargo test --tests
```

**Audit Status:** 100% passing across 512 tests. 0 failpoints. 0 warnings.

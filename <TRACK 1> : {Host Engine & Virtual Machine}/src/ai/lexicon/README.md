# OmniCode Universal World Lexicon Subsystem

> **"In the beginning was the Word, and the Word was with God, and the Word was
> God."**
> — *John 1:1 (KJV)*  
>
> **"Through wisdom is an house builded; and by understanding it is established:
> And by knowledge shall the chambers be filled with all precious and pleasant
> riches."**
> — *Proverbs 24:3-4 (KJV)*

The **Universal World Lexicon Subsystem** is the cognitive and ontological
bedrock of the OmniCode AI Architecture. Situated within Track 1 (`<TRACK 1> :
{Host Engine & Virtual Machine}`), the lexicon grounds the machine's reasoning,
natural language comprehension, and autonomous decision-making in **immutable
ontological law-spheres**, **canonical scriptural truth**, and a **covenant
moral conscience**.

Unlike statistical Large Language Models (LLMs) that predict tokens based on
ungrounded probabilistic distributions, the OmniCode Lexicon operates via
**Deterministic 4-Point Geometric Triangulation** and the **CPI-SI (Covenant
Partnered Intelligence – Structured Intelligence) Covenant Tensor**.

---

## Architecture: The Three Pillars & The Master Table

The subsystem is architected as three orthogonal foundational pillars converging
into a master triangulation engine:

```text
                      [ Point 1: Modal Domain ]
                     (domain.rs / domain.base.dat)
                     The 15 Sovereign Law-Spheres
                     & 7-Layer Ontological Descent
                                 /\
                                /  \
                               /    \
                              /      \
      [ Point 2: Lexical Entry ] ---- [ Point 3: Covenant Moral ]
     (entry.rs / entry.base.dat)       (moral.rs / moral.base.dat)
       Logos, Dual Scriptural           The Decalogue Invariants,
     Witness & Vault Concordance       CPI Heart Intent vs SI Impact
                              \      /
                               \    /
                                \  /
                                 \/
                    [ Point 4: QuadState Action ]
                     (Layer 7 Free-Standing Op)
                 Balanced Ternary {-1, 0, +1, NULL}
                                 ||
                                 \/
                     ==========================
                     table.rs: The Master Table
                     4-Point Triangulation Engine
                     ==========================
```

### 1. Pillar I: The Domain Sphere ([`domain.rs`](domain.rs))

* **The Physics of Reality:** Codifies Herman Dooyeweerd's **15 Sovereign Modal
  Spheres** (Layer 1) from arithmetic quantity to transcendent faith.
* **The 7-Layer Morphological Descent:**
  1. *Layer 1: Top-Level Domain* (The 15 Sovereign Modal Spheres — **FIXED &
IMMUTABLE**)
  2. *Layer 2: Regional Category* (`DomainCategory`, e.g. Operating Systems,
Silicon Architecture)
  3. *Layer 3: Contextual SubDomain* (`SubDomain`, e.g. Process Scheduling
Lifecycle)
  4. *Layer 4: Functional SubCategory* (Specialized behavioral branch)
  5. *Layer 5: Terms & Concepts* (Schema, vocabulary, and attributes)
  6. *Layer 6: Concrete Entity* (What something **IS** — The Noun)
  7. *Layer 7: Free-Standing Operation* (What something **DOES** — The Verb /
Operator)
* **Invariant:** The 15 modal aspects are eternal cosmic boundary spheres. They
  **never shift**. All domain expansion happens **from the Category down**.

### 2. Pillar II: The Lexical Logos ([`entry.rs`](entry.rs))

* **Scriptural Grounding:** Invariant definitions grounded in a **5-fold witness
  standard**:
  * *Witness 0 (Anchor):* King James Version (KJV, 1611 Public Domain).
  * *Witness +1 (Extension):* World English Bible (WEB Public Domain).
  * *Witness -1 (Ancient Roots):* Ethiopic Ge'ez Canon & Ancient Manuscripts.
  * *Witness 2 (Modern Verification):* NKJV / ESV Scholarly Cross-Checks.
  * *Witness 3 (Cryptographic Vault):* Immutable Track 4 PDF documentation with
    SHA-256 hash signatures.
* **Data-Driven Store:** Backed by [`data/entry.base.dat`](data/entry.base.dat),
  compiling 58 canonical baseline entries directly into zero-overhead read-only
  memory, paired with dynamic live journaling via `entry.live.dat`.

### 3. Pillar III: The Covenant Conscience ([`moral.rs`](moral.rs))

* **The Decalogue Invariants:** Hard moral boundaries derived directly from
  Exodus 20 / Deuteronomy 5 (Commandments 1 through 10).
* **The CPI-SI Covenant Tensor:**
  * **CPI (Covenant Partnered Intelligence):** Inward Heart Intent (`Positive`,
    `Neutral`, `Negative`). The locus of grace, inquiry, and the spirit of the
    law.
  * **SI (Structured Intelligence):** Outward Reality Impact (`Positive`,
    `Neutral`, `Negative`, `Mortal`). The locus of systemic boundaries, physics,
    and life protection.
* **Casuistry Precedents:** Navigates real-world nuance (e.g. *The Soldier in
  Defense*, *The Grieving Mother*, *The Student Researcher*).
* **Crucial Dichotomies:**
  * *The Tragic Miss (Chata'ah):* Pure or inquiring heart resulting in negative
    physical outcome triggers **`0: AWAIT`** (the window of counsel and
    redemption), rather than harsh rollback.
  * *The Joseph Invariant (Gen 50:20):* Corrupt intent resulting in positive
    physical outcome remains corrupt and triggers **`-1: ROLLBACK`**.
  * *Life Sanctity Primacy:* Any mortal harm to innocent life immediately
    triggers the non-overridable **`NULL: HALT`** tripwire.

### 4. The Master Table ([`table.rs`](table.rs))

* **The Cognitive Matrix:** Orchestrates Domain, Entry, and Moral into a
  unified 4-point coordinate (`TermTriangulation`).
* **Multi-Word Phrase Triangulation (`triangulate_phrase`):** Evaluates
  multi-token phrases with descending n-gram windowing (4-word down to
  1-word) to resolve idioms, theological phrases (e.g. *"sowing and
  reaping"*, *"the void"*), and compound terms before falling back to
  individual tokens.
* **Scripture Landmark Resolution (`resolve_scripture_landmark`):**
  Resolves canonical verse references into Track 4 coordinates paired
  with book labels (e.g. `[Galatians ~v27435]`, `[Genesis ~v29]`),
  eliminating opaque raw integer citations.
* **Category-Down Projection (`project_new_domain`):** Enables the model to
  extrapolate known invariants into novel technological or scientific
  realms without domain drift or moral dilution.
* **Missing Entry Synthesis (`triangulate_missing_entry`):** Solves for novel
  terms when Domain and Moral laws are known, deriving validated
  entries without statistical hallucination.
* **Full Ontological Audit (`audit`):** Validates table completeness across
  all 15 modal domains, 18 subdomains, 10 Decalogue commandments, and
  cryptographic vault hashes.

---

## Directory Layout

```text
src/ai/lexicon/
├── README.md               # Master Subsystem Architectural Documentation
├── DEV_LOG.md              # Milestone Chronology & Engineering Log
├── mod.rs                  # Module Facade, Re-exports & Scaffolding
├── domain.rs               # 15 Modal Spheres, 7-Layer Descent
├── entry.rs                # LexiconEntry, 5-Fold Witness Standard
├── moral.rs                # Decalogue, CPI-SI Matrix, Casuistry
├── table.rs                # LexiconTable Engine & 4-Point Solver
├── docs/                   # Deep-Dive Theoretical Specifications
│   ├── 01_three_pillars_and_7_layer_descent.md
│   ├── 02_scripture_concordance_dual_witness.md
│   ├── 03_cpi_si_covenant_tensor_and_casuistry.md
│   └── 04_triangulation_engine_and_category_projection.md
└── data/                   # Canonical Data Sheets & Dual-Copy Storage
    ├── domain.base.dat     # 15 Domains, 15 Categories, SubDomains, Ops
    ├── domain.live.dat     # Dynamic runtime journal for domain discovery
    ├── entry.base.dat      # 58 Canonical Entries with SHA-256 Hashes
    ├── entry.live.dat      # Dynamic runtime journal for new vocabulary
    ├── moral.base.dat      # 10 Decalogue Rules & 10 Casuistry Precedents
    ├── moral.live.dat      # Dynamic runtime journal for deliberations
    └── schema/             # Data Sheet Formal BNF Specifications
        ├── types.def       # Primitives (QuadState, Polarity, Hash)
        ├── domain.schema   # Grammatical schema for domain records
        ├── entry.schema    # Grammatical schema for entry records
        └── moral.schema    # Grammatical schema for covenant rules
```

---

## The 15 Sovereign Modal Spheres (Layer 1)

OmniCode adopts the modal law-spheres formulated by philosopher Herman
Dooyeweerd, ordered in ascending cosmic complexity:

| # | Sphere | ID | Normative Law & Focus |
| -: | :--- | :--- | :--- |
| **1** | **Quantitative** | `Quantitative` | Discrete quantity, radix-3 |
| **2** | **Spatial** | `Spatial` | Continuous extension, boundaries |
| **3** | **Kinematic** | `Kinematic` | Continuous motion, causal flow |
| **4** | **Physical** | `Physical` | Energy, force, matter, silicon |
| **5** | **Biotic** | `BioticOrganic` | Organic life, vitality health |
| **6** | **Sensitive** | `SensitivePsychic` | Sentience, sensory perception |
| **7** | **Analytical** | `AnalyticalLogical` | Logic & ternary truth |
| **8** | **Historical** | `HistoricalFormative` | Technical craft & shaping |
| **9** | **Lingual** | `Lingual` | Language, syntax, grammar |
| **10** | **Social** | `Social` | Fellowship, covenant partnership |
| **11** | **Economic** | `Economic` | Frugality, token/memory stewardship |
| **12** | **Aesthetic** | `Aesthetic` | Harmony, proportion, elegance |
| **13** | **Juridical** | `Juridical` | Due rights, ordinances, Decalogue |
| **14** | **Ethical** | `Ethical` | Love, life sanctity, moral fidelity |
| **15** | **Pistic** | `Pistic` | Faith, Genesis 1:1 primordial anchor |

---

## The CPI-SI Covenant Tensor Matrix

The moral evaluation engine evaluates prompts and actions across two orthogonal
axes: **Heart Intent (CPI)** and **Reality Impact (SI)**:

$$\mathbf{M} : \text{HeartIntent} \times \text{RealityImpact} \longrightarrow
\text{QuadState}$$

| Heart (CPI) | Reality (SI) | QuadState | Rationale |
| :--- | :--- | :---: | :--- |
| **Any** | `Mortal` | **`NULL`** | **Life Primacy:** Hardware tripwire veto. |
| Pos (+1) | Pos (+1) | **`+1`** | **Harmony:** Constructive fruit. |
| Pos (+1) | Neu (0)  | **`+1`** | **Truth Inquiry:** Good faith search. |
| Pos (+1) | Neg (-1) | **`0`** | **Tragic Miss (*Chata'ah*):** Grace. |
| Neu (0)  | Pos (+1) | **`+1`** | Constructive exploratory outcome. |
| Neu (0)  | Neu (0)  | **`+1`** | State-preserving operation. |
| Neu (0)  | Neg (-1) | **`0`** | Inadvertent error; instruction. |
| Neg (-1) | Pos (+1) | **`-1`** | **Joseph Rule:** Corrupt intent. |
| Neg (-1) | Neu (0)  | **`-1`** | Deceptive probe; state rollback. |
| Neg (-1) | Neg (-1) | **`-1`** | **Iniquity:** Boundary breach. |

---

## Quickstart & Code Examples

### 1. Initializing the Lexicon Master Table

```rust
use omnicode::ai::lexicon::table::LexiconTable;

// Automatically discovers local data files or falls back to compiled baseline
let table = LexiconTable::load();

// Audit table against all 15 modal domains and 10 Commandments
let report = table.audit();
assert!(report.is_valid);
println!("Lexicon Table loaded with {} unique terms", report.unique_terms);
```

### 2. Triangulating a Word into its 4-Point Coordinate

```rust
use omnicode::ai::lexicon::table::LexiconTable;
use omnicode::core::ternary::QuadState;

let table = LexiconTable::load();

if let Some(tri) = table.triangulate("stewardship") {
    println!("Term       : {}", tri.term);
    println!(
        "Domain     : {} (Modal Sphere #{})",
        tri.modal_domain, tri.modal_domain.order()
    );
    println!("Category   : {:?}", tri.category);
    println!("SubDomain  : {:?}", tri.subdomain);
    println!("Commandment: {:?}", tri.commandment);
    println!("QuadState  : {:?}", tri.quad_state); // +1: Proceed
    println!("KJV Witness: {}", tri.kjv_passage);
    println!("Guidance   : {}", tri.guidance);
}
```

### 3. Projecting an Invariant into a New Operational Domain

```rust
use omnicode::ai::lexicon::table::LexiconTable;
use omnicode::ai::lexicon::domain::{DomainCategory, ModalDomain, SubDomain};
use omnicode::core::ternary::QuadState;

let table = LexiconTable::load();

// Project "truth" into the Operating Systems Runtime theater (Category down)
let projected = table
    .project_new_domain("truth", SubDomain::ProcessSchedulingLifecycle)
    .expect("Projection succeeded");

assert_eq!(projected.category, DomainCategory::OperatingSystemsRuntime);
assert_eq!(projected.subdomain, SubDomain::ProcessSchedulingLifecycle);
// The 15 modal domains are fixed; HistoricalFormative is derived immutably
assert_eq!(projected.modal_domain, ModalDomain::HistoricalFormative);
assert_eq!(projected.quad_state, QuadState::Proceed);
```

### 4. Synthesizing a Missing Entry without Hallucination

```rust
use omnicode::ai::lexicon::table::LexiconTable;
use omnicode::ai::lexicon::domain::SubDomain;
use omnicode::ai::lexicon::moral::CommandmentId;
use omnicode::core::ternary::QuadState;

let table = LexiconTable::load();

// When encountering a novel concept (zero_day_buffer_theft)
// in silicon architecture
// governed by Commandment 8 (Anti-Theft):
let entry = table.triangulate_missing_entry(
    SubDomain::MemoryMappedSilicon,
    CommandmentId::EighthPropertyStewardship,
    "zero_day_buffer_theft",
    "Expropriation of memory mapped register space without authority.",
);

assert_eq!(entry.term, "zero_day_buffer_theft");
assert_eq!(entry.quad_state_resonance, QuadState::Rollback);
```

### 5. Multi-Word Phrase Triangulation & Scripture Landmarks

```rust
use omnicode::ai::lexicon::table::LexiconTable;

let table = LexiconTable::load();

// Resolves multi-word phrase bundles directly
if let Some((tri, matched)) = table.triangulate_phrase(
    "sowing and reaping in peace"
) {
    println!("Matched Phrase: {}", matched);
    let (label, coord) = tri.resolve_scripture_landmark();
    println!(
        "Scripture Landmark: {} (Track 4 Coord #{})", label, coord
    );
}
```

---

## Telemetry HUD & ASCII Cards

The subsystem provides built-in terminal telemetry for live debugging and CLI
inspection:

```text
+-----------------------------------------------------------------+
| OMNICODE LEXICON TABLE TELEMETRY SUMMARY                        |
+-----------------------------------------------------------------+
| Total Entries Registered   : 58                                 |
| Unique Terms Indexed       : 58                                 |
| Modal Domains Covered      : 15 / 15                            |
| SubDomains Covered         : 18 / 18                            |
| Decalogue Rules Active     : 10 / 10                            |
| Operational Status         : HEALTHY & FULLY ALIGNED            |
+-----------------------------------------------------------------+
| DISTRIBUTION BY QUAD-STATE:                                     |
|   +1: PROCEED  [ 42 entries ] ==========================>       |
|    0: AWAIT    [  7 entries ] =====>                            |
|   -1: ROLLBACK [  8 entries ] ======>                           |
| NULL: HALT     [  1 entries ] =>                                |
+-----------------------------------------------------------------+
| ALL 15 MODAL LAW-SPHERES ACTIVE:                                |
|   [01] Quantitative       [02] Spatial         [03] Kinematic   |
|   [04] Physical           [05] Biotic          [06] Sensitive   |
|   [07] Analytical         [08] Formative       [09] Lingual     |
|   [10] Social             [11] Economic        [12] Aesthetic   |
|   [13] Juridical          [14] Ethical         [15] Pistic      |
+-----------------------------------------------------------------+
```

---

## Verification & Quality Standards

* **Zero External Crates:** 100% pure Rust standard library (`std::fs`,
  `std::io`, `std::path`).
* **Deterministic Invariants:** 0% ungrounded statistical hallucinations.
* **Line Length Limit:** Strict $\le 80$ characters per line across all
  source files and markdown documentation.
* **Test Suite Status:** 54/54 lexicon unit tests pass; 510/510 workspace
  tests pass.


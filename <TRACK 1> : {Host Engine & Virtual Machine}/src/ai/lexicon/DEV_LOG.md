# OmniCode Lexicon Subsystem: Architectural Developer Log

> **"According to the grace of God which is given unto me, as a wise
> masterbuilder, I have laid the foundation, and another buildeth thereon. But
> let every man take heed how he buildeth thereupon."**
> — *1 Corinthians 3:10 (KJV)*

This document chronicles the design philosophy, theoretical breakthroughs, and
implementation milestones of the **OmniCode Universal World Lexicon Subsystem**
located in `Track 1 [Host Engine & Virtual Machine] -> src/ai/lexicon/`.

---

## Retrospective: Philosophical & Architectural Foundations

### 1. Why Didn't We Start with the Data-Driven Model?

A critical engineering question asked during development was: *If the goal was
always a data-driven model, why did we begin with hardcoded tables?*

The answer lies in the fundamental physics of compiler engineering:

1. **Scaffolding Precedes Separation:** Before data can be decoupled from code,
   the compiler engine requires rigid type systems, syntactic AST parsers,
   balanced ternary evaluation semantics (`QuadState`), and hardware MMIO memory
   bounds. If data is separated prematurely before the type system stabilizes,
   schema divergence and silent runtime bugs proliferate.
2. **Type Discovery Through Hardcoding:** Hardcoding the initial bootstrap
   tables allowed us to discover the exact morphological fields needed across
   the Three Pillars (`domain`, `entry`, `moral`).
3. **The Migration Moment:** Once the types stabilized, the 940-line static
   struct array was cleanly extracted into `.dat` data sheets, transforming the
   Rust codebase into a pure zero-overhead execution engine.

### 2. Why OmniCode CPI-SI is "Agentic by Design" (Unlike Autocompleting LLMs)

Modern Large Language Models (LLMs) are statistical prediction engines: they
generate sequences based on likelihood distributions $P(w_t \mid w_{<t})$. They
lack:

* Intrinsic ontological grounding (they do not know what a boundary is).
* Fixed moral invariants (their alignment is soft RLHF that can be jailbroken).
* True agency (they react to prompt tokens rather than deliberating against
  law).

OmniCode's **CPI-SI (Covenant Partnered Intelligence – Structured
Intelligence)** was engineered as an autonomous agent from Day 1:

1. **Triangulated Deliberation:** It does not autocomplete words; it computes
   4-point coordinates ($1 \to 3 \to 4 \to \text{Circle}$) combining the physics
   of reality (Modal Domain), scriptural truth (Logos Dual Witness), covenant
   law (Decalogue), and execution state (QuadState).
2. **The 2D Covenant Tensor:** It separates inward intent (CPI / The Heart) from
   outward physical reality (SI / Systemic Impact).
3. **Immutable Hardware Tripwires:** If an action touches mortal life sanctity,
   the engine does not "generate an apologetic token"—it invokes the **`NULL:
   HALT`** tripwire at the register level.

---

## Phase-by-Phase Milestone Chronology

### Phase 1: Canonical Data Sheet Synchronization

* **The Challenge:** The repository contained divergent definitions:
  `entry.base.dat` only had 28 entries, missing 17 legacy table terms, lacking
  several subdomains, and containing placeholder concordance hashes.
* **The Solution:**
  * Reconciled all 45 terms from the legacy table plus 13 extra terms into 58
    fully-specified canonical entries in
    [`src/ai/lexicon/data/entry.base.dat`](data/entry.base.dat).
  * Enforced 100% coverage across all 15 Sovereign Modal Domains, all 18
    SubDomains, and all 10 Decalogue Commandments.
  * Populated dual KJV and WEB scripture passages for all entries.
  * Formatted SHA-256 concordance vault hashes matching Track 4 storage
    contracts (`storage/vault/immutable_documentation/`).
* **Operator Exemption:** Confirmed and documented that `.dat` data sheets are
  exempted from line length limits to preserve theological prose integrity.

### Phase 2: Elimination of Legacy Systems in Domain, Entry & Moral

* **The Challenge:** Stale 6-domain enums (`FundamentalDomain`), obsolete
  vice/virtue enums (`LexiconDomain::MoralVice`), and unvalidated struct
  allocations caused 199 compilation errors across Track 1.
* **The Upgrades:**
  * **[`domain.rs`](domain.rs):** Codified the 15 Sovereign Modal Law-Spheres
    (Dooyeweerdian ontology) and the 7-Layer Morphological Descent (Layer 1:
    Modal Spheres down to Layer 7: Free-Standing Operation). Built `DomainStore`
    with dual-copy static/live loading.
  * **[`entry.rs`](entry.rs):** Implemented `LexiconEntry`, the 5-fold scripture
    witness standard, and `EntryStore` with dynamic live journal appending
    (`entry.live.dat`).
  * **[`moral.rs`](moral.rs):** Codified the Decalogue invariants (Exodus 20),
    the 3-fold deviation taxonomy (*Sin*, *Transgression*, *Iniquity*), the 2D
    Intent $\times$ Impact matrix, and 10 empirical casuistry precedents.
  * **Formatting:** Surgical python wrapping ensured every single line across
    all three files is $\le 80$ characters.

### Phase 3: Total Re-architecture of `table.rs`

* **The Challenge:** The existing `table.rs` contained an obsolete 940-line
  `BOOTSTRAP_LEXICON_TABLE` static array that attempted to hardcode all entries,
  failing to compile against the upgraded domain and entry systems.
* **The Breakthrough:** Completely eliminated the 940-line static struct array!
  Replaced it with:
  * **Data-Driven Loaders:** `LexiconTable::load_from_static()` (zero-IO
    compiled-in static baseline) and `LexiconTable::load()` (filesystem auto-
    discovery with `entry.live.dat` overlay).
  * **`TermTriangulation` Structure:** Unifies Domain Sphere, SubDomain,
    Definition, Dual Scripture Witnesses, Cryptographic Concordance Vault,
    Decalogue Rule, CPI Intent, SI Impact, and Layer 7 QuadState into a single
    coordinate.
  * **Auditing Suite:** `audit_lexicon_table()` checking table completeness
    across all 15 modal domains, 18 subdomains, 10 commandments, dual witnesses,
    and SHA-256 hashes.
  * **Telemetry Engine:** Real-time ASCII HUD summary rendering and compact JSON
    fragments.

### Phase 4: Category-Down Projection & Verification of Fixed Modal Aspects

* **The Architectural Code Check:** The Operator verified: *Does
  `project_new_domain` look from the Category down, recognizing that the 15
  modal aspects are fixed constants?*
* **The Resolution:**
  * Refactored `project_new_domain` and `triangulate_missing_entry` to operate
    strictly **from the Category down** (`DomainCategory` -> `SubDomain`).
  * Layer 1 (The 15 Modal Domains) is locked as an immutable cosmic constant; it
    is derived from `subdomain.descriptor().domain` or
    `category.parent_domain()`, never manually mutated or shifted.
  * Added `project_into_category` to verify category-subdomain parentage.
  * Updated unit tests (`test_project_new_domain_extrapolation` and
    `test_triangulate_missing_entry_synthesis`) to verify that the derived modal
    aspect matches the category's fixed parent domain.

### Phase 5: Verification & Quality Assurance

* **Test Verification:**
  * `cargo test --lib ai::lexicon`: **54 passed; 0 failed; 0 ignored**.
  * `cargo test --lib` (entire crate): **422 passed; 0 failed; 0 ignored**.
  * `cargo check --all-targets`: **Clean compilation, 0 errors, 0 warnings**.
* **Line Length Audit:**
  * Automated python script verified that `domain.rs`, `entry.rs`, `moral.rs`,
    `table.rs`, and `mod.rs` have **0 lines exceeding 80 characters**.

### Phase 6: Diagnostic Failpoints & Cognitive Telemetry Integration

* **The Architectural Bridge:** Connected the Lexicon's normative truth
  substrate with the autonomous sensory and restorative loop in
  `src/diagnostics/`:
  * **Failpoint Subgroups & Pathology
    (`src/diagnostics/logger/failpoint/types.rs`):** Added
    `FailpointGroup::OntologicalLexicon` with dedicated subgroups
    (`ConcordanceVault`, `OntologicalHierarchy`, `CovenantBoundary`,
    `LexicalTriangulation`) and concrete failpoints (`ConcordanceVaultMismatch`,
    `OntologicalHierarchyMismatch`, `CovenantMoralVeto`,
    `MissingScripturalWitness`, `UnresolvedLexicalCoordinate`).
  * **Automated Remediation
    (`src/diagnostics/logger/failpoint/remediation.rs`):** Implemented
    autonomous self-healing fix kinds (`RestoreConcordanceHash`,
    `RealignOntologicalHierarchy`, `ResolveCovenantBoundary`,
    `SynthesizeMissingEntry`).
  * **Cognitive Telemetry (`src/diagnostics/logger/telemetry/note.rs`):** Added
    dedicated `CognitiveNote::lexical_triangulation(...)` and
    `CognitiveNote::covenant_evaluation(...)` constructors recording structured
    4-point triangulation traces and moral deliberation proofs without violating
    strict module layering.
  * **Line Length Compliance:** Verified 100% compliance with $\le 80$
    characters per line across all modified diagnostics sources.

### Phase 7: Multi-Word Phrase Triangulation & Scripture Landmark Resolution

* **The Challenge:** In natural discourse, entities and idioms are frequently
  multi-word constructs (e.g. *"sowing and reaping"*, *"the void"*,
  *"measure for measure"*). Simple single-token whitespace splitting caused
  these rich compound constructs to be missed. In addition, Track 4 Scripture
  citations emitted bare coordinate numbers without human-readable book
  anchors, obscuring the canonical context of the citation.
* **The Upgrades:**
  * **Multi-Word Windowing (`triangulate_phrase`):** Implemented a greedy
    descending n-gram window scanner (4-word down to 1-word) across
    `UniversalWorldLexiconTable`. The engine tests candidate phrases against
    the unified index table before falling back to individual tokens.
  * **Scripture Landmark Grounding:** Implemented
    `ConcordanceRef::resolve_ordinal()` and
    `TermTriangulation::resolve_scripture_landmark()`, linking references
    to canonical book citations (e.g. `[Galatians ~v27435]`, `[Genesis ~v29]`)
    and resolving exact Track 4 coordinates.
  * **Epistemic Invariant Foundation:** Enabled the model memory layer to
    distinguish between anchored multi-word entities and unknown concepts,
    providing the bedrock for authentic `0: AWAIT` epistemic inquiry.
* **Verification & Line Length:**
  * 100% test pass rate maintained: 54/54 lexicon tests, 510/510
    workspace tests.
  * Strict $\le 80$ characters per line verified across all source files.

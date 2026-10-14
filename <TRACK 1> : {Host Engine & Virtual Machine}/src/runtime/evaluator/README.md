# OmniCode Runtime Evaluator Subsystem

```text
==============================================================================
BLOCK 1: METADATA & ARCHITECTURAL MANIFESTO
==============================================================================
Subsystem: OmniCode Host Engine & Virtual Machine Evaluator
Layer:     AST Execution Engine & Quad-State Operational Reducer
Path:      <TRACK 1> : {Host Engine & Virtual Machine}/src/runtime/evaluator/
==============================================================================
```

> **"And he gave unto Moses, when he had made an end of communing with him upon mount Sinai, two tables of testimony, tables of stone, written with the finger of God."**  
> — *Exodus 31:18 (KJV)*  
> *(The immutable inscription of atomic runes onto bounded physical tablets).*
> **"Prepare thy work without, and make it fit for thyself in the field; and afterwards build thine house."**  
> — *Proverbs 24:27 (KJV)*  
> *(The discipline of proving atomic sub-engines and verification invariants before constructing complex systems).*
> **"Remove not the ancient landmark, which thy fathers have set."**  
> — *Proverbs 22:28 (KJV)*  
> *(The preservation of historical memory boundaries, silicon addresses, and machine constants).*

---

## 1. Architectural Mission

The **Evaluator Subsystem** is the operational execution engine of the OmniCode Virtual Machine. It reduces Abstract Syntax Trees (AST) under balanced ternary truth logic (`+1:PROCEED`, `0:AWAIT`, `-1:ROLLBACK`, `NULL:HALT`), evaluating causal vectors, relational algebra, and Subject-Verb-Object (SVO) action verbs.

By decoupling the Evaluator into specialized sub-modules:

1. **Relational Algebra (`relations.rs`)** operates independently of causal vectors, leveraging universal scalar unwrappers and cross-substrate glyph equality from the Environment.
2. **Directional Causality (`vectors.rs`)** guarantees non-corrupting state propagation without side-effects leaking back into source expressions.
3. **Action Verbs (`verbs.rs`)** drive the atomic Character rune lifecycle (`INSCRIBE`, `TRANSDUCE`, `PROJECT`, `SEQUENCE`) with mathematical fidelity.
4. **Adversative Caveats (`connectives.rs`)** enforce reversible computing under `! BUT` by rolling back state transactions when guard conditions fail.

---

## 2. Topological Layer Structure

```bash
src/runtime/evaluator/
├── universal/                  <-- Tier 1: Universal AST Decoders & Trit Transformers
│   ├── decoder.rs              <-- extract_symbol_name, extract_trits, trits_to_integer
│   └── mod.rs                  <-- Re-exports Universal Decoders
│
├── verbs.rs                    <-- Tier 2: SVO Action Verbs (INSCRIBE, TRANSDUCE, PROJECT, SEQUENCE)
├── relations.rs                <-- Tier 2: Being Relations (=, !=, ~=, <=>, <=, >=) & Capacity Bounds
├── vectors.rs                  <-- Tier 2: Directional Causal Propagation Vectors (--> , <--, ^--, v--)
├── connectives.rs              <-- Tier 2: Logical Connectives (! BUT, & AND, | OR, -- NOT, ::)
│
├── core.rs                     <-- Tier 3: Evaluator Coordinator, AST Dispatch Loop & Module Loader
├── mod.rs                      <-- Subsystem Facade & Backward-Compatible API
└── README.md                   <-- Architectural Manifesto & Specification
```

---

## 3. Subsystem Breakdown

### Universal (`universal/decoder.rs`)

- **Symbol Extraction**: Normalizes heterogeneous AST representations (`Identifier`, `Polarity`, `[ ROLL 1 ]`, `[ Verify ]`) into canonical symbol strings.
- **Trit Extraction & Horner Expansion**: Converts 5-trit balanced ternary representations to integer values using Horner's polynomial method ($\sum_{i=0}^{N-1} t_i \cdot 3^i$).
- **Mathematical Invariant**: Guarantees byte-to-trit parity (e.g. $65 \iff [-1, 1, 1, -1, 1]$).

### SVO Action Verbs (`verbs.rs`)

- **`INSCRIBE`**: Outside-in imposition of glyph bytes into 5-trit Cell containers.
- **`TRANSDUCE`**: Bijective Radix-3 conversion of 8-bit bytes to 5 balanced trits.
- **`PROJECT`**: Outward projection of Character runes to hardware Target buffers (`%into` port / 0x000B8000).
- **`SEQUENCE`**: Continuous accumulation of Character runes into the text ribbon.
- **Vessel Operations**: Memory vessel indexed access and dynamic Storehouse partitioning.

### Relational Algebra (`relations.rs`)

- Evaluates Being Relations under balanced ternary QuadState truth.
- Distinguishes **Observational Relations** (e.g. `{ Character' = Glyph }` checking equality via `is_glyph_equal`) from **Imperative State Advances** (e.g. `[ Status' = +1:PROCEED ]`).
- Enforces invariant capacity bound checks via `capacity_in_cells()` (e.g. `{ Character' <= [ 1 Cell ] } \implies +1:PROCEED`).

### Directional Causality (`vectors.rs`)

- **`-->` (ActUpon)**: Propels causal action forward without mutating or corrupting the source expression.
- **`<--` (ActAgain)**: Manages retroaction, feedback loops, and memory reads.
- **`^--` (BuildsTo)**: Hierarchical composition.
- **`v--` (BreaksDownFrom)**: Storehouse and vessel memory partitioning.

### Logical Connectives (`connectives.rs`)

- **`! BUT`**: Adversative caveat gate wrapping actions in atomic transactions; unwinds with `rollback_transaction()` on `-1:ROLLBACK`.
- **`& AND`, `| OR`, `-- NOT`**: Balanced ternary truth logic.
- **`:: IN_RELATION_TO`**: Module imports (`import :: lib.core.memory`) and namespace path resolution.

### Core Coordinator (`core.rs`)

- Coordinates AST node dispatching across specialized sub-engines.
- Manages recursive module loading, protecting against circular imports.
- Formulates quad-state truth projections.

---

## 4. Canonical Four-Block Layout

Every source file in the Evaluator subsystem strictly follows the **Canonical 4-Block Structure**:

1. **Block 1: Metadata Block**: File scope, scripture anchors, and AI/developer context rationale.
2. **Block 2: Opening Block**: Core imports, struct/enum definitions, and baseline initializers.
3. **Block 3: Body Block**: Business logic, operations, and trait implementations.
4. **Block 4: Closing Block**: Display formatting, default traits, and verification unit tests.

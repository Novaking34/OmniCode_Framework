# OmniCode Language: Foundational Documentation Series

Welcome to the architectural and theoretical specification of the **OmniCode Language**, a dual-readable, multi-tier programming language engineered for direct human comprehension and deterministic machine execution, grounded in the principles of **Formal Ontology**, **Set Theory**, and **Balanced Ternary Logic**, powered by a high-performance **Rust Core Engine**.

OmniCode bridges the entire spectrum of computing—operating with zero-overhead control at the bare-metal low level, while natively expressing high-level mathematical abstractions, declarative logic, balanced ternary state machines, and machine-readable cognitive intent.

---

## 1. Documentation Index & Topology

The specification is organized into four comprehensive volumes:

### [Volume 1: Foundations](01_foundations/)

* **[01. Ontological Principles & Set Theory](01_foundations/01_ontological_principles.md):** The philosophical and mathematical basis of entities, intrinsic essence vs. extrinsic context, and set comprehension.
* **[02. Boundary Types & Asides](01_foundations/02_boundary_types_and_asides.md):** The 6 morphological boundary delimiters (`[]`, `<>`, `{}`, `{]`, `[}`, `()`), the elimination of dead comments, and machine-readable intent.
* **[03. The 8 Parts of Speech & Grammatical Matrix](01_foundations/03_eight_parts_of_speech_matrix.md):** Universal linguistic grounding, the 8-word sentence proof, CS counterparts, and primary/secondary grammatical roles.

### [Volume 2: Logic & Algebra](02_logic_and_algebra/)

* **[01. Balanced Ternary & The Quad-State Engine](02_logic_and_algebra/01_balanced_ternary_quad_state.md):** The 4-state execution truth space (`-1` Rollback, `0` Await, `+1` Proceed, `NULL` Halt), algebraic negation, and physics-driven control flow.
* **[02. Relational Connectives & Operators](02_logic_and_algebra/02_relational_connectives_and_operators.md):** The connective algebra (`!` But, `::` And, `:` In relation to, `|` Or, `--` Not) and state relations (`=` Is, `!=` Is not, `≈` Liken to).

### [Volume 3: Architecture & Execution](03_architecture/)

* **[01. The 3D Coordinate System of Computation](03_architecture/01_coordinate_system_of_computation.md):** The three orthogonal axes: Being (Ontology), Dynamics (Causality: `-->` and `<--`), and Scale (Layering: `^--` builds to, `v--` breaks down from).
* **[02. The Unified Execution Pipeline](03_architecture/02_execution_pipeline.md):** How a single AST and IR seamlessly feeds AOT Native Compilation (LLVM), Real-time Interpretation (REPL/VM), and Cross-Platform Transpilation (C/WASM).
* **[03. The Universal 4-Block Architecture & Rust Foundation](03_architecture/03_four_block_architecture.md):** The deterministic 4-block file lifecycle (`Metadata` $\to$ `Opening` $\to$ `Body` $\to$ `Closing`), the `OmniLinter`, and the technical rationale for using Rust as the core foundation.

### [Volume 4: Cognitive & Autonomous Systems](04_cognitive_and_autonomous_systems/)

* **[01. The AI Cortex & Neural-Symbolic Transduction Loop](04_cognitive_and_autonomous_systems/01_ai_cortex_and_neural_symbolic_loop.md):** The end-to-end cognitive loop transforming natural language prompts into canonical 4-Block Turing reels, executing with quad-state determinism, and producing dual logs.
* **[02. Episodic Scrolls & Experiential Learning Archive](04_cognitive_and_autonomous_systems/02_episodic_scrolls_and_experiential_learning.md):** The dual inscription model (`.scroll` executable reels and `.log` telemetry) and `AILearner` episodic memory digestion.
* **[03. The Restorator & Autonomous Self-Healing Diagnostics](04_cognitive_and_autonomous_systems/03_autonomous_restorator_and_self_healing.md):** Autonomous boundary auto-healing, delimiter ranking hierarchy, missing block auto-scaffolding, and transactional rollback protection.
* **[04. Bare-Metal OS & Cross-Platform Toolchains](04_cognitive_and_autonomous_systems/04_bare_metal_and_cross_platform_toolchain.md):** Bare-metal targets (`x86_64`, `aarch64`, `riscv64gc`), QEMU hardware emulation, Docker toolchain, and ISO bootloaders.
* **[05. Core Standard Library Architecture](04_cognitive_and_autonomous_systems/05_core_standard_libraries.md):** Specification of the built-in standard library in `lib/core/` (`arithmetic.omni`, `logic.omni`, `vectors.omni`, `memory.omni`).

---

## 2. Symbol & Operator Matrix

| Axis / Domain | Symbol | Syntax Example | Meaning |
| :--- | :---: | :--- | :--- |
| **Being (Y-Axis)** | `=` | `a = b` | Identity assertion (True if identical) |
| | `!=` | `a != b` | Distinctness assertion (True if distinct) |
| | `~=` / `≈` | `a ~= b` | Homomorphism / Structural similarity |
| **Dynamics (X-Axis)** | `-->` | `a --> b` | Forward causality: 'a' acts upon 'b' |
| | `<--` | `b <-- a` | Retroactive feedback: 'b' reflects upon 'a' |
| **Scale (Z-Axis)** | `^--` | `a ^-- A` | Micro-to-Macro: 'a' builds up to 'A' |
| | `v--` | `A v-- a` | Macro-to-Micro: 'A' breaks down into 'a' |
| **Connective Algebra** | `!` | `a --> b ! [ c = +1 ]` | Adversative caveat (**BUT** condition) |
| | `::` | `a : A :: b : B` | Conjunction (**AND** / Consensus) |
| | `:` | `a : A` | Ontological aspect (**IN RELATION TO**) |
| | `\|` | `a \| b` | Disjunction (**OR**) |
| | `--` | `-- a` | Algebraic negation (**NOT**) |
| **Morphological Boundaries** | `[...]` | `[ data ]` | Rigid / Fixed / Physical Denotation |
| | `<...>` | `< type >` | Dynamic / Fluid / Invariant Process |
| | `{...}` | `{ universe }` | Universal / Ambient Context / Scope |
| | `{...]` | `{ A ]` | Top-Down / Absorber (Fluid in $\to$ Fixed out) |
| | `[...}` | `[ B }` | Bottom-Up / Generator (Fixed in $\to$ Fluid out) |
| | `(...)` | `( intent aside )` | First-class Aside (Machine-readable intent) |
| **Balanced Ternary Truth** | `+1` | `+1` / `PROCEED` | Monotonic forward commit |
| | `0` | `0` / `AWAIT` | Indeterminate / Awaiting consensus |
| | `-1` | `-1` / `ROLLBACK` | Transactional contra / Veto / Rollback |
| | `NULL` | `NULL` / `HALT` | Terminal absorbing trap / Void |

---

## 3. Canonical Showcase: [`examples/canonical_notation.omni`](file:///home/professorseanex/OmniCode/examples/canonical_notation.omni)

```omni
# ==============================================================================
# BLOCK 1: METADATA BLOCK
# ==============================================================================
Module < CanonicalNotationShowcase > ( Foundations of Being, Causality, and Abstraction ) {
    version: < "1.0.0" >,
    author: < "ProfessorSeanEX" >,
    system: < OmniCode.Core >,
    semantics: < BalancedTernary :: QuadState :: BoundaryPhysics >
}

A < +1 > ( Invariant category A: axiomatic ground truth )
B < +1 > ( Invariant category B: independent set domain )

# ==============================================================================
# BLOCK 2: OPENING BLOCK
# ==============================================================================
Opening {
    requires: [ MemoryAligned :: TypeChecking ],
    initial_truth: [ +1 ]
}

a [ +1 ] ( Particular instance 'a' with mutable state )
b [ +1 ] ( Particular instance 'b' with mutable state )
a_in_A [ a ] { A }  ( Instance 'a' validated within contextual universe A )
a_top_down { A ]    ( Outside-In: Environment A actively forces definition of a )
b_bottom_up [ B }   ( Inside-Out: Definition of b emerges from context B )

# ==============================================================================
# BLOCK 3: BODY BLOCK
# ==============================================================================
Body [
    instance_typing [ a : A ]    ( 'a' stands in relation to Category A )
    identity_check [ a = a ]     ( 'a' is identical to 'a' )
    distinction_check [ a != b ] ( 'a' is not 'b' )
    similarity_check [ a ~= b ]  ( 'a' is liken to 'b' in form )

    forward_action [ a --> b ]   ( 'a' acts upon 'b' forward in time )
    feedback_action [ b <-- a ]  ( 'b' reflects back upon 'a' as retroactive feedback )

    synthesis [ a ^-- A ]        ( Low-level primitive 'a' builds up to concept A )
    decomposition [ A v-- a ]    ( Category A decomposes down into primitive 'a' )

    negation [ -- a ]
    conjunction [ a : A :: b : B ]
    pipeline [ a --> b ! [ a : A :: b : B ] ] ( 'a' acts on 'b', BUT caveat must hold )
]

# ==============================================================================
# BLOCK 4: CLOSING BLOCK
# ==============================================================================
Closing {
    verify: [
        identity_check = +1 ::
        forward_action = +1 ::
        pipeline = +1
    ],
    commit_status: [ +1 ] ( Final transaction verification: Proceed )
}
```

---

## 4. Verification Suite & Invariant Seal

The theoretical specifications in this documentation are backed by executable code and passing unit/integration tests in the Rust core:

* **24 Automated Tests:**
  * Unit tests validating boundary hierarchies, vector parsers, and quad-state logic in [`src/lib.rs`](file:///home/professorseanex/OmniCode/src/lib.rs).
  * 17 Failpoint integration tests in [`tests/test_failpoints.rs`](file:///home/professorseanex/OmniCode/tests/test_failpoints.rs).
  * 2 AI Cortex and Episodic Learner integration tests in [`tests/test_ai_model.rs`](file:///home/professorseanex/OmniCode/tests/test_ai_model.rs).
* **CLI Diagnostic Tools:**
  * `omnicode lint <file>`: Enforces 4-Block architectural compliance.
  * `omnicode restore <file>`: Autonomous self-healing verification.
  * `omnicode ai "<prompt>"`: Cognitive loop verification.
  * `omnicode scrolls`: Episodic memory archive verification.

All specifications herein are invariant and formally sealed.

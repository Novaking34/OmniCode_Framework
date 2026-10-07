# OmniCode Language: Foundational Documentation Series

Welcome to the architectural and theoretical specification of the **OmniCode Language**, an AI-native, multi-tier programming language grounded in the principles of **Formal Ontology**, **Set Theory**, and **Balanced Ternary Logic**, powered by a high-performance **Rust Core Engine**.

OmniCode is designed to bridge the entire spectrum of computing—operating with zero-overhead control at the bare-metal low level, while expressing high-level abstractions, declarative logic, and semantic reasoning natively.

---

## Documentation Index

The specification is organized into three distinct volumes:

### [Volume 1: Foundations](01_foundations/)

* **[01. Ontological Principles & Set Theory](01_foundations/01_ontological_principles.md):** The philosophical and mathematical basis of entities, intrinsic essence vs. extrinsic context, and set comprehension.
* **[02. Boundary Types & Asides](01_foundations/02_boundary_types_and_asides.md):** The 6 morphological boundary delimiters (`[]`, `<>`, `{}`, `{]`, `[}`, `()`), the elimination of dead comments, and machine-readable intent.

### [Volume 2: Logic & Algebra](02_logic_and_algebra/)

* **[01. Balanced Ternary & The Quad-State Engine](02_logic_and_algebra/01_balanced_ternary_quad_state.md):** The 4-state execution truth space (`-1` Rollback, `0` Await, `+1` Proceed, `NULL` Halt), algebraic negation, and physics-driven control flow.
* **[02. Relational Connectives & Operators](02_logic_and_algebra/02_relational_connectives_and_operators.md):** The connective algebra (`!` But, `::` And, `:` In relation to, `|` Or, `--` Not) and state relations (`=` Is, `!=` Is not, `≈` Liken to).

### [Volume 3: Architecture & Execution](03_architecture/)

* **[01. The 3D Coordinate System of Computation](03_architecture/01_coordinate_system_of_computation.md):** The three orthogonal axes: Being (Ontology), Dynamics (Causality: `-->` and `<--`), and Scale (Layering: `^--` builds to, `v--` breaks down from).
* **[02. The Unified Execution Pipeline](03_architecture/02_execution_pipeline.md):** How a single AST and IR seamlessly feeds AOT Native Compilation (LLVM), Real-time Interpretation (REPL/VM), and Cross-Platform Transpilation (C/WASM).
* **[03. The Universal 4-Block Architecture & Rust Foundation](03_architecture/03_four_block_architecture.md):** The deterministic 4-block file lifecycle (`Metadata` $\to$ `Opening` $\to$ `Body` $\to$ `Closing`), the `OmniLinter`, and the technical rationale for using Rust as the core foundation.

---

## Why Rust as the Core Foundation?

The OmniCode core is built in **Rust** to ensure the foundational layer never buckles:

1. **Memory Safety without GC:** Zero-cost pointers and deterministic resource deallocation with no garbage collection pauses.
2. **Algebraic Data Types & Exhaustive Matching:** Enforces compile-time completeness across all boundary types, quad-states, and directional vectors.
3. **Zero Runtime Buckling:** Immune to memory corruption, dangling references, and concurrency races.
4. **Native Toolchain Interoperability:** First-class C ABI compatibility and direct integration with LLVM compiler infrastructure.

---

## Canonical 4-Block Example: `examples/canonical_notation.omni`

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
    on_rollback: [ <-- unwind_state ],
    commit_status: [ +1 ]
}
```

---

## Quick Reference Symbol Matrix

| Symbol | Name | Ontological Category | Semantic Function |
| :---: | :--- | :--- | :--- |
| `[ ... ]` | Denotation | Boundary | Defined as (Mutable identity, value, or layout) |
| `< ... >` | Invariant | Boundary | Defined as (Immutable axiom or constant) |
| `{ ... }` | Connotation | Boundary | In context of (Environment, scope, conditions) |
| `{ ... ]` | Top-Down | Asymmetric Boundary | Context defines (Environment forces definition) |
| `[ ... }` | Bottom-Up | Asymmetric Boundary | Defined by context (Entity adapts to environment) |
| `( ... )` | Aside | Boundary | Aside to (Intent, semantic constraint, rationale) |
| `=` | Identity | Relation of Being | IS / TO BE |
| `!=` | Distinction | Relation of Being | IS NOT / TO BE NOT |
| `≈` / `~=` | Homomorphism | Relation of Being | TO BE LIKE / Liken to |
| `-->` | Act Upon | Temporal Vector | Forward causation, execution, emission |
| `<--` | Act Again | Temporal Vector | Backward retroaction, feedback, rollback |
| `^--` | Builds To | Scale Vector | Upward abstraction, synthesis, emergence |
| `v--` | Breaks Down From | Scale Vector | Downward decomposition, reduction, compilation |
| `!` | But | Connective | Adversative caveat, conditional constraint |
| `::` | And | Connective | Cumulative conjunction, co-presence |
| `:` | In Relation To | Connective | Relational role, projection, typing |
| `\|` | Or | Connective | Disjunction, alternative choice |
| `--` | Not | Inversion | Symmetrical ternary inversion |
| `+1` | Yes | Quad-State Truth | Proceed (commits forward) |
| `0` | Ok | Quad-State Truth | Await / Equilibrium (standby / pass-through) |
| `-1` | No | Quad-State Truth | Rollback (reverts state) |
| `NULL` | Void | Quad-State Truth | Halt / Trap (bottom type $\bot$) |

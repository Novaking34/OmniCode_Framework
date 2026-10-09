# Volume 1: Foundations

## Document 02: Boundary Types & First-Class Asides

---

### 1. The Morphological Boundary System

In OmniCode, delimiters are not passive punctuation; they are **morphological operators** that govern the boundary physics between an entity and its surrounding universe.

There are six canonical boundary types, ranked in strict ontological and linguistic hierarchy:

```text
               Rank 1: < ... >      Invariant (Immutable Axiom / Law)
               Rank 2: [ ... ]      Denotation (Mutable Definition / Essence)
               Rank 3: { ... }      Connotation (Context / Ambient Universe)
               Rank 4: [ ... }      Bottom-Up (Inside-Out: Entity Projects into Context)
               Rank 5: { ... ]      Top-Down (Outside-In: Context Conditions Entity)
               Rank 6: ( ... )      Aside (Intent, Teleology, Machine Annotation)
```

---

### 2. Detailed Semantics of Each Boundary

#### A. Symmetric Boundaries

* **`[ ... ]` (Denotation):**
  Defines an entity whose state or value can vary over time. Represents mutable structures, variables, functions, and state machines.
* **`< ... >` (Invariant):**
  Defines an entity whose state is immutable across all possible execution contexts. Represents mathematical constants, hardware ROM, zero-cost immediate literals, and axioms.
* **`{ ... }` (Connotation):**
  Defines the ambient context or environment. Can contain required capabilities, thread bindings, memory space qualifiers, or lifetime scopes.

#### B. Asymmetric Boundaries (Directional Causality)

* **`{ ... ]` (Context Defines):**
  **Outside-In Causality.** The environment dictates what the entity inside is.
  * *Example:* A memory page table dictating a virtual memory mapping, or dependency injection where the runtime environment provides the concrete implementation.
* **`[ ... }` (Defined by Context):**
  **Inside-Out Causality.** The entity's definition emerges from or adapts to the environment it is placed into.
  * *Example:* Ad-hoc polymorphism, polymorphic serialization formats, or registers adapting to caller/callee ABIs.

#### C. `( ... )` The First-Class Aside

Traditional languages discard comments (`//`, `/* */`) during lexical analysis. This causes a severe **semantic gap**:

1. Human and AI intent is lost to the compiler.
2. Code and documentation drift out of synchronization.
3. The compiler cannot verify whether the implementation satisfies the intended purpose.

In OmniCode, `( ... )` is an **Aside**:

* It is preserved as a permanent node in the Abstract Syntax Tree (AST).
* It provides semantic tags, invariants, and rationale.
* It enables native AI code generation by creating an explicit container for reasoning traces that the compiler and verifier can inspect.

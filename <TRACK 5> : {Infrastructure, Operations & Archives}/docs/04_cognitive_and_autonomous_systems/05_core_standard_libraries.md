# Volume 4: Cognitive & Autonomous Systems

## Document 05: Core Standard Library Architecture

> **Axiomatic Modules Grounded in Balanced Ternary Logic, Safe Arithmetic, and Directional Vectors.**

The OmniCode Standard Library lives under [`lib/core/`](file:///home/professorseanex/OmniCode/lib/core) and provides the core algebraic foundation for the language. Every standard library file is written in canonical 4-Block OmniCode.

---

### 1. Standard Library Modules & Topology

| Module Name | Source File | Description |
| :--- | :--- | :--- |
| `Core.Arithmetic` | [`lib/core/arithmetic.omni`](file:///home/professorseanex/OmniCode/lib/core/arithmetic.omni) | Balanced ternary arithmetic, addition, subtraction, multiplication, and non-zero division. |
| `Core.Logic` | [`lib/core/logic.omni`](file:///home/professorseanex/OmniCode/lib/core/logic.omni) | Ternary Kleene and Lukasiewicz logic gates (AND, OR, NOT, XOR, and Consensual Agreement). |
| `Core.Vectors` | [`lib/core/vectors.omni`](file:///home/professorseanex/OmniCode/lib/core/vectors.omni) | Directional coordinate vectors across Being, Causality, and Scale axes with caveat checks. |

---

### 2. Core Mathematical & Logical Specifications

#### `Core.Arithmetic`

Guarantees quad-state mathematical correctness:

* **Addition:** $a + b \implies$ produces sum when both operands are valid; returns `NULL` on overflow or trap.
* **Safe Division:** Division by zero returns `NULL:HALT` without raising an exception:

  ```omni
  divisor != 0 ! [ divisor = 0 --> NULL ]
  ```

#### `Core.Logic`

Implements full balanced ternary logic gates:

* **Consensual AND (`::`):**
  $$\text{AND}(a, b) = \min(a, b)$$
* **Ternary OR (`|`):**
  $$\text{OR}(a, b) = \max(a, b)$$
* **Ternary NOT (`--`):**
  $$\text{NOT}(x) = -x$$
* **Consensual Agreement:** Returns `+1` if both agree, `-1` if in contradiction, and `0` if either is awaiting consensus.

#### `Core.Vectors`

Implements 3D directional physics:

* **Causality (`-->`, `<--`):** Drives causal transformations forward and retroactively verifies feedback.
* **Scale (`^--`, `v--`):** Manages abstraction transitions between primitive memory buffers and high-level types.
* **Caveat Pipeline (`!`):** Evaluates guard conditions prior to committing state transitions:

  ```omni
  a --> b ! [ precondition = +1 ]
  ```

---

### 3. Invariant Proofs & Resolution Guarantees

1. **Self-Contained Execution:** All standard library functions are verified in `tests/test_failpoints.rs` (`test_lib_core_vectors`, `test_svo_arithmetic_keywords_and_infer_operator`).
2. **Deterministic Resolution:** The compiler automatically resolves `import: [ Core.Arithmetic :: Core.Logic :: Core.Vectors ]` directly to `lib/core/`.

# Transducer Architecture: Document 02

## Dynamic Reel Synthesis & AST Lowering

---

### 1. From Static Templates to Composable AST Building

Earlier prototypes in `templates.rs` contained hundreds of lines of static string
interpolation for individual, pre-scripted prompts. This created an unmaintainable
bottleneck.

In the elevated architecture, `templates.rs` acts as a dynamic AST reel builder
grounded in the 4-Block lifecycle:

```text
+-------------------------------------------------------------------------+
| BLOCK 1: METADATA BLOCK                                                 |
| - Module name derived from Agent / Persona / Topic                      |
| - Inscription of Who, What, Where, When, Why, How                       |
| - Modal Domain citation (1 of 15 Sovereign Law-Spheres)                 |
| - Scripture Landmark citation (Canonical KJV / WEB Key)                 |
+-------------------------------------------------------------------------+
| BLOCK 2: OPENING BLOCK                                                  |
| - Lexicon declarations                                                  |
| - Preconditions aligned with QuadState (+1: Proceed, 0: Await, -1)      |
| - Bounded vessel declarations with inferred entity / container types    |
+-------------------------------------------------------------------------+
| BLOCK 3: BODY BLOCK                                                     |
| - Directional rolls executing algebraic vector trajectories             |
| - SVO expressions (e.g. Subject --> Object v-- Container ! [ Caveat ])  |
+-------------------------------------------------------------------------+
| BLOCK 4: CLOSING BLOCK                                                  |
| - Invariant verification assertions                                     |
| - Transactional commit status matching target QuadState                 |
+-------------------------------------------------------------------------+
```

### 2. Eliminating Floating-Point Token Approximations

Unlike continuous token probability vectors, synthesized reels are deterministic
programs. They execute through the OmniCode VM (`Lexer` $\to$ `Parser` $\to$
`Evaluator`), which ensures:

* Zero IEEE 754 precision drift.
* Strict balanced ternary reversible state transitions.
* Structural boundary invariants checked at runtime.

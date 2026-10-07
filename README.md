# OmniCode

> **A Dual-Readable Ontological Programming Language Engineered for Unambiguous Human Comprehension and Machine Determinism.**

OmniCode is designed from first principles to bridge the entire spectrum of computing—operating with bare-metal low-level precision (pointers, registers, memory models) while expressing high-level mathematical abstractions, declarative logic, and semantic intent natively.

---

## Key Pillars

1. **Formal Ontology & Set Theory:** Clean separation of an entity's intrinsic essence (Denotation) from its environmental context (Connotation).
2. **Balanced Ternary Quad-State Engine:** Execution physics governed by $\mathbb{T}_4 = \{ -1, 0, +1, \text{NULL} \}$ (`Rollback`, `Await`, `Proceed`, `Halt`), supporting native transactional state machines without exceptions.
3. **Morphological Boundary Physics:** Brackets are active operators that encode causality direction: `[...]`, `<...>`, `{...}`, `{...]`, `[...}`, `(...)`.
4. **First-Class Asides `(...)`:** Comments are preserved in the AST as machine-readable intent and AI reasoning traces, closing the semantic gap.
5. **Universal 4-Block Architecture:** Every source file adheres to a strict 4-block lifecycle: `Metadata` $\to$ `Opening` $\to$ `Body` $\to$ `Closing`.
6. **Rust Core:** Implemented in Rust to guarantee that the compiler foundation never buckles under memory or concurrency strain.

---

## Why Rust as the Core Foundation?

The OmniCode compiler, runtime, and diagnostic engine are written in **Rust** for four decisive reasons:

1. **Zero Runtime Buckling (Memory Safety without Garbage Collection):**
   A language designed to work at the low level cannot rely on a stop-the-world garbage collector (like Java or Go). Rust guarantees compile-time memory safety, preventing segmentation faults, data races, and dangling references while compiling down to raw native machine code.
2. **Algebraic Data Types & Exhaustive Pattern Matching:**
   Compilers are fundamentally state transformers. Rust's `enum` variants with associated data (`QuadState`, `BoundaryType`, `ASTNode`) and exhaustive `match` expressions ensure that every single boundary crossing, operator, and quad-state transition is handled with zero unhandled edge cases.
3. **Zero-Cost Abstractions & Native Execution Speed:**
   The OmniCode AST, recursive-descent parser, and transactional snapshot system execute at bare-metal speed with no interpreter baggage.
4. **LLVM & C ABI Interoperability:**
   Rust has first-class bindings to LLVM (`inkwell`) and seamless C FFI (`libc`, `extern "C"`), making it the ideal host to drive OmniCode's AOT native compilation and C transpilation pipelines.

---

## The Universal 4-Block File Architecture

Every `.omni` and engine source file is partitioned into four sequential blocks:

```omni
# ==============================================================================
# BLOCK 1: METADATA BLOCK
# ( Module Identity, Version, Invariant Types, and Intent Aside )
# ==============================================================================
Module < Example > ( Kernel logic intent ) {
    version: < "1.0.0" >,
    system: < OmniCode.Core >
}

# ==============================================================================
# BLOCK 2: OPENING BLOCK
# ( Context Setup, Preconditions, Scoped Universes, and State Initialization )
# ==============================================================================
Opening {
    requires: [ MemoryAligned.64 ],
    preconditions: [ +1 ]
}
state [ +1 ] ( Initial state )

# ==============================================================================
# BLOCK 3: BODY BLOCK
# ( Core Transformations, Causality Vectors, Logic, and Caveat Pipelines )
# ==============================================================================
Body [
    state --> next_state [ +1 ]
    next_state ! [ state = +1 ] ( Caveat verification )
]

# ==============================================================================
# BLOCK 4: CLOSING BLOCK
# ( Postconditions, Invariant Integrity Proofs, Teardown, and Commit Status )
# ==============================================================================
Closing {
    verify: [ next_state = +1 ],
    commit_status: [ +1 ] ( Commit verified state )
}
```

---

## The 3D Coordinate System of Computation

Every operation in OmniCode is positioned across three orthogonal axes:

| Axis | Dimension | Operators | Semantic Function |
| :--- | :--- | :---: | :--- |
| **Y-Axis** | **Being (Identity)** | `=`, `!=`, `~=` | State of being: Is, Is Not, To Be Like (Homomorphism). |
| **X-Axis** | **Dynamics (Causality)** | `-->`, `<--` | Temporal flow: Act Upon (forward), Act Again (retroaction/rollback). |
| **Z-Axis** | **Scale (Hierarchy)** | `^--`, `v--` | Abstraction scale: Builds To (layer up), Breaks Down From (layer down). |
| **Connective** | **Algebra** | `!`, `::`, `:`, `\|`, `--` | Caveat (But), Conjunction (And), Role (In relation to), Or, Not. |

---

## Getting Started

### Prerequisites

* Rust 1.85+ (`cargo`, `rustc`)

### Interactive Terminal Environment (REPL)

Launch the interactive terminal shell where you can directly evaluate OmniCode expressions, inspect ASTs, and observe balanced ternary rollbacks live:

```bash
cargo run
# or
cargo run -- repl
```

Inside the REPL:

```text
omni[+1]> PI < 3.14159 >
=> 3.14159

omni[+1]> status [ +1 = +1 ]
=> +1:PROCEED

omni[+1]> [ +1 ] ! [ -1 ]
=> -1:ROLLBACK
  [FAILPOINT: FP_ROLLBACK_TRIGGERED] Rollback (-1) triggered in context: TernaryLiteral
  [FAILPOINT: FP_CAVEAT_VETO] Action vetoed by ! BUT caveat; state transaction rolled back

omni[-1]> :env
--- Active Symbols & Bindings (2) ---
  status = +1:PROCEED
  PI = 3.14159

omni[-1]> :load examples/canonical_notation.omni
✓ Loaded and evaluated 'examples/canonical_notation.omni': => -1:ROLLBACK

omni[-1]> :failpoints
--- Trapped Failpoint Incidents (3) ---
  * [DEBUGGER] [FP_ROLLBACK_TRIGGERED] Rollback (-1) triggered in context: TernaryLiteral
  * [EVALUATOR] [FP_CAVEAT_VETO] Action vetoed by ! BUT caveat; state transaction rolled back
```

### CLI Command Options

```bash
# Run an OmniCode source file
cargo run -- run examples/canonical_notation.omni

# Validate 4-Block structural architecture
cargo run -- lint examples/canonical_notation.omni

# Scaffold a compliant 4-Block template
cargo run -- scaffold MyService

# Run the built-in ontological verification suite
cargo run -- demo

# Run the 10-test failpoint verification test suite
cargo test
```

---

## Project Structure

```text
OmniCode/
├── Cargo.toml                     # Rust package configuration & rustyline dependency
├── docs/                          # Comprehensive 3-volume documentation series
│   ├── README.md                  # Documentation index & symbol matrix
│   ├── 01_foundations/            # Ontology, Set Theory, Boundaries, Asides
│   ├── 02_logic_and_algebra/      # Balanced Ternary, Connective Algebra
│   └── 03_architecture/           # 3D Coordinates, Execution Pipeline
├── editors/
│   └── vscode/                    # VS Code / IDE extension with syntax grammar & snippets
├── examples/
│   └── canonical_notation.omni    # Canonical 4-block showcase using A, B, a, b
├── src/
│   ├── ast/                       # Strongly typed AST preserving boundaries & asides
│   ├── cli/                       # Interactive REPL terminal, history, and CLI dispatch
│   ├── core/                      # Balanced ternary, boundaries, and operators
│   ├── diagnostics/               # DiagnosticLogger, OmniDebugger, and OmniLinter
│   ├── parser/                    # Lexer and recursive-descent parser
│   └── runtime/                   # Transactional environment and quad-state evaluator
└── tests/
    └── test_failpoints.rs         # 10 comprehensive failpoint verification tests
```

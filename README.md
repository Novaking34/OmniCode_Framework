# ==============================================================================
# BLOCK 1: METADATA BLOCK
# ( Project Identity, Philosophy, Theoretical Pillars & Technical Rationale )
# ==============================================================================

# OmniCode

> **A Dual-Readable Ontological Programming Language Engineered for Unambiguous Human Comprehension and Machine Determinism.**

OmniCode is designed from first principles to bridge the entire spectrum of computing—operating with bare-metal low-level precision (pointers, CPU registers, raw memory models) while natively expressing high-level mathematical abstractions, declarative logic, balanced ternary state machines, and machine-readable cognitive intent.

---

### Manifest Metadata

| Attribute | Specification |
| :--- | :--- |
| **Engine Version** | `0.1.0` |
| **Core Runtime** | Rust Engine (`edition = "2024"`) |
| **Execution Truth Space** | Balanced Ternary Quad-State ($\mathbb{T}_4 = \{ -1, 0, +1, \text{NULL} \}$) |
| **Structural Paradigm** | Deterministic 4-Block Architecture (`Metadata` $\to$ `Opening` $\to$ `Body` $\to$ `Closing`) |
| **Morphological Physics** | 6 Boundary Types (`[]`, `<>`, `{}`, `{]`, `[}`, `()`) |
| **Cognitive Framework** | Neural-Symbolic AI Cortex, Transducer, Episodic Memory Scrolls (`.scroll` & `.log`) |
| **Diagnostics & Healing** | Restorator Autonomous Self-Healing, OmniDebugger (Transactional Rollbacks) |
| **Author / Lead** | ProfessorSeanEX |
| **License** | MIT OR Apache-2.0 |

---

### Core Pillars

1. **Formal Ontology & Set Theory:** Clean separation of an entity's intrinsic essence (Denotation) from its environmental context (Connotation).
2. **Balanced Ternary Quad-State Engine:** Execution physics governed by $\mathbb{T}_4 = \{ -1, 0, +1, \text{NULL} \}$ (`Rollback`, `Await`, `Proceed`, `Halt`), delivering native transactional state machines without runtime exceptions or unchecked panics.
3. **Morphological Boundary Physics:** Brackets are active semantic operators encoding causality direction, containment hierarchy, and physical mutability: `[...]`, `<...>`, `{...}`, `{...]`, `[...}`, `(...)`.
4. **First-Class Asides `(...)`:** Comments are preserved in the AST as machine-readable intent, formal invariants, and AI cognitive traces, eliminating the human-machine semantic gap.
5. **Universal 4-Block Architecture:** Every source file, configuration, and episodic scroll adheres strictly to a 4-block lifecycle: `Metadata` $\to$ `Opening` $\to$ `Body` $\to$ `Closing`.
6. **Cognitive Loop & Episodic Learning:** Seamless bidirectional translation between natural language discourse and deterministic 4-Block Turing reels, archived into an episodic memory bank (`.scroll` and `.log`).
7. **Autonomous Self-Healing (Restorator):** Built-in diagnostic immune system that repairs boundary mismatches, unclosed delimiters, missing standard imports, and structural anomalies on the fly.
8. **Rust Core Foundation:** Zero runtime buckling, memory safety without garbage collection, and seamless LLVM/C ABI interoperability.

---

### Why Rust as the Core Foundation?

The OmniCode compiler, interpreter, and diagnostic engine are written in **Rust** for four decisive reasons:

1. **Zero Runtime Buckling (Memory Safety without Garbage Collection):**
   A language operating at the bare metal cannot tolerate stop-the-world garbage collection pauses or undefined behavior. Rust guarantees compile-time memory safety, preventing segmentation faults, data races, and dangling references.
2. **Algebraic Data Types & Exhaustive Pattern Matching:**
   Compilers are state transformers. Rust's `enum` types with associated payloads (`QuadState`, `BoundaryType`, `ASTNode`, `DirectionalVector`) and exhaustive pattern matching ensure that every boundary crossing and state transition is strictly accounted for.
3. **Zero-Cost Abstractions & Native Execution Speed:**
   The OmniCode AST, recursive-descent parser, transactional snapshot debugger, and evaluation pipeline run at bare-metal execution speed with no interpreter overhead.
4. **LLVM & C ABI Interoperability:**
   Rust provides first-class bindings to LLVM and frictionless C FFI (`extern "C"`), making it the optimal host to drive OmniCode's AOT native compilation and cross-platform transpilation.

---


# ==============================================================================
# BLOCK 2: OPENING BLOCK
# ( Prerequisites, Toolchain Setup, Container Emulation & Environment Invariants )
# ==============================================================================

## Getting Started & Environment Setup

### System Prerequisites

* **Rust Compiler:** Version `1.85+` (`rustc`, `cargo`)
* **Terminal Shell:** POSIX Bash or equivalent Linux/macOS shell
* **Hardware Emulation & Cross-Toolchains (Optional for Bare-Metal):**
  * `clang`, `lld`, `nasm`, `xorriso`, `qemu-system-x86_64`

### Toolchain Verification

OmniCode includes an automated diagnostic harness to verify your local build and bare-metal compilation toolchains:

```bash
./scripts/check_toolchain.sh
```

### Bare-Metal & Containerized Toolchain (Docker & Compose)

For fully isolated cross-compilation and hardware emulation across `x86_64`, `aarch64`, and `riscv64gc` bare-metal targets:

```bash
# Build and launch the containerized toolchain
docker compose up -d

# Enter the container environment
docker compose exec omnicode-toolchain bash
```

Inside the container, all LLVM extensions, cross-target sysroots, NASM assemblers, and QEMU hardware emulation stubs (with GDB stub on port `1234` and VNC on port `5900`) are pre-configured.

### Visual Studio Code IDE Extension

OmniCode provides an official VS Code extension (`omnicode-language-0.1.2`) offering TextMate syntax highlighting for `.omni` and `.scroll` files, bracket-pair colorization, and canonical 4-block snippets.

To install the extension:
```bash
code --install-extension editors/vscode/omnicode-language-0.1.2.vsix
```

### Interactive Terminal Environment (REPL)

Launch the interactive terminal shell for sub-millisecond evaluation of balanced ternary expressions, live environment inspection, and transactional rollback experiments:

```bash
cargo run
# or explicitly:
cargo run -- repl
```

#### REPL Live Commands

| Command | Action |
| :--- | :--- |
| `:env` | Display all currently bound identifiers, types, and values |
| `:ast <expr>` | Tokenize and print the Abstract Syntax Tree for an expression |
| `:failpoints` | Inspect the session's diagnostic failpoint log |
| `:load <file>` | Parse and evaluate an external `.omni` source file or `.scroll` |
| `:reset` | Reset the global environment to pristine initial state |
| `:help` | Show interactive REPL command help |
| `:exit`, `:quit` | Exit the REPL session |

---


# ==============================================================================
# BLOCK 3: BODY BLOCK
# ( Syntax, Morphological Boundaries, Quad-State, AI Cortex & Standard Library )
# ==============================================================================

## Language Syntax & Core Concepts

### 1. The Universal 4-Block Architecture

Every `.omni` program, standard library file, and episodic scroll follows the 4-block lifecycle:

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
    import: [ Core.Arithmetic :: Core.Logic :: Core.Vectors ],
    preconditions: [ +1 ]
}
state [ +1 ] ( Initial state )

# ==============================================================================
# BLOCK 3: BODY BLOCK
# ( Core Transformations, Causality Vectors, Logic, and Caveat Pipelines )
# ==============================================================================
Body [
    state --> next_state [ +1 ]
    next_state ! [ state = +1 ] ( Caveat verification: proceed only if caveat holds )
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

### 2. Morphological Boundary Delimiters

Brackets in OmniCode are active operators defining mutability, hierarchy ranking, and directional causality:

| Delimiter | Boundary Type | Semantic Role | Behavior & Physics |
| :---: | :--- | :--- | :--- |
| `[...]` | **Rigid / Fixed** | Denotation / Concrete | Fixed physical memory, immutable structure, discrete value. |
| `<...>` | **Dynamic / Fluid** | Process / Type Invariant | Mutable state, transformation vector, category identifier. |
| `{...}` | **Universal / Omni** | Scoped Universe / Holism | Enclosing set domain, parent scope, environmental context. |
| `{...]` | **Accumulator / Absorber** | Fluid In $\to$ Fixed Out | Consumes external fluid changes and outputs a fixed datum. |
| `[...}` | **Generator / Source** | Fixed In $\to$ Fluid Out | Emanates from a fixed constant and radiates mutable streams. |
| `(...)` | **First-Class Aside** | Intent / Cognitive Trace | Non-interfering metadata, machine-readable reasoning, and proofs. |

---

### 3. Balanced Ternary Quad-State Physics ($\mathbb{T}_4$)

OmniCode eliminates exceptions by treating system states as continuous physical trajectories across four discrete values:

$$\mathbb{T}_4 = \{ -1, 0, +1, \text{NULL} \}$$

* **`+1:PROCEED` (`Proceed`):** Affirmative forward execution, condition satisfied, transaction committed.
* **`0:AWAIT` (`Await`):** Indeterminate, suspended, or awaiting external input/consensus.
* **`-1:ROLLBACK` (`Rollback`):** Contra/veto state. Automatically rolls back the environment to the previous stable snapshot.
* **`NULL:HALT` (`Halt`):** Terminal void. Absorbs subsequent transformations and halts execution safely.

#### Ternary Truth Tables

| Operation | Input A | Input B | Output | Semantic Result |
| :--- | :---: | :---: | :---: | :--- |
| **Negation (`--`)** | `+1` | — | `-1` | Logical inversion / Rollback trigger |
| **Negation (`--`)** | `0` | — | `0` | Neutral balance preserved |
| **Negation (`--`)** | `-1` | — | `+1` | Contra-inversion to Proceed |
| **Consensus Conjunction (`::`)** | `+1` | `+1` | `+1` | Full agreement |
| **Consensus Conjunction (`::`)** | `+1` | `-1` | `-1` | Single veto triggers Rollback |
| **Consensus Conjunction (`::`)** | `+1` | `0` | `0` | Indeterminate awaiting resolution |

---

### 4. The 3D Coordinate System of Computation

Every operation is positioned across three orthogonal coordinate axes:

```
          Z (Scale / Hierarchy)
          ^  ^-- (Builds To / Generalize)
          |
          |
          +--------> X (Dynamics / Causality: --> Act Upon, <-- Retroact)
         /
        /
       v
      Y (Being / Identity: = Is, != Is Not, ~= Homomorphism)
```

* **Y-Axis (Being / Identity):**
  * `a = b` : Identity assertion (True if identical).
  * `a != b` : Distinctness assertion.
  * `a ~= b` : Homomorphism (Structural or type similarity).
* **X-Axis (Dynamics / Causality):**
  * `a --> b` : Forward causal drive ('a' acts upon 'b').
  * `b <-- a` : Retroactive feedback / reverse causality.
* **Z-Axis (Scale / Hierarchy):**
  * `a ^-- A` : Micro-to-Macro abstraction (Primitive 'a' builds up to Concept 'A').
  * `A v-- a` : Macro-to-Micro decomposition (Concept 'A' breaks down into Primitive 'a').
* **Connective Algebra:**
  * `!` : Caveat (**BUT** condition — guards state transitions; if false, triggers rollback).
  * `::` : Conjunction (**AND** — namespace scoping and consensus linking).
  * `:` : Role / Aspect (**IN RELATION TO** — type qualification).
  * `|` : Disjunction (**OR**).
  * `--` : Negation (**NOT**).

---

### 5. Autonomous Self-Healing Engine (`Restorator`)

The OmniCode `Restorator` operates as an autonomous immune system:

```bash
# Auto-heal syntax, boundary mismatches, and 4-block anomalies
cargo run -- restore path/to/file.omni --write
```

Capabilities:
1. **Boundary Balancing & Healing:** Detects unclosed delimiters (`[`, `<`, `{`) and closes them based on morphology rank.
2. **Structural Scaffolding:** Automatically wraps loose statements into canonical 4-Block architecture (`Metadata`, `Opening`, `Body`, `Closing`).
3. **Import Injection:** Injects missing standard library imports (`Core.Arithmetic`, `Core.Logic`, `Core.Vectors`) into the Opening block.

---

### 6. AI Cortex & Neural-Symbolic Transduction

OmniCode features native cognitive processing via the `AICortex` and `SemanticTransducer`:

```bash
cargo run -- ai "Add 5 units to inventory A which contains 12 units"
```

The AI pipeline:
1. **Natural Language Transduction:** Converts freeform human discourse into canonical 4-Block OmniCode.
2. **Autonomous Restoration:** Heals inferred boundaries and structures.
3. **Deterministic Evaluation:** Runs the generated program through the `Evaluator` and `OmniDebugger`.
4. **Episodic Inscription:** Generates dual logs:
   * **`.scroll`:** Native executable 4-Block Turing reel recording the session's episodic trace.
   * **`.log`:** Microsecond telemetry tracking channel logs, execution duration, and failpoint telemetry.

#### Inspecting Episodic Memory

```bash
cargo run -- scrolls
```

Outputs learning statistics digested by `AILearner`:
* Total inscribed episodes.
* QuadState distribution (`Proceed`, `Await`, `Rollback`, `Halt`).
* Learned repairs cataloged by the Restorator.

---

### 7. Standard Library Architecture (`lib/core/`)

OmniCode includes a native standard library in [`lib/core/`](file:///home/professorseanex/OmniCode/lib/core):

* **[`arithmetic.omni`](file:///home/professorseanex/OmniCode/lib/core/arithmetic.omni):** Quad-state safe addition, subtraction, multiplication, and non-zero division.
* **[`logic.omni`](file:///home/professorseanex/OmniCode/lib/core/logic.omni):** Ternary Kleene/Lukasiewicz logic gates (AND, OR, NOT, XOR, and Consensual Agreement).
* **[`vectors.omni`](file:///home/professorseanex/OmniCode/lib/core/vectors.omni):** Causality, Being, and Scale directional operations with caveat guard checks.

---


# ==============================================================================
# BLOCK 4: CLOSING BLOCK
# ( Verification Suite, CLI Reference, Directory Layout & Project Invariants )
# ==============================================================================

## CLI Command Reference

```text
USAGE:
  omnicode                    Start the interactive terminal shell (REPL)
  omnicode repl               Start the interactive terminal shell (REPL)
  omnicode run <file>         Execute an OmniCode source (.omni) or scroll (.scroll)
  omnicode lint <file>        Validate 4-Block architecture compliance (.omni / .scroll)
  omnicode restore <file>     Auto-heal syntax, boundary, and block anomalies (--write)
  omnicode ai "<prompt>"      Transduce prompt with AI Cortex, self-heal, execute & inscribe logs
  omnicode scrolls            Inspect episodic memory bank and learning statistics
  omnicode scaffold <Name>    Generate canonical 4-Block template [--type omni|scroll]
  omnicode filetypes          Display all formal OmniCode file types and MIME definitions
  omnicode demo               Run the built-in ontological verification suite
  omnicode --help, -h         Show help information
  omnicode --version, -v      Display engine version
```

---

## Verification & Test Harness

OmniCode maintains a 100% passing test suite with **24 unit, failpoint, and AI integration tests**:

```bash
cargo test
```

### Test Suite Breakdown

* **5 Core Unit Tests** ([`src/lib.rs`](file:///home/professorseanex/OmniCode/src/lib.rs)):
  * Boundary ranking hierarchy validation.
  * File type detection and canonical scaffold generation.
  * Directional vector shorthand and canonical parsing.
  * Quad-state micro and macro transitions.
* **17 Failpoint & Integration Tests** ([`tests/test_failpoints.rs`](file:///home/professorseanex/OmniCode/tests/test_failpoints.rs)):
  * `test_asymmetric_causality_boundaries`
  * `test_balanced_ternary_algebraic_symmetry`
  * `test_failpoint_boundary_mismatch_illegal_crossing`
  * `test_failpoint_null_trap_absorption`
  * `test_failpoint_immutable_mutation_attempt`
  * `test_failpoint_cyclic_retroaction_threshold`
  * `test_failpoint_caveat_veto_and_rollback`
  * `test_cardinal_directions_and_micro_macro_terms`
  * `test_failpoint_boundary_leak_unclosed_delimiter`
  * `test_four_block_structural_linter_validation`
  * `test_repl_eval_and_environment_persistence`
  * `test_svo_arithmetic_keywords_and_infer_operator`
  * `test_lib_core_vectors`
  * `test_canonical_notation_file`
  * `test_ai_semantic_transducer_round_trip`
  * `test_module_import_and_library_resolution`
  * `test_restorator_boundary_and_structural_auto_heal`
* **2 AI Cognitive Tests** ([`tests/test_ai_model.rs`](file:///home/professorseanex/OmniCode/tests/test_ai_model.rs)):
  * `test_ai_cortex_end_to_end_pipeline` (Prompt $\to$ Transduction $\to$ Healing $\to$ Execution $\to$ Dual Inscription)
  * `test_ai_learner_digests_dual_logs` (Episodic archive ingestion and statistical learning)

---

## Project Repository Layout

```text
OmniCode/
├── .gitignore                         # Canonical 4-block Git exclusions
├── Cargo.toml                         # Canonical 4-block Rust package manifest (Edition 2024)
├── Cargo.lock                         # Cargo dependency resolution lockfile
├── Dockerfile                         # Multi-arch bare-metal container environment
├── docker-compose.yml                 # Toolchain container orchestration & QEMU debugging
├── README.md                          # Universal 4-block root architectural documentation
├── docs/                              # Comprehensive documentation series
│   ├── README.md                      # Documentation index & symbol matrix
│   ├── 01_foundations/                # Ontology, Set Theory, Boundaries, Asides
│   ├── 02_logic_and_algebra/          # Balanced Ternary, Connective Algebra
│   ├── 03_architecture/               # 3D Coordinates, Execution Pipeline, 4-Block Lifecycle
│   └── 04_cognitive_and_autonomous/   # AI Cortex, Episodic Scrolls, Restorator, Toolchains
├── editors/
│   └── vscode/                        # VS Code Extension (v0.1.2) with TextMate & snippets
├── examples/                          # Canonical & educational demonstrations
│   ├── canonical_notation.omni        # Theoretical foundations showcase
│   ├── 01_boundaries_and_asides.omni  # Boundary morphology & first-class asides
│   ├── 02_logic_and_quad_state.omni   # Balanced ternary and caveat pipelines
│   ├── 03_cardinal_directions.omni    # 3D coordinate vector operations
│   └── 04_turing_track_tape.omni      # State-machine tape mechanics
├── lib/
│   └── core/                          # Standard library modules
│       ├── arithmetic.omni            # Quad-state arithmetic
│       ├── logic.omni                 # Ternary logic gates
│       └── vectors.omni               # Directional coordinate vectors
├── scripts/
│   └── check_toolchain.sh             # Canonical 4-block toolchain verification script
├── scrolls/                           # Episodic memory archive (.scroll & .log)
├── src/                               # Rust Core Engine
│   ├── main.rs                        # CLI entrypoint dispatcher
│   ├── lib.rs                         # Core library exports & execute() helper
│   ├── ast/nodes.rs                   # AST nodes with Asides and Boundary expressions
│   ├── cli/repl.rs                    # Interactive terminal environment (REPL)
│   ├── core/                          # Boundaries, FileType, Operators, Ternary
│   ├── diagnostics/                   # Debugger, Linter, Logger, Restorator
│   ├── parser/                        # Lexer and recursive-descent Parser
│   ├── runtime/                       # Environment and Evaluator
│   └── ai/                            # AICortex, SemanticTransducer, ScrollLogger, AILearner
└── tests/                             # Integration test suites
    ├── test_failpoints.rs             # 17 failpoint and language physics tests
    └── test_ai_model.rs               # 2 AI pipeline and episodic learner tests
```

---

## Formal Invariant Seal

OmniCode source definitions, language physics, and diagnostic boundaries are verified and sealed under the Universal 4-Block Architecture. All transformations are deterministic, bounded, and self-verifying.

# OmniCode

> **A Dual-Readable Ontological Programming Language Engineered for Unambiguous Human Comprehension and Machine Determinism.**

OmniCode is designed from first principles to bridge the entire spectrum of computing—operating with bare-metal low-level precision (pointers, CPU registers, raw memory models) while natively expressing high-level mathematical abstractions, declarative logic, balanced ternary state machines, and machine-readable cognitive intent.

---

## Manifest Metadata

| Attribute | Specification |
| :--- | :--- |
| **Engine Version** | `0.1.0` |
| **Core Runtime** | [Rust Engine (`edition = "2024"`)](file:///home/professorseanex/OmniCode/src/lib.rs) |
| **Execution Truth Space** | Balanced Ternary Quad-State ($\mathbb{T}_4 = \{ -1, 0, +1, \text{NULL} \}$) ([`QuadState`](file:///home/professorseanex/OmniCode/src/core/ternary.rs#L25-L60), [Specification](file:///home/professorseanex/OmniCode/docs/02_logic_and_algebra/01_balanced_ternary_quad_state.md)) |
| **Structural Paradigm** | Deterministic 4-Block Architecture ([`OmniLinter`](file:///home/professorseanex/OmniCode/src/diagnostics/linter.rs#L25-L60), [Specification](file:///home/professorseanex/OmniCode/docs/03_architecture/03_four_block_architecture.md)) |
| **Morphological Physics** | 6 Boundary Types (`[]`, `<>`, `{}`, `{]`, `[}`, `()`) ([`BoundaryType`](file:///home/professorseanex/OmniCode/src/core/boundaries.rs#L24-L62), [Specification](file:///home/professorseanex/OmniCode/docs/01_foundations/02_boundary_types_and_asides.md)) |
| **Cognitive Framework** | Neural-Symbolic AI Cortex, Transducer, Episodic Memory Scrolls ([`AICortex`](file:///home/professorseanex/OmniCode/src/ai/model/cortex.rs), [Specification](file:///home/professorseanex/OmniCode/docs/04_cognitive_and_autonomous_systems/01_ai_cortex_and_neural_symbolic_loop.md)) |
| **Diagnostics & Healing** | Autonomous Restorator, OmniDebugger Rollbacks ([`Restorator`](file:///home/professorseanex/OmniCode/src/diagnostics/restorator.rs), [Specification](file:///home/professorseanex/OmniCode/docs/04_cognitive_and_autonomous_systems/03_autonomous_restorator_and_self_healing.md)) |
| **Author / Lead** | ProfessorSeanEX |
| **License** | MIT OR Apache-2.0 |

---

### Core Pillars

1. **Formal Ontology & Set Theory:** Clean separation of an entity's intrinsic essence (Denotation) from its environmental context (Connotation). Detailed in [Document 01: Ontological Principles & Set Theory](file:///home/professorseanex/OmniCode/docs/01_foundations/01_ontological_principles.md).
2. **Balanced Ternary Quad-State Engine:** Execution physics governed by $\mathbb{T}_4 = \{ -1, 0, +1, \text{NULL} \}$ (`Rollback`, `Await`, `Proceed`, `Halt`), delivering native transactional state machines without runtime exceptions or unchecked panics. Defined in [Document 01: Balanced Ternary & Quad-State](file:///home/professorseanex/OmniCode/docs/02_logic_and_algebra/01_balanced_ternary_quad_state.md).
3. **Morphological Boundary Physics:** Brackets are active semantic operators encoding causality direction, containment hierarchy, and physical mutability: `[...]`, `<...>`, `{...}`, `{...]`, `[...}`, `(...)`. Detailed in [Document 02: Boundary Types & Asides](file:///home/professorseanex/OmniCode/docs/01_foundations/02_boundary_types_and_asides.md).
4. **First-Class Asides `(...)`:** Comments are preserved in the AST ([`ASTNode::Aside`](file:///home/professorseanex/OmniCode/src/ast/nodes.rs#L40-L65)) as machine-readable intent, formal invariants, and AI cognitive traces, eliminating the human-machine semantic gap.
5. **Universal 4-Block Architecture:** Every source file, configuration, and episodic scroll adheres strictly to a 4-block lifecycle: `Metadata` $\to$ `Opening` $\to$ `Body` $\to$ `Closing`. Specified in [Document 03: Universal 4-Block Architecture](file:///home/professorseanex/OmniCode/docs/03_architecture/03_four_block_architecture.md).
6. **Cognitive Loop & Episodic Learning:** Seamless bidirectional translation between natural language discourse and deterministic 4-Block Turing reels, archived into an episodic memory bank (`.scroll` and `.log`). See [Volume 4: Document 01](file:///home/professorseanex/OmniCode/docs/04_cognitive_and_autonomous_systems/01_ai_cortex_and_neural_symbolic_loop.md) and [Volume 4: Document 02](file:///home/professorseanex/OmniCode/docs/04_cognitive_and_autonomous_systems/02_episodic_scrolls_and_experiential_learning.md).
7. **Autonomous Self-Healing (Restorator):** Built-in diagnostic immune system that repairs boundary mismatches, unclosed delimiters, missing standard imports, and structural anomalies on the fly. Detailed in [Volume 4: Document 03: The Restorator](file:///home/professorseanex/OmniCode/docs/04_cognitive_and_autonomous_systems/03_autonomous_restorator_and_self_healing.md).
8. **Rust Core Foundation:** Zero runtime buckling, memory safety without garbage collection, and seamless LLVM/C ABI interoperability. Detailed in [Why Rust as Core Foundation](file:///home/professorseanex/OmniCode/docs/03_architecture/03_four_block_architecture.md#why-rust-as-the-core-foundation).

---

### Why Rust as the Core Foundation?

The OmniCode compiler, interpreter, and diagnostic engine are written in **Rust** for four decisive reasons:

1. **Zero Runtime Buckling (Memory Safety without Garbage Collection):**
   A language operating at the bare metal cannot tolerate stop-the-world garbage collection pauses or undefined behavior. Rust guarantees compile-time memory safety, preventing segmentation faults, data races, and dangling references.
2. **Algebraic Data Types & Exhaustive Pattern Matching:**
   Compilers are state transformers. Rust's `enum` types with associated payloads ([`QuadState`](file:///home/professorseanex/OmniCode/src/core/ternary.rs#L25-L60), [`BoundaryType`](file:///home/professorseanex/OmniCode/src/core/boundaries.rs#L24-L62), [`ASTNode`](file:///home/professorseanex/OmniCode/src/ast/nodes.rs#L20-L80), [`DirectionalVector`](file:///home/professorseanex/OmniCode/src/core/operators.rs#L30-L65)) and exhaustive pattern matching ensure that every boundary crossing and state transition is strictly accounted for.
3. **Zero-Cost Abstractions & Native Execution Speed:**
   The OmniCode AST, recursive-descent parser, transactional snapshot debugger, and evaluation pipeline run at bare-metal execution speed with no interpreter overhead.
4. **LLVM & C ABI Interoperability:**
   Rust provides first-class bindings to LLVM and frictionless C FFI (`extern "C"`), making it the optimal host to drive OmniCode's AOT native compilation and cross-platform transpilation pipelines documented in [Document 02: Execution Pipeline](file:///home/professorseanex/OmniCode/docs/03_architecture/02_execution_pipeline.md).

---

## 1. Getting Started & Environment Setup

### System Prerequisites

* **Rust Compiler:** Version `1.85+` ([`rustc`](https://www.rust-lang.org/), [`cargo`](file:///home/professorseanex/OmniCode/Cargo.toml))
* **Terminal Shell:** POSIX Bash or equivalent Linux/macOS shell
* **Hardware Emulation & Cross-Toolchains (Optional for Bare-Metal):**
  * `clang`, `lld`, `nasm`, `xorriso`, `qemu-system-x86_64` (See [Document 04: Bare-Metal Toolchains](file:///home/professorseanex/OmniCode/docs/04_cognitive_and_autonomous_systems/04_bare_metal_and_cross_platform_toolchain.md))

### Toolchain Verification

OmniCode includes an automated diagnostic harness to verify your local build and bare-metal compilation toolchains:

```bash
./scripts/check_toolchain.sh
```

*Script source:* [`scripts/check_toolchain.sh`](file:///home/professorseanex/OmniCode/scripts/check_toolchain.sh)

### Bare-Metal & Containerized Toolchain (Docker & Compose)

For fully isolated cross-compilation and hardware emulation across `x86_64`, `aarch64`, and `riscv64gc` bare-metal targets:

```bash
# Build and launch the containerized toolchain
docker compose up -d

# Enter the container environment
docker compose exec omnicode-toolchain bash
```

*Infrastructure files & ports:*

* Multi-architecture Docker image specification: [`Dockerfile`](file:///home/professorseanex/OmniCode/Dockerfile)
* Container orchestration & service topology: [`docker-compose.yml`](file:///home/professorseanex/OmniCode/docker-compose.yml)
* Pre-configured ports: `1234` (QEMU GDB stub), `5900` (QEMU VNC framebuffer), `8080` (Cognitive web inspector)
* Full bare-metal guide: [Document 04: Bare-Metal OS & Cross-Platform Toolchains](file:///home/professorseanex/OmniCode/docs/04_cognitive_and_autonomous_systems/04_bare_metal_and_cross_platform_toolchain.md)

### Visual Studio Code IDE Extension

OmniCode provides an official VS Code extension (`omnicode-language-0.1.2`) offering TextMate syntax highlighting for `.omni` and `.scroll` files, bracket-pair colorization, and canonical 4-block snippets.

To install the extension:

```bash
code --install-extension editors/vscode/omnicode-language-0.1.2.vsix
```

*Extension directory:* [`editors/vscode/`](file:///home/professorseanex/OmniCode/editors/vscode)  
*Extension manual:* [`editors/vscode/README.md`](file:///home/professorseanex/OmniCode/editors/vscode/README.md)  
*TextMate grammar:* [`editors/vscode/syntaxes/omnicode.tmLanguage.json`](file:///home/professorseanex/OmniCode/editors/vscode/syntaxes/omnicode.tmLanguage.json)  
*Snippets:* [`editors/vscode/snippets/omnicode.json`](file:///home/professorseanex/OmniCode/editors/vscode/snippets/omnicode.json)

### Interactive Terminal Environment (REPL)

Launch the interactive terminal shell ([`src/cli/repl.rs`](file:///home/professorseanex/OmniCode/src/cli/repl.rs)) for sub-millisecond evaluation of balanced ternary expressions, live environment inspection, and transactional rollback experiments:

```bash
cargo run
# or explicitly:
cargo run -- repl
```

#### REPL Live Commands

| Command | Action | Implementation Reference |
| :--- | :--- | :--- |
| `:env` | Display all currently bound identifiers, types, and values | [`Environment`](file:///home/professorseanex/OmniCode/src/runtime/environment.rs) |
| `:ast <expr>` | Tokenize and print the Abstract Syntax Tree for an expression | [`Parser`](file:///home/professorseanex/OmniCode/src/parser/parser.rs) |
| `:failpoints` | Inspect the session's diagnostic failpoint log | [`DiagnosticLogger`](file:///home/professorseanex/OmniCode/src/diagnostics/logger.rs) |
| `:load <file>` | Parse and evaluate an external `.omni` source file or `.scroll` | [`Evaluator`](file:///home/professorseanex/OmniCode/src/runtime/evaluator.rs) |
| `:reset` | Reset the global environment to pristine initial state | [`Environment::new`](file:///home/professorseanex/OmniCode/src/runtime/environment.rs) |
| `:help` | Show interactive REPL command help | [`run_repl`](file:///home/professorseanex/OmniCode/src/cli/repl.rs) |
| `:exit`, `:quit` | Exit the REPL session | [`cli/mod.rs`](file:///home/professorseanex/OmniCode/src/cli/mod.rs) |

---

## 2. Language Syntax & Core Concepts

### The Universal 4-Block File Architecture

Every `.omni` program, standard library module, and episodic scroll follows the 4-block lifecycle (detailed in [Document 03: Four Block Architecture](file:///home/professorseanex/OmniCode/docs/03_architecture/03_four_block_architecture.md)):

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

*See canonical implementation showcase:* [`examples/canonical_notation.omni`](file:///home/professorseanex/OmniCode/examples/canonical_notation.omni)

---

### Morphological Boundary Delimiters

Brackets in OmniCode are active operators defining mutability, hierarchy ranking, and directional causality. Implemented in [`src/core/boundaries.rs`](file:///home/professorseanex/OmniCode/src/core/boundaries.rs) and specified in [Document 02: Boundary Types & Asides](file:///home/professorseanex/OmniCode/docs/01_foundations/02_boundary_types_and_asides.md):

| Delimiter | Boundary Type | Semantic Role | Behavior & Physics |
| :---: | :--- | :--- | :--- |
| `[...]` | **Rigid / Fixed** | Denotation / Concrete | Fixed physical memory, immutable structure, discrete value. |
| `<...>` | **Dynamic / Fluid** | Process / Type Invariant | Mutable state, transformation vector, category identifier. |
| `{...}` | **Universal / Omni** | Scoped Universe / Holism | Enclosing set domain, parent scope, environmental context. |
| `{...]` | **Accumulator / Absorber** | Fluid In $\to$ Fixed Out | Consumes external fluid changes and outputs a fixed datum. |
| `[...}` | **Generator / Source** | Fixed In $\to$ Fluid Out | Emanates from a fixed constant and radiates mutable streams. |
| `(...)` | **First-Class Aside** | Intent / Cognitive Trace | Non-interfering metadata, machine-readable reasoning, and proofs. |

*Demonstration code:* [`examples/01_boundaries_and_asides.omni`](file:///home/professorseanex/OmniCode/examples/01_boundaries_and_asides.omni)

---

### Balanced Ternary Quad-State Physics ($\mathbb{T}_4$)

OmniCode eliminates exceptions by treating system states as continuous physical trajectories across four discrete values (implemented in [`src/core/ternary.rs`](file:///home/professorseanex/OmniCode/src/core/ternary.rs) and specified in [Document 01: Balanced Ternary & Quad-State](file:///home/professorseanex/OmniCode/docs/02_logic_and_algebra/01_balanced_ternary_quad_state.md)):

$$\mathbb{T}_4 = \{ -1, 0, +1, \text{NULL} \}$$

* **`+1:PROCEED` ([`PROCEED`](file:///home/professorseanex/OmniCode/src/core/ternary.rs#L12)):** Affirmative forward execution, condition satisfied, transaction committed.
* **`0:AWAIT` ([`AWAIT`](file:///home/professorseanex/OmniCode/src/core/ternary.rs#L11)):** Indeterminate, suspended, or awaiting external input/consensus.
* **`-1:ROLLBACK` ([`ROLLBACK`](file:///home/professorseanex/OmniCode/src/core/ternary.rs#L10)):** Contra/veto state. Automatically rolls back the environment to the previous stable snapshot via [`OmniDebugger`](file:///home/professorseanex/OmniCode/src/diagnostics/debugger.rs).
* **`NULL:HALT` ([`HALT`](file:///home/professorseanex/OmniCode/src/core/ternary.rs#L13)):** Terminal void. Absorbs subsequent transformations and halts execution safely.

#### Ternary Truth Tables

| Operation | Input A | Input B | Output | Semantic Result |
| :--- | :---: | :---: | :---: | :--- |
| **Negation (`--`)** | `+1` | — | `-1` | Logical inversion / Rollback trigger |
| **Negation (`--`)** | `0` | — | `0` | Neutral balance preserved |
| **Negation (`--`)** | `-1` | — | `+1` | Contra-inversion to Proceed |
| **Consensus Conjunction (`::`)** | `+1` | `+1` | `+1` | Full agreement |
| **Consensus Conjunction (`::`)** | `+1` | `-1` | `-1` | Single veto triggers Rollback |
| **Consensus Conjunction (`::`)** | `+1` | `0` | `0` | Indeterminate awaiting resolution |

*Demonstration code:* [`examples/02_logic_and_quad_state.omni`](file:///home/professorseanex/OmniCode/examples/02_logic_and_quad_state.omni)

---

### The 3D Coordinate System of Computation

Every operation is positioned across three orthogonal coordinate axes (implemented in [`src/core/operators.rs`](file:///home/professorseanex/OmniCode/src/core/operators.rs) and specified in [Document 01: 3D Coordinate System](file:///home/professorseanex/OmniCode/docs/03_architecture/01_coordinate_system_of_computation.md)):

```text
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

*Demonstration code:* [`examples/03_cardinal_directions.omni`](file:///home/professorseanex/OmniCode/examples/03_cardinal_directions.omni) and [`examples/04_turing_track_tape.omni`](file:///home/professorseanex/OmniCode/examples/04_turing_track_tape.omni)

---

### Autonomous Self-Healing Engine (`Restorator`)

The OmniCode [`Restorator`](file:///home/professorseanex/OmniCode/src/diagnostics/restorator.rs) operates as an autonomous immune system (specified in [Document 03: The Restorator](file:///home/professorseanex/OmniCode/docs/04_cognitive_and_autonomous_systems/03_autonomous_restorator_and_self_healing.md)):

```bash
# Auto-heal syntax, boundary mismatches, and 4-block anomalies
cargo run -- restore path/to/file.omni --write
```

Capabilities:

1. **Boundary Balancing & Healing:** Detects unclosed delimiters (`[`, `<`, `{`) and closes them based on morphology rank.
2. **Structural Scaffolding:** Automatically wraps loose statements into canonical 4-Block architecture (`Metadata`, `Opening`, `Body`, `Closing`).
3. **Import Injection:** Injects missing standard library imports (`Core.Arithmetic`, `Core.Logic`, `Core.Vectors`) into the Opening block.

---

### AI Cortex & Neural-Symbolic Transduction

OmniCode features native cognitive processing via the [`AICortex`](file:///home/professorseanex/OmniCode/src/ai/model/cortex.rs) and [`AITransducer`](file:///home/professorseanex/OmniCode/src/ai/transducer.rs) (specified in [Document 01: The AI Cortex](file:///home/professorseanex/OmniCode/docs/04_cognitive_and_autonomous_systems/01_ai_cortex_and_neural_symbolic_loop.md)):

```bash
cargo run -- ai "Add 5 units to inventory A which contains 12 units"
```

The AI pipeline:

1. **Natural Language Transduction:** Converts freeform human discourse into canonical 4-Block OmniCode.
2. **Autonomous Restoration:** Heals inferred boundaries and structures.
3. **Deterministic Evaluation:** Runs the generated program through [`Evaluator`](file:///home/professorseanex/OmniCode/src/runtime/evaluator.rs) and [`OmniDebugger`](file:///home/professorseanex/OmniCode/src/diagnostics/debugger.rs).
4. **Episodic Inscription:** Generates dual logs via [`ScrollLogger`](file:///home/professorseanex/OmniCode/src/ai/model/scroll.rs):
   * **`.scroll`:** Native executable 4-Block Turing reel recording the session's episodic trace.
   * **`.log`:** Microsecond telemetry tracking channel logs, execution duration, and failpoint telemetry.

#### Inspecting Episodic Memory

```bash
cargo run -- scrolls
```

Outputs learning statistics digested by [`AILearner`](file:///home/professorseanex/OmniCode/src/ai/model/learner.rs) from the [`scrolls/`](file:///home/professorseanex/OmniCode/scrolls) directory (specified in [Document 02: Episodic Scrolls](file:///home/professorseanex/OmniCode/docs/04_cognitive_and_autonomous_systems/02_episodic_scrolls_and_experiential_learning.md)):

* Total inscribed episodes.
* QuadState distribution (`Proceed`, `Await`, `Rollback`, `Halt`).
* Learned repairs cataloged by the Restorator.

---

### Standard Library Architecture (`lib/core/`)

OmniCode includes a native standard library in [`lib/core/`](file:///home/professorseanex/OmniCode/lib/core) (specified in [Document 05: Core Standard Library](file:///home/professorseanex/OmniCode/docs/04_cognitive_and_autonomous_systems/05_core_standard_libraries.md)):

* **[`arithmetic.omni`](file:///home/professorseanex/OmniCode/lib/core/arithmetic.omni):** Quad-state safe addition, subtraction, multiplication, and non-zero division.
* **[`logic.omni`](file:///home/professorseanex/OmniCode/lib/core/logic.omni):** Ternary Kleene/Lukasiewicz logic gates (AND, OR, NOT, XOR, and Consensual Agreement).
* **[`vectors.omni`](file:///home/professorseanex/OmniCode/lib/core/vectors.omni):** Causality, Being, and Scale directional operations with caveat guard checks.

---

## 3. CLI Command Reference & Verification Suite

### CLI Command Reference

Implemented in [`src/main.rs`](file:///home/professorseanex/OmniCode/src/main.rs):

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

### Verification & Test Harness

OmniCode maintains a 100% passing test suite with **24 unit, failpoint, and AI integration tests**:

```bash
cargo test
```

#### Test Suite Breakdown

* **5 Core Unit Tests** ([`src/lib.rs`](file:///home/professorseanex/OmniCode/src/lib.rs)):
  * Boundary ranking hierarchy validation ([`boundaries::tests`](file:///home/professorseanex/OmniCode/src/core/boundaries.rs#L85)).
  * File type detection and canonical scaffold generation ([`filetype::tests`](file:///home/professorseanex/OmniCode/src/core/filetype.rs#L240)).
  * Directional vector shorthand and canonical parsing ([`operators::tests`](file:///home/professorseanex/OmniCode/src/core/operators.rs#L95)).
  * Quad-state micro and macro transitions ([`ternary::tests`](file:///home/professorseanex/OmniCode/src/core/ternary.rs#L80)).
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

## 4. Project Repository Layout & Formal Seal

```text
OmniCode/
├── .gitignore                         # Canonical 4-block Git exclusions
├── Cargo.toml                         # Canonical 4-block Rust package manifest (Edition 2024)
├── Cargo.lock                         # Cargo dependency resolution lockfile
├── Dockerfile                         # Multi-arch bare-metal container environment
├── docker-compose.yml                 # Toolchain container orchestration & QEMU debugging
├── README.md                          # Root architectural documentation
├── docs/                              # Comprehensive documentation series
│   ├── README.md                      # Documentation index & symbol matrix
│   ├── 01_foundations/                # Ontology, Set Theory, Boundaries, Asides
│   ├── 02_logic_and_algebra/          # Balanced Ternary, Connective Algebra
│   ├── 03_architecture/               # 3D Coordinates, Execution Pipeline, 4-Block Lifecycle
│   └── 04_cognitive_and_autonomous_systems/ # AI Cortex, Episodic Scrolls, Restorator, Toolchains
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
│   └── check_toolchain.sh             # Toolchain verification script
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

### Formal Invariant Seal

OmniCode source definitions, language physics, and diagnostic boundaries are verified and sealed under the Universal 4-Block Architecture. All transformations are deterministic, bounded, and self-verifying.

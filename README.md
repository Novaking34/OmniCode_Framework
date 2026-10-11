# OmniCode

> **A Dual-Readable Ontological Programming Language & Bare-Metal OS Engineered for Unambiguous Human Comprehension and Machine Determinism.**

OmniCode bridges the entire spectrum of computing—operating with bare-metal low-level precision (pointers, CPU registers, raw memory models, hardware transducers) while natively expressing high-level mathematical abstractions, declarative logic, balanced ternary state machines, universal linguistics, and machine-readable cognitive intent.

---

## Manifest Metadata

| Attribute | Specification |
| :--- | :--- |
| **Engine Version** | `0.1.0` |
| **Organization** | CreativeWorkzStudio LLC |
| **Core Runtime** | [Rust Engine (`edition = "2024"`, `rust-version = "1.85+"`)](file:///home/professorseanex/OmniCode/src/lib.rs) |
| **Execution Truth Space** | Balanced Ternary Quad-State ($\mathbb{T}_4 = \{ -1, 0, +1, \text{NULL} \}$) ([`QuadState`](file:///home/professorseanex/OmniCode/src/core/ternary.rs#L25-L60), [Specification](file:///home/professorseanex/OmniCode/docs/02_logic_and_algebra/01_balanced_ternary_quad_state.md)) |
| **Structural Paradigm** | Deterministic 4-Block Architecture ([`OmniLinter`](file:///home/professorseanex/OmniCode/src/diagnostics/linter.rs), [Specification](file:///home/professorseanex/OmniCode/docs/03_architecture/03_four_block_architecture.md)) |
| **Morphological Physics** | 6 Boundary Types (`[]`, `<>`, `{}`, `{]`, `[}`, `()`) ([`BoundaryType`](file:///home/professorseanex/OmniCode/src/core/boundaries.rs#L24-L62), [Specification](file:///home/professorseanex/OmniCode/docs/01_foundations/02_boundary_types_and_asides.md)) |
| **Universal Linguistics** | 8 Parts of Speech Matrix & 8-Slot Canonical Sentences ([`linguistics.rs`](file:///home/professorseanex/OmniCode/src/core/linguistics.rs), [Specification](file:///home/professorseanex/OmniCode/docs/01_foundations/03_eight_parts_of_speech_matrix.md)) |
| **Physical Memory Model** | 8-Bit <-> 5-Trit Bijective Codec & Bounded Storehouses with `v--` Partitioning ([`cell.rs`](file:///home/professorseanex/OmniCode/src/core/cell.rs), [`vessel.rs`](file:///home/professorseanex/OmniCode/src/core/vessel.rs), [`lib/core/memory.omni`](file:///home/professorseanex/OmniCode/lib/core/memory.omni)) |
| **Operating System Genesis** | Multi-Track Process Tapes (`Roll` $\to$ `Tape` $\to$ `Track` 0/1/2) ([`OmniScheduler`](file:///home/professorseanex/OmniCode/src/terminal/scheduler.rs), [`TerminalSession`](file:///home/professorseanex/OmniCode/src/terminal/session.rs), [`boot.tape`](file:///home/professorseanex/OmniCode/lib/kernel/boot.tape)) |
| **Cognitive Framework** | Neural-Symbolic AI Cortex, `AIMemory` Store, Offline `BiblicalLexicon` ([`AICortex`](file:///home/professorseanex/OmniCode/src/ai/model/cortex.rs), [`AIMemory`](file:///home/professorseanex/OmniCode/src/ai/model/memory.rs), [`manifest.omni`](file:///home/professorseanex/OmniCode/manifest.omni)) |
| **Diagnostics & Healing** | Autonomous Restorator, OmniDebugger Rollbacks, Sensor Logger ([`Restorator`](file:///home/professorseanex/OmniCode/src/diagnostics/restorator.rs), [Specification](file:///home/professorseanex/OmniCode/docs/04_cognitive_and_autonomous_systems/03_autonomous_restorator_and_self_healing.md)) |
| **Interfaces & Studio** | Pure Standard-Library HTTP Server, Web Terminal HUD, Desktop GUI, VS Code Extension ([`server.rs`](file:///home/professorseanex/OmniCode/src/cli/server.rs), [`apps/gui/`](file:///home/professorseanex/OmniCode/apps/gui)) |
| **Test Suite Status** | **48 / 48 Passing Tests** across 3-tier harness (100% Pass Rate) |
| **Author / Lead** | ProfessorSeanEX |
| **License** | MIT OR Apache-2.0 |

---

### Core Pillars

1. **Formal Ontology & Set Theory:** Clean separation of an entity's intrinsic essence (Denotation) from its environmental context (Connotation). Detailed in [Document 01: Ontological Principles & Set Theory](file:///home/professorseanex/OmniCode/docs/01_foundations/01_ontological_principles.md).
2. **Balanced Ternary Quad-State Engine:** Execution physics governed by $\mathbb{T}_4 = \{ -1, 0, +1, \text{NULL} \}$ (`Rollback`, `Await`, `Proceed`, `Halt`), delivering native transactional state machines without runtime exceptions or unchecked panics. Defined in [Document 01: Balanced Ternary & Quad-State](file:///home/professorseanex/OmniCode/docs/02_logic_and_algebra/01_balanced_ternary_quad_state.md).
3. **Morphological Boundary Physics:** Brackets are active semantic operators encoding causality direction, containment hierarchy, and physical mutability: `[...]`, `<...>`, `{...}`, `{...]`, `[...}`, `(...)`. Detailed in [Document 02: Boundary Types & Asides](file:///home/professorseanex/OmniCode/docs/01_foundations/02_boundary_types_and_asides.md).
4. **First-Class Asides `(...)`:** Comments are preserved in the AST ([`ASTNode::Aside`](file:///home/professorseanex/OmniCode/src/ast/nodes.rs)) as machine-readable intent, formal invariants, and AI cognitive traces, eliminating the human-machine semantic gap.
5. **Universal 4-Block Architecture:** Every source file, configuration, and episodic scroll adheres strictly to a 4-block lifecycle: `Metadata` $\to$ `Opening` $\to$ `Body` $\to$ `Closing`. Specified in [Document 03: Universal 4-Block Architecture](file:///home/professorseanex/OmniCode/docs/03_architecture/03_four_block_architecture.md).
6. **Universal Linguistics & 8 Parts of Speech:** Nouns are defined; verbs are symbolized. Every statement maps cleanly into an 8-Slot Canonical Sentence and the 6 Cardinal Classification Clauses (`WHO`, `WHAT`, `WHEN`, `WHERE`, `WHY`, `HOW`). See [Document 03: Eight Parts of Speech Matrix](file:///home/professorseanex/OmniCode/docs/01_foundations/03_eight_parts_of_speech_matrix.md).
7. **Dense Memory Cells & Hardware Transducer:** Bijective hardware codec mapping 8-bit bytes ($2^8 = 256$) to 5-trit balanced ternary ($3^5 = 243$ data states $[-121 \dots +121]$ + 13 out-of-band control states). See [`src/core/cell.rs`](file:///home/professorseanex/OmniCode/src/core/cell.rs).
8. **Bounded Memory Vessels & Storehouses:** Contiguous memory blocks with invariant bounds and dynamic `v--` partitioning, delivering safe indexing and zero-cost allocation. See [`src/core/vessel.rs`](file:///home/professorseanex/OmniCode/src/core/vessel.rs) and [`lib/core/memory.omni`](file:///home/professorseanex/OmniCode/lib/core/memory.omni).
9. **Operating System Genesis (Process Tapes & Multi-Track Scheduling):** Physical execution hierarchy: `Roll` $\to$ `Tape` $\to$ `Track`. Cooperative priority scheduling across Track 0 (KERNEL), Track 1 (TERMINAL), and Track 2 (APP). Bootable via [`lib/kernel/boot.tape`](file:///home/professorseanex/OmniCode/lib/kernel/boot.tape).
10. **Cognitive Loop & Episodic Learning:** Grounded in an offline `BiblicalLexicon` (6 domains) and invariant `manifest.omni` covenant. The internal AI formulates thoughts directly in native OmniCode and archives episodic memory scrolls (`runtime/scrolls/`). See [`src/ai/`](file:///home/professorseanex/OmniCode/src/ai).
11. **Autonomous Self-Healing (Restorator):** Built-in diagnostic immune system that repairs boundary mismatches, unclosed delimiters, missing standard imports, and structural anomalies on the fly. Detailed in [Volume 4: Document 03: The Restorator](file:///home/professorseanex/OmniCode/docs/04_cognitive_and_autonomous_systems/03_autonomous_restorator_and_self_healing.md).
12. **Rust Core Foundation:** Zero runtime buckling, memory safety without garbage collection, and seamless LLVM/C ABI interoperability.

---

### Why Rust as the Core Foundation?

The OmniCode compiler, interpreter, and diagnostic engine are written in **Rust** for four decisive reasons:

1. **Zero Runtime Buckling (Memory Safety without Garbage Collection):**
   A language operating at the bare metal cannot tolerate stop-the-world garbage collection pauses or undefined behavior. Rust guarantees compile-time memory safety, preventing segmentation faults, data races, and dangling references.
2. **Algebraic Data Types & Exhaustive Pattern Matching:**
   Compilers are state transformers. Rust's `enum` types with associated payloads ([`QuadState`](file:///home/professorseanex/OmniCode/src/core/ternary.rs), [`BoundaryType`](file:///home/professorseanex/OmniCode/src/core/boundaries.rs), [`ASTNode`](file:///home/professorseanex/OmniCode/src/ast/nodes.rs), [`DirectionalVector`](file:///home/professorseanex/OmniCode/src/core/operators.rs)) and exhaustive pattern matching ensure that every boundary crossing and state transition is strictly accounted for.
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
./infra/scripts/check_toolchain.sh
```

*Script source:* [`infra/scripts/check_toolchain.sh`](file:///home/professorseanex/OmniCode/infra/scripts/check_toolchain.sh)

### Bare-Metal & Containerized Toolchain (Docker & Compose)

For fully isolated cross-compilation and hardware emulation across `x86_64`, `aarch64`, and `riscv64gc` bare-metal targets:

```bash
# Build and launch the containerized toolchain
cd infra/docker && docker compose up -d

# Enter the container environment
docker compose exec omnicode-toolchain bash
```

*Infrastructure files & ports:*

* Multi-architecture Docker image specification: [`infra/docker/Dockerfile`](file:///home/professorseanex/OmniCode/infra/docker/Dockerfile)
* Container orchestration & service topology: [`infra/docker/docker-compose.yml`](file:///home/professorseanex/OmniCode/infra/docker/docker-compose.yml)
* Pre-configured ports: `1234` (QEMU GDB stub), `5900` (QEMU VNC framebuffer), `8080` (Cognitive web inspector)
* Full bare-metal guide: [Document 04: Bare-Metal OS & Cross-Platform Toolchains](file:///home/professorseanex/OmniCode/docs/04_cognitive_and_autonomous_systems/04_bare_metal_and_cross_platform_toolchain.md)

### Visual Studio Code IDE Extension

OmniCode provides an official VS Code extension (`omnicode-language-0.1.2`) offering TextMate syntax highlighting for `.omni`, `.scroll`, and `.tape` files, bracket-pair colorization, and canonical 4-block snippets.

To install the extension:

```bash
code --install-extension apps/editors/vscode/omnicode-language-0.1.2.vsix
```

*Extension directory:* [`apps/editors/vscode/`](file:///home/professorseanex/OmniCode/apps/editors/vscode)  
*Extension manual:* [`apps/editors/vscode/README.md`](file:///home/professorseanex/OmniCode/apps/editors/vscode/README.md)  
*TextMate grammar:* [`apps/editors/vscode/syntaxes/omnicode.tmLanguage.json`](file:///home/professorseanex/OmniCode/apps/editors/vscode/syntaxes/omnicode.tmLanguage.json)  
*Snippets:* [`apps/editors/vscode/snippets/omnicode.json`](file:///home/professorseanex/OmniCode/apps/editors/vscode/snippets/omnicode.json)

---

### Executing Code & Process Tapes

OmniCode supports source reels (`.omni`), cognitive scrolls (`.scroll`), and sequential process tapes (`.tape`):

```bash
# Execute the Genesis OS Boot Process Tape
cargo run -- run lib/kernel/boot.tape

# Execute standard memory module
cargo run -- run lib/core/memory.omni

# Validate 4-Block structural compliance
cargo run -- lint manifest.omni
```

---

### Web Studio GUI Server & Desktop Application

OmniCode includes a built-in zero-dependency HTTP server hosting the Web Studio HUD and Terminal Gateway:

```bash
# Launch the local Web Studio & Terminal server on port 8080
cargo run -- gui --port 8080

# Or run the native Python/Qt desktop HUD
python3 apps/gui/desktop_app.py
```

*Endpoints available at `http://localhost:8080`:*

* `/terminal` — Interactive web-based terminal HUD
* `/studio` — 4-Block code editor and visual runtime
* `/specs` — Interactive architectural documentation viewer
* `/model` — AI Cortex neural-symbolic inspector
* `/lexicon` — Offline Biblical world lexicon explorer
* `/scrolls` — Episodic memory stream visualizer
* `/api/health` — REST health probe

---

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
| `:load <file>` | Parse and evaluate an external `.omni`, `.scroll`, or `.tape` | [`Evaluator`](file:///home/professorseanex/OmniCode/src/runtime/evaluator.rs) |
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
    import: [ Core.Arithmetic :: Core.Logic :: Core.Vectors :: Core.Memory ],
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

* **`+1:PROCEED` ([`PROCEED`](file:///home/professorseanex/OmniCode/src/core/ternary.rs)):** Affirmative forward execution, condition satisfied, transaction committed.
* **`0:AWAIT` ([`AWAIT`](file:///home/professorseanex/OmniCode/src/core/ternary.rs)):** Indeterminate, suspended, or awaiting external input/consensus.
* **`-1:ROLLBACK` ([`ROLLBACK`](file:///home/professorseanex/OmniCode/src/core/ternary.rs)):** Contra/veto state. Automatically rolls back the environment to the previous stable snapshot via [`OmniDebugger`](file:///home/professorseanex/OmniCode/src/diagnostics/debugger.rs).
* **`NULL:HALT` ([`HALT`](file:///home/professorseanex/OmniCode/src/core/ternary.rs)):** Terminal void. Absorbs subsequent transformations and halts execution safely.

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

### Universal Linguistics & 8-Slot Canonical Sentences

OmniCode bridges human cognition and silicon execution by aligning 1:1 with universal grammar:

$$\text{“}\underbrace{\text{Oh!}}_{\text{Slot 1: Interjection}}\quad \underbrace{\text{But}}_{\text{Slot 2: Conjunction}}\quad \underbrace{\text{my}}_{\text{Slot 3: Pronoun}}\quad \underbrace{\text{old}}_{\text{Slot 4: Adjective}}\quad \underbrace{\text{man}}_{\text{Slot 5: Noun}}\quad \underbrace{\text{walked}}_{\text{Slot 6: Verb}}\quad \underbrace{\text{straight}}_{\text{Slot 7: Adverb}}\quad \underbrace{\text{ahead}}_{\text{Slot 8: Preposition}}\text{.”}$$

* **Nouns are Defined:** Entities, storage containers, measures, types (`Vessel`, `Storehouse`, `Cell`).
* **Verbs are Symbolized:** Directional causal vectors (`-->`, `<--`, `v--`, `^--`, `=`).
* **6 Cardinal Classification Clauses:**
  * `WHO` — Security Principal / Caller (`WHO OmniCode_Kernel`)
  * `WHAT` — Target Entity / Vessel Payload (`WHAT SystemGenesis`)
  * `WHEN` — Lifecycle Phase / Trigger (`WHEN PowerOn`)
  * `WHERE` — Spatial Site / Storehouse Address (`WHERE ram`)
  * `WHY` — Intent Aside / Covenant Anchor (`WHY ( Initialize kernel )`)
  * `HOW` — Quad-State Outcome & Modulation (`HOW +1:PROCEED`)

---

### Physical Computing Hierarchy & Process Tapes

OmniCode models physical computing into three nested tiers:
$$\text{Roll (Instruction)} \longrightarrow \text{Tape (Process Ribbon)} \longrightarrow \text{Track (Hardware Channel)}$$

* **Roll (`Roll`):** An atomic state mutation or expression.
* **Tape (`Tape` / `.tape`):** A sequential process ribbon led by a **Tape Head** covenant and an ordered stream of subordinate rolls.
* **Track (`Track`):** Three discrete execution channels managed by [`OmniScheduler`](file:///home/professorseanex/OmniCode/src/terminal/scheduler.rs):
  * `Track 0 [KERNEL]`: Protected kernel space, hardware initialization, genesis boot.
  * `Track 1 [TERMINAL]`: Interactive console HUD, REPL environment, video buffers.
  * `Track 2 [APP]`: Userspace applications, background workers, cognitive models.

---

### Bounded Memory Vessels & Hardware Transducer

OmniCode eliminates undefined memory behavior through two foundational primitives:

1. **Bijective Hardware Cell Transducer ([`src/core/cell.rs`](file:///home/professorseanex/OmniCode/src/core/cell.rs)):**
   * Maps 8-bit bytes ($2^8 = 256$) to 5 balanced trits ($3^5 = 243$ values $[-121 \dots +121]$).
   * Allocates 13 out-of-band hardware control states: `243 = NULL:HALT`, `244 = 0:AWAIT`, `245 = -1:ROLLBACK`, `246 = +1:PROCEED`, `247..255 = Hardware Traps`.
2. **Storehouses & `v--` Partitioning ([`src/core/vessel.rs`](file:///home/professorseanex/OmniCode/src/core/vessel.rs)):**
   * Carves child workspaces with strict invariant boundaries:

     ```omni
     Root_RAM [ 4096 ]
     Root_RAM v-- Kernel_Partition [ 1024 ]
     Root_RAM v-- Terminal_Partition [ 512 ]
     ```

   * Out-of-bounds indexing returns `-1:ROLLBACK` rather than crashing the kernel.

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
3. **Import Injection:** Injects missing standard library imports (`Core.Arithmetic`, `Core.Logic`, `Core.Vectors`, `Core.Memory`) into the Opening block.

---

### AI Cortex, `AIMemory` & Biblical World Model

OmniCode incorporates a native cognitive model grounded in invariant moral law rather than ungrounded neural hallucinations:

```bash
cargo run -- ai "Initialize storehouse with 256 cells and allocate 64 to scratchpad"
```

* **`AIMemory` ([`src/ai/model/memory.rs`](file:///home/professorseanex/OmniCode/src/ai/model/memory.rs)):** Maintains associative concept graphs and active thoughts formulated directly in OmniCode syntax.
* **`BiblicalLexicon` ([`src/ai/lexicon.rs`](file:///home/professorseanex/OmniCode/src/ai/lexicon.rs)):** 100% offline knowledge base of public domain scriptures (KJV 1611 & WEB) across 6 ontological domains: *Cosmology*, *LivingWorld*, *HumanCondition*, *MoralVirtue*, *MoralVice*, *CausalLaw*.
* **`manifest.omni` ([`manifest.omni`](file:///home/professorseanex/OmniCode/manifest.omni)):** Invariant ethical bedrock defining the 7 Invariant Moral Laws (Truth, Life, Justice, Stewardship, Humility, Discernment, Void Trap).
* **Episodic Scrolls (`runtime/scrolls/`):** Dual-logging pipeline generating executable `.scroll` files and microsecond `.log` telemetry digested continuously by [`AILearner`](file:///home/professorseanex/OmniCode/src/ai/model/learner.rs).

---

## 3. CLI Command Reference & Verification Suite

### Complete Command Reference

Implemented in [`src/main.rs`](file:///home/professorseanex/OmniCode/src/main.rs):

```text
USAGE:
  omnicode                    Start the interactive terminal shell (REPL)
  omnicode repl               Start the interactive terminal shell (REPL)
  omnicode run <file>         Execute an OmniCode source (.omni), scroll (.scroll), or tape (.tape)
  omnicode lint <file>        Validate 4-Block architecture compliance (.omni / .scroll)
  omnicode restore <file>     Auto-heal syntax, boundary, and block anomalies (--write)
  omnicode ai "<prompt>"      Transduce prompt with AI Cortex, self-heal, execute & inscribe logs
  omnicode scrolls            Inspect episodic memory bank and learning statistics
  omnicode manifest           Display the canonical Covenant Manifest and moral axioms
  omnicode lexicon [term]     Inspect the 100% offline Biblical world lexicon
  omnicode gui [--port 8080]  Launch the local web GUI Studio & Terminal interface
  omnicode desktop            Launch the native desktop GUI Studio & Terminal shell
  omnicode scaffold <Name>    Generate canonical 4-Block template [--type omni|scroll]
  omnicode filetypes          Display all formal OmniCode file types and MIME definitions
  omnicode demo               Run the built-in ontological verification suite
  omnicode --help, -h         Show help information
  omnicode --version, -v      Display engine version
```

---

### Verification & 3-Tier Test Suite

OmniCode maintains a 100% passing test suite with **48 comprehensive automated tests**:

```bash
cargo test
```

#### Test Suite Breakdown

* **16 Core Unit Tests** ([`src/lib.rs`](file:///home/professorseanex/OmniCode/src/lib.rs)):
  * Boundary ranking hierarchy, cell 8-bit/5-trit round trips, out-of-band markers.
  * File type detection, scaffold generation, 8 parts of speech coverage.
  * Directional vectors shorthand, vessel creation, storehouse partitioning, scheduler lifecycle.
* **10 Component Isolation Tests** ([`tests/unit/main.rs`](file:///home/professorseanex/OmniCode/tests/unit/main.rs)):
  * Balanced ternary symmetry, cognitive notes formatting, cardinal directionals and clauses.
  * AI memory thought formulation, SVO arithmetic keywords.
* **14 Subsystem Integration Tests** ([`tests/integrated/main.rs`](file:///home/professorseanex/OmniCode/tests/integrated/main.rs)):
  * Asymmetric causality boundaries, failpoint cyclic thresholds, delimiter leaks.
  * Immutable mutation detection, 4-Block linter validation, NULL trap absorption.
  * Caveat veto rollbacks, `lib/core/vectors` and `lib/core/memory` execution, Restorator healing.
* **8 Functional & OS Lifecycle Tests** ([`tests/functional/main.rs`](file:///home/professorseanex/OmniCode/tests/functional/main.rs)):
  * Genesis boot tape execution (`lib/kernel/boot.tape`).
  * Manifest covenant process tape execution (`manifest.tape`).
  * Living terminal session boot & tape mounting commands (`mount`, `step`, `mem`, `tracks`).
  * REPL environment persistence, end-to-end AI Cortex pipeline, AILearner dual log digestion.

---

## 4. Project Repository Layout

```text
OmniCode/
├── .env.example                       # Canonical environment configuration template
├── .gitignore                         # Comprehensive 4-block exclusion manifest
├── .vscode/                           # IDE workspace configuration (associations: omni, scroll, tape)
├── Cargo.toml                         # 4-Block Rust package manifest (Edition 2024, Rust 1.85+)
├── Cargo.lock                         # Exact dependency resolution lockfile
├── README.md                          # Comprehensive architectural and orientation manual
├── manifest.omni                      # Low-level declarative covenant manifest & invariant moral laws
├── manifest.tape                      # High-level process tape executing covenant with 8-slot sentences
│
├── apps/                              # User-facing applications and developer tools
│   ├── editors/                       # Official editor integrations
│   │   └── vscode/                    # OmniCode VS Code extension (v0.1.2)
│   └── gui/                           # Web Studio HUD & Native Desktop interfaces
│       ├── desktop_app.py             # Native desktop GUI application
│       ├── terminal.html / .js        # Interactive web terminal HUD
│       ├── studio.html / app.js       # Web IDE studio interface
│       └── style.css                  # Unified design tokens & dark cyber aesthetic
│
├── <TRACK 0> : {Silicon & Bare-Metal Kernel}/
│   ├── kernel/                        # Bare-metal inception, machine code & firmware EFI
│   │   ├── bios/                      # 16-Bit Real Mode MBR (512-byte boot sector)
│   │   ├── uefi/                      # 64-Bit Native UEFI PE32+ Application (BOOTX64.EFI)
│   │   ├── multiboot/                 # Direct 32-bit ELF Kernel for hypervisor handoff
│   │   └── build_all_kernels.sh       # Unified multi-architecture build automation
│   └── TRACK.md                       # Track 0 specification manifest
│
├── <TRACK 1> : {Host Engine & Virtual Machine}/
│   ├── src/                           # Rust Core Engine & Operating System Runtime
│   │   ├── main.rs                    # CLI entrypoint dispatcher
│   │   ├── lib.rs                     # Core library exports and high-level executor
│   │   ├── ai/                        # AI Cortex, AIMemory, BiblicalLexicon, AILearner
│   │   ├── ast/                       # Strongly-typed AST with preserved Asides
│   │   ├── cli/                       # REPL, HTTP GUI Server, commands
│   │   ├── core/                      # Boundaries, Cell, FileType, Linguistics, Operators, Ternary, Vessel, Reel
│   │   ├── diagnostics/               # Debugger, Linter, Logger, Restorator, Hover Engine
│   │   ├── parser/                    # Directional Lexer and recursive-descent Parser
│   │   ├── runtime/                   # Assembler, Transactional Environment and QuadState Evaluator
│   │   └── terminal/                  # Multi-track OmniScheduler, Living TerminalSession, Studio Renderer
│   ├── tests/                         # Automated Test Harness (68 Passing Tests)
│   │   ├── unit/main.rs               # Component isolation unit tests
│   │   ├── integrated/main.rs         # Subsystem integration and failpoint tests
│   │   └── functional/main.rs         # End-to-end OS boot, QEMU bare-metal, REPL, and AI lifecycle tests
│   └── TRACK.md                       # Track 1 specification manifest
│
├── <TRACK 2> : {OmniCode Language & Standard Library}/
│   ├── lib/                           # Standard Library & OS Kernel Ribbons
│   │   ├── core/                      # Arithmetic, logic, vectors, memory vessels, cell tapes
│   │   └── kernel/                    # Genesis OS boot covenant ribbons (boot.omni, boot.tape)
│   ├── lexicon/                       # Canonical Language Lexicon (8 Parts of Speech)
│   │   ├── 01_nouns/ .. 08_interjections/ # Modular semantic vocabulary ribbons
│   │   └── operators/                 # Operator definitions (boot_kernel, casting, polarity)
│   └── TRACK.md                       # Track 2 specification manifest
│
├── <TRACK 3> : {Applications & OmniStudio Interfaces}/
│   ├── apps/                          # User-facing tooling and development environments
│   │   ├── gui/                       # OmniStudio cinematic HUD & web interface
│   │   └── editors/vscode/            # Visual Studio Code syntax & language tools
│   ├── examples/                      # Canonical demonstration tapes and verification reels
│   └── TRACK.md                       # Track 3 specification manifest
│
├── <TRACK 4> : {Variable Storage & Episodic Memory}/
│   ├── storage/                       # The AI model's persistent brain storage
│   │   ├── data/                      # Permanent documentation vaults and datasets
│   │   └── scrolls/                   # Inscribed neural-symbolic scrolls (.scroll & .log)
│   └── TRACK.md                       # Track 4 specification manifest
│
├── <TRACK 5> : {Infrastructure, Operations & Archives}/
│   ├── infra/                         # Operations, containerization, and release packaging
│   │   ├── dist/                      # Standalone distribution archives (.tar.gz)
│   │   ├── dns/                       # Dynamic DNS synchronization scripts
│   │   ├── docker/                    # Multi-architecture Dockerfile and docker-compose
│   │   ├── scripts/                   # Toolchain packaging and deployment utilities
│   │   └── systemd/                   # Production daemon service definitions
│   ├── docs/                          # Comprehensive Foundational Documentation
│   └── TRACK.md                       # Track 5 specification manifest
│
├── manifest.omni                      # Root Genesis System Covenant
└── manifest.tape                      # Root Genesis Process Boot Ribbon
```

---

### Formal Invariant Seal

OmniCode source definitions, language physics, operating system ribbons, and diagnostic boundaries are verified and sealed under the Universal 4-Block Architecture. All transformations are deterministic, bounded, and self-verifying.

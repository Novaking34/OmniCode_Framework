# OmniCode

> **A Dual-Readable Ontological Programming Language & Bare-Metal OS Engineered for Unambiguous Human Comprehension and Machine Determinism.**

---

### Manifest Metadata & Covenant Identity

| Attribute | Specification |
| :--- | :--- |
| **Engine Version** | `0.1.0` |
| **Organization** | CreativeWorkzStudio LLC |
| **Repository** | [https://github.com/Novaking34/OmniCode_Framework](https://github.com/Novaking34/OmniCode_Framework) |
| **Core Runtime** | [Rust Engine (`edition = "2024"`, `rust-version = "1.85+"`)](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/lib.rs) |
| **Execution Truth Space** | Balanced Ternary Quad-State ($\mathbb{T}_4 = \{ -1, 0, +1, \text{NULL} \}$) ([`QuadState`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/core/ternary/quad_state.rs), [Specification](file:///home/professorseanex/OmniCode/docs/02_logic_and_algebra/01_balanced_ternary_quad_state.md)) |
| **Structural Paradigm** | Implicit 4-Block Lifecycle (`Metadata` $\to$ `Opening` $\to$ `Body` $\to$ `Closing`) ([`OmniLinter`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/diagnostics/linter/core.rs), [Specification](file:///home/professorseanex/OmniCode/docs/03_architecture/03_four_block_architecture.md)) |
| **Morphological Physics** | 6 Boundary Types (`[]`, `<>`, `{}`, `{]`, `[}`, `()`) ([`BoundaryType`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/core/boundaries.rs), [Specification](file:///home/professorseanex/OmniCode/docs/01_foundations/02_boundary_types_and_asides.md)) |
| **Universal Linguistics** | 8 Parts of Speech Matrix & 8-Slot Canonical Sentences ([`linguistics.rs`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/core/linguistics.rs), [Specification](file:///home/professorseanex/OmniCode/docs/01_foundations/03_eight_parts_of_speech_matrix.md)) |
| **Physical Memory Model** | 8-Bit $\longleftrightarrow$ 5-Trit Bijective Codec & Bounded Storehouses with `v--` Partitioning ([`cell.rs`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/core/cell.rs), [`vessel.rs`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/core/vessel.rs)) |
| **Operating System Genesis** | Multi-Track Process Tapes (`Roll` $\to$ `Tape` $\to$ `Track` 0–5) ([`OmniScheduler`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/terminal/scheduler.rs), [`boot.tape`](file:///home/professorseanex/OmniCode/<TRACK%202>%20:%20{OmniCode%20Language%20&%20Standard%20Library}/lib/kernel/boot.tape)) |
| **Cognitive AI Framework** | Neural-Symbolic AI Cortex, `AIMemory`, Offline `BiblicalLexicon` ([`AICortex`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/ai/model/cortex.rs), [`manifest.omni`](file:///home/professorseanex/OmniCode/manifest.omni)) |
| **Diagnostics & Healing** | Autonomous Restorator, OmniDebugger Rollbacks, Sensor Logger ([`Restorator`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/diagnostics/restorator.rs)) |
| **Interfaces & Studio** | Built-in Zero-Dependency HTTP HUD, Web Terminal, Desktop GUI, VS Code Extension ([`apps/gui/`](file:///home/professorseanex/OmniCode/<TRACK%203>%20:%20{Applications%20&%20OmniStudio%20Interfaces}/apps/gui)) |
| **Automated Verification** | **269 / 269 Passing Tests** across 7 test suites (100% Pass Rate) + 5 Bare-Metal QEMU Boot Audits + C Transpilation Testbeds |
| **Author / Lead** | ProfessorSeanEX / CreativeWorkzStudio LLC |
| **License** | MIT OR Apache-2.0 |

---

## Part I: Foundations, Identity & Ontological Bedrock

OmniCode is an ontological programming language and bare-metal operating system engineered to eliminate the semantic gap between human comprehension, artificial intelligence deliberation, and silicon hardware execution.

Modern computing has long suffered from an artificial divide:

1. **Low-Level Code (C, Assembly, Rust):** Extreme physical control and speed, but difficult for non-specialists to read and opaque to high-level semantic reasoning.
2. **High-Level Code (Python, JavaScript):** Human-approachable syntax, but burdened by heavy runtime interpreters, unpredictable garbage collection pauses, and fragile boolean error-handling.
3. **Artificial Intelligence Models:** Capable of probabilistic reasoning, but detached from deterministic hardware boundaries, frequently hallucinating unverified operations.

OmniCode reconciles this spectrum into a single unified architecture.

```
       HUMAN COGNITION
   (5Ws and H • Natural Grammar)
              ▲
              │   Dual-Readable Transduction
              ▼
    OMNICODE REEL & TAPE
 (4-Block Lifecycle • Quad-State T4)
              ▲
              │   Deterministic Lowering & FFI
              ▼
      SILICON HARDWARE
 (CPU Registers • Balanced Ternary • MMIO)
```

---

### Perspectives Across the Spectrum

#### For the Non-Programmer: Natural Physics as Code

You do not need to memorize arbitrary programming tropes or arcane punctuation to understand OmniCode.

- **The World Has Boundaries:** When you store something, you place it inside a container (`[...]`). When an idea is fluid and adapting, it exists in open motion (`<...>`).
- **The World Has Natural Balance:** Instead of forcing every reality into a brittle binary `true` or `false`, OmniCode uses a three-way balance scale: **Proceed** (`+1`), **Await** (`0`), and **Rollback** (`-1`), with a protective **Halt** (`NULL`). If something goes wrong, the system does not crash or throw an opaque error—it simply steps back to safety like a physical ratchet.
- **Your Notes Matter to the Machine:** In conventional coding, comments are ignored by the computer. In OmniCode, parenthetical thoughts `(...)` are first-class asides. The machine reads your intent and ensures the code actually honors what you intended.

#### For the Software Engineer: Zero-Overhead Determinism

- **No Garbage Collection Pauses:** Memory management is structural. Bounded memory vessels enforce strict capacity limits at creation, with downward sub-partitioning (`v--`) guaranteeing zero memory fragmentation and zero runtime allocation thrash.
- **Transactional State Physics:** By treating execution as a trajectory across balanced ternary quad-state ($\mathbb{T}_4$), every operation is reversible. A state transition that fails invariant checks triggers an automatic snapshot rollback via the built-in `OmniDebugger`, eliminating undefined states.
- **Morphological Brackets as Semantic Operators:** Brackets are not decorative syntax; they govern causality and access rank. Rigid brackets `[...]` denote immutable memory cells; fluid brackets `<...>` denote type invariants; asymmetric brackets `{...]` and `[...}` act as accumulators and generators.
- **Native Host in Rust 2024:** Written in modern Rust with zero runtime bloat (using only `rustyline` for terminal line editing), compiling to native machine code with LTO and zero-cost abstractions.

#### For the AI Agent: A Native Mathematical & Moral Substrate

- **Grounding Against Hallucination:** Cognitive agents do not output loose, ungrounded tokens. Thoughts and actions are inscribed in the universal 4-block format (`Metadata` $\to$ `Opening` $\to$ `Body` $\to$ `Closing`).
- **Epistemic Invariant Laws:** Bound by seven unalterable Rank 1 Invariants defined in [`manifest.omni`](file:///home/professorseanex/OmniCode/manifest.omni) (Truth, Life, Justice, Stewardship, Humility, Discernment, Void Trap).
- **Episodic Learning Loops:** The AI logs its experiences into dual streams: executable cognitive scrolls (`.scroll`) and sensor logs (`.log`), which are digested continuously by the internal learning engine to evolve behavior deterministically.

---

### The Twelve Invariant Pillars

1. **Formal Ontology & Set Theory:** Clean separation of an entity's intrinsic essence (Denotation) from its environmental context (Connotation). Specified in [Document 01: Ontological Principles](file:///home/professorseanex/OmniCode/docs/01_foundations/01_ontological_principles.md).
2. **Balanced Ternary Quad-State Engine:** Execution physics governed by $\mathbb{T}_4 = \{ -1, 0, +1, \text{NULL} \}$ (`Rollback`, `Await`, `Proceed`, `Halt`), delivering native transactional state machines without unchecked panics.
3. **Morphological Boundary Physics:** Brackets are active semantic operators encoding causality direction, containment hierarchy, and physical mutability: `[...]`, `<...>`, `{...}`, `{...]`, `[...}`, `(...)`.
4. **First-Class Asides `(...)`:** Intent and rationale are preserved in the AST ([`ASTNode::Aside`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/ast/nodes.rs)) as machine-readable invariants and AI cognitive traces.
5. **Universal 4-Block Architecture:** Every source file, configuration, and episodic scroll adheres strictly to a 4-block lifecycle: `Metadata` $\to$ `Opening` $\to$ `Body` $\to$ `Closing`.
6. **Universal Linguistics & 8 Parts of Speech:** Nouns are defined; verbs are symbolized. Every statement maps cleanly into an 8-Slot Canonical Sentence and the 6 Cardinal Classification Clauses (`WHO`, `WHAT`, `WHEN`, `WHERE`, `WHY`, `HOW`).
7. **Dense Memory Cells & Hardware Transducer:** Bijective hardware codec mapping 8-bit bytes ($2^8 = 256$) to 5-trit balanced ternary ($3^5 = 243$ data states $[-121 \dots +121]$ + 13 out-of-band control states).
8. **Bounded Memory Vessels & Storehouses:** Contiguous memory blocks with invariant bounds and dynamic `v--` downward partitioning, delivering safe indexing and zero-cost allocation.
9. **Physical Computing Hierarchy & Process Tapes:** Physical execution hierarchy: `Roll` $\to$ `Tape` $\to$ `Track` (cooperative priority scheduling across Tracks 0 through 5).
10. **Cognitive Loop & Episodic Learning:** Grounded in an offline `BiblicalLexicon` (6 ontological domains) and invariant `manifest.omni` covenant. The internal AI formulates thoughts directly in native OmniCode.
11. **Autonomous Self-Healing (Restorator):** Built-in diagnostic immune system that repairs boundary mismatches, unclosed delimiters, missing standard imports, and structural anomalies on the fly.
12. **Rust Core Foundation:** Zero runtime buckling, compile-time memory safety without a garbage collector, and seamless C/LLVM ABI interoperability.

---

## Part II: Entering the Universe: Installation, Tooling & Onboarding

### System Prerequisites

- **Rust Compiler:** Version `1.85+` ([`rustc`](https://www.rust-lang.org/), [`cargo`](file:///home/professorseanex/OmniCode/Cargo.toml))
- **C Compiler & Toolchain:** `gcc` or `clang` supporting C11, plus `make` (for Track 2 C transpilation and test suites)
- **Terminal Shell:** POSIX Bash or equivalent Linux/macOS shell
- **Virtualization & Emulation (Optional for Bare-Metal Kernel):** `qemu-system-x86_64`, `nasm`, `xorriso`, `lld`

---

### Toolchain Verification

OmniCode includes an automated pre-flight diagnostic script to verify your local build environment and compilation tools:

```bash
./<TRACK\ 5>\ \:\ \{Infrastructure\,\ Operations\ \&\ Archives\}/infra/scripts/check_toolchain.sh
```

---

### Quick Start in 60 Seconds

Clone the repository and build the native engine:

```bash
git clone https://github.com/Novaking34/OmniCode_Framework.git
cd OmniCode_Framework
cargo build --release
```

#### 1. Launch the Interactive REPL Shell

```bash
cargo run -- repl
```

#### 2. Execute a Canonical Source Reel (`.omni`)

```bash
cargo run -- run examples/canonical_notation.omni
```

#### 3. Execute the Genesis OS Boot Process Tape (`.tape`)

```bash
cargo run -- run "<TRACK 2> : {OmniCode Language & Standard Library}/lib/kernel/boot.tape"
```

#### 4. Validate Structural 4-Block Compliance

```bash
cargo run -- lint manifest.omni
```

---

### The Interactive Terminal Environment (REPL)

The interactive REPL ([`src/cli/repl.rs`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/cli/repl.rs)) provides real-time balanced ternary evaluation, multi-line bracket buffering, and transactional state inspection:

| REPL Command | Action | Implementation |
| :--- | :--- | :--- |
| `:env` | Display all currently bound identifiers, types, and values | [`Environment`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/runtime/environment.rs) |
| `:ast <expr>` | Tokenize and print the Abstract Syntax Tree for an expression | [`Parser`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/parser/parser.rs) |
| `:failpoints` | Inspect the session's diagnostic failpoint telemetry log | [`DiagnosticLogger`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/diagnostics/logger/core.rs) |
| `:load <file>` | Parse and evaluate an external `.omni`, `.scroll`, or `.tape` | [`Evaluator`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/runtime/evaluator.rs) |
| `:reset` | Reset the global environment to a pristine initial state | [`Environment::new`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/runtime/environment.rs) |
| `:help` | Show interactive REPL command help | [`run_repl`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/cli/repl.rs) |
| `:exit`, `:quit` | Exit the REPL session safely | [`cli/mod.rs`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/cli/mod.rs) |

---

### Web Studio HUD & Desktop Interface

OmniCode includes a built-in, zero-dependency HTTP server that hosts the Web Studio HUD, Interactive Terminal Gateway, and live telemetry inspectors:

```bash
# Launch the Web Studio server on port 8080
cargo run -- gui --port 8080
```

Once running, navigate to `http://localhost:8080` to access:

- `/terminal` — Interactive browser-based Terminal HUD.
- `/studio` — 4-Block code editor and visual runtime inspector.
- `/specs` — Interactive architectural documentation viewer.
- `/model` — AI Cortex neural-symbolic deliberation inspector.
- `/lexicon` — 100% offline Biblical world lexicon explorer.
- `/scrolls` — Episodic memory stream visualizer.
- `/api/health` — REST health and telemetry probe.

Alternatively, launch the native desktop application:

```bash
python3 "<TRACK 3> : {Applications & OmniStudio Interfaces}/apps/gui/desktop_app.py"
```

---

### Visual Studio Code IDE Integration

The official OmniCode VS Code extension provides TextMate syntax highlighting for `.omni`, `.scroll`, and `.tape` files, bracket-pair colorization, and canonical 4-block snippets.

Install the extension:

```bash
code --install-extension "<TRACK 3> : {Applications & OmniStudio Interfaces}/apps/editors/vscode/omnicode-language-0.1.2.vsix"
```

The workspace is pre-configured with [`.vscode/settings.json`](file:///home/professorseanex/OmniCode/.vscode/settings.json) implementing the 4-block configuration standard: enforcing UTF-8, POSIX LF line endings, 80-column rulers, and tab preservation for Makefiles.

---

## Part III: The Engine of Reality: Language Physics & Cognitive Architecture

### The Universal 4-Block Architecture

Every program, configuration file, and episodic scroll in OmniCode obeys the 4-Block lifecycle. This structure provides structural containment, ensuring that no state can execute without valid metadata, setup preconditions, and closing integrity verifications.

```omni
# ==============================================================================
# BLOCK 1: METADATA BLOCK
# ( Module Identity, Version, Invariant Types, and Intent Aside )
# ==============================================================================
Module < ArithmeticEngine > ( Core arithmetic operational module ) {
    version: < "1.0.0" >,
    system: < OmniCode.Core >
}

# ==============================================================================
# BLOCK 2: OPENING BLOCK
# ( Context Setup, Preconditions, Scoped Universes, and State Initialization )
# ==============================================================================
Opening {
    import: [ Core.Arithmetic :: Core.Logic :: Core.Memory ],
    preconditions: [ +1:PROCEED ]
}
state [ +1 ] ( Initial operational state )

# ==============================================================================
# BLOCK 3: BODY BLOCK
# ( Core Transformations, Causality Vectors, Logic, and Caveat Pipelines )
# ==============================================================================
Body [
    accumulator [ 0 ]
    accumulator --> next_val [ 12 ]
    next_val ! [ accumulator = 0 ] ( Caveat check: Proceed only if accumulator was empty )
]

# ==============================================================================
# BLOCK 4: CLOSING BLOCK
# ( Postconditions, Invariant Integrity Proofs, Teardown, and Commit Status )
# ==============================================================================
Closing {
    verify: [ next_val = 12 ],
    commit_status: [ +1:PROCEED ] ( Invariant verified; commit transaction )
}
```

---

### Morphological Boundary Delimiters

In OmniCode, brackets are physical operators that govern access mutability, ranking hierarchy, and causal direction. They are implemented in [`boundaries.rs`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/core/boundaries.rs):

| Delimiter | Boundary Type | Semantic Role | Behavior & Physical Mutability |
| :---: | :--- | :--- | :--- |
| `[...]` | **Rigid / Fixed** | Denotation / Concrete | Fixed physical memory cell, immutable boundary, concrete datum. |
| `<...>` | **Fluid / Dynamic** | Process / Type Invariant | Mutable state, transformation vector, type classification. |
| `{...}` | **Universal / Omni** | Scoped Universe / Holism | Enclosing set domain, parent scope, environmental boundary. |
| `{...]` | **Accumulator / Absorber** | Fluid In $\to$ Fixed Out | Consumes external fluid stream and condenses into a fixed datum. |
| `[...}` | **Generator / Source** | Fixed In $\to$ Fluid Out | Emanates from a fixed constant and radiates a fluid stream. |
| `(...)` | **First-Class Aside** | Intent / Cognitive Trace | Non-interfering metadata, machine-readable reasoning, and formal proofs. |

---

### Balanced Ternary Quad-State Physics ($\mathbb{T}_4$)

OmniCode replaces brittle binary logic with balanced ternary quad-state physics ($\mathbb{T}_4 = \{ -1, 0, +1, \text{NULL} \}$), eliminating runtime exceptions:

- **`+1:PROCEED`:** Affirmative forward execution; condition satisfied; transaction committed.
- **`0:AWAIT`:** Indeterminate, suspended, or awaiting external input or consensus.
- **`-1:ROLLBACK`:** Contra/veto state. Automatically rolls back the environment to the previous stable snapshot via [`OmniDebugger`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/diagnostics/debugger.rs).
- **`NULL:HALT`:** Terminal void. Absorbs subsequent transformations and halts execution safely.

#### Truth Tables

| Operation | Input A | Input B | Output | Semantic Result |
| :--- | :---: | :---: | :---: | :--- |
| **Negation (`--`)** | `+1` | — | `-1` | Logical inversion / Rollback trigger |
| **Negation (`--`)** | `0` | — | `0` | Neutral balance preserved |
| **Negation (`--`)** | `-1` | — | `+1` | Contra-inversion to Proceed |
| **Consensus Conjunction (`::`)** | `+1` | `+1` | `+1` | Full agreement |
| **Consensus Conjunction (`::`)** | `+1` | `-1` | `-1` | Single veto triggers Rollback |
| **Consensus Conjunction (`::`)** | `+1` | `0` | `0` | Indeterminate awaiting resolution |

---

### Universal Linguistics & The 8 Parts of Speech Matrix

OmniCode is structurally isomorphic to universal natural language:

$$\text{“}\underbrace{\text{Oh!}}_{\text{Slot 1: Interjection}}\quad \underbrace{\text{But}}_{\text{Slot 2: Conjunction}}\quad \underbrace{\text{my}}_{\text{Slot 3: Pronoun}}\quad \underbrace{\text{old}}_{\text{Slot 4: Adjective}}\quad \underbrace{\text{man}}_{\text{Slot 5: Noun}}\quad \underbrace{\text{walked}}_{\text{Slot 6: Verb}}\quad \underbrace{\text{straight}}_{\text{Slot 7: Adverb}}\quad \underbrace{\text{ahead}}_{\text{Slot 8: Preposition}}\text{.”}$$

- **Nouns are Defined:** Entities, storage containers, measures, types (`Vessel`, `Storehouse`, `Cell`).
- **Verbs are Symbolized:** Directional causal vectors (`-->`, `<--`, `v--`, `^--`, `=`).
- **The 6 Cardinal Classification Clauses (5Ws and H):**
  - `WHO` — Security Principal / Caller (`WHO OmniCode_Kernel`)
  - `WHAT` — Target Entity / Vessel Payload (`WHAT SystemGenesis`)
  - `WHEN` — Lifecycle Phase / Trigger (`WHEN PowerOn`)
  - `WHERE` — Spatial Site / Storehouse Address (`WHERE ram`)
  - `WHY` — Intent Aside / Covenant Anchor (`WHY ( Initialize kernel )`)
  - `HOW` — Quad-State Outcome & Modulation (`HOW +1:PROCEED`)

---

### Physical Computing Hierarchy: $\text{Roll} \to \text{Tape} \to \text{Track}$

OmniCode structures execution across three physical scales:
$$\text{Roll (Atom)} \longrightarrow \text{Tape (Process Ribbon)} \longrightarrow \text{Track (Execution Channel)}$$

- **Roll (`Roll`):** An atomic state mutation or expression.
- **Tape (`Tape` / `.tape`):** A sequential process ribbon led by a **Tape Head** covenant and an ordered stream of subordinate rolls.
- **Track (`Track`):** Six distinct execution planes managed by [`OmniScheduler`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/terminal/scheduler.rs):
  - **Track 0 `[KERNEL]`:** Bare-metal silicon inception, EFI firmware, hardware registers.
  - **Track 1 `[ENGINE]`:** Host virtual machine, AST evaluator, diagnostics, CLI dispatcher.
  - **Track 2 `[LEXICON]`:** Canonical language standard library, types, C & Lua transpilation backbones.
  - **Track 3 `[STUDIO]`:** OmniStudio HUD, web interfaces, terminal gateways, editor extensions.
  - **Track 4 `[MEMORY]`:** Episodic memory storehouse, `.scroll` archives, neural weights.
  - **Track 5 `[OPS]`:** Infrastructure, containerization, toolchain packaging, documentation.

---

### Dense Memory Transduction & Vessel Partitioning

1. **Bijective Hardware Cell Transducer ([`cell.rs`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/core/cell.rs)):**
   - Maps 8-bit bytes ($2^8 = 256$) to 5 balanced trits ($3^5 = 243$ values $[-121 \dots +121]$).
   - Allocates 13 out-of-band hardware control states: `243 = NULL:HALT`, `244 = 0:AWAIT`, `245 = -1:ROLLBACK`, `246 = +1:PROCEED`, `247..255 = Hardware Traps`.
2. **Storehouses & `v--` Sub-Partitioning ([`vessel.rs`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/core/vessel.rs)):**
   - Carves child workspaces with strict invariant boundaries:

     ```omni
     Root_RAM [ 4096 ]
     Root_RAM v-- Kernel_Partition [ 1024 ]
     Root_RAM v-- Terminal_Partition [ 512 ]
     ```

   - Out-of-bounds indexing returns `-1:ROLLBACK` rather than triggering undefined memory corruption.

---

### Autonomous Self-Healing Engine (`Restorator`)

The [`Restorator`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/diagnostics/restorator.rs) serves as the system's autonomic immune system:

```bash
cargo run -- restore path/to/file.omni --write
```

- **Boundary Auto-Balancing:** Detects unclosed delimiters (`[`, `<`, `{`) and restores them according to morphological rank.

- **Structural Scaffolding:** Automatically wraps loose statements into canonical 4-Block architecture.
- **Import Injection:** Automatically provides missing standard library modules in the Opening block.

---

### Cognitive AI Cortex & Covenant Alignment

OmniCode incorporates a native cognitive model grounded in invariant ethical law rather than ungrounded neural hallucinations:

```bash
cargo run -- ai "Initialize storehouse with 256 cells and allocate 64 to scratchpad"
```

- **`AIMemory`:** Maintains associative concept graphs and active thoughts formulated directly in OmniCode syntax.

- **`BiblicalLexicon`:** 100% offline knowledge base of public domain scriptures (KJV 1611 & WEB) across 6 ontological domains: *Cosmology*, *LivingWorld*, *HumanCondition*, *MoralVirtue*, *MoralVice*, *CausalLaw*.
- **`manifest.omni`:** Invariant ethical bedrock defining the 7 Invariant Moral Laws (Truth, Life, Justice, Stewardship, Humility, Discernment, Void Trap).
- **Episodic Scrolls:** Dual-logging pipeline generating executable `.scroll` files and microsecond `.log` telemetry digested continuously by [`AILearner`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/ai/model/learner.rs).

---

## Part IV: Verification, Ecosystem Layout & Formal Invariant Seals

### Complete CLI Command Reference

Implemented in [`src/main.rs`](file:///home/professorseanex/OmniCode/<TRACK%201>%20:%20{Host%20Engine%20&%20Virtual%20Machine}/src/main.rs):

```text
USAGE:
  omnicode                    Start the interactive terminal shell (REPL)
  omnicode repl               Start the interactive terminal shell (REPL)
  omnicode run <file>         Execute an OmniCode source (.omni), scroll (.scroll), or tape (.tape)
  omnicode echo <char|msg>    Echo character or message through native balanced ternary I/O transducer
  omnicode assemble <file>    Assemble an OmniCode process tape into concrete memory layout (.tape)
  omnicode studio <file.tape> Project process tape in the cinematic HUD terminal projector
  omnicode project <file.tape>Project process tape in the cinematic HUD terminal projector
  omnicode boot [--uefi|--multiboot|--bios] [--gui] Boot bare-metal OS in QEMU
  omnicode lint <file>        Validate 4-Block architecture compliance (.omni / .scroll)
  omnicode hover <file> <l> <c> Inspect cross-scale 3-tier hovercard for symbol at position
  omnicode doc <symbol>       Query architectural docstring, invariants, and machine intent
  omnicode restore <file>     Auto-heal syntax, boundary, and block anomalies
  omnicode ai "<prompt>"      Transduce prompt with AI Cortex, self-heal & inscribe
  omnicode interact           Launch live CPI-SI cognitive discourse & learning shell
  omnicode train              Train balanced-ternary brain on ideal reels and logs
  omnicode noise [stress]     Simulate noise degradation breakdown ladder
  omnicode query <session>    Query an episodic memory scroll in O(1) time
  omnicode diagnose <session> Diagnose failpoints and view immune repairs
  omnicode rotate [--cap N]   Enforce storehouse capacity bounds (Occupancy <= Capacity)
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

### Verification & Comprehensive Test Harness

OmniCode maintains a **100% pass rate across 269 automated tests** spanning seven specialized harnesses:

```bash
cargo test
```

#### Test Suite Breakdown

| Harness | Scope & Verification Target | Status | Passing Tests |
| :--- | :--- | :---: | :---: |
| **`lib` Unit Tests** | Core boundaries, ternary algebra, cell codec, vector dispatch, lexical parsing, scheduler lifecycle | `PASS` | **210** |
| **`unit_tests`** | Component isolation, balanced ternary symmetry, cognitive notes, cardinal clauses | `PASS` | **16** |
| **`integrated_tests`** | Subsystem integration, failpoints, delimiter leaks, caveat rollbacks, Restorator healing | `PASS` | **14** |
| **`functional_tests`** | End-to-end OS boot, Genesis boot tape, REPL simulation, AI Cortex pipeline | `PASS` | **12** |
| **`assemble_tripwire_tests`** | Physical assembler lowering, tape-to-machine words, adversarial syntax traps | `PASS` | **7** |
| **`boot_tripwire_tests`** | Bare-metal boot inception, silicon addresses, hypervisor synthesis, telemetry handoff | `PASS` | **5** |
| **`platform_tripwire_tests`** | Platform adapter adversarial faults, display server contracts, subprocess traps | `PASS` | **5** |
| **Total Automated Tests** | **Comprehensive Full-System Verification** | **`100%`** | **`269 / 269`** |

#### Track 0 Bare-Metal Kernel & QEMU Telemetry Audits

All 5 bare-metal kernel targets are compiled, assembled, and verified via headless QEMU execution (`bash "<TRACK 0> : {Silicon & Bare-Metal Kernel}/kernel/build_all_kernels.sh"`):

- **Standalone BIOS MBR (State 0 Safe Mode):** 512B Sector 0 attestation, balanced ternary ALU proof `(+1)+(-1)=0`, COM1 serial broadcast $\to$ **PASS**
- **Dual-State BIOS Image (State +1 Stage 2):** Sector 1 $\to$ Sector 2 handoff, A20 gate, 32-bit Protected Mode, Flight Recorder (`0x18000`), 16-trit CPI-SI cognitive arena $\to$ **PASS**
- **Multiboot 1 Direct Hypervisor Handshake:** 32-bit ELF entry, `0x2BADB002` magic handshake, Flight Recorder (`0x108000`) $\to$ **PASS**
- **Native 64-Bit Long Mode UEFI (PE32+):** Pure x86_64, 4-level paging, OVMF firmware handoff, Flight Recorder (`0x200000`), 32-trit cognitive arena $\to$ **PASS**
- **UEFI Tripwire & Safe Mode Cascade:** Tripwire simulation (`0x0005`), register dump, downward cascade to low-power safe mode watchdog $\to$ **PASS**

#### Track 2 C Transpile Testbeds

In addition to the Rust host engine, the standard library lexicon primitives are verified through native C test harnesses:

- **Universal Base Testbed:** `make -C "<TRACK 2> : {OmniCode Language & Standard Library}/lexicon/01_nouns/ideas/types/universal/transpiled/compiled/c" test` $\to$ **PASS**
- **Character Invariant Testbed:** `make -C "<TRACK 2> : {OmniCode Language & Standard Library}/lexicon/01_nouns/ideas/types/character/transpiled/compiled/c" test` $\to$ **PASS**
- **Number Invariant Testbed:** `make -C "<TRACK 2> : {OmniCode Language & Standard Library}/lexicon/01_nouns/ideas/types/number/transpiled/compiled/c" test` $\to$ **PASS**

---

### Multi-Track Repository Layout

```text
OmniCode_Framework/
├── .env.example                       # Canonical environment configuration template
├── .gitignore                         # Comprehensive 4-block exclusion manifest
├── .vscode/                           # IDE workspace configuration (associations: omni, scroll, tape)
│   └── settings.json                  # Canonical 4-block editor mechanics & multi-track parity
├── Cargo.lock                         # Exact dependency resolution lockfile
├── Cargo.toml                         # 4-Block Rust package manifest (Edition 2024, Rust 1.85+)
├── README.md                          # Comprehensive architectural and orientation manual
├── manifest.omni                      # Declarative covenant manifest & invariant moral laws
├── manifest.tape                      # Process tape executing covenant with 8-slot sentences
├── docs/                              # Comprehensive Foundational Documentation (Volumes 1–5)
├── scripts/                           # Build scripts and documentation export automation
│
├── <TRACK 0> : {Silicon & Bare-Metal Kernel}/
│   ├── kernel/                        # Bare-metal inception, machine code & firmware EFI
│   │   ├── bios/                      # 16-Bit Real Mode MBR (512B) & Stage 2 Protected Mode (2048B)
│   │   ├── uefi/                      # 64-Bit Native UEFI PE32+ Application (BOOTX64.EFI)
│   │   ├── multiboot/                 # 32-Bit Direct ELF Kernel for hypervisor handoff (0x100000)
│   │   ├── images/                    # Centralized boot artifacts, ESP partition & serial audit logs
│   │   └── build_all_kernels.sh       # Unified multi-architecture build & 5-test QEMU verifier
│   ├── BOOT_LADDER_AND_ALTERNATE_STATES.md # 4-Tier Ladder & Alternate States Theorem treatise
│   ├── KERNEL_INCEPTION_POINT.md      # Genesis boot purpose & bare-metal visibility treatise
│   └── TRACK.md                       # Track 0 specification manifest
│
├── <TRACK 1> : {Host Engine & Virtual Machine}/
│   ├── src/                           # Rust Core Engine & Operating System Runtime
│   │   ├── main.rs                    # CLI entrypoint dispatcher
│   │   ├── lib.rs                     # Core library exports and high-level executor
│   │   ├── ai/                        # AI Cortex, AIMemory, BiblicalLexicon, AILearner
│   │   ├── ast/                       # Strongly-typed AST with preserved Asides
│   │   ├── cli/                       # REPL, HTTP GUI Server, commands
│   │   ├── core/                      # Boundaries, Cell, FileType, Linguistics, Operators, Ternary, Vessel
│   │   ├── diagnostics/               # Debugger, Linter, Logger, Restorator, Hover Engine
│   │   ├── parser/                    # Directional Lexer and recursive-descent Parser
│   │   ├── runtime/                   # Assembler, Transactional Environment and QuadState Evaluator
│   │   └── terminal/                  # Multi-track OmniScheduler, Living TerminalSession, Studio Renderer
│   ├── tests/                         # Automated Test Harness (265 Passing Tests across 7 suites)
│   └── TRACK.md                       # Track 1 specification manifest
│
├── <TRACK 2> : {OmniCode Language & Standard Library}/
│   ├── lib/                           # Standard Library & OS Kernel Ribbons
│   │   ├── core/                      # Arithmetic, logic, vectors, memory vessels, cell tapes
│   │   └── kernel/                    # Genesis OS boot covenant ribbons (boot.omni, boot.tape)
│   ├── lexicon/                       # Canonical Language Lexicon (8 Parts of Speech)
│   │   ├── 01_nouns/                  # Modular semantic vocabulary (ideas, types: universal, char, num)
│   │   ├── 02_verbs/ .. 08_interjections/ # Complete grammatical spectrum
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
└── <TRACK 5> : {Infrastructure, Operations & Archives}/
    ├── infra/                         # Operations, containerization, and release packaging
    │   ├── dist/                      # Standalone distribution archives (.tar.gz)
    │   ├── dns/                       # Dynamic DNS synchronization scripts
    │   ├── docker/                    # Multi-architecture Dockerfile and docker-compose
    │   ├── scripts/                   # Toolchain packaging and deployment utilities
    │   └── systemd/                   # Production daemon service definitions
    └── TRACK.md                       # Track 5 specification manifest
```

---

### Formal Invariant Seal

OmniCode source definitions, language physics, operating system ribbons, and diagnostic boundaries are verified and sealed under the Universal 4-Block Architecture. All transformations are deterministic, bounded, and self-verifying.

$$\text{Boundary Established} \quad\longrightarrow\quad \text{State Enclosed} \quad\longrightarrow\quad \text{Truth Verified } (+1:\text{PROCEED})$$

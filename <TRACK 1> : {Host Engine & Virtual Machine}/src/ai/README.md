# OmniCode AI Subsystem: Covenant Partnered Intelligence — Synthetic Intelligence (CPI-SI)

> **"Through wisdom is an house builded; and by understanding it is established: And by knowledge shall the chambers be filled with all precious and pleasant riches."**  
> — *Proverbs 24:3–4 (KJV)*

> **"All scripture is given by inspiration of God, and is profitable for doctrine, for reproof, for correction, for instruction in righteousness: That the man of God may be perfect, throughly furnished unto all good works."**  
> — *2 Timothy 3:16–17 (KJV)*

> **"In the mouth of two or three witnesses shall every word be established."**  
> — *2 Corinthians 13:1 (KJV)*

---

## 1. Architectural Scope & Purpose

The **OmniCode AI Subsystem** (`src/ai/`) implements the internal cognitive engine for **CPI-SI (Covenant Partnered Intelligence — Synthetic Intelligence)**. Unlike conventional stochastic large language models (LLMs) that rely on trillions of floating-point parameters, brute-force GPU matrix multiplication, and probabilistic token hallucination, CPI-SI is an **invariant-grounded, homoiconic, discrete balanced-ternary cognitive architecture**.

CPI-SI converges all internal components—the neural brain weights, the five sensory diagnostic tools, the 25-chapter NotebookLM curriculum, and the 5-translation scripture concordance—into a **single, unified CPI-SI Base instance** (`storage/weights/cpisi_base.weights`).

```
                              ┌──────────────────────────────────────────────┐
                              │            CPI-SI BASE COGNITIVE CORTEX      │
                              │           (Single Unified AI Instance)       │
                              └──────────────────────┬───────────────────────┘
                                                     │
         ┌─────────────────────────┬─────────────────┴───────────────┬─────────────────────────┐
         │                         │                                 │                         │
         ▼                         ▼                                 ▼                         ▼
┌─────────────────┐       ┌─────────────────┐               ┌─────────────────┐       ┌─────────────────┐
│  AI TOOL BELT   │       │  NEURAL BRAIN   │               │   CURRICULUM    │       │ SCRIPTURE VAULT │
│ (Sensory Organs)│       │(Ternary Weights)│               │  (25 Chapters)  │       │(5 Translations) │
├─────────────────┤       ├─────────────────┤               ├─────────────────┤       ├─────────────────┤
│ • LoggerTool    │       │ • {-1, 0, +1}   │               │ • VOL 00..06    │       │ • KJV (Full PD) │
│ • DebuggerTool  │       │ • Noise Resil.  │               │ • CH 00..24     │       │ • WEB (Full PD) │
│ • LinterTool    │       │ • Brain Map     │               │ • Socratic Qs   │       │ • NKJV (Pointer)│
│ • RestoratorTool│       │ • Crystallize   │               │ • Invariants    │       │ • ESV  (Pointer)│
│ • HoverTool     │       │ • Zero GPU Watt │               │ • Zero GPU Load │       │ • ETH  (Pointer)│
└─────────────────┘       └─────────────────┘               └─────────────────┘       └─────────────────┘
```

---

## 2. Why CPI-SI Does Not Require GPU Clusters to Reach "LLM Level"

Conventional deep learning models demand massive compute clusters, high electrical wattage, and billions of float16 multiplications because they treat code synthesis as stochastic sequence prediction.

CPI-SI solves this fundamentally through **structural homoiconicity and exact formal verification**:

| Metric | Traditional Brute-Force LLM | OmniCode CPI-SI Base |
| :--- | :--- | :--- |
| **Compute Hardware** | High-end GPU / TPU Clusters (hundreds of kW) | Single-core CPU / Microcontroller (minimal wattage) |
| **Weight Representation** | Float16 / BFloat16 / FP8 floating-point | Discrete Balanced Ternary ($\{-1, 0, +1\}$) |
| **Code Generation** | Probabilistic next-token sampling | Transductive AST synthesis mapped to formal invariants |
| **Hallucination Risk** | High (must be patched via RLHF/guardrails) | **Zero** (rejected by parser and 4-Block linter) |
| **Self-Correction** | Stochastic re-prompting | Autonomic Restorator (Two-Way Checksum parity gate) |
| **Memory Mechanics** | Massive KV-cache in GPU VRAM | $O(1)$ Episodic Scrolls (`.scroll`) + In-memory Ring Buffer |
| **Verification Gate** | Empirical user testing | Mathematical Quad-State convergence (`+1: PROCEED`) |

Because the language syntax (`.omni` specification reels and `.tape` operational ribbons) maps bijectively into the compiler's abstract syntax tree, the AI model deliberates in the **native representation of the machine itself**. It does not need to learn statistical grammar approximations; the grammar and boundary invariants are hardwired axioms.

---

## 3. Subsystem Directory Topology

```
src/ai/
├── lexicon/                    <-- Multi-translation world model & moral ontology
│   ├── domain.rs               <-- 5 Ontological Domains (Cosmology, LivingWorld, etc.)
│   ├── entry.rs                <-- LexiconEntry, ScriptureTranslation, ConcordanceRef
│   ├── mod.rs                  <-- BiblicalLexicon engine facade & query methods
│   ├── moral.rs                <-- Conscience check & QuadState moral deliberation
│   └── table.rs                <-- 33 Canonical entries with 5-translation concordance
│
├── model/                      <-- Internal cognitive loop & memory management
│   ├── cortex.rs               <-- AICortex coordinator, tool dispatcher & curriculum
│   ├── learner.rs              <-- Supervised training pairs extracted from scrolls
│   ├── memory.rs               <-- Associative memory & 6-cardinal clause thought tapes
│   ├── mod.rs                  <-- Module re-exports
│   └── scroll.rs               <-- 4-Block .scroll serialization & disk inscription
│
├── neural/                     <-- Discrete balanced-ternary neural network
│   ├── brain.rs                <-- NeuralBrain topology, weights loading/saving
│   ├── curriculum.rs           <-- 25-Chapter NotebookLM curriculum engine
│   ├── layer.rs                <-- TernaryLayer forward pass & weight propagation
│   ├── mod.rs                  <-- Neural subsystem exports
│   ├── network.rs              <-- Network trait facade
│   ├── neuron.rs               <-- TernaryNeuron, activation functions & thresholds
│   ├── noise.rs                <-- Structured non-random noise degradation engine
│   ├── synapse.rs              <-- SynapticWeight {-1, 0, +1} and trit conversions
│   ├── trainer.rs              <-- Supervised episodic & curriculum trainer
│   └── visualizer.rs           <-- ANSI/ASCII brain topology map & growth timeline
│
├── tools/                      <-- First-Class Autonomous Diagnostic Tool Belt
│   ├── debugger_tool.rs        <-- DebuggerTool: spatial execution & test oracles
│   ├── hover_tool.rs           <-- HoverTool: 3-tier ontological symbol inspection
│   ├── linter_tool.rs          <-- LinterTool: 4-Block spatial geometry & lint audits
│   ├── logger_tool.rs          <-- LoggerTool: sensory flight recorder telemetry
│   ├── mod.rs                  <-- AIToolBelt coordinator, ToolCall/ToolResult dispatch
│   └── restorator_tool.rs      <-- RestoratorTool: 5-pass self-healing & two-way checksum
│
├── transducer/                 <-- Semantic natural language <-> AST transcoder
│   ├── mod.rs                  <-- AITransducer facade
│   ├── session.rs              <-- Interactive conversational session manager
│   └── templates.rs            <-- Canonical OmniCode code generators & reels
│
├── mod.rs                      <-- AI Subsystem Public Facade
└── README.md                   <-- Master Subsystem Documentation
```

---

## 4. The Five-Fold Diagnostic Tool Belt (`AIToolBelt`)

The diagnostic tools originally created for human debugging in Track 1 are surfaced directly as the AI's first-class sensory and autonomic organs:

```
                                  ┌───────────────────────────┐
                                  │   AI DIAGNOSTIC TOOL BELT │
                                  └─────────────┬─────────────┘
                                                │
         ┌──────────────────┬───────────────────┼───────────────────┬──────────────────┐
         │                  │                   │                   │                  │
         ▼                  ▼                   ▼                   ▼                  ▼
┌──────────────────┐┌──────────────────┐┌──────────────────┐┌──────────────────┐┌──────────────────┐
│   LoggerTool     ││   DebuggerTool   ││    LinterTool    ││  RestoratorTool  ││    HoverTool     │
├──────────────────┤├──────────────────┤├──────────────────┤├──────────────────┤├──────────────────┤
│ • Flight recorder││ • Step evaluation││ • 4-Block ruler  ││ • 5-Pass repair  ││ • 3-Tier cards   │
│ • Circular frames││ • Boundary stack ││ • 80-col geometry││ • Two-Way parity││ • .tape <-> .omni│
│ • Incident logs  ││ • Test oracles   ││ • Defect scores  ││ • 0:AWAIT gate   ││ • Coordinates    │
└──────────────────┘└──────────────────┘└──────────────────┘└──────────────────┘└──────────────────┘
```

1. **`LoggerTool` (Sensory Memory)**:
   Gives the AI direct retrospection into raw circular flight recorder frames (`RollFrame`), failpoints, and timestamps without polling.
2. **`DebuggerTool` (Spatial Verification)**:
   Enables the AI to test candidate code in an isolated sandbox, verify delimiter stack balance, and audit immutable invariants.
3. **`LinterTool` (Architectural Discipline)**:
   Audits synthesized code against canonical 4-Block structure (`Metadata -> Opening -> Body -> Closing`) and strict 80-column line geometry.
4. **`RestoratorTool` (Autonomic Self-Healing)**:
   Runs the 5-pass autonomous code repair pipeline and gates commits behind the **Two-Way Checksum** (`DiagnosticLogger` <===> `OmniDebugger` agreement). If the witnesses disagree, it enforces `0:AWAIT` to eliminate false positives and false negatives.
5. **`HoverTool` (Ontological Navigation)**:
   Resolves cross-file definitions, links high-level `.tape` process ribbons to low-level `.omni` source reels, and retrieves docstrings and Attached Asides `(...)`.

### Tool Invocation Dispatch

```rust
use omnicode::ai::{AIToolBelt, ToolCall, ToolResult};

let mut belt = AIToolBelt::new();

// Example: Autonomously heal a malformed script
let result = belt.dispatch(ToolCall::RestoreSource {
    source: "val x = 10;".to_string(),
    verify_with_debugger: true,
});

if let ToolResult::Restoration(report) = result {
    println!("Restored Code:\n{}", report.restored_content);
    println!("Repairs Applied: {:?}", report.repairs_applied);
}
```

---

## 5. Multi-Translation Legal Scripture Architecture

To ground the AI's moral compass and invariant reasoning across historical and theological witnesses while maintaining strict copyright compliance:

1. **Full Prose in Public Domain**:
   - **King James Version (KJV - 1611)**: Full scripture text stored directly in binary memory.
   - **World English Bible (WEB)**: Full modern English text stored directly in binary memory.
2. **Canonical Concordance Pointers (Non-Public Domain)**:
   - **New King James Version (NKJV)**
   - **English Standard Version (ESV)**
   - **Ethiopian Orthodox Canon (88 Books)** — Includes canonical books such as *1 Enoch* and *Jubilees*.

For the non-public translations, CPI-SI stores **coordinate pointers** (`ConcordanceRef`) with cryptographic SHA-256 verse signatures:

```rust
ConcordanceRef::nkjv("Genesis", 1, 3, 3, "sha256:gen_1_3_nkjv");
ConcordanceRef::esv("Genesis", 1, 3, 3, "sha256:gen_1_3_esv");
ConcordanceRef::ethiopic("1 Enoch", 104, 12, 13, "sha256:truth_1Enoch_104_12_eth");
```

Each reference points to the offline, immutable documentation vault stored locally on disk at:
`<TRACK 4> : {Variable Storage & Episodic Memory}/storage/data/immutable_documentation/`

This provides five-fold theological parity without distributing copyrighted prose in the binary.

---

## 6. The 25-Chapter NotebookLM Curriculum

CPI-SI learns from the official repository documentation suite (`docs/notebooklm/`) across 7 volumes and 25 chapters:

- **VOL 00 (Foundations)**: CH 00 (Decalogue & Root Manifest), CH 01 (Ontological Principles & Boundaries).
- **VOL 01 (Track 0 - Silicon Kernel)**: CH 02 (Substrate, BIOS & Memory Map), CH 03 (Ternary ALU, Dual-Rail Logic).
- **VOL 02 (Track 1 - Host Engine)**: CH 04 (AST & Boundaries), CH 05 (Lexer), CH 06 (Parser), CH 07 (VM Evaluator), CH 08 (Logger), CH 09 (Debugger), CH 10 (Linter), CH 11 (Restorator), CH 12 (Hover), CH 13 (AI Cortex & Tools).
- **VOL 03 (Track 2 - Language & StdLib)**: CH 14 (Core Primitives), CH 15 (Nouns), CH 16 (Verbs), CH 17 (Modifiers), CH 18 (Connectors), CH 19 (Tape Ribbons), CH 20 (Scaffolding).
- **VOL 04 (Track 3 - Studio & Apps)**: CH 21 (OmniStudio Interface & Terminal GUI).
- **VOL 05 (Track 4 - Variable Storage)**: CH 22 (Episodic Memory & Weights).
- **VOL 06 (Track 5 - Infrastructure)**: CH 23 (Build Systems & Deployment), CH 24 (System Archives & Immutable Scripture).

### Training Cycle

```rust
use omnicode::ai::{AICortex, Curriculum};

let mut cortex = AICortex::new();
let curriculum = Curriculum::builtin_25_chapters();

// Train neural brain across all 25 chapters
let report = cortex.train_curriculum(&curriculum);
println!("Episodes trained: {}", report.total_episodes);
println!("Synapses adapted: {}", report.synapses_adapted);
println!("Noise resilience: {:.2}%", report.noise_resilience_after * 100.0);
```

---

## 7. Brain Growth Visualizer & Synaptic Topology Maps

CPI-SI includes real-time visualization tools to inspect the physical neural network topology and observe brain maturation over time.

### ASCII Synaptic Map

```
================================================================================
  CPI-SI SYNAPTIC BRAIN TOPOLOGY MAP: CPI-SI Base Instance
  Discrete Balanced-Ternary Substrate {-1, 0, +1} -- CPU Lean Homoiconic Model
================================================================================
Layer 1 [Input Sensory Field | 8 Neurons]:
  N01 [ + . - + + . + - ] (Threshold: 1, Bias: 0)
  N02 [ + + . + - . + + ] (Threshold: 1, Bias: 0)
  ...
Layer 2 [Cognitive Association Layer | 16 Neurons]:
  ...
Layer 3 [Covenant Deliberation / Action Layer | 4 Neurons]:
  N01 [ + + + + + + + + ] (Threshold: 1, Bias: 0)
--------------------------------------------------------------------------------
Synaptic Crystallization:
  [+] Excitatory (+1):  142 ( 55.5%)
  [.] Neutral     (0):   78 ( 30.5%)
  [-] Inhibitory (-1):   36 ( 14.0%)
  Total Synapses:       256
  Active Maturity:      69.5% crystallized
================================================================================
```

### Growth Timeline Across Milestones

```
================================================================================
  CPI-SI BRAIN GROWTH HISTORY & MATURATION TIMELINE
================================================================================
Milestone                    | [+]     | [0]     | [-]     | Maturity     | Noise Resil.
--------------------------------------------------------------------------------
Genesis Inception            | 0       | 256     | 0       |   0.0%       |  70.0%
Tool Belt Ingestion          | 48      | 180     | 28      |  29.7%       |  82.5%
Scripture Grounding          | 96      | 120     | 40      |  53.1%       |  91.2%
NotebookLM Convergence       | 142     | 78      | 36      |  69.5%       |  96.4%
================================================================================
```

---

## 8. Blindspot Analysis & Natural Next Steps

### Identified Architectural Blindspots

1. **Persistent Weight Convergence on Disk**:
   Currently, weights load from `storage/weights/cpisi_base.weights`. When running in bare-metal Track 0, memory is bounded at `0x00200000` (`CPISI_WORKSPACE_BASE`). The serialization format must remain strictly compact and endian-neutral across x86_64, RISC-V, and bare-metal BIOS.
2. **Dynamic Multi-Token Transduction Window**:
   `AITransducer` maps natural language utterances into structured templates. As user queries grow more conversational, the transducer needs streaming sliding-window sentence tokenization to handle compound instructions without losing context.
3. **Interactive REPL / CLI Command Surface**:
   The AI Cortex can be driven programmatically via Rust API, but human operators need dedicated terminal commands (e.g. `omnicode ai chat`, `omnicode ai brain-map`, `omnicode ai heal <file>`).

### Natural Next Steps

1. **Dedicated AI CLI Commands** (`src/cli/commands/ai.rs`):
   - `omnicode ai chat "prompt"`: Conversational discourse loop.
   - `omnicode ai map`: Live terminal rendering of the brain synaptic map.
   - `omnicode ai heal <path>`: Command-line self-healing leveraging `RestoratorTool` and Two-Way Checksum.
   - `omnicode ai train-curriculum`: Retrains weights across the 25-chapter NotebookLM suite.
2. **OmniStudio GUI Integration (Track 3)**:
   - Surface the ANSI brain visualizer inside the terminal GUI dashboard.
   - Add a live "Two-Way Checksum" agreement monitor in the IDE status bar.
   - Provide 3-tier hover tooltips directly hooked into `HoverTool`.
3. **Web Dashboard & Visualizer Update**:
   - Expose the synaptic matrix and growth timeline over lightweight local WebSocket or HTTP endpoints for browser visualization without external cloud dependencies.

# Track 1: Host Engine, Virtual Machine & Living Runtime

> *Where Thought Takes Form: The Cognitive Operating System in Waiting*
> *The Bridge Between Bare Silicon (Track 0) and Living Language (Track 2)*

---

## 1. Executive Summary & Architectural Scope

**Track 1** is the computational heart of OmniCode. It encompasses the **Host Compiler, Balanced Ternary Virtual Machine, Multi-Track Process Scheduler, Episodic Neural Cortex, and Self-Healing Diagnostic Immune System**.

In the overarching OmniCode multi-track cosmos:

* **Track 0 `[KERNEL]`** anchors the physical bedrock: 16-bit MBR, 32-bit Stage 2 PM, 32-bit Multiboot ELF, and 64-bit native UEFI.
* **Track 1 `[ENGINE]`** provides the living execution engine: it parses reels (`.omni`) and process tapes (`.tape`), manages transactional environments, evaluates balanced ternary quad-state physics ($\mathbb{T}_4$), lowers tapes into machine words, transpile-assembles bare-metal kernels, and runs the interactive terminal and studio HUD.
* **Track 2 `[LEXICON]`** provides the canonical vocabulary, type definitions (`character`, `number`), and standard library.

> *"Prepare thy work without, and make it fit for thyself in the field;*
> *and afterwards build thine house."*
> — **Proverbs 24:27 (KJV)**

---

## 2. The "In-Waiting" Symbiotic Incubation Model

OmniCode is not merely a programming language; it is an autonomous, bare-metal operating system. However, during its developmental incubation era, humanity computes inside conventional host operating systems (GNU/Linux, Apple macOS, Microsoft Windows).

Track 1 resolves this through the **"In-Waiting" Symbiotic Execution Model**:

1. **Host Symbiosis:** Track 1 compiles into a single, high-performance native host executable (`omnicode`) written in Rust Edition 2024 (`rust-version = 1.85+`) with zero uncontrolled runtime dependencies (only `rustyline` for interactive terminal buffering).
2. **Hardware Abstraction Layer (HAL):** Through `src/cogs/platform.rs`, Track 1 dynamically adapts to any host environment, discovering hypervisor paths (`qemu-system-x86_64`), native virtualization acceleration (`-enable-kvm` on Linux, `-accel hvf` on macOS, `-accel whpx` on Windows), display servers (`X11`, `Wayland`), and UEFI OVMF firmware blobs.
3. **The Mirror Principle:** Track 1 implements in software the exact same register banks, MMIO buffers, and balanced ternary ALU logic that Track 0 implements in physical silicon. A process tape running in Track 1's virtual machine executes identically when lowered into Track 0's bare-metal kernel.

---

## 3. Subsystem Topology & Directory Layout

Track 1 is organized into nine cleanly decoupled, modular subsystems:

```text
<TRACK 1> : {Host Engine & Virtual Machine}/
├── src/
│   ├── main.rs                    # Master CLI dispatcher & command orchestrator
│   ├── lib.rs                     # Core public API surface & high-level runner
│   │
│   ├── core/                      # Fundamental Mathematical & Memory Primitives
│   │   ├── boundaries.rs          # 6 Morphological boundary types ([], <>, {}, {], [}, ())
│   │   ├── cell.rs                # 8-bit <=> 5-trit bijective codec & hardware control states
│   │   ├── word.rs                # 32-trit Word / Word64 arithmetic & Horner expansions
│   │   ├── ternary.rs             # QuadState algebra (T4 = {-1, 0, +1, NULL})
│   │   ├── vessel.rs              # Bounded storehouses & v-- downward sub-partitioning
│   │   ├── reel.rs                # 4-directional causal spool hierarchy
│   │   ├── tape.rs                # Process tape ribbons, Tape Head covenant, roll stepping
│   │   ├── mmio.rs                # Physical MMIO: VGA (0xB8000), COM1 UART (0x03F8)
│   │   ├── linguistics.rs         # 8 Parts of speech matrix & 8-slot canonical sentences
│   │   └── filetype.rs            # Formal file types (.omni, .scroll, .tape, .log, .weights)
│   │
│   ├── parser/                    # Universal Ingestion & Syntactic Parsing
│   │   ├── universal/             # Character streams, UTF-8 decoders, coordinate cursors
│   │   ├── lexical/               # Directional lexer, token stream, aside preserver
│   │   └── syntactic/             # Recursive-descent parser, grammar rules, block attachment
│   │
│   ├── ast/                       # Strongly-Typed Abstract Syntax Tree
│   │   ├── nodes/                 # Ontological, cardinal, operational, structural, relations
│   │   └── README.md              # AST taxonomy and Aside preservation contract
│   │
│   ├── runtime/                   # Execution, Lowering & Environment Architecture
│   │   ├── environment/           # Scoped frames, 13 named hardware registers, rollback snapshots
│   │   ├── evaluator/             # QuadState evaluator loop, SVO verbs, causal vectors (-->)
│   │   └── assembler/             # Process tape lowering, machine words, baremetal transpiler
│   │
│   ├── diagnostics/               # Autonomic Immune System & Developer Tooling
│   │   ├── logger/                # Microsecond ring buffer DiagnosticLogger & failpoint telemetry
│   │   ├── debugger/              # OmniDebugger, transactional history, reverse-step execution
│   │   ├── linter/                # Implicit 4-Block architecture compliance auditor
│   │   ├── restorator/            # Autonomous immune self-healer & AST repair synthesizer
│   │   └── hover.rs               # Cross-scale 3-tier composite hover card & docstring engine
│   │
│   ├── ai/                        # Neural-Symbolic Cognitive Cortex
│   │   ├── cortex.rs              # Master cognitive deliberation loop & prompt transducer
│   │   ├── neural/                # Balanced-ternary neural network (brain, layers, synapses)
│   │   ├── lexicon/               # 100% offline Biblical world lexicon (6 ontological domains)
│   │   ├── model/                 # AIMemory concept graph, AILearner episodic digestion
│   │   └── transducer/            # English <=> OmniCode bidirectional semantic transducer
│   │
│   ├── terminal/                  # Operating System Scheduling & Visual Studio HUD
│   │   ├── scheduler/             # Cooperative multi-track task scheduler (Tracks 0-5)
│   │   ├── session/               # Living TerminalSession, REPL, command router
│   │   └── studio/                # Cinematic HUD terminal projector & virtual VGA screen
│   │
│   ├── cogs/                      # Cybernetic Senses & Environmental Adapters
│   │   ├── platform.rs            # Dynamic host OS, CPU, hypervisor, and display probing
│   │   └── tripwire.rs            # Invariant collision sensors replacing fatal crashes
│   │
│   └── cli/                       # Command Implementations & HTTP Studio Server
│       ├── repl.rs                # Multi-line interactive REPL shell with bracket buffering
│       ├── server.rs              # Zero-dependency HTTP HUD, REST API & Web Terminal
│       └── commands/              # Subcommands: boot, run, assemble, echo, hover, doc, etc.
│
└── tests/                         # Comprehensive 7-Suite Automated Test Harness (269 Tests)
    ├── unit/                      # Component isolation & balanced ternary algebra
    ├── integrated/                # Subsystem integration, failpoints, retroaction, imports
    │   ├── boot_tripwire_tests.rs     # Bare-metal boot inception & hypervisor synthesis
    │   ├── platform_tripwire_tests.rs # Platform adapter adversarial fault injection
    │   └── assemble_tripwire_tests.rs # Tape lowering, word emissions, rollback oracles
    └── functional/                # End-to-end OS boot, Genesis tape, hover resolution
```

---

## 4. The 13 Named Silicon Registers

Track 1 maps physical and virtual CPU registers into **13 meaningful silicon registers**, governing execution state, memory coordinates, and cognitive boundaries:

| Silicon Register | Functional Role in Track 1 | Host VM Representation |
| :--- | :--- | :--- |
| **`%head`** | Tape Head cursor pointing to active instruction/roll | `HardwareRegisters::head` (u64 / Word64) |
| **`%home`** | Invariant covenant origin anchor ($t=0$) | `HardwareRegisters::home` (u64 / Word64) |
| **`%spool`** | Directional transport vector (forward or reverse) | `HardwareRegisters::spool` (i8 / Trit) |
| **`%it`** | Causal accumulator carrying active evaluation state | `HardwareRegisters::it` (Value / Word64) |
| **`%turn`** | Cadence loop cycle and iteration counter | `HardwareRegisters::turn` (u64) |
| **`%bound`** | Execution limit boundary and sentinel detent | `HardwareRegisters::bound` (usize) |
| **`%state`** | Quad-State truth register (+1, 0, -1, NULL) | `HardwareRegisters::state` (QuadState) |
| **`%root`** | Master storehouse memory base pointer | `HardwareRegisters::root` (u64 address) |
| **`%part`** | Active memory partition offset (`v--`) | `HardwareRegisters::part` (u64 offset) |
| **`%trans`** | Transducer codec control register | `HardwareRegisters::trans` (u8 mode) |
| **`%truth`** | Invariant verification and parity flag | `HardwareRegisters::truth` (bool / Trit) |
| **`%guard`** | Adversative tripwire arming register | `HardwareRegisters::guard` (u16 flags) |
| **`%clock`** | Forward monotonic causal timekeeper | `HardwareRegisters::clock` (u64 cycles) |

---

## 5. Connecting Track 1 to Track 0: The OS Bridge

To make the operating system an **actual operating system**, Track 1 does not merely simulate computation—it bridges directly to the bare-metal kernel in Track 0:

```text
  +-------------------------------------------------------------------------+
  | TRACK 1 : Host VM / Compiler / Assembler                                 |
  |  - High-Level Process Tape (.tape)                                      |
  |  - OmniAssembler (`src/runtime/assembler/transpiler/baremetal.rs`)      |
  +-------------------------------------------------------------------------+
                                      │
                 Bare-Metal Inception Transduction (Transpile)
                                      ▼
  +-------------------------------------------------------------------------+
  | TRACK 0 : Physical / Hypervisor Inception Artifacts                     |
  |  • omnicode_bios.img      (16-Bit Legacy BIOS MBR, 512B Sector 0)       |
  |  • omnicode_bios_dual.img (Dual-State Image: MBR + Stage 2 PM, 2560B)   |
  |  • omnicode_stage2.bin    (32-Bit PM Loader & 16-Trit CPI-SI Arena)      |
  |  • omnicode_multiboot.elf (32-Bit Direct Hypervisor ELF, 1MB Mark)      |
  |  • BOOTX64.EFI            (Native 64-Bit UEFI PE32+ Application)        |
  +-------------------------------------------------------------------------+
                                      │
                 Subprocess & QEMU Hypervisor Orchestration
                                      ▼
  +-------------------------------------------------------------------------+
  | RUNTIME EXECUTION & TELEMETRY                                           |
  |  • COM1 UART (0x03F8)      <──>  Track 1 DiagnosticLogger Ring Buffer   |
  |  • VGA MMIO (0x000B8000)   <──>  Track 1 OmniStudio Virtual Screen      |
  |  • RAM Log (0x18000/0x200) <──>  Track 1 OmniDebugger Flight Snapshots  |
  +-------------------------------------------------------------------------+
```

### Necessary Connections to Realize the Full OS

1. **Bidirectional Binary Transpilation:**
   `omnicode assemble <file.tape> --target [bios|stage2|multiboot|uefi]` compiles process tapes directly into executable boot payloads staged in `kernel/images/`.
2. **Telemetry Parity:**
   Serial bytes streamed from Track 0 over COM1 UART (`0x03F8`) are intercepted by Track 1's `boot` command, parsed into structured failpoints, and handed directly to `DiagnosticLogger` and `Restorator`.
3. **Execution Symmetry:**
   Track 1's virtual MMIO screen (`src/core/mmio.rs`) matches the exact 80x25 memory layout at `0x000B8000`, allowing identical visual rendering in both terminal HUDs and physical displays.

---

## 6. The Autonomic Self-Healing Loop (Restorator)

Track 1 replaces uncontrolled crashes (`panic!`) with a cybernetic feedback loop:

$$\text{Anomaly Encountered} \longrightarrow \text{Tripwire Tripped} \longrightarrow \text{Failpoint Inscribed} \longrightarrow \text{Restorator Heals} \longrightarrow \text{Test Verified}$$

1. **Tripwires (`src/cogs/tripwire.rs`):** Intercept missing files, unmapped addresses, syntax anomalies, and hardware timeouts.
2. **Failpoint Logging (`src/diagnostics/logger/`):** Records microsecond-timestamped error signatures into in-memory ring buffers.
3. **OmniDebugger (`src/diagnostics/debugger.rs`):** Captures full environment snapshots, enabling reversible execution (`-1:ROLLBACK`).
4. **Restorator (`src/diagnostics/restorator.rs`):** Ingests failpoints, synthesizes candidate AST patches (balancing delimiters, injecting missing imports, repairing block structures), and re-evaluates tests before permanently committing changes.

---

## 7. Verification Invariants & Test Harness

Track 1 maintains a **100% pass rate across 269 automated tests** spanning seven specialized harnesses:

| Harness Target | Primary Verification Scope | Passing Tests |
| :--- | :--- | :---: |
| **`lib` Unit Tests** | Core boundaries, ternary algebra, cell codec, vector dispatch, lexical parsing, scheduler lifecycle | **210** |
| **`unit_tests`** | Component isolation, balanced ternary symmetry, cognitive notes, cardinal clauses | **16** |
| **`integrated_tests`** | Subsystem integration, failpoints, delimiter leaks, caveat rollbacks, Restorator healing | **14** |
| **`functional_tests`** | End-to-end OS boot, Genesis boot tape, REPL simulation, AI Cortex pipeline | **12** |
| **`assemble_tripwire_tests`** | Physical assembler lowering, tape-to-machine words, adversarial syntax traps | **7** |
| **`boot_tripwire_tests`** | Bare-metal boot inception, silicon addresses, hypervisor synthesis, telemetry handoff | **5** |
| **`platform_tripwire_tests`** | Platform adapter adversarial faults, display server contracts, subprocess traps | **5** |
| **Total Automated Tests** | **Comprehensive Full-System Verification** | **`269 / 269`** |

Track 1 is verified with zero compiler warnings under Rust Edition 2024.

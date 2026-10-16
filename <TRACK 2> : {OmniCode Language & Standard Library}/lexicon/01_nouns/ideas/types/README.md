# OmniCode Invariant Types: Architectural Taxonomy & Guide

Welcome to the **OmniCode Types System**! Whether you are a systems programmer, language enthusiast, philosopher, or someone completely new to computing, this guide explains how OmniCode models the building blocks of thought, language, and reality.

---

## 1. What is an "Idea { Type }" in OmniCode?

In conventional programming languages, a "type" is often just a technical label telling a compiler how many bytes of memory to reserve (like `int`, `char`, or `float`).

In **OmniCode**, a Type is fundamentally an **Idea**:

- It represents an **ontological invariant**—a universal concept that holds true regardless of what machine, operating system, or silicon chip executes it.
- It unites three worlds:
  1. **Pure Meaning (Semantics):** What the concept means in human thought, natural language, and philosophy.
  2. **Mathematical Balance (Virtual Substrate):** Symmetrical balanced ternary logic (`-1`, `0`, `+1`), where truth is dynamic equilibrium.
  3. **Physical Silicon (Hardware Target):** Concrete execution on modern 64-bit microprocessors, memory registers, and video terminals.

---

## 2. Core Concepts: Learning the OmniCode Way

### Words vs. Values, States vs. Numbers

One of the most profound breakthroughs in OmniCode is understanding that values, words, and states share the same symbols, but differ in human intent:

- **`0` as a State:** Represents **`AWAIT`**—a state of perfect equilibrium, resting quietly, waiting for the next event.
- **`"0"` as a Value:** Using literal "air-quotes" (or enclosing brackets like `{ 0 }` or `[ 0 ]`) tells the machine: *"Treat this zero not as a pause state, but as the literal mathematical quantity zero."*
- **`'AWAIT'` as a Word:** Single quotes clarify that a reserved keyword is being spoken of as a literal word rather than being triggered as a command.

Because OmniCode is a **freestanding language** designed to communicate with hardware without hidden assumptions, it reads all symbols literally. You provide the semantics that express your true intent.

### The Analogy of the Body: "Fleshing Out the Bones"

OmniCode grows organically, mirroring how a living organism develops:

1. **The Skeleton:** We first build the rigid architectural bones—the invariant laws, mathematical boundaries, and fundamental categories.
2. **The Tissue:** We link the bones together using the **ladder.baton** system, connecting universal roots to specialized types.
3. **The Flesh:** We add dynamic features, transpilers, runtime engines, and tests.
4. **Shaping & Refining:** We continually test, reshape, and harmonize all parts so the entire system breathes together across all tracks.

---

## 3. The `ladder.baton` Inheritance System

In traditional software projects, shared code is frequently copy-pasted across folders, leading to loose, duplicate files that easily get out of sync.

In OmniCode, we use the **`ladder.baton` system**:

- The **Universal Root** (`universal/`) holds the core axioms: balanced ternary math, the four causal states, connective conjunctions, and safety interlocks.
- Downstream types (like `character/` and `number/`) **inherit** these universal invariants down the ladder like a baton passed in a relay race.
- **Zero Duplicate Files:** Individual type folders contain only what is unique to their type. Universal invariants are passed down via clean module paths and compiler include flags.

```text
                    ┌─────────────────────────┐
                    │       universal/        │  <-- Ground Truth / Axioms
                    │  (ALU, States, Connect) │
                    └────────────┬────────────┘
                                 │
                         passes the baton
                                 ▼
         ┌───────────────────────┴───────────────────────┐
         │                                               │
         ▼                                               ▼
┌─────────────────┐                             ┌─────────────────┐
│   character/    │                             │     number/     │
│ (1 Cell, Glyphs)│                             │(1 Word, Scalars)│
└─────────────────┘                             └─────────────────┘
```

---

## 4. The Three-Pillar Modular Folder Structure

Every type folder in OmniCode is organized into three complementary expressions:

```bash
<type>/
├── compiled/           # 1. High-Level Source Reel (.omni)
├── interpreted/        # 2. Runtime Execution Ribbon (.tape)
└── transpiled/         # 3. Concrete Physical Bridges
    ├── compiled/c/     #    - Freestanding Bare-Metal C Backbone
    └── interpreted/lua/#    - Dynamic Coroutine Ribbon Companion
```

### Pillar 1: `compiled/` (`<type>.omni`)

The high-level declaration of the type. Written in canonical OmniCode notation, it reads like structured, elegant prose. It defines the type's metadata, biblical/philosophical witness, capacity footprint, and four-block lifecycle laws.

### Pillar 2: `interpreted/` (`<type>.tape`)

The dynamic interpreted process ribbon. Think of this like a physical strip of celluloid movie film or a Turing tape. It breaks execution down into sequential **rolls** (Inscribe, Transduce, Compare, Equilibrium, Sequence) that step forward smoothly or roll back safely if an error occurs.

### Pillar 3: `transpiled/` (Physical Language Bridges)

To allow OmniCode to run on any computer in the world today, OmniCode transpiles automatically into physical targets:

- **`compiled/c/`**: A pure, freestanding C99/C11 implementation. Contains `include/`, `src/`, `tests/`, and a `Makefile`. It requires zero third-party libraries and can run directly on bare metal (even booting on an x86 CPU without an operating system!).
- **`interpreted/lua/`**: A lightweight, embeddable Lua implementation. Uses Lua coroutines to step through tape rolls as cooperative fibers, with automatic state snapshotting and rollback.

---

## 5. Overview of Current Types

| Type Directory | Capacity & Footprint | Primary Role | Philosophical Witness |
| :--- | :--- | :--- | :--- |
| **`universal/`** | Foundation Substrate | Balanced Ternary ALU, Quad-State Truth, Connectives | Proverbs 11:1 & 24:27 |
| **`character/`** | 1 Cell (5 Trits / 8 Bits) | Atomic glyphs, letters, terminal inscription runes | Exodus 31:18 |
| **`number/`** | 1 Word (32 Trits / 64 Bits) | Scalar quantities, balanced arithmetic, zero-sum ALU | Psalm 147:4 |
| *`container/`* | Variable Bounded Space | Envelopes, memory partitions, bounded buffers | *(In transition)* |
| *`site/`* | Memory Coordinates | Memory-mapped I/O, physical RAM locations | *(In transition)* |
| *`state/`* | Causal Status Register | Fourfold causal detents, program loop steppers | *(In transition)* |

---

## 6. How to Test & Explore

OmniCode enforces a strict **"Test First, Integrate Second"** methodology. You can run unit tests at any level:

### Universal Bedrock Tests

```bash
# Test universal C axioms & ALU
cd universal/transpiled/compiled/c && make test

# Test universal Lua tape engine
lua universal/transpiled/interpreted/lua/tests/test_tape_engine.lua
```

### Character Type Tests

```bash
# Test character C backbone
cd character/transpiled/compiled/c && make test

# Test character Lua tape ribbon
lua character/transpiled/interpreted/lua/tests/test_character.lua
```

### Number Type Tests

```bash
# Test number C backbone
cd number/transpiled/compiled/c && make test

# Test number Lua tape ribbon
lua number/transpiled/interpreted/lua/tests/test_number.lua
```

### Full Host Engine (Rust) Verification

From the repository root:

```bash
cargo test --manifest-path "<TRACK 1> : {Host Engine & Virtual Machine}/Cargo.toml"
```

*(Confirms that all 252 host VM tests pass across every component).*

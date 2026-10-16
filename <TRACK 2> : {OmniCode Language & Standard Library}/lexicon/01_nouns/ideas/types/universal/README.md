# OmniCode Universal Substrate: The Bedrock of Balance

Welcome to the **Universal Foundation** of OmniCode!

The `universal/` folder is the bedrock upon which all OmniCode types, nouns, verbs, and tracks are built. It defines the physics, mathematics, and causal logic of the OmniCode universe.

---

## 1. The Core Philosophy: Balanced Ternary & Equilibrium

Most computer systems today are built on **binary logic** (`0` and `1`). Binary is inherently one-sided: it forces the world into rigid opposites (on/off, true/false, presence/absence). Representing negative numbers or undoing mistakes in binary requires awkward tricks (such as "two's complement" arithmetic).

OmniCode is founded on **Balanced Ternary**:
> *"A false balance is abomination to the LORD: but a just weight is his delight."*  
> — **Proverbs 11:1 KJV**

In balanced ternary, every basic digit (called a **trit**) can take one of three natural states centered symmetrically around zero:

- **`+1` (Positive / Surplus):** Forward motion, affirmative creation, advance.
- **`0` (Equilibrium / Rest):** Perfect balance, standstill, awaiting input.
- **`-1` (Negative / Deficit):** Backward pull, restoration, unwinding an error.

Because balanced ternary is completely symmetrical around zero, **negation is free**, subtraction is simply adding the opposite, and balance is built directly into the fabric of computation!

---

## 2. Quad-State Causal Truth: How Time & Logic Flow

OmniCode replaces simple Boolean true/false with **Quad-State Causal Truth**. Instead of just asking *"Is this true or false?"*, OmniCode asks: *"In which direction should computation flow?"*

| State | Trit Value | Meaning | Real-World Analogy |
| :--- | :---: | :--- | :--- |
| **`PROCEED`** | `+1` | Forward Causation | The green light. Everything is sound; advance the step. |
| **`AWAIT`** | `0` | Equilibrium Standby | The amber pause. The system is balanced, waiting for input or a timer. |
| **`ROLLBACK`** | `-1` | Retroactive Reversal | The rewind button. A safety guard was tripped; restore the last safe snapshot. |
| **`HALT`** | `NULL` (`-2`) | Terminal Detent | The emergency brake. Stop the processor clock immediately. |

---

## 3. Connective Operators as Natural Human Conjunctions

In natural human speech, we don't just say `AND` or `OR`. We express nuanced relationships with words like *"and"*, *"but"*, *"or"*, and *"in relation to"*.

OmniCode translates these intuitive conjunctions into exact mathematical logic gates:

### `::` (AND / Cumulative Conjunction)

- **Human Meaning:** *"Do this AND do that simultaneously."*
- **Behavior:** Both sides must succeed (`PROCEED`). If either is waiting, the result waits (`AWAIT`). If either fails, the whole conjunction rolls back (`ROLLBACK`).

### `!` (BUT / Adversative Caveat)

- **Human Meaning:** *"Execute this action, BUT only if this safety condition holds."*
- **Behavior:** The action advances, *unless* the caveat fails or halts, in which case the action is immediately blocked and rolled back.

### `|` (OR / Disjunctive Valve)

- **Human Meaning:** *"Take path A OR take path B."*
- **Behavior:** If either path can proceed, computation advances. If both are blocked, it rolls back.

### `--` (NOT / Polarity Inversion)

- **Human Meaning:** *"Flip the truth value around the center."*
- **Behavior:** Turns `PROCEED (+1)` into `ROLLBACK (-1)` and vice versa, leaving resting `AWAIT (0)` undisturbed.

### `!=` (Inequality / "Not to Be")

- **Human Meaning:** *"Verify that these two states or values are different."*

### `=!` (Is Not / "To Be Not")

- **Human Meaning:** *"Confirm that current state is NOT a specific forbidden condition."*

### `=~` (Liken To / Resemblance)

- **Human Meaning:** *"See if these two ideas are compatible or like one another."*

### `:` (In Relation To / Perspective)

- **Human Meaning:** *"View this action through the lens of this context."*

---

## 4. The Dual-Rail Silicon Secret

How can modern computers (which only have binary wires) run OmniCode's balanced ternary logic at native speeds?

OmniCode uses **Dual-Rail Polar Bit-Planes**:

- Every balanced trit is physically represented using a pair of bits: `(Negative Rail, Positive Rail)`.
  - `00`: **Equilibrium (0)** — Neither wire active.
  - `01`: **Positive (+1)** — Positive rail active.
  - `10`: **Negative (-1)** — Negative rail active.
  - `11`: **Collision / Trap (NULL)** — Both wires active simultaneously, which is an illegal hardware fault!

### Instantaneous Negation (Rail Swapping)

To negate a 64-bit word of 32 trits in conventional math, a computer has to perform multiple arithmetic operations with borrowing.
In OmniCode, **negation is just swapping the positive and negative bit-planes**! The CPU does this in just two hardware clock cycles, achieving instant SIMD (Single Instruction, Multiple Data) negation.

---

## 5. Directory Contents & Architecture

```bash
universal/
├── README.md               # This foundational guide
└── transpiled/
    ├── compiled/c/         # Freestanding C99/C11 Hardware Backbone
    │   ├── Makefile        # Pure POSIX build script
    │   ├── compile_flags.txt
    │   ├── include/
    │   │   ├── omni_base.h # Invariant axioms, ALU arithmetic, connectives
    │   │   └── omni.h      # Master umbrella header for all types
    │   └── tests/
    │       ├── test_common.h   # Minimal freestanding assertion macros
    │       └── test_universal.c# Unit testbed verifying universal laws in isolation
    └── interpreted/lua/    # Dynamic Coroutine Process Ribbon
        ├── tape_engine.lua # Coroutine fiber runner, snapshots, connectives
        └── tests/
            ├── test_common.lua     # Assertion suite for Lua
            └── test_tape_engine.lua# Unit testbed verifying tape engine in isolation
```

---

## 6. Running Universal Tests

We always test the universal foundation **before** testing any individual type:

```bash
# 1. Test the C Bare-Metal Bedrock
cd transpiled/compiled/c
make test

# 2. Test the Lua Coroutine Tape Engine
cd ../../interpreted/lua
lua tests/test_tape_engine.lua
```

Both test suites verify:

- Hardware preconditions and platform word sizes
- Balanced ternary arithmetic and carry propagation
- Dual-rail encoding, decoding, and collision traps
- The complete suite of connectives (`!`, `::`, `|`, `--`, `!=`, `=!`, `=~`, `:`)
- State vs. literal disambiguation
- Tripwire arming, tripping, and transactional rollback

# Invariant Type: < Number >

Welcome to the documentation for **Number**, the foundational scalar measurement and arithmetic invariant type in OmniCode!

---

## 1. What is a Number in OmniCode?

In conventional programming, a number is typically just a fixed binary sequence (like a 32-bit or 64-bit integer) that wraps around or overflows when limits are exceeded.

In **OmniCode**, a Number is an **Invariant Type**:

- It represents the **cosmic order of measure and quantity**.
- **Witness:**
  > *"He telleth the number of the stars; he calleth them all by their names."*  
  > — **Psalm 147:4 KJV**
- **Semantics:** `{ ScalarQuantity :: DualRailBitPlane :: ZeroSumEquilibrium }`

### Footprint & Capacity Law

- **Defined Space:** Exactly **1 Machine Word**.
- **Dual Substrate:**
  - Physical Silicon Substrate: **64 Bits** (standard 64-bit microprocessor register, `int64_t` / `uint64_t`).
  - Virtual Balanced Substrate: **32 Balanced Trits** ($3^{32} \approx 1.85 \times 10^{15}$ symmetrical states centered at 0).
- **The Core ALU Zero-Sum Equilibrium Law:**
  $$(+1) + (-1) \iff 0$$
  Balance is not just an arithmetic result; it is the fundamental proof of correct execution.

---

## 2. Dual-Rail Polar Bit-Planes on Silicon

To run balanced ternary numbers at maximum speed on standard binary computers, OmniCode packs 32 trits into a native 64-bit integer using **Dual-Rail Bit-Planes**:

| 2-Bit Code | Rail Activation | Trit Value | Meaning |
| :---: | :--- | :---: | :--- |
| `00` | Neither rail active | `0` | **Equilibrium / True Zero** |
| `01` | Positive rail active (bit 0) | `+1` | **Positive Unity** |
| `10` | Negative rail active (bit 1) | `-1` | **Negative Unity** |
| `11` | Both rails active | `NULL` | **Collision Trap / Hardware Fault** |

### Why Dual-Rail is Revolutionary

1. **Instant SIMD Negation:** Negating all 32 trits in a 64-bit word requires no arithmetic addition or two's complement carry chains. It is performed instantaneously by swapping the positive and negative bit-planes in two CPU cycles!
2. **Built-in Safety Trap:** If memory ever corrupts or attempts an impossible simultaneous positive and negative state (`11`), the tripwire automatically trips and rolls back.

---

## 3. Dynamic Packing Topologies

A byte has 8 bits, which can hold balanced trits in different kinematic arrangements depending on how the number is used:

- **`2-2-2-2` (Pure Dense):** Four 2-bit trits per byte (standard compact storage).
- **`2-1-2-1-2` (Palindromic Balance):** Symmetric resting equilibrium with guard channels.
- **`1-2-1-2-2` (Prefix Carry):** Leaves a leading 1-bit channel for fast forward carry propagation.
- **`2-2-1-2-1` (Postfix Carry):** Leaves a trailing channel for reverse borrow propagation.
- **`3-3-2`, `3-2-3`, `2-3-3` (Directional Vectors):** Partitions bits to represent spatial 3D directions (X, Y, Z axes) directly inside a single byte.

---

## 4. Directory Layout & Three Pillars

Following the OmniCode standard, `number/` is structured into three pillars:

```bash
number/
├── README.md               # This architectural guide
├── compiled/               # Pillar 1: High-Level Source Reel
│   └── number.omni         # Canonical OmniCode source code
├── interpreted/            # Pillar 2: Process Ribbon
│   └── number.tape         # Five-roll interpreted execution tape
└── transpiled/             # Pillar 3: Physical Bridges
    ├── compiled/c/         # Freestanding C99/C11 Hardware Backbone
    │   ├── Makefile        # Pure POSIX build script
    │   ├── compile_flags.txt
    │   ├── include/
    │   │   └── number.h    # omni_number_t, topologies, and dual-rail API
    │   ├── src/
    │   │   └── number.c    # Balanced ternary ALU, addition, negation, packing
    │   └── tests/
    │       └── test_number.c # Freestanding C unit test suite
    └── interpreted/lua/    # Dynamic Coroutine Process Ribbon
        ├── number.lua      # Lua tape implementation driven by TapeEngine
        └── tests/
            └── test_number.lua # Lua unit test suite
```

---

## 5. The 5-Roll Lifecycle

Both `number.tape` (interpreted) and `number.c` (compiled) execute the Number lifecycle in **5 sequential rolls**:

1. **Inscribe:** Takes a raw quantity (e.g. 4096) and encloses it safely inside the 1-word boundary.
2. **Transduce:** Converts the 64-bit integer into 32 balanced trits and packs them into dual-rail bit-planes.
3. **Compare:** Guards the word boundary and confirms that no `11` collision traps exist.
4. **Equilibrium:** Proves the algebraic Zero-Sum law $(+1) + (-1) = 0$ and confirms that double negation restores the exact original value $(-(-X) == X)$.
5. **Sequence:** Commits the number and prepares it to link smoothly into higher-level vectors and arrays.

---

## 6. How Number Inherits from Universal (`ladder.baton`)

Notice that `number/` contains **no duplicate universal files**.

It inherits the universal foundation cleanly:

- **In C:** `number.h` includes `"omni_base.h"`. The `Makefile` uses `-I../../../../universal/transpiled/compiled/c/include`.
- **In Lua:** `number.lua` loads `tape_engine.lua` from `universal/transpiled/interpreted/lua/`.

---

## 7. Running Number Tests

You can test Number across both C and Lua environments:

```bash
# Run the C unit tests
cd transpiled/compiled/c
make test

# Run the Lua unit tests
cd ../../interpreted/lua
lua tests/test_number.lua
```

Both test suites confirm:

- Radix-3 balanced ternary conversion and reconstruction
- Dual-rail bit-plane encoding, decoding, and collision detection
- SIMD rail-swap negation involution: `-(-X) == X`
- Dynamic packing and unpacking across all 7 topologies
- The ALU Zero-Sum Equilibrium Law: `(+1) + (-1) <=> 0`
- Safe execution through all 5 lifecycle rolls

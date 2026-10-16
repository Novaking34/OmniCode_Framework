# Invariant Type: < Character >

Welcome to the documentation for **Character**, the fundamental atomic rune of language and symbolic computation in OmniCode!

---

## 1. What is a Character in OmniCode?

In conventional computing, a character is often treated as just an arbitrary number from an ASCII or Unicode table.

In **OmniCode**, a Character is an **Invariant Type**:

- It represents the **atomic inscription**—the smallest indivisible mark of human language, code, or mathematics.
- **Witness:**
  > *"And he gave unto Moses, when he had made an end of communing with him upon mount Sinai, two tables of testimony, tables of stone, written with the finger of God."*  
  > — **Exodus 31:18 KJV**
- **Semantics:** `{ AtomicGlyph :: LetterRune :: BalancedCellTransduction }`

### Footprint & Capacity Law

- **Defined Space:** Exactly **1 Memory Cell**.
- **Dual Substrate:**
  - Physical Silicon Substrate: **1 Byte** (8 bits, 0 to 255).
  - Virtual Balanced Substrate: **5 Balanced Trits** ($3^5 = 243$ unique equilibrium states).
- **Invariant Boundary Law:**
  `{ Character } = [ Enclosed ]: TRUE`  
  A Character has strict single-cell capacity. It can never overflow or leak into neighboring cells.

---

## 2. Hardware Locators (`::@`)

In OmniCode, code can speak directly to physical computer devices using the hardware locator operator (`::@`):

- `[ Terminal Stdout ] ::@ 0x01`: Direct inscription to standard terminal output streams.
- `[ Screen VGA ] ::@ 0x000B8000`: Direct bare-metal inscription into the classic PC text-mode video memory buffer (allowing OmniCode characters to appear on screen even without an OS!).

---

## 3. Directory Layout & Three Pillars

Following the OmniCode standard, `character/` is organized into three pillars:

```bash
character/
├── README.md               # This architectural guide
├── compiled/               # Pillar 1: High-Level Source Reel
│   └── character.omni      # Canonical OmniCode source code
├── interpreted/            # Pillar 2: Process Ribbon
│   └── character.tape      # Five-roll interpreted execution tape
└── transpiled/             # Pillar 3: Physical Bridges
    ├── compiled/c/         # Freestanding C99/C11 Hardware Backbone
    │   ├── Makefile        # Pure POSIX build script
    │   ├── compile_flags.txt
    │   ├── include/
    │   │   └── character.h # omni_character_t definition & conversion API
    │   ├── src/
    │   │   └── character.c # Bijective transduction & boundary enforcement
    │   └── tests/
    │       ├── compile_flags.txt
    │       └── test_character.c # Freestanding C unit test suite
    └── interpreted/lua/    # Dynamic Coroutine Process Ribbon
        ├── character.lua   # Lua tape implementation driven by TapeEngine
        └── tests/
            └── test_character.lua # Lua unit test suite
```

---

## 4. The 5-Roll Lifecycle

Both `character.tape` (interpreted) and `character.c` (compiled) execute the atomic Character lifecycle in **5 sequential rolls**:

1. **Inscribe:** Takes an incoming raw glyph byte (e.g. ASCII `'A'`, value 65) and encloses it safely inside the 1-cell boundary.
2. **Transduce:** Performs bijective mathematical mapping between the 8-bit silicon byte and the 5 balanced trits:
   $$\text{Glyph} = \sum_{i=1}^{5} \text{trit}_i \times 3^{i-1}$$
3. **Compare:** Guards the single-cell boundary, verifying that round-trip transduction reproduces the exact glyph without distortion.
4. **Equilibrium:** Verifies Proverbs 11:1 algebraic balance on the trits.
5. **Sequence:** Commits the cell and prepares it to link smoothly into higher-level strings and texts.

---

## 5. How Character Inherits from Universal (`ladder.baton`)

Notice that `character/` does **not** contain duplicate copies of `omni_base.h`, `test_common.h`, or `tape_engine.lua`.

Instead, it inherits these foundational pieces directly from `universal/`:

- **In C:** `character.h` includes `"omni_base.h"`. The `Makefile` passes `-I../../../../universal/transpiled/compiled/c/include`.
- **In Lua:** `character.lua` loads `tape_engine.lua` from `universal/transpiled/interpreted/lua/`.

---

## 6. Running Character Tests

You can test Character across both C and Lua environments:

```bash
# Run the C unit tests
cd transpiled/compiled/c
make test

# Run the Lua unit tests
cd ../../interpreted/lua
lua tests/test_character.lua
```

Both test suites confirm:

- Bijective round-trip fidelity for letters, digits, whitespace, and boundary bytes
- Strict single-cell enclosure verification
- Execution through all 5 lifecycle rolls
- Parity with universal balanced ternary axioms

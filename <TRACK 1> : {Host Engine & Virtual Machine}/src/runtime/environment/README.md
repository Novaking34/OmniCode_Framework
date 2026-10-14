# OmniCode Runtime Environment Subsystem

```text
==============================================================================
BLOCK 1: METADATA & ARCHITECTURAL MANIFESTO
==============================================================================
Subsystem: OmniCode Host Engine & Virtual Machine Runtime Environment
Layer:     Execution Substrate & Transactional State Manager
Path:      <TRACK 1> : {Host Engine & Virtual Machine}/src/runtime/environment/
==============================================================================
```

> **"And he gave unto Moses, when he had made an end of communing with him upon mount Sinai, two tables of testimony, tables of stone, written with the finger of God."**  
> — *Exodus 31:18 (KJV)*  
> *(The immutable inscription of atomic runes onto bounded physical tablets).*
> **"Prepare thy work without, and make it fit for thyself in the field; and afterwards build thine house."**  
> — *Proverbs 24:27 (KJV)*  
> *(The discipline of establishing atomic data types and state substrates prior to constructing execution engines).*
> **"Remove not the ancient landmark, which thy fathers have set."**  
> — *Proverbs 22:28 (KJV)*  
> *(The preservation of historical memory boundaries, silicon addresses, and machine constants).*

---

## 1. Architectural Mission

The **Runtime Environment Subsystem** is the foundational state and data substrate for the OmniCode Virtual Machine. It decouples **value representation, physical hardware state, and transactional scoping** from the **imperative execution engine (the Evaluator)**.

By treating the environment as an independent, modular library rather than an ad-hoc dictionary:

1. **Any VM subsystem** (Evaluator, Assembler, Diagnostics, REPL, or AI Cortex) can interrogate and construct universal runtime values without circular dependencies.
2. **Reversible Execution** under balanced ternary `-1:ROLLBACK` and speculative guard failures (`! BUT`) is mathematically guaranteed through atomic transactional snapshots.
3. **Cross-Substrate Parity** is achieved across continuous text ribbons, 8-bit glyph bytes, dense 5-trit balanced ternary cells, and structured entity records.

---

## 2. Topological Layer Structure

The subsystem is organized into three strictly stratified architectural layers:

```bash
src/runtime/environment/
├── universal/                  <-- Tier 1: Universal Runtime Values & Metrics
│   ├── values.rs               <-- RuntimeValue, Unwrappers, Relational Equality, Metrics
│   └── mod.rs                  <-- Re-exports Universal Substrate
│
├── hardware/                   <-- Tier 2: Physical Silicon Register Bank & Ports
│   ├── registers.rs            <-- RegisterBank (13 Named Registers), OMNI_TARGET_*
│   └── mod.rs                  <-- Re-exports Hardware Substrate
│
├── scope/                      <-- Tier 3: Transactional Execution State & Scopes
│   ├── frame.rs                <-- CallFrame (SPOOL Activation Records)
│   ├── snapshot.rs             <-- EnvironmentSnapshot (Full Transactional Image)
│   ├── environment.rs          <-- Environment Coordinator (Bindings, Streams, State-Primes)
│   └── mod.rs                  <-- Re-exports Scope Primitives
│
├── mod.rs                      <-- Subsystem Facade & Backward-Compatible API
└── README.md                   <-- Architectural Manifesto & Specification
```

### Layer 1: Universal (`universal/`)

- **`values.rs`**: Contains `RuntimeValue`, the core algebraic data type for all runtime entities:
  - `State(QuadState)`: Balanced ternary truth (+1: Proceed, 0: Await, -1: Rollback, NULL: Halt).
  - `Number(f64)`: 64-bit numeric scalar.
  - `Text(String)`: Continuous character sequence ribbon.
  - `Cell(Cell)`: 5-trit balanced ternary cell spanning 243 discrete states.
  - `Character { glyph: u8, cell: Cell }`: Dual-substrate atomic rune unifying 8-bit ASCII glyph with a 5-trit cell.
  - `Vessel(Vessel)`: Bounded multi-cell memory vessel.
  - `EntityRecord`: Dynamic record with field map and mutability contract.
  - `Vector`: Ordered sequence of runtime values.
- **Universal Introspection & Coercion**:
  - `as_state()`, `as_number()`, `as_text()`, `as_cell()`, `as_character()`, `as_vector()`.
  - `to_quad_state()`: Universal balanced ternary truth projection.
  - `is_glyph_equal()`: Cross-substrate relational equivalence testing without type friction.
  - `capacity_in_cells()`: Universal memory metric for invariant bound checks (e.g., `{ Character' <= [ 1 Cell ] }`).
- **Dependency Rule**: Zero dependencies on `hardware/` or `scope/`. Pure data substrate.

### Layer 2: Hardware (`hardware/`)

- **`registers.rs`**: Implements the 13 Named Silicon Machine Registers:
  - `%head` (0): Instruction Pointer / Tape Head
  - `%home` (1): Activation Frame Origin
  - `%spool` (2): Stack Spool / Activation Record Floor
  - `%it` (3): Primary Accumulator
  - `%self` (4): Instance Context
  - `%caller` (5): Invoker Context
  - `%other` (6): Peer Context
  - `%state` (7): Active QuadState Status
  - `%turn` (8): Temporal Execution Cadence
  - `%bound` (9): Watchdog Horizon Limit
  - `%from` (10): Inbound Hardware Port / Input Stream
  - `%into` (11): Outbound Hardware Port / Output Stream
  - `%discard` (12): Bit-Bucket Null Sink (writes are dropped, preserving invariance)
- **Canonical Hardware Ports**:
  - `OMNI_TARGET_STDOUT` (`0x01`): Standard terminal console.
  - `OMNI_TARGET_VGA` (`0x000B8000`): Direct video buffer memory address.
  - `OMNI_TARGET_COM1` (`0x03F8`): Serial communications port.
- **Dependency Rule**: Depends only on `universal/values.rs` and `assembler::MachineRegister`.

### Layer 3: Scope (`scope/`)

- **`frame.rs`**: `CallFrame` activation records tracking caller context, base pointers, and local scopes.
- **`snapshot.rs`**: `EnvironmentSnapshot` capturing the complete six-fold transactional horizon:
  1. Symbol table bindings (`bindings`)
  2. Silicon registers (`registers`)
  3. Call stack depth (`call_stack_len`)
  4. Outbound stream length (`output_len`)
  5. Continuous sequence ribbon length (`sequence_len`)
  6. Inbound stream buffer queue (`input_stream`)
- **`environment.rs`**: The state coordinator managing transactions (`begin`, `commit`, `rollback`), state-prime resolution (`Character'` <-> `Character`), procedural frames, and transducer I/O ports.

---

## 3. Reversible Computing & Rollback Guarantees

In OmniCode, computation is inherently reversible. Under balanced ternary logic:

- `+1:PROCEED` commits mutations to the active scope.
- `-1:ROLLBACK` or guard failure (`! BUT`) immediately calls `rollback_transaction()`.

Because `EnvironmentSnapshot` records not only bindings and registers, but also stream truncation horizons (`sequence_ribbon` and `output_stream`) and clones `input_stream`, rolling back a speculative branch leaves **zero side-effects**:

```rust
pub fn rollback_transaction(&mut self) -> Result<(), String> {
    if let Some(prev) = self.snapshots.pop() {
        self.bindings = prev.bindings;
        self.registers = prev.registers;
        self.call_stack.truncate(prev.call_stack_len);
        self.output_stream.truncate(prev.output_len);
        self.sequence_ribbon.truncate(prev.sequence_len);
        self.input_stream = prev.input_stream;
        Ok(())
    } else {
        Err("FAILPOINT [RollbackUnderflow]: No active snapshot".into())
    }
}
```

---

## 4. State-Prime Resolution Invariant

Identifiers in OmniCode can be decorated with state-prime affixes (`Character'` or `Character```) indicating temporal progression:

- When looking up `Character'`, if not yet explicitly bound, the environment falls back to `Character`.
- When mutating `Character'`, the environment writes the primed identifier and synchronizes the base `Character` symbol.
- In relational comparisons (`{ Character' = Glyph }`), the evaluator queries `is_glyph_equal` without mutating base state.

---

## 5. Canonical Four-Block Layout

Every source file in the Environment subsystem strictly adheres to the **Canonical 4-Block Structure**:

1. **Block 1: Metadata Block**: File scope, scripture anchors, and AI/developer context rationale.
2. **Block 2: Opening Block**: Core imports, struct/enum definitions, and baseline initializers.
3. **Block 3: Body Block**: Business logic, operations, and trait implementations.
4. **Block 4: Closing Block**: Display formatting, default traits, and verification unit tests.

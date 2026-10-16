# OmniCode AST (Abstract Syntax Tree) Subsystem

> *"And he gave unto Moses, when he had made an end of communing with him upon mount Sinai, two tables of testimony, tables of stone, written with the finger of God."*  
> — **Exodus 31:18**  
>
> *"Prepare thy work without, and make it fit for thyself in the field; and afterwards build thine house."*  
> — **Proverbs 24:27**  
>
> *"Remove not the ancient landmark, which thy fathers have set."*  
> — **Proverbs 22:28**  
>
> *"He telleth the number of the stars; he calleth them all by their names."*  
> — **Psalm 147:4**  

---

## 1. Architectural Philosophy: The Structural Skeleton of OmniCode

In the OmniCode ecosystem, code is not an arbitrary string of characters or an impermanent script; it is a **covenantal inscription of structured thought**. Just as an organic body grows from an invariant skeletal scaffold before tissue, muscle, and skin are molded onto it ("fleshing out the bones"), the OmniCode compilation pipeline constructs an **Abstract Syntax Tree (AST)** that represents the immutable architecture of human-machine communication.

The AST subsystem in **Track 1 (Host Engine & Virtual Machine)** provides the unified, type-safe data model that bridges:

1. **The Parsing Pipeline (`src/parser/`)**: Ingests declarative source reels (`.omni`) and procedural process tapes (`.tape`).
2. **The Evaluator Runtime (`src/runtime/evaluator/`)**: Executes expressions, resolves causal vectors, verifies invariants, and manages transactional rollbacks.
3. **The Assembler & Transpiler (`src/runtime/assembler/`)**: Lowers high-level AST constructs into 64-bit balanced ternary machine words and transpiles them into standalone C and Lua targets.
4. **CPI-SI (Covenant Partnered Intelligence - Structural Intelligence)**: Embeds intelligent reasoning, contextual intent, and analogy-based linking directly into the syntax of the language.

---

## 2. Taxonomic Subsystem Organization

The AST subsystem is organized into seven modular domains under `src/ast/nodes/`, unified by the master enumeration `ASTNode` in `src/ast/nodes/mod.rs`:

```bash
src/ast/
├── mod.rs                         <-- Subsystem manifest and universal re-exports
├── README.md                      <-- Architectural manifesto and documentation
└── nodes/
    ├── mod.rs                     <-- Unified ASTNode enum and integration tests
    ├── boundaries.rs              <-- First-class asides, boundary frames, and topological casts
    ├── primitives.rs              <-- Scalar arithmetic, affix polarities, and unary inversions
    ├── relations.rs               <-- Relations of being, causal directional vectors, connectives
    ├── structural.rs              <-- Programs, modules, tapes, 4-block sections, rolls, commit gates
    ├── cardinal.rs                <-- Six Cardinal Interrogatives (WHO, WHAT, WHEN, WHERE, WHY, HOW)
    ├── operational.rs             <-- SVO Action Verbs, adversative guards, hardware locators
    └── ontological.rs             <-- Invariant types, entities, analogies, substrates, exemplars
```

---

## 3. The Seven Taxonomic Domains

### Domain 1: Boundaries & First-Class Asides (`boundaries.rs`)

Boundaries are the first-class topological containers of OmniCode. Rather than using arbitrary parentheses or curly braces, every delimiter carries an ontological ranking:

| Delimiter | Boundary Type | Semantics & Hardware Footprint |
| :--- | :--- | :--- |
| `[ ... ]` | **Denotation** | Physical allocation, concrete hardware address, fixed memory container. |
| `{ ... }` | **Connotation** | Behavioral context, dynamic execution scope, conceptual state. |
| `< ... >` | **Invariant** | Immutable covenant, compile-time verified truth, permanent constraint. |
| `{ ... ]` | **Top-Down** | Imposition of form onto physical matter (e.g. allocating memory). |
| `[ ... }` | **Bottom-Up** | Emergence of meaning from physical silicon (e.g. reading a sensor). |
| `( ... )` | **Aside** | Cognitive intent, machine reasoning, and epistemic commentary. |

#### First-Class Cognitive Asides

Asides `( ... )` are not discarded comments! They are preserved in the AST (`AsideNode`), compiled into episodic memory scrolls (`.scroll`), and evaluated by the AI Partner (CPI-SI) to verify that the runtime actions align with the programmer's stated intent.

---

### Domain 2: Primitives & Polarity Affixes (`primitives.rs`)

Governs scalar arithmetic and the fundamental duality between abstract archetypes and concrete instances:

* **`ArithmeticNode`**: Binary mathematical operations (`+`, `-`, `*`, `/`). Division by zero is trapped as `QuadState::Halt` (NULL), preserving balanced equilibrium without crashing.
* **`UnaryNotNode`**: Algebraic truth inversion (`! expr` or `-- expr`).
* **`PolarityNode`**: Attaches an abstract (`+`) or concrete (`-`) bias to an identifier:
  * `+Cell`: The abstract blueprint or type specification of a Cell (zero-cost allocation).
  * `-Cell`: A physical, allocated, memory-backed instance of a Cell in silicon.
* **`BoundaryCastNode`**: Explicit transmutations between boundary domains (e.g. `[+]` or `{-}`).

---

### Domain 3: Relations, Directional Vectors & Connectives (`relations.rs`)

Governs the 3D computational coordinate system and formal logical connectives:

#### The Relational Matrix (`RelationNode`)

Evaluates state, identity, and similarity along the Ontological Y-Axis:

* **`=` (`BeingRelation::Is`)**: Exact identity or equivalence ($A = B$).
* **`!=` (`BeingRelation::IsNot`)**: Ontological distinction and inequality ("not to be").
* **`=!` (`BeingRelation::ToBeNot`)**: State predicate denial and negation ("is not" / to be not).
* **`=~` / `~=` (`BeingRelation::ToBeLike`)**: Structural analogy, homomorphism, or duck typing ("liken to").
* **`<=>` (`BeingRelation::Equivalence`)**: Bijective transduction across dual substrates (e.g. `65 <=> [ -1, +1, +1, -1, +1 ]`).
* **`<`, `>`, `<=`, `>=`**: Bounded comparisons across contexts and cell limits.

#### The Causal Directional Vectors (`DirectionalNode`)

Drives causal energy and data flow across time, scale, and spatial dimensions:

* **`-->` / `->` (`DirectionalVector::ActUpon`)**: Forward temporal causation and pipeline emission.
* **`<--` / `<-` (`DirectionalVector::ActAgain`)**: Retroactive feedback, retry, and transaction rollback.
* **`--^` / `-^` (`DirectionalVector::BuildsTo`)**: Ascending synthesis and architectural abstraction.
* **`--v` / `-v` (`DirectionalVector::BreaksDownFrom`)**: Descending decomposition and compiler lowering to IR.

#### The Connective Matrix (`ConnectiveNode`)

* **`::` (`Connective::And`)**: Cumulative simultaneous conjunction ($A \land B$).
* **`!` (`Connective::But`)**: Adversative caveat gate. If caveat $B$ fails, transaction $A$ is automatically rolled back.
* **`|` (`Connective::Or`)**: Disjunction with short-circuit evaluation.
* **`--` (`Connective::Not`)**: Balanced ternary truth inversion.
* **`:` (`Connective::InRelationTo`)**: Role classification and module import resolution.

---

### Domain 4: Structural Architecture (`structural.rs`)

Enforces the canonical 4-Block file architecture across the ecosystem:

* **`ProgramNode`**: Complete executable reel or tape stream.
* **`ModuleNode`**: Root module header declarations in `.omni` source reels.
* **`TapeNode`**: Process tape declarations in `.tape` operational files.
* **`SectionNode`**: The four canonical architectural blocks:
  1. **Block 1 (Metadata)**: Invariant declarations, author identity, and scripture witness.
  2. **Block 2 (Opening)**: Memory mounting, hardware locators, and bridge configurations.
  3. **Block 3 (Body)**: Sequential operational rolls driving state transformation.
  4. **Block 4 (Closing)**: Verification seals, tripwire tests, and commit gates.
* **`RollNode`**: An atomic procedural transaction: `[ ROLL N ] : [ Intent ] : Action ( Aside )`.
* **`CommitStatusNode`**: The closing gate asserting fourfold equilibrium (`PROCEED`, `AWAIT`, `ROLLBACK`, `HALT`).

---

### Domain 5: Cardinal Interrogatives (`cardinal.rs`)

Enables natural machine-operator dialogue and process tape headers through the Six Cardinal Interrogatives (`CardinalClauseNode`):

1. **`WHO`**: The Agent, Subject, Authority, or Module initiating the work.
2. **`WHAT`**: The Direct Object, Substance, or Data Payload being transformed.
3. **`WHEN`**: The Temporal Trigger, Clock Tick, or Execution Precondition.
4. **`WHERE`**: The Spatial Domain, Hardware Address, or Cell Space partition.
5. **`WHY`**: The Teleological Intent, Machine Reasoning, or Invariant Covenant.
6. **`HOW`**: The Modal Vector, Execution Status, or Velocity (+1, 0, NULL, -1).

---

### Domain 6: Operational SVO Verbs & Guards (`operational.rs`)

Models active computation as Subject-Verb-Object (SVO) sentences:

* **`VerbNode`**: Action execution: `Subject --> VERB Object = Target`:
  * **`INSCRIBE`**: Writes an external or source value into a target cell.
  * **`TRANSDUCE`**: Bijectively translates values across dual substrates (bits $\iff$ trits).
  * **`GUARD`**: Asserts invariant equilibrium under adversative caveats.
  * **`PROJECT`**: Emits internal memory state to an external display, terminal, or port.
  * **`SEQUENCE`**: Chains discrete atomic cells into a continuous text ribbon.
* **`GuardNode`**: Adversative checkpoint asserting state integrity before advancing.
* **`HardwareLocatorNode`**: MMIO bus binding: `Target ::@ Address` (e.g. `[ Terminal Stdout ] ::@ 0x01`).

---

### Domain 7: Ontological Typology, Analogies & Substrates (`ontological.rs`)

Defines the mathematical nature of types and memory substrates:

* **`InvariantTypeNode`**: Declarative definition of eternal types (e.g. `< Character >`, `< Number >`).
* **`EntityNode`**: Concrete typed instance bounded by an invariant type and physical substrate.
* **`AnalogyNode`**: Four-term proportional analogy (`A : B :: C : D`), the foundation of CPI-SI relational reasoning.
* **`SubstrateNode`**: Dual-substrate definition pairing physical binary silicon bits with virtual balanced ternary trits.
* **`ExemplarNode`**: Canonical mapping between integer scalars and balanced trits (`65 <=> [ -1, +1, +1, -1, +1 ]`). Includes automated verification via Horner power expansion.

---

## 4. The Dual-Rail Balanced Ternary Substrate

A cornerstone of OmniCode is the **dual-rail balanced ternary substrate**. In physical silicon, every virtual trit requires two binary physical rails (`pos_rail` and `neg_rail`) to encode four discrete truth states:

$$\mathbb{T}_4 = \{ \text{Await} (0), \text{Proceed} (+1), \text{Rollback} (-1), \text{Halt} (\text{NULL}) \}$$

```text
Rail Bit-Plane Layout (2 bits per virtual trit):
  pos_rail bit:  0   1   0   1
  neg_rail bit:  0   0   1   1
  -----------------------------
  Encoded State: 00  01  10  11
  Semantics:     0   +1  -1  NULL / Collision Trap
```

### Key Mathematical Properties in the AST Subsystem

1. **Zero-Cost SIMD Rail-Swap**: Inverting a balanced ternary state or register requires no arithmetic ALU operations; it is a single SIMD register swap between the positive and negative bit-planes:
   $$\text{Swap}(P^+, P^-) \implies +1 \longleftrightarrow -1, \quad 0 \longleftrightarrow 0, \quad \text{NULL} \longleftrightarrow \text{NULL}$$
2. **Instant Hardware Collision Detection**: If both positive and negative rails are asserted simultaneously (`pos & neg != 0`), the hardware immediately traps the collision as a `NULL` fault, preventing illegal memory corruption.

---

## 5. State vs. Literal Disambiguation

The AST strictly distinguishes between **Equilibrium States** and **Literal Numeric Values**:

* **Equilibrium State (`0:AWAIT`)**: Represents dynamic neutrality, asynchronous standby, or an idling execution thread. Represented in the AST as `ASTNode::Ternary(QuadState::Await)`.
* **Air-Quoted Literal Value (`"0"` or `{ 0 }`)**: Represents the discrete scalar number zero ($0.0$). Represented in the AST as `ASTNode::Number(0.0)` or enclosed within a boundary.
* **Atomic Glyph (`'0'`)**: Represents the discrete 1-Cell character rune (ASCII 48 / trits `[0, 1, -1, -1, 1]`).

---

## 6. Readiness for the Next Modularization Phase

With the completion of the `universal/`, `character/`, and `number/` type modularizations in Track 2, the AST subsystem is now fully equipped with:

* Support for the complete connective and relation matrix (`::`, `!`, `|`, `--`, `!=`, `=!`, `=~`, `:`).
* First-class dual-rail substrate modeling and packing topologies in `SubstrateNode`.
* Horner-polynomial verification and zero-sum balance tracking in `ExemplarNode`.
* Reversible state kinematics and rail-swap negation in `QuadState`.

### Why We Hold Off on the Container Subsystem

The `container` type (which defines storehouses, arrays, queues, and tree structures) is an **aggregate vessel**. In accordance with Proverbs 24:27 (*"Prepare thy work without, and make it fit for thyself in the field; and afterwards build thine house"*), a vessel cannot be soundly constructed until every atomic constituent it holds is fully proven:

1. **Universal Substrate**: Dual-rail balanced ternary foundation (*Proven*).
2. **Character**: Atomic 1-Cell glyph (*Proven*).
3. **Number**: Atomic 1-Word scalar (*Proven*).
4. **State / Text / Site**: Primitive execution states, ribbons of characters, and spatial addresses (*Next Phase*).
5. **Container**: The overarching vessel holding all of the above (*Final Aggregation Phase*).

This systematic modularization ensures zero circular dependencies and guarantees that each layer of the language rests upon an unshakeable bedrock of proven invariants.

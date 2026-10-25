# Transducer Architecture: Document 03

## Continuous SVO Directional Vectors & Parity Gates

---

### 1. The Linguistic Isomorphism of SVO

In the OmniCode Framework, Subject-Verb-Object (SVO) is not an arbitrary grammatical
convention. It is a direct mapping of physical causality:

* **Subject**: The Actor, Inception Origin, or Source Vessel (`Noun Phrase`).
* **Verb**: The Directional Vector or Relation of Being (`Directional Vector`).
* **Object**: The Recipient, Target Boundary, or Payload (`Noun Phrase`).
* **Preposition**: Spatial Scoping or Partition Container (`v--`, `@`, `:`).
* **Conjunction**: Invariant Caveat Guard or Chained Pipeline (`!`, `::`).
* **Interjection**: Parenthetical Aside Intent or Hardware Interrupt (`(...)`, `HALT`).

### 2. Directional Operators Spectrum

OmniCode directional vectors represent physical state motion:

| Vector | Symbol | Computational Semantics | Natural Linguistics |
| :--- | :--- | :--- | :--- |
| **Invoke** | `-->` | Forward execution / flow / store | sends, transmits, calls |
| **Assign** | `<--` | Inbound reception / assignment / load | receives, fetches, binds |
| **Emit** | `^--` | Upward synthesis / type promotion | emits, extracts, outputs |
| **Store** | `v--` | Downward storage / vessel reservation | stores, partitions, keeps |
| **Equals** | `=` | Equilibrium copula / balance | is, equals, balances |
| **NotEquals** | `!=` | Differentiation / exclusion | differs, diverges |
| **Resembles** | `~=` | Analogy / duck typing | resembles, mirrors |

### 3. Castable Operators

In conventional grammar parsers, only verbs carry operational power. In OmniCode,
**Subjects and Objects can be cast to operators**:

* An agent (Subject) can be an active executable transform.
* A filter or operation (Object) can be passed as an operand or closure.

### 4. The Two-Way Checksum Parity Gate

Before any synthesized reel is passed to the virtual machine, it is certified
by `RestoratorCertifier::evaluate_two_way_checksum`:

1. **Forward Checksum**: Verifies that the reel's structural tokens match the
   original prompt's intent.
2. **Reverse Checksum**: Ensures all bracket boundaries (`[...]`, `<...>`, `{...}`)
   conform to Rank 1 invariants.
3. **Autonomic Self-Healing**: If a boundary mismatch occurs, the `Restorator`
   intervenes to repair delimiters automatically before execution.

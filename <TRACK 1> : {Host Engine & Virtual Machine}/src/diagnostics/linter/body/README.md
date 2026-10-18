# Linter Body Subsystem (`diagnostics::linter::body`)

> **"From whom the whole body fitly joined together and compacted by that which every joint supplieth, according to the effectual working in the measure of every part, maketh increase of the body unto the edifying of itself in love."**
> — *Colossians 2:19 (KJV)*

> **"For as the body is one, and hath many members, and all the members of that one body, being many, are one body: so also is Christ."**
> — *1 Corinthians 12:12 (KJV)*

---

## 1. Overview & Purpose

The **Body Subsystem** ([`src/diagnostics/linter/body`](file:///home/professorseanex/OmniCode/%3CTRACK%201%3E%20:%20%7BHost%20Engine%20&%20Virtual%20Machine%7D/src/diagnostics/linter/body)) audits **BLOCK 3: BODY** across all `.omni` specifications and `.tape` execution streams.

Block 3 contains the **living substance** of an OmniCode program. In specifications (`.omni`), it defines types, structures, records, bit rails (`Bit_0`..`Bit_7`), numeric representations, and semantic invariants. In execution streams (`.tape`), it contains sequential rolls, state transitions, arithmetic operations, and register transformations.

---

## 2. Block 3 Specification Requirements

### For `.omni` Specifications

- **Header**: Must begin with `[BLOCK 3: BODY]` or a valid comment header.
- **Structural Definitions**: Types, Enums, Bitfields, and Record layouts.
- **Syntactic Boundaries**: Proper nesting of brackets `[...]`, braces `{...}`, and asymmetric pairs (`[...}` / `{...]`).
- **Identifier Conventions**: Canonical snake_case for fields, TitleCase for types, and recognized exemptions for hardware bit rails (`Bit_0`..`Bit_7`).

### For `.tape` Execution Streams

- **Instruction Rolls**: Ordered steps or rolls executing state transitions.
- **Subject-Verb-Object Grammar**: Valid operations using standard verbs (`SET`, `DEFINE`, `CALCULATE`, `EMIT`, `VERIFY`, `APPLY`).
- **Binary & Numeric Literals**: Valid prefixing (e.g. `0b00101010`, `0x2A`, or integer values).

---

## 3. Scoring & Audit Rules

The body linter computes an operational score from 0.0 to 100.0:

- **Missing Header (`BLOCK 3: BODY`)**: -40 points.
- **Trivial / Empty Body**: -30 points (Block 3 must contain meaningful type definitions or instruction rolls).
- **Unclosed Delimiters in Body**: -15 points per occurrence.
- **Invalid Grammar Invariants**: -5 to -10 points per syntax issue.

---

## 4. Telemetry & Diagnostic Logger Integration

Because Block 3 contains the bulk of the logic, it is the primary generator of diagnostic telemetry:

- **Boundary Anomaly Failpoints**: Directly triggers `preflight` stack analysis and feeds remediation suggestions into the [`DiagnosticLogger`](file:///home/professorseanex/OmniCode/%3CTRACK%201%3E%20:%20%7BHost%20Engine%20&%20Virtual%20Machine%7D/src/diagnostics/logger/mod.rs).
- **Execution Step Telemetry**: In `.tape` files, roll indices are checked for monotonicity and recorded in the logger's `FlightRecorder`.
- **AI Observation Generation**: Syntax patterns in the body are recorded in episodic memory for AI reflection and auto-completion.

---

## 5. Verification & Tests

The `body` module includes 4 unit tests verifying:

- Canonical `number.omni` Block 3 definitions (scores 100.0).
- Canonical `number.tape` Block 3 instructions (scores 100.0).
- Detection of empty body blocks and malformed syntax.

Run module tests:

```bash
cargo test --lib diagnostics::linter::body
```

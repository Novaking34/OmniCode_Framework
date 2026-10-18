# Linter Syntax Subsystem (`diagnostics::linter::syntax`)

> **"Let all things be done decently and in order."**
> — *1 Corinthians 14:40 (KJV)*

> **"Remove not the ancient landmark, which thy fathers have set."**
> — *Proverbs 22:28 (KJV)*

---

## 1. Overview & Purpose

The **Syntax Subsystem** ([`src/diagnostics/linter/syntax`](file:///home/professorseanex/OmniCode/%3CTRACK%201%3E%20:%20%7BHost%20Engine%20&%20Virtual%20Machine%7D/src/diagnostics/linter/syntax)) constitutes Pass 3 of the OmniCode linting pipeline. It verifies the grammatical, spatial, and typographical conventions of OmniCode source code.

OmniCode enforces a strict, mathematical harmony across space and grammar:

1. **Grammatical Order**: Operations follow deterministic Subject-Verb-Object (SVO) alignment.
2. **Spatial Boundaries**: Lines must respect the 120-column spatial boundary; tabs are forbidden in favor of 2 or 4-space indentations.
3. **Identifier Morphology**: Identifiers within brackets must follow snake_case, with explicit canonical exemptions for hardware bit rails (`Bit_0`..`Bit_7`) and binary numeric literals (`0b`).

---

## 2. Component Architecture

```
syntax/
├── README.md        <-- This document
├── mod.rs           <-- Syntax coordinator & report aggregator
├── grammar.rs       <-- SVO verb validation, keyword rules, statement structure
└── spatial.rs       <-- 120-column limits, tab detection, bracket identifier linting
```

### Module Responsibilities

- **[`grammar.rs`](file:///home/professorseanex/OmniCode/%3CTRACK%201%3E%20:%20%7BHost%20Engine%20&%20Virtual%20Machine%7D/src/diagnostics/linter/syntax/grammar.rs)**:
  - Audits imperative and declarative verbs in `.tape` and `.omni` code.
  - Recognizes canonical SVO verbs: `DEFINE`, `SET`, `EMIT`, `VERIFY`, `APPLY`, `PARTITION`, `CALCULATE`, and `CLOSE`.
  - Flags unrecognizable or malformed instruction sequences.
- **[`spatial.rs`](file:///home/professorseanex/OmniCode/%3CTRACK%201%3E%20:%20%7BHost%20Engine%20&%20Virtual%20Machine%7D/src/diagnostics/linter/syntax/spatial.rs)**:
  - Enforces the hard 120-character line length limit.
  - Flags tab characters (`\t`), requiring pure space indentation.
  - Validates bracketed identifiers: enforces `[snake_case]` conventions, while properly allowing canonical bit identifiers (`Bit_0`, `Bit_1`, etc.) and binary literals (`0b00101100`).

---

## 3. Grammar & Spatial Rules

### Valid SVO Verbs in OmniCode

| Verb | Category | Purpose | Canonical Example |
| :--- | :--- | :--- | :--- |
| `DEFINE` | Declarative | Type, schema, or constant declaration | `DEFINE Type Character` |
| `SET` | Mutative | Register or variable assignment | `SET Register_0 = 0x41` |
| `CALCULATE` | Transform | Arithmetic or logical transformation | `CALCULATE Result = A + B` |
| `PARTITION` | Structural | Bit slicing or memory segmentation | `PARTITION Register_0 INTO [Bit_0 .. Bit_7]` |
| `EMIT` | Telemetry | Output generation or event broadcast | `EMIT Event::CharProcessed` |
| `VERIFY` | Diagnostic | Invariant or contract assertion | `VERIFY Bit_7 == 0` |
| `APPLY` | Pipeline | Function application or transform pass | `APPLY Transform::Mask` |
| `CLOSE` | Terminal | Block or session termination | `CLOSE Block` |

### Identifier Exemptions in Brackets

- Standard identifiers inside brackets must be lower snake_case: `[valid_identifier]`.
- **Bit Rails**: Hardware bit identifiers `Bit_0`, `Bit_1`, `Bit_2`, `Bit_3`, `Bit_4`, `Bit_5`, `Bit_6`, `Bit_7` (from `character.omni`) are explicitly exempted.
- **Numeric & Binary Literals**: Literals like `0b00101100`, `0x2A`, and numbers are recognized as values and exempted from identifier case rules.

---

## 4. Telemetry & Diagnostic Logger Integration

- Dispatches `SyntaxGrammarAnomaly` or `SyntaxSpatialAnomaly` failpoints to the [`DiagnosticLogger`](file:///home/professorseanex/OmniCode/%3CTRACK%201%3E%20:%20%7BHost%20Engine%20&%20Virtual%20Machine%7D/src/diagnostics/logger/mod.rs).
- Generates `FixSuggestion` instances for tabs (suggesting space replacement) and long lines (suggesting line break locations).
- Flags violations in episodic memory so automated generation scripts produce compliant code.

---

## 5. Verification & Tests

The `syntax` module includes 7 unit tests verifying:

- Valid SVO verbs and grammar structures.
- Spatial line length boundary checking (<= 120 chars).
- Bracket identifier case validation with `Bit_` and `0b` exemptions.
- Tab rejection and replacement suggestions.

Run module tests:

```bash
cargo test --lib diagnostics::linter::syntax
```

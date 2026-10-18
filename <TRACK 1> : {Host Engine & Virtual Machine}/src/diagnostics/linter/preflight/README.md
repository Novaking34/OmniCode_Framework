# Linter Preflight Subsystem (`diagnostics::linter::preflight`)

> **"For which of you, intending to build a tower, sitteth not down first, and counteth the cost, whether he have sufficient to finish it?"**
> — *Luke 14:28 (KJV)*

> **"According to all that I shew thee, after the pattern of the tabernacle, and the pattern of all the instruments thereof, even so shall ye make it."**
> — *Exodus 25:9 (KJV)*

---

## 1. Overview & Purpose

The **Preflight Subsystem** ([`src/diagnostics/linter/preflight`](file:///home/professorseanex/OmniCode/%3CTRACK%201%3E%20:%20%7BHost%20Engine%20&%20Virtual%20Machine%7D/src/diagnostics/linter/preflight)) constitutes Pass 1 of the OmniCode linting pipeline.

Before any AST parsing, structural decomposition, or semantic evaluation can begin, the preflight analyzer performs a rapid, linear-time scan of the raw text stream. Its primary mission is to detect **boundary crossings, unclosed delimiters, and illegal nesting**.

Critically, when a syntax or delimiter defect is detected, the Preflight Subsystem does not merely raise an error—it constructs a mathematically sound **automated remediation** ([`FixSuggestion`](file:///home/professorseanex/OmniCode/%3CTRACK%201%3E%20:%20%7BHost%20Engine%20&%20Virtual%20Machine%7D/src/diagnostics/logger/failpoint/remediation.rs)) and dispatches it directly to the [`DiagnosticLogger`](file:///home/professorseanex/OmniCode/%3CTRACK%201%3E%20:%20%7BHost%20Engine%20&%20Virtual%20Machine%7D/src/diagnostics/logger/mod.rs).

---

## 2. Component Architecture

```bash
preflight/
├── README.md        <-- This document
├── mod.rs           <-- Preflight coordinator & line-audit engine
└── assembly.rs      <-- Delimiter stack tracker, boundary matcher, & FixSuggestion generator
```

---

## 3. Boundary & Delimiter Mechanics

OmniCode features a multi-tiered boundary algebra governing paired tokens:

### Standard Symmetric Boundaries

- Parentheses `( ... )`: Expression grouping and mathematical precedence.
- Brackets `[ ... ]`: Structural blocks, index bounds, and identifier encapsulation.
- Braces `{ ... }`: Scopes, records, and execution rolls.
- Chevrons `< ... >`: Type parameters and semantic ontology rankings.

### Permitted Asymmetric Boundaries

OmniCode language specifications explicitly recognize two sacred asymmetric boundary patterns:

1. **`[ ... }` (Bottom-Up Emergence, Rank 4)**: Represents inductive emergence, where a component originates inside a bounded domain and emerges into a higher-order scope.
2. **`{ ... ]` (Top-Down Imposition, Rank 5)**: Represents deductive constraint, where higher-order policy is impressed upon a concrete register or data structure.

### Illegal Crossings

All other mismatched pairs (e.g., `< ... ]` or `{ ... >`) are strictly illegal. When encountered, the delimiter stack flags an immediate boundary mismatch.

---

## 4. Remediation & Diagnostic Logger Telemetry

When `preflight_with_logger` identifies a delimiter error, it dispatches structured failpoint events:

```rust
pub fn preflight_with_logger(
    source: &str,
    file_path: &str,
    logger: &DiagnosticLogger,
) -> PreflightReport
```

### Remediation Kinds Dispatched

1. **`FixKind::FixBoundaryMismatch`**:
   - Generated when an illegal delimiter crossing occurs (e.g., `<` closed with `]`).
   - The fix specifies the line, column, target token (`]`), suggested replacement (`>`), and rationale: `"Replace mismatched delimiter to match opening '<' from line X"`.
2. **`FixKind::CloseBoundary`**:
   - Generated when an opening delimiter remains unclosed at EOF (e.g., `{` left unclosed).
   - The fix specifies the closing token (`}`) and insertion position at the end of the unclosed block.

### The Autonomic Repair Loop

These remediations are consumed by the [`Restorator`](file:///home/professorseanex/OmniCode/%3CTRACK%201%3E%20:%20%7BHost%20Engine%20&%20Virtual%20Machine%7D/src/diagnostics/restorator/mod.rs) and [`OmniDebugger`](file:///home/professorseanex/OmniCode/%3CTRACK%201%3E%20:%20%7BHost%20Engine%20&%20Virtual%20Machine%7D/src/diagnostics/debugger/mod.rs) to automatically heal code without human intervention.

---

## 5. Verification & Tests

The `preflight` module includes 7 unit tests verifying:

- Clean passes on balanced symmetric and valid asymmetric boundaries (`[ ... }` and `{ ... ]`).
- Detection and accurate `FixSuggestion` generation for illegal crossings (`< ... ]`).
- Detection and accurate `FixSuggestion` generation for unclosed delimiters at EOF.
- Real-time telemetry broadcasting through `DiagnosticLogger`.

Run module tests:

```bash
cargo test --lib diagnostics::linter::preflight
```

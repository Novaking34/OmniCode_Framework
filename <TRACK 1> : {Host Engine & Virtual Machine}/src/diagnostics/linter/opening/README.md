# Linter Opening Subsystem (`diagnostics::linter::opening`)

> **"The entrance of thy words giveth light; it giveth understanding unto the simple."**
> — *Psalm 119:130 (KJV)*

> **"Open to me the gates of righteousness: I will go into them, and I will praise the LORD."**
> — *Psalm 118:19 (KJV)*

---

## 1. Overview & Purpose

The **Opening Subsystem** ([`src/diagnostics/linter/opening`](file:///home/professorseanex/OmniCode/%3CTRACK%201%3E%20:%20%7BHost%20Engine%20&%20Virtual%20Machine%7D/src/diagnostics/linter/opening)) audits **BLOCK 2: OPENING** across `.omni` and `.tape` files.

Block 2 represents the **entry gate and environment initialization** of an OmniCode translation unit. It sets up namespaces, resolves external dependencies, declares imported tapes, establishes seed registers, and configures runtime invariants prior to state allocation or instruction dispatch.

---

## 2. Block 2 Specification Requirements

In canonical files such as `character.omni`, `character.tape`, `number.omni`, and `number.tape`, `BLOCK 2: OPENING` fulfills specific initialization contracts:

### For `.omni` Specifications

- **Namespace Decl**: Declarations specifying the semantic domain (e.g. `ideas/types/number`).
- **External Imports**: Explicit imports of prerequisite schemas (e.g. `import core::math;`).
- **Type Aliases & Seeds**: Base register and enum declarations needed by the body.

### For `.tape` Execution Streams

- **Tape Binding**: Binding to target specification tapes (e.g. `USE tape::number_spec`).
- **Register Preconditioning**: Zeroing or seeding computational registers (`INIT ACCUMULATOR = 0`).
- **Environment Invariants**: Verification of runtime prerequisites.

---

## 3. Scoring & Audit Rules

The opening linter scores Block 2 from 0.0 to 100.0:

- **Missing Header (`BLOCK 2: OPENING`)**: -40 points.
- **Empty Opening Block**: -30 points (an opening block must declare at least imports, bindings, or a valid pass statement).
- **Unresolved Import Syntax**: -10 points per syntax issue.
- **Malformed Dependency Clauses**: -10 points per violation.

---

## 4. Telemetry & Diagnostic Logger Integration

When Block 2 fails linting:

- Dispatches `OpeningBlockAnomaly` to the [`DiagnosticLogger`](file:///home/professorseanex/OmniCode/%3CTRACK%201%3E%20:%20%7BHost%20Engine%20&%20Virtual%20Machine%7D/src/diagnostics/logger/mod.rs).
- Emits a remediation proposal providing canonical import or binding templates.
- Records telemetry ensuring dependent compilation passes abort if the environment setup is incomplete.

---

## 5. Verification & Tests

The `opening` module includes 4 unit tests verifying:

- Canonical `number.omni` Block 2 structure (scores 100.0).
- Canonical `number.tape` Block 2 structure (scores 100.0).
- Handling of empty or missing opening declarations with appropriate warnings.

Run module tests:

```bash
cargo test --lib diagnostics::linter::opening
```

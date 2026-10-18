# Linter Metadata Subsystem (`diagnostics::linter::metadata`)

> **"And he gave unto Moses, when he had made an end of communing with him upon mount Sinai, two tables of testimony, tables of stone, written with the finger of God."**
> — *Exodus 31:18 (KJV)*

> **"He telleth the number of the stars; he calleth them all by their names."**
> — *Psalm 147:4 (KJV)*

---

## 1. Overview & Purpose

The **Metadata Subsystem** ([`src/diagnostics/linter/metadata`](file:///home/professorseanex/OmniCode/%3CTRACK%201%3E%20:%20%7BHost%20Engine%20&%20Virtual%20Machine%7D/src/diagnostics/linter/metadata)) audits **BLOCK 1: METADATA** of all OmniCode files (`.omni` and `.tape`).

In the OmniCode philosophy, code without explicit identity, provable authorship, and foundational ethical/spiritual grounding is untrusted and unprovable. Block 1 establishes the provenance, semantic ontology, versioning epoch, and sacred witness of the code unit before any execution or type definition is evaluated.

---

## 2. Block 1 Specification Requirements

A canonical `BLOCK 1: METADATA` must include the following attributes, demonstrated in `character.omni` and `number.omni`:

| Field Name | Description | Required | Example |
| :--- | :--- | :---: | :--- |
| **Header** | Block identification tag | Yes | `[BLOCK 1: METADATA]` or `// BLOCK 1: METADATA` |
| **Title / Module** | Formal name of the specification | Yes | `Title: Canonical Character Specification` |
| **Author / Signer** | Responsible creator or authority | Yes | `Author: OmniCode Framework Authors` |
| **Epoch / Version** | Temporal epoch and semantic version | Yes | `Epoch: 2026.1 / Version: 1.0.0` |
| **License / Law** | Legal or open framework governance | Yes | `License: MIT / Apache-2.0` |
| **Witness (KJV)** | Foundational Scripture citation | Yes | `Witness: Proverbs 11:1 (KJV)` |

---

## 3. Scoring & Audit Rules

The metadata linter computes a score between 0.0 and 100.0:

- **Missing Header**: -40 points.
- **Missing Title / Module**: -15 points.
- **Missing Author**: -15 points.
- **Missing Epoch / Version**: -10 points.
- **Missing License**: -10 points.
- **Missing Scripture Witness**: -10 points.

A file with all required fields scores `100.0/100.0` and satisfies the `QuadStateGate::Proceed` criterion for Block 1.

---

## 4. Telemetry & Diagnostic Logger Integration

When metadata elements are missing or malformed:

- A `MetadataAnomaly` failpoint is emitted to the [`DiagnosticLogger`](file:///home/professorseanex/OmniCode/%3CTRACK%201%3E%20:%20%7BHost%20Engine%20&%20Virtual%20Machine%7D/src/diagnostics/logger/mod.rs).
- A remediation suggestion is created specifying the missing fields with canonical skeleton text.
- An episodic `CognitiveNote` records the violation so future generation passes automatically include the required metadata headers.

---

## 5. Verification & Tests

The `metadata` module includes 4 unit tests verifying:

- Canonical `number.omni` Block 1 metadata (scores 100.0).
- Canonical `number.tape` Block 1 metadata (scores 100.0).
- Detection of missing title, author, or witness fields with precise score degradation.

Run module tests:

```bash
cargo test --lib diagnostics::linter::metadata
```

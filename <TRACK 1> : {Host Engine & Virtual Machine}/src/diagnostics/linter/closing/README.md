# Linter Closing Subsystem (`diagnostics::linter::closing`)

> **"I am Alpha and Omega, the beginning and the end, the first and the last."**
> — *Revelation 22:13 (KJV)*

> **"When Jesus therefore had received the vinegar, he said, It is finished: and he bowed his head, and gave up the ghost."**
> — *John 19:30 (KJV)*

---

## 1. Overview & Purpose

The **Closing Subsystem** ([`src/diagnostics/linter/closing`](file:///home/professorseanex/OmniCode/%3CTRACK%201%3E%20:%20%7BHost%20Engine%20&%20Virtual%20Machine%7D/src/diagnostics/linter/closing)) audits **BLOCK 4: CLOSING** across all `.omni` and `.tape` files.

In OmniCode, execution does not simply trail off into undefined memory. Every module must reach a provable, deterministic **terminus**. Block 4 provides the terminal sealing of the translation unit: executing integrity checks, verifying that all opened boundaries have been balanced, confirming register invariants, tearing down ephemeral resources, and sealing the unit with a final witness.

---

## 2. Block 4 Specification Requirements

In canonical files such as `character.omni`, `character.tape`, `number.omni`, and `number.tape`, `BLOCK 4: CLOSING` enforces:

### For `.omni` Specifications

- **Header**: Must begin with `[BLOCK 4: CLOSING]` or a canonical comment header.
- **Invariant Proofs**: Declarations verifying that data structures adhere to their algebraic properties (e.g. `proof: bitwidth == 8`).
- **Terminal Witness**: Concluding statement sealing the specification.

### For `.tape` Execution Streams

- **Terminal State Assertion**: Verification that the execution reached an expected state (e.g. `VERIFY state == SUCCESS`).
- **Cleanup & Register Teardown**: Releasing temporary registers or clearing sensitive flags.
- **QuadState Gate Finalization**: Emitting the final `CLOSE` or `HALT` instruction.

---

## 3. Scoring & Audit Rules

The closing linter computes a terminal score from 0.0 to 100.0:

- **Missing Header (`BLOCK 4: CLOSING`)**: -40 points.
- **Empty Closing Block**: -30 points (must contain proofs, cleanup, or terminal assertions).
- **Unverified Invariants**: -10 points.
- **Unsealed Final Boundary**: -10 points.

---

## 4. Telemetry & Diagnostic Logger Integration

- **Terminal Audit Emission**: Block 4 evaluation generates the final diagnostic summary sent to the [`DiagnosticLogger`](file:///home/professorseanex/OmniCode/%3CTRACK%201%3E%20:%20%7BHost%20Engine%20&%20Virtual%20Machine%7D/src/diagnostics/logger/mod.rs).
- **RollFrame Sealing**: The final pass frame (Roll 6/6) in the logger's `FlightRecorder` is stamped based on the successful closure of Block 4.
- **QuadState Resolution**: If Block 4 is missing or unsealed, the `QuadStateGate` is downgraded to `Rollback`, preventing corrupted code from proceeding to the virtual machine.

---

## 5. Verification & Tests

The `closing` module includes 4 unit tests verifying:

- Canonical `number.omni` Block 4 terminal proofs (scores 100.0).
- Canonical `number.tape` Block 4 terminal assertions (scores 100.0).
- Detection of missing or empty closing blocks with appropriate score penalties.

Run module tests:

```bash
cargo test --lib diagnostics::linter::closing
```

# Volume 4: Cognitive & Autonomous Systems

## Document 03: The Restorator & Autonomous Self-Healing Diagnostics

> **An Autonomous Linguistic Immune System That Repairs Syntax, Delimiter Morphology, and Structural Anomalies.**

Conventional compilers are passive and punitive: upon encountering a missing delimiter or an unclosed block, they stop immediately with an error message and exit.

In contrast, OmniCode includes an **active diagnostic immune system**:

1. [`Restorator`](file:///home/professorseanex/OmniCode/src/diagnostics/restorator.rs): Proactively repairs broken source files, balances mismatched delimiters, and scaffolds missing architectural blocks.
2. [`OmniDebugger`](file:///home/professorseanex/OmniCode/src/diagnostics/debugger.rs): Maintains execution snapshots and rolls back environment state when `-1:ROLLBACK` or caveat vetoes are triggered.
3. [`DiagnosticLogger`](file:///home/professorseanex/OmniCode/src/diagnostics/logger.rs): Tracks failpoints with structured taxonomy (`FP_BOUNDARY_LEAK`, `FP_IMMUTABLE_MUTATION`, `FP_ROLLBACK_TRIGGERED`, etc.).

---

### 1. Repair Hierarchy & Morphological Preconditions

The Restorator evaluates problems along a 4-level hierarchy:

| Level | Kind | Trigger Condition | Autonomous Action |
| :---: | :--- | :--- | :--- |
| **L0** | `FormatOnly` | Inconsistent whitespace or line endings | Normalizes indentation and block separation. |
| **L1** | `ImportInjection` | Standard keywords used without module imports | Injects `import: [ Core.Arithmetic :: Core.Logic :: Core.Vectors ]`. |
| **L2** | `BoundaryBalance` | Unclosed `[`, `<`, `{` delimiters or rank crossing | Closes unclosed delimiters respecting morphological hierarchy. |
| **L3** | `BlockScaffold` | Missing `Metadata`, `Opening`, `Body`, or `Closing` | Auto-scaffolds missing 4-block headers and wraps expressions. |

---

### 2. Repair Algorithms & Operational Mechanics

#### Autonomous Boundary Balancing Algorithm

When a file contains unclosed delimiters (e.g., `state [ +1`), the Restorator scans the token stream:

1. Pushes opening delimiters onto a morphological balance stack.
2. If the stack is non-empty at EOF or before a closing block boundary, pops unclosed delimiters in reverse order.
3. Appends the corresponding closing delimiter (`]`, `>`, `}`) with appropriate rank preservation.

#### Autonomous Structural 4-Block Scaffolding

If raw code is provided without block demarcations:

```omni
x [ 10 ]
y [ 20 ]
z [ x + y ]
```

The Restorator automatically transforms it into:

```omni
# ==============================================================================
# BLOCK 1: METADATA BLOCK
# ==============================================================================
Module < AutoRestored > ( Autonomously healed module ) {
    version: < "1.0.0" >,
    system: < OmniCode.Core >
}

# ==============================================================================
# BLOCK 2: OPENING BLOCK
# ==============================================================================
Opening {
    import: [ Core.Arithmetic :: Core.Logic :: Core.Vectors ],
    preconditions: [ +1 ]
}

x [ 10 ]
y [ 20 ]

# ==============================================================================
# BLOCK 3: BODY BLOCK
# ==============================================================================
Body [
    z [ x + y ]
]

# ==============================================================================
# BLOCK 4: CLOSING BLOCK
# ==============================================================================
Closing {
    verify: [ +1 ],
    commit_status: [ +1 ]
}
```

#### CLI Command: `omnicode restore`

```bash
# Preview repairs (dry-run mode)
cargo run -- restore examples/draft_file.omni

# Apply repairs directly to the file
cargo run -- restore examples/draft_file.omni --write
```

#### Transactional Rollback: `OmniDebugger`

During execution, mutating statements take an environment snapshot before performing writes:

* If the result evaluates to `-1:ROLLBACK` or a caveat `! [ condition ]` evaluates to false, the debugger restores the snapshot.
* **Cyclic Retroaction Protection:** Tracks repeated transitions back to the same snapshot. If cycles exceed the threshold (default: 10), halts execution with `FP_CYCLIC_RETROACTION` to prevent infinite loops.

---

### 3. Verification & Invariant Proofs

1. **Non-Destructive Guarantee:** In dry-run mode, files are never modified without the `--write` flag.
2. **Linter Re-Verification:** After a file is repaired, the Restorator passes it through [`OmniLinter`](file:///home/professorseanex/OmniCode/src/diagnostics/linter.rs) to prove that the resulting code is 100% compliant before committing to disk.

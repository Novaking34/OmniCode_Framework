# ==============================================================================
# BLOCK 1: METADATA BLOCK
# ( Extension Identity, Scope & Supported IDE Platforms )
# ==============================================================================

# OmniCode VS Code & IDE Extension

> **Official Language Support Package for Visual Studio Code, Cursor, and Google Antigravity IDE.**

Provides morphological boundary colorization, 3D coordinate operator grammar, balanced ternary literals, and universal 4-Block file templates.

---

# ==============================================================================
# BLOCK 2: OPENING BLOCK
# ( Extension Topology, Package Manifest & Installation Preconditions )
# ==============================================================================

## Extension File Topology

```text
editors/vscode/
├── package.json                   # Extension manifest (v0.1.2, .omni & .scroll associations)
├── language-configuration.json    # Brackets, auto-closing pairs, comment syntax
├── snippets/
│   └── omnicode.json             # 4-Block auto-population snippets (omni-file & omni-scroll)
└── syntaxes/
    └── omnicode.tmLanguage.json  # TextMate syntax grammar scopes & keyword rules
```

### Installation

```bash
# Package the VSIX extension
npx -y @vscode/vsce package --allow-missing-repository

# Install into Antigravity IDE / VS Code
code --install-extension editors/vscode/omnicode-language-0.1.2.vsix
```

---

# ==============================================================================
# BLOCK 3: BODY BLOCK
# ( Syntax Grammar, Morphological Boundary Physics & Snippet Scaffolding )
# ==============================================================================

## Core Capabilities

### 1. Morphological Boundary Highlighting

Visually distinguishes all six ontological boundary types to eliminate bracket blindness:

* `[ ... ]` : **Denotation** (Fixed definitions, physical registers)
* `< ... >` : **Invariant** (Immutable axioms, process types)
* `{ ... }` : **Connotation** (Ambient context, scoped universes)
* `{ ... ]` : **Top-Down** (Context dictates definition / Absorber)
* `[ ... }` : **Bottom-Up** (Definition emerges from context / Generator)
* `( ... )` : **Aside** (Machine-readable intent and AI reasoning traces)

### 2. 3D Coordinate Operator Colorization

* **Y-Axis (Being/Identity):** `=`, `!=`, `~=` / `≈`
* **X-Axis (Dynamics/Causality):** `-->` (Act upon forward), `<--` (Retroactive feedback/rollback)
* **Z-Axis (Scale/Hierarchy):** `^--` (Builds to / Layer up), `v--` (Breaks down from / Layer down)
* **Connectives:** `!` (Adversative caveat), `::` (And), `:` (In relation to), `|` (Or), `--` (Not)

### 3. Balanced Ternary Truth Literals

* `+1`, `YES`, `PROCEED` (Monotonic commit)
* `0`, `OK`, `AWAIT` (Asynchronous yield / equilibrium)
* `-1`, `NO`, `ROLLBACK` (State unwinding)
* `NULL`, `VOID`, `HALT` (Terminal trap)

### 4. Universal 4-Block Auto-Population Snippets

* `omni-file` / `omni-4block`: Complete compliant `.omni` module template with interactive tab stops.
* `omni-scroll` / `scroll`: Complete compliant `.scroll` episodic trace and learning template.
* `block-1`: Scaffolds Block 1: Metadata Block.
* `block-2`: Scaffolds Block 2: Opening Block.
* `block-3`: Scaffolds Block 3: Body Block.
* `block-4`: Scaffolds Block 4: Closing Block.

---

# ==============================================================================
# BLOCK 4: CLOSING BLOCK
# ( Packaging Verification & Extension Status Invariants )
# ==============================================================================

### Verified Extension State

* **Language ID:** `omnicode`
* **File Associations:** `*.omni`, `*.scroll`
* **Grammar Version:** `0.1.2`
* **Editor Integration:** Tested on VS Code 1.75+ and Antigravity IDE.

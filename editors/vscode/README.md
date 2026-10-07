# OmniCode VS Code & IDE Extension

## `editors/vscode/`

This folder contains the official extension package for the **OmniCode Language**, providing syntax highlighting, bracket colorization, language configuration, and universal 4-Block file templates for VS Code, Cursor, and the Antigravity IDE.

---

## Features

### 1. Morphological Boundary Highlighting

Visually distinguishes all six ontological boundary types to eliminate bracket blindness:

* `[ ... ]` : **Denotation** (Mutable definitions)
* `< ... >` : **Invariant** (Immutable axioms and constants)
* `{ ... }` : **Connotation** (Ambient context and environment)
* `{ ... ]` : **Top-Down** (Context dictates definition)
* `[ ... }` : **Bottom-Up** (Definition emerges from context)
* `( ... )` : **Aside** (Machine-readable intent and AI reasoning traces)

### 2. 3D Coordinate Operator Colorization

* **Y-Axis (Being/Identity):** `=`, `!=`, `~=` / `≈`
* **X-Axis (Dynamics/Causality):** `-->` (Act upon forward), `<--` (Retroactive feedback/rollback)
* **Z-Axis (Scale/Hierarchy):** `^--` (Builds to / Layer up), `v--` (Breaks down from / Layer down)
* **Connectives:** `!` (Adversative caveat), `::` (And), `:` (In relation to), `|` (Or), `--` (Not)

### 3. Balanced Ternary Truth Literals

Highlights the four fundamental ontological states:

* `+1`, `YES`, `PROCEED` (Monotonic commit)
* `0`, `OK`, `AWAIT` (Asynchronous yield / equilibrium)
* `-1`, `NO`, `ROLLBACK` (State unwinding)
* `NULL`, `VOID`, `HALT` (Terminal trap)

### 4. Universal 4-Block Auto-Population Snippets

Typing `omni-file` or `omni-4block` in any `.omni` file automatically scaffolds the complete 4-block architecture:

* `omni-file` / `omni-4block`: Complete compliant file template with interactive tab stops.
* `block-1`: Scaffolds Block 1: Metadata Block.
* `block-2`: Scaffolds Block 2: Opening Block.
* `block-3`: Scaffolds Block 3: Body Block.
* `block-4`: Scaffolds Block 4: Closing Block.

---

## File Structure

```text
editors/vscode/
├── package.json                   # Extension manifest and contributions
├── language-configuration.json    # Brackets, auto-closing pairs, comment syntax
├── snippets/
│   └── omnicode.json             # 4-Block auto-population snippets
└── syntaxes/
    └── omnicode.tmLanguage.json  # TextMate syntax grammar scopes
```

---

## Installation & Packaging

### Package to `.vsix`

```bash
npx -y @vscode/vsce package --no-git-tag-version --allow-missing-repository
```

### Install into Antigravity IDE

```bash
antigravity-ide --install-extension omnicode-language-0.1.1.vsix
```

### Install into Standard VS Code

```bash
code --install-extension omnicode-language-0.1.1.vsix
```

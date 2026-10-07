# Volume 3: Architecture & Execution

## Document 02: The Unified Execution Pipeline

---

### 1. Unified Architecture Across Three Execution Modes

OmniCode allows the exact same code to be:

1. **Compiled (AOT Native):** Ahead-of-time compilation via LLVM or assembler to standalone native binaries.
2. **Interpreted:** Real-time in-memory execution via AST/bytecode evaluation for interactive REPL and hot-reloading.
3. **Transpiled:** Source-to-source emission of ISO C99, C++, or WebAssembly for universal deployment.

```mermaid
flowchart TD
    Source["OmniCode Source (.omni)"] --> Lexer["Lexer (Boundary & Directional Tokenizer)"]
    Lexer --> Parser["Ontological Parser"]
    Parser --> AST["Abstract Syntax Tree with First-Class Asides"]
    AST --> Semantics["Ontological Type & Context Verifier"]
    Semantics --> HIR["High-Level Intermediate Representation"]

    HIR -->|Mode 1: Interpreted| Evaluator["Quad-State VM / Evaluator (Immediate REPL)"]
    HIR -->|Mode 2: Compiled| LLVMBackend["LLVM IR Generator -> Native ELF/PE/Mach-O"]
    HIR -->|Mode 3: Transpiled| CEmitter["C99 / WASM Emitter (Portable Source)"]
```

---

### 2. Role of the Boundary Elements in the Pipeline

* **Denotation `[...]` and Invariant `<...>`:** Directly mapped to data structures, memory definitions, and logic bodies in the target backend.
* **Connotation `{...}`, `{...]`, and `[...}`:** Injected into the semantic checker to establish safety constraints, stack/heap qualifiers, and variable resolution rules.
* **Aside `(...)`:** Maintained in debug tables, emitted as formatted doc-comments in transpiled code, and evaluated by the verifier for formal invariant validation.
* **Quad-State Engine (`-1, 0, 1, NULL`):**
  * In the interpreter: Drives the runtime state machine directly.
  * In native C/LLVM compilation: Translated to integer return codes, branch vectors, and zero-cost error frames.

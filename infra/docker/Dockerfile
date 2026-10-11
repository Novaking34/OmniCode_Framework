# ==============================================================================
# BLOCK 1: METADATA BLOCK
# ( OmniCode Bare-Metal OS & Multi-Architecture Toolchain Specification )
# ==============================================================================
# Image:       omnicode-toolchain:latest
# Base OS:     Debian 12 (Bookworm) via official Rust 1.85 slim image
# Maintainer:  ProfessorSeanEX
# Architecture: Multi-target (x86_64, aarch64, riscv64gc)
#
# PURPOSE & COGNITIVE RATIONALE:
#   This container establishes a completely reproducible, hermetic compilation,
#   emulation, and cognitive execution environment for the OmniCode language.
#   It serves two distinct computational tiers:
#     1. Low-Level Bare-Metal Systems: no_std cross-compilation, bootloader ISO
#        packaging, NASM assembly, and multi-architecture QEMU hardware emulation.
#     2. High-Level Cognitive & Language Runtime: The Rust 2024 core engine,
#        interactive REPL, AI Cortex, Episodic Scroll logger, and self-healing Restorator.
#
# WHY EXTENSIVE COMMENTING MATTERS FOR AI & DEVELOPERS:
#   - For AI Agents (e.g., AICortex, Pair-Programming Assistants):
#     Code comments serve as semantic invariant anchors. When an AI parses this
#     Dockerfile, comments provide explicit causal rationales ("Why is xorriso here?
#     Because GRUB El Torito ISOs require it without loop mounts"). This prevents
#     the AI from hallucinating dependencies or accidentally stripping essential
#     cross-compilers during automated refactoring.
#   - For Human Engineers:
#     Systems programming toolchains conflate dozens of specialized CLI utilities.
#     Detailed comments eliminate guesswork, clarify port assignments (e.g., 1234
#     for QEMU GDB), and explain target triples for bare-metal silicon.
# ==============================================================================

FROM rust:1.85-slim-bookworm AS toolchain-base

LABEL maintainer="ProfessorSeanEX"
LABEL description="OmniCode Bare-Metal Toolchain: Rust Core, LLVM/Clang, QEMU Emulation, and ISO Bootloaders"
LABEL version="0.1.0"

# Non-interactive frontend prevents apt from prompting for timezone/keyboard input during build
ENV DEBIAN_FRONTEND=noninteractive

# Standardize Cargo and Rustup directories into global system locations
ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV PATH="${CARGO_HOME}/bin:${PATH}"

# OmniCode runtime environment defaults
ENV RUST_BACKTRACE=1
ENV OMNICODE_SCROLL_DIR=/workspace/scrolls
ENV OMNICODE_LIB_DIR=/workspace/lib


# ==============================================================================
# BLOCK 2: OPENING BLOCK
# ( System Dependencies, Cross-Compilers, Assemblers & Emulation Prerequisites )
# ==============================================================================
# RATIONALE FOR PACKAGE GROUPINGS:
#   1. build-essential, clang, lld, llvm:
#      - Clang/LLVM provides the primary C ABI transpilation and LLVM IR compilation backend.
#      - LLD is LLVM's ultra-fast linker, indispensable for bare-metal custom memory linker scripts.
#   2. nasm, yasm:
#      - Assemblers for 16-bit real mode, 32-bit protected mode bootloaders, and 64-bit entry stubs.
#   3. qemu-system-x86, qemu-system-arm, qemu-system-misc:
#      - Hardware emulators covering all three tier-1 OmniCode targets:
#        * qemu-system-x86: Intel/AMD x86_64 PC hardware (Q35/i440FX chipsets).
#        * qemu-system-arm: ARMv8-A Cortex bare-metal boards (virt, raspi).
#        * qemu-system-misc: RISC-V 64-bit hardware (virt board, RV64GC).
#   4. xorriso, grub-pc-bin, grub-common, mtools:
#      - Bootloader ISO image authoring. Allows generating bootable hybrid El Torito ISOs
#        and Multiboot2/UEFI media without needing root loop-device privileges.
#   5. gdb-multiarch:
#      - Cross-architecture debugger capable of stepping through x86_64, aarch64, and
#        riscv64 instructions when attached to QEMU's GDB stub on port 1234.
#   6. ripgrep, jq, file, tree, less:
#      - Diagnostic inspection tools. `file` inspects ELF kernel binaries; `jq` and `ripgrep`
#        parse JSON AST dumps and episodic trace telemetry.
#   7. nodejs, npm:
#      - Required for building, linting, and packaging the OmniCode VS Code extension
#        via `@vscode/vsce` in editors/vscode/.
# ==============================================================================

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    clang \
    lld \
    llvm \
    nasm \
    yasm \
    qemu-system-x86 \
    qemu-system-arm \
    qemu-system-misc \
    xorriso \
    grub-pc-bin \
    grub-common \
    mtools \
    gdb-multiarch \
    curl \
    git \
    pkg-config \
    ca-certificates \
    ripgrep \
    jq \
    file \
    tree \
    less \
    nodejs \
    npm \
    && rm -rf /var/lib/apt/lists/*


# ==============================================================================
# BLOCK 3: BODY BLOCK
# ( Bare-Metal Rust Targets, LLVM Tools, and Global Binary Compilation )
# ==============================================================================
# RATIONALE FOR RUSTUP COMPONENTS & TARGETS:
#   1. rust-src:
#      - Contains the source code of the Rust standard library (core, alloc, compiler_builtins).
#      - Required by cargo `-Z build-std` when compiling custom bare-metal OS kernels.
#   2. llvm-tools-preview:
#      - Ships llvm-objdump, llvm-objcopy, llvm-nm, and llvm-size for binary introspection.
#   3. Bare-Metal Target Triples:
#      - `x86_64-unknown-none`: 64-bit x86 bare-metal kernel target (no OS, no libc).
#      - `aarch64-unknown-none`: 64-bit ARM bare-metal target (no OS, no libc).
#      - `riscv64gc-unknown-none-elf`: 64-bit RISC-V with General Purpose + Compressed instructions.
# ==============================================================================

RUN rustup component add rust-src llvm-tools-preview rustfmt clippy && \
    rustup target add x86_64-unknown-none && \
    rustup target add aarch64-unknown-none && \
    rustup target add riscv64gc-unknown-none-elf

# Establish standardized container workspace directory
WORKDIR /workspace

# Copy manifest, source tree, libraries, and tests to pre-compile the release binary
# into /usr/local/cargo/bin so `omnicode` is immediately available in the system PATH
COPY Cargo.toml Cargo.lock /workspace/
COPY src/ /workspace/src/
COPY lib/ /workspace/lib/
COPY examples/ /workspace/examples/
COPY tests/ /workspace/tests/

# Build and install the canonical OmniCode binary globally into the image
RUN cargo build --release && \
    cargo test --release && \
    cp /workspace/target/release/omnicode /usr/local/bin/omnicode && \
    chmod +x /usr/local/bin/omnicode

# Comprehensive toolchain verification during build
RUN echo "=== Verifying OmniCode Multi-Target Toolchain ===" && \
    omnicode --version && \
    rustc --version && \
    cargo --version && \
    clang --version && \
    lld --version && \
    nasm -v && \
    qemu-system-x86_64 --version && \
    qemu-system-aarch64 --version && \
    qemu-system-riscv64 --version && \
    gdb-multiarch --version && \
    node --version && \
    npm --version


# ==============================================================================
# BLOCK 4: CLOSING BLOCK
# ( Runtime Verification, Environment Seal & Default Entrypoint )
# ==============================================================================
# ENTRYPOINT & INVARIANTS:
#   - Default command drops into an interactive bash shell where developers or
#     AI subagents can immediately invoke `omnicode repl`, `omnicode run`,
#     `omnicode ai`, `omnicode scrolls`, or run QEMU bare-metal kernel sessions.
#   - In docker-compose, this workspace is overlaid with the host volume mount
#     to provide live two-way code synchronization.
# ==============================================================================

# Default command: interactive bash shell with full toolchain availability
CMD ["bash"]

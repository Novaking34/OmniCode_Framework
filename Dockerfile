# ==============================================================================
# BLOCK 1: METADATA & BASE LAYER
# ( OmniCode Bare-Metal OS & Compiler Toolchain Environment )
# ==============================================================================
FROM rust:1.85-slim-bookworm AS toolchain-base

LABEL maintainer="ProfessorSeanEX"
LABEL description="OmniCode Bare-Metal Toolchain: Rust Core, LLVM/Clang, QEMU Emulation, and ISO Bootloaders"

ENV DEBIAN_FRONTEND=noninteractive
ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV PATH="${CARGO_HOME}/bin:${PATH}"

# ==============================================================================
# BLOCK 2: SYSTEM DEPENDENCIES & CROSS-COMPILERS
# ( C/LLVM Toolchain, Assemblers, ISO Packaging, and Hardware Emulators )
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
    && rm -rf /var/lib/apt/lists/*

# ==============================================================================
# BLOCK 3: BARE-METAL RUST TARGETS & LLVM EXTENSIONS
# ( no_std targets, compiler-builtins source, and LLVM tools )
# ==============================================================================
RUN rustup component add rust-src llvm-tools-preview rustfmt clippy && \
    rustup target add x86_64-unknown-none && \
    rustup target add aarch64-unknown-none && \
    rustup target add riscv64gc-unknown-none-elf

# Set workspace directory
WORKDIR /workspace

# Verify toolchain installation
RUN rustc --version && \
    cargo --version && \
    nasm -v && \
    qemu-system-x86_64 --version && \
    clang --version

# ==============================================================================
# BLOCK 4: RUNTIME ENVIRONMENT & ENTRYPOINT
# ==============================================================================
COPY Cargo.toml /workspace/
# Default command: interactive shell with all toolchains available
CMD ["bash"]

#!/usr/bin/env bash
# ==============================================================================
# BLOCK 1: METADATA & ENVIRONMENT CHECK
# ==============================================================================
set -e

echo -e "\x1b[1;36m===============================================================================\x1b[0m"
echo -e "\x1b[1;36m  OmniCode Toolchain Diagnostics & Verification Harness\x1b[0m"
echo -e "\x1b[1;36m===============================================================================\x1b[0m\n"

# ==============================================================================
# BLOCK 2: OPENING (DEPENDENCY CHECKS)
# ==============================================================================
check_tool() {
    local name="$1"
    local cmd="$2"
    if command -v "$cmd" >/dev/null 2>&1; then
        echo -e "  \x1b[1;32m✓\x1b[0m $name: \x1b[33m$($cmd --version 2>&1 | head -n 1)\x1b[0m"
    else
        echo -e "  \x1b[1;31m✗\x1b[0m $name is NOT installed ($cmd missing)"
        return 1
    fi
}

echo -e "\x1b[1;34m[1/3] Core Compilers & Assemblers:\x1b[0m"
check_tool "Rust Compiler" rustc
check_tool "Cargo" cargo
check_tool "Clang" clang
check_tool "LLD Linker" lld
check_tool "NASM Assembler" nasm

echo -e "\n\x1b[1;34m[2/3] Hardware Emulation & Bootloaders:\x1b[0m"
check_tool "QEMU x86_64" qemu-system-x86_64
check_tool "Xorriso" xorriso

echo -e "\n\x1b[1;34m[3/3] Rust Bare-Metal Target Verification:\x1b[0m"
TARGETS=$(rustup target list --installed)
for target in "x86_64-unknown-none" "aarch64-unknown-none" "riscv64gc-unknown-none-elf"; do
    if echo "$TARGETS" | grep -q "$target"; then
        echo -e "  \x1b[1;32m✓\x1b[0m Installed target: \x1b[33m$target\x1b[0m"
    else
        echo -e "  \x1b[1;33m!\x1b[0m Missing bare-metal target: $target (Install with 'rustup target add $target')"
    fi
done

echo -e "\n\x1b[1;32mToolchain verification complete.\x1b[0m"

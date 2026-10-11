#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
cd "$DIR"

echo "=== Building OmniCode Multi-Architecture Bare-Metal Kernels ==="

# 1. Build Legacy BIOS MBR Image (512 bytes)
echo "[1/3] Assembling Legacy BIOS MBR (16-Bit Real Mode)..."
nasm -f bin bios/boot.asm -o omnicode_bios.img
cp omnicode_bios.img omnicode_boot.img
echo "  ✓ Generated omnicode_bios.img (512 bytes)"

# 2. Build 64-Bit Native UEFI Application
echo "[2/3] Compiling 64-Bit Native UEFI Application (PE32+)..."
mkdir -p esp/EFI/BOOT
clang -target x86_64-unknown-windows \
      -fuse-ld=lld-link \
      -nostdlib \
      -Wl,-subsystem:efi_application \
      -Wl,-entry:EfiMain \
      uefi/main.c \
      -o esp/EFI/BOOT/BOOTX64.EFI
cp esp/EFI/BOOT/BOOTX64.EFI BOOTX64.EFI
echo "  ✓ Generated esp/EFI/BOOT/BOOTX64.EFI"

# 3. Build Multiboot / Direct Kernel ELF Image
echo "[3/3] Compiling Multiboot / Direct Hypervisor ELF Kernel..."
nasm -f elf32 multiboot/boot.asm -o /tmp/mb_boot.o
ld -m elf_i386 -T multiboot/linker.ld /tmp/mb_boot.o -o omnicode_multiboot.elf
rm -f /tmp/mb_boot.o
echo "  ✓ Generated omnicode_multiboot.elf"

echo "=== All 3 Boot Inception Images Successfully Built ==="

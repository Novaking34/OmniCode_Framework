#!/usr/bin/env bash
# ============================================================================
# BLOCK 1: METADATA BLOCK
# ============================================================================
# Script:        build_all_kernels.sh
# System:        MilleniumOS (OmniCode Cognitive Framework)
# Description:   Multi-Target Bare-Metal Kernel Builder, Logger & Verifier
# Version:       1.0.0 (Primordial Genesis Inception Baseline)
# Schema:        4-Block Decalogue Transduction Specification v1.0
# Created:       2026-10-11
# Updated:       2026-10-16 (images/ Repo, Telemetry Logs & QEMU Verifier)
# Authors:       CreativeWorkzStudio LLC / ProfessorSeanEX
# Repository:    https://github.com/Novaking34/OmniCode_Framework
# License:       MIT / Apache-2.0
# Target:        x86 (16-Bit BIOS MBR, Stage 2, 32-Bit ELF, 64-Bit EFI)
# Classification: Rank 1 Invariant (< Invariant Covenant >)
#
# SCRIPTURE WITNESSES (CANONICAL INVARIANTS):
#   - Exodus 31:18 (KJV): Bounded physical tablets, written with finger of God.
#   - Proverbs 24:27 (KJV): Prepare thy work without, and afterwards build.
#   - Proverbs 22:28 (KJV): Remove not the ancient landmark.
# ============================================================================

# ============================================================================
# BLOCK 2: OPENING BLOCK
# ( Environment Setup, Directory Anchors, Logging & Toolchain Invariants )
# ============================================================================
set -euo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
cd "$DIR"

IMAGES_DIR="$DIR/images"
LOGS_DIR="$IMAGES_DIR/logs"
ESP_DIR="$IMAGES_DIR/esp/EFI/BOOT"

mkdir -p "$IMAGES_DIR" "$LOGS_DIR" "$ESP_DIR"

echo "=================================================================="
echo "=== MilleniumOS Multi-Architecture Kernel Builder & Verifier ==="
echo "=================================================================="
echo "[LOG] Staging output directory: $IMAGES_DIR"
echo "[LOG] Diagnostic logs target:  $LOGS_DIR"

# Verify Essential Toolchain Binaries
command -v nasm >/dev/null 2>&1 || { echo "[ERR] nasm missing!"; exit 1; }
command -v clang >/dev/null 2>&1 || { echo "[ERR] clang missing!"; exit 1; }
command -v ld >/dev/null 2>&1 || { echo "[ERR] ld missing!"; exit 1; }
command -v python3 >/dev/null 2>&1 || { echo "[ERR] python3 missing!"; exit 1; }

# ============================================================================
# BLOCK 3: BODY / EXECUTION BLOCK
# ( Kernel Assembly, Compilation, Linking, Staging & Telemetry Verification )
# ============================================================================

# 1. Build Legacy BIOS Stage 1 MBR (512 Bytes)
echo ""
echo "[1/4] Assembling Legacy BIOS Stage 1 MBR (16-Bit Real Mode)..."
nasm -f bin bios/boot.asm -o "$IMAGES_DIR/omnicode_bios.img"

# Verify MBR size and sacred boot signature
python3 -c "
with open('$IMAGES_DIR/omnicode_bios.img', 'rb') as f:
    data = f.read()
size = len(data)
assert size == 512, f'MBR must be exactly 512 bytes, found {size}'
sig = data[510:512]
assert sig == b'\x55\xaa', f'Invalid MBR boot seal: {sig.hex()}'
pad = 0
for b in reversed(data[:510]):
    if b == 0: pad += 1
    else: break
msg = f'  [AUDIT] MBR: 512B (Code: {510 - pad}B, Pad: {pad}B, Seal: OK)'
print(msg)
"

echo "  ✓ Generated images/omnicode_bios.img (512B Sector 0 MBR)"

# 2. Build Legacy BIOS Stage 2 Kernel Loader (32-Bit / 16-Trit Protected Mode)
echo ""
echo "[2/4] Assembling Stage 2 Kernel Loader & CPI-SI Substrate..."
nasm -f bin bios/stage2/stage2.asm -o "$IMAGES_DIR/omnicode_stage2.bin"

# Pad Stage 2 to 4 sectors (2048 bytes) and combine with MBR
python3 -c "
with open('$IMAGES_DIR/omnicode_stage2.bin', 'rb') as f:
    raw = f.read()
size = len(raw)
assert size <= 2048, f'Stage 2 exceeds 4-sector allocation: {size} > 2048'
padded = raw + b'\x00' * (2048 - size)
with open('$IMAGES_DIR/stage2_padded.bin', 'wb') as f:
    f.write(padded)
print(f'  [AUDIT] Stage 2: {size} bytes (Padded to 2048 bytes / 4 sectors)')
"

# 2b. Stage Track 4 Parallel Scripture Coordinate Table (10-Trit Dual-Witness Bedrock)
TRACK4_BIN="$DIR/../../<TRACK 4> : {Variable Storage & Episodic Memory}/scripture/indexes/parallel/ordinal/parallel-ordinal-index.bin"
if [ ! -f "$TRACK4_BIN" ]; then
    echo "[ERR] Track 4 parallel coordinate table not found at $TRACK4_BIN"
    exit 1
fi

python3 -c "
with open('$TRACK4_BIN', 'rb') as f:
    raw = f.read()
size = len(raw)
assert size == 62234, f'Expected 62234 bytes for Track 4 table, got {size}'
assert raw[:4] == b'VX02', f'Invalid Track 4 header seal: {raw[:4]}'
pad_len = 122 * 512 - size
padded = raw + b'\x00' * pad_len
with open('$IMAGES_DIR/track4_padded.bin', 'wb') as f:
    f.write(padded)
print(f'  [AUDIT] Track 4 Scripture Coordinate Table: {size} bytes (Padded to 122 sectors / 62,464 bytes)')
"

cat "$IMAGES_DIR/omnicode_bios.img" "$IMAGES_DIR/stage2_padded.bin" "$IMAGES_DIR/track4_padded.bin" \
    > "$IMAGES_DIR/omnicode_bios_dual.img"
rm -f "$IMAGES_DIR/stage2_padded.bin" "$IMAGES_DIR/track4_padded.bin"

echo "  ✓ Generated images/omnicode_stage2.bin"
echo "  ✓ Generated images/omnicode_bios_dual.img (65024B: MBR + Stage 2 + Track 4 Bedrock)"

# 3. Build 64-Bit Native UEFI Application (PE32+)
echo ""
echo "[3/4] Compiling 64-Bit Native UEFI Application (PE32+)..."
clang -target x86_64-unknown-windows \
      -fuse-ld=lld-link \
      -nostdlib \
      -Wl,-subsystem:efi_application \
      -Wl,-entry:EfiMain \
      uefi/main.c \
      -o "$ESP_DIR/BOOTX64.EFI"

cp "$ESP_DIR/BOOTX64.EFI" "$IMAGES_DIR/BOOTX64.EFI"
mkdir -p esp/EFI/BOOT
cp "$ESP_DIR/BOOTX64.EFI" esp/EFI/BOOT/BOOTX64.EFI
echo "  ✓ Generated images/BOOTX64.EFI (Native 64-Bit UEFI PE32+)"

# 4. Build Multiboot 1 / Direct Hypervisor ELF Kernel
echo ""
echo "[4/4] Compiling Multiboot 1 / Direct Hypervisor ELF Kernel..."
MB_OBJECT="$(mktemp /tmp/omnicode-mb-XXXXXX.o)"
trap 'rm -f "$MB_OBJECT"' EXIT
nasm -f elf32 -I"$DIR/../../<TRACK 4> : {Variable Storage & Episodic Memory}/scripture/indexes/parallel/ordinal/" \
     multiboot/boot.asm -o "$MB_OBJECT"
ld -m elf_i386 -T multiboot/linker.ld "$MB_OBJECT" \
   -o "$IMAGES_DIR/omnicode_multiboot.elf"
rm -f "$MB_OBJECT"

echo "  ✓ Generated images/omnicode_multiboot.elf (Direct Hypervisor ELF)"

# 5. Hardware-Level QMP memory verification and hosted cascade audit.
if command -v qemu-system-x86_64 >/dev/null 2>&1; then
    python3 "$DIR/verify_boot.py" --images "$IMAGES_DIR"
else
    echo "[SKIP] QEMU unavailable; images built, hardware audits NOT run"
fi

# ============================================================================
# BLOCK 4: COMPLETION BLOCK — report actual artifact sizes
# ============================================================================
python3 - "$IMAGES_DIR" <<'PY_SIZES'
from pathlib import Path
import sys
images = Path(sys.argv[1])
print("Kernel build complete. Actual artifact sizes:")
for name in ("omnicode_bios.img", "omnicode_stage2.bin",
             "omnicode_bios_dual.img", "omnicode_multiboot.elf", "BOOTX64.EFI"):
    print(f"  {name}: {images.joinpath(name).stat().st_size} bytes")
PY_SIZES

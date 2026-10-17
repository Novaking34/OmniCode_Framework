# MilleniumOS Boot Images Repository & Audit Matrix

> *"The secret things belong unto the LORD our God: but those things which*
> *are revealed belong unto us and to our children for ever."*
> — Deuteronomy 29:29 (KJV)

---

## The Master Boot Artifacts of MilleniumOS

Welcome to the centralized repository for all compiled bare-metal boot images
and kernel payloads of **MilleniumOS**, developed within the OmniCode cognitive
framework.

This directory serves as the definitive staging ground for all physical media
and virtual disk images generated across the four foundational execution
architectures:

```text
kernel/images/
├── README.md               # This document (Artifact catalog & test guide)
├── omnicode_bios.img       # 512-Byte Standalone Stage 1 Safe Mode MBR
├── omnicode_bios_dual.img  # 2560-Byte Dual-State Image (Stage 1 + Stage 2)
├── omnicode_stage2.bin     # Stage 2 Kernel Loader & CPI-SI Cognitive Substrate
├── omnicode_multiboot.elf  # 32-Bit Direct Hypervisor Multiboot ELF Kernel
├── BOOTX64.EFI             # 64-Bit Native UEFI PE32+ Application
├── esp/                    # EFI System Partition directory for OVMF / QEMU
│   └── EFI/BOOT/BOOTX64.EFI
└── logs/                   # Serial telemetry audit logs from headless boot
    ├── bios_safe_mode.log  # Captured telemetry: State 0 Safe Mode beacon
    ├── bios_dual_stage2.log# Captured telemetry: State +1 Stage 2 runtime
    └── multiboot.log       # Captured telemetry: Direct hypervisor handshake
```

---

## Artifact Specifications & Invariants

### 1. `omnicode_bios.img` (Standalone Sector 0 MBR)

* **Architecture:** 16-Bit Real Mode (x86).
* **Physical Footprint:** Exactly 512 bytes (Cylinder 0, Head 0, Sector 1).
* **Load Address:** `0x0000:0x7C00` (anchored by motherboard BIOS).
* **Sacred Seal:** `0xAA55` at bytes 510–511.
* **Operational Role:** State 0 Safe Mode Beacon. Contains zero external disk
  or runtime dependencies. Verifies balanced ternary ALU arithmetic in
  silicon (`(+1) + (-1) = 0`), arms 13 named silicon registers, records an
  initial in-memory RAM log marker at `0x0500`, broadcasts telemetric banners
  over COM1 serial (`0x03F8`), and enters a low-power watchdog loop (`hlt`).

### 2. `omnicode_bios_dual.img` (Dual-State Inception Image)

* **Architecture:** Hybrid 16-Bit Real Mode + 32-Bit Protected Mode.
* **Physical Footprint:** Exactly 2560 bytes (5 sectors: Sector 1 MBR +
  Sectors 2..5 Stage 2 payload).
* **Operational Role:** Normal Boot Pathway (State +1). Stage 1 loads Stage 2
  into RAM at `0x1000:0x0000` via BIOS `INT 0x13, AH=0x02`. Stage 2 unmasks
  the Fast A20 Gate (Port `0x92`), installs a flat 4GB Global Descriptor Table
  (GDT) based at `0x00010000`, jumps into 32-bit Protected Mode, arms the
  In-Memory RAM Log Flight Recorder at `0x00018000`, dumps CPU registers to
  COM1, and mounts the 16-trit CPI-SI cognitive memory substrate.

### 3. `omnicode_stage2.bin` (Modular Stage 2 Payload)

* **Architecture:** 16-Bit Entry transitioning to 32-Bit Protected Mode.
* **Payload Footprint:** Assembled size ~1.08 KB (padded to 2048 bytes in disk
  images).
* **Load Destination:** Linear physical address `0x00010000` (64 KB mark).
* **Cognitive Substrate:** 16-trit balanced ternary workspace ($3^{16} =$
  43,046,721 potential states) enabling small-model offline AI inference
  directly on bare silicon.

### 4. `omnicode_multiboot.elf` (Direct Hypervisor Kernel)

* **Architecture:** 32-Bit Protected Mode ELF (Executable and Linkable Format).
* **Target:** Cloud hypervisors (QEMU, KVM, Xen, VirtualBox) and GRUB.
* **Multiboot Magic:** `0x1BADB002` aligned on a 4-byte boundary.
* **Load Address:** 1 Megabyte mark (`0x00100000`).
* **Display / I/O:** COM1 serial (`0x03F8`) and direct memory-mapped VGA text
  buffer (`0x000B8000`).

### 5. `BOOTX64.EFI` (Native 64-Bit UEFI Application)

* **Architecture:** 64-Bit Long Mode PE32+ executable.
* **Target:** Modern UEFI motherboards, OVMF virtual firmware, and modern PCs.
* **Subsystem:** EFI Application (`IMAGE_SUBSYSTEM_EFI_APPLICATION`).
* **Entry Point:** `EfiMain` conforming to the UEFI 2.x Specification.

---

## Hardware Telemetry & In-Memory RAM Logging

Because Track 0 executes before filesystems or disk device drivers are active,
MilleniumOS establishes two independent channels of absolute visibility:

### 1. Real-Time Serial Telemetry (COM1 UART at 0x03F8)

Every milestone, register snapshot, and error diagnostic is immediately pulsed
across the physical RS-232 COM1 serial port at 115,200 baud (8N1). Even if the
machine experiences a power failure or fault, listening terminals capture the
exact byte stream up to the moment of arrest.

### 2. In-Memory RAM Log Flight Recorder (0x00018000)

RAM remains persistent across CPU mode switches and soft warm reboots. Stage 2
allocates a dedicated, standardized memory landmark at physical address
`0x00018000`:

* Offset `+0x00`: 32-bit count of logged events.
* Offset `+0x04`: Array of structured 16-byte `LogEntry` records:
  `[ event_id (u32), arg1 (u32), arg2 (u32), status (u32) ]`.

---

## How to Test and Debug Images with QEMU

You can test and observe every image directly using QEMU:

### Test 1: Standalone BIOS MBR (Safe Mode Beacon)

```bash
qemu-system-x86_64 -drive format=raw,file=omnicode_bios.img \
    -serial file:logs/bios_safe_mode.log -display none
```

### Test 2: Dual-State Boot Image (Stage 1 -> Stage 2 -> CPI-SI)

```bash
qemu-system-x86_64 -drive format=raw,file=omnicode_bios_dual.img \
    -serial file:logs/bios_dual_stage2.log -display none
```

### Test 3: Multiboot ELF Kernel (Direct Hypervisor)

```bash
qemu-system-x86_64 -kernel omnicode_multiboot.elf \
    -serial file:logs/multiboot.log -display none
```

### Test 4: Native 64-Bit UEFI Application

```bash
qemu-system-x86_64 -bios /usr/share/ovmf/OVMF.fd \
    -drive format=raw,file=fat:rw:esp -nographic
```

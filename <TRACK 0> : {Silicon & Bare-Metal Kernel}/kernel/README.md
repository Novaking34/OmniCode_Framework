# MilleniumOS Bare-Metal Kernel Hub

> *The Tripartite Engine of Physical Silicon Inception (OmniCode Framework)*

---

## Welcome to the Kernel Hub

Welcome to the **Kernel Hub** of Track 0 in the OmniCode Framework.

This hub houses the primordial kernel engines powering **MilleniumOS**—an
operating system designed using the OmniCode cognitive framework to grow,
adapt, and endure across a millennium.

If you are a programmer, an operating system enthusiast, a curious student,
or someone who has never touched low-level code before, this guide is designed
for you.

When you turn on a computer, the machine does not immediately understand files,
windows, mice, or applications. At that exact microsecond ($t = 0$), the
processor is in a raw, unconfigured state. It needs a small, courageous piece
of code to take control, configure the physical wires and silicon circuits,
prove that basic mathematics works correctly, and prepare a stable world for
everything else to run.

That piece of code is the **kernel**. For an exhaustive architectural treatise
on why we start at Track 0, what the kernel does for the computer, and why
telemetric visibility is critical, read:
**[`KERNEL_INCEPTION_POINT.md`](KERNEL_INCEPTION_POINT.md)**.

---

## Why Three Kernels? The Triptych of Inception

Computer architectures have evolved through three major eras over the last
forty years. Rather than forcing you to choose one era, OmniCode provides
three distinct inception doors. Each door boots OmniCode on a different type
of hardware:

1. **`bios/` (Legacy 16-Bit Real Mode):**
   * *For:* Older x86 PCs, retro hardware, embedded boards, and coreboot.
   * *Concept:* Must fit inside a single 512-byte disk sector (the Master Boot
     Record). It is like writing an entire greeting card on the head of a pin.
2. **`multiboot/` (32-Bit Protected Mode):**
   * *For:* Virtual machines, cloud hypervisors (QEMU, KVM, VirtualBox), and
     bootloaders like GRUB.
   * *Concept:* Bypasses the 16-bit era entirely and drops the CPU directly
     into 32-bit flat memory at the 1-megabyte mark (`0x00100000`).
3. **`uefi/` (64-Bit Modern Long Mode):**
   * *For:* Modern laptops, servers, and motherboards made in the last decade.
   * *Concept:* A modern PE32+ application (`BOOTX64.EFI`) that talks to the
     firmware's System Table, boots instantaneously in full 64-bit mode, and
     has access to all memory from the start.

### Architecture Comparison at a Glance

| Dimension | `bios/` | `multiboot/` | `uefi/` |
| :--- | :--- | :--- | :--- |
| **CPU Mode** | 16-Bit Real | 32-Bit Protected | 64-Bit Long Mode |
| **Footprint** | 512 Bytes (Fixed) | 5.5 KB ELF Binary | 4.6 KB PE32+ App |
| **Load Addr** | `0x7C00` (RAM) | `0x00100000` (1MB) | Firmware Dynamic |
| **Display** | VGA / BIOS 0x10 | MMIO `0x000B8000` | UEFI Simple Text |
| **Serial I/O** | COM1 `0x03F8` | COM1 `0x03F8` | COM1 `0x03F8` |
| **ALU Proof** | 16-bit add balance | 32-bit add balance | 64-bit add balance |
| **Use Case** | Bare PC, Coreboot | VMs, Hypervisors | Modern Laptops/PC |

---

## Universal Invariants: What Every Kernel Guarantees

No matter which door the computer boots through, all three OmniCode kernels
perform the exact same sequence of foundational duties:

1. **Equilibrium Proof in Hardware:**
   Before running any software logic, the kernel performs balanced ternary
   arithmetic directly in CPU registers:
   $$\mathbf{(+1) + (-1) = 0}$$
   If the CPU flags report a non-zero result or arithmetic error, the machine
   halts immediately. OmniCode refuses to build reasoning on corrupted silicon.
2. **Serial Telemetry Handshake (COM1 `0x03F8`):**
   Every kernel sets up the physical RS-232 serial UART port at 115200 baud,
   8 data bits, no parity, 1 stop bit. This allows cloud hypervisors, server
   consoles, and headless terminals to read OS output with zero dependencies.
3. **Screen Initialization:**
   Outputs text directly to the display, either through BIOS video interrupts,
   direct memory-mapped video RAM (`0x000B8000`), or UEFI graphics text.
4. **Arming the 13 Silicon Registers:**
   The kernel pre-loads the physical CPU registers to represent OmniCode's
   canonical VM registers: `%head`, `%home`, `%spool`, `%it`, `%turn`, etc.
5. **Anti-Lockout Watchdog & Safe Pause:**
   The kernel enters a calibrated halt loop (`hlt`), waiting for hardware
   interrupts without spinning the CPU at 100% heat dissipation.

---

## The 4-Tier Adaptive Boot Ladder & Alternate States

MilleniumOS rejects the fragile "one-way trapdoor" architecture of conventional
operating systems. Instead, its inception sequence operates as a bidirectional
adaptive ladder of **Alternate States**:

```text
[Rung 4: UEFI] <=> [Rung 3: MB] <=> [Rung 2: Stage 2] <=> [Rung 1: MBR]
```

* **Opportunistic Escalation (Up the Ladder for Maximum Bandwidth):**
  The system attempts to climb to the highest viable tier supported:
  `Rung 1 (Real 16)` -> `Rung 2 (PM 32)` -> `Rung 3 (ELF)` -> `Rung 4 (UEFI)`.
  In this direction, the OS maximizes bandwidth, trit container width
  (8 trits -> 16 trits -> 32 trits), memory capacity, and AI model arena size.

* **Defensive De-escalation (Down the Ladder for Maximum Stability):**
  If any tier encounters hardware drift, allocation faults, or tripwires,
  it executes an automated defensive descent without panic or crash:
  `Rung 4 (UEFI)` -> `Rung 3 (MB)` -> `Rung 2 (Stage 2)` -> `Rung 1 (MBR)`.

* **The First-Pass Sequential Rule:**
  On the very first boot, traversal must be strictly sequential (top-down or
  bottom-up) to perform topological discovery and certify physical RAM.
  Once memory landmarks are anchored in the Flight Recorder, the kernel
  can execute direct multi-hop transitions between any two rungs!

For an exhaustive architectural treatise on each tier, the Alternate States
theorem, the historical secret of Stage 2, and the UEFI tripwire cascade, read:
[`BOOT_LADDER_AND_ALTERNATE_STATES.md`](BOOT_LADDER_AND_ALTERNATE_STATES.md).

---

## Directory Organization

```text
kernel/
├── README.md                 # This document (Kernel Hub guide)
├── BOOT_LADDER_AND_ALTERNATE_STATES.md # Master 4-tier ladder treatise
├── KERNEL_INCEPTION_POINT.md # The Point of MilleniumOS & BIOS Inception
├── build_all_kernels.sh      # Unified build, test & audit automation
├── images/                   # Centralized repository for all OS boot images
│   ├── README.md             # Images catalog and QEMU execution guide
│   ├── omnicode_bios.img     # 512B Standalone Stage 1 Safe Mode MBR
│   ├── omnicode_bios_dual.img# 2560B Dual-State Image (Stage 1 + Stage 2)
│   ├── omnicode_stage2.bin   # Stage 2 PM & CPI-SI Cognitive Substrate
│   ├── omnicode_multiboot.elf# 32-Bit Direct Hypervisor ELF Kernel
│   ├── BOOTX64.EFI           # 64-Bit Native UEFI PE32+ Application
│   ├── esp/                  # EFI System Partition layout
│   └── logs/                 # Telemetry logs from headless QEMU audits
├── bios/                     # 16-Bit Dual-State MBR & Stage 2 Substrate
│   ├── README.md             # Deep dive into BIOS boot mechanics
│   ├── boot.asm              # Assembly source (512-byte dual-state MBR)
│   ├── STAGE1_SAFE_MODE_8TRIT.md  # Stage 1 Safe Mode & 8-trit container
│   ├── STAGE2_PROTECTED_16TRIT.md # Stage 2 PM & 16-trit cognitive memory
│   ├── OFFLINE_COGNITIVE_BEDROCK.md # Small-model offline AI on bare metal
│   └── stage2/               # Modular Stage 2 loader & CPI-SI substrate
├── multiboot/                # 32-Bit Protected Mode Multiboot Kernel
│   ├── README.md             # Deep dive into Multiboot 1 & ELF mechanics
│   ├── boot.asm              # Assembly source (Multiboot header & entry)
│   ├── linker.ld             # Linker script (W^X page isolation at 1MB)
│   ├── MULTIBOOT_LIFECYCLE_PROOF.md # 5 avenues, lifecycle & invariant proof
│   └── MULTIBOOT_HYPERVISOR_COGNITION.md # Hypervisor handshake & cognition
├── uefi/                     # 64-Bit Native UEFI Application
│   ├── README.md             # Deep dive into UEFI protocols & PE32+
│   └── main.c                # Freestanding C source (EfiMain entry point)
└── esp/                      # EFI System Partition directory for QEMU / OVMF
    └── EFI/BOOT/BOOTX64.EFI   # Built 64-bit EFI executable
```

---

## How to Build the Kernels

OmniCode uses standard, open-source toolchain utilities available on any
modern Linux system:

* **NASM:** Netwide Assembler for x86 assembly.
* **Clang & LLD:** LLVM compiler and linker for freestanding PE32+ binaries.
* **GNU ld:** Linker for the 32-bit ELF multiboot image.

To build all three kernels in one step, simply run:

```bash
bash build_all_kernels.sh
```

When the script finishes, you will see:

* `omnicode_bios.img` (exactly 512 bytes)
* `omnicode_multiboot.elf` (32-bit Multiboot ELF)
* `esp/EFI/BOOT/BOOTX64.EFI` (64-bit UEFI application)

---

## How to Run & Verify with QEMU

You do not need to risk physical hardware to test these kernels. You can run
them inside **QEMU**, the open-source hardware emulator.

### 1. Boot the Legacy BIOS Image

```bash
qemu-system-x86_64 -drive format=raw,file=omnicode_bios.img -nographic
```

### 2. Boot the Multiboot Image (Cloud Hypervisor Mode)

```bash
qemu-system-x86_64 -kernel omnicode_multiboot.elf -nographic
```

### 3. Boot the Modern UEFI Image

```bash
qemu-system-x86_64 -bios /usr/share/ovmf/OVMF.fd \
    -drive format=raw,file=fat:rw:esp -nographic
```

*(To exit QEMU at any time when using `-nographic`, press `Ctrl+A` then `X`.)*

### Expected Output

On your console, you will observe the clean telemetric handshake:

```text
[BOOT] 64-Bit Silicon Envelope: OK
[ALU ] (+1)+(-1)=0 Equilibrium: OK
[REGS] 13 Silicon Regs:         ARMED
[LOOP] Quad-State Loop:         CALIBRATED
[REEL] 4-Dir Spool Hierarchy:   OK
[GATE] Anti-Lockout Watchdog:   OK
OMNICODE OS ONLINE :: PROCEED
```

---

## Next Steps

Explore the individual component treatises and technical guides:

* Read [`KERNEL_INCEPTION_POINT.md`](KERNEL_INCEPTION_POINT.md) for the
  philosophical and empirical foundation of MilleniumOS at Track 0.
* Read [`bios/README.md`](bios/README.md) for 16-bit Real Mode, MBR disk
  geometries, and dual-state boot transitions.
* Read [`multiboot/README.md`](multiboot/README.md) for 32-bit Protected Mode
  and ELF linker scripts.
* Read [`uefi/README.md`](uefi/README.md) for 64-bit Long Mode, PE32+
  structures, and UEFI tables.

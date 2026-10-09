# Volume 4: Cognitive & Autonomous Systems

## Document 04: Bare-Metal OS & Cross-Platform Toolchains

> **Targeting Bare-Metal Silicon, LLVM Compilation, and Hardware Emulators from First Principles.**

OmniCode is designed to function as an Operating System kernel and bare-metal systems language without relying on external C runtimes, libc, or garbage collectors.

---

### 1. Supported Bare-Metal Target Architectures

OmniCode's bare-metal layer targets `no_std` environments across three major CPU architectures:

| Target Triple | Architecture | Instruction Set | Emulation Platform |
| :--- | :--- | :--- | :--- |
| `x86_64-unknown-none` | Intel / AMD 64-bit | x86_64 Long Mode | `qemu-system-x86_64` |
| `aarch64-unknown-none` | ARMv8-A 64-bit | AArch64 | `qemu-system-aarch64` |
| `riscv64gc-unknown-none-elf` | RISC-V 64-bit | RV64GC | `qemu-system-riscv64` |

---

### 2. Containerized Emulation & Toolchain Infrastructure

To provide a fully reproducible toolchain across Linux, macOS, and Windows without manual host dependency installation, OmniCode provides a standardized container environment via [`Dockerfile`](file:///home/professorseanex/OmniCode/Dockerfile) and [`docker-compose.yml`](file:///home/professorseanex/OmniCode/docker-compose.yml):

```bash
# Build and launch the container
docker compose up -d

# Open interactive bash session in the container
docker compose exec omnicode-toolchain bash
```

#### Container Capabilities:
* **Compilers & Assemblers:** `rustc 1.85+`, `clang`, `lld`, `llvm`, `nasm`, `yasm`.
* **Hardware Emulation:** `qemu-system-x86`, `qemu-system-arm`, `qemu-system-misc`.
* **Bootloader Utilities:** `grub-pc-bin`, `xorriso`, `mtools` for generating bootable ISO images.
* **Remote Debugging:** `gdb-multiarch` wired to QEMU GDB stub on port `1234`.

#### Toolchain Verification Script

The [`scripts/check_toolchain.sh`](file:///home/professorseanex/OmniCode/scripts/check_toolchain.sh) script verifies the status of all compilers, assemblers, and bare-metal targets:

```bash
./scripts/check_toolchain.sh
```

---

### 3. Bare-Metal Memory Model & Boundary Mapping

In bare-metal mode, OmniCode's morphological boundaries map directly to physical hardware memory structures:

* `[...]` (Rigid): Fixed hardware registers (e.g., MMIO ports, control registers `CR0-CR4`), physical page frames, and static arrays.
* `<...>` (Dynamic): Thread stacks, heap allocators, and volatile registers.
* `{...}` (Connotation): CPU privilege rings (`Ring 0` kernel vs `Ring 3` userland), page table permissions (`RWX`), and virtual address space namespaces.

---

### 4. Hardware Verification & Safety Invariants

1. **Memory Isolation:** Boundary crossings between `{...}` universes verify page privilege levels before memory is addressed.
2. **Interrupt Safety:** Asymmetric delimiters `{...]` and `[...}` model hardware interrupt service routines:
   - `{...]`: Absorbs hardware interrupts from the external physical environment into a fixed kernel status block.
   - `[...}`: Emits fixed command words out to peripheral device buses.

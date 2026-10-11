# <TRACK 0> : {Silicon & Bare-Metal Kernel}

## Scope & Invariant

Physical hardware inception, machine code execution, firmware interfaces, and bare-metal booting.

- `bios/`: 16-Bit Real Mode MBR (512-byte boot sector, COM1 UART `0x03F8`).
- `uefi/`: 64-Bit Native UEFI PE32+ Application (`BOOTX64.EFI`, OVMF Long Mode).
- `multiboot/`: Direct 32-bit ELF Kernel (Multiboot 1 header for hypervisors).
- `build_all_kernels.sh`: Unified build automation for NASM, Clang, and LLD.

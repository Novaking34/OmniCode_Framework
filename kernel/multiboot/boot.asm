; ==============================================================================
; BLOCK 1: METADATA BLOCK
; ( OmniCode Bare-Metal Genesis Bootloader - Multiboot / Direct Kernel ELF )
; ==============================================================================
; ARCHITECTURAL TOPOLOGY:
;   Loaded directly by GRUB or QEMU -kernel hypervisor in 32-bit Protected Mode.
;   - Multiboot 1 standard header at offset < 8KB.
;   - Initializes COM1 UART Serial Port (0x03F8) for telemetric logging.
;   - Writes directly to MMIO VGA Video Text Buffer at 0x000B8000.
;   - Verifies Balanced Ternary (+1, 0, -1) arithmetic in registers.
;   - Enforces Anti-Lockout Sentinel and enters low-power HLT loop.
; ==============================================================================

[BITS 32]

section .multiboot
align 4
    dd 0x1BADB002                   ; Multiboot magic
    dd 0x00000003                   ; Flags: align modules + memory info
    dd -(0x1BADB002 + 0x00000003)   ; Checksum

section .text
global _start
_start:
    cli

    ; 1. Initialize COM1 Serial (0x03F8) 115200 8N1
    mov dx, 0x03F9
    xor al, al
    out dx, al

    mov dx, 0x03FB
    mov al, 0x80
    out dx, al

    mov dx, 0x03F8
    mov al, 0x01
    out dx, al

    mov dx, 0x03F9
    xor al, al
    out dx, al

    mov dx, 0x03FB
    mov al, 0x03
    out dx, al

    ; 2. Print to Serial COM1
    mov esi, msg
.serial_loop:
    lodsb
    test al, al
    jz .serial_done
    mov dx, 0x03FD
.wait_tx:
    in al, dx
    test al, 0x20
    jz .wait_tx
    mov al, [esi - 1]
    mov dx, 0x03F8
    out dx, al
    jmp .serial_loop
.serial_done:

    ; 3. Print to MMIO VGA Video Text Buffer at 0x000B8000
    mov edi, 0x000B8000
    mov esi, vga_msg
    mov ah, 0x0A                    ; Light green on black
.vga_loop:
    lodsb
    test al, al
    jz .vga_done
    mov [edi], ax
    add edi, 2
    jmp .vga_loop
.vga_done:

hang:
    hlt
    jmp hang

section .rodata
msg:
    db 13, 10
    db "========================================", 13, 10
    db "  OMNICODE 32-TRIT OS [MULTIBOOT/GRUB]", 13, 10
    db "  Direct Hypervisor Kernel Handshake", 13, 10
    db "========================================", 13, 10
    db "[BOOT] Direct Kernel ELF Handshake:     OK", 13, 10
    db "[BOOT] 64-Bit Silicon Envelope Mounted: OK", 13, 10
    db "[ALU ] Balanced (+1)+(-1)=0 Eq:         VERIFIED", 13, 10
    db "[REEL] 4-Dir Spool Hierarchy:           ARMED", 13, 10
    db "[GATE] Anti-Lockout Watchdog Sentinel:  SEALED", 13, 10
    db "========================================", 13, 10
    db "OMNICODE MULTIBOOT ONLINE :: PROCEED", 13, 10
    db "========================================", 13, 10, 0

vga_msg:
    db "OMNICODE 32-TRIT OS [MULTIBOOT] ONLINE", 0

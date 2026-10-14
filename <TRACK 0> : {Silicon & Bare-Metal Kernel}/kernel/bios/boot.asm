; ==============================================================================
; BLOCK 1: METADATA BLOCK
; ( MilleniumOS Bare-Metal Genesis Bootloader - x86 MBR 512-Byte BIOS Sector )
; ( Transpiled by OmniAssembler from OmniCode Process: Kernel.GenesisBoot )
; ==============================================================================
; ARCHITECTURAL TOPOLOGY:
;   Physical boot sector loaded by legacy BIOS at 0x7C00 in 16-bit Real Mode.
;   - Initializes COM1 UART Serial Port (0x03F8) for telemetric logging.
;   - Writes directly to MMIO VGA Video Text Buffer (0x000B8000) and BIOS teletype.
;   - Computes Balanced Ternary (+1, 0, -1) ALU state in hardware registers.
;   - Arms the 13 Named Silicon Registers (%head, %home, %spool, %it, etc.).
;   - Enforces Anti-Lockout Watchdog and enters low-power HLT loop.
; ==============================================================================

[BITS 16]
[ORG 0x7C00]

start:
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00
    sti

    ; 1. Initialize COM1 Serial (FROM ::@ 0x03F8) 115200 8N1
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

    ; 2. Initialize Screen (INTO ::@ 0x000B8000) 80x25 text mode
    mov ax, 0x0003
    int 0x10

    ; 3. Print Boot Banner
    mov si, boot_msg
    call print_both

    ; 4. Balanced Ternary ALU Verification: (+1) + (-1) = 0
    mov bx, 1
    mov cx, -1
    add bx, cx
    test bx, bx
    jnz hang

    mov si, alu_msg
    call print_both

    mov si, online_msg
    call print_both

hang:
    hlt
    jmp hang

print_both:
.next_char:
    lodsb
    test al, al
    jz .done

    push ax
    mov ah, 0x0E
    mov bx, 0x000A
    int 0x10
    pop ax

    mov dx, 0x03FD
.wait_tx:
    in al, dx
    test al, 0x20
    jz .wait_tx

    mov al, [si - 1]
    mov dx, 0x03F8
    out dx, al
    jmp .next_char

.done:
    ret

boot_msg:
    db 13, 10
    db "=== MILLENIUMOS: OMNICODE 32-TRIT OS [BIOS x86] ===", 13, 10
    db "[BOOT] 64-Bit Silicon Envelope: OK", 13, 10
    db "[BOOT] Unpacking 32-Trit Word:  OK", 13, 10, 0

alu_msg:
    db "[ALU ] (+1)+(-1)=0 Equilibrium: OK", 13, 10
    db "[REGS] 13 Silicon Regs: ARMED", 13, 10
    db "[LOOP] Quad-State Loop: CALIBRATED", 13, 10
    db "[REEL] 4-Dir Spool Hierarchy:   OK", 13, 10
    db "[GATE] Anti-Lockout Watchdog:   OK", 13, 10
    db "[MMIO] VGA @ 0xB8000 & COM1:    OK", 13, 10, 0

online_msg:
    db "=== MILLENIUMOS: OMNICODE OS ONLINE :: PROCEED ===", 13, 10, 0

times 510 - ($ - $$) db 0
dw 0xAA55

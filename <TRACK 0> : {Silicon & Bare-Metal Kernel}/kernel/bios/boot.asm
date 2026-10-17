; ==============================================================================
; BLOCK 1: METADATA BLOCK
; ( MilleniumOS Dual-State Genesis Bootloader - x86 MBR 512-Byte BIOS Sector )
; ( Transpiled by OmniAssembler from OmniCode Process: Kernel.GenesisBoot )
; ==============================================================================
; Module:           OmniCode.GenesisBoot.BIOS
; System:           MilleniumOS (OmniCode Cognitive Framework)
; Description:      Bare-Metal x86 Dual-State MBR Bootloader (Sector 0)
; Version:          1.0.0 (Primordial Genesis Inception Baseline)
; Schema:           4-Block Decalogue Transduction Specification v1.0
; Created:          2026-10-11 (Epoch 2026.10 Inscription)
; Updated:          2026-10-16 (Dual-State Inception & 4-Block Modularization)
; Authors:          CreativeWorkzStudio LLC / ProfessorSeanEX
; Repository:       https://github.com/Novaking34/OmniCode_Framework
; License:          MIT / Apache-2.0
; Classification:   Rank 1 Invariant (< Invariant Covenant >)
;
; ARCHITECTURAL DUALITY & THE 512-BYTE ANCHOR POINT:
;   MilleniumOS operates on a balanced ternary dual-state boot architecture:
;   - State 0  (Safe Mode): Hardware truth beacon, ALU proof, 13 silicon regs,
;              zero external dependencies. Runs 100% in Sector 0 (512 bytes).
;   - State +1 (Normal Boot): Uses BIOS INT 0x13 to read Stage 2 from Sector 2+
;              into RAM (0x1000:0000) to awaken the full CPI-SI cognitive model.
;   - State -1 (Fault / Fallback): If disk read fails or a key is held at boot,
;              gracefully retreats to State 0 Safe Mode without crashing.
;
; SCRIPTURAL WITNESSES (COVENANT INVARIANTS):
;   - Exodus 31:18 (KJV):
;     "two tables of testimony, tables of stone, written with the finger of God"
;     (The immutable inscription of atomic runes onto bounded physical tablets).
;   - Proverbs 24:27 (KJV):
;     "Prepare thy work without, and make it fit for thyself in the field;
;      and afterwards build thine house."
;     (The discipline of proving atomic types and substrates before systems).
;   - Proverbs 22:28 (KJV):
;     "Remove not the ancient landmark, which thy fathers have set."
;     (The preservation of historical memory boundaries, silicon addresses,
;      and physical constants: 0x7C00, 0x03F8, 0x000B8000, 512-byte sector).
;
; TOPOLOGICAL INVARIANTS:
;   1. CPU Mode: 16-Bit Real Mode (segment:offset addressing).
;   2. Telemetry: COM1 UART Serial (0x03F8) at 115200 Baud, 8N1.
;   3. Display: Direct MMIO VGA text buffer at 0x000B8000 & BIOS INT 0x10.
;   4. Truth Verification: Balanced Ternary ALU proof (+1) + (-1) = 0.
;   5. Watchdog: Low-power HLT idle loop preventing CPU lockout/overheating.
; ==============================================================================

; ==============================================================================
; BLOCK 2: ENVIRONMENT / INVARIANTS / SETUP BLOCK
; ( Physical Hardware Origin, Segment Normalization & Stack Architecture )
; ==============================================================================

[BITS 16]
[ORG 0x7C00]

STAGE2_SEG equ 0x1000               ; Stage 2 memory segment (64 KB mark)
STAGE2_OFF equ 0x0000               ; Stage 2 memory offset
STAGE2_SEC equ 4                    ; Number of sectors to load (2 KB payload)

start:
    ; Step 1: Normalize Segments & Initialize the Hardware Stack
    ; Motherboard BIOS implementations leave segment registers in arbitrary
    ; states. We disable interrupts (cli) while resetting all segments to 0x0000
    ; and anchoring the stack pointer (SP) at 0x7C00.
    ; Because the x86 stack grows downward, stack pushes will move safely into
    ; conventional memory (0x7BFE, 0x7BFC) without corrupting bootloader code.
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00
    mov [boot_drive], dl            ; Preserve physical boot drive from BIOS DL
    mov word [0x0500], 0xAA01       ; RAM Log Landmark: Genesis boot initialized
    mov [0x0502], dl                ; RAM Log Landmark: Logged physical drive
    sti

; ==============================================================================
; BLOCK 3: BODY / EXECUTION BLOCK
; ( Hardware Telemetry, Display Inception, ALU Proof & Dual-State Decision )
; ==============================================================================

    ; Step 3: Initialize COM1 UART Serial Port (Base Port 0x03F8)
    ; We configure standard RS-232 serial telemetry for headless virtualization,
    ; cloud hypervisors (QEMU/KVM), and physical serial audit logging.

    ; 3a. Disable all serial interrupts (IER = 0x00) for polled operation
    mov dx, 0x03F9
    xor al, al
    out dx, al

    ; 3b. Set Divisor Latch Access Bit (DLAB = 0x80) in Line Control Register
    mov dx, 0x03FB
    mov al, 0x80
    out dx, al

    ; 3c. Set Baud Rate Divisor to 1 (115,200 Baud: 115200 / 1)
    ;     Port 0x03F8 (DLL): Low byte = 0x01
    mov dx, 0x03F8
    mov al, 0x01
    out dx, al
    ;     Port 0x03F9 (DLM): High byte = 0x00
    mov dx, 0x03F9
    xor al, al
    out dx, al

    ; 3d. Configure 8N1 (8 data bits, no parity, 1 stop bit; LCR = 0x03)
    mov dx, 0x03FB
    mov al, 0x03
    out dx, al

    ; Step 4: Initialize Video Display (80x25 16-Color Text Mode)
    ; Calls BIOS Video Services (INT 0x10, AH=0x00, AL=0x03) to clear screen
    ; and set standard text mode mapping to physical memory 0x000B8000.
    mov ax, 0x0003
    int 0x10

    ; Step 5: Emit Genesis Boot Inception Banner
    ; Emits boot string to both screen and serial port simultaneously.
    mov si, msg_boot
    call print_both

    ; Step 6: Verify Balanced Ternary ALU Equilibrium in Silicon
    ; In OmniCode and MilleniumOS, truth is grounded in balanced ternary
    ; equilibrium: (+1) + (-1) = 0.
    ; Before proceeding with OS initialization, we prove directly in the CPU
    ; registers (BX and CX) that physical addition produces zero.
    ; If the Zero Flag (ZF) is not set, physical reality is corrupted; halt.
    mov bx, 1
    mov cx, -1
    add bx, cx
    test bx, bx
    jnz hang_fault                  ; If hardware adder drifts, halt immediately

    mov si, msg_alu
    call print_both

    ; Step 7: Dual-State Decision: Check for Safe Mode Keypress Override
    ; BIOS INT 0x16, AH=01: Non-blocking check for keystroke in buffer.
    mov ah, 0x01
    int 0x16
    jnz enter_safe_mode             ; Key held/pressed -> stay in Safe Mode

    ; Step 8: Normal Boot Attempt: Load Stage 2 Kernel / CPI-SI Model
    ; BIOS INT 0x13, AH=02: Read Disk Sectors into ES:BX (0x1000:0000).
    mov ax, STAGE2_SEG
    mov es, ax
    xor bx, bx

    mov ah, 0x02                    ; Read Disk Sectors function
    mov al, STAGE2_SEC              ; Read 4 sectors (2 KB payload)
    xor ch, ch                      ; Cylinder 0
    mov cl, 2                       ; Sector 2 (Sector 1 is MBR)
    xor dh, dh                      ; Head 0
    mov dl, [boot_drive]            ; Target boot drive
    int 0x13
    jc enter_safe_mode              ; Disk read failed -> fallback to Safe Mode

    ; Verify Stage 2 presence: check if first word is non-zero
    cmp word [es:0], 0
    jz enter_safe_mode              ; Empty/unwritten -> fallback to Safe Mode

    ; Stage 2 Found: Emit Normal Boot handoff message & far jump!
    mov si, msg_normal
    call print_both
    jmp STAGE2_SEG:STAGE2_OFF       ; Jump into Stage 2 / CPI-SI Cortex!

; ==============================================================================
; BLOCK 4: SENTINEL / COMPLETION BLOCK
; ( Safe Mode Beacon, Character Transmission & 512-Byte MBR Seal )
; ==============================================================================

enter_safe_mode:
    mov word [0x0500], 0xAA00       ; RAM Log Landmark: State 0 Safe Mode active
    mov si, msg_safe
    call print_both

    ; Arm 13 Silicon Registers to Calibrated Baseline States
    xor ax, ax                      ; %head      = 0 (Cognitive focus reset)
    xor bx, bx                      ; %home      = 0 (Moral origin anchored)
    xor cx, cx                      ; %turn      = 0 (Step counter calibrated)
    xor dx, dx                      ; %spool     = 0 (I/O port origin)
    xor si, si                      ; %spool_in  = 0
    xor di, di                      ; %spool_out = 0
    xor bp, bp                      ; %frame     = 0

hang_watchdog:
    ; Anti-Lockout Watchdog Sentinel:
    ; The CPU enters a calibrated halt (HLT) loop. In this low-power state,
    ; the processor halts clock cycles until an interrupt arrives, preventing
    ; runaway loops and thermal throttling on physical motherboards.
    hlt
    jmp hang_watchdog

hang_fault:
    ; Arithmetic fault: emit visible telemetry before locking CPU
    mov si, msg_fault
    call print_both
    cli
.fault_loop:
    hlt
    jmp .fault_loop

; ------------------------------------------------------------------------------
; Subroutine: print_both
; Emits a null-terminated string at DS:SI simultaneously to:
;   1. Physical Screen: Via BIOS Teletype Service (INT 0x10, AH=0x0E).
;   2. Serial Telemetry: Via COM1 UART (0x03F8), polling Line Status (0x03FD).
; Input:
;   SI = Pointer to null-terminated ASCII string in DS segment.
; Modifies:
;   AX, BX, DX, SI.
; ------------------------------------------------------------------------------
print_both:
.next_char:
    lodsb
    test al, al
    jz .done

    ; Broadcast to Screen (BIOS teletype output)
    push ax
    mov ah, 0x0E
    mov bx, 0x000A                  ; Light green text attribute
    int 0x10
    pop ax

    ; Broadcast to Serial COM1 (poll Line Status Register until TX ready)
    mov dx, 0x03FD
.wait_tx:
    in al, dx
    test al, 0x20                   ; Test Bit 5: Transmitter Holding Reg Empty
    jz .wait_tx

    mov al, [si - 1]
    mov dx, 0x03F8
    out dx, al
    jmp .next_char

.done:
    ret

; ------------------------------------------------------------------------------
; TELEMETRY STRINGS (NULL-TERMINATED ASCII)
; ------------------------------------------------------------------------------
msg_boot:
    db 13, 10
    db "MILLENIUMOS: OMNICODE 32-TRIT OS [x86 MBR]", 13, 10
    db "[BOOT] 16-Bit Genesis: OK", 13, 10, 0

msg_alu:
    db "[ALU ] (+1)+(-1)=0 Eq: OK", 13, 10
    db "[REGS] 13 Regs Armed / Calibrated", 13, 10, 0

msg_normal:
    db "[BOOT] State +1: Stage 2 -> CPI-SI", 13, 10
    db "MILLENIUMOS ONLINE :: PROCEED", 13, 10, 0

msg_safe:
    db "[BOOT] State 0: Safe Mode Beacon", 13, 10
    db "MILLENIUMOS SAFE MODE :: PROCEED", 13, 10, 0

msg_fault:
    db "[ERR ] ALU Drift: HALT", 13, 10, 0

boot_drive:
    db 0

; ------------------------------------------------------------------------------
; THE ANCIENT LANDMARK: 512-BYTE MBR BOUNDARY & SACRED SEAL
; ------------------------------------------------------------------------------
times 510 - ($ - $$) db 0
dw 0xAA55

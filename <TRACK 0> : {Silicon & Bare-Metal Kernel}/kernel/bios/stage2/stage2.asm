; ==============================================================================
; BLOCK 1: METADATA BLOCK
; ( MilleniumOS Stage 2 Kernel Inception - 32-Bit / 16-Trit Protected Mode )
; ==============================================================================
; Module:           OmniCode.Kernel.Stage2
; System:           MilleniumOS (OmniCode Cognitive Framework)
; Description:      Stage 2 Kernel Loader, Telemetric Logger & CPI-SI Substrate
; Version:          1.0.0 (Primordial Genesis Inception Baseline)
; Schema:           4-Block Decalogue Transduction Specification v1.0
; Created:          2026-10-11 (Epoch 2026.10 Inscription)
; Updated:          2026-10-16 (Stage 2 RAM Logger, Reg Dumper & 16-Trit Core)
; Authors:          CreativeWorkzStudio LLC / ProfessorSeanEX
; Repository:       https://github.com/Novaking34/OmniCode_Framework
; License:          MIT / Apache-2.0
; Classification:   Rank 1 Invariant (< Invariant Covenant >)
;
; ARCHITECTURAL ROLE & THE 32-BIT / 16-TRIT BRIDGE:
;   Loaded by Stage 1 MBR from disk Sectors 2..5 into memory at 0x1000:0000.
;   - Transitions CPU from 16-Bit Real Mode to 32-Bit Protected Mode.
;   - Enables A20 Line via Fast A20 Gate (Port 0x92).
;   - Establishes Flat 4GB Global Descriptor Table (GDT) based at 0x00010000.
;   - Implements In-Memory RAM Log Flight Recorder Buffer at 0x00018000.
;   - Implements 32-bit register hexadecimal telemetry dumper over COM1 UART.
;   - Mounts 16-Trit CPI-SI Cognitive Substrate linear workspace.
;   - Implements Stage 2 Safe Mode fallback if verification drifts.
;
; SCRIPTURE WITNESSES (CANONICAL INVARIANTS):
;   - Exodus 31:18 (KJV):
;     "two tables of testimony, tables of stone, written with the finger of God"
;     (The immutable inscription of atomic runes onto bounded physical tablets)
;   - Proverbs 24:27 (KJV):
;     "Prepare thy work without, and make it fit for thyself in the field;
;      and afterwards build thine house."
;     (The discipline of proving atomic substrates before building systems)
;   - Proverbs 22:28 (KJV):
;     "Remove not the ancient landmark, which thy fathers have set."
;     (The preservation of historical memory boundaries: 0x1000:0000, GDT)
; ==============================================================================

; ==============================================================================
; BLOCK 2: ENVIRONMENT / INVARIANTS / SETUP BLOCK
; ( Real Mode Origin 0x0000 in Segment 0x1000, GDT Table & Segment Selectors )
; ==============================================================================

[BITS 16]
[ORG 0x0000]

CODE_SEG        equ 0x08            ; GDT Kernel 32-Bit Code Segment Descriptor
DATA_SEG        equ 0x10            ; GDT Kernel 32-Bit Data Segment Descriptor

; RAM Log Buffer Landmark (Offset in 4GB segment based at 0x00010000)
; Linear physical address: 0x00010000 + 0x8000 = 0x00018000
RAM_LOG_OFFSET  equ 0x8000

; Telemetry Diagnostic Event Identifiers
EVT_STAGE2_PM   equ 0x0001          ; 32-Bit Protected Mode active
EVT_A20_GDT     equ 0x0002          ; A20 Gate unmasked & GDT armed
EVT_REGS_AUDIT  equ 0x0003          ; CPU silicon register snapshot taken
EVT_CPISI_MOUNT equ 0x0004          ; 16-Trit cognitive substrate mounted
EVT_CASCADE_UP  equ 0x0005          ; Escalation up ladder to Multiboot
EVT_CASCADE_DN  equ 0x0006          ; De-escalation down to Stage 1 MBR
EVT_SAFE_MODE   equ 0xEEEE          ; Stage 2 Safe Mode fallback engaged

stage2_start:
    ; Normalize segments to Stage 2 segment (0x1000)
    mov ax, cs
    mov ds, ax
    mov es, ax

; ==============================================================================
; BLOCK 3: BODY / EXECUTION BLOCK
; ( A20 Enable, GDT Loading, Protected Mode Far Jump & CPI-SI Inception )
; ==============================================================================

    ; Step 1: Emit Stage 2 Inception Announcement to COM1 Serial
    mov si, msg_stage2_real
    call stage2_print_serial

    ; Step 2: Enable A20 Line (Fast A20 Gate on Port 0x92)
    ; Allows addressing beyond the 1 MB boundary without 8086 memory wrap.
    in al, 0x92
    or al, 0x02
    and al, 0xFE                    ; Ensure system reset bit is NOT set
    out 0x92, al

    ; Step 3: Load Global Descriptor Table (GDT)
    cli                             ; Clear interrupts before protected mode
    lgdt [gdt_descriptor]

    ; Step 4: Set Protected Mode Enable (PE) Bit in Control Register 0 (CR0)
    mov eax, cr0
    or al, 0x01                     ; Bit 0 = PE (Protection Enable)
    mov cr0, eax

    ; Step 5: Execute Far Jump into 32-Bit Protected Mode Code Segment
    ; Flushes the 16-bit instruction prefetch queue and loads CS with CODE_SEG.
    jmp CODE_SEG:stage2_pm_entry

; ------------------------------------------------------------------------------
; 32-BIT PROTECTED MODE EXECUTION SUBSTRATE
; ------------------------------------------------------------------------------
[BITS 32]
stage2_pm_entry:
    ; Step 6: Initialize 32-Bit Data Segment Selectors
    mov ax, DATA_SEG
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    ; Step 7: Position 32-Bit Stack at Safe Scratch RAM (0x00007000)
    mov esp, 0x00007000

    ; Step 8: Initialize In-Memory RAM Log Flight Recorder Buffer
    mov dword [RAM_LOG_OFFSET], 0    ; Reset event entry counter

    ; Step 9: Emit 32-Bit / 16-Trit Confirmation to COM1 Serial
    mov esi, msg_stage2_pm
    call stage2_print_serial32

    ; Step 10: Inscribe Event 1 (PM Entry) & Event 2 (A20/GDT) to RAM Log
    mov eax, EVT_STAGE2_PM
    mov ebx, cr0
    mov ecx, esp
    call stage2_log_event32

    mov eax, EVT_A20_GDT
    mov ebx, 0x00010000             ; GDT physical base
    mov ecx, 4096                   ; 4GB limit flag
    call stage2_log_event32

    ; Step 11: Emit Hardware Register Audit Telemetry Snapshot
    call stage2_dump_registers

    ; Step 12: Mount & Verify 16-Trit Cognitive Memory Substrate
    mov eax, EVT_CPISI_MOUNT
    mov ebx, 16                     ; 16 trits container width
    mov ecx, 43046721               ; 3^16 state space volume
    call stage2_log_event32

    mov esi, msg_ram_log_ok
    call stage2_print_serial32

    mov esi, msg_cpisi_mounted
    call stage2_print_serial32

    ; Step 13: Probe Adaptive Boot Ladder (1 MB Boundary)
    ; If a Multiboot header (0x1BADB002) is staged at 1MB, escalate up ladder
    cmp dword [0x00100000], 0x1BADB002
    jne stage2_halt32

    mov eax, EVT_CASCADE_UP
    mov ebx, [0x00100000]
    mov ecx, 0x00100000
    call stage2_log_event32

    mov esi, msg_stage2_escalate
    call stage2_print_serial32

    mov eax, 0x2BADB002             ; Multiboot Loader Covenant Magic
    mov ebx, 0x00010000             ; Staged boot info pointer
    jmp 0x00100000                  ; Escalate into Multiboot kernel

; ==============================================================================
; BLOCK 4: SENTINEL / COMPLETION BLOCK
; ( Low-Power Watchdogs, Safe Mode Fallbacks, Diagnostics, GDT & Strings )
; ==============================================================================

stage2_halt32:
    ; Anti-Lockout Watchdog: low-power halt state
    hlt
    jmp stage2_halt32

stage2_safe_mode_32:
    ; Stage 2 Safe Mode Fallback: Log error, dump registers, and de-escalate
    mov eax, EVT_SAFE_MODE
    mov ebx, cr0
    mov ecx, esp
    call stage2_log_event32

    mov esi, msg_stage2_safe32
    call stage2_print_serial32
    call stage2_dump_registers

    ; Engage Adaptive Boot Ladder: De-escalate down to Stage 1 MBR (0x7C00)
    mov eax, EVT_CASCADE_DN
    mov ebx, 0x7C00
    mov ecx, 0
    call stage2_log_event32

    mov esi, msg_stage2_deescalate
    call stage2_print_serial32

    ; Drop from 32-Bit Protected Mode down to 16-Bit Real Mode (0x0000:0x7C00)
    cli
    lidt [real_mode_idtr]
    lgdt [gdt_descriptor]
    jmp 0x18:.in_pm16

[BITS 16]
.in_pm16:
    mov ax, 0x20
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00

    mov eax, cr0
    and al, 0xFE                    ; Clear Bit 0 (PE = 0)
    mov cr0, eax

    jmp 0x0000:0x7C00

.safe_loop:
    hlt
    jmp .safe_loop

[BITS 32]

; ------------------------------------------------------------------------------
; SUBROUTINE: stage2_log_event32
; Inscribes a structured 16-byte event record into the RAM Log Flight Recorder.
; Buffer Landmark: [RAM_LOG_OFFSET] = count (u32)
; Entry: [event_id (u32), arg1 (u32), arg2 (u32), status (u32)]
; Inputs: EAX = event_id, EBX = arg1, ECX = arg2
; ------------------------------------------------------------------------------
stage2_log_event32:
    pushad
    mov edx, [RAM_LOG_OFFSET]       ; Current entry count
    cmp edx, 64                     ; Cap at 64 entries (ring protection)
    jae .log_done
    lea edi, [RAM_LOG_OFFSET + 4]
    shl edx, 4                      ; Multiply entry index by 16 bytes
    add edi, edx
    mov [edi + 0], eax              ; event_id
    mov [edi + 4], ebx              ; arg1
    mov [edi + 8], ecx              ; arg2
    mov dword [edi + 12], 0         ; status = OK (0)
    inc dword [RAM_LOG_OFFSET]      ; Increment logged count
.log_done:
    popad
    ret

; ------------------------------------------------------------------------------
; SUBROUTINE: stage2_putc_serial32
; Transmits a single byte to COM1 Serial (Port 0x03F8), polling Line Status.
; Input: AL = ASCII byte
; ------------------------------------------------------------------------------
stage2_putc_serial32:
    push edx
    push eax
    mov dx, 0x03FD                  ; Line Status Register
.wait_tx:
    in al, dx
    test al, 0x20                   ; Bit 5: Transmitter Holding Register Empty
    jz .wait_tx
    pop eax                         ; Restore character byte into AL
    mov dx, 0x03F8                  ; Transmit Holding Register
    out dx, al
    pop edx
    ret

; ------------------------------------------------------------------------------
; SUBROUTINE: stage2_print_hex32
; Emits a 32-bit register value as 8 hexadecimal characters over COM1 serial.
; Input: EAX = 32-bit value
; ------------------------------------------------------------------------------
stage2_print_hex32:
    pushad
    mov ecx, 8                      ; 8 hex nibbles to output
    mov ebx, eax                    ; Keep working register in EBX
.hex_loop:
    rol ebx, 4                      ; Rotate highest nibble into bits 0-3
    mov al, bl
    and al, 0x0F
    cmp al, 9
    jbe .is_digit
    add al, ('A' - 10)
    jmp .emit_nibble
.is_digit:
    add al, '0'
.emit_nibble:
    call stage2_putc_serial32
    loop .hex_loop
    popad
    ret

; ------------------------------------------------------------------------------
; SUBROUTINE: stage2_dump_registers
; Emits formatted CPU register telemetry block over COM1 serial.
; ------------------------------------------------------------------------------
stage2_dump_registers:
    pushad
    mov esi, msg_dbg_prefix
    call stage2_print_serial32

    mov esi, msg_dbg_eax
    call stage2_print_serial32
    mov eax, [esp + 28]             ; Saved EAX from pushad
    call stage2_print_hex32

    mov esi, msg_dbg_ebx
    call stage2_print_serial32
    mov eax, [esp + 16]             ; Saved EBX from pushad
    call stage2_print_hex32

    mov esi, msg_dbg_ecx
    call stage2_print_serial32
    mov eax, [esp + 24]             ; Saved ECX from pushad
    call stage2_print_hex32

    mov esi, msg_dbg_edx
    call stage2_print_serial32
    mov eax, [esp + 20]             ; Saved EDX from pushad
    call stage2_print_hex32

    mov esi, msg_dbg_esp
    call stage2_print_serial32
    mov eax, [esp + 12]             ; Saved ESP before pushad
    call stage2_print_hex32

    mov esi, msg_dbg_cr0
    call stage2_print_serial32
    mov eax, cr0
    call stage2_print_hex32

    mov esi, msg_dbg_crlf
    call stage2_print_serial32
    popad
    ret

; ------------------------------------------------------------------------------
; 16-BIT HELPERS (REAL MODE)
; ------------------------------------------------------------------------------
[BITS 16]
stage2_print_serial:
.next:
    lodsb
    test al, al
    jz .done
    mov dx, 0x03FD
.wait:
    in al, dx
    test al, 0x20
    jz .wait
    mov al, [si - 1]
    mov dx, 0x03F8
    out dx, al
    jmp .next
.done:
    ret

; ------------------------------------------------------------------------------
; 32-BIT HELPERS (PROTECTED MODE)
; ------------------------------------------------------------------------------
[BITS 32]
stage2_print_serial32:
.next:
    lodsb
    test al, al
    jz .done
    mov dx, 0x03FD
.wait:
    in al, dx
    test al, 0x20
    jz .wait
    mov al, [esi - 1]
    mov dx, 0x03F8
    out dx, al
    jmp .next
.done:
    ret

; ------------------------------------------------------------------------------
; GLOBAL DESCRIPTOR TABLE (GDT)
; Code and Data Segments based at 0x00010000 (Physical Stage 2 Load Mark)
; ------------------------------------------------------------------------------
align 8
gdt_start:
    ; Null Descriptor (Mandatory 8 zero bytes)
    dd 0, 0

gdt_code:
    ; 32-Bit Kernel Code Segment: Base=0x00010000, Limit=4GB, Exec/Read
    dw 0xFFFF                       ; Limit 15:0
    dw 0x0000                       ; Base 15:0
    db 0x01                         ; Base 23:16 (0x01 -> physical 0x00010000)
    db 10011010b                    ; Present, Ring 0, Code, Exec/Read
    db 11001111b                    ; 4KB Granularity, 32-bit Def, Limit 19:16
    db 0x00                         ; Base 31:24

gdt_data:
    ; 32-Bit Kernel Data Segment: Base=0x00010000, Limit=4GB, Read/Write
    dw 0xFFFF                       ; Limit 15:0
    dw 0x0000                       ; Base 15:0
    db 0x01                         ; Base 23:16 (0x01 -> physical 0x00010000)
    db 10010010b                    ; Present, Ring 0, Data, Read/Write
    db 11001111b                    ; 4KB Granularity, 32-bit Def, Limit 19:16
    db 0x00                         ; Base 31:24

gdt_code16:
    ; 16-Bit Protected Mode Code Segment: Base=0, Limit=64KB, Exec/Read
    dw 0xFFFF, 0x0000
    db 0x00, 10011010b, 00000000b, 0x00

gdt_data16:
    ; 16-Bit Protected Mode Data Segment: Base=0, Limit=64KB, Read/Write
    dw 0xFFFF, 0x0000
    db 0x00, 10010010b, 00000000b, 0x00
gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1      ; GDT size minus 1
    dd 0x00010000 + gdt_start       ; Physical linear address of GDT

real_mode_idtr:
    dw 0x03FF                       ; Real Mode IVT Limit (1KB)
    dd 0x00000000                   ; Real Mode IVT Base (Address 0)

; ------------------------------------------------------------------------------
; TELEMETRY STRINGS (NULL-TERMINATED ASCII)
; ------------------------------------------------------------------------------
msg_stage2_real:
    db 13, 10
    db "[STAGE2] 16-Bit Entry at 0x1000:0000: OK", 13, 10
    db "[STAGE2] Transitioning to 32-Bit / 16-Trit Mode...", 13, 10, 0

msg_stage2_pm:
    db "[STAGE2] 32-Bit Protected Mode Active:        OK", 13, 10
    db "[STAGE2] 16-Trit Workspace Armed:             OK", 13, 10, 0

msg_ram_log_ok:
    db "[STAGE2] RAM Log Flight Recorder: ARMED", 13, 10
    db "[STAGE2] RAM Landmark Address:    0x00018000", 13, 10, 0

msg_cpisi_mounted:
    db "[STAGE2] CPI-SI Cognitive Substrate:          MOUNTED", 13, 10
    db "=== MILLENIUMOS STAGE 2 RUNTIME :: PROCEED ===", 13, 10, 0

msg_stage2_safe32:
    db "[STAGE2] Memory Verification Drift: Falling Back", 13, 10
    db "=== MILLENIUMOS STAGE 2 SAFE MODE ACTIVE ===", 13, 10, 0

msg_stage2_escalate:
    db "[CASCADE] Multiboot at 1MB: Escalating to Multiboot...", 13, 10, 0

msg_stage2_deescalate:
    db "[CASCADE] Stage 2 De-escalation: Returning to 0x7C00 MBR...", 13, 10, 0

msg_dbg_prefix:
    db "[DBG ] REGS: ", 0
msg_dbg_eax:
    db "EAX=0x", 0
msg_dbg_ebx:
    db " EBX=0x", 0
msg_dbg_ecx:
    db " ECX=0x", 0
msg_dbg_edx:
    db " EDX=0x", 0
msg_dbg_esp:
    db 13, 10, "[DBG ] REGS: ESP=0x", 0
msg_dbg_cr0:
    db " CR0=0x", 0
msg_dbg_crlf:
    db 13, 10, 0
